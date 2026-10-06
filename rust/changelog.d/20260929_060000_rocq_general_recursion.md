---
bump: patch
---

### Added

- The Rocq target translates recursion with no termination argument Rocq could check, which it used to reject, as a plain `Definition` over `ml_fix`, the function's one-step unfolding unfolded lazily to 2^64 nested calls. Every run that terminates computes the same value, so a program such as Collatz, gcd or an accumulating loop is now a semantic translation in every target. The JavaScript and Rust runtimes agree, and a corpus program's native Rust, Lean, Rocq and JavaScript runs match Node.
