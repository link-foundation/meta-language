---
bump: patch
---

### Fixed

- Lean translations now emit `case ... =>` blocks after `induction`/`cases`, ordered `try` closing tactics instead of `first | ...`, and no `termination_by` for structurally decreasing recursion, so every translated Lean program reparses to a clean CST with the vendored Lean grammar while still proving in Lean 4.33.1.
