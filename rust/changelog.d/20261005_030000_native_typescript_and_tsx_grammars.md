---
bump: minor
---

### Added
- TypeScript and TSX parse with the native merged grammars `parity/grammars/native/typescript.lino` and `parity/grammars/native/tsx.lino` by default in both runtimes. The import pipeline writes them from the pinned `typescript/src/grammar.json` and `tsx/src/grammar.json` of tree-sitter-typescript 0.23.2, and `parity/grammars/scanners/typescript.lino` ports their shared external scanner to seven native scanners: the five of JavaScript, the automatic semicolon after a function signature and the error recovery sentinel.
- `parity/fixtures/native-grammars/typescript.json` checks the TypeScript grammar against tree-sitter-typescript on 155 matches and 15 rejections, and `parity/fixtures/native-grammars/tsx.json` the TSX grammar on 157 matches and 16 rejections, with no ambiguity. The ledger rows `I195-GRAMMAR-NATIVE-TYPESCRIPT` and `I195-GRAMMAR-NATIVE-TSX` track the JavaScript and Rust suites, and the recovery suites cover their rejections.
- The concept records, the shared concepts and concept reuse reports, the catalogs, the grammar inventory and the native grammar docs list TypeScript and TSX; 128 more concepts that three or more native grammars name are listed as shared constructs.
