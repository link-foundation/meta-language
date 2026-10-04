---
bump: patch
---

### Fixed

- The native grammar runtime forks at a shift-reduce conflict the grammar declares, as tree-sitter's `handle_conflict` leaves it to the declared conflicts: where the items that shift the next token after a left operand rank some above and some below its reduction, both parses go on and the one that reduced the operand is kept (TypeScript's `!g<T>()` calls `!g` and `await g<T>;` instantiates `await g`, as tree-sitter-typescript parses them).
