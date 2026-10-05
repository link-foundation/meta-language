---
bump: patch
---

### Fixed

- Native recovery inserts a MISSING token only where a rule reduces before the next token, as tree-sitter does: no part of the same rule after a MISSING leaf takes input (TypeScript's `[ 0 .92 ]` misses no `,` before `.92` and builds no `as_expression`).
- The public tree has no node for a MISSING token of a hidden or anonymous kind, though the node it is missing from still has an error.
- A repair round that completes nothing asks for the offset its farthest result stopped at as the next repair point, and a round that runs out of steps ends the rounds with the previous partial tree.
