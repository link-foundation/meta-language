---
bump: minor
---

### Added
- JavaScript parses with the native merged grammar `parity/grammars/native/javascript.lino` by default in both runtimes. The import pipeline writes it from the pinned `src/grammar.json` of tree-sitter-javascript 0.25.0, and `parity/grammars/scanners/javascript.lino` ports its external scanner to five native scanners: automatic semicolons, template characters, the ternary question mark, HTML-like comments and JSX text.
- `parity/fixtures/native-grammars/javascript.json` checks the grammar against tree-sitter-javascript on 146 matches and 24 rejections, with no ambiguity. The ledger row `I195-GRAMMAR-NATIVE-JAVASCRIPT` tracks the JavaScript and Rust suites.

### Fixed
- The JavaScript scanner links no longer carry a stray space before a closing parenthesis, so the grammar round-trips its concepts and source names.
- A MISSING leaf for a literal under a lexical precedence, as the closing `/` of an unterminated JavaScript regular expression, carries the literal as its kind in both runtimes, and a MISSING leaf of no kind is a named `MISSING` node in JavaScript as in Rust.
