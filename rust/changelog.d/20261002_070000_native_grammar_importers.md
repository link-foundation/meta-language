---
bump: patch
---

### Changed
- The ABNF, BNF, EBNF, and pest importers are hand-written ports of the JavaScript importers in `js/src/grammar-importers/`. They accept the same language and report the same grammar IR and diagnostics, so the `abnf`, `bnf`, `ebnf`, and `pest_meta` crates are no longer runtime dependencies. `pest_meta` remains a development dependency, used to check emitted `.pest` grammars. Dropping these dependencies removes 22 packages from `Cargo.lock`, including `nom` 7, `rand` 0.9, and `thiserror` 1.
- An EBNF `#'...'` inline regex is reported as an unsupported construct by both importers.
