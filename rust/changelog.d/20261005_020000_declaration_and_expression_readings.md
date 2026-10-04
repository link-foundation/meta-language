---
bump: patch
---

### Fixed

- The native grammar runtime orders TypeScript's declaration and expression readings as tree-sitter does: a node reduced to different parents ranks by the rule each reduced it to alone (`namespace N {}` before `x` is a declaration), an extra reduction ranks against the precedence that holds the results (`extends A<X>` is an extends clause, not an instantiation), a reduction's precedence is that of the last node it closed (the body of `module A {}` shifts), and a zero-width scanner token one parse takes after a node both reduced decides for that parse, as tree-sitter's lexer scans it first (`namespace A {}` before a line break is an expression statement).
