---
bump: patch
---

### Fixed

- A token the external scanner scans with no width, such as the automatic semicolon of JavaScript, no longer hides the parse state a following keyword is lexed in, so `x\nclass` is rejected as the oracle rejects it.
