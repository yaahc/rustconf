use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::iter;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use eyre::{eyre, WrapErr};
use handlebars::Handlebars;
use lol_html::{element, HtmlRewriter, OutputSink, Settings};
use pulldown_cmark::{html, CowStr, Event, Options, Parser};
use serde::Serialize;
use structopt::StructOpt;
use tracing::{event, info, instrument, span, warn, Level};

#[derive(Debug, StructOpt)]
#[structopt(about = "A helper tool for compiling reveal.js slideshows from Markdown")]
struct Opt {
    /// Log level.
    ///
    /// Can be an integer 1-5 or "error", "warn", "info", "debug", "trace",
    /// case-insensitive.
    #[structopt(long, default_value = "warn")]
    trace_level: Level,

    /// Watch for changes to files and keep re-rendering?
    #[structopt(short, long)]
    watch: bool,

    /// Debounce filesystem events to a given granularity, in milliseconds.
    #[structopt(long, default_value = "250")]
    debounce_ms: u64,

    /// Slideshow template.
    #[structopt(long, parse(from_os_str), default_value = "template.html")]
    template: PathBuf,

    /// Path to html_touchup helper directory
    #[structopt(long, parse(from_os_str), default_value = "md-compiler/html-touchup")]
    html_touchup: PathBuf,

    /// Input Markdown file.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output HTML file.
    #[structopt(parse(from_os_str), default_value = "index.html")]
    output: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().expect("Couldn't install color_eyre error reporter");

    let opt = {
        let mut opt = Opt::from_args();
        opt.template = opt.template.canonicalize().wrap_err(format!(
            "Failed to canonicalize template argument {:?}",
            opt.template
        ))?;
        opt.input = opt.input.canonicalize().wrap_err(format!(
            "Failed to canonicalize input argument {:?}",
            opt.input
        ))?;
        opt
    };

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(opt.trace_level.clone())
        // .event_format(Format::default().compact())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default subscriber failed");

    let mut app = App {
        opt,
        input_buf: String::with_capacity(10_000),
        rendered_md: String::with_capacity(10_000),
        template_buf: String::with_capacity(10_000),
        handlebars: Handlebars::new(),
    };

    if app.opt.watch {
        app.watch()?;
    } else {
        app.render()?;
    }
    Ok(())
}

fn read_to_string(filename: &Path, buf: &mut String) -> eyre::Result<()> {
    let mut file =
        File::open(filename).wrap_err_with(|| format!("Failed to open {:?}", filename))?;
    let size_guess = file
        .metadata()
        .map(|met| met.len() as usize)
        .unwrap_or(10_000);
    if buf.capacity() < size_guess {
        buf.reserve(size_guess - buf.capacity());
    }
    file.read_to_string(buf)
        .wrap_err_with(|| format!("Failed to read contents of {:?}", filename))?;
    Ok(())
}

struct App {
    opt: Opt,
    input_buf: String,
    rendered_md: String,
    template_buf: String,
    handlebars: Handlebars<'static>,
}

#[derive(Serialize, Debug, Clone)]
struct TemplateContext<'a> {
    slides: &'a str,
}

