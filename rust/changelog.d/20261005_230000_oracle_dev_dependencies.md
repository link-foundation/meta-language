---
bump: patch
---

### Changed

- tree-sitter-go, tree-sitter-java, tree-sitter-regex, tree-sitter-graphql and tree-sitter-proto no longer back a default parse, so they are `[dev-dependencies]` only: their lock entries carry `"oracle": true`, `grammar_by_id` returns nothing for their ids, and their WebAssembly builds moved to `js/oracles/grammars`, outside the npm package.
