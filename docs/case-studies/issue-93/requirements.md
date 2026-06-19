# Issue #93 — Requirements Register

Every requirement extracted verbatim-in-intent from
[issue #93](https://github.com/link-foundation/meta-language/issues/93) and its
body (see [`raw-data/issue-93.json`](./raw-data/issue-93.json)), split into the
smallest independently-verifiable units. Two kinds are tracked:

- **Product requirements (`P-*`)** — capabilities the *meta-language system*
  must gain. Each maps to one or more proposed sub-issues.
- **Process requirements (`Q-*`)** — obligations on *this planning task itself*
  (data capture, case-study analysis, issue creation). Each is satisfied inside
  this PR.

The verbatim source sentences are quoted so the mapping is auditable.

## Product requirements

| ID | Requirement (intent) | Source sentence (quoted) | Addressed by |
|---|---|---|---|
| P-1 | Make it **easy to develop new grammars** that inherit from the meta-language. | "We should allow to easily develop grammars inherited from meta-language…" | A1 (grammar IR), A2 (surface syntax), E1 (CLI), E2 (registry) |
| P-2 | **Infer a grammar from example texts** (find patterns in examples of a grammar's texts). | "…by using grammar inference (find patterns in examples of texts of that grammar)." | D1–D9 (inference engine) |
| P-3 | **Reconstruct a programming-language grammar from only positive examples** of correct texts. | "Ideally it should be possible to reconstruct programming language grammar by using only examples of correct programming language texts." | D5 (black-box CFG inference), D6 (delimiter prior), D7 (generalization), E3 (benchmarks) |
| P-4 | Use **meta-notation as the basis**; the meta-language is **inherited from meta-notation**. | "We also need to make sure we use …/meta-notation as a basis for our meta-language. And our meta language itself is inherited from meta-notation." | A2 (meta-notation substrate), D6 (delimiter prior) |
| P-5 | When constructing new grammars, allow **maximum freedom of grammar inference**. | "…it should be possible to have maximum freedom of grammar inference." | A1 (expressive IR), D5–D9 (multiple inference strategies), D8 (semantic constraints) |
| P-6 | Ground the work in the **best grammar-inference papers** (online search). | "Search online best papers on the topic of grammar inference…" | [`literature-review.md`](./literature-review.md) |
| P-7 | Provide a **working Rust implementation**. | "…we need to have working rust implementation…" | All A/B/C/D/E issues (Rust, with tests) |
| P-8 | Generate a **grammar description in the meta-language itself**. | "…that is able to generate grammar description in the meta-language itself…" | A1 (IR), A2 (LiNo/meta-notation surface), D5 (inference emits into the IR) |
| P-9 | **Translate** the meta-grammar **to Rust, JavaScript, and other languages**. | "…which after that we should be able to translate to rust, javascript and other languages…" | C2–C5 (codegen), C6 (concept-aligned translation) |
| P-10 | **Parse PEG, BNF, and other grammar languages *as* meta-language** (import them into the meta-grammar representation). | "(we also should include of PEG, BNF and other languages, to be parsed as meta language)." | B1–B7 (importers) |
| P-11 | Leverage **shared concepts across formal and natural languages** for near **1-to-1 translation**. | "…meta language has shared concepts between all the languages, so if the concept is exactly the same … it will be possible to translate it 1 to 1 with little code modification…" | A3 (grammar concept ontology), C6 (concept-aligned translation) |
| P-12 | **Beat all competitors in all metrics.** | "…beats all the competitors in all metrics." | D1 (metrics harness), E3 (competitor benchmark suite), [`competitive-analysis.md`](./competitive-analysis.md) |

## Process requirements (satisfied by this PR)

| ID | Requirement (intent) | Source sentence (quoted) | Status / evidence |
|---|---|---|---|
| Q-1 | Plan **as many issues as possible** so the system fully implements everything. | "We need to plan as much issues as possible to make sure our system fully implements everything…" | Done — 34 proposed sub-issues across 6 epics; see [`proposed-issues/`](./proposed-issues/) and [`solution-plans.md`](./solution-plans.md). |
| Q-2 | **Collect issue data** into `./docs/case-studies/issue-93/`. | "…compile that data to `./docs/case-studies/issue-{id}` folder…" | Done — [`raw-data/`](./raw-data/) (issue, comments, PR snapshots). |
| Q-3 | **Deep case-study analysis**, with **online research** for additional facts. | "…use it to do deep case study analysis (also make sure to search online for additional facts and data)…" | Done — [`literature-review.md`](./literature-review.md), [`library-survey.md`](./library-survey.md), [`existing-capabilities.md`](./existing-capabilities.md), [`competitive-analysis.md`](./competitive-analysis.md). |
| Q-4 | **List each and every requirement** from the issue. | "…list of each and all requirements from the issue…" | Done — this file. |
| Q-5 | **Propose solutions / solution plans per requirement.** | "…propose possible solutions and solution plans for each requirement…" | Done — [`solution-plans.md`](./solution-plans.md). |
| Q-6 | **Check existing components/libraries** that solve a similar problem or can help. | "(we should also check known existing components/libraries, that solve similar problem or can help in solutions)." | Done — [`library-survey.md`](./library-survey.md) (50+ components, licence-vetted). |
| Q-7 | All issues must be **sub-issues of #93**. | "Make sure all issues are sub-issues of this issue…" | Done at creation — via the `addSubIssue` GraphQL mutation; see [`proposed-issues/README.md`](./proposed-issues/README.md). |
| Q-8 | Issues must have **"blocked by" relationships** configured via `gh`. | "…have properly configured blocked by relationships using gh tool, to make dependencies between issues visible." | Done at creation — via REST `POST /repos/.../issues/{n}/dependencies/blocked_by`; dependency DAG in [`solution-plans.md`](./solution-plans.md). |
| Q-9 | Each issue must be **maximally detailed** so even a weak AI agent can complete it. | "Each issue should be maximum detailed possible, so even week AI agent will be able to complete it." | Done — every spec carries Context, Scope, File-level plan, Algorithm/spec, Reuse, Acceptance criteria, Tests, Out-of-scope, and References. |

## Reading of the ambiguous points

- **"inherited from meta-notation" (P-4).** meta-notation is a *working* parser
  (Rust + TS, Unlicense) for the universal delimiter skeleton: brackets
  `() {} []` (nested), quotes `'' "" \`\`` (opaque strings), and text blocks,
  with lossless round-trip. "Inheritance" here means the meta-language's grammar
  *surface* and the inference engine's *structural prior* are both built on that
  delimiter skeleton, not that we fork the crate. See
  [`existing-capabilities.md`](./existing-capabilities.md) §"meta-notation
  lineage" for how this repo already mirrors the model.
- **"other languages" (P-9, P-10).** Treated as an open, extensible set. The
  proposed issues deliver a *trait-based, registry-driven* importer/emitter
  architecture (one issue per concrete format) so adding "another language"
  is a bounded, repeatable task rather than a core change.
- **"beats all the competitors in all metrics" (P-12).** Operationalised as: on
  the standard black-box CFG-inference benchmarks (the TreeVada/Arvada/GLADE
  corpora) the system must meet-or-exceed published precision, recall, F1, and
  wall-clock, *and* additionally win on metrics those tools do not report
  (round-trip fidelity, number of importable/exportable grammar formats,
  cross-language translation accuracy). The metric definitions and the golden
  baselines are pinned in D1 and [`competitive-analysis.md`](./competitive-analysis.md).
- **"Ideally … only … correct … texts" (P-3).** Read as *positive-only*
  inference (no labelled negatives, no membership oracle required), which is the
  hardest and most valuable setting; an optional oracle/active-learning path is
  planned (D10) but never required for the core deliverable.
</content>
</invoke>
