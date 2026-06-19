# D3 — State-merging regular inference (RPNI / EDSM)

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`D1`](./D1-inference-evaluation-harness.md) · **Blocks:** (feeds [`D5`](./D5-blackbox-cfg-inference.md))
> **Requirements:** P-2 · **Milestone:** M3
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D & §3 (row D3),
> [`literature-review.md`](../literature-review.md) §2.

## Context

Many sublanguages of a programming language are **regular**, not context-free:
token classes, comment/string bodies, number formats, identifier rules. The
classical, well-understood way to learn a regular language from labelled examples
is **state merging**: build a prefix-tree acceptor from the samples and greedily
merge states while preserving determinism and consistency. RPNI (Oncina & García
1992) is the canonical passive learner; EDSM (Abbadingo, Lang et al. 1998) is its
standard high-accuracy upgrade; ALERGIA (Carrasco & Oncina 1994) is the
*stochastic* variant that enables positive-only learning by merging on
statistical frequency compatibility ([`literature-review.md`](../literature-review.md)
§2). This issue ports those algorithms into Rust **clean-room from the papers and
permissive references**, extracting the learned automaton into a regular
[`A1`](./A1-grammar-ir.md) grammar so the regular layer feeds the CFG layer
([`D5`](./D5-blackbox-cfg-inference.md)).

**Licence note (mandatory).** RPNI/EDSM/ALERGIA must be implemented **clean-room**
from de la Higuera, *Grammatical Inference* (2010), and only from the
**permissive** reference implementations — **Apache-2.0 LearnLib** (passive RPNI)
and **MIT GIToolbox** (RPNI/EDSM/ALERGIA in MATLAB)
([`library-survey.md`](../library-survey.md) §C.3, §C.5). **Do not** read, copy,
link, or vendor **GPL flexfringe** or **LGPL libalf**
([`library-survey.md`](../library-survey.md) §C.3); flexfringe may only ever be
run as an *external* benchmark oracle (its GPL covers the tool, not our data),
never built against. State this in the PR description and record the clean-room
provenance.

## Goal

Provide a Rust state-merging regular-inference engine: build an (augmented)
prefix-tree acceptor from labelled (or positive-only stochastic) samples, perform
**RPNI** ordered merges and **EDSM** evidence-driven merges (and **optional
ALERGIA** stochastic merges) while preserving determinism/consistency, then
**extract a regular grammar** into the [`A1`](./A1-grammar-ir.md) IR
(right-linear rules). Scored with [`D1`](./D1-inference-evaluation-harness.md).

## Scope

**In scope**
- A new module `src/grammar/inference/state_merging.rs` (under the `inference::`
  namespace from [`D1`](./D1-inference-evaluation-harness.md)).
- `Sample` input type (positive + optional negative strings, over a token/char
  alphabet — accepts output of [`D2`](./D2-lexical-class-inference.md)).
- **(A)PTA** construction.
- **RPNI** (ordered blue-fringe merges, determinism via recursive folding).
- **EDSM** (score candidate merges by overlapping accept/reject evidence; merge
  the highest-scoring consistent pair).
- **ALERGIA** (optional, stochastic): Hoeffding-bound compatibility test on
  symbol/final-state frequencies; produces a probabilistic automaton.
- **Extraction** of the merged DFA/PDFA into a right-linear [`A1`](./A1-grammar-ir.md)
  grammar.

**Out of scope** (owned elsewhere)
- Context-free / hierarchical structure → [`D4`](./D4-sequitur-compression.md),
  [`D5`](./D5-blackbox-cfg-inference.md).
- Active learning with a membership/equivalence oracle (L\*/TTT) →
  [`D10`](./D10-active-learning-oracle.md).
- Tokenisation / lexical classes (the alphabet) →
  [`D2`](./D2-lexical-class-inference.md).
- Metric scoring → [`D1`](./D1-inference-evaluation-harness.md).
- Any use of GPL flexfringe / LGPL libalf as a dependency → forbidden (see
  licence note).

## Design / specification

### Sample & alphabet

```rust
/// A labelled example over an alphabet of symbols `S` (S = String token or char).
pub struct Sample {
    pub positives: Vec<Vec<Symbol>>,   // strings in the language (S+)
    pub negatives: Vec<Vec<Symbol>>,   // strings not in the language (S−); may be empty
}
pub type Symbol = String;   // a token from D2, or a single-char string
```

Positive-only operation (P-3) is supported in two ways: (a) RPNI/EDSM with an
*empty* negative set degenerate to "merge whenever determinism allows" (which
over-generalises — Gold 1967, [`literature-review.md`](../literature-review.md)
§0 — so it is offered only behind the stochastic ALERGIA path or with a structural
stop), and (b) **ALERGIA**, the principled positive-only learner, which merges on
statistical compatibility instead of needing negatives.

### Step 1 — (Augmented) Prefix Tree Acceptor

Build a tree-shaped DFA where the path spelling each positive sample ends in an
**accepting** state and (APTA) each negative sample ends in a **rejecting** state;
shared prefixes share states. For the stochastic case, annotate each state with
the **count** of samples passing through it and the count ending there (final
frequency) — these feed ALERGIA.

