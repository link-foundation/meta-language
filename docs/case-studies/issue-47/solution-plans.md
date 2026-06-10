# Solution Plans - Issue #47

This document proposes one solution plan per requirement cluster from
[`requirements.md`](./requirements.md) (R-1 ... R-22), grounded in the
competitor and ecosystem research in
[`competitors-code-tooling.md`](./competitors-code-tooling.md),
[`competitors-natural-language.md`](./competitors-natural-language.md), and
[`formats-storage-apis.md`](./formats-storage-apis.md). Each plan names the
existing components and libraries to reuse and ends in a proposed
implementation issue under [`proposed-issues/`](./proposed-issues/).

Research and audit date: 2026-06-10. Versions and licenses cited below were
verified in the linked research documents.

## Plan index

| Plan | Requirements | Proposed issue | Blocked by |
|---|---|---|---|
| S-1 Readonly/mutable engine configuration | R-1, R-18 | [`01`](./proposed-issues/01-readonly-mutable-engine-configuration.md) | - |
| S-2 Data-exchange format grammars | R-3, R-4 | [`02`](./proposed-issues/02-data-exchange-format-grammars.md) | - |
| S-3 Next programming-language grammar wave | R-2 | [`03`](./proposed-issues/03-programming-language-grammar-wave.md) | - |
| S-4 Pluggable language-parser registry | R-17, R-18 | [`04`](./proposed-issues/04-pluggable-language-parser-registry.md) | - |
| S-5 Links-notation network serialization | R-7 | [`05`](./proposed-issues/05-lino-network-serialization.md) | - |
| S-6 Doublets binary storage backend | R-8, R-18 | [`06`](./proposed-issues/06-doublets-binary-storage-backend.md) | 01, 05 |
| S-7 Rust types/traits ↔ links codec | R-9 | [`07`](./proposed-issues/07-rust-types-links-codec.md) | 05 |
| S-8 Natural-language grammatical-correctness parsing | R-5 | [`08`](./proposed-issues/08-natural-language-grammar-parsing.md) | - |
| S-9 Exact-match shared concept space | R-6 | [`09`](./proposed-issues/09-shared-concept-space-exact-match.md) | - |
| S-10 Configurable translation-rule registry | R-15, R-17 | [`10`](./proposed-issues/10-translation-rule-registry.md) | 04, 09 |
| S-11 Single-language restriction profiles | R-16 | [`11`](./proposed-issues/11-language-restriction-profiles.md) | 10 |
| S-12 Query/transform algebra enrichment | R-11+, richest-feature-set | [`12`](./proposed-issues/12-query-transform-algebra.md) | - |
| S-13 Incremental re-parse and structural sharing | richest-feature-set | [`13`](./proposed-issues/13-incremental-reparse-structural-sharing.md) | - |
| S-14 API-style parity contract | R-12, R-13, R-14 | [`14`](./proposed-issues/14-api-style-parity-contract.md) | 06, 10 |
| S-15 Competitor corpora wave 2 + coverage gate | R-19, R-20 | [`15`](./proposed-issues/15-competitor-corpora-and-coverage-gate.md) | 02, 08, 12, 14 |

R-10 (formal-ai as the heaviest user) is satisfied across S-2, S-5, S-6, and
S-10: `formal-ai` already defines `LinkStoreBackend { LinoProjection,
DoubletsRs, DoubletsWeb }` and consumes `.lino` seed/benchmark files, so the
storage and translation plans deliberately match its stack (see
[`formats-storage-apis.md`](./formats-storage-apis.md) Part B).
R-21 (sub-issues with blocked-by markings) and R-22 (case-study compilation)
are delivered by this case study itself.

## S-1: Readonly/mutable engine configuration (R-1)

**Problem.** `LinkNetwork` is always mutable; `NetworkSnapshot` provides
immutability only as an opt-in versioning pattern, not a user configuration.

**Plan.**
- Add an `AccessMode { ReadOnly, Mutable }` knob to `ParseConfiguration` and a
  `LinkNetwork::freeze()` / `as_read_only()` boundary that yields a read-only
  view type exposing only `&self` operations (parse, query, project,
  reconstruct, verify).
- Model the split the way the Rust ecosystem does (see
  [`formats-storage-apis.md`](./formats-storage-apis.md) Part C §5): reads on a
  shared reference, writes on `&mut self` or an explicit mutable fork, plus a
  frozen wrapper type so misuse is a compile-time error where possible and a
  configuration-checked runtime error at API boundaries.
