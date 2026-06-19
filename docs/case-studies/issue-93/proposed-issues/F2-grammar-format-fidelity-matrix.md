# F2 — Grammar-format fidelity matrix

> **Epic:** F — Documentation & fidelity · **Blocked by:** [`B1`](./B1-bnf-importer.md), [`C1`](./C1-bnf-ebnf-abnf-emitters.md) · **Blocks:** —
> **Requirements:** P-9, P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic F (F2), §5 milestone M2,
> [`requirements.md`](../requirements.md) P-9, P-10.

## Context

P-10 makes other grammar notations importable *as* meta-language
([`B1`](./B1-bnf-importer.md)–B7) and re-emittable ([`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md)),
and P-9 asks for parity-style coverage of capabilities. The open question those two
raise is **what survives a round trip** through the [`A1`](./A1-grammar-ir.md) IR:
not every construct exists in every notation (ABNF has no ordered choice; BNF has no
repetition operator; GBNF is a restricted subset), so some imports → IR → emits are
byte-identical, some are *equivalent* (same language, different spelling), and some
are *lossy* (a construct must degrade through a documented fallback).

The crate already answers exactly this shape of question for **document** formats:
[`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md) is a
per-format concept-support matrix with documented lossy fallbacks, backed by
`LanguageProfile` (`src/language_profile.rs:15`), `DOCUMENT_FORMATS` /
`CROSS_FORMAT_CONCEPTS` / `document_format_profile`
(`src/document_formatting/profile.rs:18`, `:23`, `:42`), and a generating test
(`every_ordered_format_pair_round_trips_through_the_concept_layer`,
`tests/unit/cross_format_reconstruction.rs:112`). The parity registry
(`src/parity.rs`, e.g. `GRAMMAR_EMBEDDING_TARGETS` per
[`README.md`](../../../../README.md) §Parity Implementation:297-318) is the parallel
precedent for capability coverage as a tested API.

F2 produces the same artefact for **grammar** formats, reusing that pattern. It is
blocked by the first importer ([`B1`](./B1-bnf-importer.md)) and the first emitter
([`C1`](./C1-bnf-ebnf-abnf-emitters.md)) — once one import↔emit pair exists the
matrix infrastructure can land at **Milestone M2** and each later importer/emitter
adds its row.

## Goal

Deliver a **grammar-format round-trip fidelity matrix** — a `LanguageProfile`-style
capability profile per grammar format plus a documented-fallback table — recording
for each format whether each grammar-algebra construct is **lossless**,
**equivalent**, or **lossy** on `import → IR → emit → re-import`, published as
`docs/grammar/fidelity.md` and *generated from a test* so the doc cannot drift from
the code (P-9, P-10).

## Scope

**In scope**
- A `GrammarFormatProfile` (a `LanguageProfile` over grammar-algebra concepts, or a
  thin wrapper if the construct vocabulary differs), with:
  - `GRAMMAR_FORMATS: &[&str]` — the importable/emittable formats present so far.
  - `GRAMMAR_CONSTRUCTS: &[&str]` — the [`A1`](./A1-grammar-ir.md) `GrammarExpr`
    construct vocabulary (sequence, ordered-choice, unordered-choice, optional,
    zero-or-more, one-or-more, repeat-range, char-range, char-class, any-char,
    terminal, case-insensitive-terminal, non-terminal, and-predicate, not-predicate,
    capture, rule-kind {atomic, silent, token}).
  - `grammar_format_profile(format) -> Option<GrammarFormatProfile>`.
- The doc page `docs/grammar/fidelity.md`, modeled section-for-section on
  [`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md):
  entry point, capability profiles, per-format support matrix (✅ / ⚠️), documented
  fallbacks table, round-trip guarantee.
- A generating test that builds the matrix and the round-trip claims from the actual
  importers/emitters, so the doc is reproducible (and a stale doc fails CI).

**Out of scope** (owned elsewhere)
- The importers and emitters themselves → [`B1`](./B1-bnf-importer.md)–B7,
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md),
  [`C7`](./C7-tree-sitter-grammar-js-emitter.md); F2 only *measures* them and adds rows
  as they land.
- Cross-*notation translation through concepts* → [`C6`](./C6-concept-aligned-translation.md)
  (F2 covers same-format round trips and import↔emit fidelity; C6 owns the
  concept-layer translation matrix, analogous to the document story).
- The end-to-end runnable examples → [`E5`](./E5-end-to-end-integration-examples.md).
- The subsystem prose docs → [`F1`](./F1-grammar-subsystem-docs.md) (F1's
  `import-export.md` links here).

## Design / specification

### Three fidelity levels (the matrix cell values)

Mirroring the document matrix's ✅ / ⚠️ but with the grammar-specific third level:

| Level | Meaning | Cell |
|---|---|---|
| **lossless** | `emit(import(text))` is byte-identical to canonicalised input, and `import(emit(g))` equals `g`. | ✅ |
| **equivalent** | Round trip changes the *spelling* but the recovered `Grammar` accepts the same language (e.g. ABNF unordered choice ↔ IR `Choice{ordered:false}`); `import(emit(g))` is structurally equal modulo a documented normalisation. | ≈ |
| **lossy** | The format cannot represent the construct; it degrades through a **documented** fallback (e.g. BNF has no `*`, so `ZeroOrMore(x)` emits as a recursive helper rule `x_star ::= "" \| x x_star`). | ⚠️ |

Each ⚠️ cell has exactly one entry in the documented-fallback table — the invariant
the test enforces, identical in spirit to the document profile rule that every
concept is "either natively supported or has exactly one documented fallback"
([`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md) §Capability profiles).

