# D9 — LLM-assisted naming & merge selection (optional)

> **Epic:** D — Inference engine · **Blocked by:** [`A3`](./A3-grammar-concept-ontology.md), [`D5`](./D5-blackbox-cfg-inference.md) · **Blocks:** —
> **Requirements:** P-3, P-12 · **Milestone:** M4
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D (D9),
> [`literature-review.md`](../literature-review.md) §6,
> [`competitive-analysis.md`](../competitive-analysis.md) §4.

## Context

The current SOTA, **NatGI** (2025, [`literature-review.md`](../literature-review.md)
§4), beats TreeVada by ~25 F1 points using three innovations; two of them are an
**LLM inside the inference loop**: it (1) generates *meaningful, natural
non-terminal names* and (2) *selects promising rule merges* during bubble
exploration. [`competitive-analysis.md`](../competitive-analysis.md) §4 maps both
onto assets this project already owns deterministically — the **351-concept
shared ontology** ([`A3`](./A3-grammar-concept-ontology.md), P-11) supplies
meaningful names, and the structural prior plus MDL drive merges — so the
project's strategy is to **reproduce NatGI's pipeline without requiring an LLM**.

But P-12 ("beat all competitors in all metrics") and the F1-parity risk
([`competitive-analysis.md`](../competitive-analysis.md) §5) mean we still want to
*be able to* use an LLM as an **optional accelerator**, so the project can report
**both** "best deterministic F1" and "best LLM-assisted F1." The hard constraint
is **P-7**: the *working Rust implementation* must never *require* a model. This
issue therefore introduces the LLM-assist **behind a trait abstraction with a
default deterministic implementation**, gated by a feature flag — so
[`D5`](./D5-blackbox-cfg-inference.md) (merge selection) and the naming pass call
the *same trait* whether or not an LLM is present.

This is also the project's single LLM-integration point. Per Anthropic's tool-use
guidance, any LLM call is wrapped behind a narrow trait so the core never depends
on a provider; the default path is provider-free and fully deterministic.

## Goal

Define two small **advisor traits** — `NamingAdvisor` (proposes non-terminal
names) and `MergeAdvisor` (ranks candidate rule merges) — each with:
- a **default deterministic implementation** grounded in the
  [`A3`](./A3-grammar-concept-ontology.md) concept ontology (names) and in
  MDL/structural heuristics (merges), which the inference core uses by default and
  which satisfies P-7 with **no model and no network**; and
- an **optional LLM-backed implementation** behind a `llm-assist` Cargo feature,
  mirroring NatGI, used only when explicitly enabled.

The advisors are *advisory*: their output is a *suggestion* that the deterministic
core validates and may override, so a misbehaving or absent LLM can never produce
an invalid grammar or a non-deterministic core result.

## Scope

**In scope**
- A new module `src/grammar/infer/advisor.rs` (under [`D5`](./D5-blackbox-cfg-inference.md)'s
  `src/grammar/infer/`).
- The `NamingAdvisor` and `MergeAdvisor` traits + request/response value types.
- `ConceptNamingAdvisor` — the **default** name proposer, looking names up in the
  [`A3`](./A3-grammar-concept-ontology.md) concept ontology.
- `MdlMergeAdvisor` — the **default** merge ranker, scoring candidate merges by
  the same MDL/grammar-size objective [`D7`](./D7-generalization-mdl-minimization.md)
  uses (description-length delta).
- An optional `LlmNamingAdvisor` / `LlmMergeAdvisor` behind `feature = "llm-assist"`,
  with a provider-agnostic `LlmClient` trait so no concrete provider is hard-wired.
- A deterministic **fallback wrapper** that calls the LLM advisor but falls back to
  the default advisor on error/timeout/disabled-feature, and **validates** every
  LLM suggestion before use.

**Out of scope** (owned elsewhere)
- The bubble/merge *mechanism* and the parse forest → [`D5`](./D5-blackbox-cfg-inference.md);
  D9 only *ranks/selects* among candidates D5 proposes.
- The MDL objective itself → [`D7`](./D7-generalization-mdl-minimization.md); D9's
  default merge advisor *reuses* it.
- Seeding the concept ontology with grammar-construct concepts →
  [`A3`](./A3-grammar-concept-ontology.md); D9 *reads* it.
- Emitting GBNF for downstream LLM-constrained generation → C3 (a different use of
  LLMs; not this issue).
