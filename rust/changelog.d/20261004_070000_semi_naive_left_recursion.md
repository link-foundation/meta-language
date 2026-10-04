---
bump: patch
---

### Changed
- Left recursion grows each pass only from the results the pass before added or changed, so an operator chain of n links takes n passes of one seed each instead of n passes of every seed. A Rust source such as the `weird-exprs.rs` test of rustc now parses within the default step budget; every tree and ambiguity stays the same.
