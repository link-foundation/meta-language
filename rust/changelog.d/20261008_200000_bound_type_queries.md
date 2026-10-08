---
bump: minor
---

### Added

- Translate JavaScript `typeof` for typed bindings and primitive literals in both runtimes. Generated decisions retain Number/BigInt and object type names. Shared execution fixtures cover the primitive and array cases; operands with effects or exceptions retain diagnostics.
