---
bump: patch
---

### Fixed
- Automatic recovery no longer looks for a skip that ends at an external scanner token, as tree-sitter, whose recovery lexes in its error state where a scanner refuses to run, never resumes at one. A scanner such as the content of a Rust string reads to the end of the input before it fails, so the scan from every later offset was quadratic: a Rust file of a few kilobytes with one stray byte ran out of its step budget and became a single ERROR over the whole input, while it is now repaired with one ERROR at the stray byte. The JS and Rust executors skip alike.
