---
bump: patch
---

### Fixed
- The native C and Rust grammars skip only ASCII white space, as tree-sitter's `\s` does: a no-break, ideographic or line separator space between items is an error, as in the tree-sitter oracles.
