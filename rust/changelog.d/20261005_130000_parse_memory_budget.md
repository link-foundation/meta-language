---
bump: minor
---

### Added

- Every parse has a hard memory budget. The memo cells it keeps, across its runs, repair rounds and embedded grammars, count against `FeatureParseOptions::memory_limit` (2,000,000 cells by default). A parse that needs more ends with a `memoryBudget` rejection whose message names the limit ("the parse needed more than N memo cells"), instead of growing until the process runs out of memory. The JavaScript package has the same budget as `memoryLimit`.

### Fixed

- An ANTLR lexer rule with `-> type(NAME)` is re-typed only once, so importing an emitted ANTLR grammar again gives the same grammar.
