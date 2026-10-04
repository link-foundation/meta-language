---
bump: patch
---

### Changed
- tree-sitter-javascript is a development dependency only: the native merged grammar is the default JavaScript parse, `grammar_by_id("javascript")` returns nothing, and the pinned oracle is loaded only by `pinned_javascript_oracle_gives_the_fixture`.
