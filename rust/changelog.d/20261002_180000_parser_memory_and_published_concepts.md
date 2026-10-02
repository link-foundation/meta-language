---
bump: patch
---

### Added
- `CROSS_FORMAT_CONCEPT_IDS` lists the readable cross-format concept ids. `CROSS_FORMAT_CONCEPTS` keeps the identities published before the naming migration, such as `strong`. `LanguageProfile::supports_concept` and `LanguageProfile::concept_fallback` accept either spelling, so callers written against the published list (formal-ai's `issue_425` tests) keep resolving.
- The formal-ai workload regenerates formal-ai's self-AST census with formal-ai's `regenerate_self_ast_census` example against the candidate crate before running its tests. The probe fails if the refresh changes anything except a document's `total_link_count`. That count grows deliberately, from the grammar provenance link and the `hidden_text` tokens split from grammar gap text.

### Fixed
- The JavaScript parser compiles a vendored grammar when it is first used, not all 60 at import.
- V8 tiered the grammars' generated lexers up to TurboFan in the background. Some lexers are single functions of 130 to 360 KB (PowerShell, Swift, Markdown and its inline grammar), and each tier-up took 0.5 to 2 GB, enough to kill whole-suite runs after every parse had returned. The parser now compiles grammar modules with a tiering budget that no parse exhausts, so their code stays on V8's baseline compiler. The runtime and every other module keep the process's budget. `check:default-cst` loads its grammars the same way.
- The JavaScript parser maps string offsets to UTF-8 bytes and points through one typed-array checkpoint per 64 characters with a binary search. It no longer builds an object per source character. Markdown inline regions share one inline parser and still delete each tree.
- AGENTS.md and CONTRIBUTING.md list the targeted local checks, the resource limits for local runs, and the suites that run only in CI.
