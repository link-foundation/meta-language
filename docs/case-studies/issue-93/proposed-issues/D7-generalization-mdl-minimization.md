# D7 — Generalization & MDL/Occam minimization

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`D5`](./D5-blackbox-cfg-inference.md) · **Blocks:** —
> **Requirements:** P-3, P-5 · **Milestone:** M4
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D (§4 DAG — D7 is on the
> critical path `A1 → D6 → D5 → {D7,E3,E5}`),
> [`literature-review.md`](../literature-review.md) §3, §4,
> [`competitive-analysis.md`](../competitive-analysis.md) §4.

## Context

[`D5`](./D5-blackbox-cfg-inference.md) emits a grammar that is **correct on the
positive examples but over-fit**: its bubble-and-merge fixpoint leaves many narrow,
single-use rules and placeholder non-terminals (`n0`, `n1`, …). That is by design —
D5 guarantees recall on the training set, not compactness. To clear the P-12 bar
against NatGI ([`competitive-analysis.md`](../competitive-analysis.md) §2–§3) the
grammar must become **compact, general, and readable** without losing recall or
blowing precision open.

The literature names the tool. **Stolcke & Omohundro, "Inducing Probabilistic
Grammars by Bayesian Model Merging" (ICGI 1994)** gives the principled objective:
an **MDL/Bayesian prior balancing data fit against grammar size**, the formal basis
for an Occam preference for small grammars
([`literature-review.md`](../literature-review.md) §3). And **NatGI's third
innovation is hierarchical delta debugging (HDD)** to *simplify parse trees* (§4);
[`competitive-analysis.md`](../competitive-analysis.md) §4 maps "hierarchical delta
debugging" onto "MDL/Occam minimization over the links IR — D7". This issue
delivers that minimiser. Gold (§0) is the through-line: positive-only inference
*needs* a bias, and the Occam preference for the smallest grammar that still fits is
precisely such a bias. D6 supplied the *structural* prior to D5; D7 supplies the
*size/simplicity* prior on the way out.

## Goal

Turn [`D5`](./D5-blackbox-cfg-inference.md)'s over-fit parse forest / grammar into a
**compact, general** A1 `Grammar` by (a) defining an **MDL cost function** (grammar
size + data encoding length), (b) running a **model-merging search** that greedily
applies size-reducing transformations whose cost decreases, and (c) using
**[`D1`](./D1-inference-evaluation-harness.md) metrics to gate** the result so the
minimiser never under- or over-generalises past published thresholds. Add an
HDD-style **rule-simplification** pass (prune, inline, factor) as the
delta-debugging analogue.

## Scope

**In scope**
- A new public module `src/grammar/inference/minimize.rs` (under the `inference`
  module shared with [`D5`](./D5-blackbox-cfg-inference.md)/[`D6`](./D6-delimiter-structural-prior.md)).
- The **MDL cost function** `mdl_cost(grammar, examples) -> Mdl` (grammar bits +
  data bits), with a documented, deterministic encoding.
- The **merge-search** `minimize(grammar, examples, opts) -> MinimizeResult`:
  greedy, cost-monotone model merging over the A1 IR.
- The **HDD-style simplification** transforms (unreachable-rule pruning,
  single-use-rule inlining, common-prefix/suffix factoring, alternation dedup).
- **D1 gating:** every accepted step must keep precision/recall/F1 within an
  explicit budget; reject steps that over-generalise (precision drop) and never
  under-generalise (recall must stay 1.0 on the training examples).

**Out of scope** (owned elsewhere)
- Producing the initial over-fit grammar → [`D5`](./D5-blackbox-cfg-inference.md).
- The delimiter structural prior → [`D6`](./D6-delimiter-structural-prior.md).
- Metric *definitions* and the sampler → [`D1`](./D1-inference-evaluation-harness.md)
  (D7 *calls* D1; it does not redefine F1/MDL).
- Semantic (beyond-CFG) constraints → D8. Concept-aligned naming → A3/D9
  (D7 keeps deterministic names; renaming is orthogonal).
- The IR, builder, links encoding → [`A1`](./A1-grammar-ir.md).
- Competitor corpora / CI gate → [`E3`](./E3-competitor-benchmark-suite.md).

