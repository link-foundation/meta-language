# Unmapped requirements — detailed continuation checklist

Planning companion to [the continuation plan](issue-227-remaining-work-plan-2026-10-31.md). This file records future work, not behavior evidence or a completion report. Snapshot commit: `9405c25fbbcb101d39953e188608bcd6d8734664`; manifest SHA-256: `3693e490c73507b0850bb9ee4a6d2aac540b8dac5d3e607f61f74fe8f9d756ae`. All exact obligations and deferred verification declarations below come from the existing atomic manifest; they are not weakened or replaced here.

80 requirements in this planning category. Checkboxes are intentionally unchecked until a source review or later execution establishes their condition. A missing mapping does not establish absence of code, and a present mapping does not establish full drafting.

## BASELINE — Own-code JavaScript ↔ Rust baseline

5 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#baseline). Candidate locations: [js/src/self-translation.js](../../js/src/self-translation.js), [js/src/project-translation.js](../../js/src/project-translation.js), [js/src/project-translation-archive.js](../../js/src/project-translation-archive.js), [js/src/translation](../../js/src/translation), [rust/src/self_translation.rs](../../rust/src/self_translation.rs), [rust/src/project_translation.rs](../../rust/src/project_translation.rs), [docs/self-translation.md](../../docs/self-translation.md).

### I195-SEMANTICS-CONSTRUCT-INVENTORY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [BASELINE — Own-code JavaScript ↔ Rust baseline](issue-227-remaining-work-plan-2026-10-31.md#baseline) · P1.

