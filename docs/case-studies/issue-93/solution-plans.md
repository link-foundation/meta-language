# Solution Plans & Issue Decomposition (issue #93)

> Satisfies Q-5 ("propose solutions and solution plans for each requirement")
> and Q-1 ("plan as many issues as possible"). Per-requirement traceability is in
> [`requirements.md`](./requirements.md); library choices in
> [`library-survey.md`](./library-survey.md); the prior art in
> [`literature-review.md`](./literature-review.md); the P-12 bar in
> [`competitive-analysis.md`](./competitive-analysis.md); the reusable substrate
> in [`existing-capabilities.md`](./existing-capabilities.md).

## 1. Strategy in one paragraph

Grammar inference from positive examples is impossible without an inductive bias
(Gold 1967). meta-language already *owns* the two biases the current SOTA (NatGI
2025) had to bolt on: **(a)** meta-notation's lossless **delimiter skeleton** is
the exact "bracket-guided" structural prior NatGI credits for most of its gains,
and **(b)** the **351-concept shared ontology** is the source of "meaningful,
natural non-terminal names" NatGI uses an LLM to fabricate. So the plan is not to
invent a novel inference algorithm but to **reproduce the published SOTA pipeline
on a substrate that supplies its strongest priors for free, store everything as
links in the existing network, and then win the secondary metrics no competitor
contests** (cross-format import/emit, parser codegen, GBNF, cross-language
translation). Everything is built in Rust (P-7), stored as `LinkType::Grammar`
links (P-8), authored in a meta-notation-derived surface (P-4), and gated by a
benchmark harness (P-12).

## 2. Solution plans per epic

### Epic A — Meta-grammar foundation (the keystone)
*Addresses P-1, P-4, P-5, P-8, P-11.*
A grammar must be a first-class value in the links network before anything can
import, emit, infer, or translate it.

- **A1 — Grammar IR / expression algebra.** Define an 18-ish-variant PEG/CFG
  expression algebra (sequence, ordered & unordered choice, repetition `* + ?`,
  optional, terminal/string, non-terminal ref, char-class/range, `.` any,
  positive/negative lookahead, capture/label, repetition bounds) plus
  `GrammarRule` and `Grammar`. Model it after `pest_meta::ast::Expr` and
  `ungrammar`. Persist as links tagged `LinkType::Grammar`; provide
  `ToLinks`/`FromLinks` so a grammar round-trips through the network.
  **Reuse:** `link_network.rs` (`LinkType::Grammar` at :51), `rust_codec`
  `ToLinks/FromLinks`, `self_description.rs:28-32`. **Keystone — blocks most.**
- **A2 — Grammar surface syntax (meta-notation-derived).** A textual surface to
  *author* grammars in the meta-language, parsed via the meta-notation delimiter
  skeleton and serialised as LiNo. Declares the meta-language grammar surface a
  formal derivative of meta-notation (P-4).
  **Reuse:** `links-notation`, `lino_parser`, `lino_serialization`, meta-notation
  model (see existing-capabilities §2). **Blocked by A1.**
- **A3 — Grammar-construct concept ontology.** Seed grammar-algebra concepts
  (rule, sequence, choice, repetition, terminal, non-terminal, predicate,
  capture…) into the concept ontology so every IR node is concept-aligned —
  the basis of 1-to-1 translation (P-11) and concept-named non-terminals (D9).
  **Reuse:** `concept_ontology.rs`, `seed_common_concept_ontology()`,
  `semantics.rs`. **Blocked by A1.**