- Bundling, shipping, or pinning any specific model/provider/SDK. D9 defines the
  *trait*; wiring a concrete `LlmClient` is left to deployment and documented as
  optional. **No provider dependency is added to the default build.**

## Design / specification

### Advisor traits

```rust
/// Proposes a human-meaningful name for an inferred non-terminal.
/// The core treats the result as a *suggestion*; it validates and may rename.
pub trait NamingAdvisor {
    /// Given the rule's right-hand side and the surrounding grammar, return a
    /// ranked list of candidate names (best first). Never panics; may be empty.
    fn propose_names(&self, req: &NamingRequest<'_>) -> Vec<NameCandidate>;
}

/// Ranks candidate rule merges during generalization / bubble exploration.
pub trait MergeAdvisor {
    /// Score each candidate merge in `[0.0, 1.0]` (higher = more promising).
    /// Order of the returned scores matches `req.candidates`.
    fn rank_merges(&self, req: &MergeRequest<'_>) -> Vec<MergeScore>;
}

#[derive(Clone, Debug)]
pub struct NamingRequest<'a> {
    pub grammar: &'a Grammar,            // A1 IR (context)
    pub rule_expr: &'a GrammarExpr,      // the RHS to be named
    pub sample_yields: &'a [String],     // example substrings this rule derives
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameCandidate {
    pub name: String,
    pub concept: Option<String>,         // A3 concept id, if grounded
    pub source: AdviceSource,            // Deterministic | Llm
}

#[derive(Clone, Debug)]
pub struct MergeRequest<'a> {
    pub grammar: &'a Grammar,
    pub candidates: &'a [MergeCandidate],// pairs/sets of rules D5 proposes merging
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MergeScore { pub score: f64, pub source: AdviceSource }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdviceSource { Deterministic, Llm }
```

### Default deterministic implementations (the P-7 path)

`ConceptNamingAdvisor` (default `NamingAdvisor`):
1. Compute a **signature** of `rule_expr` (e.g. "ordered choice of two terminals",
   "repetition of `digit`") using the [`A1`](./A1-grammar-ir.md) algebra shape.
2. Look the signature / its sample yields up in the
   [`A3`](./A3-grammar-concept-ontology.md) concept ontology (exact-match interning,
   per [`existing-capabilities.md`](../existing-capabilities.md) §1). If a concept
   matches (e.g. yields are all integers → the `number`/`integer` concept), return
   its canonical name with `concept: Some(id)`.
3. Otherwise fall back to a **deterministic structural name** derived from the
   shape (`seq_3`, `choice_2`, `digit_plus`) — stable and collision-free within the
   grammar. This guarantees a usable, deterministic name with **no model**.

`MdlMergeAdvisor` (default `MergeAdvisor`):
1. For each candidate merge, compute the **description-length delta** (Δ bits) the
   merge would induce, reusing [`D7`](./D7-generalization-mdl-minimization.md)'s
   MDL/grammar-size scoring (Occam objective from
   [`literature-review.md`](../literature-review.md) §3, Stolcke & Omohundro).
2. Map Δ to a `score` in `[0,1]` (merges that *reduce* description length score
   high). Deterministic; no RNG.

### Optional LLM implementations (the accelerator)

Behind `#[cfg(feature = "llm-assist")]`:

```rust
/// Provider-agnostic LLM boundary. The core depends only on this trait, never on
/// a concrete provider, so no model/SDK is required for the default build.
pub trait LlmClient: Send + Sync {
    /// Single prompt -> completion. Implementations may call any backend.
    fn complete(&self, prompt: &str) -> Result<String, LlmError>;
}

pub struct LlmNamingAdvisor<C: LlmClient> { client: C, fallback: ConceptNamingAdvisor }
pub struct LlmMergeAdvisor<C: LlmClient>  { client: C, fallback: MdlMergeAdvisor }
```

