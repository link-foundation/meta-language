---
bump: patch
---

### Added
- A native merged JSON grammar in Links Notation, `parity/grammars/native/json.lino`. It merges RFC 8259, ECMA-404 and tree-sitter-json 0.24.8: it accepts comments, several top-level values and a plus sign in an exponent, and it keeps a leading byte order mark as a `byte_order_mark` leaf. Both native executors run it. The default JSON parse still uses tree-sitter-json until the grammar has recovery rules.
- `parity/fixtures/native-grammars/json.json` and `js/scripts/generate-native-grammar-fixtures.mjs` check the grammar against tree-sitter-json: the native rows equal the oracle rows on every match, divergences carry their RFC 8259 reason, and invalid sources are rejected. `npm run check:native-grammars` fails on a stale fixture, and CI runs it. The ledger row `I195-GRAMMAR-NATIVE-JSON` tracks the JavaScript and Rust suites.
- `docs/grammar/native-grammars.md` describes the format, the merge and the oracle check.

### Fixed
- The JavaScript UTF-8 decoders keep a leading byte order mark (`ignoreBOM: true`). Before, a Links Notation grammar with the literal `%EF%BB%BF` lost it on reading, and the text of a leaf starting at byte 0 dropped it.
