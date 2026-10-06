---
bump: minor
---

### Added

- Translation: string `trim`/`trimStart`/`trimEnd` translate into Rust with the JavaScript whitespace set (Unicode White_Space without U+0085, plus U+FEFF). Lean and Rocq refuse them with a reason.
- Translation: the lexer reads `\u{…}` escapes, and in JavaScript also `\uXXXX` escapes, where a surrogate pair becomes one code point. A lone surrogate is refused, because Rust strings cannot hold it.