## Design / specification

### The MDL cost function

The Occam objective is two-part description length (minimise the sum):

```
mdl_cost(G, D) = L(G) + L(D | G)
```

- **`L(G)` — grammar encoding (bits).** A deterministic encoding of the A1 IR: per
  `GrammarRule`, a fixed header cost plus a per-node cost over its expression tree
  (a fixed bit cost per `GrammarExpr` variant tag + terminals/char-classes by
  literal length + a non-terminal reference as `log2(num_rules)`). Smaller,
  more-reused grammars cost fewer bits — the Occam pressure
  ([`literature-review.md`](../literature-review.md) §3).
- **`L(D | G)` — data encoding (bits).** The cost of encoding the corpus *given* the
  grammar: per example, the bits to specify its derivation (`log2(#alternatives)` /
  repetition-count bits at each choice/repetition point). A *too tight* grammar (one
  rule per example) has cheap data bits but huge `L(G)`; a *too loose* one has cheap
  `L(G)` but expensive, ambiguous derivations. The minimum of the sum is the
  Occam-optimal trade-off — the Stolcke–Omohundro objective.

```rust
/// Two-part Minimum Description Length cost (bits). Lower is better.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mdl {
    /// L(G): bits to encode the grammar itself.
    pub grammar_bits: f64,
    /// L(D | G): bits to encode the corpus given the grammar.
    pub data_bits: f64,
}
impl Mdl { #[must_use] pub fn total(self) -> f64 { self.grammar_bits + self.data_bits } }

/// Deterministic MDL cost of `grammar` on `examples`.
/// The bit-cost constants are documented module-level and never RNG/locale-derived.
pub fn mdl_cost(grammar: &Grammar, examples: &[String]) -> Mdl;
```

Determinism: the encoding is a pure function of the IR + corpus; all per-variant
costs are documented constants; the data-bits derivation uses the same
deterministic recogniser as [`D5`](./D5-blackbox-cfg-inference.md) (canonical
left-most derivation for ties). No randomness.

### The merge-search (named phases)

```
G0 (over-fit, from D5) ──▶ [P1 candidates] ──▶ [P2 score ΔMDL] ──▶ [P3 D1 gate] ──▶ [P4 apply best] ──▶ fixpoint ──▶ G*
                            (merges+HDD)         (mdl_cost)         (precision/recall)   (Occam-greedy)
```

**Phase 1 — Enumerate candidate transformations.** From the current grammar,
deterministically enumerate size-reducing candidates of three kinds (Stolcke–
Omohundro model merging + HDD simplification):
1. **Non-terminal merge** — unify two rules with structurally similar bodies
   (collapse `n3` and `n7` into one), the generalising step.
2. **Inline** — replace a single-use non-terminal by its body (HDD-style removal of
   a redundant level), the simplifying step.
3. **Factor** — pull a common prefix/suffix out of the alternatives of a `Choice`
   into a shared sub-rule, and dedup identical alternatives. Reduces `L(G)`.
Candidates are enumerated in a fixed total order (by rule index, then transform
kind) so the search is reproducible.

**Phase 2 — Score by ΔMDL.** For each candidate, build the trial grammar and
compute `mdl_cost(trial, examples).total() - mdl_cost(current, examples).total()`.
Keep only candidates with **ΔMDL < 0** (strictly reduce description length) — the
Occam acceptance test.

**Phase 3 — D1 gate (over-/under-generalisation guard).** A negative ΔMDL is
*necessary* but not *sufficient*: a merge can lower bits while quietly
over-generalising. So each surviving candidate is checked with
[`D1`](./D1-inference-evaluation-harness.md):
- **recall floor (no under-generalisation):** the trial grammar must still parse
  **every positive example** (recall on the training set stays 1.0) — reject
  otherwise. This preserves the P-3 correctness D5 established.
- **precision ceiling (no over-generalisation):** sample N strings from the trial
  grammar via [`D1`](./D1-inference-evaluation-harness.md); reject if the
  measured precision (against the oracle, or the sampling self-consistency proxy
  when positive-only) drops by more than `opts.precision_budget`.
Only candidates passing *both* gates remain admissible.