### Capability profile API

```rust
/// Formats the grammar round-trip layer can import and emit (grows per B*/C*).
pub const GRAMMAR_FORMATS: &[&str] = &["bnf", /* ebnf, abnf, peg, gbnf, … as they land */];

/// The A1 GrammarExpr construct vocabulary classified by the matrix.
pub const GRAMMAR_CONSTRUCTS: &[&str] = &[ "sequence", "ordered-choice", /* … */ ];

/// Per-format capability profile over GRAMMAR_CONSTRUCTS; `None` for unknown formats.
pub fn grammar_format_profile(format: &str) -> Option<GrammarFormatProfile>;
```

`GrammarFormatProfile` reuses `LanguageProfile`
(`with_concept_fallback` at `src/language_profile.rs:167`, `concept_fallback` at
`:179`, `supports_concept` at `:191`) where the vocabulary fits; if grammar
constructs need their own enum it is a thin newtype carrying the same
`BTreeMap<String,String>` fallback table, so the "support-or-fallback" invariant and
its `Display`-able violation (`LanguageProfileViolation` at
`src/language_profile.rs:406`) are inherited rather than re-implemented.

### Doc page shape (`docs/grammar/fidelity.md`)

Section-for-section parallel to
[`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md):

1. **Intro** — one paragraph: import → IR → emit → re-import, and the
   lossless/equivalent/lossy contract (P-9, P-10).
2. **Entry point** — `grammar_format_profile(format)` and the round-trip functions
   it characterises (`import_bnf` ∘ `emit_bnf`, etc.); a runnable doctest like the
   document page's, asserting one lossless and one lossy example.
3. **Capability profiles** — the support-or-fallback invariant.
4. **Per-format construct support** — the matrix (constructs × formats, ✅ / ≈ / ⚠️).
5. **Documented lossy fallbacks** — format × construct × fallback strategy.
6. **Round-trip guarantee** — what the generating test proves and over which pairs.

The matrix and fallback tables are emitted by the generating test into a checked
form (see Tests), so the committed page is verifiably current.

### Per-importer / per-emitter coverage

The matrix rows reference each importer ([`B1`](./B1-bnf-importer.md),
[`B2`](./B2-ebnf-importer.md), [`B3`](./B3-abnf-importer.md),
[`B4`](./B4-peg-importer.md), [`B5`](./B5-tree-sitter-json-importer.md),
[`B6`](./B6-antlr-importer.md), [`B7`](./B7-lark-gbnf-importer.md)) and
emitter ([`C1`](./C1-bnf-ebnf-abnf-emitters.md), [`C2`](./C2-peg-emitter.md),
[`C3`](./C3-gbnf-emitter.md), [`C7`](./C7-tree-sitter-grammar-js-emitter.md)). At M2
only the [`B1`](./B1-bnf-importer.md)/[`C1`](./C1-bnf-ebnf-abnf-emitters.md) rows are
populated; the page and test are structured so a later issue adds a format by adding
its profile + fixtures, with no infrastructure change (each such issue's plan
already says "add the F2 row").

## File-level plan

| File | Change |
|---|---|
| `src/grammar/fidelity.rs` | New. `GRAMMAR_FORMATS`, `GRAMMAR_CONSTRUCTS`, `GrammarFormatProfile`, `grammar_format_profile`. |
| `src/grammar/mod.rs` | `pub mod fidelity;`. |
| `src/lib.rs` | Re-export `grammar_format_profile`, `GRAMMAR_FORMATS`, `GRAMMAR_CONSTRUCTS`. |
| `docs/grammar/fidelity.md` | New. The matrix doc (parallels `docs/cross-format-fidelity.md`). |
| `tests/unit/grammar_fidelity.rs` | New. Generating + invariant tests (see below). |
| `tests/unit/mod.rs` | Add `mod grammar_fidelity;`. |
| `tests/fixtures/grammar/roundtrip/` | New. Per-format fixtures exercising each construct (BNF first). |
| `README.md` | One line under §Parity Implementation (297) linking to `docs/grammar/fidelity.md`. |
| `changelog.d/` | Add a fragment. |

## Reuse

- **Document fidelity pattern (direct model):**
  [`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md);
  `LanguageProfile` (`src/language_profile.rs:15`), `with_concept_fallback`
  (`:167`), `concept_fallback` (`:179`), `supports_concept` (`:191`),
  `LanguageProfileViolation` (`:406`); `DOCUMENT_FORMATS` /
  `CROSS_FORMAT_CONCEPTS` / `document_format_profile`
  (`src/document_formatting/profile.rs:18`, `:23`, `:42`).
