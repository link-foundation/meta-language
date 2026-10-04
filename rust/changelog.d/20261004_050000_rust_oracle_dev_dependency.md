---
bump: minor
---

### Changed
- The patched tree-sitter-rust parser is the oracle of the native Rust grammar only: the crate no longer compiles or publishes it, `grammar_by_id("rust")` and `grammar_names("rust")` return `None`, and its WebAssembly build moves out of the npm package to `js/oracles/grammars`.