**Phase 4 — Apply best, iterate.** Apply the admissible candidate with the most
negative ΔMDL (ties broken by the fixed enumeration order — deterministic).
Re-enumerate and repeat until no admissible cost-reducing transformation remains
(**fixpoint**). The result `G*` is the Occam-minimal grammar reachable by greedy
model merging under the D1 gates.

```rust
#[derive(Clone, Copy, Debug)]
pub struct MinimizeOptions {
    /// Max precision drop tolerated by the D1 gate (default a documented constant).
    pub precision_budget: f64,
    /// D1 sampling budget for the precision check (default a constant).
    pub sample_budget: usize,
    /// Defensive iteration cap (default a documented constant).
    pub max_iterations: usize,
}

#[derive(Clone, Debug)]
pub struct MinimizeResult {
    pub grammar: Grammar,
    pub before: Mdl,
    pub after: Mdl,
    /// Transform counts for D1 reporting / E3 (no behaviour impact).
    pub report: MinimizeReport,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MinimizeReport {
    pub merges_applied: usize,
    pub inlines_applied: usize,
    pub factorings_applied: usize,
    pub candidates_rejected_by_mdl: usize,
    pub candidates_rejected_by_gate: usize,
}

/// Generalise + minimise an over-fit grammar under an MDL objective and D1 gates.
pub fn minimize(grammar: &Grammar, examples: &[String], opts: MinimizeOptions) -> MinimizeResult;
```

### Over-/under-generalisation, made precise

The crux of P-3/P-5 ("maximum freedom" must not mean "accept everything"):
- **Under-generalisation** (rejects valid inputs) is guarded absolutely — Phase 3's
  recall floor forbids dropping any positive example, so D7 can only *keep or widen*
  the accepted language relative to D5's output.
- **Over-generalisation** (accepts garbage) is guarded by the MDL objective (a wildly
  general grammar has cheap `L(G)` but expensive ambiguous `L(D|G)`, so ΔMDL turns
  positive and the step is rejected) *and* the D1 precision-budget gate. MDL is the
  soft Occam pressure; the D1 gate is the hard floor.

### Determinism notes

`minimize` is a pure function of `(grammar, examples, opts)`: candidate enumeration
and tie-breaking use fixed total orders (rule index, transform kind — no `HashMap`
iteration, RNG, or clock); `mdl_cost` is deterministic (constants + canonical
derivation); the D1 sampler is seeded by
[`D1`](./D1-inference-evaluation-harness.md) (D7 passes a fixed budget, not a seed).
A property test asserts `minimize(g, x, o) == minimize(g, x, o)` (structural
`PartialEq` after name canonicalisation) and `after.total() <= before.total()`.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/minimize.rs` | New. `Mdl`, `mdl_cost`, `MinimizeOptions`, `MinimizeResult`, `MinimizeReport`, `minimize`, and the private Phase 1–4 fns + transform helpers. |
| `src/grammar/inference/mod.rs` | Add `pub mod minimize;` (module created by [`D5`](./D5-blackbox-cfg-inference.md)/[`D6`](./D6-delimiter-structural-prior.md)). |
| `src/lib.rs` | `pub use grammar::inference::minimize::{minimize, mdl_cost, Mdl, MinimizeOptions, MinimizeResult};` next to the D5/D6 re-exports. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_minimize` unit + integration modules. |
| `tests/fixtures/grammar/inference/` | Reuse [`D5`](./D5-blackbox-cfg-inference.md)'s corpora; add an intentionally over-fit grammar fixture to minimise directly. |
| `changelog.d/` | Fragment (`scripts/create-changelog-fragment.rs`). |

## Reuse

- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/builder — the IR the transforms
  rewrite; `referenced_nonterminals()` for reachability (prune/inline) and
  reference-rewriting on merge.
- [`D5`](./D5-blackbox-cfg-inference.md) `InferenceResult.grammar` (input) and its
  built-in recogniser (`src/grammar/inference/recognizer.rs`) for the recall floor —
  reuse, do not re-implement.
