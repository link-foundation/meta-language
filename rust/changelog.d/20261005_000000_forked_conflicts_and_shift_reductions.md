---
bump: patch
---

### Fixed

- The native grammar runtime settles two more parse conflicts as tree-sitter does: a shift into a right operand whose first part an LR parser would reduce below the operator first (TypeScript's `<C>e.f` asserts the type of `e.f`), the reduction a shift needs before it (`keyof U & V`), and two parses that fork at a conflict the grammar declares, where the rule defined first is kept (`<A>(a): T => a` is an arrow function). A token's scanner item (TypeScript's `function_signature_automatic_semicolon`) is now requested where the parse asks for it, so a scanner's `expected` sees it.
