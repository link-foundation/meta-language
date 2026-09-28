---
bump: patch
---

### Fixed
- The JavaScript runtime no longer splits astral characters (UTF-16 surrogate pairs) that straddle web-tree-sitter's 5119-code-unit input chunk boundary: grammar-backed parses read the text through a chunked callback whose chunks never end on a high surrogate, so files such as `regex-syntax`'s Unicode tables parse cleanly in both runtimes. Both runtimes test every alignment around the boundary for JavaScript, Rust, Lean and Rocq.
