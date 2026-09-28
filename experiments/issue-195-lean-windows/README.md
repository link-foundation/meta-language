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
non-ASCII symbol names as universal character names (`"×"`), and MSVC
without `/utf-8` encodes them in code page 1252 (`×` becomes the single byte
0xD7), so the name is not UTF-8. The fix vendors the affected grammars (Lean
and Haskell) so `rust/build.rs` compiles them with `-utf-8`.