- [`D1`](./D1-inference-evaluation-harness.md) sampler + precision/recall/F1/MDL —
  the Phase 3 gate and the MDL definitions D7 must not diverge from (`mdl_cost`
  should match D1's grammar-size scoring; coordinate the encoding).
- Stolcke & Omohundro Bayesian model merging and NatGI's HDD — ported from the
  papers (algorithms are free; [`literature-review.md`](../literature-review.md) §3,
  §4). No GPL code vendored; no new third-party dependency (pure analysis over A1).

## Acceptance criteria

- [ ] `mdl_cost` returns a two-part `Mdl` whose `total()` strictly *decreases* when
      a many-rule over-fit grammar is replaced by an equivalent compact one (assert
      on a hand-built over-fit/compact pair for the same language).
- [ ] `minimize` reduces rule count and `Mdl.total()` on [`D5`](./D5-blackbox-cfg-inference.md)'s
      output for every fixture corpus, with `after.total() <= before.total()`.
- [ ] **Recall floor:** the minimised grammar still parses **100% of the positive
      examples** (no under-generalisation) — asserted via
      [`D1`](./D1-inference-evaluation-harness.md) on every fixture.
- [ ] **Precision ceiling:** measured precision after minimisation does not drop by
      more than `opts.precision_budget` vs the input grammar — a candidate that
      would over-generalise is rejected (assert the rejection path is exercised).
- [ ] Recursion is preserved: minimising the recursive-list grammar keeps a
      self-referential rule (Occam merging must not destroy needed recursion).
- [ ] **D1 metrics on a small corpus:** on the bundled arithmetic + JSON-subset
      corpora, precision/recall/F1 before *and* after minimisation are recorded and
      F1 is non-decreasing (the absolute competitor bar lives in
      [`E3`](./E3-competitor-benchmark-suite.md)).
- [ ] **Determinism:** `minimize(g, x, o)` is reproducible across runs (structural
      `PartialEq` after name canonicalisation); no RNG/`HashMap`-order in the path.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (`grammar_minimize`):
  - MDL monotonicity: over-fit vs compact grammar for the *same* language → assert
    `mdl_cost(compact).total() < mdl_cost(overfit).total()`.
  - merge: two structurally-identical rules → unified, `merges_applied >= 1`.
  - inline: a single-use non-terminal → inlined, reachable rule count drops.
  - factor: a `Choice` with a common prefix → prefix factored out.
  - precision gate: a merge that over-generalises (sampled precision drop > budget)
    → rejected and counted in `candidates_rejected_by_gate`.
  - determinism: two runs equal up to rule renaming.
- `tests/integration/` (`grammar_minimize`):
  - end-to-end [`D5`](./D5-blackbox-cfg-inference.md) → D7 on the JSON-subset corpus
    → rule count drops, recall stays 1.0,
    [`D1`](./D1-inference-evaluation-harness.md) F1 non-decreasing.
  - recursion preservation on the recursive-list corpus.
- Pure in-process, no network/IO; fixtures inline or under
  `tests/fixtures/grammar/inference/`.

## References

- **Stolcke & Omohundro, "Inducing Probabilistic Grammars by Bayesian Model Merging," ICGI 1994** —
  the MDL/Bayesian objective (grammar size vs data fit); formal basis of the Occam
  cost ([`literature-review.md`](../literature-review.md) §3).
- **Arefin, Rahman, Csallner, "Black-box CFG Inference for Readable & Natural Grammars" (NatGI), 2025** —
  arXiv 2509.26616; hierarchical delta debugging is its 3rd technique = D7's
  simplification ([`literature-review.md`](../literature-review.md) §4,
  [`competitive-analysis.md`](../competitive-analysis.md) §4).
- **Nevill-Manning & Witten, "Identifying Hierarchical Structure in Sequences" (Sequitur), JAIR 1997** —
  arXiv cs/9709102; digram-uniqueness + rule-utility, the simplification intuition
  behind inline/factor ([`literature-review.md`](../literature-review.md) §3).
- **Gold, "Language Identification in the Limit," 1967** — why the Occam/MDL bias is
  required for positive-only inference ([`literature-review.md`](../literature-review.md) §0).
- [`solution-plans.md`](../solution-plans.md) §Epic D,
  [`competitive-analysis.md`](../competitive-analysis.md) §4,
  [`existing-capabilities.md`](../existing-capabilities.md) §3.
