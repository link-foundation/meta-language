---
bump: minor
---

### Added
- Rust parses with the native merged grammar `parity/grammars/native/rust.lino` by default in both runtimes. The import pipeline writes it from the pinned `src/grammar.json` of tree-sitter-rust 0.24.2 with the oracle's patch, and `parity/grammars/scanners/rust.lino` ports its external scanner to native scanner links.
- `parity/fixtures/native-grammars/rust.json` checks the grammar against tree-sitter-rust on 170 matches and 35 rejections, with no ambiguity. The ledger row `I195-GRAMMAR-NATIVE-RUST` tracks the JavaScript and Rust suites.
- `parity/fixtures/native-default-cst-expected.json` gives the native rows of embedded regions in a native language; the Markdown fenced Rust recovery region is repaired with a MISSING `}`.

### Fixed
- The feature grammar executor settles a reduce/reduce conflict by precedence, as tree-sitter does (Rust's `m!(x);` is an expression statement), and two parses that build the same tree are no longer an ambiguity.
