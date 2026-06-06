# Case Study: Issue #3 — Plan full implementation for the top-10 programming and top-10 natural languages, plus mixed-mode txt/Markdown/HTML

## Summary

Issue [#3](https://github.com/link-foundation/meta-language/issues/3) is a **planning and research issue** (`documentation`, `enhancement`). It asks us to plan the work needed so this crate's Rust libraries can losslessly process **in and out** the top-10 programming languages and top-10 natural languages, with full support for **txt, Markdown, and HTML in mixed mode**, to "copy all the tests from competitors" so their users can migrate smoothly, and to compile the supporting data into this `docs/case-studies/issue-3/` folder. It builds directly on the founding vision in [issue #1](https://github.com/link-foundation/meta-language/issues/1).

The deliverable of this issue is **this case study folder**, not runtime code. It contains: the collected raw data, an online-research fact-check, three library/competitor surveys, a traceable requirements register, a solution plan per requirement, and the source specifications used to file 16 implementation issues ([#5](https://github.com/link-foundation/meta-language/issues/5) through [#20](https://github.com/link-foundation/meta-language/issues/20)) with an idempotent creation script. The pull request that adds this folder makes **documentation-only** changes; it does not modify `src/` runtime behaviour.

## The issue's seven requirements, at a glance

Issue #3's body decomposes into seven explicit asks (full register in [`requirements.md`](./requirements.md), Part A, IDs `I3-1`…`I3-7`):

1. **Create issues** so the Rust libraries cover in/out processing for the top-10 programming + top-10 natural languages and mixed-mode txt/Markdown/HTML. → filed issues [#5](https://github.com/link-foundation/meta-language/issues/5) through [#20](https://github.com/link-foundation/meta-language/issues/20), with source specs in [`proposed-issues/`](./proposed-issues/).
2. **Copy all competitor tests** and match every feature similar projects support, so their users can transition. → [`competitor-test-suites.md`](./competitor-test-suites.md) + implementation issues [#19](https://github.com/link-foundation/meta-language/issues/19)/[#20](https://github.com/link-foundation/meta-language/issues/20).
3. **Reference issue #1** for vision and initial implementation. → [`ecosystem-foundations.md`](./ecosystem-foundations.md) cross-walks the vision to the current code.
4. **Compile data** into `./docs/case-studies/issue-3`. → [`raw-data/`](./raw-data/) (verbatim GitHub JSON).
5. **Deep case-study analysis** with online fact-checking. → [`online-research.md`](./online-research.md).
6. **List each and every requirement.** → [`requirements.md`](./requirements.md) (Parts A–E, fully traceable).
7. **Propose solutions/plans per requirement**, checking existing components/libraries. → [`solution-plans.md`](./solution-plans.md) + [`rust-libraries-survey.md`](./rust-libraries-survey.md).

## What we found

### 1. The current implementation is a self-contained scaffold

The crate today is ~2,600 lines of Rust whose **only** dependency is `clap`. It ships its own LiNo-style lossless parser, the full public API surface (`LinkNetwork`, `NetworkProjection`, `verify_full_match`, `reconstruct_text`, `LinkQuery`, `SubstitutionRule`/`apply_substitution`, `NetworkSnapshot`, `TruthValue`, `EmbeddedRegion`, `ParseConfiguration`), and registry/fixture tracking gated by tests in [`docs/parity-roadmap.md`](../../parity-roadmap.md). There is **no tree-sitter, rowan/cstree, or doublets integration yet**. So the contract surface and the parity-tracking scaffolding exist; the outstanding work is real grammar integration, a populated concept ontology, and adopting competitor corpora. The requirements register encodes this precisely with a four-value status vocabulary (Tracked / API-scaffolded / Partial / Not started) so no item is over- or under-claimed.

### 2. There is a real coverage gap: `txt`

Issue #3's title and body explicitly require **"txt, markdown and html"**, but [`docs/parity-roadmap.md`](../../parity-roadmap.md)'s `MARKUP_LANGUAGE_TARGETS` lists only Markdown and HTML. Plain text is also the correct **fallback container** for mixed-mode error recovery (any unrecognized region degrades to a lossless `txt` span). This is the one untracked target the case study surfaces, and implementation issue [#5](https://github.com/link-foundation/meta-language/issues/5) closes it.

### 3. The library landscape favours reuse over reinvention

The full survey is in [`rust-libraries-survey.md`](./rust-libraries-survey.md); the headline is that **tree-sitter** can serve as the single universal lossless parser front-end, with official Rust grammar crates already covering 7 of the 10 TIOBE languages. The gaps and the recommended fillers:

| Need | Recommended reuse | Notes |
|---|---|---|
| Universal CST front-end | `tree-sitter` 0.26.x | Lossless, error-recovering, injection-capable (mixed mode) |
| 7 of 10 PLs | official `tree-sitter-{python,c,java,cpp,c-sharp,javascript,r}` | Already published |
| Visual Basic | (no official grammar) | Biggest gap — issue [#10](https://github.com/link-foundation/meta-language/issues/10) |
| SQL | `tree-sitter-sequel` | Dialect work — issue [#11](https://github.com/link-foundation/meta-language/issues/11) |
| Delphi/Object Pascal | generic `tree-sitter-pascal` | Needs Delphi validation — issue [#12](https://github.com/link-foundation/meta-language/issues/12) |
| Markdown | `pulldown-cmark` / `comrak` / `markdown-rs` | Source-span capable |
| HTML | `lol_html` (byte-preserving), `html5ever` | Lossless rewriting |
| NL segmentation | `unicode-segmentation`, `lindera` (CJK), `unicode-bidi`/`-normalization` | Grapheme/word/sentence + script handling |
| Language ID | `lingua` / `whatlang` | Mixed-language region detection |
| Storage substrate | `doublets` 0.4.0 (Unlicense) | The crate is `doublets`; `doublets-rs` is the repo |

The architecture reference (green/red persistent trees) comes from **rowan/cstree**; the storage model from **doublets**. All six external competitor suites (tree-sitter, LibCST, Recast, jscodeshift, rowan/cstree, Roslyn) are MIT or Apache-2.0/MIT — **license-compatible** with this repo's Unlicense, so their tests can be ported with provenance comments (see [`competitor-test-suites.md`](./competitor-test-suites.md) and the licensing-safety table in [`online-research.md`](./online-research.md)).

### 4. The founding figures needed correcting

Online fact-checking ([`online-research.md`](./online-research.md)) confirmed the repo's **TIOBE May 2026** programming ranking and reconciled the **Ethnologue 2025** natural-language ordering (the French/Arabic and Portuguese/Russian boundaries depend on the L1-vs-total-speaker metric — issue [#6](https://github.com/link-foundation/meta-language/issues/6) records the chosen convention). It also corrected several round-number estimates carried over from issue #1: the meta-expression lexicon is **351** concepts (not "328"); the per-language binding test count is **~138** (not "90+"); the formal-ai corpus should cite the actual `data/seed/*.lino` + `data/benchmarks/*.lino` files (not the unverified "706"); and the storage crate is **`doublets`** (the repo is `doublets-rs`).

## Recommended solution architecture

[`solution-plans.md`](./solution-plans.md) maps every requirement to a solution and a build-vs-reuse decision, then phases the work into six stages that map one-to-one onto the filed implementation issues:

- **Phase 0 — close tracking gaps (small, immediate):** add `txt` as a first-class container ([#5](https://github.com/link-foundation/meta-language/issues/5)); record the NL ordering convention ([#6](https://github.com/link-foundation/meta-language/issues/6)).
- **Phase 1 — the keystone:** tree-sitter adapter + the 7 official grammars ([#7](https://github.com/link-foundation/meta-language/issues/7)); mixed-mode embedding via injection ([#8](https://github.com/link-foundation/meta-language/issues/8)); NL segmentation + identification ([#9](https://github.com/link-foundation/meta-language/issues/9)).
- **Phase 2 — gap grammars (parallelizable):** Visual Basic ([#10](https://github.com/link-foundation/meta-language/issues/10)), SQL dialects ([#11](https://github.com/link-foundation/meta-language/issues/11)), Delphi ([#12](https://github.com/link-foundation/meta-language/issues/12)).
- **Phase 3 — transform & representation:** enrich the substitution/query matcher ([#13](https://github.com/link-foundation/meta-language/issues/13)); persistent snapshots ([#14](https://github.com/link-foundation/meta-language/issues/14)); unified query+transform surface ([#15](https://github.com/link-foundation/meta-language/issues/15)).
- **Phase 4 — semantics (deepest):** self-description roots ([#16](https://github.com/link-foundation/meta-language/issues/16)); common concept ontology from meta-expression ([#17](https://github.com/link-foundation/meta-language/issues/17)); cross-language reconstruction + formalization config ([#18](https://github.com/link-foundation/meta-language/issues/18)).
- **Phase 5 — corpus adoption (broad, ongoing):** external competitor suites ([#19](https://github.com/link-foundation/meta-language/issues/19)); ecosystem corpora ([#20](https://github.com/link-foundation/meta-language/issues/20)).

## Filed implementation issues

Issue #3 asks us to "create issues." The first draft kept the 16 issue bodies as source specs because creating public GitHub issues is an outward-facing action. After maintainer feedback on PR #4 explicitly requested creation, the specs were filed with `gh` on 2026-06-06 as issues [#5](https://github.com/link-foundation/meta-language/issues/5) through [#20](https://github.com/link-foundation/meta-language/issues/20).

```sh
cd docs/case-studies/issue-3/proposed-issues
./create-issues.sh            # preview (default — creates nothing)
./create-issues.sh --create   # file missing issues (idempotent: skips titles that already exist)
```

The script reads `title`/`labels` from each spec's frontmatter, rewrites relative doc links to absolute GitHub URLs, and only uses labels that already exist in the repo (`enhancement`, `documentation`). The full filed-issue index, phasing, and dependency table live in [`proposed-issues/README.md`](./proposed-issues/README.md).

## Document index

| File | What it is |
|---|---|
| [`requirements.md`](./requirements.md) | Traceable requirements register (Parts A–E): process asks, language/format coverage, vision capabilities, parity, and non-functional requirements, each with a current-state status. |
| [`solution-plans.md`](./solution-plans.md) | One solution per requirement (9 solutions), a build-vs-reuse summary, and the six-phase plan. |
| [`rust-libraries-survey.md`](./rust-libraries-survey.md) | Survey of reusable Rust crates (tree-sitter + grammars, Markdown/HTML, NL, storage) with versions and gap analysis. |
| [`competitor-test-suites.md`](./competitor-test-suites.md) | Survey of six external test suites: locations, formats, licenses, adaptation plans, and the four universal parity pillars. |
| [`ecosystem-foundations.md`](./ecosystem-foundations.md) | The link-foundation / linksplatform "internal competitors" — APIs, test shapes, and a corrections table for issue #1's figures. |
| [`online-research.md`](./online-research.md) | Fact-check: TIOBE May 2026, Ethnologue 2025 ordering nuances, license-safety table, and corrected founding figures. |
| [`proposed-issues/`](./proposed-issues/) | Source specs for filed issues #5–#20, issue-number index, and idempotent `create-issues.sh`. |
| [`raw-data/`](./raw-data/) | Verbatim GitHub JSON (see below). |

## Collected data

Raw GitHub data is stored verbatim in [`raw-data/`](./raw-data/):

- `issue-3.json`, `issue-3-comments.json` — the planning issue and its comments.
- `issue-1.json`, `issue-1-comments.json` — the founding vision issue (referenced by #3) and its comments, including the "try both when they can coexist; offer both with config when they conflict" guidance.
- `pr-2.json`, `pr-4.json` — the prior merged PR and this case study's own PR for context.
- `implementation-issues-5-20.json` — the 16 filed implementation issues created from [`proposed-issues/`](./proposed-issues/) on 2026-06-06.

## Status

**Planning complete — implementation tracked, not yet built.** This case study fully discharges issue #3's seven asks: the data is compiled, the analysis is fact-checked against external sources, every requirement is listed and traced, and a solution plan plus filed issues #5–#20 cover all of in/out processing for the top-10 programming and top-10 natural languages and mixed-mode txt/Markdown/HTML. The honest current-state finding is that the crate is a well-scoped scaffold: its public contract and parity-tracking exist, while real grammar integration, the concept ontology, and competitor-corpus adoption are the outstanding work — now enumerated, phased, and tracked as implementation issues.
