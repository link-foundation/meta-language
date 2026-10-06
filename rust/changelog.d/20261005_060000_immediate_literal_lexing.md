---
bump: patch
---

### Fixed

- Native grammars under `(matching longest)`: a literal the grammar also
  takes as an immediate token (`(immediateToken (literal [))`) is no longer
  lexed plainly where the immediate one is valid, as a tree-sitter lexer
  prefers the immediate token there. Lean's `foo[1:2:3]` opens a subscript
  (and is rejected) instead of applying `foo` to a range.
