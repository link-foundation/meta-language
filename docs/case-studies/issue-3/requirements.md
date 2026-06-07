# Requirements Register - Issue #3

This register extracts the requirements from
[issue #3](https://github.com/link-foundation/meta-language/issues/3) and the
founding vision in
[issue #1](https://github.com/link-foundation/meta-language/issues/1), then maps
each requirement to the current implementation state after merging the latest
default branch on 2026-06-07.

The original solution plan remains in [`solution-plans.md`](./solution-plans.md).
The issue bodies generated from that plan remain in [`proposed-issues/`](./proposed-issues/).
Those issues were filed as [#5](https://github.com/link-foundation/meta-language/issues/5)
through [#20](https://github.com/link-foundation/meta-language/issues/20) and are
now closed.

## Status Vocabulary

| Status | Meaning |
|---|---|
| **Done** | The issue #3 process requirement is complete. |
| **Implemented baseline** | Runtime behavior and executable tests exist for the requirement's scoped baseline. |
| **Implemented fixture gate** | Parity is represented by provenance-bearing executable fixtures and tests. |
| **Tracked for expansion** | The baseline is implemented; future dialects, larger corpora, or additional language-specific cases can extend the existing registry. |

## Part A - Issue #3 Process Requirements

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **I3-1** | Create issues so the Rust libraries cover in/out processing for the top-10 programming languages, top-10 natural languages, and mixed-mode txt/Markdown/HTML. | Issues [#5](https://github.com/link-foundation/meta-language/issues/5) through [#20](https://github.com/link-foundation/meta-language/issues/20) were filed with `gh` from the source specs in [`proposed-issues/`](./proposed-issues/) and are now closed. | **Done** |
| **I3-2** | Copy competitor tests and support comparable features so users can transition. | External and ecosystem targets are represented in `PARITY_TARGETS` and `PARITY_FIXTURES`; tests require provenance, fixture coverage, recovery behavior, transform expectations, and minimum fixture counts for each target. Implemented by [#19](https://github.com/link-foundation/meta-language/issues/19) and [#20](https://github.com/link-foundation/meta-language/issues/20). | **Implemented fixture gate** |
| **I3-3** | Reference issue #1 for vision and initial implementation. | Vision requirements are extracted in Parts C-E; raw snapshots are in [`raw-data/issue-1.json`](./raw-data/issue-1.json) and [`raw-data/issue-1-comments.json`](./raw-data/issue-1-comments.json). | **Done** |
| **I3-4** | Collect data under `./docs/case-studies/issue-3`. | Research documents, issue specs, and raw GitHub snapshots are stored in this folder. | **Done** |
| **I3-5** | Do deep case-study analysis with online facts and data. | [`online-research.md`](./online-research.md), [`rust-libraries-survey.md`](./rust-libraries-survey.md), [`competitor-test-suites.md`](./competitor-test-suites.md), and [`ecosystem-foundations.md`](./ecosystem-foundations.md) record the analysis. | **Done** |
| **I3-6** | List each and every requirement. | This register covers the process, language, capability, parity, and cross-cutting requirements. | **Done** |
| **I3-7** | Propose solutions and plans for each requirement, checking existing components/libraries. | [`solution-plans.md`](./solution-plans.md) and [`rust-libraries-survey.md`](./rust-libraries-survey.md) supplied the plan used to file issues #5-#20. | **Done** |

## Part B - Language And Format Coverage

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **LANG-PL** | Parse and reconstruct the top-10 TIOBE-May-2026 programming-language targets: Python, C, Java, C++, C#, JavaScript, Visual Basic, R, SQL, and Delphi/Object Pascal. | `PROGRAMMING_LANGUAGE_TARGETS` lists all ten. `LANGUAGE_FIXTURES` round-trip all ten. `LinkNetwork::parse` dispatches to tree-sitter grammars for Python, C, Java, C++, C#, JavaScript, Visual Basic, R, `sql-ansi`, and Delphi/Object Pascal. | **Implemented baseline** |
| **LANG-NL** | Parse and reconstruct the ten target natural languages: English, Mandarin Chinese, Hindi, Spanish, Modern Standard Arabic, French, Bengali, Portuguese, Russian, and Urdu. | `NATURAL_LANGUAGE_TARGETS` uses the Ethnologue/Britannica total-speaker order. Fixtures round-trip all ten. Natural-language parsing annotates regions with segmentation, language identification, normalization, and bidi metadata. | **Implemented baseline** |
| **LANG-TXT** | Treat `txt` as a first-class container target. | `MARKUP_LANGUAGE_TARGETS` includes `txt`; tests assert whole-buffer txt regions and txt fallback when content-driven region detection cannot classify a fenced block. | **Implemented baseline** |
| **LANG-MD** | Full Markdown container support. | Markdown is registered, fixtures round-trip, and fenced-code/HTML regions are detected and attached as embedded regions in one network. | **Implemented baseline** |
| **LANG-HTML** | Full HTML container support. | HTML is registered, fixtures round-trip, and script/style/style-attribute regions are parsed as JavaScript/CSS embedded regions. | **Implemented baseline** |
| **LANG-MIX** | Mixed-mode txt/Markdown/HTML with embedded code/HTML/CSS/JS in one links network. | `GRAMMAR_EMBEDDING_TARGETS` covers Markdown code, Markdown HTML, HTML JavaScript, and HTML CSS. Tests assert name-driven and content-driven detection, txt fallback, byte-exact reconstruction, and embedded syntax connected to region links. | **Implemented baseline** |

## Part C - Vision Capabilities From Issue #1

### C.1 Universal Representation

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-1** | One mutable links network representing CST, AST, and semantic relations simultaneously over the same links. | `NetworkProjection::{Lossless, ConcreteSyntax, AbstractSyntax, Semantic}` exposes views over one `LinkNetwork`. | **Implemented baseline** |
| **CORE-2** | Lossless reconstruction with explicit trivia and configurable trivia attachment. | `reconstruct_text()` round-trips fixtures byte-for-byte; `TriviaAttachmentPolicy` supports containment, token, or both policies. | **Implemented baseline** |
| **CORE-3** | Rich per-link metadata: named/anonymous, fields, byte ranges, points, and error/missing/extra flags. | `LinkMetadata`, `LinkType`, `LinkFlags`, `ByteRange`, `Point`, and `SourceSpan` are populated by the generic and tree-sitter-backed parsers; tests assert tree-sitter fields become `LinkType::Field` links. | **Implemented baseline** |
| **CORE-4** | Error recovery and partial parsing represented as links. | Tree-sitter recovery cases preserve original bytes while exposing error, has-error, and missing flags through `verify_full_match()`. | **Implemented baseline** |

### C.2 Parse, Verify, And Mixed Languages

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-5** | `parse(text, language) -> network` for selected programming and natural languages. | `LinkNetwork::parse` uses `BuiltInLanguageParser`, tree-sitter grammar dispatch, natural-language annotation, and lossless text fallback. | **Implemented baseline** |
| **CORE-6** | Verify whether text fully matches a language and return failing regions/links. | `verify_full_match()` reports `VerificationIssueKind` entries with spans for error, has-error, and missing links. | **Implemented baseline** |
| **CORE-7** | Mixed-language parsing with auto-detected regions in one unified network. | Mixed-region tests assert Markdown/HTML host parsing, embedded JavaScript/CSS/HTML/Rust/SQL regions, content-driven detection, and txt fallback in one network. | **Implemented baseline** |

### C.3 Self-Description

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-8** | The meta language describes itself in its own terms. | `LinkNetwork::self_describing()` seeds root links for `link`, `reference`, `relation link`, `language`, `grammar`, `type`, `Type`, `concept`, `point`, `field`, `trivia`, `region`, and `object`; tests assert definitions use only root vocabulary. | **Implemented baseline** |
| **CORE-9** | Include common roots of common ontologies. | The self-description network includes the relative-meta-logic-style `(Type: Type Type)` root and definition closure tests. | **Implemented baseline** |

### C.4 Shared Concepts

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-10** | A common concept layer mapped to per-language syntax, with shared concept identity across languages. | `seed_common_concept_ontology()` imports the verified 351-concept meta-expression lexicon, seeds structural concepts, maps concepts to multiple languages, and tests shared concept identity. | **Implemented baseline** |

### C.5 Snapshots And Mutation

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-11** | Immutable snapshots and mutable snapshots for editing. | `NetworkSnapshot` and `MutableNetworkSnapshot` support immutable snapshots, editable forks, and commits. | **Implemented baseline** |
| **CORE-12** | Versioning over time with provenance and forward commits. | Snapshot tests assert version numbers, parent versions, provenance, forward commits, and structural sharing of unchanged links. | **Implemented baseline** |

### C.6 Transformation

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-13** | Transform via link-cli-style match/substitute rules. | `SubstitutionRule`, `VariableSubstitutionRule`, and `apply_substitution()` cover create, update, delete, swap, and variable binding cases. | **Implemented baseline** |
| **CORE-14** | Advanced matching by syntax, meaning, and type. | `LinkQuery::from_sexpression`, captures, by-type matching, language/term/name predicates, and query tests cover the baseline syntax/type matcher; semantic projection and concept links provide the meaning layer seed. | **Implemented baseline** |
| **CORE-15** | Host-evaluated predicates/text conditions over structural captures. | `QueryPredicateHost` and `SourceTextPredicateHost` allow host-side predicate evaluation; tests exercise equality and source-text replacement predicates. | **Implemented baseline** |

### C.7 Reconstruction And Formalization

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-16** | Same-language and cross-language reconstruction through concepts. | `reconstruct_text()` preserves same-language bytes; `reconstruct_text_as()` reconstructs the Hawaii statehood fixture between English and Russian through shared concept links. | **Implemented baseline** |
| **CORE-17** | Configurable formalization/deformalization. | `ParseConfiguration` exposes `FormalizationLevel` and `NaturalizationDirection`; tests assert lexical, concept, and logical formalization outputs. | **Implemented baseline** |

## Part D - Competitor And Ecosystem Parity

Each parity target is represented in `PARITY_TARGETS` and has one or more
provenance-bearing executable fixtures in `PARITY_FIXTURES`. Tests assert that
every advertised capability is exercised by fixtures and that fixture
reconstruction, verification, and transform expectations hold.

| ID | Source project | Required parity | Current state | Status |
|---|---|---|---|---|
| **PAR-1** | tree-sitter | Lossless CST, recovery, mixed-language injection, query matching. | Multiple ported fixtures cover CST, extras/trivia, error corpus, query source, and fenced-code mixed regions. | **Implemented fixture gate** |
| **PAR-2** | LibCST | Python lossless parse, trivia, metadata, parser errors, transforms, reconstruction. | Ported Python fixtures cover comments, empty lines, parse errors, and identifier transforms. | **Implemented fixture gate** |
| **PAR-3** | Recast | JavaScript/TypeScript parse-print preservation. | Ported JavaScript fixtures cover comments, regexp properties, empty programs, and source-preserving replacement. | **Implemented fixture gate** |
| **PAR-4** | jscodeshift | Transform workflows over JavaScript/TypeScript. | Ported input/output-style fixtures plus query/replace tests cover captured identifier transforms. | **Implemented fixture gate** |
| **PAR-5** | Rowan + cstree | Persistent CSTs, immutable snapshots, checkpoints, interning. | Fixtures and snapshot tests cover trivia round-trips, versioning, structural sharing, and interned metadata terms. | **Implemented fixture gate** |
| **PAR-6** | Roslyn | C# syntax, trivia, diagnostics, formatting, transforms. | Ported fixtures and recovery tests cover C# parsing, diagnostics, trivia, and replacements. | **Implemented fixture gate** |
| **PAR-7** | links-notation | LiNo doublets, triplets, N-tuples, indentation, self-reference. | Ported fixtures cover all listed forms and record verified cross-language test counts. | **Implemented fixture gate** |
| **PAR-8** | link-cli | Single match/substitute create, update, delete, swap. | Ported fixtures cite `Foundation.Data.Doublets.Cli.Tests`; substitution tests cover the operation forms. | **Implemented fixture gate** |
| **PAR-9** | lino-objects-codec | Object round-trip, shared references, circular references. | Ported fixtures plus object identity tests cover primitive, shared, and circular object cases. | **Implemented fixture gate** |
| **PAR-10** | relative-meta-logic | Dependent types, many-valued evaluation, paradox cases. | Ported fixtures and `TruthValue` tests cover dependent, many-valued, and liar-paradox cases. | **Implemented fixture gate** |
| **PAR-11** | formal-ai | Formalization corpus and semantic reconstruction expectations. | Ported fixtures cite actual `data/seed/` and `data/benchmarks/` corpus files, avoiding the unverified 706-count claim. | **Implemented fixture gate** |
| **PAR-12** | meta-expression | Formalize, semantic-link, naturalize, spans, self-reference. | Ported Hawaii, `1 + 1`, and self-reference fixtures plus the 351-concept ontology seed cover the baseline. | **Implemented fixture gate** |

## Part E - Cross-Cutting Requirements

| ID | Requirement | Source | Current state |
|---|---|---|---|
| **NFR-1** | Use links-network terminology; translate external node/edge/tree vocabulary at adapter boundaries. | Issue #1 terminology note | Honored in public API and docs; tree-sitter details stay behind `tree_sitter_adapter`. |
| **NFR-2** | Tests live under `tests/`; CI rejects test modules under `src/`. | `CONTRIBUTING.md` | Honored by all implementation PRs. |
| **NFR-3** | Changelog fragment per user-facing runtime change. | `CONTRIBUTING.md` and CI | Issues #5-#20 each landed a changelog fragment. |
| **NFR-4** | Crate/file-size limits are enforced. | `CONTRIBUTING.md` | Package include list excludes case studies and generated artifacts. |
| **NFR-5** | Try compatible alternatives; expose config when choices conflict. | Issue #1 comment | Implemented through configurable trivia attachment, region detection, language identification detector, formalization level, and naturalization direction. |
| **NFR-6** | First implementation language is Rust. | Issue #1 | This crate is Rust-first. |
| **NFR-7** | Reuse existing ecosystem components rather than reinvent. | Issue #1 and I3-7 | Reused tree-sitter grammar crates, `lingua`, `whatlang`, `lindera`, Unicode crates, and imported ecosystem fixture shapes. |

## Traceability Summary

- Issue #3 process requirements are complete.
- Coverage requirements for the requested programming languages, natural
  languages, and txt/Markdown/HTML mixed mode have implementation baselines and
  executable tests.
- Vision capability requirements from issue #1 have baseline runtime support
  through the modules and tests added by issues #5-#20.
- Competitor and ecosystem parity is no longer represented by single illustrative
  entries only; the crate now has provenance-bearing executable fixture gates with
  minimum coverage assertions.
- Remaining future work is expansion work: additional SQL dialect keys, deeper
  Delphi/Visual Basic fixtures, more upstream corpus ports, more concept mappings,
  and more language-specific parser alternatives can be added under the current
  registries without changing the issue #3 plan.
