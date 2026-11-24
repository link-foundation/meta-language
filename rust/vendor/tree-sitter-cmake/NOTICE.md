# Vendored tree-sitter CMake parser

`src/parser.c.gz` and the `src/tree_sitter` headers are the unmodified
generated grammar assets of
[`uyha/tree-sitter-cmake`](https://github.com/uyha/tree-sitter-cmake)
revision `a3ec2b2144daf7132e7ee824d8cab9610523d10f` (`v0.7.6`),
with the same parser and header bytes as `v0.7.5`.
`src/scanner.c` is that revision's scanner with `scanner-state.patch` applied.

The `v0.7.5` scanner read uninitialised state, so it could emit a spurious
`bracket_argument_content` and recover `project(Demo\nset(X 1)\n` differently
across platforms. `v0.7.6` zero-initialises the state, where zero represents an
open bracket. The patch resets both fields to the explicit no-bracket state
and retains the established include paths. The patched scanner bytes match
the previous build, so the published WebAssembly oracle still uses the same
parser and scanner inputs.
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
