# D6 — Delimiter-skeleton structural prior

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`A2`](./A2-grammar-surface-syntax.md) · **Blocks:** [`D5`](./D5-blackbox-cfg-inference.md)
> **Requirements:** P-3, P-4, P-5 · **Milestone:** M3
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D,
> [`literature-review.md`](../literature-review.md) §0, §4,
> [`competitive-analysis.md`](../competitive-analysis.md) §4.

## Context

**Gold 1967** proves regular (and a fortiori context-free) languages are **not
identifiable in the limit from positive data alone**
([`literature-review.md`](../literature-review.md) §0), so every practical
positive-only system must inject an **inductive bias**. P-3 ("reconstruct a
programming-language grammar from only correct texts") is exactly that hard setting,
making a structural prior **not optional but the principled core of the approach**
([`solution-plans.md`](../solution-plans.md) §1).

This project owns such a prior **natively**. meta-notation parses the universal
**delimiter skeleton** — brackets `()` `{}` `[]` (nested), quotes `''` `""` `` `` ``
(opaque), and unquoted text blocks — losslessly
([`library-survey.md`](../library-survey.md) §D.1,
[`existing-capabilities.md`](../existing-capabilities.md) §2). The SOTA **NatGI
2025** credits **bracket-guided bubble exploration** as the *#1 driver of its gains*
([`literature-review.md`](../literature-review.md) §4) — it had to *bolt* delimiter
awareness onto TreeVada. Here the signal is free:
[`competitive-analysis.md`](../competitive-analysis.md) §4 maps "bracket-guided
bubble exploration" one-to-one onto "meta-notation delimiter skeleton, parsed
losslessly by construction (P-4) → D6". [`A2`](./A2-grammar-surface-syntax.md)
already reuses the delimiter pass to *author* grammars, but nothing turns an example
*program's* delimiter structure into a **seed parse forest** for inference
([`existing-capabilities.md`](../existing-capabilities.md) §3). D6 delivers that
bridge: the explicit structural prior [`D5`](./D5-blackbox-cfg-inference.md) consumes
to make positive-only CFG inference tractable.

## Goal

Provide a **structural-prior module** that takes raw example strings, runs them
through the meta-notation / LiNo delimiter pass (the same one
[`A2`](./A2-grammar-surface-syntax.md) uses), and emits, per example, a **seed
parse tree** whose interior nodes follow the delimiter skeleton. The collection of
seed trees plus the alignment metadata is the **`StructuralPrior`** that
[`D5`](./D5-blackbox-cfg-inference.md) bubbles and merges over, replacing the flat
single-node trees Arvada/TreeVada start from with **delimiter-structured** ones.

## Scope

**In scope**
- A new public module `src/grammar/inference/prior.rs` (under the `inference`
  module created by [`D5`](./D5-blackbox-cfg-inference.md); D6 may land first and
  create the parent `src/grammar/inference/mod.rs`).
- A `SeedTree` type (a generic parse tree over input spans) and a
  `StructuralPrior` type (a batch of `SeedTree`s + the shared alphabet/segmentation).
- `build_structural_prior(examples: &[String], opts: PriorOptions) -> StructuralPrior`.
- The delimiter→tree lowering using meta-notation block kinds
  (`paren`/`curly`/`square`/`singleQuote`/`doubleQuote`/`backtick`/`text`).
- A determinism guarantee: identical input slices yield byte-identical trees.

**Out of scope** (owned elsewhere)
- The bubble/merge search and the acceptance check → [`D5`](./D5-blackbox-cfg-inference.md).
- The final A1 `Grammar` emission → [`D5`](./D5-blackbox-cfg-inference.md).
- Generalization / MDL minimisation of the result → [`D7`](./D7-generalization-mdl-minimization.md).
- The grammar IR, builder, links encoding → [`A1`](./A1-grammar-ir.md).
- The *authoring* surface syntax → [`A2`](./A2-grammar-surface-syntax.md) (D6 reuses
  its skeleton stage, but targets example *programs*, not grammar text).
- Metric definitions → [`D1`](./D1-inference-evaluation-harness.md).

## Design / specification

### Why the delimiter skeleton is a sound structural prior

Delimiters are the one syntactic feature that is **lossless, language-agnostic, and
recoverable without any grammar** ([`library-survey.md`](../library-survey.md)
§D.1). A balanced `(...)`/`{...}`/`[...]` group is almost always a *constituent* (an
argument list, a block, an index); a quoted run is almost always a single *atomic*
terminal. Seeding inference with these constituents shrinks the bubble search space
(D5 need not *discover* bracket structure by trial merges) and yields
**well-bracketed, readable** non-terminals — the property NatGI used an LLM to
obtain. This is the principled answer to Gold (§0): the prior restricts the
hypothesis class to grammars *consistent with the observed delimiter nesting*.

### Types and signatures

```rust
/// One node of a seed parse tree over a single example string.
///
/// Leaves carry a byte span into the example; interior nodes carry the
/// delimiter family that produced them (or `Group` for an inferred-only group).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeedNode {
    /// An opaque terminal slice (a `text` run, or a quoted string kept whole).
    Leaf { span: ByteSpan, kind: LeafKind },
    /// A bracketed/grouped constituent: the delimiter pair plus ordered children.
    Group { delimiter: Delimiter, children: Vec<SeedNode>, span: ByteSpan },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafKind { Text, SingleQuote, DoubleQuote, Backtick }

/// The delimiter family of a `Group` (mirrors meta-notation block kinds).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter { Paren, Curly, Square, /// synthetic root wrapping a whole example
    Root }

/// Half-open byte range `[start, end)` into the owning example string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteSpan { pub start: usize, pub end: usize }

/// One example plus its seed tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedTree { pub example: String, pub root: SeedNode }

/// The batch of seed trees + shared analysis handed to D5.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralPrior {
    pub trees: Vec<SeedTree>,
    /// Distinct terminal slices observed across all leaves (interned, sorted).
    pub alphabet: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct PriorOptions {
    /// If true, adjacent text/quote leaves between two groups are merged into a
    /// single leaf (coarser seeds, fewer bubbles); default false (finer seeds).
    pub coalesce_runs: bool,
    /// Whitespace handling for `text` runs (see Determinism). Default `Trim`.
    pub whitespace: WhitespacePolicy,
}

/// Build the structural prior for a corpus of positive examples.
pub fn build_structural_prior(examples: &[String], opts: PriorOptions) -> StructuralPrior;
```

### Staged algorithm

**Phase 1 — Skeletonise (reuse, do not reinvent).** For each example, run the
meta-notation / LiNo delimiter pass exactly as [`A2`](./A2-grammar-surface-syntax.md)
§"Parse pipeline" does — `LinkNetwork::parse(example, "grammar-prior",
ParseConfiguration::default())` (`src/link_network.rs:350`), reusing the same
parenthesis/atom handling as `src/lino_parser.rs:95-185`. **No new lexer.** This
yields the lossless block sequence (`paren`/`curly`/`square`/quotes/`text`).
Surface unbalanced-delimiter inputs by falling back to a single `Root` group whose
children are flat `text`/quote leaves (an example with no recoverable structure
still produces a valid, if shallow, seed — never an error).

**Phase 2 — Lower skeleton → `SeedNode`.** Walk the block sequence and map:
- a bracket block (`paren`/`curly`/`square`) → `Group { delimiter, children, span }`,
  recursing into its content;
- a quoted block → `Leaf { kind: SingleQuote|DoubleQuote|Backtick, span }` (quotes
  are opaque: their interior is **not** parsed, matching meta-notation semantics);
- a `text` run → one or more `Leaf { kind: Text, span }`, split per the
  `whitespace`/`coalesce_runs` policy (Determinism below).
Wrap the whole example in a synthetic `Group { delimiter: Root, .. }` so every
seed tree has a single root spanning `[0, len)`.

**Phase 3 — Intern alphabet.** Collect every leaf's slice text into a
`BTreeSet<String>`, then materialise `alphabet: Vec<String>` in sorted order. This
gives D5 the terminal inventory and a stable, deterministic ordering for its merge
iteration (a key determinism lever vs Arvada — see
[`D5`](./D5-blackbox-cfg-inference.md) §Determinism).

**Phase 4 — Assemble.** Return `StructuralPrior { trees, alphabet }`.

### How D5 consumes the prior (the hand-off contract)

[`D5`](./D5-blackbox-cfg-inference.md) treats each `SeedNode::Group` as a
**pre-formed bubble candidate** — a contiguous sibling span already known to be a
constituent — instead of enumerating all O(n²) sibling spans blindly. Every `Group`
becomes a candidate non-terminal whose body is its children; bubbling proceeds
**inside-out** (deepest groups first) so recursion is introduced where the nesting
already implies it; the `alphabet` seeds D5's terminal set; and `LeafKind` tells D5
which leaves lower to `Terminal` vs `TerminalInsensitive`
([`A1`](./A1-grammar-ir.md)). This is **literally NatGI's bracket-guided
exploration** — produced by construction, not heuristically:
[`competitive-analysis.md`](../competitive-analysis.md) §4 made executable.

### Determinism notes

D6 is a pure function of `(examples, opts)`: block ordering is the LiNo parser's
deterministic source order; `whitespace: WhitespacePolicy` fixes leaf splitting
(`Trim` = strip leading/trailing ASCII whitespace and split on maximal internal
runs; `Keep` = one leaf per run verbatim — `Trim` default); the alphabet is
`BTreeSet`-ordered (lexicographic), never `HashSet`; no randomness or clock. A
property test asserts `build_structural_prior(x, o) == build_structural_prior(x, o)`
and that re-ordering the *examples* re-orders `trees` correspondingly while each tree
and the `alphabet` are unchanged.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | New (if not already created by [`D5`](./D5-blackbox-cfg-inference.md)). `pub mod prior;` + shared `inference` re-exports. |
| `src/grammar/inference/prior.rs` | New. `SeedNode`, `LeafKind`, `Delimiter`, `ByteSpan`, `SeedTree`, `StructuralPrior`, `PriorOptions`, `WhitespacePolicy`, `build_structural_prior`, and the four private phase fns. |
| `src/grammar/mod.rs` | Add `pub mod inference;` (module tree rooted by [`A1`](./A1-grammar-ir.md)). |
| `src/lib.rs` | `pub use grammar::inference::prior::{StructuralPrior, SeedTree, SeedNode, build_structural_prior};` next to the A1/A2 re-exports (`src/lib.rs:44,60-61`). |
| `tests/unit/mod.rs` | Register a `grammar_prior` unit-test module. |
| `tests/fixtures/grammar/prior/` | A few example programs (a parenthesised arithmetic expr, a JSON object, an S-expression list) whose seed-tree shape is asserted. |
| `changelog.d/` | Add a fragment (`scripts/create-changelog-fragment.rs`). |

## Reuse

- **meta-notation delimiter model** — the mandated P-4 prior; consumed via the LiNo
  parser, not forked ([`library-survey.md`](../library-survey.md) §D.1,
  [`existing-capabilities.md`](../existing-capabilities.md) §2).
- `LinkNetwork::parse` (`src/link_network.rs:350`) + the parenthesis/atom skeleton
  pass `src/lino_parser.rs:95-185` — **the same stage [`A2`](./A2-grammar-surface-syntax.md)
  reuses**; do not write a new lexer.
- [`A1`](./A1-grammar-ir.md) `GrammarFormat::Inferred`, `Terminal`/`TerminalInsensitive`
  — the leaf-kind targets D5 will lower into.
- `links-notation` crate 0.13 (`Cargo.toml:53`) — underlying parser (Unlicense).
- No new third-party dependency: D6 is pure analysis over existing parsing.

## Acceptance criteria

- [ ] `build_structural_prior` returns one `SeedTree` per example, each rooted in a
      `Group { delimiter: Root, .. }` spanning the whole example.
- [ ] Every balanced `()`/`{}`/`[]` group in an input becomes a `SeedNode::Group`
      of the matching `Delimiter`; every quoted run becomes a single opaque
      `Leaf` of the matching `LeafKind`; the quote interior is **not** sub-parsed.
- [ ] Unbalanced-delimiter input does **not** error: it yields a shallow `Root`
      seed of flat leaves (assert no panic, valid tree).
- [ ] `alphabet` is the sorted set of distinct leaf slices (assert ordering is
      lexicographic and deduplicated).
- [ ] **Determinism:** `build_structural_prior(x, o) == build_structural_prior(x, o)`
      byte-for-byte; documented `WhitespacePolicy` default is the only knob that
      changes leaf splitting.
- [ ] No new lexer is introduced — the skeleton stage calls `LinkNetwork::parse`
      (verifiable by inspection / a test asserting the intermediate network is
      non-empty for a non-trivial input).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (`grammar_prior`):
  - parenthesised arithmetic `"a*(b+c)"` → assert the `(b+c)` subtree is a `Paren`
    `Group` with three leaves (`b`, `+`, `c`) and the root has children `a`, `*`,
    `(...)`.
  - JSON object `'{"k": [1, 2]}'` → assert a `Curly` root-child containing a
    `Square` group, and that `"k"` is a single `DoubleQuote` leaf (interior not split).
  - S-expression `"(f (g x) y)"` → assert nested `Paren` groups and inside-out
    nesting depth.
  - unbalanced `"(a [b"` → assert a flat `Root` fallback, no panic.
  - determinism: same input twice → equal `StructuralPrior`; permuting examples
    permutes `trees` but each tree and `alphabet` are unchanged.
- Pure in-process, no IO; fixtures inline or under `tests/fixtures/grammar/prior/`.

## References

- **Gold, "Language Identification in the Limit," *Information and Control* 10(5):447–474, 1967** —
  positive-only inference needs a bias ([`literature-review.md`](../literature-review.md) §0).
- **Arefin, Rahman, Csallner, "Black-box CFG Inference for Readable & Natural Grammars" (NatGI), 2025** —
  arXiv 2509.26616; bracket-guided bubble exploration is its #1 technique
  ([`literature-review.md`](../literature-review.md) §4,
  [`competitive-analysis.md`](../competitive-analysis.md) §4).
- **Arefin, Rahman, Csallner, "Fast Deterministic Black-box CFG Inference" (TreeVada), ICSE 2024** —
  arXiv 2308.06163; the bracket/paren prior D5 ports and D6 supplies natively.
- meta-notation: <https://github.com/link-foundation/meta-notation> ·
  [`library-survey.md`](../library-survey.md) §D.1,
  [`existing-capabilities.md`](../existing-capabilities.md) §2,
  [`solution-plans.md`](../solution-plans.md) §Epic D.
