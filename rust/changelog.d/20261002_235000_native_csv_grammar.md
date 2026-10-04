---
bump: patch
---

### Added
- A native merged CSV grammar in Links Notation, `parity/grammars/native/csv.lino`. It merges tree-sitter-csv (revision `f6bf6e3` with the RFC 4180 quotes patch) and RFC 4180: it builds the tree-sitter-csv trees of rows and of number, float, boolean and text fields, reads quoted fields with commas, line breaks and doubled quotes, accepts an empty last field at the end of the input, and keeps line breaks and the spaces after a closing quote as named leaves. Both native executors run it with no ambiguity. The default CSV parse still uses tree-sitter-csv until the grammar has recovery rules.
- `parity/fixtures/native-grammars/csv.json` checks the grammar against tree-sitter-csv on 123 matches, 8 RFC 4180 divergences and 15 rejections. The ledger row `I195-GRAMMAR-NATIVE-CSV` tracks the JavaScript and Rust suites.
