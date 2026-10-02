> This document is subordinate to the authoritative
> [vision and architecture specification](vision.md). It is the consumer
> matrix that the vision's Downstream consumers section refers to. Where this
> document and the vision disagree, the vision wins and this document must be
> corrected.

# Downstream consumers

This document maps what the two downstream consumers named in the vision
actually use or require from meta-language to meta-language capabilities,
rows of the issue 195 requirement ledger
([parity/issue-195-requirements.json](../parity/issue-195-requirements.json))
and tests in this repository. It records an audit. It is not a statement that
either consumer is served completely, and it does not mark any ledger row as
satisfied.

## Audit scope

- Audit date: 2026-09-29.
- relative-meta-logic (RML),
  <https://github.com/link-foundation/relative-meta-logic>, inspected at
  commit `c5513f2b958a8bd160b58c7054d408d4c446bf98` (committed 2026-07-12,
  default branch).
- formal-ai, <https://github.com/link-assistant/formal-ai>, inspected at
  commit `d209aac6461b355f1a527831202af3423135f7e6` (committed 2026-09-27,
  default branch). There is no `link-foundation/formal-ai` repository; the
  consumer lives under `link-assistant`.
- RML pull request
  [#184](https://github.com/link-foundation/relative-meta-logic/pull/184) was
  open at the audit date, head `f24d0f1f183fc92b39563d2f6fd5d3795d3235c5`
  on branch `issue-183-7fedfddffe9c`. Its implementation is audited separately
  below; the default-branch table describes the earlier committed consumer.
- meta-language itself was read at the working tree of this branch, whose
  JavaScript and Rust packages both declare version 0.58.2.

Every consumer link below points at a file at the inspected commit. Line
numbers refer to that commit.

## How Status is assigned

- **covered by tests**: a test in this repository exercises the
  meta-language capability that the consumer calls, in the named runtime.
  This is an in-repository test run. It is not a run of the consumer's own
  workload against a clean installed artifact.
- **not yet verified**: meta-language may provide the capability, but no test
  here exercises it the way the consumer uses it, or the consumer's own use
  was not run against the current meta-language during this audit.
- **not yet implemented**: meta-language does not provide the capability the
  consumer requires, or the consumer has not adopted it and the route it
  needs does not exist yet.

"none" in the Ledger rows column means that no ledger row covers the
capability. That is a gap in the ledger, not a claim that the capability is
out of scope.

## Dependency summary

| Consumer | JavaScript dependency | Rust dependency | Links Notation used directly |
|---|---|---|---|
| RML | `meta-language` `^0.46.0` in [js/package.json][rml-js-pkg] (L37) | `meta-language = "0.49.0"` in [rust/Cargo.toml][rml-cargo] (L26) | `links-notation` 0.13.0 in both runtimes |
| formal-ai | none; no npm dependency on meta-language | `meta-language` 0.58.2, optional but in the `default` feature set, in [rust/Cargo.toml][fai-cargo] (L53, L80, L83) | `links-notation` 0.16.1 declared; no direct calls in `rust/src` |

Both consumers depend on meta-language today. RML uses it as an overlay: a
facade of smoke checks next to its own Links Notation parser, evaluator and
proof kernel. formal-ai uses it from Rust as its only CST and AST engine and
has no JavaScript use of it.

## relative-meta-logic

| Consumer usage or requirement | meta-language capability | Ledger rows | Tests | Status |
|---|---|---|---|---|
| Parse RML text into a link network and reconstruct it byte for byte: `parseRmlToMetaLanguage` and `reconstructRmlFromMetaLanguage` in [js/src/rml-meta-language.mjs][rml-facade-js] (L42, L50), mirrored in [rust/src/meta_language_support.rs][rml-facade-rs] (L56, L60) | `LinkNetwork.parse` with the label `RML`, which is not a registered language and so falls back to the lossless text network (a Language link plus source tokens, no Syntax links); `reconstructText` / `reconstruct_text` | `I195-GRAMMAR-LOSSLESS-TREES` | `rust/tests/unit/parity_corpora.rs` (three RML-labelled fixtures, reconstruction checked in Rust only) | covered by tests |
| Parse RML through an RML dialect registered in meta-language's parser registry, requirement R4 in [docs/case-studies/issue-181/requirements.md][rml-reqs] (L54) | The native Links Notation grammar (label `LiNo`, Links Notation 0.22.0) exists and has a CST; no RML dialect on top of it is registered, so the `RML` label gets no syntax tree | `I195-CST-lino`, `I195-LINO-UPGRADE` | `js/tests/lino-grammar.test.js`, `rust/tests/unit/lino_grammar.rs` (the LiNo grammar only) | not yet implemented |
| Extract RML links after a meta-language round trip and compare parse and evaluator results: `parseRmlLinksViaMetaLanguage` and `rmlMetaLanguageParityReport` in [js/src/rml-meta-language.mjs][rml-facade-js] (L54, L58), Rust L64, L70 | Only the text round trip comes from meta-language; link extraction and evaluation are RML's own code on `links-notation` 0.13 | `I195-GRAMMAR-LOSSLESS-TREES` | none here; RML's own [js/tests/meta-language-support.test.mjs][rml-test-js] and [rust/tests/meta_language_support_tests.rs][rml-test-rs] | not yet verified |
| Rewrite a JavaScript identifier: `rewriteJavaScriptIdentifierViaMetaLanguage` in [js/src/rml-meta-language.mjs][rml-facade-js] (L85), Rust L97; a textual query `(identifier) @target (#eq? @target "from")` replaced with `ReplacementRule.capturedText` | `LinkQuery.fromSexpression`, `find` / `replace` and captured-text replacement over the JavaScript CST | `I195-XFORM-javascript-query`, `I195-XFORM-javascript-replace` | `js/tests/core.test.js`, `rust/tests/unit/query_transform.rs`, `rust/tests/unit/query_matching.rs` | covered by tests |
| The same rename made binding-safe (RML's rewrite renames every matching token, including shadowed and unrelated bindings) | Scope-aware binding rename | `I195-RENAME-javascript` | `js/tests/issue-195-binding-rename.test.js`, `rust/tests/unit/issue_195_binding_rename.rs` | not yet implemented |
| Substitution smoke: `metaLanguageSubstitutionSmoke` in [js/src/rml-meta-language.mjs][rml-facade-js] (L106), Rust L120 | `SubstitutionRule` and `applySubstitution` / `apply_substitution` | `I195-PARITY-TRANSFORMS` | `js/tests/core.test.js`, `rust/tests/unit/substitution.rs`, `rust/tests/unit/link_network.rs` | covered by tests |
| Rule-based rendering smoke: `renderMetaLanguageTranslationSmoke` in [js/src/rml-meta-language.mjs][rml-facade-js] (L125), Rust L137; a `LinkQuery.byType(SourceToken).withTerm('(')` rule with a fixed template | `TranslationRuleSet`, `TranslationRule` and `reconstructTextAsWithRules`; JavaScript `LinkType.Token` is an alias of `SourceToken`, Rust uses `LinkType::Token` | `I195-PARITY-TRANSLATIONS` | `js/tests/translation.test.js`, `js/tests/core.test.js`, `rust/tests/unit/translation_rules.rs` | covered by tests |
| Truth-value smoke: `metaLanguageTruthSmoke` in [js/src/rml-meta-language.mjs][rml-facade-js] (L137), Rust L150 | `TruthValue`, `Probability` and `ProbabilisticTruthValue` | `I195-DOWNSTREAM-RML-WORKLOADS` | `js/tests/core.test.js` (probabilistic truth values cover relative-meta-logic probability cases), `rust/tests/unit/link_network.rs` (`probabilistic_truth_values_cover_relative_meta_logic_probability_cases`) | covered by tests |
| Feature report of the meta-language exports RML relies on: `metaLanguageFeatureReport` in [js/src/rml-meta-language.mjs][rml-facade-js] (L147), Rust L162 | The exported API names above in both packages | `I195-PARITY-TRANSFORMS`, `I195-PARITY-TRANSLATIONS` | `js/tests/parity.test.js`, `rust/tests/unit/api_style_parity.rs` | covered by tests |
| All expression manipulation through meta-language query and rewrite, requirement R5 in [docs/case-studies/issue-181/requirements.md][rml-reqs] (L55); today `matchProofPattern` and `_applyTactic` in [js/src/rml-links.mjs][rml-links-js] (L2214, L3986) are hand-written | Structured query, replace, insert, delete, move and clone over link networks | `I195-XFORM-javascript-query`, `I195-XFORM-javascript-replace`, `I195-PARITY-TRANSFORMS` | `js/tests/issue-195-structured-transformations.test.js`, `rust/tests/unit/issue_195_structured_transformations.rs` (JavaScript, Rust, Lean and Rocq programs, not RML) | not yet implemented |
| Parse Links Notation directly and agree between runtimes: [js/src/rml-links.mjs][rml-links-js] (L17, `links-notation` `Parser`) and [rust/src/lib.rs][rml-lib-rs] (L335, `parse_lino_to_links`) | Native Links Notation 0.22.0 grammar in both runtimes, with the regression inputs RML reported upstream parsed in linear time and nesting beyond 64 levels becoming an error node | `I195-LINO-UPGRADE`, `I195-LINO-UPSTREAM-REGRESSIONS`, `I195-LINO-COMPATIBILITY-MATRIX` | `js/tests/lino-grammar-scaling.test.js`, `rust/tests/unit/lino_grammar_scaling.rs`, `js/tests/links-notation.test.js`, `rust/tests/unit/links_notation.rs` | covered by tests |
| RML's production Links Notation parsing routed through meta-language instead of `links-notation` 0.13 | Same grammar as the previous row; RML has not adopted it | `I195-LINO-UPGRADE` | none | not yet implemented |
| Token-level, byte-faithful CSTs for Rust, JavaScript, Lean and Rocq from issue 138: [js/src/cst-convert.mjs][rml-cst-js] and siblings, tested by [js/tests/cst.test.mjs][rml-cst-test] | Lossless CSTs for the four languages | `I195-CST-javascript`, `I195-CST-rust`, `I195-CST-lean`, `I195-CST-rocq`, `I195-GRAMMAR-LOSSLESS-TREES`, `I195-PARITY-CST` | `js/tests/default-cst-expectations.test.js`, `rust/tests/unit/default_cst_expectations.rs`, `js/tests/four-language-conformance.test.js`, `rust/tests/unit/four_language_conformance.rs` | covered by tests |
| Convergence of RML's own CST converters with meta-language's CSTs, requirement N4 in [docs/case-studies/issue-181/requirements.md][rml-reqs] (L75) | Same CSTs as the previous row; RML still maintains separate converters | `I195-PARITY-CST` | none | not yet implemented |
| Lean and Rocq export of RML theories: [js/src/lean-export.mjs][rml-lean-js], [js/src/rml-rocq.mjs][rml-rocq-js], [rust/src/lean_export.rs][rml-lean-rs], [rust/src/rocq.rs][rml-rocq-rs]; phase MX4 in [meta-language-integration.md][rml-mx] (L177) plans to route them through meta-language | Translation to Lean and Rocq exists only from JavaScript, Rust, Lean and Rocq programs; there is no translation from RML source | `I195-TRANSLATE-javascript-to-lean`, `I195-TRANSLATE-javascript-to-rocq`, `I195-SEMANTICS-PROOF-PRESERVATION` | `rust/tests/unit/translation_emit_lean.rs`, `rust/tests/unit/translation_emit_rocq.rs` (from JavaScript, not RML) | not yet implemented |
| JavaScript and Rust parity of everything RML uses, requirement N1 in [docs/case-studies/issue-181/requirements.md][rml-reqs] | Paired packages with parity checks | `I195-PARITY-CST`, `I195-PARITY-TRANSFORMS`, `I195-PARITY-TRANSLATIONS`, `I195-PARITY-DIAGNOSTICS` | `js/tests/parity.test.js`, `rust/tests/unit/api_style_parity.rs` | covered by tests |
| RML's Rust facade built against the current crate: RML pins `meta-language = "0.49.0"` | Crate 0.58.2 on crates.io | `I195-DELIVERY-CRATE-CANDIDATE`, `I195-DELIVERY-CRATE-PUBLISHED` | `js/tests/issue-195-delivery.test.js` (checks this repository's packaging, not RML) | not yet verified |
| RML's JavaScript facade on the current package: RML declares `^0.46.0` | npm `latest` is 0.46.0; the 0.58.2 JavaScript package is not published to npm | `I195-DELIVERY-NPM-CANDIDATE`, `I195-DELIVERY-NPM-PUBLISHED` | `js/tests/issue-195-delivery.test.js` (checks this repository's packaging, not RML) | not yet verified |
| RML workloads run against clean installed meta-language artifacts | Candidate and published packages consumed by RML's own test suites | `I195-DOWNSTREAM-RML-WORKLOADS`, `I195-DELIVERY-RML-CANDIDATE`, `I195-DELIVERY-RML-PUBLISHED` | `js/tests/issue-195-downstream-rml-workloads.test.js`, `rust/tests/unit/issue_195_downstream_rml_workloads.rs` (validate the report of the CI job that runs RML's workload tests on the candidates) | not yet verified |

