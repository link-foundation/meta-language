# D8 — Semantic-constraint inference (ISLearn-style)

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`D5`](./D5-blackbox-cfg-inference.md) · **Blocks:** —
> **Requirements:** P-5 · **Milestone:** M4
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D (D8),
> [`literature-review.md`](../literature-review.md) §5,
> [`library-survey.md`](../library-survey.md) §C.2.

## Context

A context-free grammar fixes the *shape* of valid inputs but cannot express the
*semantic* constraints real languages carry: a name must be **defined before
use**, a **length field** must equal the byte count it prefixes, two repeated
sections must have **equal counts**, a checksum must match. Gold's negative
result and the whole positive-only line ([`literature-review.md`](../literature-review.md)
§0, §4) operate purely at the CFG level; the SOTA inference engines
([`D5`](./D5-blackbox-cfg-inference.md)) stop at context-free structure. Yet
requirement **P-5** asks for *"maximum freedom of grammar inference"* — which
[`requirements.md`](../requirements.md) reads as: the inferred representation
must be able to carry semantic constraints, not just context-free shape.

ISLa ([`literature-review.md`](../literature-review.md) §5) is the published
grammar-aware constraint language — first-order logic with quantifiers over
**derivation trees** plus SMT for atomic string/numeric predicates — and
**ISLearn** is its companion *learner*: it *mines* such invariants from example
inputs. This issue ports the ISLearn **mining pipeline** (augment → instantiate →
filter → DNF → rank) so that, given the CFG [`D5`](./D5-blackbox-cfg-inference.md)
infers and the positive corpus, we attach a layer of beyond-context-free
invariants. **ISLa and ISLearn are GPL-3.0** ([`library-survey.md`](../library-survey.md)
§C.2), so this is a **clean-room reimplementation from the paper**, not a port of
their code.

The codebase already owns the two pieces a constraint layer needs and would
otherwise have to invent: a **many-valued / probabilistic truth value**
(`src/semantics.rs:3` `TruthValue`, `:106` `ProbabilisticTruthValue`, re-exported
at `src/lib.rs:87`) for representing whether an invariant holds, and a
**structural query engine over the links network** (`src/query.rs:7` `LinkQuery`,
`src/query_algebra.rs:19` `LinkRule`, re-exported at `src/lib.rs:74-82`) for
matching the quantified tree patterns an invariant ranges over. The derivation
trees produced by [`D5`](./D5-blackbox-cfg-inference.md) are stored as links
(per [`A1`](./A1-grammar-ir.md)), so the same query machinery the rest of the
crate uses applies directly.

## Goal

Given an inferred `Grammar` (the [`A1`](./A1-grammar-ir.md) IR from
[`D5`](./D5-blackbox-cfg-inference.md)) and the positive example corpus, **mine a
set of semantic invariants** that hold across the corpus and express constraints
a CFG cannot — def-before-use, length fields, equal counts — represented as a
ranked **disjunctive normal form (DNF)** of grammar-aware atoms, evaluated with
the existing `TruthValue` semantics and matched with the existing `LinkQuery`
over derivation trees. The invariants attach to the grammar (a new
`SemanticConstraint` carried alongside the rules) so downstream consumers can
check or generate inputs that respect them.

## Scope

**In scope**
- A new module `src/grammar/infer/semantic.rs` (under the inference module
  [`D5`](./D5-blackbox-cfg-inference.md) introduces at `src/grammar/infer/`).
- A **pattern catalog** of invariant *templates* (def-before-use, equal-count,
  length-field, ordered/uniqueness) with named placeholders for non-terminals.
- The five-stage ISLearn pipeline: **example augmentation** (grammar + k-path
  mutation), **instantiation** (bind catalog placeholders to the inferred
  grammar's non-terminals), **filtering** (drop instantiations that fail on any
  positive/augmented sample), **DNF combination**, and **ranking** by specificity
  and recall.
