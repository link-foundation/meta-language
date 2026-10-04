---
bump: patch
---

### Changed
- tree-sitter-typescript is a development dependency only: the native merged grammars are the default TypeScript and TSX parses, `grammar_by_id("typescript")` and `grammar_by_id("tsx")` return nothing, and the pinned oracles are loaded only by `pinned_typescript_oracle_gives_the_fixture` and `pinned_tsx_oracle_gives_the_fixture`.
