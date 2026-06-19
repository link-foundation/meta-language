# C6 — Concept-aligned cross-language grammar translation

> **Epic:** C — Emitters & codegen · **Blocked by:** A1, A3 · **Blocks:** —
> **Requirements:** P-9, P-11 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`existing-capabilities.md`](../existing-capabilities.md) §1.

## Context

Requirement **P-11** asks that, because the meta-language shares a concept layer,
a grammar can be **translated 1-to-1 across natural languages with little code
modification**. The crate already ships the exact machinery: the 351-concept
ontology (`seed_common_concept_ontology`, `src/concept_ontology.rs:423`;
`README.md:68`), the declarative `TranslationRule`/`TranslationTemplate` engine
(`src/translation_rules.rs`, re-exported `src/lib.rs:101-104`), and the proven
cross-language entry point `reconstruct_text_as` (`src/reconstruction.rs:14`) that
already round-trips English ↔ Russian ↔ concept (`README.md:235-258`). What is
missing is connecting a *grammar's human-facing surface* — its **rule names and
doc comments**, not its structure — to that machinery. [`A3`](./A3-grammar-concept-ontology.md)
seeds grammar-construct concepts and gives each [`A1`](./A1-grammar-ir.md) rule an
optional `concept` alignment (`GrammarRule.concept`, see [`A1`](./A1-grammar-ir.md)
"Design"). This issue uses that alignment to translate the surface while leaving
the algebra byte-identical — the concrete realisation of P-11.

## Goal

Given an A1 `Grammar` whose rules are concept-aligned (via [`A3`](./A3-grammar-concept-ontology.md)
or inference D9) and a target natural language, produce an **equivalent
`Grammar`** in which every rule name and doc comment is rendered in the target
language through the **shared concept layer**, while the expression algebra,
rule order, references, and start symbol are **unchanged** (structural
`PartialEq` on the algebra holds before/after, modulo the renamed `NonTerminal`
references). Reuse the existing translation engine rather than hand-rolling a
dictionary.

## Scope

**In scope**
- A new module `src/grammar/translate.rs`.
- A public `translate_grammar_surface(grammar: &Grammar, target_language: &str,
  rules: &TranslationRuleSet) -> Result<Grammar, GrammarTranslateError>`.
- The concept-alignment → translate → re-emit pipeline (below), driven by
  `GrammarRule.concept` and the A3 grammar-construct concepts.
- Consistent renaming: when a rule `r` is renamed, every `NonTerminal("r")` in the
  grammar is renamed too, so the result is a valid grammar.
- A default `TranslationRuleSet` builder seeding rules for the A3 grammar concepts
  (so common rule names like `expression`/`выражение` translate out of the box),
  plus acceptance of a caller-supplied rule set.

**Out of scope** (owned elsewhere)
- Seeding the grammar-construct concepts themselves → [`A3`](./A3-grammar-concept-ontology.md).
- Translating the *structure* or changing the grammar's language semantics — C6
  only renames human-facing tokens; the algebra is preserved.
- Emitting the translated grammar to a concrete notation/parser (BNF/PEG/Rust/JS)
  → [`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C5`](./C5-javascript-parser-codegen.md);
  C6 produces a translated **`Grammar`** that those emitters then consume.
- LLM-assisted naming when no concept is aligned → **D9** (C6 keeps a
  deterministic fallback: an unaligned rule is left untranslated).

## Design / specification

### Why reuse, not reinvent

The README already demonstrates `reconstruct_text_as("Russian", …)` rendering a
parsed sentence into Russian through concept mappings, backed by the 351-concept
lexicon (`README.md:235-258`,`:315`). A grammar rule name is just another
concept-bearing token: align it to a concept, then render that concept's
expression in the target language using the **same** `TranslationRuleSet`
mechanism (`src/translation_rules.rs:206-296`). This is precisely the "shared
concepts → 1-to-1 translation with little code modification" P-11 promises — the
"little code" is this thin adapter, because the engine already exists.

