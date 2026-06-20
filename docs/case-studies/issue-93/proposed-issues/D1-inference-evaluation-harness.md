# D1 — Inference evaluation harness

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`D3`](./D3-state-merging-regular-inference.md), [`D5`](./D5-blackbox-cfg-inference.md), [`E3`](./E3-competitor-benchmark-suite.md)
> **Requirements:** P-12 · **Milestone:** M3 — **build first among the D issues**
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D & §3 (row D1),
> [`competitive-analysis.md`](../competitive-analysis.md) §2 (metric definitions),
> [`literature-review.md`](../literature-review.md) §0, §4.

## Context

P-12 ("beats all competitors in all metrics") is only meaningful if "the
metrics" are pinned to a single, reproducible definition *before* any inference
algorithm exists — otherwise every later D-issue would invent its own scoring
and the competitor numbers in [`competitive-analysis.md`](../competitive-analysis.md)
§2 would not be comparable. The black-box CFG-inference line this project
competes against — GLADE → Arvada → TreeVada → Kedavra → NatGI
([`literature-review.md`](../literature-review.md) §4) — all report the *same*
three primary numbers (precision, recall, F1) computed by **sampling**, because
none of them has access to the golden grammar's decision procedure as a closed
form; they approximate language equivalence by generating strings and checking
membership both ways. This issue builds that scoring harness once, so D3/D5 are
measurable the moment they emit a `Grammar`, and E3 can gate the P-12 claim in CI.

