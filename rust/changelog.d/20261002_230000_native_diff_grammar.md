---
bump: patch
---

### Added
- A native merged unified diff grammar in Links Notation, `parity/grammars/native/diff.lino`. It merges tree-sitter-diff 0.1.0, the GNU diffutils unified format and the git patch format: it builds the tree-sitter-diff trees of blocks, headers, hunks and changes, reads a hunk line by its first character as GNU diff does, accepts abbreviated object names of four to forty hex digits and a block cut at the end of the input, and keeps line breaks and blank lines as named leaves. Both native executors run it with no ambiguity. The default Diff parse still uses tree-sitter-diff until the grammar has recovery rules.
- `parity/fixtures/native-grammars/diff.json` checks the grammar against tree-sitter-diff on 115 matches, 8 GNU and git divergences and 14 rejections. The ledger row `I195-GRAMMAR-NATIVE-DIFF` tracks the JavaScript and Rust suites.

### Fixed
- The native grammar fixtures count a source as recovered by the oracle when the root's has-error flag is set, not only when a row has an error or missing flag: tree-sitter can insert a missing line break at the end of the input that no row shows. `oracleRecovers` in `js/scripts/native-grammar-rows.mjs` checks both, and the JSON, INI and Diff suites use it.
- The dependency inventory records emscripten/emsdk 6.0.11 as the newest image; the delivered image stays 4.0.15, held by tree-sitter 0.27.0.