### Pipeline (concept-alignment → translate → re-emit)

1. **Align.** For each `GrammarRule`, obtain its concept id. Preferred source:
   the rule's own `concept: Option<String>` populated by [`A3`](./A3-grammar-concept-ontology.md)
   / inference D9. If `None`, attempt an exact-match lookup of the rule name
   against the ontology (`LinkNetwork::intern_concept` is the interning entry
   point, `src/concept_ontology.rs:493`; concept expressions are attached via
   `insert_concept_expression`, `:501`). If still unresolved, the rule is **left
   untranslated** (deterministic fallback — never fabricate a name).
2. **Translate.** For each aligned concept, resolve the target-language surface
   form. Build a `TranslationRule` per concept whose templates carry the
   per-language strings (`TranslationRule::new(name, query).with_template("Russian",
   "выражение")`, `src/translation_rules.rs:209`,`:255`), collected into a
   `TranslationRuleSet` (`src/translation_rules.rs:23-60`). The same
   concept→language expression table the natural-language translator already uses
   supplies these forms, so grammar-rule translation and sentence translation
   share one source of truth. Doc comments (`GrammarRule.doc`) are translated by
   running their text through `reconstruct_text_as_with_rules` on a throwaway
   network (`src/reconstruction.rs:35`), reusing the existing sentence translator
   verbatim.
3. **Re-emit.** Construct a new `Grammar` via the A1 builder: copy each rule's
   `expr`, `kind`, and `concept`, substitute the translated `name`, and rewrite
   every `NonTerminal(old)` → `NonTerminal(new)` using the rename map. Preserve
   rule order and remap the start symbol. The result is a structurally identical
   grammar with a translated surface; `source_format` is carried over.

### Structure-preservation invariant

Let `strip_names(g)` erase rule names (replace each name and matching
`NonTerminal` with a positional index). Then
`strip_names(translate_grammar_surface(g, lang, rules)) ==
strip_names(g)` for every grammar and language — the algebra is invariant under
translation. This is the headline test and the formal statement of "1-to-1,
structure preserved."

### Fn signatures

```rust
#[derive(Debug)]
pub enum GrammarTranslateError {
    /// A rule references a concept absent from the ontology / rule set.
    UnknownConcept { rule: String, concept: String },
    /// Two distinct rules translate to the same name (would collide).
    NameCollision { language: String, name: String },
}

/// Translates a grammar's human-facing surface (rule names + doc comments) into
/// `target_language` through the shared concept layer, preserving structure.
pub fn translate_grammar_surface(
    grammar: &Grammar,
    target_language: &str,
    rules: &TranslationRuleSet,
) -> Result<Grammar, GrammarTranslateError>;

/// Builds a default rule set covering the A3 grammar-construct concepts.
#[must_use]
pub fn grammar_concept_translation_rules() -> TranslationRuleSet;
```

### Worked example

Input grammar (English surface, concept-aligned by A3):

```
expression = term ("+" term)* ;   // concept: grammar::concept::expression
term       = factor ("*" factor)* ;
```

`translate_grammar_surface(g, "Russian", rules)` →

```
выражение = слагаемое ("+" слагаемое)* ;
слагаемое = множитель ("*" множитель)* ;
```

Identical algebra (`Sequence`/`ZeroOrMore`/`Choice`/terminals untouched); only
the concept-aligned names and any doc comments are rendered in Russian. Round-trip
back to English with the inverse language reproduces the original (the engine's
existing English↔Russian symmetry, `README.md:235-258`).

## File-level plan

