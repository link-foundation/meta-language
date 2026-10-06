---
bump: patch
---

### Fixed

- Native grammars under `(matching longest)`: where one parse ended a silent
  rule under a precedence with a token and the other shifted on in the same
  rule, which it ends later with that precedence, the associativity of the
  rule decides, right to shift and left to reduce. Lean's `set_option pp.all
  true` names the option `pp.all` instead of applying a projection `.all` of
  `pp` to `true`.
