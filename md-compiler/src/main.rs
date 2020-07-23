use std::error;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::iter;
use std::path::{Path, PathBuf, StripPrefixError};
use std::sync::Arc;

use eyre::{eyre, WrapErr};
use handlebars::Handlebars;
use lol_html::{element, HtmlRewriter, OutputSink, Settings};
use pulldown_cmark::{html, CowStr, Event, Parser};
use serde::Serialize;
use structopt::StructOpt;
use thiserror::Error;
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

    /// Input Markdown file.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output HTML file.
    #[structopt(parse(from_os_str), default_value = "index.html")]
    output: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().expect("Couldn't install color_eyre error reporter");
    use tracing_subscriber::fmt::format::Format;

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
        println!(
            "{:?} + {:?} => {:?}",
            &self.opt.input, &self.opt.template, &self.opt.output
        );
        read_to_string(&self.opt.input, &mut self.input_buf)?;
        read_to_string(&self.opt.template, &mut self.template_buf)?;
        let parser = Self::map_parser(Parser::new(&self.input_buf));
        html::push_html(&mut self.rendered_md, parser);

        let mut rewritten_html = Vec::with_capacity(self.rendered_md.len());
        let mut rewriter = html_rewriter(|c: &[u8]| rewritten_html.extend_from_slice(c))?;
        rewriter
            .write(&self.rendered_md.as_bytes())
            .map_err(|err| eyre!(format!("{}", err)))?;

        let template_name = "input_template";

        self.handlebars
            .register_template_source(template_name, &mut self.template_buf.as_bytes())?;

        let template_context = TemplateContext {
            slides: &String::from_utf8(rewritten_html)?,
        };

        let output = File::create(&self.opt.output)?;
        self.handlebars
            .render_to_write(template_name, &template_context, output)?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn watch(&mut self) -> eyre::Result<()> {
        use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
        use std::time::Duration;

        self.render()?;

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_millis(self.opt.debounce_ms)).unwrap();

        watcher
            .watch(
                &self.opt.input.parent().unwrap_or(&self.opt.input),
                RecursiveMode::NonRecursive,
            )
            .unwrap();
        watcher
            .watch(
                &self.opt.template.parent().unwrap_or(&self.opt.template),
                RecursiveMode::NonRecursive,
            )
            .unwrap();

        let input_fn = &self
            .opt
            .input
            .file_name()
            .expect("Input file should have a filename")
            .to_owned();
        let template_fn = &self
            .opt
            .template
            .file_name()
            .expect("Input file should have a filename")
            .to_owned();
        let is_relevant = |path: &Path| -> bool {
            path.file_name()
                .map(|file_name| file_name == input_fn || file_name == template_fn)
                .unwrap_or(true)
        };

        event!(Level::INFO, "initialized filesystem watcher");

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
                        self.render()?;
                    }
                }
                DebouncedEvent::Chmod(path) => {
                    if is_relevant(&path) {
                        self.render()?;
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

fn html_rewriter<O: OutputSink>(sink: O) -> eyre::Result<HtmlRewriter<'static, O>> {
    Ok(HtmlRewriter::try_new(
        Settings {
            element_content_handlers: vec![element!("pre > code", |el| {
                if let Some(class) = el.get_attribute("class") {
                    let new_header = unescape_fenced_header(&class);
                    let mut new_class = String::new();
                    for component in new_header {
                        if component.starts_with('[') {
                            el.set_attribute("data-line-numbers", &component)?;
                        } else if let Some(inx) = component.find('=') {
                            let (key, val) = component.split_at(inx);
                            el.set_attribute(key, &val[1..])?;
                        } else {
                            new_class.push_str(&component);
                            new_class.push(' ');
                        }
                    }
                    el.set_attribute("class", new_class.trim_end_matches(' '))?;
                }
                Ok(())
            })],
            ..Settings::default()
        },
        sink,
    )?)
}

fn escape_fenced_header(header: CowStr<'_>) -> CowStr<'_> {
    // Haha ACAB
    header.replace(' ', "\u{101312}").into()
}

fn unescape_fenced_header(header: &str) -> Vec<String> {
    let mut ret = String::with_capacity(header.len());
    let mut next_is_entity = false;
    let mut skip_next: usize = 0;
    for cp in header.chars() {
        if skip_next != 0 {
            skip_next -= 1;
            continue;
        }

        if next_is_entity {
            // https://github.com/raphlinus/pulldown-cmark/blob/master/src/escape.rs#L92
            // Thank G-d these are unique so I don't have to figure out lookahead or anything.
            ret.push(match cp {
                'q' => {
                    skip_next = 4;
                    '"'
                }
                'a' => {
                    skip_next = 3;
                    '&'
                }
                'l' => {
                    skip_next = 2;
                    '<'
                }
                'g' => {
                    skip_next = 2;
                    '>'
                }
                _ => unreachable!(),
            })
        } else {
            match cp {
                '\u{101312}' => ret.push(' '),
                '&' => {
                    next_is_entity = true;
                }
                c => ret.push(c),
            }
        }
    }
    ret.split_ascii_whitespace().map(str::to_owned).collect()
}

struct MappedParser<'a> {
    inner: Parser<'a>,
    has_notes: bool,
    started_paragraph: bool,
    lookahead: Option<Event<'a>>,
}

impl<'a> MappedParser<'a> {
    pub fn new(inner: Parser<'a>) -> Self {
        Self {
            inner,
            has_notes: false,
            started_paragraph: false,
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
            Event::Rule => Event::Html(
                format!(
                    "{}</section>\n<section>",
                    if self.has_notes { "</aside>" } else { "" }
                )
                .into(),
            ),
            Event::Text(text) => {
                if self.started_paragraph && text.starts_with("Notes: ") {
                    self.lookahead = Some(Event::Text(
                        text.strip_prefix("Notes: ").unwrap().to_owned().into(),
                    ));
                    self.has_notes = true;
                    Event::Html(r#"<aside class="notes">"#.into())
                } else {
                    Event::Text(text)
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(header))) => Event::Start(
                Tag::CodeBlock(CodeBlockKind::Fenced(escape_fenced_header(header))),
            ),
            event => event,
        });
        self.started_paragraph = started_paragraph;
        ret
    }
}