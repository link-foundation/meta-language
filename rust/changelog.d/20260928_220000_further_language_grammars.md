---
bump: patch
---

### Added
- Default grammar CSTs for Haskell, OCaml, OCaml interfaces, Zig, Bash and Dart in both runtimes, through `tree-sitter-haskell`, `tree-sitter-ocaml`, `tree-sitter-zig`, `tree-sitter-bash` and `tree-sitter-dart`. Each language has labels, file extensions, a vendored WebAssembly grammar for JavaScript, and pinned structural and recovery CST expectations checked against the tree-sitter CLI.
