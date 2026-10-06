---
bump: patch
---

### Added

- JavaScript functions no longer need JSDoc to translate: a parameter or result type JSDoc leaves out is inferred by unification from the function bodies, the calls and the top-level statements, in both the JavaScript and the Rust runtime. BigInt literals are `bigint`, `+` with a string is concatenation, a type nothing constrains is a Number, and uses that need two different types are reported as a type error with their span. A new corpus program with unannotated functions runs natively in Rust, Lean, Rocq and JavaScript with the same output as Node.
