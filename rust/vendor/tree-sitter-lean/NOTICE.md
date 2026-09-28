# Vendored tree-sitter Lean parser

`src/parser.c.gz`, `src/scanner.c` and the `src/tree_sitter` headers are the
unmodified generated grammar assets of
[`wvhulle/tree-sitter-lean`](https://github.com/wvhulle/tree-sitter-lean)
revision `bd942cd2795016239be02b3b3d5ef635645ddd38` ("bump cargo version"),
byte-identical to the `src` files of the `tree-sitter-lean4` 0.3.0 crate.

The crate is not used because its build script compiles `parser.c` without
MSVC's `/utf-8`. Generated parsers spell non-ASCII node names as universal
character names (`"\u00d7"` for `×`), and MSVC then encodes them in the
Windows code page, so on Windows `Node::kind` returned names that are not
UTF-8 (tree-sitter panicked on `×`) or that silently read `?` (`→`, `∀`,
`⟨`). `build.rs` compiles this copy with `-utf-8` like the other vendored
grammars. `node js/scripts/build-vendored-grammars.mjs --only lean` clones
the revision and requires its `src/parser.c` to equal `src/parser.c.gz`;
regenerating `grammar.js` with tree-sitter CLI 0.25.10 reproduces the same
`parser.c`.

The uncompressed `parser.c` SHA-256 is
`d04d21e8ad0132f9f3410296817a4317fc11d4f8cbd35c62dc3fe0b72f4f9a6b`.
The `scanner.c` SHA-256 is
`6927b5de4e9818befd7f040b20ec14c0b87f7f7524cf2bb3fb4a5ad75d63e2c5`.
The `parser.h` SHA-256 is
`180b893c8734778fd32f372dfbc27bd6ad1cd2221f26150b31256ff6716320d2`.

The crate manifest declares `license = "MIT"` without a license file; see
`LICENSE`. `build.rs` expands the deterministically compressed parser into
Cargo's output directory before compilation.
