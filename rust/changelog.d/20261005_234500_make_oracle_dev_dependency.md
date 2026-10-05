---
bump: patch
---

### Changed

- tree-sitter-make no longer backs a default parse, so it is a `[dev-dependencies]` entry only: its lock entry carries `"oracle": true`, `grammar_by_id` returns nothing for `make`, and its WebAssembly build moved to `js/oracles/grammars`, outside the npm package.
