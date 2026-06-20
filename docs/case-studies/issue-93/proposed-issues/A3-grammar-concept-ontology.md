# A3 — Grammar-construct concept ontology

> **Epic:** A — Meta-grammar foundation · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`C6`](./C6-concept-aligned-translation.md), [`D9`](./D9-llm-assisted-naming-merge.md)
> **Requirements:** P-11 · **Milestone:** M1
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic A,
> [`existing-capabilities.md`](../existing-capabilities.md) §1 ("Concept ontology"),
> [`library-survey.md`](../library-survey.md) §D.3.

## Context

Requirement **P-11** is the basis for near 1-to-1 cross-language translation: shared
concepts mean "if the concept is exactly the same … it will be possible to translate it
1-to-1" ([`requirements.md`](../requirements.md) P-11). The repo already owns a **concept
ontology**: `seed_common_concept_ontology()` (`src/concept_ontology.rs:423`) seeds a
351-concept semantic lexicon plus a table of **structural** PL concepts
(`STRUCTURAL_CONCEPTS`, `src/concept_ontology.rs:209` — `function`, `binding`, `loop`,
`sequence`, …), interning each via `intern_concept` (`:493`, `LinkType::Concept` at
`src/link_network.rs:53`) and attaching per-language syntax via `insert_concept_syntax_mapping`
(`:678`). The existing test pins the counts (`tests/unit/concept_ontology.rs`:
`lexicon_concepts() == 351`, `structural_concepts() >= 6`).