```rust
struct Apta {
    states: Vec<AptaState>,                 // index = state id; 0 = root
    transitions: Vec<BTreeMap<Symbol, usize>>, // per-state symbol → next state
}
struct AptaState { accepting: bool, rejecting: bool, arrival_count: u64, final_count: u64 }
```

### Step 2 — RPNI (Regular Positive and Negative Inference)

Oncina & García 1992 ([`literature-review.md`](../literature-review.md) §2):
1. Fix the canonical (length-lexicographic) order of states from the APTA.
2. Maintain a **red** set (confirmed representatives, initially `{root}`) and a
   **blue** fringe (immediate non-red successors of red states).
3. For each blue state `b` in order, try to **merge** `b` into each red state `r`
   in order. A merge is tentatively applied, then made deterministic by
   **recursively folding** any resulting non-deterministic transitions
   (determinisation by merging target states).
4. Accept the merge iff it stays **consistent** — no accepting state becomes
   equal to a rejecting state (no positive/negative collision). If no red merge is
   consistent, **promote** `b` to red.
5. Repeat until the blue fringe is empty.

### Step 3 — EDSM (Evidence-Driven State Merging)

Lang, Pearlmutter & Price 1998 (Abbadingo,
[`literature-review.md`](../literature-review.md) §2): same red/blue framework,
but instead of taking the first consistent merge, **score** every candidate
(red, blue) merge by the **evidence** — the number of overlapping
accept/accept and reject/reject labels witnessed during the determinising fold —
and perform the **highest-scoring consistent merge** first; promote a blue state
to red only when it has no consistent merge with any red state. This is the
standard accuracy upgrade and the one E3 reports against for regular targets.

```rust
pub enum MergeStrategy { Rpni, Edsm, Alergia { alpha: f64 } }
pub fn infer_dfa(sample: &Sample, strategy: MergeStrategy) -> InferredAutomaton;
```

### Step 4 — ALERGIA (optional, stochastic — positive-only)

Carrasco & Oncina 1994 ([`literature-review.md`](../literature-review.md) §2):
merge two states when their outgoing-symbol and final-state **relative
frequencies are statistically compatible** under a **Hoeffding bound**: two
observed proportions `p̂₁ = f₁/n₁`, `p̂₂ = f₂/n₂` are deemed compatible at
confidence `α` when

```text
| f₁/n₁ − f₂/n₂ |  <  sqrt( ½ · ln(2/α) ) · ( 1/sqrt(n₁) + 1/sqrt(n₂) )
```

Merge recursively (as in RPNI) only when *all* corresponding frequencies are
compatible; the result is a **probabilistic** automaton (PDFA). Represent the
emission/final probabilities with the crate's existing fixed-point
`Probability` / `ProbabilisticTruthValue` (`src/semantics.rs:51-156`), not raw
`f64` fields, so the stochastic grammar's confidences are consistent with the
relative-meta-logic semantics used elsewhere.

### Step 5 — Extract a regular grammar into the A1 IR

Convert the merged (P)DFA to a **right-linear** grammar
([`A1`](./A1-grammar-ir.md)):
- One `GrammarRule` (`NonTerminal`) per DFA state `q`; the start rule is the
  initial state.
- For each transition `q --a--> q'`: add an alternative `Sequence([Terminal(a), NonTerminal(q')])`
  to `q`'s rule (`Choice { ordered: false, .. }`).
- For each accepting state `q`: add the `Empty` alternative to `q`'s rule.
- For ALERGIA's PDFA, carry each alternative's `Probability` on the produced rule
  node (via the rule's `concept`/doc channel or a parallel weights map — keep the
  pure-structural `Grammar` unchanged and attach weights alongside).

```rust
pub struct InferredAutomaton { /* states, transitions, optional weights */ }
impl InferredAutomaton { pub fn to_grammar(&self) -> Grammar; }
```

The resulting `Grammar` is scored with [`D1`](./D1-inference-evaluation-harness.md)
(`evaluate`) against the golden oracle.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | Add `pub mod state_merging;` (create the module if [`D1`](./D1-inference-evaluation-harness.md) has not). |
| `src/grammar/inference/state_merging.rs` | New. `Symbol`, `Sample`, `Apta`, `MergeStrategy`, `InferredAutomaton`, `infer_dfa`, `to_grammar`, RPNI/EDSM/ALERGIA logic, the Hoeffding test. |
| `src/lib.rs` | Add `pub use grammar::inference::state_merging::{Sample, MergeStrategy, InferredAutomaton, infer_dfa};`. |
| `tests/unit/mod.rs` | Register a new `inference_state_merging` unit-test module. |
| `changelog.d/` | Add a fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- **[`A1`](./A1-grammar-ir.md) IR** — `Grammar`, `GrammarExpr::{Terminal, NonTerminal, Sequence, Choice, Empty}`,
  `GrammarRule`, builder — the extraction target; reuse `Grammar::with_rule`/`set_start`.
