---
bump: patch
---

### Fixed
- The vendored Rocq grammar (`meta-language.patch`) now parses the pinned Rocq 9.2.0 stdlib and Corelib files (Init, Classes.Morphisms, Program.Basics, Arith.PeanoNat, Bool, Lists.List, Sorting.Permutation, Structures.Orders) without error nodes: module functors and types, records, classes and instances, schemes, `Arguments` scopes, hints, obligations, subset/projection/`rew`/quotation terms, Ltac term matches, closures, `eval`, `tryif`, tacticals, rewrite multiplicities, occurrence clauses and focused goal blocks.
