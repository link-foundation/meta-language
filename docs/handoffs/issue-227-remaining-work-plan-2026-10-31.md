# Issue #227 / PR #228 detailed remaining-work plan — 2026-10-31

This is a handoff and future execution plan, not another architecture specification. [The vision](../vision.md), registered requirement sources and atomic manifest remain authoritative. The user requested detailed planning after the stop checkpoint. Production development and behavioral verification remain paused; this documentation update does not resume the active goal.

## Snapshot and coverage

- Reviewed register snapshot: `9405c25fbbcb101d39953e188608bcd6d8734664`; latest implementation remains `2cf908a6d2a0b54009746cb65eabd7dcefffc505`.
- Atomic manifest: [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino); SHA-256 `3693e490c73507b0850bb9ee4a6d2aac540b8dac5d3e607f61f74fe8f9d756ae`.
- **473 requirements, 80 without implementation mapping, 393 mapped for every required runtime, 1017 declared verification cells.** Mapping coverage is 83.1%, not a full-draft or delivery percentage. All registered paths resolve through the project’s JSON-to-Links compatibility adapter.
- Every requirement is included below or in an appendix. None is excluded because it has paths, a fixture, an old PASS or a capability declaration. Full-scope draft classifications and current completion remain unproven.
- Live source audit for this plan matched 66 requirement sources/14 reports; receipt: `issue-195-artifacts/source-authoring/handoff-plan-source-audit-2026-10-31.log`. No behavioral tests, compilation, acceptance gates or CI feedback were executed/read.

## Reading order

1. Read [the stop checkpoint](issue-227-checkpoint-2026-10-31.md), latest user steering and the authoritative vision.
2. Use the work packages below to select full batches and dependencies. The first milestone is actual own-code JavaScript/Rust translation, not more isolated subset extensions.
3. Open [all 80 unmapped requirements](issue-227-unmapped-requirements-2026-10-31.md) to distinguish existing-but-unregistered code from missing implementation.
4. Open [all 393 mapped requirements](issue-227-mapped-requirements-review-2026-10-31.md) for source review and remaining implementation/control work. Mapped rows must not disappear from the backlog.
5. Update row review notes with a real commit and construct-level gaps before reporting a full-draft percentage.

## Tracking rules and phase boundary

Keep three distinct measurements: **mapping coverage**, **full-scope code/control drafting**, and **executed requirement delivery**. The counts above establish only the first. The appendices contain unchecked planning tasks and future evidence paths; no task is presented as an observed pass. Do not add completion fields to executable evidence or fabricate observations.

For each row, a future source audit must record fully drafted / partially drafted / undrafted / continuing operational duty, exact reviewed commit, implemented constructs, remaining code/control gaps and blocking requirement IDs. A fully drafted row requires executable full-scope behavior and independent control code for every declared assertion/fixture/runtime, public integration and reproducible native generation; a stub, preserved source envelope, test plan, mapping or unsupported descriptor does not qualify. Real execution, publishing and consumer results remain later delivery obligations.

Draft the later CI/control/publication procedures now without executing them. Once every row’s actual code/control draft is complete, start the verification/CI-fix phase; do not wait for unexecuted delivery evidence to exist before permitting that phase, and do not claim delivery merely because drafting is complete. Keep historical verified observations separate and tied to their original commits.

The latest user steering keeps implementation paused and work solo. Earlier delegation/count requests are preserved historical obligations, not authorization to spawn agents now. Never compile Rust locally. Keep disk use bounded; remove only proven task-owned regenerable scratch and retain evidence/receipts. Source-only generation must not execute represented fixture programs.

## Ordered milestones

1. **Source audit and work ledger:** TRACE plus OPERATIONS; statically classify every row and build a complete own-source construct inventory.
2. **Critical baseline code draft:** BASELINE + priority GRAMMAR + MEANING/EDIT/EMBED dependencies. Select complete batches by the real modules they unblock; retain all broader language obligations.
3. **Remaining full code draft:** finish all catalog rows and FORMAL, MERGE, DYNAMIC and INFERENCE; draft EVIDENCE and DELIVERY automation/controls in parallel where independent, while staying within the user’s current solo constraint.
4. **Global draft audit:** every row has full-scope executable code and independent controls, exact current mappings and no undisclosed construct gap. Missing formal proof/decision-procedure code remains a draft gap.
5. **Verification and CI repair:** run all declared cells on exact commits, inspect failures, repair actual behavior and repeat coherent batches. Native compilation, full corpora, provers and clean consumers remain CI-only.
6. **Release and final delivery:** matching published packages, exact downstream installations/workloads, required merge/main reports and every ledger obligation established. Only then consider completion/closing directives.

## Work packages

Dependencies are architectural/data prerequisites. Do not create a schedule deadlock by requiring a package’s later execution evidence before another package’s source can be drafted. Execution of these packages requires the user to resume development.

### BASELINE

**Own-code JavaScript ↔ Rust baseline** · P1. 17 requirements, 5 unmapped.

