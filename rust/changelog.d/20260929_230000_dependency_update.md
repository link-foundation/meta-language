---
bump: minor
---

### Changed

- Every Rust dependency is updated to the latest release its version requirement allows (`cargo update`), including `clap` 4.6, `pest` 2.9 and `zerovec` 0.11.8.
- The minimum supported Rust version is now 1.90, the highest `rust-version` any locked dependency declares (`tree-sitter-language` 0.1.8). A new `Minimum Supported Rust Version` CI job builds the library, binaries and tests with that toolchain.
