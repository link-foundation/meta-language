# Vendored tree-sitter CSV parser

`src/parser.c.gz` and `src/tree_sitter/parser.h` are the generated `csv`
grammar assets from
[`tree-sitter-grammars/tree-sitter-csv`](https://github.com/tree-sitter-grammars/tree-sitter-csv)
tag `v1.2.0` (revision `cda48a5e890b30619da5bc3ff55be1b1d3d08c8d`), the same
source as the `tree-sitter-csv` 1.2.0 crate. The crate itself is not used
because it pins `cc ~1.0.82`, which conflicts with other grammar crates.

The uncompressed `parser.c` SHA-256 is
`2676f33c1a418ae94ee59bb3acbe6734f387bfaafcb484dee6ce5791c43231f8`.
The `parser.h` SHA-256 is
`83a7aebb8067b898fa0975e4c627148ec2407617865b42edc91537f6bf651591`.

The grammar is MIT licensed; see `LICENSE`. `build.rs` expands the
deterministically compressed parser into Cargo's output directory before
compilation.
