---
bump: patch
---

Preserve separate statements after prefixed Python strings, restore enclosing
formatted-string scanner state after nested strings, and prefer format tokens
over longer comments when their lexical precedence is higher. Reconcile
exception-group markers and collection splat suffixes through recorded import
rules, retaining the original grammar alternatives. Add shared focused tree
regressions for both runtime executors.