- Reuse `NetworkSnapshot`'s `Arc<LinkNetwork>` sharing for the frozen form so
  the feature composes with existing snapshot versioning instead of adding a
  second immutability mechanism.

**Reuse.** `src/snapshots.rs`, `src/configuration.rs`; precedent: Roslyn's
fully immutable red/green trees and rowan/cstree persistent trees
([`competitors-code-tooling.md`](./competitors-code-tooling.md)).

## S-2: Data-exchange format grammars (R-3, R-4)

**Problem.** No mainstream data-exchange format has a wired grammar; JSON,
YAML, XML, TOML, CSV, INI, protobuf, GraphQL all fall back to plain-text
tokens.

**Plan.**
- Adopt the verified-compatible tree-sitter grammar crates (all use the same
  `tree-sitter-language ^0.1` binding model as the project's tree-sitter
  0.25.8): `tree-sitter-json` 0.24.8, `tree-sitter-yaml` 0.7.2,
  `tree-sitter-toml-ng` 0.7.0, `tree-sitter-xml` 0.7.0 (XML + DTD),
  `tree-sitter-ini` 1.4.0, `tree-sitter-proto` 0.4.0, `tree-sitter-graphql`
  0.1.0.
- CSV and JSON5 lack modern-binding crates.io releases (both pin tree-sitter
  ~0.20); vendor the upstream grammars or defer those two to a follow-up noted
  in the roadmap (no silent gaps).
- Add a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` mirroring
  `MARKUP_LANGUAGE_TARGETS`, with `LANGUAGE_FIXTURES` round-trip fixtures per
  format and mixed-region detection (e.g. JSON in Markdown fences).

**Reuse.** `src/tree_sitter_adapter.rs` grammar wiring pattern from PRs
#26-#28 and #44-#46; crate table in
[`formats-storage-apis.md`](./formats-storage-apis.md) Part A.

## S-3: Next programming-language grammar wave (R-2)

**Problem.** 17 grammars are wired (TIOBE top-10 + Rust, Go, Ruby,
TypeScript/TSX, HTML, CSS), but "all popular programming languages" extends
past the current set: PHP, Swift, Kotlin, Scala, Lua, Perl are all in the
TIOBE top-20 with maintained official tree-sitter grammars.

**Plan.**
- Wire `tree-sitter-php`, `tree-sitter-swift`, `tree-sitter-kotlin` (community,
  verify binding generation), `tree-sitter-scala`, `tree-sitter-lua`, and
  `tree-sitter-perl`, following the exact pattern of PR #44-#46.
- Extend `PROGRAMMING_LANGUAGE_TARGETS` (or add a `TIOBE_11_20_TARGETS` tier)
  plus per-language `LANGUAGE_FIXTURES` with UTF-8 and recovery cases.
- Document the acquisition order and any grammar whose crates.io release pins
  an old tree-sitter (vendor or defer explicitly).

**Reuse.** Same adapter seam as S-2; TIOBE source already cited in
`docs/parity-roadmap.md`.

## S-4: Pluggable language-parser registry (R-17, R-18)

**Problem.** `LinkNetwork::parse` hardwires `BuiltInLanguageParser`; users
cannot register a new language, override a grammar, or replace parser
dispatch, although the `LanguageParser` trait already exists.

**Plan.**
- Introduce a `ParserRegistry` (language key → `Arc<dyn LanguageParser>`) with
  the built-in set pre-registered; `ParseConfiguration` gains a
  `with_parser_registry(...)` override so every entry point honors it.
- Follow TXL's "grammar override" idea and SWC's plugin lesson
  ([`competitors-code-tooling.md`](./competitors-code-tooling.md)): user
  registrations shadow built-ins for the same key rather than forking the
  pipeline.
- Keep terminology translation at the boundary per `docs/parity-roadmap.md`:
  registered parsers must produce links, not foreign ASTs.

**Reuse.** `LanguageParser` trait in `src/language_parser.rs`,
`QueryPredicateHost` extension precedent in `src/query.rs`.

## S-5: Links-notation network serialization (R-7)

**Problem.** LiNo parses in (`src/lino_parser.rs`) but arbitrary networks
cannot be written back out as links-notation text; only
`self_description_text` emits LiNo-style lines.

**Plan.**
- Implement `LinkNetwork::to_lino()` / `from_lino()` providing a total
  serialization of any network (references, names, metadata links) with a
  round-trip property test: `from_lino(to_lino(n))` is isomorphic to `n`.
- Align the dialect with the `links-notation` Rust crate 0.13.0 (the reference
  parser, also pinned by formal-ai) so emitted text is consumable by every
  ecosystem parser; record any divergence as parity fixtures.
- Reuse the doublets-style id discipline for unnamed links so binary and text
  storage share one addressing scheme (prepares S-6).

**Reuse.** `src/lino_parser.rs`, `links-notation` crate;
[`formats-storage-apis.md`](./formats-storage-apis.md) Part B.

## S-6: Doublets binary storage backend (R-8, R-18) - blocked by 01, 05

**Problem.** No binary links storage exists; `Cargo.toml` has no `doublets`
dependency.

**Plan.**
- Extract a storage trait with `&self` reads / `&mut self` writes (S-1 defines
  the access-mode semantics) covering create/read/update/delete/search over
  links, with the in-memory `LinkNetwork` as the default implementation.
- Add a feature-gated `doublets` backend using `doublets` 0.4.0 (published
  2026-05-29, stable Rust 1.85, Unlicense, file-mapped persistence) and adopt
  its own three-layer API precedent: raw `Links<T>` ops, ergonomic defaults,
  iterator extensions.
- Match formal-ai's `LinkStoreBackend` enum (LinoProjection, DoubletsRs,
  DoubletsWeb) so meta-language can be dropped under it; doublets-web remains
  out of process (WASM) and is documented as an exchange target via the same
  binary layout rather than a linked dependency.
- S-5's serializer provides the text↔binary bridge fixtures.

**Reuse.** `doublets` crate; doublets-vs-RDF analysis in
[`formats-storage-apis.md`](./formats-storage-apis.md) Part B.

## S-7: Rust types/traits ↔ links codec (R-9) - blocked by 05

**Problem.** `LinkType::Object` and `insert_object` cover identity and
circular references, but arbitrary user structs/enums/traits cannot be encoded
to links and decoded back.

**Plan.**
- Provide `ToLinks` / `FromLinks` traits with implementations for primitives,
  `Vec`, `Option`, maps, and a `#[derive(Links)]` proc-macro (or a
  serde-Serializer adapter writing into the network — decide in the issue;
  serde adapter reuses the whole serde ecosystem at zero macro cost).
