---
bump: patch
---

### Fixed
- The Rocq grammar parses Recdef's `Function`, mutually recursive and redefining `Ltac` definitions, the `first [ ... ]` and `solve [ ... ]` tacticals and Ltac `let ... in` without error nodes, so translated Rocq programs round-trip through clean CSTs. The vendored parser is regenerated from upstream with `meta-language-tactics.patch`.
