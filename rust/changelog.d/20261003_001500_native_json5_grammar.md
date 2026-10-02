---
bump: patch
---

### Added
- A native merged JSON5 grammar in Links Notation, `parity/grammars/native/json5.lino`. It merges tree-sitter-json5-orchard 0.1.0 and the JSON5 1.0.0 specification: it builds the tree-sitter-json5-orchard trees of objects with quoted and unquoted member names, arrays, single- and double-quoted strings, hexadecimal, signed, `Infinity` and `NaN` numbers, trailing commas and line and block comments, and it reads the JSON5 white space (NBSP, LS, PS, a byte order mark and every Zs space), ECMAScript 5.1 identifier names and string escapes the oracle recovers from. Both native executors run it with no ambiguity. The default JSON5 parse still uses tree-sitter-json5-orchard until the grammar has recovery rules.
- `parity/fixtures/native-grammars/json5.json` checks the grammar against tree-sitter-json5-orchard on 120 matches, 26 JSON5 specification divergences and 36 rejections. The ledger row `I195-GRAMMAR-NATIVE-JSON5` tracks the JavaScript and Rust suites.

### Changed
- uuid is updated to 1.27.0, released on 2026-10-02, so the delivered lockfile stays on the current stable release the dependency audit requires.
