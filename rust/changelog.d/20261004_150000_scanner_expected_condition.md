---
bump: patch
---

### Added
- A grammar scanner may ask `(expected ITEM)`: whether the parse requested a literal or a rule at the scanner's context offset, as a tree-sitter scanner reads `valid_symbols`. A parse that answered before a later request at the same offset runs again with the requests so far. The Rust runtime answers it as the JavaScript runtime does, and the grammar feature union fixture's layout feature exercises it with a mutation and two load errors.
