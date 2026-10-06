---
bump: patch
---

### Fixed

- A left-recursive rule no longer grows again a result that a pass grew by no width, and the same-tree checks along two leftmost chains remember the pairs they compared, so Lean's `elab "a" : term => (` recovers in a fraction of the time it took and the generative Lean suite fits its CI time limit, in JavaScript and Rust.
