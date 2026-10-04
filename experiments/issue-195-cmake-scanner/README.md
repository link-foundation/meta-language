# tree-sitter-cmake scanner initial state

`cargo run` builds the tree-sitter-cmake 0.7.5 parser with a copy of its
external scanner whose initial state is chosen at build time, then prints the
tree of the issue 195 CMake recovery row (`project(Demo\nset(X 1)\n`) and of a
bracket argument/comment sample.

    INITIAL_TOKEN=garbage CARGO_TARGET_DIR=/tmp/cmake-probe-target cargo run -q
    INITIAL_TOKEN=zero    CARGO_TARGET_DIR=/tmp/cmake-probe-target cargo run -q
    INITIAL_TOKEN=none    CARGO_TARGET_DIR=/tmp/cmake-probe-target cargo run -q

Upstream's `tree_sitter_cmake_external_scanner_create` returns uninitialised
`ts_malloc` memory and an empty `deserialize` resets only the bracket level,
so the first scan reads an arbitrary remembered token:

| Initial state | Recovery tree |
| --- | --- |
| `zero` (token = `BRACKET_ARGUMENT_OPEN`) | `(ERROR (identifier) (argument_list …) (bracket_argument_content))`, as on macOS and Windows CI |
| `garbage` (0xA5 bytes) | `(source_file (normal_command … (MISSING ")")))`, as on Linux and WebAssembly |
| `none` (no bracket open, the vendored patch) | `(source_file (normal_command … (MISSING ")")))` |

Bracket arguments and comments parse identically in all three. The vendored
copy in `rust/vendor/tree-sitter-cmake` applies `scanner-state.patch`, which
starts every parse in the `none` state.
