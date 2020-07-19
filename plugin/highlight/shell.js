const _HighlightShellSession = {
  id: "highlight-shell-session",

  init: function (reveal) {
    const highlight = reveal.getPlugin("highlight");

    highlight.hljs.registerLanguage("shell-session", function (_hljs) {
      return {
        contains: [
          {
            className: "keyword",
            begin: "^\\$",
            end: " ",
            keywords: "$",
            starts: {
              className: "title",
              subLanguage: "bash",
              end: "\n",
            },
          },
        ],
      };
    });
  },
};

const HighlightShellSession = () => _HighlightShellSession;
