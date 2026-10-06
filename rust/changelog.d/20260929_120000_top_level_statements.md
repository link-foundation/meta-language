---
bump: patch
---

### Added

- Translate top-level JavaScript `let`, assignments, `if`, blocks and loops, in both runtimes: the statements before each print, constant or assertion are lowered as a function body is, and main binds each top-level variable they declare or assign, with a corpus program whose native Rust, Lean, Rocq and JavaScript runs match Node.

### Fixed

- A variable named `label` is no longer rejected as a statement keyword.