- **`src/semantics.rs` `Probability` / `ProbabilisticTruthValue`** (`:51`, `:106`;
  re-exported `src/lib.rs` `ProbabilisticTruthValue, Probability, TruthValue`) for
  the ALERGIA stochastic case — use `Probability::from_ratio(f, n)` (`:76`) for the
  observed proportions and carry `ProbabilisticTruthValue` confidences.
- **[`D1`](./D1-inference-evaluation-harness.md)** `evaluate` / `MembershipOracle`
  for scoring; **[`D2`](./D2-lexical-class-inference.md)** `LexicalModel::tokenize`
  to turn raw example texts into the `Symbol` sequences `Sample` expects.
- **`std::collections::BTreeMap`** for deterministic, ordered transition maps
  (canonical state order is required for RPNI reproducibility — no `HashMap`).
- **Clean-room sources only:** de la Higuera 2010, Apache LearnLib (RPNI), MIT
  GIToolbox (RPNI/EDSM/ALERGIA) — [`library-survey.md`](../library-survey.md)
  §C.3, §C.5. **Never** flexfringe (GPL) / libalf (LGPL).

## Acceptance criteria

- [ ] `Sample`, `MergeStrategy`, `InferredAutomaton`, `infer_dfa`, and
      `InferredAutomaton::to_grammar` are public and documented (crate denies
      missing docs if configured — check `src/lib.rs`).
- [ ] APTA construction is correct: shared prefixes share states; positives end
      accepting, negatives end rejecting.
- [ ] **RPNI** on a textbook labelled sample (e.g. learn `(ab)*` from positives
      `{ε, ab, abab}` and negatives `{a, b, aba}`) yields a DFA that accepts all
      positives and rejects all negatives; the extracted `Grammar` does too
      (verified via [`D1`](./D1-inference-evaluation-harness.md) — recall 1.0 on
      positives, rejects negatives).
- [ ] **EDSM** produces an automaton at least as accurate as RPNI on the same
      sample and merges by descending evidence score (assert the chosen merge
      order on a crafted case).
- [ ] **ALERGIA** (optional) merges frequency-compatible states under the
      documented Hoeffding bound and yields a PDFA whose alternative
      probabilities use `Probability`/`ProbabilisticTruthValue`; lowering `alpha`
      makes merging stricter (fewer merges) — assert the monotonic effect.
- [ ] Determinism: a fixed `Sample` + `MergeStrategy` yields a byte-identical
      `InferredAutomaton` and `Grammar` (canonical state ordering; no `HashMap`
      iteration in the merge path).
- [ ] PR description records **clean-room provenance** (de la Higuera /
      LearnLib / GIToolbox) and explicitly states flexfringe/libalf were not used.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic /
      nursery `warn` per `Cargo.toml` `[lints.clippy]`), `cargo test --all-features`
      pass; `rust-script scripts/check-no-src-tests.rs` passes (tests under
      `tests/`).

## Tests

- `tests/unit/` (new `inference_state_merging` module):
  - APTA shape on a tiny sample (assert state/transition counts).
  - RPNI learns `(ab)*` (and a second regular language, e.g. "even number of
    `a`s") from labelled samples; assert accept/reject on held-out strings.
  - EDSM accuracy ≥ RPNI on the same sample; assert highest-evidence merge chosen.
  - ALERGIA: a stochastic sample where two states have compatible frequencies
    merges; tightening `alpha` prevents it; probabilities are valid
    `Probability` values.
  - Extraction: `to_grammar` round-trips acceptance with the automaton (a string
    is accepted by the DFA iff the extracted `Grammar` accepts it, via
    [`D1`](./D1-inference-evaluation-harness.md)).
  - Determinism: repeated runs are byte-identical.
- Pure in-process, no network. Fixtures inline or under
  `tests/fixtures/grammar/regular/`.

## References

- Oncina & García, "Inferring Regular Languages in Polynomial Updated Time"
  (RPNI), 1992 — [`literature-review.md`](../literature-review.md) §2.
- Lang, Pearlmutter & Price, "Results of the Abbadingo One DFA Learning
  Competition … EDSM," ICGI 1998 — [`literature-review.md`](../literature-review.md)
  §2.
- Carrasco & Oncina, "Learning Stochastic Regular Grammars … ALERGIA," ICGI 1994
  — the Hoeffding-bound merge test; [`literature-review.md`](../literature-review.md)
  §2.
- de la Higuera, *Grammatical Inference: Learning Automata and Grammars*,
  Cambridge University Press, 2010 — the canonical clean-room source for PTA /
  RPNI / EDSM / ALERGIA; [`literature-review.md`](../literature-review.md) §0, §2.
- Licence hygiene & clean-room sources: [`library-survey.md`](../library-survey.md)
  §C.3 (LearnLib Apache / libalf LGPL / flexfringe GPL), §C.5 (GIToolbox MIT);
  [`solution-plans.md`](../solution-plans.md) §3 row D3 (Addresses P-2).