- `LlmNamingAdvisor::propose_names` prompts the model with the RHS + sample yields
  + the **A3 concept vocabulary** ("choose a name; prefer one of these concept
  names if it fits"), so LLM output is *grounded in the same ontology* the
  deterministic path uses (this is the key difference from NatGI's free-form
  naming — names stay concept-aligned, serving P-11).
- `LlmMergeAdvisor::rank_merges` asks the model to rank the candidate merges; the
  scores are then **clamped and re-validated** against the MDL delta (a merge the
  LLM loves but that explodes description length is down-weighted).

### Deterministic fallback wrapper (the safety net)

A `FallbackAdvisor<A, D>` (generic over an optional accelerator `A` and the default
`D`) implements both traits and:
- if the `llm-assist` feature is **off**, or no `LlmClient` is configured, delegates
  **entirely** to the deterministic advisor — this is the default `infer` path;
- if on, calls the LLM advisor, and on **any** `LlmError`, timeout, empty result,
  or a suggestion that **fails validation** (see below), silently falls back to the
  deterministic advisor.

**Validation of every LLM suggestion** (so the LLM can never corrupt the result):
- a proposed **name** must be a valid identifier, unique within the grammar, and —
  if it claims a `concept` — that concept must actually exist in
  [`A3`](./A3-grammar-concept-ontology.md); otherwise the candidate is rejected and
  the deterministic name is used.
- a proposed **merge score** is clamped to `[0,1]`; the *selected* merge is still
  the one [`D5`](./D5-blackbox-cfg-inference.md) accepts under its oracle/MDL guard,
  so an LLM can only *reorder* exploration, never force an invalid grammar.

Because the core always goes through `FallbackAdvisor` and the deterministic
branch is pure, **the core inference result is identical with the feature off and
with the feature on-but-LLM-unavailable** — the property the acceptance criteria
pin.

### Reporting both numbers (P-12)

The `infer` pipeline records `AdviceSource` per decision, so the
[`E3`](./E3-competitor-benchmark-suite.md) harness can run the **same** corpus
twice — `--no-llm` (deterministic) and `--llm-assist` — and report **both**
F1 numbers, exactly the dual-reporting [`competitive-analysis.md`](../competitive-analysis.md)
§5 calls for as the F1-parity-risk mitigation.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/infer/advisor.rs` | New. `NamingAdvisor`/`MergeAdvisor` traits, request/response types, `ConceptNamingAdvisor`, `MdlMergeAdvisor`, `FallbackAdvisor`; behind `cfg(feature = "llm-assist")`: `LlmClient`, `LlmNamingAdvisor`, `LlmMergeAdvisor`, `LlmError`. |
| `src/grammar/infer/mod.rs` | `pub mod advisor;` + re-export the traits and default advisors (module owned by [`D5`](./D5-blackbox-cfg-inference.md); create if D9 lands first). |
| `src/grammar/infer/*` (D5/D7) | Call sites take `&dyn MergeAdvisor` / `&dyn NamingAdvisor` (default `FallbackAdvisor<_, MdlMergeAdvisor>` etc.) instead of hard-coding heuristics — a thin, additive seam. |
| `Cargo.toml` | Add an **optional** `llm-assist` feature (`[features] llm-assist = []`). **No provider dependency in the default build.** If a concrete client is added later, gate its dep `optional = true` under this feature. Note: `pedantic`/`nursery` lints stay `warn` (`Cargo.toml:105-106`). |
| `src/lib.rs` | `pub use grammar::infer::advisor::{NamingAdvisor, MergeAdvisor, ConceptNamingAdvisor, MdlMergeAdvisor, AdviceSource};` |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_infer_advisor` modules. |
| `changelog.d/` | Fragment. |

## Reuse

- [`A3`](./A3-grammar-concept-ontology.md) concept ontology — the **grounding** for
  names; `ConceptNamingAdvisor` reads it (built on `concept_ontology.rs` /
  `seed_common_concept_ontology()`, per [`existing-capabilities.md`](../existing-capabilities.md)
  §1). This is what makes names concept-aligned (P-11) rather than free-form.
- [`D7`](./D7-generalization-mdl-minimization.md) MDL/grammar-size scoring — the
  **default merge ranking**; `MdlMergeAdvisor` reuses it (Occam objective,
  [`literature-review.md`](../literature-review.md) §3).
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr` — request context and the
  expression signature used for deterministic naming.
- [`D5`](./D5-blackbox-cfg-inference.md) — supplies the merge candidates and applies
  the chosen merges under its own oracle/MDL guard (D9 only advises).
- The trait pattern mirrors the crate's existing pluggability seam
  (`LanguageParser` + `ParserRegistry`, `src/language_parser.rs:7-15`,
  `src/parser_registry.rs:50-159`): a default impl plus user-supplied overrides.

## Acceptance criteria

- [ ] `ConceptNamingAdvisor` returns a concept-grounded name when the rule's yields
      match an [`A3`](./A3-grammar-concept-ontology.md) concept (e.g. all-integer
      yields → `integer`/`number`), and a deterministic structural name otherwise;
      names are unique within the grammar.
- [ ] `MdlMergeAdvisor` ranks merges by MDL delta deterministically (reusing
      [`D7`](./D7-generalization-mdl-minimization.md)); a description-length-reducing
      merge outranks a neutral one.
- [ ] With the `llm-assist` feature **off**, the crate builds and **all inference
      is deterministic and provider-free** (P-7) — no LLM trait is reachable from
      the default path, and the `infer` result is byte-identical across runs.
- [ ] With the feature **on** but `LlmClient` returning an error/timeout, the
      `FallbackAdvisor` falls back to the deterministic advisor and produces the
      **same** core result as the feature-off run (verified by a test using a
      deliberately-failing fake `LlmClient`).
- [ ] An LLM-proposed name that is invalid / non-unique / claims a nonexistent
      concept is **rejected** by validation and the deterministic name is used.
- [ ] The pipeline records `AdviceSource` per decision so
      [`E3`](./E3-competitor-benchmark-suite.md) can report both deterministic and
      LLM-assisted F1 (P-12 dual reporting).
- [ ] No concrete LLM provider/SDK is a default dependency; if one is added it is
      `optional = true` under `llm-assist`, and its licence is recorded in the PR.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery `warn`, `Cargo.toml:105-106`), and `cargo test --all-features` all
      pass; `rust-script scripts/check-no-src-tests.rs` passes (tests under
      `tests/`, not `src/`). Note `--all-features` enables `llm-assist`, so the LLM
      code path must also compile and lint cleanly.

## Tests

- Unit (`tests/unit/`, new `grammar_infer_advisor` module):
  - `ConceptNamingAdvisor`: hand-built grammars → assert concept-grounded names for
    matching shapes, deterministic structural names otherwise, uniqueness.
  - `MdlMergeAdvisor`: candidate merges with known MDL deltas → assert ranking
    order is deterministic and reduction-favouring.
  - `FallbackAdvisor` with a **fake failing `LlmClient`** (feature on) → asserts the
    result equals the deterministic-only result; with a **fake well-behaved
    `LlmClient`** returning a valid concept name → asserts it is used; returning an
    invalid/nonexistent-concept name → asserts it is rejected and the deterministic
    name wins.
  - feature-flag matrix: a test compiled with and without `llm-assist` confirms the
    default path is identical (use `#[cfg(...)]` guards / a small determinism
    fixture).
- Integration (`tests/integration/`):
  - run a small inference end-to-end (stub [`D5`](./D5-blackbox-cfg-inference.md)
    with a hand-built grammar if not merged) once deterministic, once with a fake
    LLM client; assert the deterministic result is stable and the `AdviceSource`
    log distinguishes the two runs.
- The LLM tests use **fakes only** (a local `LlmClient` impl) — **no network, no
  real model** in CI; deterministic.

## References

- NatGI — Arefin, Rahman, Csallner, "Black-box Context-free Grammar Inference for
  Readable & Natural Grammars," 2025 ([arXiv 2509.26616](https://arxiv.org/abs/2509.26616))
  — LLM for non-terminal names + merge selection (the pattern this issue mirrors,
  grounded in the concept ontology instead of free-form).
- Wang, Zhang, et al., "Grammar Prompting for Domain-Specific Language Generation
  with LLMs," NeurIPS 2023 — few-shot grammar steering of an LLM.
- [`literature-review.md`](../literature-review.md) §6 (LLM-assisted inference;
  "always behind a deterministic-fallback flag so P-7 never *requires* a model"),
  [`competitive-analysis.md`](../competitive-analysis.md) §4 (NatGI technique →
  native equivalent) and §5 (report both deterministic and LLM-assisted F1),
  [`library-survey.md`](../library-survey.md) §E, [`solution-plans.md`](../solution-plans.md)
  §Epic D (D9), [`A3`](./A3-grammar-concept-ontology.md),
  [`D5`](./D5-blackbox-cfg-inference.md), [`D7`](./D7-generalization-mdl-minimization.md).
