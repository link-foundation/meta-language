---
bump: patch
---

### Fixed

- A rule of a left-associative precedence whose sequence ends in optional parts is reduced before a token those parts could begin with where that token may also follow the rule, as an LR parser reduces it, so Rust's `break -1` is a binary expression of `break` and `1`, and Lean's `#eval x[i]!` reads as the oracle reads it.
- A token a silent rule reduced alone where the other parse shifted it parts the two parses on that token, before either node is reduced, so Lean's `f a.b` is read as the oracle reads it.
- An alias shared by tokens of different content (Lean's `unnamed_token`) names no lookahead such a reduction is decided by, and a conflict the grammar declares between the rule and a rule that begins with the token keeps both parses (Lean's `hash_command` and `explicit`), so `#check @ident` is read.
