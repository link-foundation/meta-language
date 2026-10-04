---
bump: minor
---

### Changed
- tree-sitter-c is no longer a production dependency: C parses with the native merged grammar by default, so the crate is a development dependency that pins the oracle in the tests, and `grammar_by_id` and `grammar_names` return `None` for the `c` oracle id.
