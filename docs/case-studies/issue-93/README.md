# Case Study: Issue #93 — Easy grammar extensibility and grammar inference

## Summary

Issue [#93](https://github.com/link-foundation/meta-language/issues/93) asks to
make it **easy to develop new grammars that inherit from the meta-language**, and
to **infer a grammar from example texts** — ideally reconstructing a programming
language's grammar from *only positive examples* of correct programs. It further
requires:

- using [`link-foundation/meta-notation`](https://github.com/link-foundation/meta-notation)
  as the basis (the meta-language is *inherited from* meta-notation), maximising
  the freedom of grammar inference;
- searching online for the **best grammar-inference papers** and grounding the
  design in them;
- a **working Rust implementation** that generates a grammar description *in the
  meta-language itself*, then **translates it to Rust, JavaScript, and other
  languages**, and also **parses PEG, BNF, and other grammar languages *as*
  meta-language**;
- leveraging **shared concepts across all (formal and natural) languages** for
  near 1-to-1 translation;
- planning **as many issues as possible** so the system fully implements
  everything and **beats all competitors in all metrics**;
- collecting the data into `docs/case-studies/issue-93/` and doing a **deep
  case-study analysis** (with online research): list every requirement, propose a
  solution plan per requirement, and check existing components/libraries;
- making every planned issue a **sub-issue of #93** with **"blocked by"
  relationships** configured via the `gh` tool, each issue **maximally detailed**
  so even a weak AI agent can complete it;
- executing the planning inside the single existing pull request
  [PR #94](https://github.com/link-foundation/meta-language/pull/94) on branch
  `issue-93-8ae76e61befe`.

This folder is the issue #93 case-study record: the raw issue/PR evidence, the
requirement register, the grammar-inference literature review, the
library/ecosystem survey, the existing-capability gap analysis, the competitive
analysis that operationalises "beat all competitors," the per-requirement
solution plans with the issue dependency DAG, and the full set of
maximally-detailed source specs for the 34 sub-issues to be filed under #93.

Investigation date: 2026-06-19.

## Key findings

- **This is a planning + research issue, not a bug.** The deliverable is a
  complete, dependency-ordered backlog of maximally-detailed sub-issues (Q-1,
  Q-9), grounded in a deep case study (Q-3) — not a code change to ship the
  feature now. The feature itself is a large greenfield subsystem.
- **Gold (1967) makes the central design choice for us.** Positive-only inference
  of regular/context-free languages is impossible *without an inductive bias*.
  meta-language already owns the two biases the current SOTA (NatGI 2025) had to
  bolt on: meta-notation's **delimiter skeleton** (the "bracket-guided" prior)
  and the **351-concept shared ontology** (the source of natural non-terminal
  names). See [`literature-review.md`](./literature-review.md).
- **The metrics ladder to beat is published.** GLADE → Arvada → TreeVada →
  Kedavra → **NatGI (SOTA, F1 ≈ 0.57)**. The plan reproduces the SOTA pipeline on
  a substrate that supplies its strongest priors for free, then wins the
  secondary metrics no competitor contests (cross-format import/emit, parser
  codegen, GBNF, cross-language translation). See
  [`competitive-analysis.md`](./competitive-analysis.md).
- **Excellent reuse seams already exist.** `LinkType::Grammar`, the concept
  ontology, `TranslationRule`/`rust_codec`, `ParserRegistry`/`LanguageParser`,
  and ~30 tree-sitter grammars (as golden CST oracles) mean almost every new
  component attaches to a tested boundary. The **Grammar IR (A1) is the
  keystone**. See [`existing-capabilities.md`](./existing-capabilities.md).
- **Licence hygiene is settled.** Port MIT/Apache work (TreeVada, Arvada, GLADE,
  LearnLib, Sequitur, GIToolbox); depend on `bnf`/`ebnf`/`abnf`/`pest_meta`;
  clean-room GPL work (ISLa/ISLearn, flexfringe); study-only CC-NC/no-licence work
  (Mimid, REINAM). See [`library-survey.md`](./library-survey.md).
- **Decomposition: 34 sub-issues across 6 epics** (A foundation, B import,
  C export/codegen, D inference, E tooling/benchmarks, F docs), with a dependency
  DAG rooted at A1. See [`solution-plans.md`](./solution-plans.md) and
  [`proposed-issues/`](./proposed-issues/).

## Document index

| Document | Purpose | Requirement |
|---|---|---|
| [`requirements.md`](./requirements.md) | Every requirement (P-1…P-12, Q-1…Q-9) extracted verbatim-in-intent, with source quotes and "addressed by" mapping. | Q-4 |
| [`literature-review.md`](./literature-review.md) | Best grammar-inference papers by family, with portability/licence notes and the design through-line. | P-6, Q-3 |
| [`library-survey.md`](./library-survey.md) | 50+ existing components/libraries, licence-vetted, with port/depend/avoid recommendations. | Q-6 |
| [`existing-capabilities.md`](./existing-capabilities.md) | What the crate already provides, the meta-notation lineage, and the gap analysis mapping to issues. | Q-3 |
| [`competitive-analysis.md`](./competitive-analysis.md) | The competitor landscape, pinned metrics, and the concrete bar for "beat all competitors" (P-12). | P-12, Q-3 |
| [`solution-plans.md`](./solution-plans.md) | Per-epic solution plans, the complete 34-issue list, the dependency DAG, and release sequencing. | Q-1, Q-5 |
| [`proposed-issues/`](./proposed-issues/) | The 34 maximally-detailed source specs (one file per sub-issue) + the index mapping spec → filed issue #. | Q-1, Q-7, Q-8, Q-9 |
| [`raw-data/`](./raw-data/) | Raw GitHub evidence: issue #93 JSON, its comments, and the PR #94 snapshot. | Q-2 |

## Status

- Case-study analysis: **complete** (this folder).
- Sub-issue specs authored: **34** (in [`proposed-issues/`](./proposed-issues/)).
- GitHub sub-issues filed under #93: **34 issues (#95–#128)**, each a native
  sub-issue of #93, with **51 blocked-by edges** wired per the dependency DAG.
  The spec → issue mapping is in
  [`proposed-issues/README.md`](./proposed-issues/README.md).
</content>