**Exact required behavior:** The four-language inventory covers classes and objects, arrays and mutation, standard-library behavior, imports, higher-order functions, asynchronous behavior, ownership, macros and notation, types, universes, effects, proofs and tactics. Every entry is represented and transformed through the common links representation, and normal JavaScript is accepted without annotations.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = full JavaScript, Rust, Lean and Rocq construct and project inventory · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#four-language-semantics-and-translation](../../docs/vision.md#four-language-semantics-and-translation).
**Registered inventories:** [parity/fixtures/four-language-conformance.json](../../parity/fixtures/four-language-conformance.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [BASELINE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#baseline) for this construct: **full JavaScript, Rust, Lean and Rocq construct and project inventory**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-semantics-construct-inventory-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-semantics-construct-inventory`.
  - Observable assertions to implement/control: `everyConstructRepresented`, `everyConstructTransformed`, `projectsTransformed`, `normalJavaScriptAccepted`.
  - Future commit-bound artifact: `issue-195-results/i195-semantics-construct-inventory-javascript-behavior.json` (not current evidence).
- `i195-semantics-construct-inventory-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-semantics-construct-inventory`.
  - Observable assertions to implement/control: `everyConstructRepresented`, `everyConstructTransformed`, `projectsTransformed`, `normalJavaScriptAccepted`.
  - Future commit-bound artifact: `issue-195-results/i195-semantics-construct-inventory-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-TRANSLATION-OWN-CODEBASE-BASELINE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [BASELINE — Own-code JavaScript ↔ Rust baseline](issue-227-remaining-work-plan-2026-10-31.md#baseline) · P1.

**Exact required behavior:** Deliver full JavaScript and Rust parsing and source emission, with complete concrete syntax by default, and translation between both languages for the real project codebase as the most critical end-to-end baseline. Validate complete source inventories, lossless source reconstruction and executable semantics independently; carried original source, registration or declarations do not establish semantic translation. Complete the remaining languages and capabilities after this baseline while preserving every inherited requirement.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete JavaScript Rust parsing emission and bidirectional translation as the first own-codebase baseline · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [BASELINE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#baseline) for this construct: **complete JavaScript Rust parsing emission and bidirectional translation as the first own-codebase baseline**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-translation-own-codebase-baseline-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:automationFirstOwnCodebaseBaselineRequest`.
  - Observable assertions to implement/control: `javascriptRustCompleteParsingAndEmissionPrioritized`, `realOwnSourceInventoryExercised`, `bidirectionalExecutableTranslationChecked`, `defaultConcreteSyntaxAndLosslessSourceRetained`, `remainingScopePreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-own-codebase-baseline-javascript-behavior.json` (not current evidence).
- `i195-translation-own-codebase-baseline-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:automationFirstOwnCodebaseBaselineRequest`.
  - Observable assertions to implement/control: `javascriptRustCompleteParsingAndEmissionPrioritized`, `realOwnSourceInventoryExercised`, `bidirectionalExecutableTranslationChecked`, `defaultConcreteSyntaxAndLosslessSourceRetained`, `remainingScopePreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-own-codebase-baseline-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-TRANSLATION-PRIORITY-JAVASCRIPT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [BASELINE — Own-code JavaScript ↔ Rust baseline](issue-227-remaining-work-plan-2026-10-31.md#baseline) · P1.

**Exact required behavior:** Current implementation priority maximizes JavaScript to editable meta-language and meta-language to JavaScript translation, retaining complete source, structures and semantics while preserving every earlier requirement.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = immediate bidirectional JavaScript and meta-language translation priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/bidirectional-translation-priority-2026-10-12.txt](../../parity/sources/bidirectional-translation-priority-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/bidirectional-translation-priority-2026-10-12.txt](../../parity/sources/bidirectional-translation-priority-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [BASELINE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#baseline) for this construct: **immediate bidirectional JavaScript and meta-language translation priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-translation-priority-javascript-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:translationPriorityRequest`.
  - Observable assertions to implement/control: `javascriptToMetaLanguageExecuted`, `metaLanguageToJavascriptExecuted`, `javascriptPriorityPreservesInheritedScope`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-javascript-javascript-behavior.json` (not current evidence).
- `i195-translation-priority-javascript-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:translationPriorityRequest`.
  - Observable assertions to implement/control: `javascriptToMetaLanguageExecuted`, `metaLanguageToJavascriptExecuted`, `javascriptPriorityPreservesInheritedScope`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-javascript-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-TRANSLATION-PRIORITY-MULTI-PROJECT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [BASELINE — Own-code JavaScript ↔ Rust baseline](issue-227-remaining-work-plan-2026-10-31.md#baseline) · P1.

**Exact required behavior:** The same automated translation and common concept algorithms operate on multiple real JavaScript and Rust projects with pinned inputs and independent behavioral validation. Project-specific source rewrites or carried originals do not establish successful reusable executable translation.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = reusable source translation across multiple JavaScript and Rust projects · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/multi-project-translation-reuse-2026-10-12.txt](../../parity/sources/multi-project-translation-reuse-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/multi-project-translation-reuse-2026-10-12.txt](../../parity/sources/multi-project-translation-reuse-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [BASELINE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#baseline) for this construct: **reusable source translation across multiple JavaScript and Rust projects**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-translation-priority-multi-project-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:multiProjectTranslationRequest`.
  - Observable assertions to implement/control: `multipleRealProjectsTranslated`, `sharedTranslationAlgorithmsReused`, `projectBehaviorsIndependentlyValidated`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-multi-project-javascript-behavior.json` (not current evidence).
- `i195-translation-priority-multi-project-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:multiProjectTranslationRequest`.
  - Observable assertions to implement/control: `multipleRealProjectsTranslated`, `sharedTranslationAlgorithmsReused`, `projectBehaviorsIndependentlyValidated`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-multi-project-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-TRANSLATION-PRIORITY-RUST

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [BASELINE — Own-code JavaScript ↔ Rust baseline](issue-227-remaining-work-plan-2026-10-31.md#baseline) · P1.

**Exact required behavior:** Current implementation priority maximizes Rust to editable meta-language and meta-language to Rust translation, including complete source and semantic preservation obligations. Earlier languages, knowledge-format support and inherited requirements remain in scope.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = immediate bidirectional Rust and meta-language translation priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/bidirectional-translation-priority-2026-10-12.txt](../../parity/sources/bidirectional-translation-priority-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/bidirectional-translation-priority-2026-10-12.txt](../../parity/sources/bidirectional-translation-priority-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [BASELINE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#baseline) for this construct: **immediate bidirectional Rust and meta-language translation priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-translation-priority-rust-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:translationPriorityRequest`.
  - Observable assertions to implement/control: `rustToMetaLanguageExecuted`, `metaLanguageToRustExecuted`, `rustPriorityPreservesInheritedScope`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-rust-javascript-behavior.json` (not current evidence).
- `i195-translation-priority-rust-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:translationPriorityRequest`.
  - Observable assertions to implement/control: `rustToMetaLanguageExecuted`, `metaLanguageToRustExecuted`, `rustPriorityPreservesInheritedScope`.
  - Future commit-bound artifact: `issue-195-results/i195-translation-priority-rust-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## GRAMMAR — Full CST grammars and source unions

9 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#grammar). Candidate locations: [js/src/native-grammar-parser.js](../../js/src/native-grammar-parser.js), [js/src/grammar-lowering.js](../../js/src/grammar-lowering.js), [js/src/grammar-reverse.js](../../js/src/grammar-reverse.js), [js/src/data/native-grammars](../../js/src/data/native-grammars), [rust/src/native_grammar_parser.rs](../../rust/src/native_grammar_parser.rs), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino).

### I195-GRAMMAR-DEPENDENCY-BOUNDARY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** The installed production packages parse every catalog language with every external grammar engine, subprocess, foreign generated parser, download and oracle asset absent. Changing one native linked grammar rule changes the output of the public parse path without a host-code patch.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = native dependency boundary of the installed packages · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#native-merged-grammars](../../docs/vision.md#native-merged-grammars).
**Registered inventories:** [docs/vision.md](../../docs/vision.md).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **native dependency boundary of the installed packages**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-dependency-boundary-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-dependency-boundary`.
  - Observable assertions to implement/control: `parsesWithExternalEnginesAbsent`, `noSubprocessOrDownloadAtParseTime`, `nativeRuleChangeChangesParse`, `generatedAccelerationDerivedFromNativeGrammar`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-dependency-boundary-javascript-behavior.json` (not current evidence).
- `i195-grammar-dependency-boundary-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-dependency-boundary`.
  - Observable assertions to implement/control: `parsesWithExternalEnginesAbsent`, `noSubprocessOrDownloadAtParseTime`, `nativeRuleChangeChangesParse`, `generatedAccelerationDerivedFromNativeGrammar`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-dependency-boundary-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-LANGUAGE-CATALOG

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** The versioned catalog lists every popular language with its versions or editions, dialects, aliases, extensions and embedded languages. For each language it inventories the official, ANTLR grammars-v4, tree-sitter and other mature grammar sources with pinned revisions, licenses, provenance and independent test corpora, reconciled with every dispatcher and downstream requirement.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = versioned language catalog with the strongest grammar sources per language · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **versioned language catalog with the strongest grammar sources per language**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-language-catalog-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-language-catalog`.
  - Observable assertions to implement/control: `everyLanguageVersioned`, `everyApplicableGrammarSourceInventoried`, `sourcesPinnedWithLicenseAndProvenance`, `independentCorporaPinned`, `reconciledWithDispatchersAndConsumers`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-language-catalog-javascript-behavior.json` (not current evidence).
- `i195-grammar-language-catalog-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-language-catalog`.
  - Observable assertions to implement/control: `everyLanguageVersioned`, `everyApplicableGrammarSourceInventoried`, `sourcesPinnedWithLicenseAndProvenance`, `independentCorporaPinned`, `reconciledWithDispatchersAndConsumers`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-language-catalog-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-LOSSLESS-TREES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Native grammar parsing of every catalog language yields a lossless concrete syntax tree with node kinds, fields, hierarchy, exact tokens, trivia, spans, error and missing nodes and embedded-language boundaries. The optional abstract tree keeps provenance to the concrete tree, and both trees can be serialized, reloaded without the source, edited, generated back to source and reparsed.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = full lossless concrete syntax trees from native grammars, with optional abstract trees · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **full lossless concrete syntax trees from native grammars, with optional abstract trees**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-lossless-trees-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-lossless-trees`.
  - Observable assertions to implement/control: `nativeTreesLossless`, `fieldsAndSpansExact`, `errorAndMissingNodesRecovered`, `abstractTreeProvenanceKept`, `treesReloadedWithoutSource`, `editedTreesGeneratedAndReparsed`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-lossless-trees-javascript-behavior.json` (not current evidence).
- `i195-grammar-lossless-trees-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-lossless-trees`.
  - Observable assertions to implement/control: `nativeTreesLossless`, `fieldsAndSpansExact`, `errorAndMissingNodesRecovered`, `abstractTreeProvenanceKept`, `treesReloadedWithoutSource`, `editedTreesGeneratedAndReparsed`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-lossless-trees-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.
**Downstream obligations:** link-assistant/formal-ai; link-foundation/relative-meta-logic.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-NATIVE-MERGED

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** The shipped grammar of every catalog language is a native Links Notation grammar merged from all of its inventoried sources. Default parsing executes it, not a wrapped external engine, a copied generated parser or a stored foreign grammar string.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = native Links Notation grammar as the shipped canonical grammar of every language · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#native-merged-grammars](../../docs/vision.md#native-merged-grammars).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **native Links Notation grammar as the shipped canonical grammar of every language**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] For meta-language runtime packages, audit every advertised alias/edition and pinned grammar-source union, scanner/recovery family and complete CST fields; author a construct-by-construct accepted/rejected original corpus rather than a representative sample.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-native-merged-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-native-merged`.
  - Observable assertions to implement/control: `everyLanguageHasNativeGrammar`, `nativeGrammarMergedFromAllSources`, `defaultParseExecutesNativeGrammar`, `noForeignGrammarStoredAsString`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-native-merged-javascript-behavior.json` (not current evidence).
- `i195-grammar-native-merged-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-grammar-native-merged`.
  - Observable assertions to implement/control: `everyLanguageHasNativeGrammar`, `nativeGrammarMergedFromAllSources`, `defaultParseExecutesNativeGrammar`, `noForeignGrammarStoredAsString`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-native-merged-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-PRIORITY-JAVASCRIPT-COMPLETE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Prioritize a complete JavaScript grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete JavaScript grammar as an initial delivery priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **complete JavaScript grammar as an initial delivery priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-priority-javascript-complete-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `javascriptGrammarPrioritized`, `javascriptCompleteConcreteSyntaxValidated`, `javascriptOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-javascript-complete-javascript-behavior.json` (not current evidence).
- `i195-grammar-priority-javascript-complete-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `javascriptGrammarPrioritized`, `javascriptCompleteConcreteSyntaxValidated`, `javascriptOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-javascript-complete-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-PRIORITY-MARKDOWN-COMPLETE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Prioritize a complete Markdown grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete Markdown grammar as an initial delivery priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **complete Markdown grammar as an initial delivery priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-priority-markdown-complete-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `markdownGrammarPrioritized`, `markdownCompleteConcreteSyntaxValidated`, `markdownOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-markdown-complete-javascript-behavior.json` (not current evidence).
- `i195-grammar-priority-markdown-complete-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `markdownGrammarPrioritized`, `markdownCompleteConcreteSyntaxValidated`, `markdownOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-markdown-complete-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-PRIORITY-OWN-SOURCE-TRANSLATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Deliver full grammars first for JavaScript, Markdown, Rust, TypeScript and the other languages used by these projects to enable the best available reusable tools and automated translation of their own code through the common editable links representation. This priority preserves the full scope for other languages and does not equate a concrete parse or carried original with executable semantic translation.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete project language grammars for translation of our own code · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **complete project language grammars for translation of our own code**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-priority-own-source-translation-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `ownProjectLanguagesReceiveInitialGrammarPriority`, `ownCodeUsesCommonEditableTranslationRepresentation`, `otherLanguageAndSemanticObligationsRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-own-source-translation-javascript-behavior.json` (not current evidence).
- `i195-grammar-priority-own-source-translation-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `ownProjectLanguagesReceiveInitialGrammarPriority`, `ownCodeUsesCommonEditableTranslationRepresentation`, `otherLanguageAndSemanticObligationsRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-own-source-translation-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-PRIORITY-RUST-COMPLETE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Prioritize a complete Rust grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete Rust grammar as an initial delivery priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **complete Rust grammar as an initial delivery priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-priority-rust-complete-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `rustGrammarPrioritized`, `rustCompleteConcreteSyntaxValidated`, `rustOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-rust-complete-javascript-behavior.json` (not current evidence).
- `i195-grammar-priority-rust-complete-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `rustGrammarPrioritized`, `rustCompleteConcreteSyntaxValidated`, `rustOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-rust-complete-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-PRIORITY-TYPESCRIPT-COMPLETE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [GRAMMAR — Full CST grammars and source unions](issue-227-remaining-work-plan-2026-10-31.md#grammar) · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest.

**Exact required behavior:** Prioritize a complete TypeScript grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete TypeScript grammar as an initial delivery priority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt)
**Vision clause:** [docs/vision.md#language-catalog-and-default-parsing](../../docs/vision.md#language-catalog-and-default-parsing).
**Registered inventories:** [parity/sources/own-language-complete-grammar-priority-2026-10-16.txt](../../parity/sources/own-language-complete-grammar-priority-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [GRAMMAR's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#grammar) for this construct: **complete TypeScript grammar as an initial delivery priority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-priority-typescript-complete-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `typescriptGrammarPrioritized`, `typescriptCompleteConcreteSyntaxValidated`, `typescriptOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-typescript-complete-javascript-behavior.json` (not current evidence).
- `i195-grammar-priority-typescript-complete-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:ownLanguageGrammarPriorityRequest`.
  - Observable assertions to implement/control: `typescriptGrammarPrioritized`, `typescriptCompleteConcreteSyntaxValidated`, `typescriptOwnSourceFilesExercised`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-priority-typescript-complete-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## EMBED — Recursive any-grammar parsing

5 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#embed). Candidate locations: [js/src/dynamic-language-cli.js](../../js/src/dynamic-language-cli.js), [js/src/native-grammar-parser.js](../../js/src/native-grammar-parser.js), [js/src/source-project-archive.js](../../js/src/source-project-archive.js), [rust/src/native_grammar_parser.rs](../../rust/src/native_grammar_parser.rs), [docs/vision.md](../../docs/vision.md).

### I195-CST-ANY-GRAMMAR-RECURSIVE-RULE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EMBED — Recursive any-grammar parsing](issue-227-remaining-work-plan-2026-10-31.md#embed) · P1 substrate, P2 full catalog.

**Exact required behavior:** Provide an inspectable linked any-grammar or all-grammar rule that can select and invoke supported grammar definitions recursively through the common algorithms. Selection, embedded boundaries, ambiguities and failures remain explicit; adding a supported grammar does not require special cases for every possible host and child language pair.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = generic any grammar or all grammar selection rule · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/fixtures/grammar-selection-json-string.lino](../../parity/fixtures/grammar-selection-json-string.lino), [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EMBED's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#embed) for this construct: **generic any grammar or all grammar selection rule**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-cst-any-grammar-recursive-rule-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `genericGrammarSelectionRepresentedAsLinks`, `supportedGrammarsRecursivelyInvoked`, `selectionAmbiguityAndFailuresExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-any-grammar-recursive-rule-javascript-behavior.json` (not current evidence).
- `i195-cst-any-grammar-recursive-rule-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `genericGrammarSelectionRepresentedAsLinks`, `supportedGrammarsRecursivelyInvoked`, `selectionAmbiguityAndFailuresExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-any-grammar-recursive-rule-rust-behavior.json` (not current evidence).
- `i195-cst-any-grammar-recursive-rule-javascript-json-string-profile` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-selection-javascript`.
  - Observable assertions to implement/control: `jsonStringAnyAllCandidateWitnessesPreserved`, `jsonStringUnsupportedAndNonScalarProfilesRefused`, `mappedProfileMemoKeysKeepExactGrammarNames`, `jsonStringLexicalAndMalformedInputsRefused`, `invalidUtf8RootCannotClaimUtf8SourcePlane`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-any-grammar-recursive-rule-javascript-json-string-profile.json` (not current evidence).
- `i195-cst-any-grammar-recursive-rule-rust-json-string-profile` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-selection-rust`.
  - Observable assertions to implement/control: `jsonStringAnyAllCandidateWitnessesPreserved`, `jsonStringUnsupportedAndNonScalarProfilesRefused`, `mappedProfileMemoKeysKeepExactGrammarNames`, `jsonStringLexicalAndMalformedInputsRefused`, `invalidUtf8RootCannotClaimUtf8SourcePlane`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-any-grammar-recursive-rule-rust-json-string-profile.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EMBED — Recursive any-grammar parsing](issue-227-remaining-work-plan-2026-10-31.md#embed) · P1 substrate, P2 full catalog.

**Exact required behavior:** Both packages support deeply recursive parsing, including Markdown within Markdown and Markdown containing JavaScript containing JSON whose content contains Markdown and Python, as well as mixed logs and documents. Recursion follows linked embedding definitions and can return to an earlier language; it is not restricted to one nesting level or to Markdown and HTML hosts. Non-consuming cycles and resource exhaustion report diagnostics rather than silently omitting nested trees.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = deep same-language and mixed-language recursive concrete parsing · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/fixtures/grammar-selection-json-string.lino](../../parity/fixtures/grammar-selection-json-string.lino), [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EMBED's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#embed) for this construct: **deep same-language and mixed-language recursive concrete parsing**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-cst-deeply-recursive-mixed-languages-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `sameLanguageRecursionSupported`, `markdownJavaScriptJsonMarkdownPythonNestingSupported`, `mixedLogsAndDocumentsSupported`, `cyclesAndResourceLimitsReportedWithoutSilentTruncation`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-deeply-recursive-mixed-languages-javascript-behavior.json` (not current evidence).
- `i195-cst-deeply-recursive-mixed-languages-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `sameLanguageRecursionSupported`, `markdownJavaScriptJsonMarkdownPythonNestingSupported`, `mixedLogsAndDocumentsSupported`, `cyclesAndResourceLimitsReportedWithoutSilentTruncation`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-deeply-recursive-mixed-languages-rust-behavior.json` (not current evidence).
- `i195-cst-deeply-recursive-mixed-languages-javascript-json-string-profile` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-recursion-javascript`.
  - Observable assertions to implement/control: `nestedMappedSourcesRetainActualDerivationOrder`, `mappedParsersRetainExistingLimitRefusals`, `mappedCycleAndSyntaxFailuresRetainSourceRoutes`, `failedMappedParseCannotAuthenticateMissingRoot`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-deeply-recursive-mixed-languages-javascript-json-string-profile.json` (not current evidence).
- `i195-cst-deeply-recursive-mixed-languages-rust-json-string-profile` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-recursion-rust`.
  - Observable assertions to implement/control: `nestedMappedSourcesRetainActualDerivationOrder`, `mappedParsersRetainExistingLimitRefusals`, `mappedCycleAndSyntaxFailuresRetainSourceRoutes`, `failedMappedParseCannotAuthenticateMissingRoot`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-deeply-recursive-mixed-languages-rust-json-string-profile.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-CST-DEFAULT-EVERY-LANGUAGE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EMBED — Recursive any-grammar parsing](issue-227-remaining-work-plan-2026-10-31.md#embed) · P1 substrate, P2 full catalog.

**Exact required behavior:** Every supported language grammar generates the complete lossless concrete syntax tree by default, including tokens, trivia, fields, hierarchy, order, spans, recovery information and embedded boundaries. A token list, retained source envelope or abstract projection cannot substitute for that default.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = full concrete syntax as the default grammar output for every supported language · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EMBED's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#embed) for this construct: **full concrete syntax as the default grammar output for every supported language**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-cst-default-every-language-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `everySupportedLanguageDefaultsToCompleteConcreteSyntax`, `allConcreteStructureAndSourceBytesPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-default-every-language-javascript-behavior.json` (not current evidence).
- `i195-cst-default-every-language-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `everySupportedLanguageDefaultsToCompleteConcreteSyntax`, `allConcreteStructureAndSourceBytesPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-default-every-language-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-CST-EMBEDDED-GLOBAL-CONCEPT-SPACE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EMBED — Recursive any-grammar parsing](issue-227-remaining-work-plan-2026-10-31.md#embed) · P1 substrate, P2 full catalog.

**Exact required behavior:** All host and embedded grammars use the same global concept space at every nesting level. Repeated or same-language embeddings preserve stable concept identities, source-qualified grammar references and the existing requirement that deduplication requires a proved one-to-one correspondence.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = one global concept space across all recursively embedded grammars · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EMBED's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#embed) for this construct: **one global concept space across all recursively embedded grammars**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-cst-embedded-global-concept-space-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `hostAndEmbeddedGrammarsShareGlobalConceptSpace`, `recursiveConceptIdentityAndProvenancePreserved`, `unprovedMeaningsRemainDistinct`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-embedded-global-concept-space-javascript-behavior.json` (not current evidence).
- `i195-cst-embedded-global-concept-space-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `hostAndEmbeddedGrammarsShareGlobalConceptSpace`, `recursiveConceptIdentityAndProvenancePreserved`, `unprovedMeaningsRemainDistinct`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-embedded-global-concept-space-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EMBED — Recursive any-grammar parsing](issue-227-remaining-work-plan-2026-10-31.md#embed) · P1 substrate, P2 full catalog.

**Exact required behavior:** Deeply embedded concrete trees preserve their full enclosing hierarchy, exact original source bytes, delimiters, escapes, trivia and mappings to host spans. Serialization, reload and translation retain those boundaries and provenance so nested edits and translations do not discard or reinterpret surrounding content.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = lossless recursive boundaries and nested translation provenance · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt)
**Vision clause:** [docs/vision.md#concrete-and-abstract-syntax-trees](../../docs/vision.md#concrete-and-abstract-syntax-trees).
**Registered inventories:** [parity/fixtures/grammar-selection-json-string.lino](../../parity/fixtures/grammar-selection-json-string.lino), [parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt](../../parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EMBED's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#embed) for this construct: **lossless recursive boundaries and nested translation provenance**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-cst-recursive-boundaries-lossless-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `completeEmbeddedHierarchyAndBoundariesPreserved`, `nestedEscapesAndHostSpansRemainExact`, `nestedSerializationReloadAndTranslationPreserveSource`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-recursive-boundaries-lossless-javascript-behavior.json` (not current evidence).
- `i195-cst-recursive-boundaries-lossless-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveConcreteSyntaxRequest`.
  - Observable assertions to implement/control: `completeEmbeddedHierarchyAndBoundariesPreserved`, `nestedEscapesAndHostSpansRemainExact`, `nestedSerializationReloadAndTranslationPreserveSource`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-recursive-boundaries-lossless-rust-behavior.json` (not current evidence).
- `i195-cst-recursive-boundaries-lossless-javascript-json-string-profile` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-boundaries-javascript`.
  - Observable assertions to implement/control: `shippedJsonPythonConcreteTreesRemainExact`, `jsonStringUtf8IntervalsAndQuoteOwnershipExact`, `mappedSourceRegistryAndGrammarReplayTextAndBinary`, `serializableMappingForgeriesRefusedAfterReload`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-recursive-boundaries-lossless-javascript-json-string-profile.json` (not current evidence).
- `i195-cst-recursive-boundaries-lossless-rust-json-string-profile` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `grammar-selection-json-string-boundaries-rust`.
  - Observable assertions to implement/control: `shippedJsonPythonConcreteTreesRemainExact`, `jsonStringUtf8IntervalsAndQuoteOwnershipExact`, `mappedSourceRegistryAndGrammarReplayTextAndBinary`, `serializableMappingForgeriesRefusedAfterReload`.
  - Future commit-bound artifact: `issue-195-results/i195-cst-recursive-boundaries-lossless-rust-json-string-profile.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## MEANING — Global concepts, recursive definitions and link foundations

8 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#meaning). Candidate locations: [js/src/global-concept-space.js](../../js/src/global-concept-space.js), [js/src/formal-concept-definition.js](../../js/src/formal-concept-definition.js), [js/src/concept-ontology.js](../../js/src/concept-ontology.js), [js/src/concept-views.js](../../js/src/concept-views.js), [rust/src/global_concept_space.rs](../../rust/src/global_concept_space.rs), [rust/src/formal_concept_definition.rs](../../rust/src/formal_concept_definition.rs), [parity/fixtures/formal-concept-definitions.lino](../../parity/fixtures/formal-concept-definitions.lino).

### I195-ASSOCIATIVE-STACK-PROJECT-WIDE-USE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** An executable project-wide audit covers both packages, code, grammars, translation, tools, documentation and dependencies against the relevant associative stack obligations. Direct runtime providers, compatibility or theoretical alignment and development tools are distinguished; unrelated formal-ai dependencies are not blindly introduced.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = associative stack adoption across the entire project · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt)
**Vision clause:** [docs/vision.md#associative-stack-and-theoretical-foundation](../../docs/vision.md#associative-stack-and-theoretical-foundation).
**Registered inventories:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **associative stack adoption across the entire project**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-associative-stack-project-wide-use-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:associativeFoundationRequest`.
  - Observable assertions to implement/control: `entireProjectStackUsageAudited`, `actualCodeAndDependenciesTraced`, `runtimeCompatibilityAndDevelopmentRolesDistinct`.
  - Future commit-bound artifact: `issue-195-results/i195-associative-stack-project-wide-use-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-FORMAL-CONCEPT-DEFINITIONS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** Every concept has a recursively inspectable formal definition represented as links, with explicit axioms, operations, bindings and declared foundational primitives. An English description or opaque semantic signature is insufficient as its formal definition.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = deep formal definition of every concept · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt)
**Vision clause:** [docs/vision.md#recursive-formal-concept-definitions](../../docs/vision.md#recursive-formal-concept-definitions).
**Registered inventories:** [parity/fixtures/formal-concept-definitions.json](../../parity/fixtures/formal-concept-definitions.lino), [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **deep formal definition of every concept**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-formal-concept-definitions-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `everyConceptFormallyDefined`, `definitionsAreInspectableLinks`, `foundationalPrimitivesExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-definitions-javascript-behavior.json` (not current evidence).
- `i195-formal-concept-definitions-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `everyConceptFormallyDefined`, `definitionsAreInspectableLinks`, `foundationalPrimitivesExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-definitions-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-FORMAL-CONCEPT-MEANING-COMPARISON

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** Meaning comparison uses the complete recursive formal definitions and checked obligations under their explicit foundations. Equal names, source text, labels or declared meaning signatures alone cannot certify semantic equivalence; unresolved obligations preserve separate concepts.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = formal definitions as evidence of equal concept meaning · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt)
**Vision clause:** [docs/vision.md#recursive-formal-concept-definitions](../../docs/vision.md#recursive-formal-concept-definitions).
**Registered inventories:** [parity/fixtures/formal-concept-definitions.json](../../parity/fixtures/formal-concept-definitions.lino), [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **formal definitions as evidence of equal concept meaning**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-formal-concept-meaning-comparison-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `formalDefinitionsCompared`, `semanticObligationsChecked`, `unprovedMeaningEquivalenceRejected`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-meaning-comparison-javascript-behavior.json` (not current evidence).
- `i195-formal-concept-meaning-comparison-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `formalDefinitionsCompared`, `semanticObligationsChecked`, `unprovedMeaningEquivalenceRejected`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-meaning-comparison-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-FORMAL-CONCEPT-REFERENCES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** Concept definitions can use other concepts recursively while retaining stable identities, qualified references and the selected foundation. Shared and cyclic references serialize and reload without flattening definitions or discarding their assumptions.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = mutual references between formal concept definitions · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt)
**Vision clause:** [docs/vision.md#recursive-formal-concept-definitions](../../docs/vision.md#recursive-formal-concept-definitions).
**Registered inventories:** [parity/fixtures/formal-concept-definitions.json](../../parity/fixtures/formal-concept-definitions.lino), [parity/sources/recursive-formal-definitions-2026-10-12.txt](../../parity/sources/recursive-formal-definitions-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **mutual references between formal concept definitions**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-formal-concept-references-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `recursiveConceptReferencesPreserved`, `sharedAndCyclicDefinitionsReloaded`, `definitionAssumptionsPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-references-javascript-behavior.json` (not current evidence).
- `i195-formal-concept-references-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:recursiveDefinitionsRequest`, `formal-concept-definitions`.
  - Observable assertions to implement/control: `recursiveConceptReferencesPreserved`, `sharedAndCyclicDefinitionsReloaded`, `definitionAssumptionsPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-formal-concept-references-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-LINK-FOUNDATION-PHYSICAL-ANALOGY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** The user wave, particle and fractal analogy is preserved verbatim as motivation and a hypothesis. Documentation and formal models do not report those physical propositions, universal self-similarity or freedom from contradictions as established facts or consistency proofs.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = recorded wave and fractal motivation with an explicit epistemic boundary · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt](../../parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt)
**Vision clause:** [docs/vision.md#links-as-fundamental-entities-and-relationships](../../docs/vision.md#links-as-fundamental-entities-and-relationships).
**Registered inventories:** [parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt](../../parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **recorded wave and fractal motivation with an explicit epistemic boundary**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-link-foundation-physical-analogy-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:linksFoundationRequest`.
  - Observable assertions to implement/control: `physicalAnalogyCapturedVerbatim`, `physicalPropositionsMarkedAsHypotheses`, `noAnalogyPresentedAsConsistencyProof`.
  - Future commit-bound artifact: `issue-195-results/i195-link-foundation-physical-analogy-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-LINK-FOUNDATION-UNIVERSAL-REPRESENTATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** Every represented concept, entity, relationship, root and formal definition is fundamentally a link or composition of links in the same network. Independent host objects or hidden side tables cannot substitute for the common representation.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = links as the fundamental representation of concepts, entities and relationships · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt](../../parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt)
**Vision clause:** [docs/vision.md#links-as-fundamental-entities-and-relationships](../../docs/vision.md#links-as-fundamental-entities-and-relationships).
**Registered inventories:** [parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt](../../parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **links as the fundamental representation of concepts, entities and relationships**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-link-foundation-universal-representation-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:linksFoundationRequest`.
  - Observable assertions to implement/control: `conceptsEntitiesRelationshipsAreLinks`, `rootsAndDefinitionsAreLinks`, `noHiddenEntityRepresentation`.
  - Future commit-bound artifact: `issue-195-results/i195-link-foundation-universal-representation-javascript-behavior.json` (not current evidence).
- `i195-link-foundation-universal-representation-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:linksFoundationRequest`.
  - Observable assertions to implement/control: `conceptsEntitiesRelationshipsAreLinks`, `rootsAndDefinitionsAreLinks`, `noHiddenEntityRepresentation`.
  - Future commit-bound artifact: `issue-195-results/i195-link-foundation-universal-representation-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-THEORY-FOUNDATION-CONSUMER-BOUNDARIES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** The shared philosophical and theoretical links foundation does not silently choose a universal proof logic, Type:Type axiom or consumer execution policy. Consumer-owned logical semantics and proof authority remain separate explicit model choices with checked correspondences.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = theoretical foundation separated from consumer proof authority · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt)
**Vision clause:** [docs/vision.md#associative-stack-and-theoretical-foundation](../../docs/vision.md#associative-stack-and-theoretical-foundation).
**Registered inventories:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **theoretical foundation separated from consumer proof authority**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-theory-foundation-consumer-boundaries-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:associativeFoundationRequest`.
  - Observable assertions to implement/control: `theoreticalAndConsumerFoundationsDistinct`, `noSilentUniversalLogicOrSelfTyping`, `consumerProofAuthorityPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-theory-foundation-consumer-boundaries-javascript-behavior.json` (not current evidence).
- `i195-theory-foundation-consumer-boundaries-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:associativeFoundationRequest`.
  - Observable assertions to implement/control: `theoreticalAndConsumerFoundationsDistinct`, `noSilentUniversalLogicOrSelfTyping`, `consumerProofAuthorityPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-theory-foundation-consumer-boundaries-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-THEORY-FOUNDATION-PROJECT-WIDE-APPLICATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MEANING — Global concepts, recursive definitions and link foundations](issue-227-remaining-work-plan-2026-10-31.md#meaning) · P1 architecture.

**Exact required behavior:** The common concept, grammar, ontology, taxonomy, entity and relationship representation applies the adopted links meta-theory definitions throughout both packages. Model choices, theoretical premises and operational encodings remain explicit inspectable links with source provenance.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = meta-theory foundation applied to the common links model · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt)
**Vision clause:** [docs/vision.md#associative-stack-and-theoretical-foundation](../../docs/vision.md#associative-stack-and-theoretical-foundation).
**Registered inventories:** [parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt](../../parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MEANING's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#meaning) for this construct: **meta-theory foundation applied to the common links model**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-theory-foundation-project-wide-application-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:associativeFoundationRequest`.
  - Observable assertions to implement/control: `metaTheoryDefinitionsAppliedProjectWide`, `modelPremisesAreInspectableLinks`, `operationalEncodingsRetainProvenance`.
  - Future commit-bound artifact: `issue-195-results/i195-theory-foundation-project-wide-application-javascript-behavior.json` (not current evidence).
- `i195-theory-foundation-project-wide-application-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:associativeFoundationRequest`.
  - Observable assertions to implement/control: `metaTheoryDefinitionsAppliedProjectWide`, `modelPremisesAreInspectableLinks`, `operationalEncodingsRetainProvenance`.
  - Future commit-bound artifact: `issue-195-results/i195-theory-foundation-project-wide-application-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## MERGE — Grammar import, conversion and automatic reconciliation

4 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#merge). Candidate locations: [js/src/grammar-interchange.js](../../js/src/grammar-interchange.js), [js/src/grammar-merge.js](../../js/src/grammar-merge.js), [js/src/grammar-merge-decisions.js](../../js/src/grammar-merge-decisions.js), [js/src/grammar-reverse.js](../../js/src/grammar-reverse.js), [parity/fixtures/merge-quality-evidence.lino](../../parity/fixtures/merge-quality-evidence.lino), [parity/fixtures/grammar-merge.lino](../../parity/fixtures/grammar-merge.lino).

### I195-INTERCHANGE-CROSS-FORMAT-TOOLS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MERGE — Grammar import, conversion and automatic reconciliation](issue-227-remaining-work-plan-2026-10-31.md#merge) · P2; source-union support P1.

**Exact required behavior:** Cross-format conversions and native -> external -> native cycles run under the actual independent target tools. The tools compare accepted and rejected programs, parse structure, lexical decisions, precedence, actions and diagnostics with the source grammar.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = cross-format conversion checked by the real target tools · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#grammar-import-conversion-and-reverse-conversion](../../docs/vision.md#grammar-import-conversion-and-reverse-conversion).
**Registered inventories:** [parity/fixtures/grammar-importers.json](../../parity/fixtures/grammar-importers.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MERGE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#merge) for this construct: **cross-format conversion checked by the real target tools**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-interchange-cross-format-tools-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-interchange-cross-format-tools`.
  - Observable assertions to implement/control: `crossFormatCyclesTested`, `targetToolsExecuted`, `acceptanceAndStructureCompared`, `diagnosticsCompared`.
  - Future commit-bound artifact: `issue-195-results/i195-interchange-cross-format-tools-javascript-behavior.json` (not current evidence).
- `i195-interchange-cross-format-tools-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-interchange-cross-format-tools`.
  - Observable assertions to implement/control: `crossFormatCyclesTested`, `targetToolsExecuted`, `acceptanceAndStructureCompared`, `diagnosticsCompared`.
  - Future commit-bound artifact: `issue-195-results/i195-interchange-cross-format-tools-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-INTERCHANGE-FORMAT-FEATURES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MERGE — Grammar import, conversion and automatic reconciliation](issue-227-remaining-work-plan-2026-10-31.md#merge) · P2; source-union support P1.

**Exact required behavior:** Every feature of BNF, EBNF, ABNF, PEG, Peggy, Pest, ANTLR, tree-sitter source and JSON, Lark, GBNF, Yacc/Bison, lexer specifications and the other reconciled formats has an explicit native representation and operational meaning. Imports follow included grammars, lexer and scanner definitions and action code, and keep comments, locations, original names and formatting provenance.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = every feature of every grammar format imported with its supporting definitions · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#grammar-import-conversion-and-reverse-conversion](../../docs/vision.md#grammar-import-conversion-and-reverse-conversion).
**Registered inventories:** [parity/fixtures/grammar-importers.json](../../parity/fixtures/grammar-importers.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MERGE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#merge) for this construct: **every feature of every grammar format imported with its supporting definitions**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-interchange-format-features-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-interchange-format-features`.
  - Observable assertions to implement/control: `everyFormatFeatureRepresented`, `includedGrammarsFollowed`, `lexerScannerAndActionsImported`, `provenancePreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-interchange-format-features-javascript-behavior.json` (not current evidence).
- `i195-interchange-format-features-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-interchange-format-features`.
  - Observable assertions to implement/control: `everyFormatFeatureRepresented`, `includedGrammarsFollowed`, `lexerScannerAndActionsImported`, `provenancePreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-interchange-format-features-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-MERGE-AUTOMATIC-PIPELINE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MERGE — Grammar import, conversion and automatic reconciliation](issue-227-remaining-work-plan-2026-10-31.md#merge) · P2; source-union support P1.

**Exact required behavior:** The default pipeline discovers and imports the inventoried sources, recognizes corresponding concepts under different names, reconciles and normalizes them, deduplicates equivalent concepts, preserves unique ones, and emits the canonical native grammar with a reproducible provenance and decision report, without user interaction.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = non-interactive multi-source grammar merge pipeline · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#automatic-merging-concept-recognition-deduplication-and-renaming](../../docs/vision.md#automatic-merging-concept-recognition-deduplication-and-renaming).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MERGE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#merge) for this construct: **non-interactive multi-source grammar merge pipeline**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-merge-automatic-pipeline-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-merge-automatic-pipeline`.
  - Observable assertions to implement/control: `sourcesDiscoveredAndImported`, `correspondingConceptsRecognized`, `uniqueConceptsPreserved`, `decisionReportReproducible`, `noInteractionRequired`.
  - Future commit-bound artifact: `issue-195-results/i195-merge-automatic-pipeline-javascript-behavior.json` (not current evidence).
- `i195-merge-automatic-pipeline-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-merge-automatic-pipeline`.
  - Observable assertions to implement/control: `sourcesDiscoveredAndImported`, `correspondingConceptsRecognized`, `uniqueConceptsPreserved`, `decisionReportReproducible`, `noInteractionRequired`.
  - Future commit-bound artifact: `issue-195-results/i195-merge-automatic-pipeline-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-MERGE-SHIPPED-GRAMMARS-ARE-MERGED

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [MERGE — Grammar import, conversion and automatic reconciliation](issue-227-remaining-work-plan-2026-10-31.md#merge) · P2; source-union support P1.

**Exact required behavior:** The production grammars and the native grammars used in tests are outputs of the multi-source merge. Each compatible source keeps its unique valid coverage, dialects are not united indiscriminately, and upstream oracles stay independent of the merged output.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = shipped and test grammars produced by the multi-source merge · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#automatic-merging-concept-recognition-deduplication-and-renaming](../../docs/vision.md#automatic-merging-concept-recognition-deduplication-and-renaming).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [MERGE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#merge) for this construct: **shipped and test grammars produced by the multi-source merge**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-merge-shipped-grammars-are-merged-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-merge-shipped-grammars-are-merged`.
  - Observable assertions to implement/control: `shippedGrammarsAreMergeOutputs`, `testGrammarsAreMergeOutputs`, `uniqueSourceCoverageKept`, `oraclesIndependent`.
  - Future commit-bound artifact: `issue-195-results/i195-merge-shipped-grammars-are-merged-javascript-behavior.json` (not current evidence).
- `i195-merge-shipped-grammars-are-merged-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-merge-shipped-grammars-are-merged`.
  - Observable assertions to implement/control: `shippedGrammarsAreMergeOutputs`, `testGrammarsAreMergeOutputs`, `uniqueSourceCoverageKept`, `oraclesIndependent`.
  - Future commit-bound artifact: `issue-195-results/i195-merge-shipped-grammars-are-merged-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability

11 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#dynamic). Candidate locations: [js/src/source-project-archive.js](../../js/src/source-project-archive.js), [js/src/dynamic-language-cli.js](../../js/src/dynamic-language-cli.js), [js/src/unsigned-link-network.js](../../js/src/unsigned-link-network.js), [js/src/global-concept-space.js](../../js/src/global-concept-space.js), [rust/src/global_concept_space.rs](../../rust/src/global_concept_space.rs), [docs/vision.md](../../docs/vision.md).

### I195-DYNAMIC-BINARY-LINKS-RECONSTRUCTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Binary Links Notation can serialize, reload and extend the same complete dynamic representation with parity to its text form and without hidden authoritative side tables.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = dynamic reconstruction from binary Links Notation · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **dynamic reconstruction from binary Links Notation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-binary-links-reconstruction-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `binaryLinksDynamicRoundTripExecuted`, `textAndBinaryDynamicRepresentationsAgree`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-binary-links-reconstruction-javascript-behavior.json` (not current evidence).
- `i195-dynamic-binary-links-reconstruction-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `binaryLinksDynamicRoundTripExecuted`, `textAndBinaryDynamicRepresentationsAgree`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-binary-links-reconstruction-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DYNAMIC-LANGUAGE-EXTENSION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Users can apply the shared algorithms to extend the dynamic meta-language with new languages, grammars and formal concepts. Extensions retain the global concept space, validation, source provenance and default behavior of existing catalog languages.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = algorithmic runtime extension with other languages · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **algorithmic runtime extension with other languages**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-language-extension-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `dynamicLanguageExtensionExecuted`, `extensionUsesSharedAlgorithms`, `existingLanguageBehaviorPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-language-extension-javascript-behavior.json` (not current evidence).
- `i195-dynamic-language-extension-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `dynamicLanguageExtensionExecuted`, `extensionUsesSharedAlgorithms`, `existingLanguageBehaviorPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-language-extension-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DYNAMIC-LINK-CLI-RECONSTRUCTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** The dynamic representation can be recreated and processed through link-cli with a reproducible public path, preserving the complete trees, concept definitions and selected model.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = dynamic reconstruction through link-cli · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **dynamic reconstruction through link-cli**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-link-cli-reconstruction-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `linkCliReconstructionExecuted`, `linkCliPreservesFullTreesAndConcepts`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-link-cli-reconstruction-javascript-behavior.json` (not current evidence).
- `i195-dynamic-link-cli-reconstruction-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `linkCliReconstructionExecuted`, `linkCliPreservesFullTreesAndConcepts`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-link-cli-reconstruction-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DYNAMIC-RECONSTRUCTION-RESOURCE-MEASUREMENT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Both packages measure actual complete CST and concept-space reconstruction through the public link-cli, text Links Notation and binary Links Notation paths. Measurements record reconstruction latency and peak resources together with pinned input identity, representation, size, algorithm and execution environment provenance. The reconstructed graphs preserve the complete trees, rules, recursive formal definitions, identities, references, selected models, source bytes and provenance. Lazy loading, token-only reconstruction and descriptor creation cannot count as a measured full rebuild. These measurements retain the source qualitative speed requirement without inventing a numeric deadline.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = measured latency and resources for complete dynamic reconstruction · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **measured latency and resources for complete dynamic reconstruction**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-reconstruction-resource-measurement-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `completeDynamicReconstructionMeasured`, `reconstructionLatencyAndResourcesRecorded`, `measurementInputAndEnvironmentProvenanceRetained`, `fullRepresentationPreservationRetained`, `lazyOrTokenOnlyReconstructionNotCounted`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-reconstruction-resource-measurement-javascript-behavior.json` (not current evidence).
- `i195-dynamic-reconstruction-resource-measurement-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `completeDynamicReconstructionMeasured`, `reconstructionLatencyAndResourcesRecorded`, `measurementInputAndEnvironmentProvenanceRetained`, `fullRepresentationPreservationRetained`, `lazyOrTokenOnlyReconstructionNotCounted`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-reconstruction-resource-measurement-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DYNAMIC-SNAPSHOT-RECONSTRUCTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** A prepared static snapshot can be reconstructed on demand into the complete CST and concept space, retaining rules, formal definitions, identities, recursive links, source bytes and provenance instead of reconstructing a token list or descriptor.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = static snapshot becomes a complete dynamic representation on demand · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **static snapshot becomes a complete dynamic representation on demand**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-snapshot-reconstruction-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `snapshotReconstructsFullCst`, `snapshotReconstructsFullConceptSpace`, `dynamicReconstructionRetainsProvenance`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-snapshot-reconstruction-javascript-behavior.json` (not current evidence).
- `i195-dynamic-snapshot-reconstruction-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `snapshotReconstructsFullCst`, `snapshotReconstructsFullConceptSpace`, `dynamicReconstructionRetainsProvenance`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-snapshot-reconstruction-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DYNAMIC-TEXT-LINKS-RECONSTRUCTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Text Links Notation can serialize, reload and extend the complete dynamic CST and concept space without the original grammar or source buffers.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = dynamic reconstruction from text Links Notation · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **dynamic reconstruction from text Links Notation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dynamic-text-links-reconstruction-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `textLinksDynamicRoundTripExecuted`, `textLinksPreservesCompleteRepresentation`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-text-links-reconstruction-javascript-behavior.json` (not current evidence).
- `i195-dynamic-text-links-reconstruction-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `textLinksDynamicRoundTripExecuted`, `textLinksPreservesCompleteRepresentation`.
  - Future commit-bound artifact: `issue-195-results/i195-dynamic-text-links-reconstruction-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-KNOWLEDGE-OWL-SUPPORT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Both packages represent the required OWL constructs, formal axioms and chosen semantic profiles as links, preserving distinctions and obligations without silently replacing the consumer foundation.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = full Web Ontology Language representation and operations · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **full Web Ontology Language representation and operations**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-knowledge-owl-support-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `owlConstructsAndAxiomsPreserved`, `owlSemanticProfilesExplicit`, `consumerFoundationRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-owl-support-javascript-behavior.json` (not current evidence).
- `i195-knowledge-owl-support-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `owlConstructsAndAxiomsPreserved`, `owlSemanticProfilesExplicit`, `consumerFoundationRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-owl-support-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-KNOWLEDGE-RDF-OWL-CONVERSION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Public converters and exporters in both packages execute faithful knowledge-format conversion cycles from the shared representation without returning a saved input or silently weakening axioms and identities.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = faithful RDF and OWL conversion cycles · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **faithful RDF and OWL conversion cycles**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-knowledge-rdf-owl-conversion-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `knowledgeConversionCyclesExecuted`, `knowledgeAxiomsAndIdentitiesPreserved`, `noSavedInputConversionShortcut`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-conversion-javascript-behavior.json` (not current evidence).
- `i195-knowledge-rdf-owl-conversion-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `knowledgeConversionCyclesExecuted`, `knowledgeAxiomsAndIdentitiesPreserved`, `noSavedInputConversionShortcut`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-conversion-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-KNOWLEDGE-RDF-OWL-IMPORT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Public importers in both packages import required RDF and OWL representations into the common concept and links space with their full structure, provenance and explicit diagnostics for unresolved obligations.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = complete RDF and OWL importers · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **complete RDF and OWL importers**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-knowledge-rdf-owl-import-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `rdfAndOwlPublicImportersExecuted`, `importedKnowledgeStructureAndProvenanceRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-import-javascript-behavior.json` (not current evidence).
- `i195-knowledge-rdf-owl-import-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `rdfAndOwlPublicImportersExecuted`, `importedKnowledgeStructureAndProvenanceRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-import-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-KNOWLEDGE-RDF-OWL-MERGE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Both packages merge imported RDF and OWL knowledge through the common links space, deduplicating only proved correspondences while retaining unique concepts, conflicting axioms, uncertainty and source-qualified provenance.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = meaning-preserving RDF and OWL merges · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **meaning-preserving RDF and OWL merges**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-knowledge-rdf-owl-merge-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `knowledgeMergesExecuted`, `knowledgeDeduplicationProved`, `conflictsAndUncertaintyPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-merge-javascript-behavior.json` (not current evidence).
- `i195-knowledge-rdf-owl-merge-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `knowledgeMergesExecuted`, `knowledgeDeduplicationProved`, `conflictsAndUncertaintyPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-owl-merge-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-KNOWLEDGE-RDF-SUPPORT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DYNAMIC — Dynamic Links reconstruction and RDF/OWL interoperability](issue-227-remaining-work-plan-2026-10-31.md#dynamic) · P2.

**Exact required behavior:** Both packages preserve the complete required RDF constructs, identities, relationships, literals, provenance and selected semantics as inspectable links. A finite triple example or opaque payload does not establish full RDF support.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = full Resource Description Framework representation and operations · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DYNAMIC's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#dynamic) for this construct: **full Resource Description Framework representation and operations**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-knowledge-rdf-support-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `rdfConstructsPreserved`, `rdfSemanticsAndProvenanceExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-support-javascript-behavior.json` (not current evidence).
- `i195-knowledge-rdf-support-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `rdfConstructsPreserved`, `rdfSemanticsAndProvenanceExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-knowledge-rdf-support-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## INFERENCE — Automatic grammar discovery and comparative research

12 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#inference). Candidate locations: [js/src/grammar-inference.js](../../js/src/grammar-inference.js), [js/src/grammar-inference-refinement.js](../../js/src/grammar-inference-refinement.js), [js/src/grammar-inference-rules.js](../../js/src/grammar-inference-rules.js), [rust/src/grammar/inference](../../rust/src/grammar/inference), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino).

### I195-GRAMMAR-INFERENCE-AUTOMATIC-DISCOVERY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Both packages discover or infer grammar definitions using recorded modern research and explicit assumptions, then validate and integrate the inferred definitions through the native pipeline and concept space with independent accepted and rejected observations.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = automatic grammar inference through research-backed algorithms · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt](../../parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **automatic grammar inference through research-backed algorithms**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-automatic-discovery-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `automaticGrammarDiscoveryExecuted`, `researchAndAssumptionsPinned`, `inferredGrammarsIndependentlyValidated`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-automatic-discovery-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-automatic-discovery-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:dynamicKnowledgeRequest`.
  - Observable assertions to implement/control: `automaticGrammarDiscoveryExecuted`, `researchAndAssumptionsPinned`, `inferredGrammarsIndependentlyValidated`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-automatic-discovery-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-CI-RESOURCE-POLICY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Use targeted JavaScript authoring locally; run Rust compilation, broad discovery experiments, whole corpora and downstream workloads in CI with bounded caches and per-run artifacts. Record critical scope, API parity and exact executed observations without completion claims for missing behavior.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = bounded JavaScript-first inference development and CI artifacts · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **bounded JavaScript-first inference development and CI artifacts**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-ci-resource-policy-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `javascriptFirstInferenceDevelopment`, `expensiveInferenceAndRustRunInCI`, `boundedCachesAndRunArtifacts`, `exactEvidenceAndApiParityRecorded`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-ci-resource-policy-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-COMPARATIVE-BENCHMARKS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Benchmark existing inference and selected modern methods with the same pinned workloads and budgets. Retain all failures and disagreement traces, select reproducible improvements without weakening acceptance, and keep statistical coverage distinct from semantic proof.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = reproducible inference comparison within common budgets · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **reproducible inference comparison within common budgets**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-comparative-benchmarks-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `commonBudgetsAndWorkloadsUsed`, `failuresAndDisagreementsRetained`, `reproducibleImprovementMeasured`, `statisticsNotPresentedAsProof`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-comparative-benchmarks-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-DETERMINISTIC-REFINEMENT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Seeded discovery is deterministic and supports counterexample-guided refinement, incremental updates and checkpoint resume without losing the examples, decisions, assumptions or rejected alternatives used to obtain the grammar.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = deterministic counterexample-guided incremental discovery · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **deterministic counterexample-guided incremental discovery**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-deterministic-refinement-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `seededDiscoveryDeterministic`, `counterexampleAndIncrementalRefinementExecuted`, `checkpointResumeRetainsEvidence`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-deterministic-refinement-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-deterministic-refinement-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `seededDiscoveryDeterministic`, `counterexampleAndIncrementalRefinementExecuted`, `checkpointResumeRetainsEvidence`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-deterministic-refinement-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-DYNAMIC-TRANSLATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** After installing a discovered language dynamically, ordinary parsing and Rust/JavaScript translation execute through the common algorithms. Changing a linked definition changes execution without changing the generic host runtime; stale certificates and incompatible definitions are rejected or revalidated explicitly.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = discovered languages execute and translate in the generic runtime · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **discovered languages execute and translate in the generic runtime**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-dynamic-translation-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoveredLanguageParsesAndTranslates`, `linkedMutationChangesGenericExecution`, `staleCertificatesRejectedOrRevalidated`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-dynamic-translation-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-dynamic-translation-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoveredLanguageParsesAndTranslates`, `linkedMutationChangesGenericExecution`, `staleCertificatesRejectedOrRevalidated`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-dynamic-translation-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-EXISTING-WORK-AUDIT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Audit and reuse the lexical, state-merging, context-free, structural-prior, active-learning and minimization work associated with issues93,114,117 through123 and the existing Rust inference modules. Closed issues do not prove complete capability or remove JavaScript parity obligations.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = existing inference foundations and JavaScript parity gaps · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **existing inference foundations and JavaScript parity gaps**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-existing-work-audit-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `existingInferenceModulesAudited`, `closedIssueEvidenceRechecked`, `javascriptParityGapsRecorded`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-existing-work-audit-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-GENERIC-UNION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Discover lexical classes, recursion and semantic relations generically, preserving every source production and provenance. Merge discovered definitions with the applicable official and mature grammars through the existing automatic native pipeline.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = generic structural and semantic discovery through the source-union pipeline · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **generic structural and semantic discovery through the source-union pipeline**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-generic-union-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `lexicalRecursiveAndSemanticDiscoveryExecuted`, `allSourceProductionsAndProvenanceRetained`, `discoveredAndOfficialGrammarsMerged`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-generic-union-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-generic-union-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `lexicalRecursiveAndSemanticDiscoveryExecuted`, `allSourceProductionsAndProvenanceRetained`, `discoveredAndOfficialGrammarsMerged`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-generic-union-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-INDEPENDENT-EVALUATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Separate discovery training from holdout evaluation and test adversarial rejections and real projects under several independent oracles. Record precision, recall, F1, CST fields, trivia, spans, attributed behavior, reconstruction, grammar size, determinism, query counts, elapsed time and peak memory.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = independent held-out grammar discovery evaluation · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **independent held-out grammar discovery evaluation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-independent-evaluation-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `trainingAndHoldoutSeparated`, `multipleIndependentOraclesAndProjectsUsed`, `allRequiredQualityAndResourceMetricsRecorded`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-independent-evaluation-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-NATIVE-OUTPUT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Discovery emits executable Links grammars with lossless CST structure, lexer and scanner state, precedence, ambiguity, recovery and attributed semantic actions. Foreign parser blobs, opaque sources and host callbacks cannot stand in for native operational definitions.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = fully native executable discovered grammars and attributed behavior · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **fully native executable discovered grammars and attributed behavior**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-native-output-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoveredGrammarNativeAndExecutable`, `discoveredCstAndScannerSemanticsPreserved`, `attributedActionsAndRecoveryExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-native-output-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-native-output-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoveredGrammarNativeAndExecutable`, `discoveredCstAndScannerSemanticsPreserved`, `attributedActionsAndRecoveryExplicit`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-native-output-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-PUBLIC-API-CLI

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** JavaScript and Rust library and CLI interfaces accept pinned positive and negative examples, independent membership or parser oracles, optional execution traces and existing grammars. Diagnostics identify unresolved context sensitivity, scanners and action semantics rather than silently approximating them.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = matching public grammar discovery library and CLI interfaces · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **matching public grammar discovery library and CLI interfaces**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-public-api-cli-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `matchingDiscoveryApiAndCliExecuted`, `examplesOraclesTracesAndSourcesAccepted`, `unsupportedSemanticsPreciselyDiagnosed`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-public-api-cli-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-public-api-cli-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `matchingDiscoveryApiAndCliExecuted`, `examplesOraclesTracesAndSourcesAccepted`, `unsupportedSemanticsPreciselyDiagnosed`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-public-api-cli-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-RESEARCH-SELECTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Compare the pinned XVada, Crucio, NatGI and attributed grammar-inference methods with recorded revisions, implementations, licenses, independent oracles, datasets and configurations. Paper-reported results remain hypotheses for local reproduction, and method selection retains its evidence rationale.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = pinned modern grammar discovery research and selection rationale · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **pinned modern grammar discovery research and selection rationale**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-research-selection-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `researchRevisionsAndLicensesPinned`, `fourResearchMethodsEvaluated`, `selectionRationaleAndReproductionRecorded`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-research-selection-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-GRAMMAR-INFERENCE-SOURCE-FREE-SNAPSHOTS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [INFERENCE — Automatic grammar discovery and comparative research](issue-227-remaining-work-plan-2026-10-31.md#inference) · P2 after baseline substrate.

**Exact required behavior:** Discovered snapshots restore and extend through link-cli, text Links Notation and a versioned binary links representation without their original source buffers, retaining identities, cycles, roots, full CSTs and proofs.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = source-free discovered grammar snapshots in text and versioned binary links · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/issues/224](https://github.com/link-foundation/meta-language/issues/224)
**Vision clause:** [docs/vision.md#dynamic-knowledge-and-language-extension](../../docs/vision.md#dynamic-knowledge-and-language-extension).
**Registered inventories:** [parity/sources/grammar-inference-issue-224.txt](../../parity/sources/grammar-inference-issue-224.txt).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [INFERENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#inference) for this construct: **source-free discovered grammar snapshots in text and versioned binary links**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-grammar-inference-source-free-snapshots-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoverySnapshotsReloadWithoutSource`, `linkCliTextAndVersionedBinaryAgree`, `snapshotIdentitiesRootsTreesAndProofsRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-source-free-snapshots-javascript-behavior.json` (not current evidence).
- `i195-grammar-inference-source-free-snapshots-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:grammarInferenceIssue`.
  - Observable assertions to implement/control: `discoverySnapshotsReloadWithoutSource`, `linkCliTextAndVersionedBinaryAgree`, `snapshotIdentitiesRootsTreesAndProofsRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-grammar-inference-source-free-snapshots-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## EVIDENCE — Independent acceptance, parity and failure-detecting gates

1 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#evidence). Candidate locations: [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [js/scripts/check-issue-195-acceptance.mjs](../../js/scripts/check-issue-195-acceptance.mjs), [js/scripts/run-issue-195-evidence.mjs](../../js/scripts/run-issue-195-evidence.mjs), [js/scripts/issue-195-acceptance-lib.mjs](../../js/scripts/issue-195-acceptance-lib.mjs), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino).

### I195-ACCEPTANCE-INDEPENDENT-ORACLES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [EVIDENCE — Independent acceptance, parity and failure-detecting gates](issue-227-remaining-work-plan-2026-10-31.md#evidence) · P1 control authorship; execution after full draft.

**Exact required behavior:** Every native merged grammar runs against pinned independent ANTLR, tree-sitter and official parser, compiler and prover corpora and real projects, with multiple oracles where available. Every construct and source discrepancy traces to a test and a justified resolution.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = native merged grammars checked against independent oracles and real projects · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#acceptance-and-evidence](../../docs/vision.md#acceptance-and-evidence).
**Registered inventories:** [parity/language-grammar-inventory.json](../../parity/language-grammar-inventory.lino).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [EVIDENCE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#evidence) for this construct: **native merged grammars checked against independent oracles and real projects**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-acceptance-independent-oracles-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-acceptance-independent-oracles`.
  - Observable assertions to implement/control: `independentCorporaExecuted`, `multipleOraclesCompared`, `discrepanciesTracedAndResolved`, `realProjectsParsed`.
  - Future commit-bound artifact: `issue-195-results/i195-acceptance-independent-oracles-javascript-behavior.json` (not current evidence).
- `i195-acceptance-independent-oracles-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-acceptance-independent-oracles`.
  - Observable assertions to implement/control: `independentCorporaExecuted`, `multipleOraclesCompared`, `discrepanciesTracedAndResolved`, `realProjectsParsed`.
  - Future commit-bound artifact: `issue-195-results/i195-acceptance-independent-oracles-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## DELIVERY — Dependencies, matching releases and downstream consumers

3 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#delivery). Candidate locations: [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [js/package.json](../../js/package.json), [rust/Cargo.toml](../../rust/Cargo.toml), [parity/language-features.lino](../../parity/language-features.lino), [docs/vision.md](../../docs/vision.md).

### I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DELIVERY — Dependencies, matching releases and downstream consumers](issue-227-remaining-work-plan-2026-10-31.md#delivery) · P3 execution after acceptance readiness.

**Exact required behavior:** Neither published package declares tree-sitter, web-tree-sitter, peggy or another external grammar engine as a production dependency or ships vendored grammar binaries. Independent oracles remain only as test and development dependencies.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = external parser engines removed from the production packages · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#dependencies](../../docs/vision.md#dependencies).
**Registered inventories:** [docs/vision.md](../../docs/vision.md).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DELIVERY's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#delivery) for this construct: **external parser engines removed from the production packages**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-dependency-production-parsers-removed-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-dependency-production-parsers-removed`.
  - Observable assertions to implement/control: `noExternalParserProductionDependency`, `noVendoredGrammarBinariesShipped`, `oraclesOnlyInTestScope`.
  - Future commit-bound artifact: `issue-195-results/i195-dependency-production-parsers-removed-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DOWNSTREAM-FORMAL-AI-PUBLISHED

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DELIVERY — Dependencies, matching releases and downstream consumers](issue-227-remaining-work-plan-2026-10-31.md#delivery) · P3 execution after acceptance readiness.

**Exact required behavior:** link-assistant/formal-ai installs the published npm and crates.io releases that carry this work and runs its workloads against them in a clean consumer.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = formal-ai consuming the published packages · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-5885245090](https://github.com/link-foundation/meta-language/pull/196#issuecomment-5885245090)
**Vision clause:** [docs/vision.md#packages-and-publication](../../docs/vision.md#packages-and-publication).
**Registered inventories:** [docs/vision.md](../../docs/vision.md).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DELIVERY's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#delivery) for this construct: **formal-ai consuming the published packages**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-downstream-formal-ai-published-javascript-published` — runtime `javascript`, kind `positive`, checkpoint `release-delivery`.
  - Original fixtures: `planned:repository-directive:i195-downstream-formal-ai-published`.
  - Observable assertions to implement/control: `publishedReleasesInstalled`, `workloadsRunOnPublishedReleases`.
  - Future commit-bound artifact: `issue-195-results/i195-downstream-formal-ai-published-javascript-published.json` (not current evidence).
- `i195-downstream-formal-ai-published-rust-published` — runtime `rust`, kind `positive`, checkpoint `release-delivery`.
  - Original fixtures: `planned:repository-directive:i195-downstream-formal-ai-published`.
  - Observable assertions to implement/control: `publishedReleasesInstalled`, `workloadsRunOnPublishedReleases`.
  - Future commit-bound artifact: `issue-195-results/i195-downstream-formal-ai-published-rust-published.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `delivery:formal-ai`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.
**Downstream obligations:** link-assistant/formal-ai.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [DELIVERY — Dependencies, matching releases and downstream consumers](issue-227-remaining-work-plan-2026-10-31.md#delivery) · P3 execution after acceptance readiness.

**Exact required behavior:** Both runtime packages translate the required Rust, JavaScript and TypeScript projection pairs through the common editable links representation, execute formal-ai projection rules, and preserve the required semantics on clean installed artifacts. A TypeScript concrete syntax tree alone does not count as translation.

**Full scope:** language = meta-language runtime packages · version = the npm package and the Rust crate of the same release · edition = every catalog language, grammar format and edition · construct = Rust, JavaScript and TypeScript semantic projection for formal-ai · aliases = (none) · extensions = (none)
**Required runtimes:** javascript, rust. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380](https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380)
**Vision clause:** [docs/vision.md#downstream-consumers](../../docs/vision.md#downstream-consumers).
**Registered inventories:** [docs/downstream-consumers.md](../../docs/downstream-consumers.md).

**Implementation references to inspect:**

- javascript: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.
- rust: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [DELIVERY's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#delivery) for this construct: **Rust, JavaScript and TypeScript semantic projection for formal-ai**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-downstream-typescript-translations-javascript-behavior` — runtime `javascript`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-downstream-typescript-translations`.
  - Observable assertions to implement/control: `allRequiredProjectionPairsTranslate`, `consumerRulesExecute`, `semanticOutcomesPreserved`, `cleanInstalledArtifactsUsed`.
  - Future commit-bound artifact: `issue-195-results/i195-downstream-typescript-translations-javascript-behavior.json` (not current evidence).
- `i195-downstream-typescript-translations-rust-behavior` — runtime `rust`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `planned:repository-directive:i195-downstream-typescript-translations`.
  - Observable assertions to implement/control: `allRequiredProjectionPairsTranslate`, `consumerRulesExecute`, `semanticOutcomesPreserved`, `cleanInstalledArtifactsUsed`.
  - Future commit-bound artifact: `issue-195-results/i195-downstream-typescript-translations-rust-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`, `suite:rust`.
**Package obligations:** `npm:meta-language`, `crates.io:meta-language`.
**Downstream obligations:** link-assistant/formal-ai.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## OPERATIONS — Resource safety, workflow, storage, naming and version policy

20 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#operations). Candidate locations: [AGENTS.md](../../AGENTS.md), [CLAUDE.md](../../CLAUDE.md), [.gitignore](../../.gitignore), [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [scripts/clean-caches.mjs](../../scripts/clean-caches.mjs), [scripts/with-cache-cleanup.mjs](../../scripts/with-cache-cleanup.mjs), [parity/naming/canonical-concepts.lino](../../parity/naming/canonical-concepts.lino).

### I195-LOCAL-RESOURCE-CAREFUL-BOUNDED-USE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Contributors check available system resources and keep local disk, memory and process use bounded while performing this task, preventing meta-language work from exhausting the host resources. Resource care does not remove inherited project requirements or replace required CI verification.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = careful local resource use within available capacity · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **careful local resource use within available capacity**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-local-resource-careful-bounded-use-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:localResourceScopeRequest`.
  - Observable assertions to implement/control: `localResourceUseChecked`, `localWorkBoundedToAvailableResources`.
  - Future commit-bound artifact: `issue-195-results/i195-local-resource-careful-bounded-use-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-LOCAL-RESOURCE-PROJECT-OWNED-CLEANUP

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Cleanup performed for this task removes only disk space demonstrably owned by meta-language work. Unrelated user files, other projects and system data remain untouched; recovering disk space does not authorize broader deletion.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = cleanup limited to disk space owned by this meta-language task · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **cleanup limited to disk space owned by this meta-language task**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-local-resource-project-owned-cleanup-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:localResourceScopeRequest`.
  - Observable assertions to implement/control: `metaLanguageOwnershipEstablishedBeforeCleanup`, `unrelatedDiskSpacePreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-local-resource-project-owned-cleanup-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-LOCAL-RESOURCE-RUST-COMPILATION-CI-ONLY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** No contributor compiles Rust locally for any purpose during this task. Rust compilation and compiled validation run only in CI/CD; local authoring and bounded JavaScript checks preserve all required native verification obligations.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = Rust compilation exclusively in CI/CD for this task · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/meta-language-local-resource-scope-2026-10-16.txt](../../parity/sources/meta-language-local-resource-scope-2026-10-16.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **Rust compilation exclusively in CI/CD for this task**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-local-resource-rust-compilation-ci-only-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:localResourceScopeRequest`.
  - Observable assertions to implement/control: `noLocalRustCompilation`, `rustCompilationRunsOnlyInCi`.
  - Future commit-bound artifact: `issue-195-results/i195-local-resource-rust-compilation-ci-only-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-AUTOMATED-TRANSLATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Bulk work develops JavaScript behavior first, ports through automated translation where supported and applies the relevant pinned practices from Router PR761, formal-ai PR1188 and Relative Meta Logic PR184. Unsupported translation constructs remain implementation gaps rather than hand-written substitutes or success declarations.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = JavaScript-first automated translation and referenced best practices · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **JavaScript-first automated translation and referenced best practices**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-automated-translation-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `javascriptFirstBulkDevelopment`, `automatedTranslationUsed`, `referencedPracticesAudited`, `unsupportedTranslationGapsRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-automated-translation-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-AUTOMATION-FIRST-COVERAGE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Use as much automation as possible and prioritize general changes that cover many requirements and actual source constructs with the least actions and resources, targeting twenty percent effort for eighty percent results. Reuse existing source inventories, checked contexts, translation workers and reports. Retain every individual failure, source range and unresolved obligation; measured coverage and resource observations are required before claiming the target ratio.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = automated high coverage development with few actions · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **automated high coverage development with few actions**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-automation-first-coverage-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:automationFirstOwnCodebaseBaselineRequest`.
  - Observable assertions to implement/control: `existingAutomationReused`, `actualSourceFailuresGroupedWithoutLoss`, `highCoverageChangesPrioritized`, `resourceAndCoverageClaimsRequireObservations`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-automation-first-coverage-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-BULK-EXACT-SCOPE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Bulk work covers every registered issue, pull-request and conversation obligation, preserving all inherited scope. Drafting, implementation and reporting do not substitute narrower capabilities or omit unresolved requirements.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = one bulk draft of exactly all issue, pull-request and conversation obligations · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **one bulk draft of exactly all issue, pull-request and conversation obligations**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-bulk-exact-scope-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `entireRegisteredScopeIncluded`, `noScopeSubstitutionOrOmission`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-bulk-exact-scope-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-COMPLETE-DRAFT-BEFORE-TESTING

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Draft every outstanding requirement and known CI repair before testing the complete integrated batch. Include shared behavior, both runtime adapters, automatic translation and complete fixtures; refresh generated histories and manifests once after drafting, then execute required validation. Continue independent drafting while existing CI executes, preserve every inherited gate and distinguish unverified drafts from executed evidence.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = complete outstanding requirements drafted before integrated testing · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/bulk-draft-before-testing-2026-10-23.txt](../../parity/sources/bulk-draft-before-testing-2026-10-23.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/bulk-draft-before-testing-2026-10-23.txt](../../parity/sources/bulk-draft-before-testing-2026-10-23.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **complete outstanding requirements drafted before integrated testing**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-complete-draft-before-testing-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:bulkDraftBeforeTestingRequest`.
  - Observable assertions to implement/control: `allOutstandingRequirementsDraftedBeforeTesting`, `knownCiRepairsIncluded`, `metadataIntegratedOnceAfterDrafting`, `completeValidationRetained`, `draftAndEvidenceDistinguished`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-complete-draft-before-testing-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-CONTRIBUTOR-COMMITS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Every contributor with implementation work commits its explicit owned paths, preserving other agents staged changes and retaining reviewable commit provenance for the bulk batch.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = every contributor commits its reviewable work · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **every contributor commits its reviewable work**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-contributor-commits-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `everyContributorWorkCommitted`, `ownedPathsAndCommitProvenanceRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-contributor-commits-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-DRAFT-WITHOUT-COMPILATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Continue bulk drafting all inherited requirements in implementation code while any requirement remains undrafted. A handoff preserves remaining work and does not replace continuing that work. Execute no local code tests or build compilation during this drafting phase; retain every required later compilation, behavioral verification, fixture, assertion and CI gate. Source-only code generation remains drafting, with no original/generated program execution or executable build artifact presented as verification.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = continue actual implementation drafting without local compilation · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/continue-bulk-draft-without-tests-or-compilation-2026-10-27.txt](../../parity/sources/continue-bulk-draft-without-tests-or-compilation-2026-10-27.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/continue-bulk-draft-without-tests-or-compilation-2026-10-27.txt](../../parity/sources/continue-bulk-draft-without-tests-or-compilation-2026-10-27.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **continue actual implementation drafting without local compilation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-draft-without-compilation-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:draftWithoutTestsOrCompilationRequest`.
  - Observable assertions to implement/control: `remainingHandoffWorkContinuesInImplementationCode`, `noLocalBuildCompilationDuringDrafting`, `noCodeTestsBeforeFullDraft`, `allLaterCompilationAndVerificationRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-draft-without-compilation-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-FAST-CONTINUOUS-PROGRESS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Deliver the complete authorized scope as quickly as available resources and correct verification permit, using useful parallel work and coherent batches. Continue independent drafting and implementation while CI runs instead of waiting idle; speed never substitutes for required behavior or evidence.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = fast continuous implementation between bulk feedback cycles · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **fast continuous implementation between bulk feedback cycles**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-fast-continuous-progress-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `independentWorkContinuesDuringCi`, `deliveryLatencyMinimizedWithoutScopeReduction`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-fast-continuous-progress-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-GRADUAL-AGENT-REDUCTION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** After the later user steering, active contributors finish their existing scoped work and agent activity gradually concludes. Once all active contributors finish, the coordinator continues implementation and CI follow-up alone. The earlier parallel drafting phase remains recorded, and no inherited project scope is removed.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = gradual conclusion of agent work followed by coordinator-only implementation · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/gradual-agent-reduction-2026-10-12.txt](../../parity/sources/gradual-agent-reduction-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/gradual-agent-reduction-2026-10-12.txt](../../parity/sources/gradual-agent-reduction-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **gradual conclusion of agent work followed by coordinator-only implementation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-gradual-agent-reduction-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:agentReductionRequest`.
  - Observable assertions to implement/control: `existingAgentWorkConcludesGradually`, `coordinatorWorksAloneAfterAgentCompletion`, `inheritedProjectScopeRetained`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-gradual-agent-reduction-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-LATEST-REPOSITORY

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Before drafting the batch, the coordinator obtains the latest authorized repository and pull-request refs, reconciles required main changes and records the checkout revision used for implementation and CI feedback.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = latest repository version in the shared local checkout · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **latest repository version in the shared local checkout**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-latest-repository-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `latestAuthorizedRefsObtained`, `checkoutRevisionRecorded`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-latest-repository-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-MINIMAL-LOCAL-RESOURCES

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Local work uses the minimum resources needed for targeted JavaScript validation and authoring. Full suites, whole-registry experiments, downstream workload clones and native toolchain matrices remain CI work.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = minimal local resources and targeted JavaScript validation · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **minimal local resources and targeted JavaScript validation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-minimal-local-resources-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `targetedJavaScriptOnlyLocally`, `expensiveChecksDelegatedToCI`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-minimal-local-resources-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-NO-CODE-TESTS-BEFORE-FULL-DRAFT

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Continue drafting exactly the full issue, pull-request and conversation scope in implementation code. Do not run code tests, validation, behavioral comparisons or fixture oracles until every requirement has an implementation draft. Preserve all fixtures, assertions and later verification obligations. A registry entry, carried source, stub or unexecuted test declaration is insufficient implementation drafting, and drafting is distinct from verified delivery.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = no code testing before exactly all requirements have implementation drafts · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/draft-entire-scope-without-code-tests-2026-10-24.txt](../../parity/sources/draft-entire-scope-without-code-tests-2026-10-24.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/draft-entire-scope-without-code-tests-2026-10-24.txt](../../parity/sources/draft-entire-scope-without-code-tests-2026-10-24.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **no code testing before exactly all requirements have implementation drafts**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-no-code-tests-before-full-draft-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:draftEntireScopeWithoutCodeTestsRequest`.
  - Observable assertions to implement/control: `everyRequirementHasImplementationDraft`, `codeExecutionForTestingDeferredUntilWholeDraft`, `allLaterVerificationRetained`, `stubsAndDeclarationsDoNotReplaceBehavior`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-no-code-tests-before-full-draft-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-NO-LOCAL-RUST-COMPILATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** No contributor compiles Rust locally for testing in this workflow. JavaScript validation develops the system behavior, and compiled Rust parity, platform and native execution checks run in CI.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = no local Rust compilation during the requested workflow · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **no local Rust compilation during the requested workflow**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-no-local-rust-compilation-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `noRustCompiledLocally`, `rustExecutionChecksRunInCI`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-no-local-rust-compilation-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-PARALLEL-DELEGATION

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Independent bulk tasks are split among as many useful authorized agents as the available concurrency permits, with clear ownership and integration coordination over the shared checkout.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = maximum useful independent agent delegation · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **maximum useful independent agent delegation**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-parallel-delegation-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `independentTasksDelegated`, `availableConcurrencyUsedWithOwnership`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-parallel-delegation-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-PREVIOUS-CI-ERRORS-ADDRESSED

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Every prior CI/CD error is addressed with concrete implementation and the applicable targeted verification before the integrated feedback push. No failing check is described as intended and no acceptance gate is weakened.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = all previous CI errors addressed before the feedback push · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **all previous CI errors addressed before the feedback push**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-previous-ci-errors-addressed-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `allPreviousCiErrorsAddressed`, `targetedFixVerificationRecorded`, `gatesRemainIntact`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-previous-ci-errors-addressed-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-PREVIOUS-CI-TERMINAL

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Before pushing the next integrated batch, the coordinator verifies the prior relevant CI/CD runs are fully finished and inspects their complete failure feedback. The batch has new concrete requirements or fixes to deliver.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = previous CI fully finished before a new batch push · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **previous CI fully finished before a new batch push**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-previous-ci-terminal-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `previousCiRunsTerminal`, `completeFeedbackInspected`, `newConcreteBatchChangesPresent`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-previous-ci-terminal-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-SINGLE-PUSH-COORDINATOR

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** Exactly one authorized coordinator pushes the integrated batch after reviewing all committed work. Other contributors commit their changes without pushing.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = one coordinator performs batch pushes · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/global-concept-space-2026-10-12.txt](../../parity/sources/global-concept-space-2026-10-12.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **one coordinator performs batch pushes**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-single-push-coordinator-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:globalConceptSpace`.
  - Observable assertions to implement/control: `singlePushCoordinatorEnforced`, `integratedBatchReviewedBeforePush`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-single-push-coordinator-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-WORKFLOW-THREE-SUBAGENT-CURRENT-PHASE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [OPERATIONS — Resource safety, workflow, storage, naming and version policy](issue-227-remaining-work-plan-2026-10-31.md#operations) · P0 ongoing; execution gates drafted with code.

**Exact required behavior:** The current user instruction authorizes at most three subagents to speed useful independent work after the historical coordinator-only phase. Preserve clear source ownership, contributor commits, bounded resource use and one push coordinator; resource-heavy local checks and Git index writes remain serialized. Earlier phase instructions and all project obligations remain recorded.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = up to three subagents for the current parallel implementation phase · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt)
**Vision clause:** [docs/vision.md#critical-conversation-requirements](../../docs/vision.md#critical-conversation-requirements).
**Registered inventories:** [parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt](../../parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [OPERATIONS's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#operations) for this construct: **up to three subagents for the current parallel implementation phase**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.
- [ ] Identify which historical instruction remains applicable under the latest user request; retain superseded source history without executing an obsolete parallelism/phase instruction. Do not interpret an ongoing workflow duty as a missing parser feature.

**Exact deferred verification cells (execution deferred):**

- `i195-workflow-three-subagent-current-phase-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:automationFirstOwnCodebaseBaselineRequest`.
  - Observable assertions to implement/control: `activeSubagentCountAtMostThree`, `independentSourceOwnershipPreserved`, `contributorCommitsAndSinglePushRetained`, `localHeavyChecksSerialized`.
  - Future commit-bound artifact: `issue-195-results/i195-workflow-three-subagent-current-phase-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

## TRACE — Full-scope audit, source register and draft tracking

2 requirements; [implementation/dependency plan](issue-227-remaining-work-plan-2026-10-31.md#trace). Candidate locations: [docs/vision.md](../../docs/vision.md), [parity/issue-195-sources.lino](../../parity/issue-195-sources.lino), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino), [js/scripts/issue-195-requirements.mjs](../../js/scripts/issue-195-requirements.mjs), [docs/handoffs/issue-227-checkpoint-2026-10-31.md](../../docs/handoffs/issue-227-checkpoint-2026-10-31.md).

### I195-CONVERSATION-CURRENT-VISION-REQUIREMENTS

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [TRACE — Full-scope audit, source register and draft tracking](issue-227-remaining-work-plan-2026-10-31.md#trace) · P0 resume prerequisite.

**Exact required behavior:** Every requirement-bearing request in the conversation through this instruction is preserved verbatim with its capture provenance and hash, reflected in docs/vision.md and mapped to critical atomic ledger obligations. This includes default full concrete syntax, optional abstract syntax, recursive any-grammar embedding in the global concept space, initial complete JavaScript Markdown Rust and TypeScript grammars, and Router best-practice reuse. All earlier obligations and source bytes remain intact.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = complete recording of current requests in the authoritative vision and atomic requirements · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt](../../parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt)
**Vision clause:** [docs/vision.md#javascript-first-and-self-translation](../../docs/vision.md#javascript-first-and-self-translation).
**Registered inventories:** [parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt](../../parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [TRACE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#trace) for this construct: **complete recording of current requests in the authoritative vision and atomic requirements**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-conversation-current-vision-requirements-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:routerPracticeRecordingRequest`.
  - Observable assertions to implement/control: `allCurrentMessagesCapturedVerbatimAndHashed`, `currentRequestsReflectedInVisionAndAtomicLedger`, `earlierSourcesRequirementsAndScopesPreserved`.
  - Future commit-bound artifact: `issue-195-results/i195-conversation-current-vision-requirements-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

### I195-PRACTICE-ROUTER-761-REUSE

**Planning category:** No implementation mapping; locate existing code, then draft missing behavior and controls.
**Work package:** [TRACE — Full-scope audit, source register and draft tracking](issue-227-remaining-work-plan-2026-10-31.md#trace) · P0 resume prerequisite.

**Exact required behavior:** Review Router PR761 at a recorded revision and reuse as many applicable ideas as possible for complete original-source inventories, checked semantic links, serialization and reparsing before emission, deterministic regeneration, independent body and proof review, explicit unsupported gaps, same-commit JavaScript-first CI and bounded resources. Record applicability and actual validation; do not blindly import project-specific formats, versions or workflow policies that conflict with this project.

**Full scope:** language = repository tooling (Node.js scripts, documentation and CI workflows) · version = Node.js 22 or later · edition = every checkout and CI job of this repository · construct = review and reuse applicable best practices from Router pull request 761 · aliases = (none) · extensions = (none)
**Required runtimes:** tooling. Critical: yes; full-scope release blocker: yes.

**Authoritative source:** [parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt](../../parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt)
**Vision clause:** [docs/vision.md#javascript-first-and-self-translation](../../docs/vision.md#javascript-first-and-self-translation).
**Registered inventories:** [parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt](../../parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt).

**Implementation references to inspect:**

- tooling: no registered entry point; search the work package’s candidate locations and record a real mapping only after scope review.

**Remaining draft work:**

- [ ] Review the exact behavior/scope above in every required runtime; record the implementation commit, actual constructs and missing code/control gaps. Search existing code before creating a new implementation.
- [ ] Complete [TRACE's implementation sequence](issue-227-remaining-work-plan-2026-10-31.md#trace) for this construct: **review and reuse applicable best practices from Router pull request 761**. Use its candidate locations/dependencies; preserve the entire row scope.
- [ ] Draft independent original/negative/boundary/version controls for every cell below; automatically generate shared native code and complete public integration/mappings. Retain old inputs and assertions.
- [ ] Record fully drafted / partially drafted / undrafted / continuing operational duty with concrete residual gaps. Keep execution and delivery as separate later obligations.

**Exact deferred verification cells (execution deferred):**

- `i195-practice-router-761-reuse-tooling-behavior` — runtime `tooling`, kind `positive`, checkpoint `pre-merge`.
  - Original fixtures: `conversation-contract:routerPracticeRecordingRequest`.
  - Observable assertions to implement/control: `routerPracticeRevisionAndSourcesRecorded`, `applicablePracticesReusedWithTraceability`, `unsupportedAndUnverifiedWorkRemainsExplicit`, `incompatibleProjectPoliciesNotBlindlyCopied`.
  - Future commit-bound artifact: `issue-195-results/i195-practice-router-761-reuse-tooling-behavior.json` (not current evidence).

- [ ] After the global draft gate, execute every declared cell/assertion/fixture and retain actual commit-bound observations; fix every failure without skipped coverage.
**Deferred workflow:** [.github/workflows/ci.yml](../../.github/workflows/ci.yml).
**Registered execution groups:** `suite:javascript`.
**Package obligations:** `repository tooling, not packaged`.

**Fill on resume:** reviewed commit: ___; full-scope draft classification: unknown; remaining code/control gaps: ___; blocking requirement IDs: ___; later actual evidence: ___ (none assessed by this plan).

