const _HighlightRustCompiler = {
  id: "highlight-rust-compiler",

  init: function (reveal) {
    console.log("initializing rust plugin");
    const highlight = reveal.getPlugin("highlight");

    highlight.hljs.registerLanguage("rust-compiler", function (_hljs) {
      console.log("registering rust-compiler language");
      return {
        contains: [
          {
            className: "error",
            begin: "^error",
            end: "-->",
            excludeEnd: true,
            contains: [
              {
                className: "error-link",
                begin: /\[(?=E)/,
                excludeBegin: true,
                end: /]:/,
                excludeEnd: true,
              },
            ],
            starts: {
              className: "comment",
              end: "\n",
              contains: [
                {
                  className: "path",
                  begin: " ",
                  excludeBegin: true,
                  end: ":",
                  excludeEnd: true,
                },
              ],
            },
          },
          {
            className: "comment",
            begin: /\d+ +\|/,
            end: / +/,
            starts: {
              subLanguage: "rust",
              end: "\n",
            },
          },
          {
            className: "comment",
            begin: /\|/,
            end: "\n",
          },
        ],
      };
    });

    const error_index_base_url = "https://doc.rust-lang.org/error-index.html#";
    const oldHighlightBlock = highlight.highlightBlock;
    highlight.highlightBlock = function (block) {
      oldHighlightBlock(block);
      block.querySelectorAll(".hljs-error-link").forEach((span) => {
        const link = document.createElement("a");
        link.href = error_index_base_url + span.innerText;
        link.innerHTML = span.innerHTML;
        span.replaceWith(link);
      });
    };
  },
};

const HighlightRustCompiler = () => _HighlightRustCompiler;
