# D5 — Black-box CFG inference engine (TreeVada port)

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`D1`](./D1-inference-evaluation-harness.md), [`D6`](./D6-delimiter-structural-prior.md) · **Blocks:** [`D7`](./D7-generalization-mdl-minimization.md), D8, D9, [`E1`](./E1-cli-grammar-subcommands.md), [`E3`](./E3-competitor-benchmark-suite.md), [`E5`](./E5-end-to-end-integration-examples.md)
> **Requirements:** P-2, P-3, P-12 · **Milestone:** M3
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D (§3 table, §4 DAG — D5 is on
> the critical path `A1 → D6 → D5 → {D7,E3,E5}`),
> [`literature-review.md`](../literature-review.md) §4,
> [`competitive-analysis.md`](../competitive-analysis.md).

## Context

This is the **primary research deliverable**. P-2 ("infer a grammar from example
texts") and especially P-3 ("reconstruct a programming-language grammar from **only
positive examples**") are the issue's headline asks, and P-12 ("beat all competitors
in all metrics") is measured here. The black-box, positive-leaning CFG-inference
line is well charted — **GLADE (PLDI'17) → Arvada (ASE'21) → TreeVada (ICSE'24) →
Kedavra (ASE'24) → NatGI (2025)** ([`literature-review.md`](../literature-review.md)
§4) — and the ladder to beat is published (NatGI F1 ≈ 0.57; TreeVada ≈ 0.32 on the
same suite; [`competitive-analysis.md`](../competitive-analysis.md) §2). The strategy
is *not* to invent a novel algorithm but to **port the published SOTA pipeline onto a
substrate that supplies its two strongest priors for free**
([`solution-plans.md`](../solution-plans.md) §1). Nothing in the crate infers
grammars today ([`existing-capabilities.md`](../existing-capabilities.md) §3):
[`A1`](./A1-grammar-ir.md) gives the `Grammar` IR to emit into,
[`D1`](./D1-inference-evaluation-harness.md) the precision/recall/F1/MDL metrics, and
[`D6`](./D6-delimiter-structural-prior.md) the **delimiter-skeleton structural
prior** that makes positive-only inference tractable; D5 stitches them into the
engine.

**Why this substrate beats the SOTA.** NatGI's #1 innovation is *bracket-guided
bubble exploration*, which [`competitive-analysis.md`](../competitive-analysis.md) §4
maps one-to-one onto "meta-notation delimiter skeleton, parsed losslessly by
construction (P-4)". That bracket signal is **native here**, delivered by
[`D6`](./D6-delimiter-structural-prior.md) as seed trees — so D5 starts where NatGI
had to bolt structure on. The other NatGI innovations are owned by siblings:
meaningful non-terminal names → A3/D9; HDD simplification →
[`D7`](./D7-generalization-mdl-minimization.md).

## Goal

Provide a **deterministic, positive-only, black-box CFG inference engine** that
takes a corpus of example strings, consumes the
[`D6`](./D6-delimiter-structural-prior.md) structural prior, and produces an A1
`Grammar` (`source_format = Inferred`). Port **TreeVada** (MIT, deterministic,
bracket-prior) as the core; fold in **Arvada's** bubble-and-merge (recursion
introduction) and **Kedavra's** incremental segmentation. This is the engine
[`D7`](./D7-generalization-mdl-minimization.md) generalises and
[`E3`](./E3-competitor-benchmark-suite.md) benchmarks against the published corpora.

## Scope

**In scope**
- A new public module `src/grammar/inference/cfg.rs` (under the `inference` module;
  the parent `src/grammar/inference/mod.rs` may already exist from
  [`D6`](./D6-delimiter-structural-prior.md)).
- The **oracle abstraction** (`Oracle` trait): positive-only by default, with an
  optional membership oracle.
- The staged pipeline: **seed → bubble → merge (acceptance-checked) → emit**.
- `infer_cfg(examples: &[String], oracle: &dyn Oracle, opts: InferenceOptions) -> InferenceResult`
  returning an A1 `Grammar` plus a small report (rule/merge counts, timings).
- Determinism as a hard, tested property (a competitive advantage over Arvada).
- Kedavra-style incremental segmentation for large inputs (segment → infer per
  segment → stitch), behind an option.

**Out of scope** (owned elsewhere)
- The delimiter seed forest itself → [`D6`](./D6-delimiter-structural-prior.md)
  (D5 *consumes* `StructuralPrior`).
- Metric definitions / golden-corpus runner → [`D1`](./D1-inference-evaluation-harness.md)
  (D5 *calls* D1 to gate acceptance and in tests).
- MDL/Occam generalisation of the over-fit forest → [`D7`](./D7-generalization-mdl-minimization.md).
- Semantic (beyond-CFG) constraints → D8. LLM-assisted naming/merge selection → D9.
- Active-learning (L\*/TTT) oracle path → D10 (D5's membership oracle is *passive
  use* of a parser, not query synthesis).
- Vendoring competitor corpora / CI gate → [`E3`](./E3-competitor-benchmark-suite.md).
- The `infer` CLI subcommand → [`E1`](./E1-cli-grammar-subcommands.md).

## Design / specification

### The oracle abstraction

TreeVada/Arvada validate each candidate merge against a **boolean membership
oracle**. P-3 is *positive-only* ([`requirements.md`](../requirements.md) P-3
reading): no labelled negatives, no oracle *required*. D5 therefore defines an
oracle trait with a **positive-only default** and an **optional membership** path:

```rust
/// Decides whether a generalisation step is acceptable.
///
/// Positive-only inference cannot ask "is string s in the language?" — Gold 1967
/// (see D6 / literature-review §0). So the default oracle is *generalisation-bounded*:
/// it accepts a merge iff it still parses every positive example AND does not
/// over-generalise beyond an MDL/sampling budget (delegated to D1). An optional
/// `MembershipOracle` (e.g. an existing parser from `ParserRegistry`) tightens
/// acceptance when available, but is never required for P-3.
pub trait Oracle {
    /// Does the candidate grammar still accept every positive example?
    fn accepts_all_positive(&self, grammar: &Grammar, examples: &[String]) -> bool;
    /// Optional membership check; `None` ⇒ positive-only (no negative signal).
    fn membership(&self) -> Option<&dyn MembershipOracle> { None }
}

/// Optional black-box acceptor over arbitrary strings (D10 wraps real parsers).
pub trait MembershipOracle {
    fn accepts(&self, candidate: &str) -> bool;
}

/// The default, positive-only oracle: parse-all-examples + D1 over-generalisation guard.
pub struct PositiveOnlyOracle { /* holds D1 sampling budget + parser */ }
```

The acceptance check is the heart of correctness:
- **Recall floor:** reject a merge if the resulting grammar fails to parse any
  positive example. Examples are parsed via
  [`E2`](./E2-inferred-grammar-runtime-parser.md)'s evaluator if available, else a
  small built-in CFG recogniser (Earley-style; see Phase 3).
- **Over-generalisation guard:** with no `MembershipOracle`, D5 asks
  [`D1`](./D1-inference-evaluation-harness.md) to sample N strings from the candidate
  and rejects if the *sampled-string self-parse divergence* exceeds a budget (a cheap
  "this merge blew the language open" proxy). With a `MembershipOracle`, the guard
  uses real negatives instead.

### Staged pipeline (named phases)

```
examples ──▶ [P0 Segment?] ──▶ [P1 Seed] ──▶ [P2 Bubble] ──▶ [P3 Merge] ──▶ [P4 Emit] ──▶ Grammar
                (Kedavra)        (D6)        (Arvada+TV)    (acceptance)     (A1)
```

**Phase 0 — Segment (Kedavra, optional).** If `opts.incremental` is set, split
each example at delimiter boundaries (reusing D6's seed-tree top-level children as
natural segments), infer per segment, then stitch the per-segment grammars by
unioning rules with identical bodies. This is Kedavra's *incremental* idea
([`literature-review.md`](../literature-review.md) §4): it bounds memory/time on
large inputs and improves precision under limited examples. Default off for small
corpora (whole-string inference).

**Phase 1 — Seed (D6 hand-off).** Call `build_structural_prior(examples, …)` from
[`D6`](./D6-delimiter-structural-prior.md) for `StructuralPrior { trees, alphabet
}`, then convert each `SeedTree` into an initial parse tree: `Leaf{Text}` → a
tentative `Terminal`; `Leaf{SingleQuote|DoubleQuote|Backtick}` →
`Terminal`/`TerminalInsensitive` (`Backtick` ⇒ insensitive, per
[`A1`](./A1-grammar-ir.md)); `Group{Paren|Curly|Square}` → a tentative non-terminal
whose body is its children. This replaces the *flat* trees Arvada/TreeVada start
from: **the delimiter structure is already present**, so the bubble search begins
pre-bracketed — the concrete realisation of NatGI's bracket-guidance
([`competitive-analysis.md`](../competitive-analysis.md) §4).

**Phase 2 — Bubble (Arvada bubble-and-merge + TreeVada determinism).** Iteratively
introduce candidate non-terminals by "bubbling" contiguous sibling spans into a new
rule (*"these k adjacent children form a reusable unit N; replace them with `N` and
add `N ::= <those children>`"*), **prioritising D6's `Group` spans first** (known
constituents) before other sibling spans. This is Arvada's recursion-introduction
step ([`literature-review.md`](../literature-review.md) §4) — but **TreeVada
removes Arvada's randomised search**: spans are enumerated in a **fixed order**
(deepest-first, then left-to-right by span start, then by D6's sorted `alphabet`
for ties). Determinism is a *competitive property* (see below).

**Phase 3 — Merge (acceptance-checked).** Two non-terminals (or a non-terminal and
a structurally-equal sub-tree) merge into one when the `Oracle` accepts:
1. propose a merge of `A`, `B` (D6-`Group`-derived candidates first);
2. build the trial `Grammar` with `A`, `B` unified (`A`'s name kept, `B`'s
   references rewritten);
3. parse all positive examples (built-in recogniser or
   [`E2`](./E2-inferred-grammar-runtime-parser.md)) — reject if any no longer parses;
4. run the over-generalisation guard ([`D1`](./D1-inference-evaluation-harness.md)
   sampling, or a `MembershipOracle`) — reject if it over-generalises;
5. accept iff both pass; record the merge.
Merging *introduces recursion* (when `A` ends up referencing a rule that references
`A`), turning a finite seed forest into a recursive CFG — Arvada's core
contribution. Iterate Phases 2–3 to a fixpoint (no bubble or merge accepted).

**Phase 4 — Emit A1 `Grammar`.** Materialise the surviving non-terminals as
[`A1`](./A1-grammar-ir.md) `GrammarRule`s (bodies as `Sequence`/`Choice`/`Repeat`/
terminals), set the start rule to the synthetic `Root` non-terminal, and set
`source_format = Some(GrammarFormat::Inferred)`. Non-terminals get deterministic
placeholder names (`n0`, `n1`, … in discovery order); D9 may rename them via the
concept ontology and [`D7`](./D7-generalization-mdl-minimization.md) compacts the
rule set — D5 emits the *over-fit* but *correct-on-positives* grammar.

```rust
#[derive(Clone, Copy, Debug)]
pub struct InferenceOptions {
    /// Kedavra-style per-segment inference for large inputs (default false).
    pub incremental: bool,
    /// Cap on bubble/merge iterations (defensive; default a documented constant).
    pub max_iterations: usize,
    /// Over-generalisation sampling budget handed to D1 (default a constant).
    pub sample_budget: usize,
}

#[derive(Clone, Debug)]
pub struct InferenceResult {
    pub grammar: Grammar,
    /// Counts/timings for D1 reporting & E3 benchmarking (no behaviour impact).
    pub report: InferenceReport,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InferenceReport {
    pub rules: usize,
    pub bubbles_proposed: usize,
    pub merges_accepted: usize,
    pub merges_rejected: usize,
}

/// Infer a CFG from positive examples using the D6 structural prior.
pub fn infer_cfg(
    examples: &[String],
    oracle: &dyn Oracle,
    opts: InferenceOptions,
) -> InferenceResult;
```

### Determinism (a competitive property — vs Arvada)

Arvada is *nondeterministic* (randomised merge search); TreeVada's headline is a
**deterministic** redesign that is also ~2.4× faster
([`literature-review.md`](../literature-review.md) §4,
[`competitive-analysis.md`](../competitive-analysis.md) §2). D5 inherits and *tests*
this: candidate enumeration uses fixed total orders (deepest-first; then
left-to-right by span start; then by D6's lexicographically-sorted `alphabet`) —
**no RNG, seed, clock, or `HashMap` iteration order** (use `BTreeMap`/`Vec`); and
`infer_cfg(x, o, p)` returns a structurally-equal `Grammar` on repeated runs,
invariant to example *input order* up to rule renaming (tests canonicalise names
before comparison). "Determinism (no random seeds) is itself a competitive property"
([`competitive-analysis.md`](../competitive-analysis.md) §2) — so the determinism
test is an acceptance criterion, not a nicety.

### The built-in recogniser

Phase 3 must parse examples against a *candidate* A1 grammar that may be ambiguous
mid-inference. Use a small **Earley-style recogniser** over the A1 IR (handles
arbitrary CFGs incl. ambiguity, unlike a PEG/packrat matcher), kept internal and
minimal — it answers *membership* only, not parse-tree extraction. If
[`E2`](./E2-inferred-grammar-runtime-parser.md) lands first, prefer its evaluator and
keep the recogniser as the dependency-free fallback. (Do **not** pull in a heavyweight
Earley crate; `earlgrey` is a study reference only —
[`library-survey.md`](../library-survey.md) §A.11.)

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | Ensure `pub mod cfg;` and shared re-exports (created by [`D6`](./D6-delimiter-structural-prior.md) or here). |
| `src/grammar/inference/cfg.rs` | New. `Oracle`, `MembershipOracle`, `PositiveOnlyOracle`, `InferenceOptions`, `InferenceResult`, `InferenceReport`, `infer_cfg`, and the private Phase 0–4 fns. |
| `src/grammar/inference/recognizer.rs` | New. Minimal Earley-style membership recogniser over the A1 IR (internal). |
| `src/grammar/mod.rs` | Ensure `pub mod inference;`. |
| `src/lib.rs` | `pub use grammar::inference::cfg::{infer_cfg, Oracle, MembershipOracle, PositiveOnlyOracle, InferenceOptions, InferenceResult};` next to the A1/D6 re-exports. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `cfg_inference` unit + integration modules. |
| `tests/fixtures/grammar/inference/` | Small corpora: parenthesised arithmetic examples, a JSON subset, an S-expression list, a tiny recursive list language. |
| `changelog.d/` | Fragment (`scripts/create-changelog-fragment.rs`). |

## Reuse

- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/builder + `GrammarFormat::Inferred`
  — emission target and recogniser input IR.
- [`D6`](./D6-delimiter-structural-prior.md) `StructuralPrior`/`SeedTree` — the
  delimiter-seeded initial forest (Phase 1); **the NatGI bracket-guidance hook**.
- [`D1`](./D1-inference-evaluation-harness.md) sampling + precision/recall/F1/MDL —
  the over-generalisation guard (Phase 3) and the test metrics.
- TreeVada (MIT, Python) — **primary port**; Arvada (MIT) bubble-merge; Kedavra
  (incremental segmentation) ([`library-survey.md`](../library-survey.md) §C.1,
  [`literature-review.md`](../literature-review.md) §4). All MIT/portable — port,
  don't vendor GPL.
- `ParserRegistry`/`LanguageParser` (`src/parser_registry.rs:50-159`,
  `src/language_parser.rs:7-9`) — source of an optional `MembershipOracle`
  (coordinate with [`E2`](./E2-inferred-grammar-runtime-parser.md)/D10; don't duplicate).
- The ~30 wired tree-sitter grammars (`src/tree_sitter_adapter.rs:136-207`) are
  golden CST oracles / corpora for tests and [`E3`](./E3-competitor-benchmark-suite.md).

## Acceptance criteria

- [ ] `infer_cfg` produces an A1 `Grammar` with `source_format = Inferred` that
      **parses 100% of its input positive examples** (recall = 1.0 on the training
      set) for every fixture corpus.
- [ ] The engine consumes [`D6`](./D6-delimiter-structural-prior.md)'s
      `StructuralPrior`: a test asserts a bracketed input yields a grammar whose
      rule structure mirrors the delimiter nesting (the bracket-guidance property).
- [ ] **Recursion is introduced:** the recursive-list fixture yields a grammar with
      a self-referential rule (e.g. `list ::= item | item "," list`).
- [ ] The `Oracle` abstraction works both ways: positive-only (default) infers
      without negatives; supplying a `MembershipOracle` tightens at least one
      fixture's precision (assert measured precision via [`D1`](./D1-inference-evaluation-harness.md)
      is ≥ the positive-only run).
- [ ] **Determinism:** `infer_cfg` returns a structurally-equal grammar across
      repeated runs and is invariant to example input order up to rule renaming
      (canonicalise names, then `PartialEq`). No RNG/seed/`HashMap`-order in the path.
- [ ] D5 reports [`D1`](./D1-inference-evaluation-harness.md) **precision/recall/F1
      on a small corpus** in tests (not just training recall): on the bundled
      arithmetic + JSON-subset corpora, F1 is recorded and a regression threshold
      is asserted (the absolute bar vs competitors lives in
      [`E3`](./E3-competitor-benchmark-suite.md), not here).
- [ ] Malformed/empty corpora are handled gracefully (empty input → empty grammar;
      unparsable example → shallow seed via D6) — **never a panic**.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (`cfg_inference`):
  - parenthesised arithmetic corpus → assert the inferred grammar parses every
    example and that `(... )` groups map to a non-terminal (bracket-guidance).
  - recursive-list corpus (`"[]"`, `"[a]"`, `"[a,b]"`, `"[a,b,c]"`) → assert a
    self-referential rule exists (recursion introduced).
  - determinism: two runs equal; permuted-input run equal up to rule renaming.
  - oracle: same corpus inferred positive-only vs with a `MembershipOracle` stub →
    assert precision (via [`D1`](./D1-inference-evaluation-harness.md)) does not drop.
  - acceptance check: a merge that would drop a positive example is rejected
    (assert that example still parses in the final grammar).
- `tests/integration/` (`cfg_inference`):
  - full pipeline on a JSON-subset corpus → [`D1`](./D1-inference-evaluation-harness.md)
    precision/recall/F1 computed and asserted above a recorded threshold.
  - incremental mode (`opts.incremental = true`) on a larger corpus → same
    language as whole-string mode (assert recall = 1.0 and comparable F1).
- Pure in-process, no network/IO; fixtures inline or under
  `tests/fixtures/grammar/inference/`.

## References

- **Arefin, Rahman, Csallner, "Fast Deterministic Black-box Context-free Grammar Inference" (TreeVada), ICSE 2024** —
  arXiv 2308.06163; MIT. **Primary port:** deterministic, bracket-prior, ~2.4× faster than Arvada
  ([`literature-review.md`](../literature-review.md) §4, [`library-survey.md`](../library-survey.md) §C.1).
- **Kulkarni, Lemieux, Sen, "Learning Highly Recursive Input Grammars" (Arvada), ASE 2021** —
  arXiv 2108.13340; MIT. Tree-based bubble-and-merge (recursion introduction).
- **Li et al., "Incremental Context-free Grammar Inference in Black Box Settings" (Kedavra), ASE 2024** —
  arXiv 2408.16706. Incremental segmentation (Phase 0).
- **Arefin, Rahman, Csallner, "Black-box CFG Inference for Readable & Natural Grammars" (NatGI), 2025** —
  arXiv 2509.26616; SOTA, F1 ≈ 0.57; bracket-guided exploration = D6 here
  ([`competitive-analysis.md`](../competitive-analysis.md) §4).
- **Bastani, Sharma, Aiken, Liang, "Synthesizing Program Input Grammars" (GLADE), PLDI 2017** —
  Apache-2.0; the cited baseline (read the PLDI'22 critical replication).
- **Gold, "Language Identification in the Limit," 1967** — why the
  [`D6`](./D6-delimiter-structural-prior.md) prior is mandatory
  ([`literature-review.md`](../literature-review.md) §0).
- [`solution-plans.md`](../solution-plans.md) §Epic D,
  [`competitive-analysis.md`](../competitive-analysis.md) §2–§4,
  [`existing-capabilities.md`](../existing-capabilities.md) §3.