| File | Change |
|---|---|
| `src/grammar/translate.rs` | New. `translate_grammar_surface`, `grammar_concept_translation_rules`, `GrammarTranslateError`, the align/translate/re-emit walk and rename map. |
| `src/grammar/mod.rs` | `pub mod translate;` (from [`A1`](./A1-grammar-ir.md)). |
| `src/lib.rs` | `pub use grammar::translate::{translate_grammar_surface, grammar_concept_translation_rules, GrammarTranslateError};` next to the `translation_rules` re-exports (`src/lib.rs:101-104`). |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_translate` tests. |
| `tests/fixtures/grammar/translate/` | Concept-aligned source grammars + expected translated forms (en↔ru, optionally en↔zh per the lexicon's `en/hi/ru/zh` languages). |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

> **No new dependency.** C6 is a thin adapter over existing in-crate engines.

## Reuse

- `TranslationRule`/`TranslationTemplate`/`TranslationRuleSet` (declarative,
  LiNo-serializable via `to_lino`/`from_lino`) — `src/translation_rules.rs:23-317`,
  re-exported `src/lib.rs:101-104`.
- `reconstruct_text_as_with_rules` for doc-comment text — `src/reconstruction.rs:35`.
- Concept ontology: `seed_common_concept_ontology` (`src/concept_ontology.rs:423`),
  `intern_concept` (`:493`), `insert_concept_expression` (`:501`),
  `insert_concept_mapping` (`:517`) — all `impl LinkNetwork` (`src/concept_ontology.rs:382`).
- [`A3`](./A3-grammar-concept-ontology.md) grammar-construct concepts and the
  `GrammarRule.concept` alignment field from [`A1`](./A1-grammar-ir.md).
- The 351-concept lexicon (`README.md:68`,`:315`) supplies the per-language
  surface forms; grammar and sentence translation share it.

## Acceptance criteria

- [ ] `translate_grammar_surface` renders every concept-aligned rule name and doc
      comment into the target language, leaving unaligned rules unchanged
      (deterministic fallback, no fabrication).
- [ ] The structure-preservation invariant holds: `strip_names(translate(g)) ==
      strip_names(g)` for every fixture grammar and language.
- [ ] `NonTerminal` references are renamed consistently with their rules; the
      output is a valid grammar (A1 `referenced_nonterminals()` finds no dangling
      references introduced by translation).
- [ ] A name collision (two rules → one target name) yields
      `GrammarTranslateError::NameCollision`, never a silent overwrite.
- [ ] en→ru→en round-trip reproduces the original grammar (reusing the engine's
      existing English↔Russian symmetry).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:103-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live
      under `tests/`, not `src/`).

## Tests

- Unit (`tests/unit/`, new `grammar_translate` module): build a concept-aligned
  arithmetic grammar via the A1 builder + A3 concepts; translate to Russian;
  assert the expected rule names (`expression`→`выражение`, …) and that the
  algebra is byte-identical under `strip_names`.
- Unit: a grammar with an unaligned rule → that rule's name is unchanged; aligned
  rules still translate (fallback path).
- Unit: two rules forced to the same target name → `Err(NameCollision)`.
- Unit: a rule with a `doc` comment → the comment is translated via
  `reconstruct_text_as_with_rules` and the structure is preserved.
- Integration: en→ru→en round-trip equality; and translate then emit via
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md)/[`C4`](./C4-rust-parser-codegen.md) to
  confirm a translated grammar still emits a valid notation/parser.
- Serialize the `TranslationRuleSet` via `to_lino` and reload via `from_lino`
  (`src/translation_rules.rs`), asserting round-trip equality (reuses the engine's
  own contract).

## References

- `src/translation_rules.rs:23-317` (engine), `src/reconstruction.rs:14-68`
  (`reconstruct_text_as*`), `src/concept_ontology.rs:382-533` (ontology API),
  `README.md:64-69`,`:235-258`,`:315` (cross-language reconstruction + lexicon).
- [`existing-capabilities.md`](../existing-capabilities.md) §1 (translation rows),
  [`library-survey.md`](../library-survey.md) §D.3 (351-concept lexicon, internalised),
  [`solution-plans.md`](../solution-plans.md) §Epic C (C6).
- [`A1`](./A1-grammar-ir.md), [`A3`](./A3-grammar-concept-ontology.md),
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md), [`C4`](./C4-rust-parser-codegen.md).