[rml-js-pkg]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/package.json
[rml-cargo]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/Cargo.toml
[rml-facade-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/src/rml-meta-language.mjs
[rml-facade-rs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/src/meta_language_support.rs
[rml-test-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/tests/meta-language-support.test.mjs
[rml-test-rs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/tests/meta_language_support_tests.rs
[rml-reqs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/docs/case-studies/issue-181/requirements.md
[rml-mx]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/docs/case-studies/issue-181/meta-language-integration.md
[rml-links-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/src/rml-links.mjs
[rml-lib-rs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/src/lib.rs
[rml-cst-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/src/cst-convert.mjs
[rml-cst-test]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/tests/cst.test.mjs
[rml-lean-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/src/lean-export.mjs
[rml-rocq-js]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/js/src/rml-rocq.mjs
[rml-lean-rs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/src/lean_export.rs
[rml-rocq-rs]: https://github.com/link-foundation/relative-meta-logic/blob/c5513f2b958a8bd160b58c7054d408d4c446bf98/rust/src/rocq.rs

### Current RML pull request 184

This table inspects the pinned pull request head
`f24d0f1f183fc92b39563d2f6fd5d3795d3235c5`, rather than treating its
new code as part of the older default branch. Its JavaScript package still
pins meta-language `^0.46.0` and its Rust crate pins 0.58.2. On 2026-09-29,
a packed JavaScript 0.58.2 candidate from this branch passed all 145 targeted
consumer tests covering the meta-language facade, LiNo front end and formal
workspace after the source rendering correction below. The complete RML
JavaScript suite also passed (1,506 tests). Rust candidate integration has not
yet been recorded as an issue 195 acceptance observation. These direct
consumer test runs are not observations of the issue 195 acceptance suite.

The workloads below are every file of that head that reaches meta-language:
the sources that import the package or a source that does, and the tests that
import such a source. [`parity/fixtures/rml-pr184-workloads.json`](../parity/fixtures/rml-pr184-workloads.json)
records them with their blob ids, and `node js/scripts/issue-195-rml-pr184.mjs
--online` derives them again from the pull request on GitHub, so a new head or
a new consuming file fails the audit until this section is updated.

| Consumer usage or requirement | meta-language capability | Ledger rows | Tests | Status |
|---|---|---|---|---|
| Parse and normalize RML's Links Notation source in both runtimes, including comments, indentation, Unicode space, quote runs, depth and scaling cases | [JavaScript front end][rml-pr184-lino-js], [Rust front end][rml-pr184-lino-rs] and [shared cases][rml-pr184-lino-cases] still call `links-notation` 0.20 after consumer-owned preprocessing. This pull request does not call meta-language's LiNo grammar. | `I195-CST-lino`, `I195-LINO-UPGRADE`, `I195-LINO-UPSTREAM-REGRESSIONS`, `I195-LINO-COMPATIBILITY-MATRIX`, `I195-DOWNSTREAM-RML-PR184-AUDIT` | `js/tests/lino-grammar.test.js`, `rust/tests/unit/lino_grammar.rs` (meta-language only) | not yet verified |
| Round-trip RML source through meta-language and compare links and evaluation | [JavaScript facade][rml-pr184-meta-js] and [Rust facade][rml-pr184-meta-rs] parse with the unregistered `RML` label, reconstruct tokens, then call RML's own LiNo parser and evaluator. Neither facade tests a registered RML grammar. | `I195-GRAMMAR-LOSSLESS-TREES`, `I195-DOWNSTREAM-RML-WORKLOADS` | `rust/tests/unit/parity_corpora.rs` (meta-language only) | not yet verified |
| Rewrite a JavaScript identifier as part of RML's meta-language integration | The [JavaScript facade][rml-pr184-meta-js] reconstructs a meta-language network, then uses RML's own `parseJs` and `printJs` CST for lexical edits; the [Rust facade][rml-pr184-meta-rs] uses meta-language query and replacement. The two implementations are not the same binding-aware operation. | `I195-XFORM-javascript-query`, `I195-XFORM-javascript-replace`, `I195-RENAME-javascript`, `I195-PARITY-TRANSFORMS` | `js/tests/issue-195-binding-rename.test.js`, `rust/tests/unit/issue_195_binding_rename.rs` (meta-language only) | not yet implemented |
| Load and validate linked Lean and Rocq formal corpus content | [JavaScript corpus][rml-pr184-corpus-js] and [Rust corpus][rml-pr184-corpus-rs] first round-trip the source through the `RML` network, then use RML's own form parser and trusted contract to interpret and fingerprint declarations. Meta-language does not validate declarations or certify proofs. | `I195-DOWNSTREAM-RML-WORKLOADS`, `I195-GRAMMAR-LOSSLESS-TREES`, `I195-SEMANTICS-PROOF-PRESERVATION` | [JavaScript consumer tests][rml-pr184-theory-js], [Rust consumer tests][rml-pr184-theory-rs]; none here | not yet verified |
| Read an executable meta-theory network, including link-defined logics, from RML source in both runtimes | The [JavaScript network][rml-pr184-network-js] and [Rust network][rml-pr184-network-rs] round-trip each theory source through the meta-language facade before RML's own form parser, proof-object checker and linked-program registry interpret it. Meta-language only carries the source. | `I195-DOWNSTREAM-RML-WORKLOADS`, `I195-GRAMMAR-LOSSLESS-TREES` | [JavaScript consumer tests][rml-pr184-theory-js], [linked-theory tests][rml-pr184-theory-linked-js], [Rust consumer tests][rml-pr184-theory-rs]; none here | not yet verified |
| Use meta-language substitution rules, translation rules and truth values from RML planning | The [JavaScript facade][rml-pr184-meta-js] and [Rust facade][rml-pr184-meta-rs] apply a `SubstitutionRule`, render through a `TranslationRuleSet` and combine `TruthValue` and `ProbabilisticTruthValue`; the [JavaScript facade tests][rml-pr184-support-js] and [Rust facade tests][rml-pr184-support-rs] check the results. | `I195-PARITY-SEMANTICS`, `I195-PARITY-TRANSFORMS`, `I195-DOWNSTREAM-RML-WORKLOADS` | `js/tests/core.test.js`, `js/tests/translation.test.js`, `rust/tests/unit/substitution.rs`, `rust/tests/unit/translation_rules.rs`, `rust/tests/unit/link_network.rs` (meta-language only) | not yet verified |
| Keep JavaScript and Rust RML behavior aligned across the pull request's tests | [JavaScript LiNo tests][rml-pr184-tests-js], [Rust LiNo tests][rml-pr184-tests-rs], and facade tests in both runtimes exercise consumer code. The 145 targeted tests and complete JavaScript suite (1,506 tests) passed on a packed candidate; the Rust candidate and full acceptance matrix remain unverified. | `I195-PARITY-CST`, `I195-PARITY-TRANSFORMS`, `I195-DOWNSTREAM-RML-WORKLOADS`, `I195-DELIVERY-RML-CANDIDATE` | `js/tests/core.test.js`, `rust/tests/unit/source_generation.rs` (source rendering parity) | not yet verified |

[rml-pr184-lino-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/src/rml-lino-frontend.mjs
[rml-pr184-lino-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/src/lino_frontend.rs
[rml-pr184-lino-cases]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/test-corpus/lino-frontend/cases.json
[rml-pr184-meta-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/src/rml-meta-language.mjs
[rml-pr184-meta-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/src/meta_language_support.rs
[rml-pr184-corpus-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/src/rml-formal-corpus.mjs
[rml-pr184-corpus-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/src/formal_corpus.rs
[rml-pr184-tests-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/tests/lino-frontend.test.mjs
[rml-pr184-tests-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/tests/lino_frontend_tests.rs
[rml-pr184-theory-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/tests/theory-network.test.mjs
[rml-pr184-theory-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/tests/theory_network_tests.rs
[rml-pr184-theory-linked-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/tests/theory-network-linked.test.mjs
[rml-pr184-network-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/src/rml-theory-network.mjs
[rml-pr184-network-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/src/theory_network.rs
[rml-pr184-support-js]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/js/tests/meta-language-support.test.mjs
[rml-pr184-support-rs]: https://github.com/link-foundation/relative-meta-logic/blob/f24d0f1f183fc92b39563d2f6fd5d3795d3235c5/rust/tests/meta_language_support_tests.rs

## formal-ai

| Consumer usage or requirement | meta-language capability | Ledger rows | Tests | Status |
|---|---|---|---|---|
| Validate generated programs: `parse_with_meta_language` in [rust/src/coding/cst.rs][fai-cst] (L151) checks full match, Syntax link counts and text preservation for the languages listed in [data/seed/program-cst-grammars.lino][fai-grammars] | `LinkNetwork::parse`, `verify_full_match`, `projected_links` and `reconstruct_text` for JavaScript, Python, Rust, Java, C#, C, C++, TypeScript, Go, Ruby and PHP | `I195-CST-javascript`, `I195-CST-python`, `I195-CST-rust`, `I195-CST-java`, `I195-CST-c-sharp`, `I195-CST-c`, `I195-CST-c-plus-plus`, `I195-CST-typescript`, `I195-CST-go`, `I195-CST-ruby`, `I195-CST-php`, `I195-GRAMMAR-LOSSLESS-TREES` | `rust/tests/unit/default_cst_expectations.rs`, `rust/tests/unit/grammar_parsing.rs` | covered by tests |
| Formalize source and data files up to 32 KiB, including fenced code inside Markdown: [rust/src/summarization/file.rs][fai-file] (L27, L411, L419) | CSTs for code labels and for JSON, YAML, TOML, HTML, CSS, XML and INI; embedded regions for fenced code | `I195-CST-json`, `I195-CST-yaml`, `I195-CST-toml`, `I195-CST-html`, `I195-CST-css`, `I195-CST-xml`, `I195-CST-ini`, `I195-EMBED-markdown-language-selected-fenced-code` | `rust/tests/unit/default_cst_expectations.rs`, `rust/tests/unit/embedded_regions.rs` | covered by tests |
| Parse large files without superlinear time; formal-ai reported the quadratic `point_at_byte` as meta-language issue 193 and records it in [dev/log/issues/1017/pulls/1018/README.md][fai-1018] | Linear-time position lookup during parsing | `I195-GRAMMAR-LOSSLESS-TREES` | `rust/tests/unit/parse_scaling.rs` | covered by tests |
| Convert documents between plain text, Markdown, HTML, PDF and DOCX: `convert_document_format` in [rust/src/document_formats.rs][fai-docs] (L172) using `CROSS_FORMAT_CONCEPTS`, `parse_markup_document`, `render_docx_package` and `reconstruct_text_as` | Document formats and cross-format reconstruction | `I195-CST-txt`, `I195-CST-markdown`, `I195-CST-html`, `I195-CST-pdf`, `I195-CST-docx` | `rust/tests/unit/cross_format_reconstruction.rs`, `rust/tests/unit/docx_document.rs`, `rust/tests/unit/pdf_document.rs`, `rust/tests/unit/document_formatting.rs` | covered by tests |
| Check memory queries written in GraphQL and SQL for exact syntax: labels `GraphQL` and `sql-ansi` and `validate_exact_syntax` in [rust/src/memory_query_language/mod.rs][fai-mql] (L711) | GraphQL and ANSI SQL CSTs with full-match verification and reconstruction | `I195-CST-graphql`, `I195-CST-sql-ansi` | `rust/tests/unit/graphql_adapter.rs`, `rust/tests/unit/query_plan.rs`, `rust/tests/unit/default_cst_expectations.rs` | covered by tests |
| Edit Rust and JavaScript through links: `apply_link_edit` in [rust/src/agentic_coding/link_edit_rules.rs][fai-edit] (L180) with InsertMember, ReplaceLiteral and RenameIdentifier rules, tested by formal-ai's [rust/tests/unit/issue_1085_link_edit_rules.rs][fai-edit-test] | `apply_edit` over a `ByteRange` followed by `verify_full_match` | `I195-XFORM-rust-insert`, `I195-XFORM-rust-replace`, `I195-XFORM-javascript-insert`, `I195-XFORM-javascript-replace` | `rust/tests/unit/link_network.rs`, `rust/tests/unit/issue_195_structured_transformations.rs` | covered by tests |
| RenameIdentifier made binding-safe (formal-ai's rule renames whole tokens, not bindings) | Scope-aware binding rename | `I195-RENAME-rust`, `I195-RENAME-javascript` | `rust/tests/unit/issue_195_binding_rename.rs`, `rust/tests/unit/issue_195_binding_rename_corpus.rs` | not yet implemented |
| Project programs between Rust, JavaScript and TypeScript with rule sets written in Links Notation: `project` in [rust/src/rust_projection.rs][fai-proj] (L65) using `TranslationRuleSet::from_lino`, `LinkNetwork::from_lino` and `query_matches`, tested by formal-ai's [issue_1138_rust_projection.rs][fai-proj-test] | Rule sets and networks serialized to and from Links Notation; query matching | `I195-PARITY-TRANSLATIONS` | `rust/tests/unit/translation_rules.rs` (`translation_rule_sets_round_trip_through_lino`), `rust/tests/unit/lino_serialization.rs`, `rust/tests/unit/query_matching.rs` | covered by tests |
| formal-ai's own projection rule sets run against the current crate | Same API as the previous row | `I195-PARITY-TRANSLATIONS` | none | not yet verified |
| Translation between Rust and JavaScript in both directions, required by [the three-roots architect note][fai-three-roots] | Rust to JavaScript and JavaScript to Rust translation; formal-ai does not call it at the inspected commit | `I195-TRANSLATE-rust-to-javascript`, `I195-TRANSLATE-javascript-to-rust`, `I195-SEMANTICS-FAITHFUL-BEHAVIOR` | `rust/tests/unit/issue_195_translation_pairs.rs`, `rust/tests/unit/issue_195_translation_behavior.rs`, `rust/tests/unit/translation_emit_javascript.rs`, `rust/tests/unit/translation_emit_rust.rs` | not yet verified |
| Translation to and from TypeScript, required by the same [architect note][fai-three-roots] (L10) | TypeScript has a CST but is not a translation pair in meta-language | `I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS`, `I195-DOWNSTREAM-FORMAL-AI-WORKLOADS` | none | not yet implemented |
| Census and serialization of formal-ai's own Rust syntax: `ast_census` and `network_lino` in [rust/src/agentic_coding/self_ast.rs][fai-self-ast] (L148, L270) and `parse_network` in [rust/src/grammar_kinds.rs][fai-kinds] (L167) | `to_lino` / `from_lino` and grammar node kinds over the Rust CST | `I195-CST-rust`, `I195-GRAMMAR-CONCEPT-DISTINCTIONS` | `rust/tests/unit/lino_serialization.rs`, `rust/tests/unit/default_cst_expectations.rs` | covered by tests |
| formal-ai's Links Notation seed and benchmark data parse and reconstruct losslessly | LiNo CST; formal-ai fixtures from `data/seed/` and `data/benchmarks/` in the parity corpora | `I195-CST-lino` | `rust/tests/unit/parity_corpora.rs` (Rust only) | covered by tests |
| A JavaScript use of meta-language inside formal-ai | None exists at the inspected commit; formal-ai's [package.json][fai-pkg] has no meta-language dependency | `I195-DOWNSTREAM-FORMAL-AI-WORKLOADS`, `I195-DOWNSTREAM-FORMAL-AI-PUBLISHED` | none | not yet implemented |
| formal-ai built against the current crate: it declares 0.58.2 | Crate 0.58.2 on crates.io | `I195-DELIVERY-CRATE-CANDIDATE`, `I195-DELIVERY-CRATE-PUBLISHED` | `js/tests/issue-195-delivery.test.js` (checks this repository's packaging, not formal-ai) | not yet verified |
| formal-ai workloads run against clean installed meta-language artifacts, candidate and published | Candidate and published crate consumed by formal-ai's own test suites | `I195-DOWNSTREAM-FORMAL-AI-WORKLOADS`, `I195-DOWNSTREAM-FORMAL-AI-PUBLISHED` | none | not yet verified |

[fai-cargo]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/Cargo.toml
[fai-pkg]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/package.json
[fai-cst]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/coding/cst.rs
[fai-grammars]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/data/seed/program-cst-grammars.lino
[fai-file]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/summarization/file.rs
[fai-1018]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/dev/log/issues/1017/pulls/1018/README.md
[fai-docs]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/document_formats.rs
[fai-mql]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/memory_query_language/mod.rs
[fai-edit]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/agentic_coding/link_edit_rules.rs
[fai-edit-test]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/tests/unit/issue_1085_link_edit_rules.rs
[fai-proj]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/rust_projection.rs
[fai-proj-test]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/tests/unit/issue_1138_rust_projection.rs
[fai-three-roots]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/docs/architect-notes/2026-09-24-three-roots-full-parity-via-the-meta-language.md
[fai-self-ast]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/agentic_coding/self_ast.rs
[fai-kinds]: https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/grammar_kinds.rs

## Facts recorded during the audit

- npm and crates.io are not in step. The npm `latest` dist-tag for
  `meta-language` is 0.46.0, while crates.io and both package manifests in
  this repository are at 0.58.2. RML's JavaScript range `^0.46.0` therefore
  resolves to 0.46.0.
- RML's facade tests in `js/tests/meta-language-support.test.mjs` (seven
  tests) passed on meta-language 0.46.0 and also passed when pointed at this
  branch's JavaScript sources through a scratch link. The second run is not a
  clean installed artifact and does not satisfy any delivery or downstream
  row. In the current RML pull request, a packed 0.58.2 JavaScript candidate
  initially failed two of 145 targeted tests because `renderSource('RML')`
  inserted recovery `)` tokens in otherwise lossless source. Both cases pass
  after the fix, as do all 145 targeted tests. The Rust candidate workload
  has not yet been recorded as an acceptance observation.
- RML parses with the label `RML`, which is not registered. Both runtimes
  then fall back to the lossless text network; `rust/src/parser_registry.rs`
  documents the fallback. A JavaScript probe during the audit returned a
  Language link and one source token link per character, with no Syntax
  links. The `LiNo` label gives the full Links Notation 0.22.0 CST.
- RML's issue 181 records R4 (an RML dialect in the parser registry), R5 (all
  manipulation through meta-language query and rewrite) and R6 (a strategy
  library) as missing, and R1 to R3 as an overlay or partial.
- RML filed meta-language issues 163, 165, 166, 171, 172 and 173. All were
  closed at the audit date. Issue 171 asked for lockstep npm and crate
  versions; the registries still differ.
- RML pull request 184 adds one Links Notation front end
  (`js/src/rml-lino-frontend.mjs`, `rust/src/lino_frontend.rs`) on
  `links-notation` 0.20 and reported Links Notation issues 312 to 316: runtime
  disagreement on lone carriage returns, Unicode spaces and singlets; lost
  names on indented identifier lines; exponential nested groups and quadratic
  time in Rust `parse_lino_to_links`; a Rust stack overflow with no 64-level
  depth limit; and quadratic quote runs. All five were closed at the audit
  date. meta-language's own Links Notation 0.22.0 grammar is tested against
  these inputs (see the RML table), but RML does not use it.
- formal-ai reported the quadratic `point_at_byte` (meta-language issue 193)
  and the docs.rs build problem caused by `lindera` (meta-language issue 181);
  both were closed. Its SQL and GraphQL semantic proposals are meta-language
  issues 187 and 188, also closed.
- formal-ai's `REQUIREMENTS.md` rows R347, R381, R1085-3 and R890-2 mention
  meta-language. Row R526-3 uses "meta-language" for formal-ai's internal
  meaning representation of natural language and does not refer to this
  project.

## What remains unverified

- The targeted and full RML JavaScript suites passed against a clean packed
  candidate, but Rust candidate and published workloads and all formal-ai candidate
  and published workloads have not been recorded by the acceptance runner.
  `I195-DOWNSTREAM-RML-WORKLOADS`,
  `I195-DOWNSTREAM-FORMAL-AI-WORKLOADS` and
  `I195-DOWNSTREAM-FORMAL-AI-PUBLISHED` remain unverified.
- `I195-DELIVERY-RML-CANDIDATE`, `I195-DELIVERY-RML-PUBLISHED`,
  `I195-DELIVERY-NPM-PUBLISHED` and `I195-DELIVERY-CRATE-PUBLISHED` are not
  shown by this document.
- `I195-DOWNSTREAM-CONSUMER-MATRIX` asks for a check that fails when a row or
  test named here does not exist. This document supplies the matrix. No such
  check exists yet, so that row is not satisfied by this document alone.
- A "covered by tests" status only says that this repository tests the
  capability the consumer calls. It does not say the consumer's own tests
  pass on the current release.

## Boundary with relative-meta-logic

meta-language does not take over RML's foundations or proof authority. RML's
kernel, its many-valued and probabilistic semantics, its proof checking and
its certified tactics stay in RML, as RML's requirement N6 states: soundness
must not be weakened and strategies must reuse the existing certified kernel
steps. meta-language supplies parsing, lossless representation, query,
rewrite and translation machinery that RML may call. A proof result counts
because RML's kernel accepts it, not because meta-language produced or
translated it. The same applies to formal-ai: meta-language is a syntax and
translation engine there and does not decide what formal-ai accepts as true.
