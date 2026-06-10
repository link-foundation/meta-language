# Case Study: Issue #47 - Compare all competitors in all scopes; richest feature set

## Summary

Issue [#47](https://github.com/link-foundation/meta-language/issues/47) asks
for a careful comparison with all competitors in all scopes and for the
richest feature set, spanning: readonly/mutable engine configuration; CST/AST
for all popular programming languages, data-exchange formats, other formal
languages, and natural languages with actual grammatical-correctness parsing;
an exact-match shared concept space; storage as links-notation text, binary
doublets, and Rust traits/types; equal operations across chaining, direct OOP,
and link-cli-style APIs; user-configurable to/from-meta-language translation
with single-language restriction profiles; nothing deferred in the
vision/roadmap; 100% test coverage copying competitor test cases; and
decomposition into GitHub sub-issues with blocked-by markings, all executed on
this issue's branch and merged through one pull request
([PR #48](https://github.com/link-foundation/meta-language/pull/48)).

This folder is the issue #47 case-study record: requirement extraction,
online competitor research across the three scopes (code tooling,
natural-language grammar systems and concept spaces, data formats and
links-ecosystem storage), per-requirement solution plans, and the source
specs for the filed implementation sub-issues.

Research and audit date: 2026-06-10.

## Key findings

- **Audit:** 22 requirements were extracted; 2 are Implemented, 15 Partial,
  5 Missing on the audit date. The largest gaps: no data-exchange format
  grammars, no binary doublets storage, no grammatical-correctness parsing
  for natural languages, no engine-level readonly configuration, no general
  network→LiNo serializer, cross-language translation gated on a hard-coded
  demo, and no API-style parity contract. See
  [`requirements.md`](./requirements.md).
- **Code tooling:** 22 competitor projects surveyed. Top capabilities to
  match: incremental re-parse and red/green structural sharing (tree-sitter,
  Roslyn, rowan), composable rule algebra and snapshot rule tests (ast-grep),
  ellipsis/typed metavariables (Semgrep, Coccinelle), quasiquote templates
  and conservative reprinting (Babel, Recast), traversal strategies
  (Stratego, Rascal), grammar-less fallback (Comby), structural diff
  (difftastic). See
  [`competitors-code-tooling.md`](./competitors-code-tooling.md).
- **Natural language:** Grammatical Framework + Resource Grammar Library is
  the only surveyed system doing exactly the issue's job - parse-or-reject
  grammaticality with no semantics, covering the ten target languages
  (LGPL/BSD). UD supplies the morphosyntax vocabulary and corpora; Wikidata
  lexemes (CC0) and WordNet CILI ids (CC BY) supply the exact-match concept
  discipline; LanguageTool/nlprule supply explainable negative checks;
  BabelNet is rejected on licensing. See
  [`competitors-natural-language.md`](./competitors-natural-language.md).
- **Formats and storage:** seven tree-sitter format grammars are
  drop-in-compatible with the project's tree-sitter 0.25.8 (JSON, YAML,
  TOML, XML, INI, protobuf, GraphQL); CSV and JSON5 need vendoring.
  `doublets` 0.4.0 is a viable binary backend (stable Rust, Unlicense,
  file-mapped persistence), and formal-ai - the heaviest planned user -
  already defines a `LinkStoreBackend { LinoProjection, DoubletsRs,
  DoubletsWeb }` stack to match. See
  [`formats-storage-apis.md`](./formats-storage-apis.md).

## Issue #47 requirements → deliverables

| Ask | Delivered by |
|---|---|
| Compare all competitors in all scopes | Three research documents + new parity targets planned in spec `#15` |
| List each and all requirements | [`requirements.md`](./requirements.md) (R-1 ... R-22) |
| Propose solutions and plans, checking existing components/libraries | [`solution-plans.md`](./solution-plans.md) (S-1 ... S-15, phased) |
| Compile data to `docs/case-studies/issue-47` | This folder, including [`raw-data/`](./raw-data/) |
| Create GitHub issues with blocked-by markings, as subtasks of #47 | [`proposed-issues/`](./proposed-issues/) specs, filed as sub-issues with native dependency links (table below) |
| Execute all tasks on this issue's branch, one big PR | Stated in every filed issue; branch `issue-47-76af108c0f24`, [PR #48](https://github.com/link-foundation/meta-language/pull/48) |

## Filed implementation sub-issues

The 15 specs were filed on 2026-06-10 with
`proposed-issues/create-issues.sh --create` as issues
[#49](https://github.com/link-foundation/meta-language/issues/49) through
[#63](https://github.com/link-foundation/meta-language/issues/63), attached as
GitHub sub-issues of #47 (`sub_issues` API), with blocked-by relationships
wired through the native issue-dependencies API. Specs remain the immutable
source.

| Spec | GitHub issue | Blocked by |
|---|---|---|
| `#01` Readonly/mutable engine configuration | [#49](https://github.com/link-foundation/meta-language/issues/49) | - |
| `#02` Data-exchange format grammars | [#50](https://github.com/link-foundation/meta-language/issues/50) | - |
| `#03` Programming-language grammar wave | [#51](https://github.com/link-foundation/meta-language/issues/51) | - |
| `#04` Pluggable language-parser registry | [#52](https://github.com/link-foundation/meta-language/issues/52) | - |
| `#05` LiNo network serialization | [#53](https://github.com/link-foundation/meta-language/issues/53) | - |
| `#06` Doublets binary storage backend | [#54](https://github.com/link-foundation/meta-language/issues/54) | #49, #53 |
| `#07` Rust types/traits ↔ links codec | [#55](https://github.com/link-foundation/meta-language/issues/55) | #53 |
| `#08` Natural-language grammar parsing | [#56](https://github.com/link-foundation/meta-language/issues/56) | - |
| `#09` Exact-match shared concept space | [#57](https://github.com/link-foundation/meta-language/issues/57) | - |
| `#10` Translation-rule registry | [#58](https://github.com/link-foundation/meta-language/issues/58) | #52, #57 |
| `#11` Language restriction profiles | [#59](https://github.com/link-foundation/meta-language/issues/59) | #58 |
| `#12` Query/transform algebra enrichment | [#60](https://github.com/link-foundation/meta-language/issues/60) | - |
| `#13` Incremental re-parse + structural sharing | [#61](https://github.com/link-foundation/meta-language/issues/61) | - |
| `#14` API-style parity contract | [#62](https://github.com/link-foundation/meta-language/issues/62) | #54, #58 |
| `#15` Competitor corpora wave 2 + coverage gate | [#63](https://github.com/link-foundation/meta-language/issues/63) | #50, #56, #60, #62 |

## Document index

| File | Purpose |
|---|---|
| [`requirements.md`](./requirements.md) | Traceable register of every issue #47 requirement with current-state evidence. |
| [`solution-plans.md`](./solution-plans.md) | One plan per requirement cluster with library reuse and phasing. |
| [`competitors-code-tooling.md`](./competitors-code-tooling.md) | 22-project source-code syntax tooling survey with test-suite locations. |
| [`competitors-natural-language.md`](./competitors-natural-language.md) | Natural-language grammar systems and shared concept spaces / interlinguas survey. |
| [`formats-storage-apis.md`](./formats-storage-apis.md) | Data-exchange format grammars, links-ecosystem storage targets, Rust API styles. |
| [`proposed-issues/`](./proposed-issues/) | Source specs for the filed sub-issues plus the idempotent filing script. |
| [`raw-data/`](./raw-data/) | Raw GitHub snapshots: issue #47, its comments, PR #48, all repo issues. |

## Status

Research, requirement extraction, solution planning, and sub-issue
specification are complete. Implementation proceeds through the filed
sub-issues in the phased order above, on branch `issue-47-76af108c0f24`,
merging through [PR #48](https://github.com/link-foundation/meta-language/pull/48).
