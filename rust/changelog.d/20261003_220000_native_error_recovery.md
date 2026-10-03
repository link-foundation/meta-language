---
bump: minor
---

### Added
- Automatic error recovery in the native grammar executor. With `FeatureParseOptions::error_recovery` (`errorRecovery` in JavaScript) a parse the grammar rejects is repaired without recovery rules in the grammar: at the farthest failing element the executor inserts a zero-width MISSING leaf or skips to the element's next match behind an ERROR leaf, whichever costs less, in rounds of repair points bounded by `max_repairs` (default 32). The tree stays lossless and is reported as `recovered`. Every rejection of the seven native grammar fixtures records its repaired tree, and both executors build the same trees. The ledger row `I195-GRAMMAR-NATIVE-RECOVERY` tracks the JavaScript and Rust suites.