- Keep parity with `lino-objects-codec` 0.2.1 (Rust) / 0.4.0 (npm): port its
  shared-reference and circular-reference cases as fixtures; S-5's text form
  makes the encoding inspectable.
- Trait *types* (not just values): represent Rust type declarations as links
  via the existing self-description roots (`type`, `Type`, `field`), so a
  type's shape is itself queryable data.

**Reuse.** `src/parity_fixtures.rs` lino-objects-codec fixtures,
`src/self_description.rs`; serde precedent in
[`formats-storage-apis.md`](./formats-storage-apis.md) Part C.

## S-8: Natural-language grammatical-correctness parsing (R-5)

**Problem.** Natural-language support is identification + segmentation +
script annotations; nothing parses grammar, so grammatical correctness cannot
be checked. Issue #47 explicitly scopes semantics out and grammar in.

**Plan** (staged; see
[`competitors-natural-language.md`](./competitors-natural-language.md)
"Recommended approach signals"):
1. **Vocabulary first.** Adopt Universal Dependencies' UPOS/UFeats/deprel
   inventory as link-type vocabulary for morphosyntax links; import CoNLL-U
   with `rs-conllu` for fixtures and regression corpora (per-treebank licenses
   recorded in provenance).
2. **Word-level correctness.** Build morphological lexica from Wikidata
   lexeme Forms (CC0) first and UniMorph TSV (CC BY-SA) second; an unknown or
   wrongly inflected form is a recoverable `is_error` link, mirroring the
   existing parse-recovery contract.
3. **Sentence-level correctness.** Grammatical Framework is the only surveyed
   system doing exactly this job (parse-or-reject, no semantics, RGL covers
   the top-10 target languages, LGPL/BSD): compile RGL grammars to PGF and
   read them from Rust (the young `gf-core` crate now; a native PMCFG reader
   over the links network as the robust long-term path).