Prerequisites: [TRACE](#trace), [GRAMMAR](#grammar), [MEANING](#meaning), [EDIT](#edit). Search/reuse: [js/src/self-translation.js](../../js/src/self-translation.js), [js/src/project-translation.js](../../js/src/project-translation.js), [js/src/project-translation-archive.js](../../js/src/project-translation-archive.js), [js/src/translation](../../js/src/translation), [rust/src/self_translation.rs](../../rust/src/self_translation.rs), [rust/src/project_translation.rs](../../rust/src/project_translation.rs), [docs/self-translation.md](../../docs/self-translation.md). Paths are candidates for review, not proof of requirement coverage.

1. Freeze complete tracked source inventories for this repository and the named downstream projects. Record original commit, path, byte/hash, language edition and dependency/std versions for every input; retain unresolved imports explicitly.
2. Build one construct-to-source inventory for every JavaScript/Rust item and generated runtime prelude. Classify grammar, binding, typing, operation/effect, ownership/lifetime, emitter and project-linking gaps separately. Use it to select complete batches that unblock the largest real modules; do not choose another small isolated feature without an inventory dependency.
3. Draft complete JavaScript classes/prototypes/descriptors, arrays/mutation, closures/defaults, module interfaces, asynchronous/effect behavior and standard-library semantics used by the inventories. Draft the corresponding Rust generics/traits, borrowing/moves/drops, macros, associated items, constants and standard-library interfaces. Preserve each requirement’s broader scope beyond those minimum projects.
4. Connect both directions through editable typed Links and complete original CSTs. Emit from the edited current graph; saved originals and carried source are provenance/gaps, never replacement executable semantics. Keep diagnostics and unsupported constructs visible until implemented.
5. Author semantic-original versus actual-generated-product controls, source reconstruction, no-carried-item checks, cross-module signatures and source-version bounds. Automatically translate the complete shared JavaScript batch to Rust; fix the translator for unsupported constructs.
6. Draft whole-project automation and clean-consumer controls now. Execute originals, generated products, lossless round trips and runtime parity only after the global code-draft gate; full own-code baseline then becomes the first verification milestone.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### GRAMMAR

**Full CST grammars and source unions** · P1 for JavaScript/Rust, then TypeScript/Markdown; P2 rest. 155 requirements, 9 unmapped.

Prerequisites: [TRACE](#trace). Search/reuse: [js/src/native-grammar-parser.js](../../js/src/native-grammar-parser.js), [js/src/grammar-lowering.js](../../js/src/grammar-lowering.js), [js/src/grammar-reverse.js](../../js/src/grammar-reverse.js), [js/src/data/native-grammars](../../js/src/data/native-grammars), [rust/src/native_grammar_parser.rs](../../rust/src/native_grammar_parser.rs), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino). Paths are candidates for review, not proof of requirement coverage.

1. For each language/edition/alias listed below, inventory every applicable pinned official and mature grammar source, scanners and real-source corpus; map each construct to the union and preserve disagreements. Registration or one upstream grammar does not cover the whole row.
2. Download external grammars only into task-owned ignored temporary storage. Record revisions, licenses and hashes; automatically convert imports and corpus data to Links. Keep external source grammars and binaries out of Git and production packages.
3. Implement generic lowering for every outstanding grammar operator and scanner family: lexical contexts, remembered/counting delimiters, quotes/interpolation, fenced blocks, offside/layout and recovery. Encode adjustments as shared rules/decorators, not generated-file edits.
4. Generate full native rules automatically. CST is the default: tokens, trivia, fields, hierarchy, order, spans, recovery and embedding boundaries must survive both codecs. AST is an explicit projection with provenance to the same complete CST.
5. Complete JavaScript and Rust source coverage first, then TypeScript/TSX and Markdown. Apply the same generation pipeline to every remaining catalog row; document binary/document/natural-language handling explicitly rather than substituting plain text.
6. Author independent accepted/rejected/ambiguous/error/Unicode/real-project corpus controls per construct and source discrepancy. Tree-sitter and other external engines are independent test oracles only. Native independence and whole-catalog coverage are deferred CI proof obligations.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### EMBED

**Recursive any-grammar parsing** · P1 substrate, P2 full catalog. 11 requirements, 5 unmapped.

Prerequisites: [GRAMMAR](#grammar), [MEANING](#meaning). Search/reuse: [js/src/dynamic-language-cli.js](../../js/src/dynamic-language-cli.js), [js/src/native-grammar-parser.js](../../js/src/native-grammar-parser.js), [js/src/source-project-archive.js](../../js/src/source-project-archive.js), [rust/src/native_grammar_parser.rs](../../rust/src/native_grammar_parser.rs), [docs/vision.md](../../docs/vision.md). Paths are candidates for review, not proof of requirement coverage.

1. Locate the existing embedding dispatch and retained-boundary models before creating another parser. Model language selection, host/child grammar references, delimiters/escapes and return-to-host continuation as inspectable Links rules.
2. Implement one any/all-grammar invocation path across supported definitions; avoid a bespoke handler for every host/child pair. Share the same global concept identities through same-language recursion and nested language changes.
3. Carry complete parent/child CST hierarchy and original byte/unit span mappings through parse, edit, serialize, reload and emission. Preserve exact enclosing delimiters/trivia when nested source changes.
4. Define deterministic ambiguity/selection diagnostics and non-consuming-cycle/resource-budget handling. No level may silently omit a child tree or flatten it into an opaque token.
5. Author the requested Markdown→JavaScript→JSON→Markdown/Python example, Markdown within Markdown, repeated-language recursion, logs, recovery, escaped boundaries and malformed/non-consuming cycles for both runtimes/codecs. Execution is deferred.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### MEANING

**Global concepts, recursive definitions and link foundations** · P1 architecture. 32 requirements, 8 unmapped.

Prerequisites: [TRACE](#trace). Search/reuse: [js/src/global-concept-space.js](../../js/src/global-concept-space.js), [js/src/formal-concept-definition.js](../../js/src/formal-concept-definition.js), [js/src/concept-ontology.js](../../js/src/concept-ontology.js), [js/src/concept-views.js](../../js/src/concept-views.js), [rust/src/global_concept_space.rs](../../rust/src/global_concept_space.rs), [rust/src/formal_concept_definition.rs](../../rust/src/formal_concept_definition.rs), [parity/fixtures/formal-concept-definitions.lino](../../parity/fixtures/formal-concept-definitions.lino). Paths are candidates for review, not proof of requirement coverage.

1. Audit the pinned associative stack and meta-theory sources against both packages, grammar/transformation tools and dependencies. Classify direct runtime use, compatibility/theory and development tools; record concrete deviations and repair work without importing unrelated dependencies.
2. Represent every concept/entity/relationship/root as links. Preserve self-loop vertex representation, multiple roots and optional type roots; distinguish ontology relations from taxonomy specialization and expose both as library entry points.
3. Make every grammar use the same global concept space. Copy source-qualified translated concepts first, preserving distinct identities, definitions and source provenance instead of coalescing equal names.
4. Complete recursively inspectable formal definitions, binders, axioms, operations, foundations and declared primitives, including mutual/cyclic references. Serialize/reload stable references and assumptions without flattening definitions into prose or opaque signatures.
5. Implement checked one-to-one meaning correspondence over those complete definitions before sharing concepts. Preserve unresolved equivalence obligations and counterexamples; structural resemblance or equal labels alone never authorizes deduplication.
6. Audit exact unsigned-64-bit identity transport throughout parser/CST/concept/schema/CLI APIs, not only graph codecs. Author complete concept/definition/equivalence/root/round-trip controls for both runtimes. Keep physical-wave analogies attributed as philosophical context rather than empirical proofs.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### EDIT

**Binding-safe structured edits and transformations** · P1 baseline dependency. 36 requirements, 0 unmapped.

Prerequisites: [GRAMMAR](#grammar), [MEANING](#meaning). Search/reuse: [js/src/translation](../../js/src/translation), [js/src/typed-program-links.js](../../js/src/typed-program-links.js), [js/src/network.js](../../js/src/network.js), [rust/src/translation](../../rust/src/translation), [rust/src/typed_program_links.rs](../../rust/src/typed_program_links.rs). Paths are candidates for review, not proof of requirement coverage.

1. Audit each query/construct/clone/insert/delete/replace/move/emit row against real editable graph operations, actual current source and every declared language; map ownership of syntax, binders, references, types and provenance.
2. Complete lexical scope and capture avoidance through module, nested, recursive and generated constructs. Renaming/moving a binder must update only its actual references and preserve shadowing, raw/Unicode names and proof dependencies.
3. Maintain CST/source mappings and invalidate/rebuild affected semantic structures after edits. Emission must observe current graph state and exact selected roots; stale cached structures or saved-source restoration cannot mask edits.
4. Author independent mutation sequences, alpha-equivalence/capture counterexamples, ambiguous/invalid edits, codec reloads and actual compiler/prover consumers. Retain the existing positive and negative assertion inputs and defer execution.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### MERGE

**Grammar import, conversion and automatic reconciliation** · P2; source-union support P1. 19 requirements, 4 unmapped.

Prerequisites: [GRAMMAR](#grammar), [MEANING](#meaning), [EDIT](#edit). Search/reuse: [js/src/grammar-interchange.js](../../js/src/grammar-interchange.js), [js/src/grammar-merge.js](../../js/src/grammar-merge.js), [js/src/grammar-merge-decisions.js](../../js/src/grammar-merge-decisions.js), [js/src/grammar-reverse.js](../../js/src/grammar-reverse.js), [parity/fixtures/merge-quality-evidence.lino](../../parity/fixtures/merge-quality-evidence.lino), [parity/fixtures/grammar-merge.lino](../../parity/fixtures/grammar-merge.lino). Paths are candidates for review, not proof of requirement coverage.

1. Inventory every required grammar format, extension, feature and conversion direction in the rows; implement the complete feature union and exact source/provenance retention rather than only parse/export capability labels.
2. Use binding-aware deterministic renaming and meaning-backed reconciliation across independent sources. Keep uncertainty, alternatives and conflicts explicit; accept shared concepts only after MEANING correspondence obligations pass.
3. Make shipped native grammars products of the actual automatic merge pipeline, retaining pinned input identities and reproducible decisions. Record rules/decorators for adjustments and avoid generated-source edits.
4. Author actual import→links→export→external-consumer cycles, deterministic reorderings, binding collisions and real source disagreements. Record quality measurements only from later independent executions, never from declared success fields.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### DYNAMIC

**Dynamic Links reconstruction and RDF/OWL interoperability** · P2. 14 requirements, 11 unmapped.

Prerequisites: [MEANING](#meaning), [GRAMMAR](#grammar), [EDIT](#edit). Search/reuse: [js/src/source-project-archive.js](../../js/src/source-project-archive.js), [js/src/dynamic-language-cli.js](../../js/src/dynamic-language-cli.js), [js/src/unsigned-link-network.js](../../js/src/unsigned-link-network.js), [js/src/global-concept-space.js](../../js/src/global-concept-space.js), [rust/src/global_concept_space.rs](../../rust/src/global_concept_space.rs), [docs/vision.md](../../docs/vision.md). Paths are candidates for review, not proof of requirement coverage.

1. Audit existing snapshot, exact-ID, dynamic-language and archive implementations for each unmapped row before adding new modules. Reconstruct full CSTs, grammar definitions, concepts and semantics from retained text/binary Links without hidden authoritative side tables or reparsing saved source.
2. Complete public library and link-cli reconstruction/extension paths. Preserve selected roots, IDs, allocator state, definitions, version/source provenance and exact binary/text behavior; keep static prepared snapshots as an initial mode, not a permanent restriction.
3. Implement RDF and OWL import, export/conversion and merge with explicit IRI, blank-node, literal/datatype/language-tag, graph and ontology semantics. Keep unsupported profiles/conflicts explicit and preserve distinctions not proved equivalent.
4. Connect dynamic language additions to the same grammar/concept algorithms and semantic translations; demonstrate extension without rebuilding a hand-curated language table.
5. Author complete reconstruction, extension, merge, lossless codec, malformed/ambiguous input and resource-bound controls. Draft the later performance measurement harness and retain independent source/oracle datasets; measure only during verification.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### INFERENCE

**Automatic grammar discovery and comparative research** · P2 after baseline substrate. 14 requirements, 12 unmapped.

Prerequisites: [GRAMMAR](#grammar), [MEANING](#meaning), [DYNAMIC](#dynamic). Search/reuse: [js/src/grammar-inference.js](../../js/src/grammar-inference.js), [js/src/grammar-inference-refinement.js](../../js/src/grammar-inference-refinement.js), [js/src/grammar-inference-rules.js](../../js/src/grammar-inference-rules.js), [rust/src/grammar/inference](../../rust/src/grammar/inference), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino). Paths are candidates for review, not proof of requirement coverage.

1. Inspect the existing inference implementation and registered issue first. Retain the separate discovery issue and link its unfinished scope to this handoff; do not create a duplicate if it already exists.
2. Re-read the exact pinned papers and implementations named by the requirements, including XVada, Crucio, NatGI and attributed methods. Record revisions, licenses, datasets, oracle access and reproducible configurations; recent-paper findings require primary-source research when resumed.
3. Draft generic active discovery, deterministic refinement/minimization, feature union and dynamic translation/integration paths in inspectable shared definitions. Emit native grammars and formal/global concept references through the ordinary pipelines.
4. Complete public API/CLI and language-independent corpus/oracle adapters. Preserve uncertainty and held-out rejection behavior; a paper’s reported results or one toy grammar cannot establish project coverage.
5. Author comparative/held-out independent benchmarks and correctness/coverage/quality/resource metrics with fixed seeds and budgets. Keep downloads/clones/large experiments and native compilation in CI; no benchmark execution before the complete code draft.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### FORMAL

**Lean/Rocq semantics and all directed translations** · P2 after own-code baseline; preserve scope now. 52 requirements, 0 unmapped.

Prerequisites: [BASELINE](#baseline), [MEANING](#meaning), [EDIT](#edit). Search/reuse: [js/src/translation](../../js/src/translation), [rust/src/translation](../../rust/src/translation), [parity/language-features.lino](../../parity/language-features.lino), [docs/vision.md](../../docs/vision.md). Paths are candidates for review, not proof of requirement coverage.

1. Inventory each language’s complete attributes, effects, recursion, scopes, modules/project dependencies, macros/notation, elaboration traces, types/universes, proof terms and tactics. Retain original language/edition/construct scopes in every row below.
2. Draft shared inspectable semantic representations with explicit foundations and side conditions; bind proof dependencies and effects to actual source owners. Preserve distinctions where cross-language meaning is not established.
3. Implement every declared directed language pair through edited common Links, including both source parsing and target emission. Do not replace missing semantics with carried originals, unsupported descriptors or handwritten Rust replacements.
4. Author independent native compiler/prover and real-project controls, proof/effect/abort preservation, mutations and lossless graph transports. Schedule all native Lean/Rocq/Rust compilation and execution in CI after the global draft gate.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### EVIDENCE

**Independent acceptance, parity and failure-detecting gates** · P1 control authorship; execution after full draft. 33 requirements, 1 unmapped.

Prerequisites: [TRACE](#trace), [BASELINE](#baseline), [GRAMMAR](#grammar), [MEANING](#meaning), [EDIT](#edit), [MERGE](#merge), [DYNAMIC](#dynamic), [INFERENCE](#inference), [FORMAL](#formal). Search/reuse: [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [js/scripts/check-issue-195-acceptance.mjs](../../js/scripts/check-issue-195-acceptance.mjs), [js/scripts/run-issue-195-evidence.mjs](../../js/scripts/run-issue-195-evidence.mjs), [js/scripts/issue-195-acceptance-lib.mjs](../../js/scripts/issue-195-acceptance-lib.mjs), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino). Paths are candidates for review, not proof of requirement coverage.

1. Audit all 1,017 declared verification cells and every assertion/fixture pairing below. Replace merely planned or declaration-only positive fixtures with actual originals and independent expectations; retain protected inputs and coverage.
2. Draft complete JavaScript/native parity, grammar/oracle corpora, generated/metamorphic/fuzz/edit-sequence controls and real downstream workloads. Match the scope of each universal claim with explicit proofs/decision procedures or retain its unresolved obligation.
3. Ensure fault injections fail for removed features, fallback parsers, carried/relabeled translations, stale structures, weakened oracles, skipped jobs/cells and dropped source rows. Preserve existing gates and assertions; correct wrong expectations only with recorded evidence and version ranges.
4. After every requirement has a full-scope code/control draft, inspect CI feedback, fix failures in coherent batches and rerun the appropriate controls. Rust follows passing JavaScript; long native/prover/corpus/consumer jobs remain CI-only and report every failure.
5. Record observations only after actual assertions execute at the recorded commit, for every declared fixture/runtime. No source generation, manifest path, feature flag, old PASS report or coverage count establishes a current successful requirement.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### DELIVERY

**Dependencies, matching releases and downstream consumers** · P3 execution after acceptance readiness. 15 requirements, 3 unmapped.

Prerequisites: [BASELINE](#baseline), [GRAMMAR](#grammar), [MERGE](#merge), [DYNAMIC](#dynamic), [FORMAL](#formal), [EVIDENCE](#evidence). Search/reuse: [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [js/package.json](../../js/package.json), [rust/Cargo.toml](../../rust/Cargo.toml), [parity/language-features.lino](../../parity/language-features.lino), [docs/vision.md](../../docs/vision.md). Paths are candidates for review, not proof of requirement coverage.

1. Draft reproducible clean-consumer candidate builds/installations for matching npm/crates releases, with complete package assets, declarations, binaries and public APIs. Retain pre-merge input locks for required PR gates and separate non-blocking current-main/upstream reports.
2. Audit/remove remaining external production grammar engines or vendored binaries after native replacements cover their actual scope. Retain external independent oracles only in development/testing; reconcile current stable dependencies and document compatibility/version boundaries.
3. Prepare pinned formal-ai, relative-meta-logic and other required consumer inventories, projection rules and Rust/JavaScript/TypeScript workloads. Use the ordinary published interfaces and both runtimes, preserving complete source and executable semantics.
4. Only after acceptance and independent consumer evidence, publish matching approved releases, verify exact installed artifacts and downstream workloads, reconcile main reports and perform the required merge/continuation actions. Publication, merge and delivery are not satisfied by a local draft or old CI result.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### OPERATIONS

**Resource safety, workflow, storage, naming and version policy** · P0 ongoing; execution gates drafted with code. 65 requirements, 20 unmapped.

Prerequisites: [TRACE](#trace). Search/reuse: [AGENTS.md](../../AGENTS.md), [CLAUDE.md](../../CLAUDE.md), [.gitignore](../../.gitignore), [.github/workflows/ci.yml](../../.github/workflows/ci.yml), [scripts/clean-caches.mjs](../../scripts/clean-caches.mjs), [scripts/with-cache-cleanup.mjs](../../scripts/with-cache-cleanup.mjs), [parity/naming/canonical-concepts.lino](../../parity/naming/canonical-concepts.lino). Paths are candidates for review, not proof of requirement coverage.

1. Reconcile historical workflow requests with the latest user steering: production development is paused, work remains solo, no local Rust compilation and no behavioral tests/validation gates or CI-feedback fixes before the full draft. Preserve superseded source rows without reactivating obsolete agent-count or push instructions.
2. Maintain task-owned resource limits, shared checkout, safe cache receipts and targeted cleanup. Never delete unrelated data, dirty worktrees, active tasks, journals or retained evidence. Verify inherited Cargo target ownership before any cleanup; keep long corpus/consumer/native jobs in CI.
3. Draft stage ordering, single-pusher batching, complete failure reporting and cancellation diagnostics. Resume implementation while CI runs; do not use waiting for CI as a reason to stop drafting. User-requested stop/checkpoint documentation is a distinct authorized action.
4. Complete automated JSON-to-Links conversion and documented unavoidable external JSON boundaries; retain compatibility aliases. Ignore temporary external grammars, record download provenance and keep source artifacts out of the tracked tree.
5. Keep AGENTS/CLAUDE concise; permit evidence-backed correction of wrong assertions while retaining inputs/coverage. Select version-sensitive expectations from explicit active language/dependency ranges and diagnose uncovered versions.
6. Use canonical English concept nouns and operation verbs, linked vocabulary/abbreviation records and WordNet fixtures. Draft naming/resource/workflow enforcement controls without weakening gates; execute them only in the permitted phase.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

### TRACE

**Full-scope audit, source register and draft tracking** · P0 resume prerequisite. 10 requirements, 2 unmapped.

Prerequisites: latest source audit and user steering. Search/reuse: [docs/vision.md](../../docs/vision.md), [parity/issue-195-sources.lino](../../parity/issue-195-sources.lino), [parity/issue-195-requirements.lino](../../parity/issue-195-requirements.lino), [js/scripts/issue-195-requirements.mjs](../../js/scripts/issue-195-requirements.mjs), [docs/handoffs/issue-227-checkpoint-2026-10-31.md](../../docs/handoffs/issue-227-checkpoint-2026-10-31.md). Paths are candidates for review, not proof of requirement coverage.

1. Read authoritative vision and the latest user steering. Perform the required live source audit, re-read edited/new requirement sources, reconcile obligations and refresh the hashed register only after review. Carry every recorded source/scope/alias/runtime/assertion forward.
2. Statically audit all 473 rows below against current code, source generators and independent control definitions. The 80 missing mappings need code discovery before new implementation; the 393 mapped rows need full-scope review, not an automatic done label.
3. For each row, record reviewed commit, concrete implemented behavior/constructs, exact remaining code/control gaps and blocking prerequisite IDs. Classify fully drafted, partially drafted, undrafted or continuing operational obligation only after that review; keep behavior verification separate.
4. Use the source requirements to assess Router #761 and the other named project practices. Record applicability, reused mechanisms and justified deviations in the authoritative vision/traceability, not a competing architecture specification.
5. Update the atomic manifest through its generator and connect real entry points, actual fixtures and future evidence cells. Report mapping coverage, full-scope draft coverage and executed verification coverage separately; never turn a path mapping into completion evidence.

**Draft outputs:** exact row mappings, complete shared/source/public integration changes, native generation recipe where applicable, independent controls for every cell below and explicit remaining source-review notes. **Later delivery outputs:** actual commit-bound assertion observations and required native/consumer/operational artifacts.

## Area coverage index

Every area has a work package and every row has a detailed checklist.

| Requirement area | Total | Unmapped | Mapped, review pending | Work package |
| --- | ---: | ---: | ---: | --- |
| acceptance-gate-fault-injection | 11 | 0 | 11 | [EVIDENCE](#evidence) |
| assertion-version-ranges | 3 | 0 | 3 | [OPERATIONS](#operations) |
| associative-data-storage | 3 | 0 | 3 | [OPERATIONS](#operations) |
| associative-foundation | 5 | 3 | 2 | [MEANING](#meaning) |
| automatic-merge | 8 | 2 | 6 | [MERGE](#merge) |
| binding-safe-transformation | 4 | 0 | 4 | [EDIT](#edit) |
| cache-cleanup | 8 | 0 | 8 | [OPERATIONS](#operations) |
| continuous-integration | 5 | 0 | 5 | [OPERATIONS](#operations) |
| contributor-instructions | 2 | 0 | 2 | [OPERATIONS](#operations) |
| conversation-contract | 2 | 0 | 2 | [TRACE](#trace) |
| conversation-practice-reuse | 2 | 2 | 0 | [TRACE](#trace) |
| default-cst | 80 | 0 | 80 | [GRAMMAR](#grammar) |
| dependency-freshness | 3 | 1 | 2 | [DELIVERY](#delivery) |
| directed-translation | 12 | 0 | 12 | [FORMAL](#formal) |
| documentation-consistency | 1 | 0 | 1 | [TRACE](#trace) |
| downstream-consumers | 6 | 2 | 4 | [DELIVERY](#delivery) |
| dynamic-knowledge | 11 | 11 | 0 | [DYNAMIC](#dynamic) |
| embedded-language | 5 | 0 | 5 | [EMBED](#embed) |
| external-conformance | 4 | 0 | 4 | [EVIDENCE](#evidence) |
| external-grammar-storage | 3 | 0 | 3 | [OPERATIONS](#operations) |
| formal-concept-definitions | 3 | 3 | 0 | [MEANING](#meaning) |
| four-language-semantics | 40 | 0 | 40 | [FORMAL](#formal) |
| full-semantics | 4 | 1 | 3 | [BASELINE](#baseline) |
| generative-testing | 4 | 0 | 4 | [EVIDENCE](#evidence) |
| global-concept-space | 9 | 0 | 9 | [MEANING](#meaning) |
| grammar-importer | 5 | 0 | 5 | [MERGE](#merge) |
| grammar-inference | 14 | 12 | 2 | [INFERENCE](#inference) |
| grammar-interchange | 6 | 2 | 4 | [MERGE](#merge) |
| independent-acceptance | 5 | 1 | 4 | [EVIDENCE](#evidence) |
| javascript-first | 7 | 0 | 7 | [BASELINE](#baseline) |
| links-foundation | 6 | 2 | 4 | [MEANING](#meaning) |
| links-notation-fitness | 3 | 0 | 3 | [DYNAMIC](#dynamic) |
| modern-javascript-parity | 2 | 0 | 2 | [BASELINE](#baseline) |
| native-grammar | 70 | 4 | 66 | [GRAMMAR](#grammar) |
| native-validation | 4 | 0 | 4 | [EVIDENCE](#evidence) |
| ontology-and-taxonomy | 6 | 0 | 6 | [MEANING](#meaning) |
| own-language-grammar-priority | 5 | 5 | 0 | [GRAMMAR](#grammar) |
| package-delivery | 6 | 0 | 6 | [DELIVERY](#delivery) |
| readable-naming | 4 | 0 | 4 | [OPERATIONS](#operations) |
| recursive-concrete-syntax | 6 | 5 | 1 | [EMBED](#embed) |
| resource-limits | 15 | 0 | 15 | [OPERATIONS](#operations) |
| runtime-parity | 5 | 0 | 5 | [EVIDENCE](#evidence) |
| shared-concepts | 3 | 0 | 3 | [MEANING](#meaning) |
| structured-transformation | 32 | 0 | 32 | [EDIT](#edit) |
| translation-priority | 4 | 4 | 0 | [BASELINE](#baseline) |
| vision-and-traceability | 5 | 0 | 5 | [TRACE](#trace) |
| workflow-contract | 22 | 20 | 2 | [OPERATIONS](#operations) |

## Complete requirement index

This index contains every manifest ID exactly once. “Mapped” means only that all required runtime entry-point lists are nonempty. It is not a full-draft classification. Each link carries the exact original behavior/scope, implementation references, a draft sequence, all fixture IDs/assertions/test IDs and deferred evidence path.

| Requirement | Category | Work package |
| --- | --- | --- |
| [I195-ACCEPTANCE-FINITE-CLAIMS-DOCUMENTED](issue-227-mapped-requirements-review-2026-10-31.md#i195-acceptance-finite-claims-documented) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-ACCEPTANCE-GATE-MUTATIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-acceptance-gate-mutations) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-ACCEPTANCE-INDEPENDENT-ORACLES](issue-227-unmapped-requirements-2026-10-31.md#i195-acceptance-independent-oracles) | Unmapped; code discovery/draft pending | [EVIDENCE](#evidence) |
| [I195-ACCEPTANCE-PR-CHECKS-PASSABLE](issue-227-mapped-requirements-review-2026-10-31.md#i195-acceptance-pr-checks-passable) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-ACCEPTANCE-REQUIRED-MERGE-CHECK](issue-227-mapped-requirements-review-2026-10-31.md#i195-acceptance-required-merge-check) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-ASSERTION-EVIDENCE-BASED-CORRECTION](issue-227-mapped-requirements-review-2026-10-31.md#i195-assertion-evidence-based-correction) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSERTION-EXPLICIT-VERSION-RANGES](issue-227-mapped-requirements-review-2026-10-31.md#i195-assertion-explicit-version-ranges) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSERTION-LANGUAGE-EDITION-ORDER](issue-227-mapped-requirements-review-2026-10-31.md#i195-assertion-language-edition-order) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSERTION-VERSION-COVERAGE](issue-227-mapped-requirements-review-2026-10-31.md#i195-assertion-version-coverage) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSOCIATIVE-DATA-AUTOMATED-CONVERSION](issue-227-mapped-requirements-review-2026-10-31.md#i195-associative-data-automated-conversion) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSOCIATIVE-DATA-JSON-EXCEPTIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-associative-data-json-exceptions) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSOCIATIVE-DATA-STORAGE](issue-227-mapped-requirements-review-2026-10-31.md#i195-associative-data-storage) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-ASSOCIATIVE-STACK-PINNED-SOURCE-AUDIT](issue-227-mapped-requirements-review-2026-10-31.md#i195-associative-stack-pinned-source-audit) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-ASSOCIATIVE-STACK-PROJECT-WIDE-USE](issue-227-unmapped-requirements-2026-10-31.md#i195-associative-stack-project-wide-use) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-CACHE-CLEANUP-BUDGET](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-budget) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-ENTRY-POINT](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-entry-point) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-EVENTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-events) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-GRAMMAR-CACHES](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-grammar-caches) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-HOOK](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-hook) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-MEASURED](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-measured) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-POLICY](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-policy) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CACHE-CLEANUP-SAFETY](issue-227-mapped-requirements-review-2026-10-31.md#i195-cache-cleanup-safety) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CI-ACCEPTANCE-AFTER-JAVASCRIPT](issue-227-mapped-requirements-review-2026-10-31.md#i195-ci-acceptance-after-javascript) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CI-ACCEPTANCE-STAGE-ORDER](issue-227-mapped-requirements-review-2026-10-31.md#i195-ci-acceptance-stage-order) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CI-REPORT-EVERY-FAILURE](issue-227-mapped-requirements-review-2026-10-31.md#i195-ci-report-every-failure) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CI-RUST-AFTER-JAVASCRIPT](issue-227-mapped-requirements-review-2026-10-31.md#i195-ci-rust-after-javascript) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CI-SKIPPED-STAGE-ONE-GATE-ERROR](issue-227-mapped-requirements-review-2026-10-31.md#i195-ci-skipped-stage-one-gate-error) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CONFORMANCE-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-conformance-javascript) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-CONFORMANCE-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-conformance-lean) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-CONFORMANCE-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-conformance-rocq) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-CONFORMANCE-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-conformance-rust) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-CONTRIBUTOR-INSTRUCTIONS-CONCISE](issue-227-mapped-requirements-review-2026-10-31.md#i195-contributor-instructions-concise) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-CONVERSATION-CRITICAL-DELIVERY](issue-227-mapped-requirements-review-2026-10-31.md#i195-conversation-critical-delivery) | Mapped; full review pending | [TRACE](#trace) |
| [I195-CONVERSATION-CURRENT-VISION-REQUIREMENTS](issue-227-unmapped-requirements-2026-10-31.md#i195-conversation-current-vision-requirements) | Unmapped; code discovery/draft pending | [TRACE](#trace) |
| [I195-CONVERSATION-VERBATIM-REGISTER](issue-227-mapped-requirements-review-2026-10-31.md#i195-conversation-verbatim-register) | Mapped; full review pending | [TRACE](#trace) |
| [I195-CST-agda](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-agda) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-ANY-GRAMMAR-RECURSIVE-RULE](issue-227-unmapped-requirements-2026-10-31.md#i195-cst-any-grammar-recursive-rule) | Unmapped; code discovery/draft pending | [EMBED](#embed) |
| [I195-CST-bash](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-bash) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-bengali](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-bengali) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-c](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-c) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-c-plus-plus](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-c-plus-plus) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-c-sharp](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-c-sharp) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-cmake](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-cmake) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-css](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-css) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-csv](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-csv) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-dart](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-dart) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES](issue-227-unmapped-requirements-2026-10-31.md#i195-cst-deeply-recursive-mixed-languages) | Unmapped; code discovery/draft pending | [EMBED](#embed) |
| [I195-CST-DEFAULT-EVERY-LANGUAGE](issue-227-unmapped-requirements-2026-10-31.md#i195-cst-default-every-language) | Unmapped; code discovery/draft pending | [EMBED](#embed) |
| [I195-CST-delphi-object-pascal](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-delphi-object-pascal) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-diff](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-diff) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-docx](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-docx) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-dtd](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-dtd) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-elixir](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-elixir) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-elm](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-elm) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-EMBEDDED-GLOBAL-CONCEPT-SPACE](issue-227-unmapped-requirements-2026-10-31.md#i195-cst-embedded-global-concept-space) | Unmapped; code discovery/draft pending | [EMBED](#embed) |
| [I195-CST-english](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-english) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-erlang](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-erlang) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-EXPLICIT-AST-SELECTION](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-explicit-ast-selection) | Mapped; full review pending | [EMBED](#embed) |
| [I195-CST-french](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-french) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-go](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-go) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-graphql](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-graphql) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-groovy](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-groovy) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-haskell](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-haskell) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-hcl](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-hcl) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-hindi](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-hindi) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-html](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-html) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-ini](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-ini) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-java](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-java) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-javascript) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-json](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-json) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-json5](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-json5) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-kotlin](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-kotlin) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-lean) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-lino](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-lino) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-lua](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-lua) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-make](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-make) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-mandarin-chinese](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-mandarin-chinese) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-markdown](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-markdown) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-matlab](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-matlab) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-modern-standard-arabic](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-modern-standard-arabic) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-nix](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-nix) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-ocaml](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-ocaml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-ocaml-interface](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-ocaml-interface) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-odin](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-odin) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-pdf](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-pdf) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-perl](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-perl) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-php](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-php) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-portuguese](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-portuguese) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-powershell](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-powershell) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-protocol-buffers](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-protocol-buffers) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-python](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-python) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-r](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-r) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-racket](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-racket) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS](issue-227-unmapped-requirements-2026-10-31.md#i195-cst-recursive-boundaries-lossless) | Unmapped; code discovery/draft pending | [EMBED](#embed) |
| [I195-CST-regex](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-regex) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-rocq) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-ruby](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-ruby) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-russian](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-russian) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-rust) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-scala](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-scala) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-scheme](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-scheme) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-solidity](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-solidity) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-spanish](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-spanish) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-ansi](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-ansi) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-bigquery](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-bigquery) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-mysql](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-mysql) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-oracle](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-oracle) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-postgres](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-postgres) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-server](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-server) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-snowflake](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-snowflake) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-sql-sqlite](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-sql-sqlite) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-swift](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-swift) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-toml](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-toml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-tsx](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-tsx) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-txt](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-txt) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-typescript](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-typescript) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-urdu](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-urdu) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-visual-basic](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-visual-basic) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-xml](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-xml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-yaml](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-yaml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-CST-zig](issue-227-mapped-requirements-review-2026-10-31.md#i195-cst-zig) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-DECORATORS-EVERY-LEVEL](issue-227-mapped-requirements-review-2026-10-31.md#i195-decorators-every-level) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-DELIVERY-CRATE-CANDIDATE](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-crate-candidate) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DELIVERY-CRATE-PUBLISHED](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-crate-published) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DELIVERY-NPM-CANDIDATE](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-npm-candidate) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DELIVERY-NPM-PUBLISHED](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-npm-published) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DELIVERY-RML-CANDIDATE](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-rml-candidate) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DELIVERY-RML-PUBLISHED](issue-227-mapped-requirements-review-2026-10-31.md#i195-delivery-rml-published) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DEPENDENCY-CURRENT-STABLE-DELIVERY](issue-227-mapped-requirements-review-2026-10-31.md#i195-dependency-current-stable-delivery) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DEPENDENCY-INVENTORY](issue-227-mapped-requirements-review-2026-10-31.md#i195-dependency-inventory) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED](issue-227-unmapped-requirements-2026-10-31.md#i195-dependency-production-parsers-removed) | Unmapped; code discovery/draft pending | [DELIVERY](#delivery) |
| [I195-DEVELOPMENT-RULES-DOCUMENTED](issue-227-mapped-requirements-review-2026-10-31.md#i195-development-rules-documented) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-DOCUMENTATION-RECONCILED](issue-227-mapped-requirements-review-2026-10-31.md#i195-documentation-reconciled) | Mapped; full review pending | [TRACE](#trace) |
| [I195-DOWNSTREAM-CONSUMER-MATRIX](issue-227-mapped-requirements-review-2026-10-31.md#i195-downstream-consumer-matrix) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DOWNSTREAM-FORMAL-AI-PUBLISHED](issue-227-unmapped-requirements-2026-10-31.md#i195-downstream-formal-ai-published) | Unmapped; code discovery/draft pending | [DELIVERY](#delivery) |
| [I195-DOWNSTREAM-FORMAL-AI-WORKLOADS](issue-227-mapped-requirements-review-2026-10-31.md#i195-downstream-formal-ai-workloads) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DOWNSTREAM-RML-PR184-AUDIT](issue-227-mapped-requirements-review-2026-10-31.md#i195-downstream-rml-pr184-audit) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DOWNSTREAM-RML-WORKLOADS](issue-227-mapped-requirements-review-2026-10-31.md#i195-downstream-rml-workloads) | Mapped; full review pending | [DELIVERY](#delivery) |
| [I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS](issue-227-unmapped-requirements-2026-10-31.md#i195-downstream-typescript-translations) | Unmapped; code discovery/draft pending | [DELIVERY](#delivery) |
| [I195-DYNAMIC-BINARY-LINKS-RECONSTRUCTION](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-binary-links-reconstruction) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-DYNAMIC-LANGUAGE-EXTENSION](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-language-extension) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-DYNAMIC-LINK-CLI-RECONSTRUCTION](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-link-cli-reconstruction) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-DYNAMIC-RECONSTRUCTION-RESOURCE-MEASUREMENT](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-reconstruction-resource-measurement) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-DYNAMIC-SNAPSHOT-RECONSTRUCTION](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-snapshot-reconstruction) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-DYNAMIC-TEXT-LINKS-RECONSTRUCTION](issue-227-unmapped-requirements-2026-10-31.md#i195-dynamic-text-links-reconstruction) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-EMBED-css-style-attribute-css](issue-227-mapped-requirements-review-2026-10-31.md#i195-embed-css-style-attribute-css) | Mapped; full review pending | [EMBED](#embed) |
| [I195-EMBED-html-css](issue-227-mapped-requirements-review-2026-10-31.md#i195-embed-html-css) | Mapped; full review pending | [EMBED](#embed) |
| [I195-EMBED-html-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-embed-html-javascript) | Mapped; full review pending | [EMBED](#embed) |
| [I195-EMBED-markdown-html](issue-227-mapped-requirements-review-2026-10-31.md#i195-embed-markdown-html) | Mapped; full review pending | [EMBED](#embed) |
| [I195-EMBED-markdown-language-selected-fenced-code](issue-227-mapped-requirements-review-2026-10-31.md#i195-embed-markdown-language-selected-fenced-code) | Mapped; full review pending | [EMBED](#embed) |
| [I195-EXTERNAL-GRAMMAR-IGNORED-DIRECTORY](issue-227-mapped-requirements-review-2026-10-31.md#i195-external-grammar-ignored-directory) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-EXTERNAL-GRAMMAR-TEMPORARY-DOWNLOAD](issue-227-mapped-requirements-review-2026-10-31.md#i195-external-grammar-temporary-download) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-EXTERNAL-GRAMMAR-UNTRACKED](issue-227-mapped-requirements-review-2026-10-31.md#i195-external-grammar-untracked) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-FORMAL-CONCEPT-DEFINITIONS](issue-227-unmapped-requirements-2026-10-31.md#i195-formal-concept-definitions) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-FORMAL-CONCEPT-MEANING-COMPARISON](issue-227-unmapped-requirements-2026-10-31.md#i195-formal-concept-meaning-comparison) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-FORMAL-CONCEPT-REFERENCES](issue-227-unmapped-requirements-2026-10-31.md#i195-formal-concept-references) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-GATE-FAULT-dropped-fields-trivia-or-spans](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-dropped-fields-trivia-or-spans) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-falsified-capability-declaration](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-falsified-capability-declaration) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-lexical-parser-fallback](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-lexical-parser-fallback) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-plain-text-parser-fallback](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-plain-text-parser-fallback) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-relabeled-source-translation](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-relabeled-source-translation) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-removed-inventory-row-or-alias](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-removed-inventory-row-or-alias) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-rename-capture-bug](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-rename-capture-bug) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-skipped-test-or-job](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-skipped-test-or-job) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-stale-structure-after-edit](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-stale-structure-after-edit) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-stubbed-binding-resolution](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-stubbed-binding-resolution) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GATE-FAULT-unsupported-translation-descriptor](issue-227-mapped-requirements-review-2026-10-31.md#i195-gate-fault-unsupported-translation-descriptor) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GENERATIVE-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-generative-javascript) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GENERATIVE-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-generative-lean) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GENERATIVE-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-generative-rocq) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GENERATIVE-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-generative-rust) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-GLOBAL-CONCEPT-COPY](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-copy) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-DISTINCTIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-distinctions) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-EQUIVALENCE](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-equivalence) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-PROVENANCE](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-provenance) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-REUSE](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-reuse) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-RUNTIME-PARITY](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-runtime-parity) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-SERIALIZATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-serialization) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-SPACE](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-space) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GLOBAL-CONCEPT-UNPROVED](issue-227-mapped-requirements-review-2026-10-31.md#i195-global-concept-unproved) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-GRAMMAR-BULK-PIPELINE](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-bulk-pipeline) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-CHARACTER-CLASS-SETS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-character-class-sets) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-CONCEPT-DISTINCTIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-concept-distinctions) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-CONCEPT-REUSE-REPORT](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-concept-reuse-report) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-CONCEPT-TRANSLATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-concept-translation) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-DECLARED-SETTLING](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-declared-settling) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-DEPENDENCY-BOUNDARY](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-dependency-boundary) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-FEATURE-UNION](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-feature-union) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-HEXADECIMAL-SCALARS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-hexadecimal-scalars) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-INFERENCE-AUTOMATIC-DISCOVERY](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-automatic-discovery) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-CI-RESOURCE-POLICY](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-ci-resource-policy) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-COMPARATIVE-BENCHMARKS](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-comparative-benchmarks) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-DETERMINISTIC-REFINEMENT](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-deterministic-refinement) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-DYNAMIC-TRANSLATION](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-dynamic-translation) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-EXISTING-WORK-AUDIT](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-existing-work-audit) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-FORMAL-CONCEPT-INTEGRATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-inference-formal-concept-integration) | Mapped; full review pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-GENERIC-UNION](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-generic-union) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-INDEPENDENT-EVALUATION](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-independent-evaluation) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-NATIVE-OUTPUT](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-native-output) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-PUBLIC-API-CLI](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-public-api-cli) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-RESEARCH-SELECTION](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-research-selection) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-SEPARATE-ISSUE](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-inference-separate-issue) | Mapped; full review pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-INFERENCE-SOURCE-FREE-SNAPSHOTS](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-inference-source-free-snapshots) | Unmapped; code discovery/draft pending | [INFERENCE](#inference) |
| [I195-GRAMMAR-LANGUAGE-CATALOG](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-language-catalog) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-LOSSLESS-TREES](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-lossless-trees) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-AGDA](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-agda) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-C](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-c) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-CMAKE](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-cmake) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-CPP](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-cpp) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-CSS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-css) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-CSV](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-csv) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-DART](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-dart) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-DIFF](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-diff) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-DTD](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-dtd) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-ERLANG](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-erlang) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-GO](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-go) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-GRAPHQL](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-graphql) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-GROOVY](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-groovy) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-HCL](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-hcl) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-HTML](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-html) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-INI](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-ini) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-JAVA](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-java) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-JAVASCRIPT](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-javascript) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-JSON](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-json) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-JSON5](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-json5) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-LEAN](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-lean) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-LUA](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-lua) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-MAKE](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-make) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-MERGED](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-native-merged) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-NIX](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-nix) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-ODIN](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-odin) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-PASCAL](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-pascal) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-POWERSHELL](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-powershell) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-PROTO](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-proto) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-PYTHON](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-python) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-R](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-r) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-RACKET](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-racket) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-RECOVERY](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-recovery) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-REGEX](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-regex) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-ROCQ](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-rocq) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-RUST](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-rust) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-SCHEME](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-scheme) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-SOLIDITY](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-solidity) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-SQL](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-sql) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-TOML](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-toml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-TSX](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-tsx) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-TYPESCRIPT](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-typescript) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-VB](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-vb) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-XML](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-xml) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATIVE-ZIG](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-native-zig) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-NATURAL-LANGUAGE-REPRESENTATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-natural-language-representation) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-PRIORITY-JAVASCRIPT-COMPLETE](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-priority-javascript-complete) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-PRIORITY-MARKDOWN-COMPLETE](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-priority-markdown-complete) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-PRIORITY-OWN-SOURCE-TRANSLATION](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-priority-own-source-translation) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-PRIORITY-RUST-COMPLETE](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-priority-rust-complete) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-PRIORITY-TYPESCRIPT-COMPLETE](issue-227-unmapped-requirements-2026-10-31.md#i195-grammar-priority-typescript-complete) | Unmapped; code discovery/draft pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-CONTEXT-TOKENS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-context-tokens) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-COUNTED-DELIMITERS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-counted-delimiters) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-DELIMITER-FAMILIES](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-delimiter-families) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-DELIMITER-RUNS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-delimiter-runs) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-FENCED-BLOCKS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-fenced-blocks) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-FRAGMENTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-fragments) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-HERE-DOCUMENTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-here-documents) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-INTERPOLATED-QUOTES](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-interpolated-quotes) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-OFFSIDE-LAYOUT](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-offside-layout) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-REMEMBERED-CONTENT](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-remembered-content) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-REMEMBERED-DELIMITERS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-remembered-delimiters) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SCANNER-STATE-SEMANTICS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-scanner-state-semantics) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-GRAMMAR-SHARED-CONCEPTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-grammar-shared-concepts) | Mapped; full review pending | [GRAMMAR](#grammar) |
| [I195-IMPORT-abnf](issue-227-mapped-requirements-review-2026-10-31.md#i195-import-abnf) | Mapped; full review pending | [MERGE](#merge) |
| [I195-IMPORT-bnf](issue-227-mapped-requirements-review-2026-10-31.md#i195-import-bnf) | Mapped; full review pending | [MERGE](#merge) |
| [I195-IMPORT-ebnf](issue-227-mapped-requirements-review-2026-10-31.md#i195-import-ebnf) | Mapped; full review pending | [MERGE](#merge) |
| [I195-IMPORT-pest](issue-227-mapped-requirements-review-2026-10-31.md#i195-import-pest) | Mapped; full review pending | [MERGE](#merge) |
| [I195-IMPORT-tree-sitter-grammar-json](issue-227-mapped-requirements-review-2026-10-31.md#i195-import-tree-sitter-grammar-json) | Mapped; full review pending | [MERGE](#merge) |
| [I195-INTERCHANGE-API-CLI](issue-227-mapped-requirements-review-2026-10-31.md#i195-interchange-api-cli) | Mapped; full review pending | [MERGE](#merge) |
| [I195-INTERCHANGE-CROSS-FORMAT-TOOLS](issue-227-unmapped-requirements-2026-10-31.md#i195-interchange-cross-format-tools) | Unmapped; code discovery/draft pending | [MERGE](#merge) |
| [I195-INTERCHANGE-FAITHFUL-LOWERING](issue-227-mapped-requirements-review-2026-10-31.md#i195-interchange-faithful-lowering) | Mapped; full review pending | [MERGE](#merge) |
| [I195-INTERCHANGE-FORMAT-FEATURES](issue-227-unmapped-requirements-2026-10-31.md#i195-interchange-format-features) | Unmapped; code discovery/draft pending | [MERGE](#merge) |
| [I195-INTERCHANGE-MUTATION-GUARDS](issue-227-mapped-requirements-review-2026-10-31.md#i195-interchange-mutation-guards) | Mapped; full review pending | [MERGE](#merge) |
| [I195-INTERCHANGE-REVERSE-CONVERSION](issue-227-mapped-requirements-review-2026-10-31.md#i195-interchange-reverse-conversion) | Mapped; full review pending | [MERGE](#merge) |
| [I195-KNOWLEDGE-OWL-SUPPORT](issue-227-unmapped-requirements-2026-10-31.md#i195-knowledge-owl-support) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-KNOWLEDGE-RDF-OWL-CONVERSION](issue-227-unmapped-requirements-2026-10-31.md#i195-knowledge-rdf-owl-conversion) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-KNOWLEDGE-RDF-OWL-IMPORT](issue-227-unmapped-requirements-2026-10-31.md#i195-knowledge-rdf-owl-import) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-KNOWLEDGE-RDF-OWL-MERGE](issue-227-unmapped-requirements-2026-10-31.md#i195-knowledge-rdf-owl-merge) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-KNOWLEDGE-RDF-SUPPORT](issue-227-unmapped-requirements-2026-10-31.md#i195-knowledge-rdf-support) | Unmapped; code discovery/draft pending | [DYNAMIC](#dynamic) |
| [I195-LINK-FOUNDATION-MULTIPLE-ROLES](issue-227-mapped-requirements-review-2026-10-31.md#i195-link-foundation-multiple-roles) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-LINK-FOUNDATION-OPTIONAL-TYPE-ROOT](issue-227-mapped-requirements-review-2026-10-31.md#i195-link-foundation-optional-type-root) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-LINK-FOUNDATION-PHYSICAL-ANALOGY](issue-227-unmapped-requirements-2026-10-31.md#i195-link-foundation-physical-analogy) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-LINK-FOUNDATION-RECURSIVE-COMPOSITION](issue-227-mapped-requirements-review-2026-10-31.md#i195-link-foundation-recursive-composition) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-LINK-FOUNDATION-SELF-LOOP-VERTEX](issue-227-mapped-requirements-review-2026-10-31.md#i195-link-foundation-self-loop-vertex) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-LINK-FOUNDATION-UNIVERSAL-REPRESENTATION](issue-227-unmapped-requirements-2026-10-31.md#i195-link-foundation-universal-representation) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-LINO-COMPATIBILITY-MATRIX](issue-227-mapped-requirements-review-2026-10-31.md#i195-lino-compatibility-matrix) | Mapped; full review pending | [DYNAMIC](#dynamic) |
| [I195-LINO-UPGRADE](issue-227-mapped-requirements-review-2026-10-31.md#i195-lino-upgrade) | Mapped; full review pending | [DYNAMIC](#dynamic) |
| [I195-LINO-UPSTREAM-REGRESSIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-lino-upstream-regressions) | Mapped; full review pending | [DYNAMIC](#dynamic) |
| [I195-LOCAL-RESOURCE-CAREFUL-BOUNDED-USE](issue-227-unmapped-requirements-2026-10-31.md#i195-local-resource-careful-bounded-use) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-LOCAL-RESOURCE-PROJECT-OWNED-CLEANUP](issue-227-unmapped-requirements-2026-10-31.md#i195-local-resource-project-owned-cleanup) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-LOCAL-RESOURCE-RUST-COMPILATION-CI-ONLY](issue-227-unmapped-requirements-2026-10-31.md#i195-local-resource-rust-compilation-ci-only) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-MERGE-AUTOMATIC-PIPELINE](issue-227-unmapped-requirements-2026-10-31.md#i195-merge-automatic-pipeline) | Unmapped; code discovery/draft pending | [MERGE](#merge) |
| [I195-MERGE-BINDING-AWARE-RENAME](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-binding-aware-rename) | Mapped; full review pending | [MERGE](#merge) |
| [I195-MERGE-DETERMINISM](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-determinism) | Mapped; full review pending | [MERGE](#merge) |
| [I195-MERGE-MEANING-AWARE-DEDUPLICATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-meaning-aware-deduplication) | Mapped; full review pending | [MERGE](#merge) |
| [I195-MERGE-QUALITY-EVIDENCE](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-quality-evidence) | Mapped; full review pending | [MERGE](#merge) |
| [I195-MERGE-REAL-RECONCILIATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-real-reconciliation) | Mapped; full review pending | [MERGE](#merge) |
| [I195-MERGE-SHIPPED-GRAMMARS-ARE-MERGED](issue-227-unmapped-requirements-2026-10-31.md#i195-merge-shipped-grammars-are-merged) | Unmapped; code discovery/draft pending | [MERGE](#merge) |
| [I195-MERGE-UNCERTAINTY-PRESERVED](issue-227-mapped-requirements-review-2026-10-31.md#i195-merge-uncertainty-preserved) | Mapped; full review pending | [MERGE](#merge) |
| [I195-NAMING-CI-ENFORCEMENT](issue-227-mapped-requirements-review-2026-10-31.md#i195-naming-ci-enforcement) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-NAMING-CONCEPT-RECORDS](issue-227-mapped-requirements-review-2026-10-31.md#i195-naming-concept-records) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-NAMING-CONVENTION](issue-227-mapped-requirements-review-2026-10-31.md#i195-naming-convention) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-NAMING-NATIVE-GRAMMARS](issue-227-mapped-requirements-review-2026-10-31.md#i195-naming-native-grammars) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-NATIVE-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-native-javascript) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-NATIVE-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-native-lean) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-NATIVE-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-native-rocq) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-NATIVE-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-native-rust) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-ONTOLOGY-LIBRARY-ENTRYPOINT](issue-227-mapped-requirements-review-2026-10-31.md#i195-ontology-library-entrypoint) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-ONTOLOGY-MULTIPLE-LINK-ROOTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-ontology-multiple-link-roots) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-ONTOLOGY-TAXONOMY-MEMBERSHIP](issue-227-mapped-requirements-review-2026-10-31.md#i195-ontology-taxonomy-membership) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-ONTOLOGY-TAXONOMY-RELATION-DISTINCTION](issue-227-mapped-requirements-review-2026-10-31.md#i195-ontology-taxonomy-relation-distinction) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-PARITY-CST](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-cst) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-PARITY-DIAGNOSTICS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-diagnostics) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-PARITY-FEATURE-COMPLETENESS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-feature-completeness) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-modern-javascript-array-behavior) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-obsolete-native-expectations) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-PARITY-SEMANTICS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-semantics) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-PARITY-TRANSFORMS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-transforms) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-PARITY-TRANSLATIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-parity-translations) | Mapped; full review pending | [EVIDENCE](#evidence) |
| [I195-PRACTICE-ROUTER-761-REUSE](issue-227-unmapped-requirements-2026-10-31.md#i195-practice-router-761-reuse) | Unmapped; code discovery/draft pending | [TRACE](#trace) |
| [I195-RENAME-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-rename-javascript) | Mapped; full review pending | [EDIT](#edit) |
| [I195-RENAME-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-rename-lean) | Mapped; full review pending | [EDIT](#edit) |
| [I195-RENAME-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-rename-rocq) | Mapped; full review pending | [EDIT](#edit) |
| [I195-RENAME-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-rename-rust) | Mapped; full review pending | [EDIT](#edit) |
| [I195-RESOURCE-AGENT-RULES](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-agent-rules) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-BOUNDED-MEMORY-REGRESSIONS](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-bounded-memory-regressions) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-BOUNDED-SEQUENTIAL-EVIDENCE](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-bounded-sequential-evidence) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-CACHE-CLEANUP-WRAPPER](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-cache-cleanup-wrapper) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-CI-COMPILE-GATE](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-ci-compile-gate) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-CI-CONCURRENCY](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-ci-concurrency) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-CI-EVIDENCE-STAGES](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-ci-evidence-stages) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-CI-TEST-MATRIX](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-ci-test-matrix) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-GRAMMAR-TIERING-BUDGET](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-grammar-tiering-budget) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-INLINE-PARSER-REUSE](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-inline-parser-reuse) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-LAZY-GRAMMARS](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-lazy-grammars) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-NATIVE-OUTPUT-CLEANUP](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-native-output-cleanup) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-PARITY-DIGESTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-parity-digests) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-PARSE-MEMORY-BUDGET](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-parse-memory-budget) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-RESOURCE-SOURCE-BOUNDARIES](issue-227-mapped-requirements-review-2026-10-31.md#i195-resource-source-boundaries) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-SELF-TRANSLATION-CARRIED-ZERO](issue-227-mapped-requirements-review-2026-10-31.md#i195-self-translation-carried-zero) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SELF-TRANSLATION-ROUND-TRIP](issue-227-mapped-requirements-review-2026-10-31.md#i195-self-translation-round-trip) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SELF-TRANSLATION-SHARED-CORPUS](issue-227-mapped-requirements-review-2026-10-31.md#i195-self-translation-shared-corpus) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SELF-TRANSLATION-TOOL](issue-227-mapped-requirements-review-2026-10-31.md#i195-self-translation-tool) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SEM-javascript-attributes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-attributes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-effects](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-effects) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-macros-and-notation](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-macros-and-notation) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-modules-and-imports](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-modules-and-imports) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-project-context-and-dependencies](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-project-context-and-dependencies) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-proof-terms-and-tactics](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-proof-terms-and-tactics) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-recursive-definitions](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-recursive-definitions) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-scopes-and-bindings](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-scopes-and-bindings) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-surface-expansion-elaboration-traces](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-surface-expansion-elaboration-traces) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-javascript-types-and-universes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-javascript-types-and-universes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-attributes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-attributes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-effects](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-effects) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-macros-and-notation](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-macros-and-notation) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-modules-and-imports](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-modules-and-imports) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-project-context-and-dependencies](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-project-context-and-dependencies) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-proof-terms-and-tactics](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-proof-terms-and-tactics) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-recursive-definitions](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-recursive-definitions) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-scopes-and-bindings](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-scopes-and-bindings) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-surface-expansion-elaboration-traces](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-surface-expansion-elaboration-traces) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-lean-types-and-universes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-lean-types-and-universes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-attributes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-attributes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-effects](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-effects) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-macros-and-notation](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-macros-and-notation) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-modules-and-imports](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-modules-and-imports) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-project-context-and-dependencies](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-project-context-and-dependencies) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-proof-terms-and-tactics](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-proof-terms-and-tactics) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-recursive-definitions](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-recursive-definitions) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-scopes-and-bindings](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-scopes-and-bindings) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-surface-expansion-elaboration-traces](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-surface-expansion-elaboration-traces) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rocq-types-and-universes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rocq-types-and-universes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-attributes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-attributes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-effects](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-effects) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-macros-and-notation](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-macros-and-notation) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-modules-and-imports](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-modules-and-imports) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-project-context-and-dependencies](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-project-context-and-dependencies) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-proof-terms-and-tactics](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-proof-terms-and-tactics) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-recursive-definitions](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-recursive-definitions) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-scopes-and-bindings](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-scopes-and-bindings) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-surface-expansion-elaboration-traces](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-surface-expansion-elaboration-traces) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEM-rust-types-and-universes](issue-227-mapped-requirements-review-2026-10-31.md#i195-sem-rust-types-and-universes) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-SEMANTICS-CONSTRUCT-INVENTORY](issue-227-unmapped-requirements-2026-10-31.md#i195-semantics-construct-inventory) | Unmapped; code discovery/draft pending | [BASELINE](#baseline) |
| [I195-SEMANTICS-FAITHFUL-BEHAVIOR](issue-227-mapped-requirements-review-2026-10-31.md#i195-semantics-faithful-behavior) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SEMANTICS-PROOF-PRESERVATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-semantics-proof-preservation) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-SHARED-CONCEPTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-shared-concepts) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-SHARED-PROVENANCE](issue-227-mapped-requirements-review-2026-10-31.md#i195-shared-provenance) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-SHARED-SCHEMA](issue-227-mapped-requirements-review-2026-10-31.md#i195-shared-schema) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-STRING-TRIM-SEMANTICS](issue-227-mapped-requirements-review-2026-10-31.md#i195-string-trim-semantics) | Mapped; full review pending | [BASELINE](#baseline) |
| [I195-TAXONOMY-LIBRARY-ENTRYPOINT](issue-227-mapped-requirements-review-2026-10-31.md#i195-taxonomy-library-entrypoint) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-TAXONOMY-MULTIPLE-LINK-ROOTS](issue-227-mapped-requirements-review-2026-10-31.md#i195-taxonomy-multiple-link-roots) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-THEORY-FOUNDATION-CONSUMER-BOUNDARIES](issue-227-unmapped-requirements-2026-10-31.md#i195-theory-foundation-consumer-boundaries) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-THEORY-FOUNDATION-PINNED-SOURCE-AUDIT](issue-227-mapped-requirements-review-2026-10-31.md#i195-theory-foundation-pinned-source-audit) | Mapped; full review pending | [MEANING](#meaning) |
| [I195-THEORY-FOUNDATION-PROJECT-WIDE-APPLICATION](issue-227-unmapped-requirements-2026-10-31.md#i195-theory-foundation-project-wide-application) | Unmapped; code discovery/draft pending | [MEANING](#meaning) |
| [I195-TRANSLATE-javascript-to-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-javascript-to-lean) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-javascript-to-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-javascript-to-rocq) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-javascript-to-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-javascript-to-rust) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-lean-to-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-lean-to-javascript) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-lean-to-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-lean-to-rocq) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-lean-to-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-lean-to-rust) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rocq-to-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rocq-to-javascript) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rocq-to-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rocq-to-lean) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rocq-to-rust](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rocq-to-rust) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rust-to-javascript](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rust-to-javascript) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rust-to-lean](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rust-to-lean) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATE-rust-to-rocq](issue-227-mapped-requirements-review-2026-10-31.md#i195-translate-rust-to-rocq) | Mapped; full review pending | [FORMAL](#formal) |
| [I195-TRANSLATION-OWN-CODEBASE-BASELINE](issue-227-unmapped-requirements-2026-10-31.md#i195-translation-own-codebase-baseline) | Unmapped; code discovery/draft pending | [BASELINE](#baseline) |
| [I195-TRANSLATION-PRIORITY-JAVASCRIPT](issue-227-unmapped-requirements-2026-10-31.md#i195-translation-priority-javascript) | Unmapped; code discovery/draft pending | [BASELINE](#baseline) |
| [I195-TRANSLATION-PRIORITY-MULTI-PROJECT](issue-227-unmapped-requirements-2026-10-31.md#i195-translation-priority-multi-project) | Unmapped; code discovery/draft pending | [BASELINE](#baseline) |
| [I195-TRANSLATION-PRIORITY-RUST](issue-227-unmapped-requirements-2026-10-31.md#i195-translation-priority-rust) | Unmapped; code discovery/draft pending | [BASELINE](#baseline) |
| [I195-VISION-CONTRADICTION-AUDIT](issue-227-mapped-requirements-review-2026-10-31.md#i195-vision-contradiction-audit) | Mapped; full review pending | [TRACE](#trace) |
| [I195-VISION-FOUNDATION-NEUTRAL](issue-227-mapped-requirements-review-2026-10-31.md#i195-vision-foundation-neutral) | Mapped; full review pending | [TRACE](#trace) |
| [I195-VISION-REQUIREMENT-TRACEABILITY](issue-227-mapped-requirements-review-2026-10-31.md#i195-vision-requirement-traceability) | Mapped; full review pending | [TRACE](#trace) |
| [I195-VISION-SOURCE-REGISTER](issue-227-mapped-requirements-review-2026-10-31.md#i195-vision-source-register) | Mapped; full review pending | [TRACE](#trace) |
| [I195-VISION-SPECIFICATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-vision-specification) | Mapped; full review pending | [TRACE](#trace) |
| [I195-WORKFLOW-AUTOMATED-TRANSLATION](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-automated-translation) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-AUTOMATION-FIRST-COVERAGE](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-automation-first-coverage) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-BULK-EXACT-SCOPE](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-bulk-exact-scope) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-CANCELLED-CI-INVESTIGATION](issue-227-mapped-requirements-review-2026-10-31.md#i195-workflow-cancelled-ci-investigation) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-COMPLETE-DRAFT-BEFORE-CI-FEEDBACK](issue-227-mapped-requirements-review-2026-10-31.md#i195-workflow-complete-draft-before-ci-feedback) | Mapped; full review pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-COMPLETE-DRAFT-BEFORE-TESTING](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-complete-draft-before-testing) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-CONTRIBUTOR-COMMITS](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-contributor-commits) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-DRAFT-WITHOUT-COMPILATION](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-draft-without-compilation) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-FAST-CONTINUOUS-PROGRESS](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-fast-continuous-progress) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-GRADUAL-AGENT-REDUCTION](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-gradual-agent-reduction) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-LATEST-REPOSITORY](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-latest-repository) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-MINIMAL-LOCAL-RESOURCES](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-minimal-local-resources) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-NO-CODE-TESTS-BEFORE-FULL-DRAFT](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-no-code-tests-before-full-draft) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-NO-LOCAL-RUST-COMPILATION](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-no-local-rust-compilation) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-PARALLEL-DELEGATION](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-parallel-delegation) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-PREVIOUS-CI-ERRORS-ADDRESSED](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-previous-ci-errors-addressed) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-PREVIOUS-CI-TERMINAL](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-previous-ci-terminal) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-SINGLE-PUSH-COORDINATOR](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-single-push-coordinator) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-WORKFLOW-THREE-SUBAGENT-CURRENT-PHASE](issue-227-unmapped-requirements-2026-10-31.md#i195-workflow-three-subagent-current-phase) | Unmapped; code discovery/draft pending | [OPERATIONS](#operations) |
| [I195-XFORM-javascript-clone](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-clone) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-construct](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-construct) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-delete](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-delete) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-emit](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-emit) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-insert](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-insert) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-move](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-move) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-query](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-query) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-javascript-replace](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-javascript-replace) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-clone](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-clone) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-construct](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-construct) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-delete](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-delete) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-emit](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-emit) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-insert](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-insert) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-move](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-move) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-query](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-query) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-lean-replace](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-lean-replace) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-clone](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-clone) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-construct](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-construct) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-delete](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-delete) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-emit](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-emit) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-insert](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-insert) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-move](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-move) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-query](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-query) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rocq-replace](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rocq-replace) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-clone](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-clone) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-construct](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-construct) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-delete](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-delete) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-emit](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-emit) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-insert](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-insert) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-move](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-move) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-query](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-query) | Mapped; full review pending | [EDIT](#edit) |
| [I195-XFORM-rust-replace](issue-227-mapped-requirements-review-2026-10-31.md#i195-xform-rust-replace) | Mapped; full review pending | [EDIT](#edit) |

## Next session handoff

Remain paused until instructed. On resumption, update source snapshots first, then perform TRACE’s static full-scope review and identify a complete own-code batch. The contemplated collection Default/trait-scope extension is only a candidate: select it only if the construct inventory shows it unblocks baseline modules. Do not restart a stream of small extensions without updating requirement-level gaps.

Keep this plan linked from the stop checkpoint. Reconcile new/edited atomic rows into both appendices and the index; preserve protected source, behavior, scope, aliases, fixtures and runtimes. When a mapping is added, move its planning entry between appendices without calling it fully drafted until the static audit supports that classification. When verification starts, replace future artifact references with actual recorded observations only through the repository evidence workflow.
