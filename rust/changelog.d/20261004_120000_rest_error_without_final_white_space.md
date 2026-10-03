---
bump: patch
---

### Fixed
- The ERROR leaf that automatic recovery makes of the rest of the input no longer covers the white space at the end of the input, which now follows it as separators, as tree-sitter keeps that white space out of its ERROR nodes. A stray `]` before the final newline is now `ERROR@61..62` instead of `ERROR@61..63`; two recorded repairs of the native Rust fixture follow, and the JS and Rust executors trim alike.