### Epic B — Grammar-format importers ("parse PEG, BNF, … as meta-language")
*Addresses P-10.* Each importer is an independent `LanguageParser`-style adapter
that parses a format and lowers it into the A1 IR. All blocked by A1 (and reuse
A2's lowering helpers); otherwise mutually independent → maximal parallelism.

- **B1 — BNF importer.** Reuse the `bnf` crate (MIT). 
- **B2 — EBNF importer.** Reuse the `ebnf`/`ebnf-fmt` crate.
- **B3 — ABNF (RFC 5234) importer.** Reuse the `abnf` crate.
- **B4 — PEG importer.** Reuse `pest_meta` to parse `.pest`; lower its `ast::Expr`
  (near-isomorphic to A1) → IR.
- **B5 — tree-sitter `grammar.json` importer.** Parse the JSON DSL of the ~30
  already-wired grammars (`tree_sitter_adapter.rs:136-207`) → IR.
- **B6 — ANTLR v4 `.g4` importer.** Parse the ANTLR meta-grammar → IR (lexer/
  parser rule split, fragment handling).
- **B7 — Lark + GBNF importer.** Parse Lark `.lark` and llama.cpp GBNF → IR
  (closes the loop with the C3 emitter for round-trip tests).

### Epic C — Emitters & codegen ("translate to Rust, JavaScript, and other languages")
*Addresses P-9, P-11.* Each emitter consumes the A1 IR. Text emitters reuse the
`TranslationRule`/`TranslationTemplate` engine instead of ad-hoc string building.

- **C1 — BNF/EBNF/ABNF text emitters.** IR → each notation; paired with B1–B3 for
  round-trip fidelity (F2).
- **C2 — PEG emitter.** IR → `.pest` (and/or `peg`/`winnow`-ready form).
- **C3 — GBNF emitter.** IR → llama.cpp GBNF for direct LLM-constraint use
  (a metric no competitor reports — P-12 secondary win).
- **C4 — Rust parser codegen.** IR → runnable Rust parser source (target `pest`
  or `winnow`). **Reuse:** `rust_codec`, `RustTypeShape`.
- **C5 — JavaScript parser codegen.** IR → JS parser (target `peggy`/PEG.js).
- **C6 — Concept-aligned cross-language grammar translation.** Use A3 concepts +
  the existing `reconstruct_text_as` mechanism to translate a grammar's
  *human-facing* surface (rule names, comments) 1-to-1 across natural languages
  while preserving structure (P-11). **Reuse:** `translation_rules.rs`,
  `reconstruct_text_as`.
- **C7 — tree-sitter `grammar.js` emitter.** IR → tree-sitter grammar (pairs with
  B5 for round-trip; widens ecosystem reach).

### Epic D — Inference engine
*Addresses P-2, P-3, P-5, P-12.* The research core. Built bottom-up: metrics
first (so every later issue is measurable), then lexical → regular → structural →
full CFG → generalization → semantic → LLM-assist → optional oracle.

- **D1 — Inference evaluation harness (build first).** Precision/recall/F1 by
  sampling, MDL/grammar-size scoring, golden-corpus runner. Pins the P-12 metric
  definitions. **Reuse:** `parity.rs` fixture discipline, tree-sitter CSTs as
  oracles.
- **D2 — Tokenisation / lexical-class inference.** Infer terminal/character
  classes and token boundaries from examples (the lexer layer).
- **D3 — State-merging regular inference.** RPNI + EDSM (clean-room from
  de la Higuera 2010 / Apache LearnLib / MIT GIToolbox); optional ALERGIA for the
  stochastic case. Covers regular sublanguages feeding the CFG layer.
- **D4 — Sequitur structural-compression pass.** Linear-time hierarchical
  structure from sequences (digram-uniqueness + rule-utility) as a cheap,
  unencumbered first structural pass.
- **D5 — Black-box CFG inference engine.** Port TreeVada (MIT, deterministic,
  bracket-prior) as the core; fold in Arvada's bubble-merge and Kedavra's
  incremental segmentation. **Primary research deliverable. Blocked by A1, D1, D6.**
- **D6 — Delimiter-skeleton structural prior.** Feed meta-notation's bracket/
  quote/text-block skeleton into D5 as the structural prior (this is NatGI's #1
  technique, native here — P-4). **Blocked by A1, A2.**
- **D7 — Generalization & MDL/Occam minimization.** Turn the over-fit parse
  forest into a compact grammar (Bayesian model-merging / HDD-style simplification).
- **D8 — Semantic-constraint inference.** ISLearn-style invariant mining
  (instantiate→filter→DNF→rank) over the inferred CFG for beyond-context-free
  constraints (P-5 "maximum freedom"). **Reuse:** `TruthValue`, `LinkQuery`.
  Clean-room (ISLearn is GPL).
- **D9 — LLM-assisted naming & merge selection (optional).** Mirror NatGI's LLM
  use, but ground names in the A3 concept ontology and keep a deterministic
  fallback so P-7 never *requires* a model. **Blocked by A3, D5.**
- **D10 — Optional active learning (oracle path).** L\*/TTT when a membership
  oracle (e.g. an existing parser) is available; never required for P-3.
  **Reuse:** `ParserRegistry` as oracle source.

### Epic E — Tooling, integration, benchmarking
*Addresses P-1, P-7, P-12.*

- **E1 — CLI subcommands.** `infer`, `import-grammar`, `emit-grammar`,
  `translate-grammar` on the existing `clap` scaffold (`main.rs`).
- **E2 — Inferred-grammar runtime parser.** Wrap an A1 grammar as a
  `LanguageParser` and register it in `ParserRegistry` so an inferred grammar
  immediately parses new text. **Reuse:** `parser_registry.rs`,
  `language_parser.rs`. **Blocked by A1.**
- **E3 — Competitor benchmark suite.** Vendor the published TreeVada/Arvada/GLADE
  corpora, run D1 metrics against them, gate P-12 claims in CI. **Blocked by D1, D5.**
- **E4 — Grammar authoring ergonomics.** Validation, diagnostics, and friendly
  errors for hand-written grammars (serves P-1 "easy to develop").
- **E5 — End-to-end integration tests + `examples/`.** Full pipelines
  (examples → infer → emit Rust/JS/GBNF → re-parse) wired as runnable examples.

### Epic F — Documentation
- **F1 — Grammar-subsystem user & architecture docs.** How to author, import,
  emit, infer, translate; architecture overview.
- **F2 — Grammar-format fidelity matrix.** Round-trip fidelity per format, modeled
  on `docs/cross-format-fidelity.md`.

## 3. Complete issue list (34 issues)

| ID | Title | Epic | Blocked by | Addresses |
|---|---|---|---|---|
| A1 | Grammar IR / expression algebra | A | — | P-1,P-5,P-8 |
| A2 | Grammar surface syntax (meta-notation-derived) | A | A1 | P-1,P-4,P-8 |
| A3 | Grammar-construct concept ontology | A | A1 | P-11 |
| B1 | BNF importer | B | A1 | P-10 |
| B2 | EBNF importer | B | A1 | P-10 |
| B3 | ABNF importer | B | A1 | P-10 |
| B4 | PEG (`.pest`) importer | B | A1 | P-10 |
| B5 | tree-sitter `grammar.json` importer | B | A1 | P-10 |
| B6 | ANTLR v4 `.g4` importer | B | A1 | P-10 |
| B7 | Lark + GBNF importer | B | A1 | P-10 |
| C1 | BNF/EBNF/ABNF emitters | C | A1 | P-9 |
| C2 | PEG (`.pest`) emitter | C | A1 | P-9 |
| C3 | GBNF emitter (LLM interop) | C | A1 | P-9,P-12 |
| C4 | Rust parser codegen | C | A1 | P-7,P-9 |
| C5 | JavaScript parser codegen | C | A1 | P-9 |
| C6 | Concept-aligned cross-language translation | C | A1,A3 | P-9,P-11 |
| C7 | tree-sitter `grammar.js` emitter | C | A1 | P-9 |
| D1 | Inference evaluation harness | D | A1 | P-12 |
| D2 | Tokenisation / lexical-class inference | D | A1 | P-2 |
| D3 | State-merging regular inference (RPNI/EDSM) | D | A1,D1 | P-2 |
| D4 | Sequitur structural-compression pass | D | A1 | P-2 |
| D5 | Black-box CFG inference engine (TreeVada port) | D | A1,D1,D6 | P-2,P-3,P-12 |
| D6 | Delimiter-skeleton structural prior | D | A1,A2 | P-3,P-4,P-5 |
| D7 | Generalization & MDL/Occam minimization | D | A1,D5 | P-3,P-5 |
| D8 | Semantic-constraint inference (ISLearn-style) | D | A1,D5 | P-5 |
| D9 | LLM-assisted naming & merge selection (optional) | D | A3,D5 | P-3,P-12 |
| D10 | Optional active learning (L\*/TTT oracle path) | D | A1,E2 | P-2 |
| E1 | CLI: infer/import/emit/translate-grammar | E | A1,B1,C1,D5 | P-1,P-7 |
| E2 | Inferred-grammar runtime parser (registry) | E | A1 | P-1,P-7 |
| E3 | Competitor benchmark suite | E | D1,D5 | P-12 |
| E4 | Grammar authoring ergonomics | E | A1,A2 | P-1 |
| E5 | End-to-end integration tests + `examples/` | E | C4,C5,D5,E1 | P-7 |
| F1 | Grammar-subsystem user & architecture docs | F | A1 | P-1 |
| F2 | Grammar-format fidelity matrix | F | B1,C1 | P-9,P-10 |

## 4. Dependency DAG

```
A1 (Grammar IR) ── keystone; blocks all of B*, C*, D*, E2, F1
│
├─ A2 ──┬─ D6 ─┐
│       └─ E4  │
├─ A3 ──┬─ C6  │
│       └─ D9 ─┤
│             │
├─ B1..B7 (importers, parallel) ─┐
│                                ├─ F2 (with C1)
├─ C1..C7 (emitters, parallel) ──┘
│   └─ C4,C5 ─┐
│             ├─ E5
├─ D1 ─┬─ D3  │
│      ├─ D5 ─┼─ D7, D8, D9   (D5 also needs D6)
│      └─ E3 ─┘
├─ D2, D4 (parallel structural passes)
├─ D10 (needs E2)
├─ E1 (needs A1,B1,C1,D5)
└─ E2 ── D10
```

**Critical path:** `A1 → D6 → D5 → {D7,E3,E5}`. A1 unblocks the widest fan-out, so
it ships first; D6 (the meta-notation prior) and D1 (metrics) are the next
priorities because D5 (the SOTA-beating engine) needs both.

## 5. Release sequencing (suggested milestones)

1. **M1 — Foundation:** A1, A2, A3 (+ F1 stub). Grammars become first-class.
2. **M2 — Interop:** B1–B7, C1–C3, C7, F2. Import/emit any format (P-9, P-10).
3. **M3 — Inference core:** D1, D2, D3, D4, D6, D5. Positive-only CFG inference (P-2, P-3).
4. **M4 — Beat the SOTA:** D7, D8, D9, E3. Generalization + benchmarks vs NatGI (P-12).
5. **M5 — Productise:** C4, C5, C6, E1, E2, E4, E5, D10. Codegen, CLI, translation, examples (P-1, P-7, P-9, P-11).

Each issue is independently shippable behind its dependencies; the per-issue
specs in [`proposed-issues/`](./proposed-issues/) carry full implementation
detail (Q-9).
</content>
