---
bump: patch
---

### Fixed

- Keep the generated local OCaml toolchain out of source checkout status so acceptance suites can run after installing Rocq. Evidence still rejects source edits and untracked tests, and reports the offending paths when the checkout is dirty.
