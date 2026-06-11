# Requirements Register - Issue #47

This register extracts every distinct requirement from
[issue #47](https://github.com/link-foundation/meta-language/issues/47)
("We need to carefully compare all competitors in all scopes with out project,
and make sure we have the richest features set") and maps each requirement to
the implementation state of this repository on branch
`issue-47-76af108c0f24` as of 2026-06-10.

The issue body snapshot is stored at
[`raw-data/issue-47.json`](./raw-data/issue-47.json). The register style follows
the prior [issue #3 requirements register](../issue-3/requirements.md).

## Status Vocabulary

| Status | Meaning |
|---|---|
| **Implemented** | Runtime behavior plus executable tests exist for the requirement as stated. |
| **Partial** | A real baseline exists, but a named part of the requirement is absent or demo-grade. |
| **Missing** | No runtime behavior, configuration surface, or artifact exists for the requirement. |

## Summary Table

| ID | Requirement | Status | Evidence | Gap |
|---|---|---|---|---|
| R-1 | Engine readonly or mutable per user configuration | Partial | `src/snapshots.rs` `NetworkSnapshot` / `MutableNetworkSnapshot` | No engine-level readonly mode; `LinkNetwork` is always mutable; `ParseConfiguration` has no mutability switch |
| R-2 | CST/AST for all popular programming languages | Partial | `src/tree_sitter_adapter.rs:grammar_for_language` (17 grammars), `NetworkProjection` | "All popular" not met: no PHP, Swift, Kotlin, Scala, Perl, etc.; no dynamic grammar loading |
| R-3 | Data-exchange formats | Missing | Only LiNo (`src/lino_parser.rs`); `Cargo.toml` has no JSON/XML/YAML/TOML/CSV grammar | Mainstream interchange formats fall back to structureless plain-text tokens |
| R-4 | Other popular formal languages | Partial | `sql-ansi` via `tree-sitter-sequel`, CSS, LiNo; RML parity fixtures | RML and any non-wired formal language parse via plain-text fallback; no regex/BNF/math grammars |
| R-5 | Natural languages with actual grammatical-correctness parsing | Partial | `src/natural_language.rs:annotate_natural_language` (identification, segmentation, NFC, bidi) | No grammar parsing: no POS/morphology/syntax links, so grammatical correctness cannot be checked |
| R-6 | Shared concept space reused only on exact match | Partial | `src/concept_ontology.rs:seed_common_concept_ontology` (351-concept lexicon); `src/lino_parser.rs:reference_for_atom` exact `find_term` reuse | Translation through concepts is demo-grade: only the hard-coded statehood proposition |
| R-7 | Storage as links-notation text | Partial | `src/lino_parser.rs` parses LiNo; `LinkNetwork::self_description_text` emits LiNo-style lines | No general network-to-LiNo serializer; arbitrary networks cannot round-trip to LiNo text |
| R-8 | Binary doublets storage (doublets-rs / doublets-web) | Missing | No `doublets` dependency in `Cargo.toml`; no binary persistence in `src/` | Entire binary-links storage substrate absent |
| R-9 | Rust traits/types representation | Partial | `LinkType::Object`, `LinkNetwork::insert_object`; lino-objects-codec fixtures in `src/parity_fixtures.rs` | No derive/trait codec mapping arbitrary user Rust structs/traits to links and back |
| R-10 | formal-ai: any language treated easily as data | Partial | formal-ai target in `src/parity.rs:PARITY_TARGETS`; seed/benchmark fixtures in `src/parity_fixtures.rs` | Language-as-data holds only for wired grammars; formalization beyond the statehood demo absent |
| R-11 | link-cli-style transformations and substitutions | Implemented | `src/substitution.rs` (`SubstitutionRule`, `VariableSubstitutionRule`); `LinkNetwork::apply_substitution` / `apply_variable_substitution`; ported create/update/delete/swap fixtures | None for the scoped baseline |
| R-12 | Chaining API adapter | Partial | Builder chaining: `LinkQuery::by_type().with_term()...`, `LinkMetadata::with_*`, `ParseConfiguration::with_*` | No fluent pipeline (jscodeshift-style) chaining parse → query → transform → reconstruct |
| R-13 | Direct OOP API | Implemented | `LinkNetwork` methods: `parse`, `query_matches`, `apply_substitution`, `snapshot`, `reconstruct_text`, `verify_full_match` | None for the operations that exist |
| R-14 | All operations available through all API styles equally | Missing | Queries have builder + S-expression forms (`src/query.rs:from_sexpression`); substitutions only direct methods | No parity contract or test that each operation is reachable in every style |
| R-15 | Translate any language to and from meta language | Partial | `LinkNetwork::parse` (to-meta); `reconstruct_text` (byte-exact); `reconstruct_text_as` (`src/reconstruction.rs`) | Cross-language output is gated on the hard-coded `has_statehood_proposition` demo |
| R-16 | Single-language restriction profiles (e.g. JS-to-JS) | Missing | No symbol in `src/` models per-language feature capability profiles | Cannot restrict meta-language operations to "only what JavaScript supports exactly" |
| R-17 | User-expandable/configurable translation rules | Partial | `LinkNetwork::insert_concept_mapping` (public); `LanguageParser` trait; `QueryPredicateHost` trait | `LinkNetwork::parse` hardwires `BuiltInLanguageParser`; no rule-based translation registry |
| R-18 | Everything replaceable and configurable | Partial | `src/configuration.rs:ParseConfiguration` (trivia, region detection, language ID backend, formalization, direction) | Grammar set, parser dispatch, reconstruction, and storage backend are not swappable |
| R-19 | Nothing deferred or unimplemented in vision/roadmap | Partial | Issues #37/#39 closed; registries gated by tests (`tests/unit/parity_corpora.rs`); roadmap deferrals are tied to open issues #47, #50, and #70 | Full SQL dialect keys, Delphi-specific compiler coverage, and complete upstream corpus copying remain future work under tracked issues |
| R-20 | 100% test coverage copying competitor test cases | Partial | `cargo llvm-cov --fail-under-lines 84.30` job in `.github/workflows/release.yml`; 68 provenance-bearing `PARITY_FIXTURES` | The current wave ports representative assertion shapes; complete upstream test-suite copying remains under issue #47 |
| R-21 | Create sub-issues with blocked-by markings | Missing | `raw-data/issue-47.json`: `sub_issues_summary.total = 0`, `blocked_by = 0` | No sub-issues or dependency ordering exist for issue #47 |
| R-22 | Case-study compilation in `docs/case-studies/issue-47` | Partial | [`raw-data/`](./raw-data/) holds issue, comments, all-issues, PR #48 snapshots | No analysis, online research, solution plans, or proposed-issues documents (this register is the first analysis artifact) |

## Detailed Findings

### R-1 - Readonly or mutable engine per user configuration

> "We need to make sure our engine works as readonly or mutable as per user configuration"

**Status: Partial.**
`src/snapshots.rs` implements an immutable/mutable split:
`NetworkSnapshot` holds an `Arc<LinkNetwork>` (shared, read-only view with
`version`, `parent_version`, `provenance`), `NetworkSnapshot::to_mutable`
forks an editable `MutableNetworkSnapshot`, and `commit` / `commit_as`
produce forward-versioned immutable snapshots. Tests cover snapshot
versioning (`tests/unit/link_network.rs`).

**Gap.** This is an opt-in versioning pattern, not a *configuration*. The
core `LinkNetwork` exposes unconditional mutators (`insert_link`,
`set_references`, `set_span`, `set_flags`, `apply_substitution`), and
`ParseConfiguration` (`src/configuration.rs`) has no readonly/mutable
setting. There is no way for a user to configure the engine so that a
network is enforced read-only.

### R-2 - CST/AST for all popular programming languages

> "CST/AST for all popular programming languages"

**Status: Partial.**
`src/tree_sitter_adapter.rs:grammar_for_language` wires 17 grammar keys:
Python, C, Java, C++, C#, JavaScript, TypeScript, TSX, Visual Basic,
Delphi/Object Pascal, Rust, Go, R, Ruby, `sql-ansi`, HTML, and CSS
(dependencies pinned in `Cargo.toml`). The TIOBE top-ten registry
(`src/parity.rs:PROGRAMMING_LANGUAGE_TARGETS`) is fully grammar-backed, and
CST vs AST are projections over one lossless network
(`NetworkProjection::ConcreteSyntax` / `AbstractSyntax` in
`src/link_network.rs`), gated by `tests/unit/grammar_parsing.rs`.

**Gap.** "All popular programming languages" is wider than the top ten
plus four extras: PHP, Swift, Kotlin, Scala, Perl, Lua, Dart, Haskell,
shell, etc. have no grammar; any unknown label silently degrades to the
structureless `parse_lossless_text` fallback
(`src/language_parser.rs:BuiltInLanguageParser::parse_source`). There is
no mechanism to register additional tree-sitter grammars at runtime.

### R-3 - Data-exchange formats

> "data exchange formats"

**Status: Missing.**
The only structured data-notation parser is LiNo
(`src/lino_parser.rs`, routed by the `"lino"` label in
`src/language_parser.rs`). `Cargo.toml` contains no `tree-sitter-json`,
`tree-sitter-yaml`, `tree-sitter-xml`, `tree-sitter-toml`, or CSV grammar,
and neither `src/parity.rs` registries nor `docs/parity-roadmap.md` list a
data-exchange-format family at all. Parsing `"JSON"` today produces only
lossless plain-text token links with no structure.

**Gap.** The whole format family (JSON, XML, YAML, TOML, CSV, protobuf
text, etc.) needs grammar wiring, registry entries, and fixtures.

### R-4 - Other popular formal languages

> "and other popular formal languages"

**Status: Partial.**
Formal languages beyond mainstream programming languages that are wired:
SQL (`sql-ansi` via `tree-sitter-sequel`, documented in
`docs/parity-roadmap.md` "SQL Dialect Coverage"), CSS, and LiNo.
`relative-meta-logic` fixtures exercise an `"RML"` language label
(`src/parity_fixtures.rs`).

**Gap.** `"RML"` has no grammar in
`src/tree_sitter_adapter.rs:grammar_for_language`, so those fixtures rely
on the plain-text fallback. No grammars exist for regular expressions,
BNF/EBNF, lambda calculus, mathematical notation, or logic languages, and
the roadmap explicitly defers SQL dialect keys (BigQuery, SQLite,
PostgreSQL, T-SQL).

### R-5 - Natural languages with actual grammatical-correctness parsing

> "natural languages (including actual parsing of their grammar with no semantic checks ... but grammatical correctness, syntax correctness and so on - should be fully supported)"

**Status: Partial.**
`src/natural_language.rs:annotate_natural_language` annotates the ten
`NATURAL_LANGUAGE_TARGETS` (`src/parity.rs`) with: language identification
(lingua or whatlang, selectable via
`ParseConfiguration::with_language_identification_detector`), word
segmentation (`unicode-segmentation`, `lindera`-jieba for Mandarin),
Unicode normalization, and bidi metadata. All ten language fixtures
round-trip byte-exactly.

**Gap.** This is tokenization plus identification, not grammar parsing.
There are no part-of-speech, morphology, phrase-structure, or dependency
links, and `verify_full_match` (`src/link_network.rs`) emits no
grammatical-correctness diagnostics for natural text - an ungrammatical
English sentence parses "clean". The issue explicitly requires
grammatical/syntax correctness to be "fully supported"; no engine for
that exists in the crate.

### R-6 - Shared concept space with exact-match-only reuse

> "advanced shared concepts space between languages, which should be reused when only matched exactly"

**Status: Partial.**
`src/concept_ontology.rs:seed_common_concept_ontology` imports the
verified 351-concept meta-expression semantic lexicon
(`src/data/semantic-lexicon.json`), seeds structural concepts, and creates
per-language syntax mapping links; `tests/unit/concept_ontology.rs`
asserts shared concept identity. Exact-match reuse is real:
`src/lino_parser.rs:reference_for_atom` reuses an existing link only on an
exact `find_term` hit, and `insert_concept_syntax_mapping` deduplicates
identical mappings.

**Gap.** The promised payoff - "simplify automated translation between
any languages" - is only demonstrated for one hard-coded proposition:
`src/reconstruction.rs:has_statehood_proposition` /
`reconstruct_statehood`. The concept space is not consulted for general
sentences or for code.

### R-7 - Storage presented as links-notation text

> "fully support storage (presenting as links notation (text based as in ... links-notation)"

**Status: Partial.**
`src/lino_parser.rs` parses LiNo doublets, triplets, N-tuples, indented
definitions, named links, and self-references into `LinkType::Relation`
links (gated by `tests/unit/links_notation.rs` and the links-notation
parity fixtures with the verified 137/138/138/140 upstream test-count
provenance). `LinkNetwork::self_description_text`
(`src/link_network.rs`) serializes the self-description roots as
LiNo-style definition lines.

**Gap.** There is no general `LinkNetwork` → LiNo serializer: an arbitrary
parsed network (for example a Python file) cannot be exported as
links-notation text and re-imported. Storage "presented as links
notation" therefore only covers the input direction plus one special-case
output.

### R-8 - Binary doublets storage

> "binary links as in https://github.com/linksplatform/doublets-rs and https://github.com/linksplatform/doublets-web"

**Status: Missing.**
`Cargo.toml` has no `doublets` (or any persistence) dependency, and the
only mentions of "doublets" in `src/` are provenance strings naming the
upstream C# test project (`src/parity_fixtures.rs`). `LinkNetwork`
(`src/link_network.rs`) is purely in-memory (`BTreeMap`-backed) with no
binary encode/decode or memory-mapped storage. Closed issue #14 mentioned
an "optional doublets substrate" but no such substrate landed.

### R-9 - Rust traits/types representation

> "and also Rust traits/types representation"

**Status: Partial.**
The crate exposes a strongly typed Rust surface (`Link`, `LinkId`,
`LinkMetadata`, `LinkType`, `LinkFlags` in `src/link_network.rs` /
`src/link_flags.rs`), `LinkType::Object` with
`LinkNetwork::insert_object`, and object identity / shared-reference /
circular-reference behavior ported from lino-objects-codec
(`src/parity_fixtures.rs`, `ObjectRoundTrip` capability in
`src/parity.rs`).

**Gap.** There is no codec that maps *user-defined* Rust structs, enums,
or trait objects into links and back (no derive macro, no serde-style
`ToLinks`/`FromLinks` trait). "Rust traits/types representation" is
currently limited to the crate's own internal types.

### R-10 - formal-ai as the heaviest user: any language treated easily as data

> "github.com/link-assistant/formal-ai will be our the most heavy user, so we need to make sure any language can be treated easily as data"

**Status: Partial.**
formal-ai is a first-class parity target
(`src/parity.rs:PARITY_TARGETS`) with `FormalizationRoundTrip`,
`SemanticEvaluation`, and `CrossLanguageReconstruction` capabilities, and
its fixtures cite actual `data/seed/*.lino` and `data/benchmarks/*.lino`
files (`src/parity_fixtures.rs`, gated by `tests/unit/parity_corpora.rs`).
Because every parse produces one queryable, substitutable links network,
language-as-data holds for all wired grammars.

**Gap.** "Any language" inherits the R-2/R-3/R-4/R-5 grammar gaps, and the
formalization pipeline formal-ai would drive is demo-grade (R-6, R-15).

### R-11 - link-cli-style transformations and substitutions

> "we can do any transformations and substitutions in style of github.com/link-foundation/link-cli"

**Status: Implemented.**
`src/substitution.rs` provides `SubstitutionRule`
(`new`/`create`/`delete`) and `VariableSubstitutionRule` with
link-cli-style `$variable` patterns, index variables, and
`SubstitutionBindings`; `LinkNetwork::apply_substitution` and
`apply_variable_substitution` (`src/link_network.rs`) return
created/updated/deleted reports. Create, update, delete, and swap fixtures
ported from `Foundation.Data.Doublets.Cli.Tests` are gated in
`src/parity_fixtures.rs` plus `tests/unit/substitution.rs`. The
transform surface (`src/transform.rs:ReplacementRule`) can also wrap both
rule kinds.

### R-12 - Chaining API adapter

> "other traditional API adapters, like chaining"

**Status: Partial.**
Builder-style chaining exists for query construction
(`src/query.rs`: `LinkQuery::by_type(..).with_term(..).with_language(..)
.with_named(..)`), metadata (`LinkMetadata::with_*`), and configuration
(`ParseConfiguration::with_*`).

**Gap.** There is no fluent chained pipeline over *operations* - nothing
like jscodeshift's `j(source).find(..).replaceWith(..).toSource()` that
chains parse, query, transform, and reconstruction; each step is a
separate direct method call on `LinkNetwork`.

### R-13 - Direct OOP API

> "direct OOP methods and so on"

**Status: Implemented.**
`LinkNetwork` (`src/link_network.rs`) is a conventional object API:
`parse`, `query_links` / `query_matches` / `query_matches_with`,
`apply_substitution`, `apply_variable_substitution`,
`apply_replacements` (`src/transform.rs`), `snapshot`
(`src/snapshots.rs`), `reconstruct_text` / `reconstruct_text_as`,
`verify_full_match`, `seed_common_concept_ontology`, and link accessors.
All exercised by `tests/unit/`.

### R-14 - All API styles support all the same operations

> "all the same operations should be possible to be used by all kinds of ways ... All of APIs styles should support all the same operations"

**Status: Missing.**
Style coverage is uneven and unverified: queries have two styles (builder
and S-expression text via `src/query.rs:LinkQuery::from_sexpression`),
substitutions and snapshots have only direct methods, and there is no
chaining adapter (R-12). No test or contract asserts operation parity
across API styles, so the explicit requirement that every operation be
reachable in every style has no implementation or gate.

### R-15 - Translate any language to and from meta language

> "provide ability to translate any other language to meta language and from meta language"

**Status: Partial.**
To-meta: `LinkNetwork::parse` converts text in any wired language into
the links network. From-meta: `reconstruct_text` regenerates
byte-identical source, and `reconstruct_text_as`
(`src/reconstruction.rs`) renders other target languages and
formalization levels (`FormalizationLevel::{Natural, Lexical, Concept,
Logical}` in `src/configuration.rs`), gated by
`tests/unit/cross_language_reconstruction.rs`.

**Gap.** Cross-language emission works only when the network contains the
single hard-coded `proposition:statehood` semantic link
(`src/reconstruction.rs:has_statehood_proposition`); everything else
falls back to the original source. General translation through the
concept space is not implemented.

### R-16 - Single-language restriction profiles

> "if we want to keep working with single language for example for javascript to javascript transformation - we just restrict ourselves with using only features of meta language that JavaScript supports exactly"

**Status: Missing.**
No symbol in `src/` models a per-language feature profile or a restricted
operating mode. Same-language transformation works *implicitly*
(parse JavaScript, transform, reconstruct), but there is no API to
declare "JavaScript-only" and have the engine reject or hide
meta-language features that JavaScript does not support, and no
capability matrix mapping meta-language features to per-language support.

### R-17 - User-expandable/configurable translation rules

> "all missing translation from meta language and to meta language are expandable and configurable by end user ... cross language rules based translation, with full freedom of configuration"

**Status: Partial.**
Extension points that exist: `LinkNetwork::insert_concept_mapping`
(`src/concept_ontology.rs`) lets users add concept-to-syntax mappings per
language; the `LanguageParser` trait (`src/language_parser.rs`) is public
so a custom parser can be invoked directly; `QueryPredicateHost`
(`src/query.rs`) lets callers supply custom predicate evaluation via
`query_matches_with`.

**Gap.** `LinkNetwork::parse` hardwires `BuiltInLanguageParser`
(`src/link_network.rs:344`) with no registry or injection point for
user-supplied parsers or emitters, and there is no rule-based translation
system (no user-defined to-meta/from-meta rewrite rules beyond the single
concept-syntax map consulted by the statehood demo).

### R-18 - Everything replaceable and configurable

> "everything in our system should be replaceable, configurable and so on"

**Status: Partial.**
`src/configuration.rs:ParseConfiguration` makes several behaviors
configurable: `TriviaAttachmentPolicy`, `RegionDetectionPolicy`,
`LanguageIdentificationDetector` (lingua vs whatlang),
`FormalizationLevel`, and `NaturalizationDirection`.

**Gap.** Major subsystems are fixed: the grammar table
(`src/tree_sitter_adapter.rs:grammar_for_language` is a private match),
the parser used by `parse` (R-17), the reconstruction strategy, the
natural-language segmenters, and the storage representation (in-memory
only, R-8) cannot be replaced by the end user.

### R-19 - Nothing deferred or left unimplemented in vision/roadmap

> "We should check that nothing is defered or left unimplemented in our vision and roadmap"

**Status: Partial.**
Roadmap audit issues #37 and #39 ("Implement unimplemented in our vision
and roadmap") are closed, and registry tests in
`tests/unit/link_network.rs` keep every advertised target, capability, and
fixture present.

**Gap.** `docs/parity-roadmap.md` still contains explicit deferrals: SQL
dialects are advertised "until separate dialect grammars such as BigQuery,
SQLite, PostgreSQL, or T-SQL are wired and tested under their own keys",
and Delphi "version-specific ... differences ... remain outside the
advertised grammar-backed scope". The natural-language grammar gap (R-5),
data-exchange gap (R-3), and doublets gap (R-8) are likewise unimplemented
vision items.

### R-20 - 100% test coverage copying competitor test cases

> "We should have 100% tests coverage, which should copy most of test cases from our competitors in each sector/scope"

**Status: Partial.**
Coverage is measured (`cargo llvm-cov` in
`.github/workflows/release.yml`, uploaded to Codecov) and competitor test
porting is structural: 48 provenance-bearing fixtures in
`src/parity_fixtures.rs` across all 13 `PARITY_TARGETS`, with minimum
fixture counts and capability coverage enforced by
`tests/unit/parity_corpora.rs` and `tests/unit/link_network.rs`.

**Gap.** No 100% threshold is enforced anywhere (the Codecov step sets
`fail_ci_if_error: false`; no `--fail-under-lines` flag), and the ported
corpora are small samples - e.g. links-notation provenance records
137-140 upstream cases per language while only a handful are ported -
so "most of test cases from our competitors" is not met.

### R-21 - Sub-issue creation with blocked-by markings

> "For each aspect of this task, we should create issues on GitHub, with clear blocked by markings ... all tasks should also be subtasks of this task"

**Status: Missing.**
The issue snapshot itself records the absence:
[`raw-data/issue-47.json`](./raw-data/issue-47.json) shows
`"sub_issues_summary": {"total": 0, ...}` and
`"issue_dependencies_summary": {"blocked_by": 0, ...}`, and
[`raw-data/all-issues.json`](./raw-data/all-issues.json) contains no
issues filed for issue #47 aspects. Unlike issue #3 (which produced
proposed-issue specs and filed #5-#20), no `proposed-issues/` folder or
filed sub-issues exist for this task.

### R-22 - Case-study compilation and analysis in `docs/case-studies/issue-47`

> "make sure we compile that data to `./docs/case-studies/issue-{id}` folder, and use it to do deep case study analysis ... list of each and all requirements ... propose possible solutions and solution plans"

**Status: Partial.**
Raw data is collected: [`raw-data/`](./raw-data/) holds `issue-47.json`,
`issue-47-comments.json`, `all-issues.json`, and `pr-48.json`.

**Gap.** Compared with the issue #3 case study
(`../issue-3/`: `requirements.md`, `solution-plans.md`,
`online-research.md`, `rust-libraries-survey.md`,
`competitor-test-suites.md`, `ecosystem-foundations.md`,
`proposed-issues/`), this folder previously contained only raw data. This
register supplies the requirements list; the deep analysis, online
research, per-requirement solution plans, and component/library survey
documents are still absent.
