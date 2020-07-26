# RustConf slides

`index.html` is the "main page" but just includes
`rust-for-non-systems-programmers.md` (see: [reveal.js Markdown docs][rjs-md]).

## Development: `./dev.py`

`./dev.py` will start a local development server, which includes:

- Sass compiling / watching for changes to the slides and code highlight themes.
- `sfz` (`cargo install sfz`) for a static file server
- The `./md-compiler` binary to compile `template.html` (Handlebars syntax) and
  the input file to `index.html` (it also runs `cargo build release` on launch
  to keep the binary up to date)

If you're using nixpkgs, you can get the dependencies set up with `nix-shell
--command ./dev.py` or `just dev`.

You can also use `./dev.py --no-open` to skip opening your browser to
`localhost:5000`.

## Styles

Slide theme Sass is in `css/theme/source/rustconf.sass`.

Code highlight Sass is in `plugin/highlight/rustconf.sass`.

For the `rust-compiler` and `shell-session` [highlight.js] plugins, see the
[highlight.js docs language definition guide][hjs-lang-def] and/or the [plugin API guide][hjs-plugins].

## Markdown addons

To reveal each item in a list one at a time, write `<list fragments>`
beforehand. Note that this applies recursively, so you don't need another to
nest lists.

```markdown
<list fragments>

- My list
- Goes here
    - Nested lists.
    - Are shown one at a time, too.
```

To add extra attributes (like a class) to a slide, add a `<slide>` right after the horizontal rule:

```markdown
---

<slide class=big-title>

## Content here...
```

Add a "Next slide" note with `Next slide:` in a `Notes:` section.

```markdown
Notes: Speaker notes...

Next slide: Talk about tooling.
```

In code blocks, there's a few features.

Highlight line numbers with `[...]`:

    ```rust [1-10|5|20-30]
    this syntax-highlights as rust and then highlights first lines 1-20, then
    line 5, and then lines 20-30
    ```

You can also add classes:

    ```rust no-line-numbers left
    this is left-aligned instead of centered, and line numbers are hidden even
    if highlights are specified
    ```

The `shell-session` language highlights an interactive bash session:

    ```shell-session
    $ cat hello.txt
    Hi, Jane!
    ```

The `rust-compiler` language highlights Rust compiler output:

    ```rust-compiler
    error messages...
    ```

[rjs-md]: https://revealjs.com/markdown/
[hjs-lang-def]: https://highlightjs.readthedocs.io/en/latest/language-guide.html
[highlight.js]: https://highlightjs.org/
[hjs-plugins]: https://highlightjs.readthedocs.io/en/latest/plugin-api.html
