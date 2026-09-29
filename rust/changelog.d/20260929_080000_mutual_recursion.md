---
bump: patch
---

### Added

- The Lean and Rocq targets translate mutually recursive functions, which they used to reject. Lean writes each group as a `mutual` block of `partial def`s. Rocq writes it as one `ml_fix` over the sum of the functions' parameter tuples, with each function a plain `Definition` projecting the whole, so every run that terminates computes the same value. Both runtimes agree, and a corpus program (even and odd, Hofstadter's female and male sequences, and three functions taking turns) has native Rust, Lean, Rocq and JavaScript runs that match Node.