impl App {
    fn render(&mut self) -> eyre::Result<()> {
        self.input_buf.clear();
        self.rendered_md.clear();
        self.template_buf.clear();

        println!(
            "{:?} + {:?} => {:?}",
            &self.opt.input, &self.opt.template, &self.opt.output
        );
        read_to_string(&self.opt.input, &mut self.input_buf)?;
        read_to_string(&self.opt.template, &mut self.template_buf)?;

        let parser = Self::map_parser(Parser::new(&self.input_buf));
        html::push_html(&mut self.rendered_md, parser);

        let template_name = "input_template";

        self.handlebars
            .register_template_source(template_name, &mut self.template_buf.as_bytes())?;

        let template_context = TemplateContext {
            slides: &self.rendered_md,
        };

        {
            let output = BufWriter::new(File::create(&self.opt.output)?);
            self.handlebars
                .render_to_write(template_name, &template_context, output)?;
        }

        let mut child = Command::new("nix-shell")
            .args(&[
                self.opt.html_touchup.as_os_str(),
                OsStr::new("--command"),
                &{
                    let mut ret: OsString = "python3.8 ".into();
                    ret.push(&self.opt.html_touchup.join("html_touchup.py"));
                    ret
                },
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .wrap_err("Failed to launch Python helper html_touchup")?;

        {
            let mut stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| eyre!("Failed to get html_touchup's stdin handle"))?;

            let mut output = BufReader::new(File::open(&self.opt.output)?);

            io::copy(&mut output, &mut stdin)?;
        }

        let html_touchup_out = child
            .wait_with_output()
            .wrap_err("Failed to get html_touchup's output")?;

        if !html_touchup_out.status.success() {
            return Err(eyre!("Failed to execute html_touchup")
                .wrap_err(String::from_utf8_lossy(&html_touchup_out.stderr).to_string()));
        }

        File::create(&self.opt.output)?
            .write_all(&html_touchup_out.stdout)
            .wrap_err("Failed to write html_touchup's output")?;

        Ok(())
    }

    #[instrument(skip(self))]
    fn watch(&mut self) -> eyre::Result<()> {
        use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
        use std::time::Duration;

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_millis(self.opt.debounce_ms))?;

        watcher.watch(
            &self.opt.input.parent().unwrap_or(&self.opt.input),
            RecursiveMode::NonRecursive,
        )?;
        watcher.watch(
            &self.opt.template.parent().unwrap_or(&self.opt.template),
            RecursiveMode::NonRecursive,
        )?;
        watcher.watch(&self.opt.html_touchup, RecursiveMode::NonRecursive)?;

        let filename = |p: &Path| {
            p.file_name()
                .unwrap_or_else(|| panic!(format!("{:?} should have a filename", p)))
                .to_owned()
        };

        let input_fn = filename(&self.opt.input);
        let template_fn = filename(&self.opt.template);
        let html_touchup_fn = filename(&self.opt.html_touchup.join("html_touchup.py"));
        let is_relevant = |path: &Path| -> bool {
            path.file_name()
                .map(|file_name| {
                    file_name == input_fn
                        || file_name == template_fn
                        || file_name == html_touchup_fn
                })
                .unwrap_or(true)
        };

        fn print_res(res: eyre::Result<()>) {
            if let Err(res) = res {
                println!("{}", res);
            }
        }

        event!(Level::INFO, "initialized filesystem watcher");

        print_res(self.render());

        loop {
            let event = {
                let span = span!(Level::INFO, "watch");
                let _guard = span.enter();
                rx.recv()?
            };
            let span = span!(Level::INFO, "filesystem event", event = ?event);
            let _guard = span.enter();
            event!(Level::INFO, ?event);
            match event {
                DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                    if is_relevant(&path) {
                        print_res(self.render());
                    }
                }
                DebouncedEvent::Chmod(path) => {
                    if is_relevant(&path) {
                        print_res(self.render());
                    }
                }
                DebouncedEvent::Remove(_path) => {
                    event!(Level::INFO, "remove (unimplemented)");
                }
                DebouncedEvent::Rename(_from, _to) => {
                    event!(Level::INFO, "rename (unimplemented)");
                }
                DebouncedEvent::Rescan => {
                    event!(Level::INFO, "rescanning watched files");
                }
                DebouncedEvent::Error(err, path) => {
                    if let Some(path) = &path {
                        event!(Level::ERROR, ?path);
                    }
                    return Err(eyre!("Watch error for path {:?}: {:?}", path, err));
                }
                _ => {
                    event!(Level::DEBUG, "unhandled event");
                }
            }
        }
    }

    fn map_parser(parser: Parser) -> impl Iterator<Item = Event> {
        iter::once(Event::Html("<section>".into()))
            .chain(MappedParser::new(parser))
            .chain(iter::once(Event::Html("</section>".into())))
    }
}

fn escape_fenced_header(header: CowStr<'_>) -> CowStr<'_> {
    // Haha ACAB
    header.replace(' ', "\u{101312}").into()
}

struct MappedParser<'a> {
    inner: Parser<'a>,
    has_notes: bool,
    started_paragraph: bool,
    lookahead: Option<Event<'a>>,
    in_code: bool,
}

impl<'a> MappedParser<'a> {
    pub fn new(inner: Parser<'a>) -> Self {
        Self {
            inner,
            has_notes: false,
            started_paragraph: false,
            in_code: false,
            lookahead: None,
        }
    }
}

impl<'a> Iterator for MappedParser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use pulldown_cmark::{CodeBlockKind, Tag};
        use std::mem;

        if self.lookahead.is_some() {
            return mem::take(&mut self.lookahead);
        }

        let event = self.inner.next()?;
        let started_paragraph = matches!(event, Event::Start(Tag::Paragraph));

        let ret = Some(match event {
            Event::Rule => {
                let ret = Event::Html(
                    format!(
                        "{}</section>\n<section>",
                        if self.has_notes { "</aside>" } else { "" }
                    )
                    .into(),
                );
                self.has_notes = false;
                ret
            }
            Event::Text(text) => {
                if self.started_paragraph && text.starts_with("Notes:") {
                    self.lookahead = Some(Event::Text(
                        text.strip_prefix("Notes:").unwrap().to_owned().into(),
                    ));
                    self.has_notes = true;
                    Event::Html(r#"</p><aside class="notes"><p>"#.into())
                } else if self.in_code {
                    Event::Text(text)
                } else {
                    Event::Text(
                        text.replace("---", "—")
                            .replace("--", "–")
                            .replace("...", "…")
                            .into(),
                    )
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(header))) => {
                self.in_code = true;
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(escape_fenced_header(
                    header,
                ))))
            }
            Event::Start(Tag::CodeBlock(_)) => {
                self.in_code = true;
                event
            }
            Event::End(Tag::CodeBlock(_)) => {
                self.in_code = false;
                event
            }
            event => event,
        });
        self.started_paragraph = started_paragraph;
        ret
    }
}

// What'd you expect down here, *tests*? I have to laugh.
