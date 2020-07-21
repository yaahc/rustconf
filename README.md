# RustConf slides

`index.html` is the "main page" but just includes
`rust-for-non-systems-programmers.md` (see: [reveal.js Markdown docs][rjs-md]).

## Development: `./dev.py`

`./dev.py` will start a local development server, which includes:

- Sass compiling / watching for changes to the slides and code highlight themes.
- `sfz` (`cargo install sfz`) for a static file server

If you're using nixpkgs, you can get the dependencies set up with `nix-shell
--command ./dev.py` or `just dev`.

## Styles

Slide theme Sass is in `css/theme/source/rustconf.sass`.

Code highlight Sass is in `plugin/highlight/rustconf.sass`.

For the `rust-compiler` and `shell-session` [highlight.js] plugins, see the
[highlight.js docs language definition guide][hjs-lang-def] and/or the [plugin API guide][hjs-plugins].

[rjs-md]: https://revealjs.com/markdown/
[hjs-lang-def]: https://highlightjs.readthedocs.io/en/latest/language-guide.html
[highlight.js]: https://highlightjs.org/
[hjs-plugins]: https://highlightjs.readthedocs.io/en/latest/plugin-api.html
