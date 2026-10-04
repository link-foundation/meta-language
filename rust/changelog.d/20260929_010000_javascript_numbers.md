---
bump: patch
---

### Added
- JavaScript Numbers translate as IEEE-754 binary64 in both runtimes instead of being rejected, so `console.log(42)`, `console.log(6 * 7)` and `console.log(0.1 + 0.2)` are semantic translations. Rust uses `f64` with an `ml_number` prelude (ECMAScript `Number::toString`, `console.log` formatting of `-0`, SameValue); Lean uses `Float`, with Number-dependent assertions checked at run time; Rocq uses `PrimFloat`, with assertions proved by the kernel. `%` is the exact truncated remainder in every target, and `assert.strictEqual` compares with SameValue (NaN equals NaN, 0 differs from -0).
