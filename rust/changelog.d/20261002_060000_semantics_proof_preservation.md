---
bump: minor
---

### Fixed
- A Rocq proof step now rewrites with each hypothesis or lemma at most eight times. An unbounded `rewrite <- ?ih` with `ih : Tree.mirror l = l` rewrote `l` into `Tree.mirror l` forever, so the Rocq translation of a false Lean theorem searched for over eleven minutes before failing. It now fails in under two seconds, and every true corpus proof still checks.

### Added
- The shared translation corpus (`parity/fixtures/four-language-conformance.json`) records, for every Lean and Rocq theorem, a false restatement and a restatement that holds only on the bounded domain of `--ml-check-theorems`. The proof preservation tests in both runtimes observe three things. Each theorem is carried as an obligation naming the source theorem, and every false restatement is rejected by the source kernel and by every target. Into Lean and Rocq each obligation is discharged by the target kernel with no axiom of the translation. Into JavaScript and Rust a theorem is a bounded check, reported apart from the source-kernel proof, which the bounded-only restatement passes while every kernel rejects it.
