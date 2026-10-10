---
bump: patch
---

### Fixed

- Import quoted ANTLR braced Unicode escapes and literal ranges in both runtimes using the generated shared decoder. Supplementary scalars and NUL are retained; malformed, surrogate and out-of-range escapes fail explicitly. This removes the quoted Unicode import failure in the pinned Python lexer without modifying its upstream grammar.
