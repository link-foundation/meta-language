---
bump: patch
---

### Changed
- The issue 195 acceptance workflow installs Rust 1.99.0, and the four-language contracts, the translation dependency lists and the toolchain evidence declare Rust 1.99.0.
- The grammar crates follow their current releases: `tree-sitter-graphql` 0.3.0 and `tree-sitter-haskell` 0.24.1, with the JavaScript WebAssembly parsers rebuilt from the same crates by tree-sitter CLI 0.27.0 so both runtimes parse with the same generated parsers. `yoke-derive` follows 0.8.4.