4. **Explainable negatives.** Port LanguageTool-style pass/fail rule
   sentences (LGPL; `nlprule` proves the Rust port) and DELPH-IN "mal-rule"
   ideas so failures carry explanations, again as error links.

**Reuse.** `src/natural_language.rs`, recovery flags in `src/link_flags.rs`;
`NATURAL_LANGUAGE_TARGETS` gates extended with grammar fixtures per language.

## S-9: Exact-match shared concept space (R-6)

**Problem.** The 351-concept lexicon and `reference_for_atom` exact-`find_term`
reuse exist, but concepts are not interned across languages with a stated
exact-match discipline, and translation through concepts is demo-grade.

**Plan.**
- Copy Wikidata's two-layer design: language-bound lexeme links connected to
  language-free concept links (all CC0, no licensing friction).
- Institutionalize the issue's "reuse only when matched exactly" rule the way
  WordNet CILI does: a concept is reused only on exact interlingual-id match,
  otherwise a new concept link is minted; store ILI ids (CC BY) and Wikidata
  Q-ids as alias links on concept links.
- Generalize `seed_common_concept_ontology` into an import surface that can
  load concept sets from LiNo files (S-5 format), keeping the 351-concept
  seed as the default.

**Reuse.** `src/concept_ontology.rs`, `src/lino_parser.rs`;
[`competitors-natural-language.md`](./competitors-natural-language.md)
"Exact-match shared concept space".

## S-10: Configurable translation-rule registry (R-15, R-17) - blocked by 04, 09

**Problem.** To-meta translation (parse) is total, but from-meta cross-language
output is gated on the hard-coded statehood proposition, and users cannot add
or replace translation rules.

**Plan.**
- Replace the hard-coded gate with a `TranslationRuleSet`: ordered, named
  rules mapping concept/structure patterns (LinkQuery-shaped left sides) to
  per-language syntax templates (concept-to-language mappings generalized from
  `insert_concept_mapping`).
