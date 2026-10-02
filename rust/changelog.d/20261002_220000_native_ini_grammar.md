---
bump: patch
---

### Added
- A native merged INI grammar in Links Notation, `parity/grammars/native/ini.lino`. It merges tree-sitter-ini 1.4.0 and the Python configparser INI file structure: it builds the tree-sitter-ini trees, comments as extras included, accepts a last comment line without a line break, and keeps blank lines, line breaks and comment markers as named leaves. Both native executors run it with no ambiguity. The default INI parse still uses tree-sitter-ini until the grammar has recovery rules.
- `parity/fixtures/native-grammars/ini.json` checks the grammar against tree-sitter-ini on 55 matches, 9 configparser divergences and 15 rejections. A last line without a line break is a divergence: tree-sitter-ini completes it with a missing line break, which only the has-error flag of its root shows. The ledger row `I195-GRAMMAR-NATIVE-INI` tracks the JavaScript and Rust suites.
- The oracle row projection reads two more fixture fields: `anonymous` leaves are not rows but count in the spans, and `extras` nodes are rows with flag X. The Rust projection moved to `rust/tests/unit/issue_195_native_grammar_rows.rs`, shared by the JSON and INI suites.

### Fixed
- The acceptance manifest pins the current digest of the dependency inventory after the libc update.
