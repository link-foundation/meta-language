---
bump: patch
---

### Added
- A native merged Racket grammar in Links Notation, `parity/grammars/native/racket.lino`. It merges tree-sitter-racket 0.25.0 and the reader chapter of the Racket Reference: it builds the tree-sitter-racket trees of lists with dots in any bracket style, vectors, flvectors and fxvectors, structures, hash tables, boxes, graph labels, quote, quasiquote, unquote and syntax forms, booleans, characters, strings, byte strings, regular expressions, numbers and extflonums, symbols, keywords, `#lang`, `#!` and `#reader` extensions and line, datum and nested block comments. Here strings replace the oracle's external scanner with grammar actions that store the terminator in a state variable, and the grammar reads the line feeds a backslash quotes in characters and symbols, which the oracle recovers from. Both native executors run it with no ambiguity. The default Racket parse still uses tree-sitter-racket until the grammar has recovery rules.
- `parity/fixtures/native-grammars/racket.json` checks the grammar against tree-sitter-racket on 306 matches, 22 Racket Reference divergences and 60 rejections. The ledger row `I195-GRAMMAR-NATIVE-RACKET` tracks the JavaScript and Rust suites.
