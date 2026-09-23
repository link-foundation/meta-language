---
bump: minor
---

### Added
- Add parity-tested structured frontends and lossless emitters for JavaScript,
  Rust, Lean, and Rocq/Coq in both runtime packages.
- Add real grammar CST dispatch for JavaScript, Rust, and Lean in both
  runtimes, Rocq/Coq in JavaScript, and the existing JavaScript language
  inventory; add a shared 57-target coverage ledger that keeps fallback paths
  visibly incomplete.
- Add versioned capability reports, parser extension registration in
  JavaScript, and explicit fail-closed contracts for all 12 directed language
  pairs.

### Fixed
- Prevent JavaScript identifier queries and replacements from matching text in
  string literals or comments.
- Parse JavaScript regular-expression literals and template interpolations
  structurally, and report invalid programs through grammar recovery flags.
