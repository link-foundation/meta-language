# Lean grammar crash on Windows (issue #195)

`issue_195_generative_lean` panicked on `windows-latest` with
`Utf8Error { valid_up_to: 0, error_len: None }` inside `Node::kind`.

- `replay.c` links tree-sitter 0.25.10 `src/lib.c` with the Lean `parser.c` and
  `scanner.c` under ASAN/UBSAN and replays the generative cases, checking each
  node's symbol and that its type name is valid UTF-8.
- `to-records.mjs` turns a trace written by the suite
  (`ISSUE_195_GENERATIVE_TRACE=/tmp/trace.jsonl cargo test --test unit issue_195_generative_lean`)
  into the binary records `replay.c` reads.

The replay is clean on Linux for LF and CRLF inputs, so the grammar does not
crash. The root cause is the compiler: generated `parser.c` files spell
non-ASCII symbol names as universal character names (`"\u00d7"`), and MSVC
without `/utf-8` encodes them in code page 1252 (`×` becomes the single byte
0xD7), so the name is not UTF-8. The fix vendors the Lean grammar so
`rust/build.rs` compiles it with `-utf-8`; the Haskell crate already passes
`-utf-8` on MSVC.

- `simulate-msvc-names.mjs` rewrites the vendored `parser.c.gz` symbol names
  the way MSVC without `/utf-8` compiles them. With it applied,
  `cargo test --test unit default_cst` fails the grammar name digest check
  (`names that are not UTF-8: Some(["\u{fffd}"])`), so the check catches the
  bug on Linux too. Restore the file with `git checkout` afterwards.

An audit of every `tree-sitter-*` crate in `rust/Cargo.toml` (symbol and field
name tables containing `\u` or high `\x` escapes, and `utf-8` in the crate's
`bindings/rust/build.rs`) found only `tree-sitter-haskell`, which already
compiles with `-utf-8` on MSVC.
