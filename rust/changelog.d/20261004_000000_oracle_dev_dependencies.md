---
bump: minor
---

### Changed
- The tree-sitter grammars of the languages the native grammars parse by default (tree-sitter-json, tree-sitter-ini, tree-sitter-diff, tree-sitter-json5-orchard, tree-sitter-scheme, tree-sitter-racket and the vendored tree-sitter-csv) are no longer production dependencies: the crates are development dependencies that pin the oracle in the tests, and the vendored CSV parser is no longer compiled or published. `grammar_by_id` and `grammar_names` return `None` for these oracle ids; the native grammar is the parse of these languages.
