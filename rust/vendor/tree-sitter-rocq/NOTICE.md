# Vendored tree-sitter Rocq parser

`src/parser.c.gz` and `src/tree_sitter/parser.h` are generated grammar assets
from `aruzdh/tree-sitter-rocq` revision
`300fe33fc299c30f736fd56d8ef8a28b08acd4e6`. The upstream Cargo manifest
declares the grammar as MIT licensed.

The uncompressed `parser.c` SHA-256 is
`fc714e6d0dbecff5264dbea99768d1b44474d1de69726f6ce4cc2121125d8c9c`.
The `parser.h` SHA-256 is
`180b893c8734778fd32f372dfbc27bd6ad1cd2221f26150b31256ff6716320d2`.

The generated C source is stored deterministically compressed to keep every
checked-in source file below the repository's 1,000-line limit. `build.rs`
expands it into Cargo's output directory before compilation.
