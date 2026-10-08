---
bump: patch
---

- Retry declaration-only modules in a complete checked scope, including JSDoc data types shared across functions.
- Preserve exact source restoration for generated declarations with standalone Rust attributes by counting target CST items.
- Verify the complete frontend decision module has zero carried items, executes its generated Rust and restores its source exactly.
