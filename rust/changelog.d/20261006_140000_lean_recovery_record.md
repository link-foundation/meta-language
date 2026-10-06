---
bump: patch
---

### Fixed

- The native Lean recovery of the generative case `:f f : Foo where\n  bar :=⟩` now builds the oracle's `tactic_apply` over an `application`, because a precedence conflict compares the parts inside first. Its record in `parity/fixtures/native-recovery.json` is updated to match. The repair sites are unchanged.
