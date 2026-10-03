---
bump: patch
---

### Added
- A native merged Scheme grammar in Links Notation, `parity/grammars/native/scheme.lino`. It merges tree-sitter-scheme 0.24.7 and the R7RS small report: it builds the tree-sitter-scheme trees of lists with any bracket style, vectors, `#vu8(` byte vectors, quote, quasiquote, unquote and syntax forms, booleans, characters, strings with escape sequences, the R5RS, R6RS and R7RS numbers, symbols, keywords, directives and line, datum and nested block comments, and it reads the R7RS `#u8(` bytevectors and `#<n>=` / `#<n>#` datum labels the oracle recovers from. Both native executors run it with no ambiguity. The default Scheme parse still uses tree-sitter-scheme until the grammar has recovery rules.
- `parity/fixtures/native-grammars/scheme.json` checks the grammar against tree-sitter-scheme on 264 matches, 23 R7RS small divergences and 50 rejections. The ledger row `I195-GRAMMAR-NATIVE-SCHEME` tracks the JavaScript and Rust suites.