[`A1`](./A1-grammar-ir.md) gives the `GrammarExpr` algebra and a `GrammarRule { concept:
Option<String>, .. }` field "populated by A3 / inference D9" — but **the grammar-algebra
concepts do not exist in the ontology**, and nothing aligns an IR node to a concept
([`existing-capabilities.md`](../existing-capabilities.md) §3, gap row "No grammar-construct
concepts"). This issue seeds those concepts and wires the alignment, so every `GrammarExpr`
variant is concept-tagged — the prerequisite for concept-aligned cross-language translation
([`C6`](./C6-concept-aligned-translation.md)) and concept-named non-terminals
([`D9`](./D9-llm-assisted-naming-merge.md)).

## Goal

Seed the **grammar-construct concepts** (one per `GrammarExpr` variant plus `rule` /
`non-terminal` / `terminal`) into the existing ontology with stable ids and per-format
syntax mappings, expose the variant→concept mapping functions, and extend
`seed_common_concept_ontology()` to install them — so every A1 IR node is concept-aligned.

## Scope

**In scope**
- A module `src/grammar/concepts.rs` (re-exported from `src/lib.rs`) with the
  grammar-concept table and the variant→concept mapping.
- A `GRAMMAR_CONCEPTS` table mirroring `STRUCTURAL_CONCEPTS`
  (`src/concept_ontology.rs:175-209`): `{ id, definition, syntax: &[(format, surface)] }`.
- `grammar_expr_concept_id(&GrammarExpr) -> &'static str` and
  `rule_concept_id(&GrammarRule) -> Option<&str>` (rule's explicit `concept`, else its
  top-level expr's concept), plus `annotate_grammar_concepts(&mut Grammar)`.
- Seeding: extend `seed_common_concept_ontology()` to install grammar concepts (and/or a
  sibling `seed_grammar_concept_ontology()`), interning via `intern_concept` and mapping
  via `insert_concept_syntax_mapping`.

**Out of scope** (owned elsewhere)
- The IR / `GrammarRule.concept` field itself → [`A1`](./A1-grammar-ir.md).
- The surface syntax → [`A2`](./A2-grammar-surface-syntax.md).
- *Using* concepts to translate rule names/comments across natural languages →
  [`C6`](./C6-concept-aligned-translation.md).
- LLM-assisted naming / merge selection → [`D9`](./D9-llm-assisted-naming-merge.md).
- New concept-ontology *machinery* (interning, syntax mapping, alias links already exist
  in `src/concept_ontology.rs`).

## Design / specification

### The grammar concepts

One concept per A1 `GrammarExpr` variant ([`A1`](./A1-grammar-ir.md) `GrammarExpr`), plus the
three structural roles (`rule`, `non-terminal`, `terminal`). Concept ids are stable,
language-free, hyphenated tokens (the ontology matches ids **exactly** —
`src/concept_ontology.rs:488-495`), prefixed `grammar.` to avoid colliding with the existing
`sequence` structural concept (`src/concept_ontology.rs:261`).

| Concept id | A1 `GrammarExpr` variant / role | Cross-format surface (`syntax` examples) |
|---|---|---|
| `grammar.rule` | `GrammarRule` | BNF `::=`, EBNF `=`, PEG `=`, meta `(name: …)` |
| `grammar.sequence` | `Sequence` | PEG `a b`, EBNF `a , b` |
| `grammar.ordered-choice` | `Choice { ordered: true }` | PEG `/` |
| `grammar.unordered-choice` | `Choice { ordered: false }` | BNF/EBNF/ABNF `\|`, ABNF `/` |
| `grammar.repetition` | `Repeat { .. }` | EBNF `{ }`, GBNF `{m,n}` |
| `grammar.zero-or-more` | `ZeroOrMore` | PEG/regex `*` |
| `grammar.one-or-more` | `OneOrMore` | PEG/regex `+` |
| `grammar.optional` | `Optional` | EBNF `[ ]`, PEG `?` |
| `grammar.terminal` | `Terminal` / `TerminalInsensitive` | `"lit"` / ABNF `%i"lit"` |
| `grammar.non-terminal` | `NonTerminal` | BNF `<name>`, PEG `name` |
| `grammar.char-class` | `CharClass` | PEG/W3C `[a-z]`, GBNF `[^...]` |
| `grammar.char-range` | `CharRange` | `'a'..'z'`, ABNF `%x30-39` |
| `grammar.any-char` | `AnyChar` | PEG `.` |
| `grammar.positive-predicate` | `And` | PEG `&e` |
| `grammar.negative-predicate` | `Not` | PEG `!e` |
| `grammar.capture` | `Capture` | grammar-expressions `{name:e}`, pest `push` |
| `grammar.empty` | `Empty` | ε / empty alternative |

The `syntax` column reuses the `GrammarFormat` names ([`A1`](./A1-grammar-ir.md)) and the
per-language-tuple shape of `STRUCTURAL_CONCEPTS` (`src/concept_ontology.rs:209`). These
mappings are what [`C6`](./C6-concept-aligned-translation.md) queries for 1-to-1 translation
and what importers (B1–B7) / emitters (C1–C7) use to label nodes.

```rust
/// A grammar-algebra concept: stable id, definition, per-format surface forms.
struct GrammarConcept {
    id: &'static str,
    definition: &'static str,
    syntax: &'static [(&'static str, &'static str)],
}

const GRAMMAR_CONCEPTS: &[GrammarConcept] = &[ /* the rows above */ ];

/// Concept id for a `GrammarExpr` variant (total; never panics).
#[must_use]
pub fn grammar_expr_concept_id(expr: &GrammarExpr) -> &'static str;

/// Concept id for a rule: its explicit `concept`, else its top expr's concept.
#[must_use]
pub fn rule_concept_id(rule: &GrammarRule) -> Option<&str>;
```

### How concepts attach to A1 IR nodes

Two attachment points, both already present in [`A1`](./A1-grammar-ir.md):

1. **Per-rule:** `GrammarRule.concept: Option<String>`.
   `annotate_grammar_concepts(&mut Grammar)` fills any `None` rule concept with
   `rule_concept_id(rule).map(str::to_owned)` — the alignment
   [`D9`](./D9-llm-assisted-naming-merge.md) reads when proposing concept-named non-terminals.
2. **Per-expression:** A1's links encoding already tags each `GrammarExpr` node with its
   variant, so `grammar_expr_concept_id` lets a caller resolve the matching concept link by id
   via `LinkNetwork::find_term(id)` (`src/link_network.rs:726`) once seeded. No new field is
   added to `GrammarExpr` — the variant *is* the tag.

### Extending `seed_common_concept_ontology()`

`seed_common_concept_ontology()` returns `ConceptOntologySeedReport::new(lexicon, structural,
formatting, alias_links, syntax_mappings)` (`src/concept_ontology.rs:479`, 5-field constructor
at `:57`). Extend it so grammar concepts are installed and counted **without disturbing the
existing counts**:

- Add a `grammar_concepts: usize` field + `grammar_concepts()` accessor to
  `ConceptOntologySeedReport` (mirroring `structural_concepts()` at `:80-83`); thread it
  through the constructor.
- Loop over `GRAMMAR_CONCEPTS` exactly like the `STRUCTURAL_CONCEPTS` loop (`:445-460`):
  `intern_concept(id, Some(definition))`, then `insert_concept_syntax_mapping(link, id, format,
  surface, true)` per `syntax` tuple, accumulating `syntax_mappings`.
- Leave `lexicon`/`structural`/`formatting` untouched; only **add** the new field, so the
  pinned `== 351` / `>= 6` assertions (`tests/unit/concept_ontology.rs`) still hold.

Optionally expose `LinkNetwork::seed_grammar_concept_ontology(&mut self) -> usize` (the grammar
count) so callers can seed just this layer; have `seed_common_concept_ontology` call it.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/concepts.rs` | New. `GrammarConcept`, `GRAMMAR_CONCEPTS`, `grammar_expr_concept_id`, `rule_concept_id`, `annotate_grammar_concepts`. |
| `src/grammar/mod.rs` | Add `pub mod concepts;` (module created by [`A1`](./A1-grammar-ir.md)). |
| `src/concept_ontology.rs` | Add `grammar_concepts` field + accessor to `ConceptOntologySeedReport` (`:47-102`); seed `GRAMMAR_CONCEPTS` inside `seed_common_concept_ontology` (`:423-486`), optionally via a new `seed_grammar_concept_ontology`. |
| `src/lib.rs` | `pub use grammar::concepts::{grammar_expr_concept_id, rule_concept_id, annotate_grammar_concepts};` near the existing concept-ontology re-export (`src/lib.rs:44`). |
| `tests/unit/concept_ontology.rs` | Extend with grammar-concept assertions (existing `== 351` / `>= 6` must still pass). |
| `changelog.d/` | Add a fragment (`scripts/create-changelog-fragment.rs`). |

## Reuse

- `LinkNetwork::seed_common_concept_ontology` + `STRUCTURAL_CONCEPTS` table pattern
  (`src/concept_ontology.rs:175-209, 423-486`) — copy the table shape and seeding loop verbatim.
- `intern_concept` (`:493`) + `insert_concept_syntax_mapping` (`:678`) — interning and per-format
  syntax mapping; **do not** re-implement concept machinery.
- `LinkType::Concept` (`src/link_network.rs:53`); `ConceptOntologySeedReport`
  re-exported at `src/lib.rs:44`; `LinkNetwork::find_term` (`src/link_network.rs:726`) for
  concept-link lookup by id.
- A1 `GrammarExpr` / `GrammarRule.concept` ([`A1`](./A1-grammar-ir.md)) — the variants and the
  attachment field.
- `src/semantics.rs` `TruthValue` / `ProbabilisticTruthValue` — available if a confidence weight
  is later attached to inferred alignments (not required here); the 351-concept lexicon is
  already internal ([`library-survey.md`](../library-survey.md) §D.3).

## Acceptance criteria

- [ ] `GRAMMAR_CONCEPTS` contains one concept per A1 `GrammarExpr` variant plus `grammar.rule`,
      `grammar.non-terminal`, `grammar.terminal`; each has a non-empty definition and ≥1 `syntax`
      tuple.
- [ ] `grammar_expr_concept_id` is **total** over `GrammarExpr` (every variant returns a stable
      id; no panic), and the returned id exists in `GRAMMAR_CONCEPTS`.
- [ ] `rule_concept_id` returns the rule's explicit `concept` when set, else the concept of its
      top-level expression.
- [ ] After `seed_common_concept_ontology()` (or `seed_grammar_concept_ontology()`), every
      `grammar.*` concept id is interned (`find_term(id)` is `Some` with `LinkType::Concept`) and
      has its per-format syntax mappings (`LinkType::Semantic` links).
- [ ] The seed report exposes a `grammar_concepts()` count equal to `GRAMMAR_CONCEPTS.len()`, and
      the **pre-existing** assertions `lexicon_concepts() == 351` and `structural_concepts() >= 6`
      (`tests/unit/concept_ontology.rs`) still pass unchanged.
- [ ] `annotate_grammar_concepts` fills `GrammarRule.concept` for rules that had `None`, leaving
      explicitly-set concepts untouched.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy pedantic/nursery
      are `warn` per `Cargo.toml`), and `cargo test --all-features` all pass;
      `rust-script scripts/check-no-src-tests.rs` passes (tests live under `tests/`, not `src/`).

## Tests

- `tests/unit/concept_ontology.rs` (extend the existing file):
  - assert `GRAMMAR_CONCEPTS.len()` equals the number of `GrammarExpr` variants + 3 roles, and
    that ids are unique and `grammar.`-prefixed.
  - exhaustive match over each `GrammarExpr` variant → `grammar_expr_concept_id` returns a
    distinct id present in the table.
  - seed a `self_describing` network, call the seed, then for every `grammar.*` id assert
    `find_term(id)` is a `Concept` link and that each `syntax` tuple produced a `Semantic`
    mapping link.
  - regression: `lexicon_concepts() == 351`, `structural_concepts() >= 6`,
    `grammar_concepts() == GRAMMAR_CONCEPTS.len()`.
  - `rule_concept_id`: explicit-concept rule returns it; concept-less rule returns its expr's
    concept; `annotate_grammar_concepts` fills only the `None` rules.
- Pure in-process, no IO; reuse `LinkNetwork::self_describing()` as in the existing test.

## References

- Concept ontology source: `src/concept_ontology.rs:175-209` (`STRUCTURAL_CONCEPTS`),
  `:423-486` (`seed_common_concept_ontology`), `:488-703` (intern / syntax-mapping machinery).
- [`existing-capabilities.md`](../existing-capabilities.md) §1 (concept ontology row), §3
  (gap row), [`library-survey.md`](../library-survey.md) §D.3 (351-concept lexicon internalised),
  [`solution-plans.md`](../solution-plans.md) §Epic A.
- Downstream consumers: [`C6`](./C6-concept-aligned-translation.md) (concept-aligned translation),
  [`D9`](./D9-llm-assisted-naming-merge.md) (concept-named non-terminals); sibling keystone:
  [`A1`](./A1-grammar-ir.md).
