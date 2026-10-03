---
bump: patch
---

### Fixed
- Where a token from an external scanner conflicts with a token the lexer matches, both runtimes now take the scanner's token, under any alias. Tree-sitter runs the external scanner before its lexer wherever one of the scanner's tokens is valid. For example, JavaScript's automatic semicolon after `return` before a line break now wins over the next line's expression.
- The tree-sitter importer now substitutes an `inline` rule that an alias names, as tree-sitter does, so the alias gives one leaf. It also keeps an alias of a lexical `IMMEDIATE_TOKEN` rule as an immediate token, for example a string fragment.
