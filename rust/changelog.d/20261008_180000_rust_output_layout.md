---
bump: minor
---

### Changed

- Self-translation into Rust lays out a line longer than 100 characters the way rustfmt lays out a list that does not fit: the items of its longest parenthesized or bracketed list go one per line, indented four spaces past the line, and nested lists break from the outside in (#217). A single item keeps no trailing comma, so a parenthesized expression stays an expression, and string and character literals are never split. Both runtimes lay out alike (`wrapRust` in `js/src/translation/rust-layout.js`, `wrap_rust` in `rust/src/translation/rust_layout.rs`), and the new `long-lines-to-rust` case pins it. A consumer can now hold its committed Rust byte-equal to the translator and still keep every line readable.
