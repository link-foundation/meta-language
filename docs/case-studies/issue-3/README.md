# Case Study: Issue #3 - Top-10 languages and mixed-mode txt/Markdown/HTML

## Summary

Issue [#3](https://github.com/link-foundation/meta-language/issues/3) asked for
a deep case study and implementation plan for lossless in/out processing across
the top-10 programming languages, top-10 natural languages, and mixed-mode
txt/Markdown/HTML. It also asked that competitor and ecosystem test coverage be
tracked so users of similar tools have a clear migration path.

This folder is the issue #3 case-study record. The initial PR filed the plan and
created implementation issues [#5](https://github.com/link-foundation/meta-language/issues/5)
through [#20](https://github.com/link-foundation/meta-language/issues/20). After
the June 7 maintainer request to pull the latest default branch and continue the
implementation audit, this PR was updated onto `origin/main` after all 16 follow-up
issues were implemented and closed through PRs
[#21](https://github.com/link-foundation/meta-language/pull/21) through
[#36](https://github.com/link-foundation/meta-language/pull/36).

The current result is no longer planning-only: the crate now includes the
baseline runtime coverage and executable tests for every implementation issue
created from this case study.

## Implementation Status

| Issue | Area | Implemented by |
|---|---|---|
| [#5](https://github.com/link-foundation/meta-language/issues/5) | Plain-text (`txt`) container target | [PR #21](https://github.com/link-foundation/meta-language/pull/21) |
| [#6](https://github.com/link-foundation/meta-language/issues/6) | Natural-language ordering convention | [PR #22](https://github.com/link-foundation/meta-language/pull/22) |
| [#7](https://github.com/link-foundation/meta-language/issues/7) | tree-sitter lossless parser front end | [PR #23](https://github.com/link-foundation/meta-language/pull/23) |
| [#8](https://github.com/link-foundation/meta-language/issues/8) | Mixed-mode embedded grammar parsing | [PR #24](https://github.com/link-foundation/meta-language/pull/24) |
| [#9](https://github.com/link-foundation/meta-language/issues/9) | Natural-language segmentation and identification | [PR #25](https://github.com/link-foundation/meta-language/pull/25) |
| [#10](https://github.com/link-foundation/meta-language/issues/10) | Visual Basic grammar support | [PR #26](https://github.com/link-foundation/meta-language/pull/26) |
| [#11](https://github.com/link-foundation/meta-language/issues/11) | SQL baseline dialect support | [PR #27](https://github.com/link-foundation/meta-language/pull/27) |
| [#12](https://github.com/link-foundation/meta-language/issues/12) | Delphi/Object Pascal grammar support | [PR #28](https://github.com/link-foundation/meta-language/pull/28) |
| [#13](https://github.com/link-foundation/meta-language/issues/13) | Substitution/query matcher enrichment | [PR #29](https://github.com/link-foundation/meta-language/pull/29) |
| [#14](https://github.com/link-foundation/meta-language/issues/14) | Persistent immutable snapshots | [PR #30](https://github.com/link-foundation/meta-language/pull/30) |
| [#15](https://github.com/link-foundation/meta-language/issues/15) | Unified query + transform surface | [PR #31](https://github.com/link-foundation/meta-language/pull/31) |
| [#16](https://github.com/link-foundation/meta-language/issues/16) | Self-description roots as links | [PR #32](https://github.com/link-foundation/meta-language/pull/32) |
| [#17](https://github.com/link-foundation/meta-language/issues/17) | Common concept ontology | [PR #33](https://github.com/link-foundation/meta-language/pull/33) |
| [#18](https://github.com/link-foundation/meta-language/issues/18) | Cross-language reconstruction and formalization knobs | [PR #34](https://github.com/link-foundation/meta-language/pull/34) |
| [#19](https://github.com/link-foundation/meta-language/issues/19) | External competitor parity fixtures | [PR #35](https://github.com/link-foundation/meta-language/pull/35) |
| [#20](https://github.com/link-foundation/meta-language/issues/20) | Ecosystem parity fixtures | [PR #36](https://github.com/link-foundation/meta-language/pull/36) |

## Current Crate Capabilities

The merged implementation on `origin/main` now provides:

- `txt`, Markdown, and HTML as document-container targets.
- Tree-sitter-backed parsing for Python, C, Java, C++, C#, JavaScript, R,
  Visual Basic, `sql-ansi`, Delphi/Object Pascal, HTML, CSS, and Rust embedded
  regions.
- Byte-exact reconstruction from parsed links networks, including recovery cases.
- Mixed Markdown and HTML region detection with embedded JavaScript, CSS, HTML,
  SQL, Rust, and plain-text fallback regions connected into one network.
- Natural-language segmentation and identification for the ten target natural
  languages, including Mandarin dictionary segmentation, RTL/bidi annotations,
  and switchable `lingua`/`whatlang` identification.
- Enriched S-expression queries, capture predicates, substitution rules, and
  source-preserving replacement transforms.
- Immutable/mutable snapshots with structural sharing checks.
- Self-description roots, a 351-concept ontology seed, concept-to-language
  mappings, and cross-language reconstruction/formalization examples.
- Executable parity fixtures for tree-sitter, LibCST, Recast, jscodeshift,
  Rowan, cstree, Roslyn, links-notation, link-cli, lino-objects-codec,
  relative-meta-logic, formal-ai, and meta-expression.

The authoritative implementation matrix lives in
[`../../parity-roadmap.md`](../../parity-roadmap.md), and the requirement-level
audit is in [`requirements.md`](./requirements.md).

## Issue #3 Requirements

Issue #3's seven explicit asks are all addressed:

| ID | Ask | Current result |
|---|---|---|
| I3-1 | Create issues for full in/out coverage of the target programming languages, natural languages, and mixed-mode txt/Markdown/HTML | Issues #5-#20 were created and are now closed. |
| I3-2 | Copy competitor tests and match similar-project feature coverage | `PARITY_FIXTURES` and tests now cover external and ecosystem parity targets with provenance and executable gates. |
| I3-3 | Reference issue #1 for the vision | Vision requirements are extracted in `requirements.md` and cross-walked in `ecosystem-foundations.md`. |
| I3-4 | Compile data under `docs/case-studies/issue-3/` | Raw issue/PR snapshots and research documents are stored here. |
| I3-5 | Do deep case-study analysis with online facts | `online-research.md`, `rust-libraries-survey.md`, and competitor/ecosystem surveys record the research basis. |
| I3-6 | List every requirement | `requirements.md` maintains the traceable register. |
| I3-7 | Propose solution plans and check existing libraries | `solution-plans.md` and `rust-libraries-survey.md` record the phased plan that became issues #5-#20. |

## Document Index

| File | Purpose |
|---|---|
| [`requirements.md`](./requirements.md) | Current-state audit of every issue #3 and issue #1 requirement. |
| [`solution-plans.md`](./solution-plans.md) | Original solution plan and library reuse strategy used to file issues #5-#20. |
| [`rust-libraries-survey.md`](./rust-libraries-survey.md) | Survey of reusable Rust crates and grammar gaps. |
| [`competitor-test-suites.md`](./competitor-test-suites.md) | External competitor test-suite survey and adoption plan. |
| [`ecosystem-foundations.md`](./ecosystem-foundations.md) | link-foundation / linksplatform ecosystem parity and corrected figures. |
| [`online-research.md`](./online-research.md) | Ranking, license, and source fact-checks. |
| [`proposed-issues/`](./proposed-issues/) | Source specs used to create issues #5-#20. |
| [`raw-data/`](./raw-data/) | Raw GitHub issue and PR snapshots used by the case study. |

## Raw Data

Raw GitHub data is stored in [`raw-data/`](./raw-data/):

- `issue-3.json`, `issue-3-comments.json` - the planning issue and comments.
- `issue-1.json`, `issue-1-comments.json` - the founding vision issue and comments.
- `pr-2.json`, `pr-4.json` - the prior core PR and this case-study PR.
- `implementation-issues-5-20.json` - current snapshots of the 16 filed and
  closed implementation issues.
- `implementation-prs-21-36.json` - current snapshots of the merged PRs that
  implemented issues #5-#20.

## Status

Planning, issue creation, implementation follow-through, and audit are complete
for the scope decomposed from issue #3 into issues #5-#20. No remaining gap was
found against those closed issues' acceptance criteria during the default-branch
audit. Future work should extend coverage by adding more dialects, grammars, and
ported upstream fixtures under the existing registries and tests rather than by
reopening the original planning issue.
