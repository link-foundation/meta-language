---
bump: major
---

### Added
- Read and write every form of the grammar feature union (docs/grammar/feature-union.md) in the native listing, links and JSON forms, with grammar declarations (`GrammarDeclarations`: matching, imports, modes, extras, conflicts, macros and scanners) and rule fields (`RuleAttributes`: parameters, channel, modes and action).
- Run feature union grammars with a native executor (`compile_feature_grammar`, `FeatureGrammarParser`) that uses no parser generator. It matches `parity/fixtures/grammar-feature-union.json` byte for byte with the JavaScript executor: concrete syntax trees, ambiguities, rejections and the mutation of every feature, plus external scanners and semantic actions run as link definitions.

### Changed
- BREAKING: `GrammarExpr` has a new `Feature` variant that holds the feature union forms. Exhaustive matches on `GrammarExpr` must handle it.
- BREAKING: `GrammarLoweringStep` has new `Declarations` and `Attributes` variants, `GrammarMergeDecisionKind` has a new `DeclarationConflict` variant and `GrammarMergeAlternativeReason` has a new `DeclarationConflict` variant. Exhaustive matches on these enums must handle them.

### Fixed
- `merge_grammars` no longer drops grammar declarations and rule fields. It renames them with the rules, unites imports, modes, extras and conflict groups, keeps the first matching, macro and scanner of a name and reports a different later one as a `declaration-conflict` decision and alternative. Rules that differ only in their parameters, channel, modes or action stay distinct.
- `lower_grammar` no longer drops grammar declarations and rule fields. It records them as `declarations` and `attributes` metadata steps, which makes the lowering `approximate`; `reconstruct_grammar` restores them, and `check_grammar_lowering` reports `feature-dropped` when they are missing.
