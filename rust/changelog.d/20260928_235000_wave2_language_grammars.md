---
bump: patch
---

### Added
- Default grammar CSTs for Agda, CMake, Diff, Elixir, Elm, Erlang, Groovy, HCL (Terraform), Make, MATLAB, Nix, Odin, PowerShell, Racket, Regex, Scheme and Solidity in both runtimes. Each has labels and extension dispatch (`CMakeLists.txt` and `Makefile` match by file name; Regex is label-only), a vendored WebAssembly grammar for JavaScript, a pinned crate for Rust, and structural and recovery expectations that the tree-sitter CLI cross-checks. The catalog now lists 80 languages, 67 of them with a default grammar.