- A `SemanticConstraint` value type + a `mine_semantic_constraints(...)` entry
  point that returns the ranked DNF.
- Evaluation of each atom over a derivation tree via `LinkQuery`/`LinkRule`,
  yielding a `TruthValue`.

**Out of scope** (owned elsewhere)
- The CFG itself and the derivation trees it parses → [`D5`](./D5-blackbox-cfg-inference.md).
- A full SMT solver. D8 ships a small, **closed set of atomic predicate
  evaluators** (string equality, count equality, prefix/length, membership,
  ordering) implemented directly in Rust; a pluggable SMT backend is a noted
  follow-up, not a blocker (record the seam, do not implement it here).
- LLM assistance for proposing or naming constraints → [`D9`](./D9-llm-assisted-naming-merge.md)
  (optional accelerator; D8 must work fully without it).
- Surface syntax for *authoring* constraints by hand → out of scope for #93
  (note it; the IR is enough for the inference deliverable).
- A links encoding of `SemanticConstraint` for full round-trip persistence may be
  added in a follow-up; here it is enough that constraints attach to the in-memory
  grammar and serialise via `Debug`/`serde` (match A1's existing derive policy).

## Design / specification

### Types and signatures

```rust
/// A grammar-aware atomic predicate over derivation trees, evaluated with the
/// existing `TruthValue` semantics. Each atom ranges over bindings produced by a
/// `LinkQuery`/`LinkRule` match against the derivation tree (stored as links).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintAtom {
    /// Every value matched at `use_path` must equal some value matched earlier
    /// (in document order) at `def_path`. Models def-before-use.
    DefBeforeUse { def: NonTerminalRef, use_: NonTerminalRef },
    /// The number of `left` matches equals the number of `right` matches.
    EqualCount { left: NonTerminalRef, right: NonTerminalRef },
    /// The integer value parsed at `field` equals the length (in elements or
    /// bytes, per `unit`) of the sibling subtree at `body`.
    LengthField { field: NonTerminalRef, body: NonTerminalRef, unit: LengthUnit },
    /// All values matched at `target` are pairwise distinct.
    Unique { target: NonTerminalRef },
    /// Values matched at `target` are non-decreasing in document order.
    Ordered { target: NonTerminalRef },
}

/// A reference into the inferred grammar plus the structural query used to
/// extract its instances from a derivation tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonTerminalRef {
    pub rule: String,            // a rule name in the inferred Grammar
    pub query: LinkQuery,        // reuse src/query.rs:7 to bind matches in a tree
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthUnit { Elements, Bytes, Chars }

/// One conjunctive clause: all atoms must hold (TruthValue::and over atoms).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintClause { pub atoms: Vec<ConstraintAtom> }

/// The mined invariant for a grammar: a DNF (OR of clauses), ranked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticConstraint {
    pub clauses: Vec<ConstraintClause>,   // disjunction; empty => trivially true
    pub specificity: u32,                 // ranking key (see §Ranking)
    pub recall: Probability,              // fraction of corpus satisfied (src/semantics.rs Probability)
}

/// A reusable invariant *template* with named placeholders for non-terminals.
pub struct ConstraintPattern {
    pub name: &'static str,
    /// Slots the instantiator binds to concrete non-terminals of the grammar.
    pub slots: &'static [&'static str],
    /// Builds candidate atoms from one binding of slots -> rule names.
    pub instantiate: fn(&BTreeMap<&'static str, String>) -> Vec<ConstraintAtom>,
}

/// The catalog of templates D8 ships (def-before-use, equal-count, length-field,
/// unique, ordered). Mirrors ISLearn's "pattern catalog".
pub fn default_pattern_catalog() -> Vec<ConstraintPattern>;

/// The pipeline entry point.
pub fn mine_semantic_constraints(
    grammar: &Grammar,                // inferred CFG (A1 IR, from D5)
    positive_examples: &[String],     // the same positive-only corpus D5 used (P-3)
    config: &SemanticInferenceConfig,
) -> SemanticConstraint;

#[derive(Clone, Debug)]
pub struct SemanticInferenceConfig {
    pub catalog: Vec<ConstraintPattern>, // default = default_pattern_catalog()
    pub k_path_depth: usize,             // k for k-path augmentation (default 3)
    pub max_augmented: usize,            // cap on generated augmented samples
    pub min_recall: Probability,         // discard clauses below this recall
}
```

### Pipeline (clean-room from ISLearn — [`literature-review.md`](../literature-review.md) §5)

The five stages exactly mirror ISLearn's *augment → instantiate → filter → DNF →
rank* described in [`library-survey.md`](../library-survey.md) §C.2 (do **not**
read ISLearn's GPL source; implement from this description and the paper).

1. **Parse the corpus into derivation trees.** For each positive example, parse
   it with the inferred `Grammar` (via the runtime parser of
   [`E2`](./E2-inferred-grammar-runtime-parser.md), or D5's internal parser if E2
   is not yet wired) to obtain a derivation tree stored as links
   ([`A1`](./A1-grammar-ir.md)). These trees are the universe every quantifier
   ranges over.

2. **Augment examples** (grammar + k-path mutation). To gain *negative-ish*
   discriminating power without requiring labelled negatives (P-3 stays
   positive-only), generate **variants** of each tree by mutating subtrees the
   grammar permits, covering each *k-path* (root-to-leaf path of length ≤ `k`)
   through the grammar at least once. Variants that change a value an invariant
   should constrain (e.g. renaming a defined symbol but not its use) become the
   evidence that *filters out* over-general patterns. Cap at `max_augmented`.
   *(This is the same role GLADE/Arvada's oracle plays — here the augmentation +
   the corpus itself act as the discriminator.)*

3. **Instantiate the pattern catalog.** For each `ConstraintPattern`, enumerate
   bindings of its slots to the grammar's non-terminals (`grammar.rule_names()`
   from [`A1`](./A1-grammar-ir.md)), build the candidate `ConstraintAtom`s, and
   attach the `LinkQuery` that extracts each slot's instances from a tree. Prune
   bindings that are structurally impossible (e.g. a `LengthField` whose `field`
   never appears as a sibling of `body` in any tree — checked cheaply with
   `LinkQuery`).

4. **Filter.** Evaluate every candidate atom over **every** parsed positive tree
   and every augmented variant. An atom is **kept** iff it evaluates to
   `TruthValue::True` (or `Both` treated per §Truth handling) on all positive
   trees — i.e. it is consistent with the corpus — *and* it is **falsified by at
   least one augmented variant** (i.e. it is non-trivial / discriminating).
   Discard the rest. This is ISLearn's "filter patterns that don't hold across
   samples" step, extended with the discrimination check so we do not keep
   vacuous invariants.

5. **Combine into DNF + rank.** Group surviving atoms into conjunctive clauses
   (atoms that always co-occur on the same trees), OR the clauses into a
   `SemanticConstraint` (DNF), then **rank**:
   - **specificity** = number of trees the invariant *constrains* (atoms that
     touch more nodes / longer k-paths score higher) — prefer tighter invariants;
   - **recall** = fraction of the positive corpus the invariant is satisfied by
     (it should be all of it after filtering, so recall breaks ties via the
     augmented set: prefer invariants satisfied by *fewer* augmented mutants,
     i.e. more discriminating). Compute `recall` as a `Probability`
     (`src/semantics.rs` `Probability::from_ratio`).
   Drop any clause below `config.min_recall`.

### Atom evaluation (reuse the query engine + truth semantics)

Each atom is evaluated against a single derivation tree, returning a
`TruthValue`:

- Build the slot bindings by running the atom's `NonTerminalRef::query`
  (`LinkQuery`) against the tree via the network's existing matching path
  (`src/query.rs`), or compose multi-slot matches with `LinkRule`
  (`src/query_algebra.rs:171` `LinkRule::matches`).
- `DefBeforeUse`: collect ordered `def` values and `use_` values; the atom is
  `True` iff every `use_` value appears in the set of `def` values seen earlier
  in document order, else `False`; `Unknown` if a slot produced no matches.
- `EqualCount`: `True` iff `|left matches| == |right matches|`.
- `LengthField`: parse the `field` match as an integer; `True` iff it equals the
  measured length of `body` in the chosen `unit`; `False`/`Unknown` on parse
  failure.
- `Unique`/`Ordered`: standard set / monotonicity checks over the matched values.
- A clause's truth is the `TruthValue::and`-fold over its atoms
  (`src/semantics.rs:13`); the constraint's truth over a tree is the
  `TruthValue::or`-fold over clauses (`:24`). For corpora that warrant
  confidence weighting, the same logic carries over to
  `ProbabilisticTruthValue` (`src/semantics.rs:106`) — expose a
  `evaluate_probabilistic(...)` variant returning `ProbabilisticTruthValue`.

### Truth handling

`TruthValue::Both` (paraconsistent) only arises if a tree both satisfies and
contradicts an atom under different bindings; treat `Both` as *not kept* during
filtering (conservative), and document that choice. `Unknown` (no match) does not
falsify an invariant on its own — an invariant about a construct absent from a
given input is vacuously satisfied there.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/infer/semantic.rs` | New. `ConstraintAtom`, `NonTerminalRef`, `ConstraintClause`, `SemanticConstraint`, `ConstraintPattern`, `SemanticInferenceConfig`, `default_pattern_catalog`, `mine_semantic_constraints`, atom evaluators. |
| `src/grammar/infer/mod.rs` | Add `pub mod semantic;` and re-export the public items (module created by [`D5`](./D5-blackbox-cfg-inference.md); if D8 lands first, create it). |
| `src/grammar/mod.rs` | Optional: a `Grammar::with_semantic_constraint(SemanticConstraint)` accessor so a mined invariant attaches to the grammar value ([`A1`](./A1-grammar-ir.md) owns `Grammar`; add the field/accessor in coordination, behind the same `PartialEq`/`Debug` derives). |
| `src/lib.rs` | `pub use grammar::infer::semantic::{SemanticConstraint, ConstraintAtom, ConstraintClause, mine_semantic_constraints};` next to the other grammar re-exports. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_infer_semantic` test modules. |
| `tests/fixtures/grammar/semantic/` | Small corpora that *carry* a constraint: a "define then reference" mini-language (def-before-use), a TLV/length-prefixed format (length field), a paired-delimiter format (equal counts). |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- `TruthValue` / `ProbabilisticTruthValue` / `Probability` — invariant truth and
  recall scoring (`src/semantics.rs:3,106,51`, re-exported `src/lib.rs:87`). Use
  `TruthValue::and`/`or` (`:13`/`:24`) to fold clauses; `Probability::from_ratio`
  for recall.
- `LinkQuery` (`src/query.rs:7`, re-exported `src/lib.rs:74`) and `LinkRule`
  (`src/query_algebra.rs:19`, `matches` at `:171`, re-exported `src/lib.rs:78-82`)
  — match the quantified tree patterns each atom ranges over. Reuse
  `LinkQuery::from_sexpression` (`src/query.rs:34`) so catalog templates can carry
  human-readable query strings.
- [`A1`](./A1-grammar-ir.md) `Grammar` (`rule_names()`, `referenced_nonterminals()`)
  and its links encoding (derivation trees and the grammar are both links).
- [`D5`](./D5-blackbox-cfg-inference.md) supplies the inferred `Grammar` and the
  derivation trees; D8 consumes them and the same positive corpus.
- [`E2`](./E2-inferred-grammar-runtime-parser.md) to re-parse examples into trees
  if D5's internal trees are not exposed.
- **Do not** add an SMT or GPL dependency; the atomic predicates are
  hand-implemented (clean-room — ISLa/ISLearn are GPL, see
  [`library-survey.md`](../library-survey.md) §C.2).

## Acceptance criteria

- [ ] `mine_semantic_constraints` returns, for the def-before-use fixture, a
      `SemanticConstraint` whose DNF contains a `DefBeforeUse` atom over the right
      non-terminals, and **rejects** an input that uses an undefined symbol when
      that input is checked against the constraint.
- [ ] For the length-prefix fixture it mines a `LengthField` atom; for the
      paired-delimiter fixture an `EqualCount` atom.
- [ ] An over-general candidate (one that holds on the positive corpus but is not
      discriminated by any augmented mutant) is **filtered out** (not present in
      the result) — verified by a unit test.
- [ ] Atom evaluation returns `TruthValue` and folds clauses with
      `TruthValue::and`/`or`; a `evaluate_probabilistic` path returns
      `ProbabilisticTruthValue`.
- [ ] Ranking is deterministic: the same corpus + catalog yields the same ordered
      DNF on repeated runs (no RNG, or a fixed seed if augmentation samples).
- [ ] The pipeline runs with **no LLM and no SMT solver** (P-7 determinism; the
      LLM accelerator is [`D9`](./D9-llm-assisted-naming-merge.md) and stays
      optional).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:105-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live
      under `tests/`, not `src/`).

