# Proposed sub-issues for #93

The 34 maximally-detailed source specs for the issue
[#93](https://github.com/link-foundation/meta-language/issues/93) grammar
extensibility & inference initiative — one Markdown file per planned sub-issue
(requirement Q-1, Q-9). Each spec is self-contained: `Context`, `Goal`, `Scope`
(in/out), `Design / specification`, `File-level plan`, `Reuse`, `Acceptance
criteria`, `Tests`, and `References`.

Read the design context first:
[`solution-plans.md`](../solution-plans.md) (per-epic plans, the DAG, release
sequencing), [`requirements.md`](../requirements.md) (P-/Q- register),
[`existing-capabilities.md`](../existing-capabilities.md) (reuse seams),
[`library-survey.md`](../library-survey.md) (licence-vetted components),
[`literature-review.md`](../literature-review.md) and
[`competitive-analysis.md`](../competitive-analysis.md) (the metrics to beat).

## Epics

- **A — Meta-grammar foundation:** the grammar IR, its surface syntax, and its
  concept alignment. **A1 is the keystone** every other issue depends on.
- **B — Grammar-format importers:** parse BNF/EBNF/ABNF/PEG/ANTLR/Lark/GBNF/
  tree-sitter *as* meta-language (P-10).
- **C — Emitters & codegen:** emit those formats back, plus Rust/JS parser
  codegen and concept-aligned cross-language translation (P-9, P-11).
- **D — Inference engine:** the research core — metrics harness, lexical/regular/
  structural inference, the black-box CFG engine, generalization, semantic
  constraints, and the optional LLM/active-learning paths (P-2, P-3, P-5, P-12).
- **E — Tooling, integration, benchmarking:** CLI, runtime parser, the competitor
  benchmark suite, authoring ergonomics, and end-to-end examples (P-1, P-7, P-12).
- **F — Documentation & fidelity:** user/architecture docs and the format
  fidelity matrix (P-1, P-9, P-10).

## Index (spec → filed issue)

All 34 specs are **filed as GitHub issues #95–#128**, each attached as a native
**sub-issue of [#93](https://github.com/link-foundation/meta-language/issues/93)**
(REST `POST /issues/93/sub_issues`) with every **blocked-by** edge in the DAG wired
(REST `POST /issues/{n}/dependencies/blocked_by`) — 34 sub-issues, 51 dependency
edges, satisfying Q-7 and Q-8. The **Filed as** column links each spec to its issue.

| Spec | Title | Epic | Blocked by | Addresses | Milestone | Filed as |
|---|---|---|---|---|---|---|
| [A1](./A1-grammar-ir.md) | Grammar IR / expression algebra | A | — | P-1, P-5, P-8 | M1 | [#95](https://github.com/link-foundation/meta-language/issues/95) |
| [A2](./A2-grammar-surface-syntax.md) | Grammar surface syntax (meta-notation-derived) | A | A1 | P-1, P-4, P-8 | M1 | [#96](https://github.com/link-foundation/meta-language/issues/96) |
| [A3](./A3-grammar-concept-ontology.md) | Grammar-construct concept ontology | A | A1 | P-11 | M1 | [#97](https://github.com/link-foundation/meta-language/issues/97) |
| [B1](./B1-bnf-importer.md) | BNF importer | B | A1 | P-10 | M2 | [#99](https://github.com/link-foundation/meta-language/issues/99) |
| [B2](./B2-ebnf-importer.md) | EBNF importer | B | A1 | P-10 | M2 | [#100](https://github.com/link-foundation/meta-language/issues/100) |
| [B3](./B3-abnf-importer.md) | ABNF importer | B | A1 | P-10 | M2 | [#101](https://github.com/link-foundation/meta-language/issues/101) |
| [B4](./B4-peg-importer.md) | PEG (`.pest`) importer | B | A1 | P-10 | M2 | [#102](https://github.com/link-foundation/meta-language/issues/102) |
| [B5](./B5-tree-sitter-json-importer.md) | tree-sitter `grammar.json` importer | B | A1 | P-10 | M2 | [#103](https://github.com/link-foundation/meta-language/issues/103) |
| [B6](./B6-antlr-importer.md) | ANTLR v4 `.g4` importer | B | A1 | P-10 | M2 | [#104](https://github.com/link-foundation/meta-language/issues/104) |
| [B7](./B7-lark-gbnf-importer.md) | Lark + GBNF importer | B | A1 | P-10 | M2 | [#105](https://github.com/link-foundation/meta-language/issues/105) |
| [C1](./C1-bnf-ebnf-abnf-emitters.md) | BNF/EBNF/ABNF emitters | C | A1 | P-9 | M2 | [#106](https://github.com/link-foundation/meta-language/issues/106) |
| [C2](./C2-peg-emitter.md) | PEG (`.pest`) emitter | C | A1 | P-9 | M2 | [#107](https://github.com/link-foundation/meta-language/issues/107) |
| [C3](./C3-gbnf-emitter.md) | GBNF emitter (LLM interop) | C | A1 | P-9, P-12 | M2 | [#108](https://github.com/link-foundation/meta-language/issues/108) |
| [C4](./C4-rust-parser-codegen.md) | Rust parser codegen | C | A1 | P-7, P-9 | M5 | [#109](https://github.com/link-foundation/meta-language/issues/109) |
| [C5](./C5-javascript-parser-codegen.md) | JavaScript parser codegen | C | A1 | P-9 | M5 | [#110](https://github.com/link-foundation/meta-language/issues/110) |
| [C6](./C6-concept-aligned-translation.md) | Concept-aligned cross-language translation | C | A1, A3 | P-9, P-11 | M5 | [#112](https://github.com/link-foundation/meta-language/issues/112) |
| [C7](./C7-tree-sitter-grammar-js-emitter.md) | tree-sitter `grammar.js` emitter | C | A1 | P-9 | M2 | [#111](https://github.com/link-foundation/meta-language/issues/111) |
| [D1](./D1-inference-evaluation-harness.md) | Inference evaluation harness | D | A1 | P-12 | M3 | [#113](https://github.com/link-foundation/meta-language/issues/113) |
| [D2](./D2-lexical-class-inference.md) | Tokenisation / lexical-class inference | D | A1 | P-2 | M3 | [#114](https://github.com/link-foundation/meta-language/issues/114) |
| [D3](./D3-state-merging-regular-inference.md) | State-merging regular inference (RPNI/EDSM) | D | A1, D1 | P-2 | M3 | [#117](https://github.com/link-foundation/meta-language/issues/117) |
| [D4](./D4-sequitur-compression.md) | Sequitur structural-compression pass | D | A1 | P-2 | M3 | [#115](https://github.com/link-foundation/meta-language/issues/115) |
| [D5](./D5-blackbox-cfg-inference.md) | Black-box CFG inference engine (TreeVada port) | D | A1, D1, D6 | P-2, P-3, P-12 | M3 | [#118](https://github.com/link-foundation/meta-language/issues/118) |
| [D6](./D6-delimiter-structural-prior.md) | Delimiter-skeleton structural prior | D | A1, A2 | P-3, P-4, P-5 | M3 | [#116](https://github.com/link-foundation/meta-language/issues/116) |
| [D7](./D7-generalization-mdl-minimization.md) | Generalization & MDL/Occam minimization | D | A1, D5 | P-3, P-5 | M4 | [#119](https://github.com/link-foundation/meta-language/issues/119) |
| [D8](./D8-semantic-constraint-inference.md) | Semantic-constraint inference (ISLearn-style) | D | A1, D5 | P-5 | M4 | [#120](https://github.com/link-foundation/meta-language/issues/120) |
| [D9](./D9-llm-assisted-naming-merge.md) | LLM-assisted naming & merge selection (optional) | D | A3, D5 | P-3, P-12 | M4 | [#121](https://github.com/link-foundation/meta-language/issues/121) |
| [D10](./D10-active-learning-oracle.md) | Optional active learning (L\*/TTT oracle path) | D | A1, E2 | P-2 | M5 | [#123](https://github.com/link-foundation/meta-language/issues/123) |
| [E1](./E1-cli-grammar-subcommands.md) | CLI: infer/import/emit/translate-grammar | E | A1, B1, C1, D5 | P-1, P-7 | M5 | [#124](https://github.com/link-foundation/meta-language/issues/124) |
| [E2](./E2-inferred-grammar-runtime-parser.md) | Inferred-grammar runtime parser (registry) | E | A1 | P-1, P-7 | M5 | [#122](https://github.com/link-foundation/meta-language/issues/122) |
| [E3](./E3-competitor-benchmark-suite.md) | Competitor benchmark suite | E | D1, D5 | P-12 | M4 | [#125](https://github.com/link-foundation/meta-language/issues/125) |
| [E4](./E4-grammar-authoring-ergonomics.md) | Grammar authoring ergonomics | E | A1, A2 | P-1 | M5 | [#126](https://github.com/link-foundation/meta-language/issues/126) |
| [E5](./E5-end-to-end-integration-examples.md) | End-to-end integration tests + `examples/` | E | C4, C5, D5, E1 | P-7 | M5 | [#128](https://github.com/link-foundation/meta-language/issues/128) |
| [F1](./F1-grammar-subsystem-docs.md) | Grammar-subsystem user & architecture docs | F | A1 | P-1 | M1 | [#98](https://github.com/link-foundation/meta-language/issues/98) |
| [F2](./F2-grammar-format-fidelity-matrix.md) | Grammar-format fidelity matrix | F | B1, C1 | P-9, P-10 | M2 | [#127](https://github.com/link-foundation/meta-language/issues/127) |

## Critical path

`A1 → D6 → D5 → {D7, E3, E5}` — the grammar IR unblocks everything; the
delimiter-skeleton prior (D6) unblocks the black-box CFG engine (D5), which in
turn unblocks generalization (D7), the competitor benchmark (E3), and the
end-to-end examples (E5). Full DAG in [`solution-plans.md`](../solution-plans.md) §4.
