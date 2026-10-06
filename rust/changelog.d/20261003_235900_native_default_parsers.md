---
bump: minor
---

### Added
- The native merged grammars are the default parsers of JSON, INI, Diff, CSV, JSON5, Scheme and Racket. `parse_programming_language`, embedded regions and incremental reparses run the catalog's native grammar with automatic error recovery and project its tree as the pinned tree-sitter oracle places nodes, so valid input keeps the oracle's default CST rows and invalid input gets the native repair. The crate ships the grammars under `src/data/native-grammars/`, and the catalog lists each native grammar first in `grammars`, the tree-sitter grammars in `oracleGrammars`, and the projection kinds in `nativeGrammars`. New public items: `NativeGrammarEntry`, `native_grammar`, `native_grammars` and `oracle_grammar_provenance`.
