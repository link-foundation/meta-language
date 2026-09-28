# Vendored tree-sitter CMake parser

`src/parser.c.gz` and the `src/tree_sitter` headers are the unmodified
generated grammar assets of
[`uyha/tree-sitter-cmake`](https://github.com/uyha/tree-sitter-cmake)
revision `e997bd0b275ca525ce9befecedf5299031183661` (`v0.7.5`),
byte-identical to the `src` files of the `tree-sitter-cmake` 0.7.5 crate.
`src/scanner.c` is that revision's scanner with `scanner-state.patch` applied.

Upstream's external scanner allocates its state with `ts_malloc` and never
initialises the remembered token, and deserialising an empty state resets only
the bracket level. The first scan therefore reads whatever the allocator
returned: a zero token means "a bracket argument is open", so during error
recovery the scanner emitted a spurious `bracket_argument_content` and the
recovery tree of `project(Demo\nset(X 1)\n` had an `ERROR` root on macOS and
Windows but `source_file` with a missing `)` on Linux and in WebAssembly. The
patch starts every parse with no bracket open; upstream's test corpus passes
with it (`experiments/issue-195-cmake-scanner` reproduces all three trees).
`node js/scripts/build-vendored-grammars.mjs --only cmake` clones the
revision, applies the patch and requires its `src/parser.c` to equal
`src/parser.c.gz`, so both runtimes compile the same scanner.

The uncompressed `parser.c` SHA-256 is
`ece92c2e3fbf15634fb90fab91d0af749348c069d27208eeb10dcdd4e77fe5a2`.
The patched `scanner.c` SHA-256 is
`172ffa925ec7e1bb38e6fc840db63636d6f83741c5e3001687f1119beea815be`.
The `parser.h` SHA-256 is
`180b893c8734778fd32f372dfbc27bd6ad1cd2221f26150b31256ff6716320d2`.

The grammar is MIT licensed; see `LICENSE`. `build.rs` expands the
deterministically compressed parser into Cargo's output directory before
compilation.