There is no metrics code today: `src/parity.rs` tests *parse parity* (does the
text round-trip losslessly), not *inference quality*
([`existing-capabilities.md`](../existing-capabilities.md) §3, row "No inference
evaluation harness"). But the substrate to evaluate against already exists — the
executable fixtures and the ~30 tree-sitter grammars are golden oracles (see
**Reuse**).

## Goal

Provide a deterministic Rust **inference-evaluation harness** that scores an
inferred [`Grammar`](./A1-grammar-ir.md) against a golden reference (another
`Grammar`, or any membership oracle) using the exact precision/recall/F1
**sampling** definitions the competitors report, plus an **MDL / grammar-size**
score for the Occam objective (D7) and a **golden-corpus runner** that executes a
named corpus and emits a `BenchmarkReport`. This pins the P-12 metric
definitions for every downstream issue.

## Scope

**In scope**
- A new module `src/grammar/inference/eval.rs` (re-exported from `src/lib.rs`).
- A **membership oracle** abstraction: a trait an inferred or golden grammar (and
  later a tree-sitter parser, E2/D10) can implement to answer "does string `s`
  belong to language `L`?".
- A **sampler**: deterministic, seeded generation of strings from a `Grammar`
  (derivation-tree expansion with a recursion-depth budget and per-rule fuel).
- **Precision / recall / F1** computed by two-way sampling (formulas below).
- **MDL / grammar-size** scoring (bits, plus a raw symbol count).
- A **golden-corpus runner** + `BenchmarkReport`/`MetricScores` value types.
- The corpus *registry* convention (reuse `src/parity.rs` discipline).

**Out of scope** (owned elsewhere)
- The actual inference algorithms that *produce* grammars → [`D2`](./D2-lexical-class-inference.md),
  [`D3`](./D3-state-merging-regular-inference.md), [`D4`](./D4-sequitur-compression.md),
  [`D5`](./D5-blackbox-cfg-inference.md).
- Vendoring the published TreeVada/Arvada/GLADE corpora and wiring CI gates →
  [`E3`](./E3-competitor-benchmark-suite.md) (E3 *uses* this harness).
- Readability / round-trip / format-coverage secondary metrics
  ([`competitive-analysis.md`](../competitive-analysis.md) §2) → those are scored
  by the emitter/importer issues (B*/C*/F2); D1 owns only the **primary**
  precision/recall/F1 + MDL definitions.
- Any LLM-based scoring → never (deterministic only, P-7).

## Design / specification

### Membership oracle

```rust
/// Decides language membership for a target language `L`.
pub trait MembershipOracle {
    /// Returns `true` iff `text` is in the language.
    fn accepts(&self, text: &str) -> bool;
}
```

Provide a blanket adapter `GrammarOracle<'g>(&'g Grammar)` implementing
`MembershipOracle` via a recogniser over the [`A1`](./A1-grammar-ir.md) IR (a
small backtracking PEG/CFG matcher; ordered `Choice` short-circuits, unordered
`Choice` tries all alternatives, repetition is greedy with backtracking). A
tree-sitter grammar can later be wrapped the same way (E2) so a real parser acts
as the golden oracle — this is what makes the ~30 wired grammars usable as gold.

### Sampler (string generation from a `Grammar`)

```rust
pub struct SampleConfig {
    pub seed: u64,           // deterministic RNG seed (no system entropy — P-7)
    pub count: usize,        // number of strings to draw
    pub max_depth: usize,    // recursion-depth budget per derivation
    pub repeat_cap: usize,   // max expansions for `*`/`+`/Repeat{max:None}
}
pub fn sample(grammar: &Grammar, config: &SampleConfig) -> Vec<String>;
```

Algorithm — derivation-tree expansion from `grammar.start_rule()`:
1. Seed a small deterministic PRNG (e.g. SplitMix64 / xorshift implemented
   inline; **no external `rand` for the core** so results are bit-stable across
   platforms — pin the algorithm in code, document it).
2. Expand the start non-terminal. For each `GrammarExpr` variant:
   - `Terminal`/`TerminalInsensitive` → emit the literal.
   - `CharRange(a,b)` / `CharClass` → pick one character deterministically from
     the range/items.
   - `AnyChar` → pick from a fixed printable-ASCII pool.
   - `Sequence` → expand children left-to-right.
   - `Choice` → pick one alternative (ordered: bias to earlier; unordered:
     uniform over the PRNG).
   - `Optional` → include with p≈0.5; `ZeroOrMore`/`OneOrMore`/`Repeat` → draw a
     count within `repeat_cap`/`min..=max`.
   - `NonTerminal` → recurse, decrementing the depth budget; when the budget hits
     0, prefer the shortest terminating alternative (compute per-rule
     nullability/min-length once, cache it) to guarantee termination.
   - `And`/`Not`/`Capture` → expand the inner expr (predicates do not emit text;
     `Capture` is transparent for generation).
3. Deduplicate identical samples within a draw (keep insertion order).

Termination must be **guaranteed**: a left-recursive rule with no terminating
alternative is reported as `EvalError::NonTerminating { rule }` rather than
looping. Compute a "can terminate within k steps" table over the rule graph
(standard nullable/reachable fixpoint) before sampling.

### Primary metrics — the exact sampling formulas

These are the definitions every competitor reports
([`competitive-analysis.md`](../competitive-analysis.md) §2; NatGI/TreeVada in
[`literature-review.md`](../literature-review.md) §4). Let `G_inf` be the
inferred grammar/oracle and `G_gold` the golden grammar/oracle.

- **Precision** = of strings *sampled from `G_inf`*, the fraction *accepted by
  `G_gold`* (how often the inferred grammar produces valid strings — over-general
  grammars score low):

  ```text
  precision = |{ s ∈ sample(G_inf) : G_gold.accepts(s) }| / |sample(G_inf)|
  ```

- **Recall** = of strings *sampled from `G_gold`*, the fraction *accepted by
  `G_inf`* (how much of the true language the inferred grammar covers —
  over-specific grammars score low):

  ```text
  recall = |{ s ∈ sample(G_gold) : G_inf.accepts(s) }| / |sample(G_gold)|
  ```

- **F1** = harmonic mean (define `F1 = 0` when `precision + recall == 0` to avoid
  division by zero):

  ```text
  F1 = 2 · precision · recall / (precision + recall)
  ```

When a corpus ships a fixed positive set (the example texts themselves) instead
of a golden grammar, **recall** is measured directly as the fraction of held-out
positive examples `G_inf` accepts, and **precision** still uses sampling from
`G_inf` against whatever oracle the corpus provides. Both paths must be
supported; document which a report used (`scoring_mode: GoldenGrammar | Corpus`).

### MDL / grammar-size score (the Occam objective for D7)

Two-part MDL (Stolcke & Omohundro, [`literature-review.md`](../literature-review.md)
§3): description length of the grammar plus the description length of the data
given the grammar.

```text
size_symbols(G) = Σ_rules ( 1 + symbols_in(rule.expr) )      // raw node count
L(G)            = bits to encode G's rules/symbols           // ⌈log2⌉ per symbol over the alphabet of (terminals ∪ nonterminals ∪ operators)
L(D | G)        = Σ_{s ∈ D} ( bits to encode s's derivation under G )
mdl(G, D)       = L(G) + L(D | G)                            // lower is better
```

Expose both `size_symbols` (cheap, used as the Occam tie-breaker) and `mdl`
(used by D7's model-merging stop condition). Give exact, documented bit
accounting so two runs are comparable; floating-point is fine but the formula
must be fixed in code and doc-commented.

### Report types

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct MetricScores {
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub size_symbols: usize,
    pub mdl_bits: f64,
}

#[derive(Clone, Debug)]
pub struct BenchmarkReport {
    pub corpus: &'static str,
    pub scores: MetricScores,
    pub samples_drawn: usize,
    pub seed: u64,
}

pub enum EvalError { NonTerminating { rule: String }, EmptyCorpus, /* … */ }

pub fn evaluate(
    inferred: &Grammar,
    golden: &dyn MembershipOracle,
    golden_sampler: Option<&Grammar>,   // None ⇒ corpus-mode recall
    positives: &[&str],                 // held-out positive examples (corpus-mode)
    config: &SampleConfig,
) -> Result<MetricScores, EvalError>;
```

### Golden-corpus runner

Mirror the `src/parity.rs` fixture discipline: a `const` slice of corpus
descriptors, each naming a language label, a positive-example set, and an oracle
source (a golden `Grammar` *or* a tree-sitter language label). A `run_corpus`
function executes one descriptor through `evaluate` and returns a
`BenchmarkReport`. The vendored competitor corpora and CI gating live in
[`E3`](./E3-competitor-benchmark-suite.md); D1 ships the runner + a couple of
in-repo smoke corpora built from existing fixtures.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | New (or extend if a sibling D-issue created it). `pub mod eval;` and the shared `inference::` namespace. |
| `src/grammar/inference/eval.rs` | New. `MembershipOracle`, `GrammarOracle`, `SampleConfig`, `sample`, `MetricScores`, `BenchmarkReport`, `EvalError`, `evaluate`, `size_symbols`, `mdl`, the recogniser, the seeded PRNG, the corpus runner. |
| `src/lib.rs` | Add `pub use grammar::inference::eval::{MembershipOracle, GrammarOracle, MetricScores, BenchmarkReport, SampleConfig, evaluate};` next to the existing `grammar` re-exports. |
| `tests/unit/mod.rs` | Register a new `inference_eval` unit-test module (mirror the `grammar_parsing` registration). |
| `tests/integration/mod.rs` | Register a golden-corpus smoke integration test. |
| `changelog.d/` | Add a fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- **`src/parity.rs` fixture discipline** — the corpus registry copies the pattern
  of `LANGUAGE_FIXTURES` / `PROGRAMMING_LANGUAGE_TARGETS` (const slices of
  descriptors with accessors), re-exported at `src/lib.rs:66-72`. `LanguageFixture`
  already exposes `language()`/`source()`/`description()` (`src/parity.rs:545-568`)
  — reuse those source strings as positive examples for the smoke corpora.
- **tree-sitter CSTs as golden oracles** — the ~30 wired grammars
  (`src/tree_sitter_adapter.rs:136-207`; `PROGRAMMING_LANGUAGE_TARGETS` at
  `src/parity.rs:619-670`) parse real source losslessly; wrapping a tree-sitter
  parse as a `MembershipOracle` (does the text parse without error?) gives a free
  high-quality acceptor for recall/precision. (The oracle wrapper that depends on
  the registry lands with E2; D1's trait is the seam.)
- **[`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`** — both the input to score
  and the structure the recogniser/sampler walk; reuse `start_rule()`,
  `rule(name)`, `referenced_nonterminals()`.
- **Do not** pull in `rand`/`fastrand` for the core sampler — inline a documented
  PRNG so scores are bit-reproducible (P-7). A dev-dependency RNG for fuzz-style
  tests is acceptable if gated to `tests/`.

## Acceptance criteria

- [ ] `MembershipOracle`, `GrammarOracle`, `SampleConfig`, `sample`,
      `MetricScores`, `BenchmarkReport`, `EvalError`, `evaluate`, `size_symbols`,
      `mdl`, and the corpus runner are public and documented (doc-comment on each
      public item; crate denies missing docs if configured — check `src/lib.rs`
      lints).
- [ ] `precision`, `recall`, `f1` match the formulas above; verified on
      hand-constructed cases: identical grammars ⇒ `f1 == 1.0`; a strictly
      over-general grammar ⇒ `precision < 1.0`, `recall == 1.0`; a strictly
      over-specific grammar ⇒ `recall < 1.0`, `precision == 1.0`; disjoint
      languages ⇒ `f1 == 0.0`.
- [ ] `sample` is deterministic: the same `(grammar, SampleConfig.seed)` yields
      the identical `Vec<String>` across runs; left-recursive non-terminating
      grammars return `EvalError::NonTerminating` (never loop / overflow stack).
- [ ] `mdl(G, D)` and `size_symbols(G)` are documented, deterministic, and a
      smaller equivalent grammar scores a lower MDL than a redundant one.
- [ ] The golden-corpus runner executes at least two in-repo smoke corpora built
      from `src/parity.rs` fixtures and returns populated `BenchmarkReport`s.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic /
      nursery are `warn` per `Cargo.toml` `[lints.clippy]`), and
      `cargo test --all-features` all pass; `rust-script scripts/check-no-src-tests.rs`
      passes (tests live under `tests/`, not `src/`).

## Tests

- `tests/unit/` (new `inference_eval` module):
  - **Metric correctness:** build small `Grammar`s with [`A1`](./A1-grammar-ir.md)'s
    builder (identical, over-general, over-specific, disjoint) and assert the
    precision/recall/F1 relationships in the acceptance criteria.
  - **Sampler determinism:** same seed ⇒ identical samples; different seed ⇒
    (usually) different samples but always in-language for a finite grammar.
  - **Termination:** a left-recursive grammar yields `NonTerminating`; a bounded
    recursive grammar (arithmetic expr) samples and terminates within `max_depth`.
  - **MDL monotonicity:** a grammar and its de-duplicated/minimised form; assert
    `mdl(min) <= mdl(redundant)` and `size_symbols(min) < size_symbols(redundant)`.
- `tests/integration/`:
  - Run a smoke corpus (e.g. a tiny hand-built JSON-subset golden grammar vs. an
    intentionally over-general inferred grammar) end-to-end through `run_corpus`
    and assert the `BenchmarkReport` fields are populated and within expected
    bounds.
- Pure in-process, no network. Keep corpora inline or under
  `tests/fixtures/grammar/corpora/`.

## References

- de la Higuera, *Grammatical Inference*, Cambridge University Press, 2010 —
  canonical source for the evaluation models; see
  [`literature-review.md`](../literature-review.md) §0.
- Stolcke & Omohundro, "Inducing Probabilistic Grammars by Bayesian Model
  Merging," ICGI 1994 — the two-part MDL / description-length objective;
  [`literature-review.md`](../literature-review.md) §3.
- TreeVada (Arefin et al., ICSE 2024, arXiv 2308.06163) and NatGI (Arefin et al.,
  2025, arXiv 2509.26616) — the precision/recall/F1-by-sampling protocol and the
  F1 ladder (NatGI ≈ 0.57, +25 pts over TreeVada);
  [`literature-review.md`](../literature-review.md) §4,
  [`competitive-analysis.md`](../competitive-analysis.md) §2–§3.
- [`competitive-analysis.md`](../competitive-analysis.md) §2 pins these as the
  P-12 primary metrics; [`solution-plans.md`](../solution-plans.md) §3 row D1.