- **Generating-test pattern:**
  `tests/unit/cross_format_reconstruction.rs:112`
  (`every_ordered_format_pair_round_trips_through_the_concept_layer`) and `:204`
  (`every_format_profile_reports_support_or_a_fallback_for_each_concept`).
- **Parity-coverage precedent:** `src/parity.rs` /
  [`README.md`](../../../../README.md) §Parity Implementation:297-318
  (`GRAMMAR_EMBEDDING_TARGETS` etc. — capability scope as a tested API).
- The importers/emitters under test — [`B1`](./B1-bnf-importer.md) `import_bnf`,
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md) `emit_bnf`; [`A1`](./A1-grammar-ir.md)
  `Grammar`/`GrammarExpr` for structural-equality comparison.

## Acceptance criteria

- [ ] `grammar_format_profile(format)` returns a profile for every entry in
      `GRAMMAR_FORMATS`, listing each `GRAMMAR_CONSTRUCTS` member as natively
      supported **or** carrying exactly one documented fallback.
- [ ] `docs/grammar/fidelity.md` contains the per-format support matrix (constructs ×
      formats, ✅ / ≈ / ⚠️) and the documented-fallback table, structured like
      [`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md).
- [ ] For every format in `GRAMMAR_FORMATS` and every construct it marks **lossless**
      or **equivalent**, a fixture proves `import(emit(g))` recovers a `Grammar`
      structurally equal to `g` (modulo the documented normalisation for
      equivalent); every **lossy** construct exercises its documented fallback.
- [ ] The matrix and fallback tables in the doc are reproduced by the generating
      test (the test asserts the committed Markdown matches what it generates, so a
      stale doc fails CI — no external link/lint tool required).
- [ ] At least the [`B1`](./B1-bnf-importer.md)/[`C1`](./C1-bnf-ebnf-abnf-emitters.md)
      (BNF) row is fully populated and passing; adding a later format requires only a
      new profile + fixtures, not infrastructure changes.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml:103-106`), `cargo test
      --all-features`, and `cargo test --doc` (the fidelity-page doctest) pass;
      `rust-script scripts/check-no-src-tests.rs` passes (tests stay under `tests/`).

## Tests

Modeled on `tests/unit/cross_format_reconstruction.rs`:

- `every_grammar_format_profile_reports_support_or_a_fallback_for_each_construct`
  — for each `GRAMMAR_FORMATS` × `GRAMMAR_CONSTRUCTS`, assert native support xor a
  single documented fallback (parallels `:204`).
- `every_lossless_construct_round_trips_through_the_ir` — for each format, build a
  `Grammar` using its lossless/equivalent constructs, assert
  `import(emit(g))` is structurally equal to `g` (modulo documented normalisation)
  (parallels `:112`).
- `every_lossy_construct_uses_its_documented_fallback` — assert each ⚠️ construct
  emits the fallback recorded in the profile and re-imports to an equivalent
  language.
- `fidelity_doc_matrix_matches_generated_matrix` — generate the Markdown matrix +
  fallback tables from the profiles and assert the committed `docs/grammar/fidelity.md`
  contains them, keeping doc and code in lockstep.
- Doctest: the `docs/grammar/fidelity.md` entry-point example (one lossless, one
  lossy assertion) runs under `cargo test --doc`
  (`.github/workflows/release.yml:262`).

## References

- [`requirements.md`](../requirements.md) P-9, P-10;
  [`solution-plans.md`](../solution-plans.md) §Epic F (F2), §4 DAG (`B*/C* → F2`),
  §5 milestone M2.
- Model: [`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md),
  `docs/pdf-fidelity.md`, `docs/docx-fidelity.md`;
  `src/language_profile.rs:15,167,179,191,406`;
  `src/document_formatting/profile.rs:18,23,42`;
  `tests/unit/cross_format_reconstruction.rs:112,204`; `src/parity.rs` /
  [`README.md`](../../../../README.md) §Parity Implementation:297-318.
- Measured subsystems: [`A1`](./A1-grammar-ir.md), [`B1`](./B1-bnf-importer.md)–[`B7`](./B7-lark-gbnf-importer.md),
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md),
  [`C7`](./C7-tree-sitter-grammar-js-emitter.md); linked from
  [`F1`](./F1-grammar-subsystem-docs.md).