## Tests

- Unit (`tests/unit/`, new `grammar_infer_semantic` module):
  - **catalog instantiation**: a tiny hand-built `Grammar` ([`A1`](./A1-grammar-ir.md)
    builder) yields the expected candidate atoms for each template.
  - **atom evaluation**: build derivation trees by hand (or parse fixtures) and
    assert each atom returns the right `TruthValue` on satisfying and violating
    trees, including `Unknown` for absent constructs and the `Both` handling.
  - **filtering**: a candidate that holds on the positive set but is undiscriminated
    is dropped; one that is discriminated by an augmented mutant is kept.
  - **DNF + ranking**: two co-occurring atoms collapse into one clause; ranking is
    stable and respects `min_recall`.
- Integration (`tests/integration/`):
  - end-to-end on each `tests/fixtures/grammar/semantic/` corpus: infer a CFG
    (stub D5 with a hand-built grammar if D5 is not yet merged), mine constraints,
    then check that a held-out *valid* input satisfies the DNF and a *crafted
    invalid* input (undefined use / wrong length / mismatched counts) does not.
  - recall is reported as a `Probability` and equals 1.0 on the positive corpus.
- No network/IO beyond reading fixtures; deterministic.

## References

- Steinhöfel & Zeller, "Input Invariants (ISLa)," ESEC/FSE 2022
  ([arXiv 2208.12049](https://arxiv.org/abs/2208.12049)) — FOL over derivation
  trees + SMT (GPL; **clean-room from the paper**).
- ISLearn (companion learner) — augment → instantiate → filter → DNF → rank;
  pattern catalog; ranking by specificity & recall ([github.com/rindPHI/islearn](https://github.com/rindPHI/islearn),
  GPL; **clean-room**). See [`library-survey.md`](../library-survey.md) §C.2.
- "Passive Model Learning of Visibly Deterministic Context-free Grammars,"
  [arXiv 2508.16305](https://arxiv.org/pdf/2508.16305), 2025 — the regular↔CFG
  bridge for delimiter-structured inputs.
- [`literature-review.md`](../literature-review.md) §5 (constraint/semantic
  inference, P-5 takeaway), [`solution-plans.md`](../solution-plans.md) §Epic D
  (D8), [`A1`](./A1-grammar-ir.md), [`D5`](./D5-blackbox-cfg-inference.md),
  [`D9`](./D9-llm-assisted-naming-merge.md).
