---
bump: patch
---

### Changed
- A MISSING leaf that automatic recovery inserts now comes before the white space that precedes the repair point, at the end of the token it follows, as tree-sitter places its missing leaves, which have no padding. `{"a" 1}` is repaired with a MISSING `":"` at 4 instead of 5, and the recorded repairs of the native JSON, INI, diff, JSON5, C and Rust fixtures and the default CST rows of invalid input follow; the JS and Rust runtimes place them alike.
