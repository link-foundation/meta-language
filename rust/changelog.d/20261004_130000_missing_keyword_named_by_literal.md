---
bump: patch
---

### Fixed
- A MISSING leaf that automatic recovery inserts for a keyword token (a literal and the lookaheads after it, as a merged native grammar writes `return`) is now named by its literal, as tree-sitter names its keyword token, instead of having no kind, which the canonical CST lines printed as `MISSING null`. `oc =c =>=` now holds `(return_expression (MISSING@7 "return"))`; the recorded repairs of the native C and Rust fixtures follow, and the JS and Rust executors name it alike.