- Make every stage user-replaceable: rule sets are values (loadable from LiNo
  via S-5, registrable like parsers via S-4's registry pattern), so end users
  get the issue's "full freedom of configuration".
- Quasiquote-style templates with placeholders (Babel `template`/GritQL
  precedent in
  [`competitors-code-tooling.md`](./competitors-code-tooling.md)) keep
  generated text aligned with grammar nodes rather than string concatenation.
- Acceptance: the statehood demo becomes one rule set among others, and at
  least one new translation pair runs entirely from user-supplied rules.

## S-11: Single-language restriction profiles (R-16) - blocked by 10

**Problem.** No way to restrict operations to "only features of meta language
that JavaScript supports exactly" for same-language (JS→JS) workflows.

**Plan.**
- Add `LanguageProfile` links: per-language capability sets naming which
  concepts, link types, and translation rules a target language supports
  (profiles are themselves links, hence queryable and user-editable).
- `ParseConfiguration::with_profile("JavaScript")` (or a transform-time
  profile argument) makes any operation that would leave the profile fail with
  a diagnostic link instead of producing untranslatable output.
- Profiles compose with S-10 rule sets: a profile is effectively the domain of
  the rule set, computed or declared.

## S-12: Query/transform algebra enrichment (R-11 extension, richest feature set)

**Problem.** `LinkQuery` covers type/term/language/named/S-expression/captures
and `ReplacementRule`/`SubstitutionRule` cover replace-and-substitute, but the
competitor survey shows a richer operator set users now expect.

**Plan** (operators sourced from
[`competitors-code-tooling.md`](./competitors-code-tooling.md) "Feature ideas"):
- Relational/composable rule algebra: `inside`, `has`, `precedes`, `follows`,
  `all/any/not`, and named reusable sub-rules (ast-grep).
- Ellipsis and typed metavariables: `...` gap matching and kind-constrained
  captures (Semgrep, Coccinelle), plus grammar-less fallback matching for
  unwired languages (Comby) — which meta-language can ground in its existing
  plain-text token links.
- Quasiquote replacement templates with placeholder safety and
  parenthesization-conservative reprinting (Babel template, Recast).
- Traversal-strategy combinators: `topdown`, `bottomup`, `innermost`,
  `fixpoint` (Stratego/Rascal) over links.
- A valid/invalid snapshot test harness for rules (ast-grep's YAML test idea)
  to make rule suites self-verifying.

**Reuse.** `src/query.rs`, `src/transform.rs`, `src/substitution.rs`.

## S-13: Incremental re-parse and structural sharing (richest feature set)

**Problem.** Every edit re-parses from scratch; competitors (tree-sitter
incremental parsing, Roslyn/rowan red-green trees, difftastic structural diff)
treat incrementality and sharing as core.

**Plan.**
- Expose tree-sitter's native incremental parsing (`InputEdit` + old tree)
  through the adapter: `LinkNetwork::apply_edit(range, new_text)` re-parses
  only affected regions and reuses untouched links (ids stable outside the
  edited span).
- Extend `NetworkSnapshot` structural sharing so an edited fork shares
  unchanged links with its parent (rowan/cstree precedent), and add a
  structural diff between two snapshots (difftastic precedent) returning
  changed-link sets — which also gives transforms cheap dry-run previews.

**Reuse.** tree-sitter 0.25 incremental API already in the dependency tree;
`src/snapshots.rs`.

## S-14: API-style parity contract (R-12, R-13, R-14) - blocked by 06, 10

**Problem.** Operations are unevenly reachable: queries have builder +
S-expression forms, substitutions only direct methods; nothing guarantees "all
operations in all styles".

**Plan.**
- Declare the operation inventory as data: an `API_OPERATIONS` registry
  (parse, query, transform, substitute, serialize, snapshot, translate,
  verify, ...) × style (direct method, fluent chain, link-cli substitution,
  S-expression/LiNo text) — the same registry-plus-gate pattern the crate
  already uses for languages and parity targets.
- Implement the missing styles: a fluent pipeline (`network.find(q).replace(r)
  .reconstruct()` — jscodeshift precedent) as a default-implemented extension
  trait over the storage trait from S-6 (sqlx/diesel/sea-orm layering: many
  styles, one executor), and link-cli-style text operations for each
  operation where applicable (link-cli 0.2.7 semantics: create `() ((1 1))`,
  delete `((1 1)) ()`, update same-index, read identity).
- Gate it: a unit test iterates `API_OPERATIONS` and asserts each operation
  has an executable fixture per style, so the matrix can never silently
  regress (this is the R-14 contract).

## S-15: Competitor corpora wave 2 + coverage gate (R-19, R-20) - blocked by 02, 08, 12, 14

**Problem.** 48 ported fixtures are a small fraction of upstream suites; the
llvm-cov job enforces no threshold; the roadmap still defers SQL dialects and
Delphi-specific coverage.

**Plan.**
- Port the five highest-value suites identified in
  [`competitors-code-tooling.md`](./competitors-code-tooling.md): Coccinelle
  `tests/` transform triples, tree-sitter `test/corpus` + `error_corpus`,
  Semgrep `tests/patterns/<lang>/`, srcML `test/parser/testsuite` round-trip
  cases, LibCST adversarial whitespace fixtures (runner-up: babel-parser
  fixtures) — each as provenanced `PARITY_FIXTURES`, sampled per construct
  rather than wholesale where suites are huge, with sampling documented.
- Add new `PARITY_TARGETS` for the surveyed competitors not yet tracked
  (ast-grep, Semgrep, Comby, GritQL, srcML, difftastic, Babel, SWC,
  OpenRewrite, Spoon, JavaParser, Rascal, Stratego/Spoofax, TXL, MPS,
  Coccinelle; plus GF, UD, LanguageTool from the natural-language survey, and
  doublets-rs/links-notation storage gates) so the "all competitors in all
  scopes" comparison is executable, not prose.
- Turn the existing `cargo llvm-cov` job into a ratcheted gate: record current
  line coverage, fail CI below the recorded floor, raise the floor with each
  wave toward the issue's 100% goal; audit `docs/parity-roadmap.md` so every
  deferral is either implemented or tracked by an open issue (R-19).

## Phasing

- **Phase 1 (parallel, no dependencies):** 01, 02, 03, 04, 05, 08, 09, 12, 13.
- **Phase 2:** 06 (after 01, 05), 07 (after 05), 10 (after 04, 09).
- **Phase 3:** 11 (after 10), 14 (after 06, 10).
- **Phase 4:** 15 (after 02, 08, 12, 14) — the closing audit/coverage gate.

Issue #47 instructs that all sub-issue work lands on the issue-47 branch and
merges through one pull request ([PR #48](https://github.com/link-foundation/meta-language/pull/48)).
