# D4 — Sequitur structural-compression pass

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** (feeds [`D5`](./D5-blackbox-cfg-inference.md), [`D7`](./D7-generalization-mdl-minimization.md))
> **Requirements:** P-2 · **Milestone:** M3
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D & §3 (row D4),
> [`literature-review.md`](../literature-review.md) §3.

## Context

Before the expensive black-box CFG search ([`D5`](./D5-blackbox-cfg-inference.md))
runs, a **cheap, unencumbered, linear-time** structural pass can already expose
the repeated substructure in a sequence and propose a hierarchy of rules.
**Sequitur** (Nevill-Manning & Witten, *JAIR* 7:67–82, 1997,
[`literature-review.md`](../literature-review.md) §3) does exactly this: it infers
a context-free grammar from a *single* sequence, online and in linear time, by
maintaining two invariants — **digram uniqueness** (no pair of adjacent symbols
appears twice) and **rule utility** (every rule is used more than once). Its
output is a hierarchical grammar that compresses the input, and it needs **no
oracle and no negative examples** — making it an ideal first structural pass
feeding D5 and a seed for the MDL/generalisation stage
([`D7`](./D7-generalization-mdl-minimization.md)).

The algorithm is **unencumbered** — port it directly from the 1997 paper. (The
crates.io `sequitur` crate is an unrelated file-sequence library, and the
reference-site code licence is unverified, so we implement from the paper, not
from any code — [`library-survey.md`](../library-survey.md) §C.5.) Nothing in the
crate does hierarchical sequence compression today
([`existing-capabilities.md`](../existing-capabilities.md) §3, row "No grammar
inference of any kind").

## Goal

Port Sequitur into Rust: consume a sequence of symbols (tokens from
[`D2`](./D2-lexical-class-inference.md), or characters), build a hierarchical
grammar online while preserving digram-uniqueness and rule-utility, and emit the
result as a [`A1`](./A1-grammar-ir.md) `Grammar` so it can feed
[`D5`](./D5-blackbox-cfg-inference.md) / [`D7`](./D7-generalization-mdl-minimization.md) and be
scored by [`D1`](./D1-inference-evaluation-harness.md).

## Scope

**In scope**
- A new module `src/grammar/inference/sequitur.rs` (under the `inference::`
  namespace from [`D1`](./D1-inference-evaluation-harness.md)).
- The online Sequitur algorithm with both invariants and the four operations
  (append, enforce digram-uniqueness, create rule, enforce rule-utility).
- `run_sequitur(sequence: &[Symbol]) -> Grammar` producing a hierarchical
  [`A1`](./A1-grammar-ir.md) grammar (one start rule + generated `R1, R2, …`
  rules of `Sequence`s and `NonTerminal`s).
- Linear-time data structures (doubly-linked symbol list + a digram index).

**Out of scope** (owned elsewhere)
- Generalising the compressed grammar across *multiple* sequences / merging rules
  → [`D5`](./D5-blackbox-cfg-inference.md) (bubble-merge) and
  [`D7`](./D7-generalization-mdl-minimization.md) (MDL minimisation). Sequitur compresses one
  sequence; it does not generalise.
- Tokenisation (producing the `Symbol` stream) →
  [`D2`](./D2-lexical-class-inference.md).
- Metric scoring → [`D1`](./D1-inference-evaluation-harness.md).
- Stochastic / probabilistic structure → out of scope (Sequitur is exact, not
  statistical; the stochastic path is [`D3`](./D3-state-merging-regular-inference.md)
  ALERGIA).

## Design / specification

### Representation

The grammar under construction is a set of rules; the start rule (call it `S`)
holds the (shrinking) representation of the input. Each rule body is a
**doubly-linked list** of symbols, where a symbol is either a **terminal** or a
**non-terminal reference** to another rule. A global **digram index** maps each
adjacent pair `(x, y)` to its single occurrence (the uniqueness witness).

```rust
type Symbol = String;                  // terminal token, or a generated rule name "R{n}"
// Internal linked-list / arena representation (indices, not Rc) for borrow-checker sanity:
struct SymbolNode { value: SymRef, prev: Option<usize>, next: Option<usize> }
enum SymRef { Terminal(Symbol), Rule(RuleId) }
struct DigramIndex(BTreeMap<(SymKey, SymKey), usize>); // digram → node index; BTreeMap for determinism
```

Use an **arena of nodes with index links** (not `Rc<RefCell<…>>`) to keep the
doubly-linked structure borrow-checker-friendly and clippy-clean.

### The two invariants (verbatim from the 1997 paper)

1. **Digram uniqueness** — no two adjacent symbols (a *digram*) may appear more
   than once in the entire grammar. When appending a symbol creates a duplicate
   digram, *enforce* uniqueness (see operations).
2. **Rule utility** — every rule must be referenced more than once. When a rule
   drops to a single reference, *enforce* utility by inlining it.

### The online algorithm

Process the input symbol by symbol; after each append, restore the invariants:

1. **Append** the next input symbol to the end of rule `S`, forming a new last
   digram `(p, last)`.
2. **Enforce digram uniqueness.** Look the new digram up in the digram index:
   - If it does not occur elsewhere, record it and continue.
   - If it matches an existing occurrence elsewhere:
     - If that other occurrence **is exactly the body of an existing rule `R`**
       (i.e. `R → x y`), **replace** the new digram with a reference to `R`.
     - Otherwise **create a new rule** `R → x y`, replace *both* occurrences of
       the digram with a reference to `R`, and register `R`'s body digram.
   - Replacement may create a *new* duplicate digram at the substitution
     boundary; repeat enforcement until stable (this is what keeps it linear
     amortised — each symbol is touched O(1) times on average).
3. **Enforce rule utility.** If a replacement left some rule `R` referenced only
   once, **remove `R`** and substitute its body inline at the sole reference site;
   update the digram index for the changed boundaries.

Each operation is O(1) amortised given the linked list + hash/btree digram index,
giving the paper's overall **linear time** (use `BTreeMap` for the index so
iteration/serialisation order is deterministic — P-7 — accepting the log factor;
document the choice).

### Emit into the A1 IR

When the input is exhausted, convert the rule set to a
[`A1`](./A1-grammar-ir.md) `Grammar`:
- The start rule `S` → a `GrammarRule { name: "start", expr: Sequence([...]), .. }`
  whose body is the sequence of terminals/non-terminal refs.
- Each generated rule `R{n}` → a `GrammarRule` whose `expr` is a `Sequence` of its
  body symbols; terminals become `GrammarExpr::Terminal`, rule refs become
  `GrammarExpr::NonTerminal("R{n}")`.
- `source_format = Some(GrammarFormat::Inferred)`; `set_start("start")`.

```rust
pub fn run_sequitur(sequence: &[Symbol]) -> Grammar;
```

Because Sequitur only *compresses* (it never generalises beyond the one input),
the produced grammar derives **exactly** the input sequence: a useful invariant
to test (recall 1.0 on the input under [`D1`](./D1-inference-evaluation-harness.md),
precision restricted to the single string). Downstream
[`D5`](./D5-blackbox-cfg-inference.md)/[`D7`](./D7-generalization-mdl-minimization.md) then
generalise it.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | Add `pub mod sequitur;` (create the module if [`D1`](./D1-inference-evaluation-harness.md) has not). |
| `src/grammar/inference/sequitur.rs` | New. The arena/linked-list, digram index, the four operations, `run_sequitur`, the A1-IR emission. |
| `src/lib.rs` | Add `pub use grammar::inference::sequitur::run_sequitur;`. |
| `tests/unit/mod.rs` | Register a new `inference_sequitur` unit-test module. |
| `changelog.d/` | Add a fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- **[`A1`](./A1-grammar-ir.md) IR** — `Grammar`, `GrammarExpr::{Terminal, NonTerminal, Sequence}`,
  `GrammarRule`, `GrammarFormat::Inferred`, builder — the emission target; reuse
  `Grammar::with_rule`/`set_start`/`builder()`.
- **[`D2`](./D2-lexical-class-inference.md)** `LexicalModel::tokenize` to produce
  the `Symbol` stream from a raw example text (Sequitur runs over tokens, not raw
  bytes, so generated rules are meaningful).
- **[`D1`](./D1-inference-evaluation-harness.md)** `evaluate` / `size_symbols` /
  `mdl` to verify the compressed grammar derives the input and to measure its
  size (Sequitur's compression is itself an MDL signal for D7).
- **`std::collections::BTreeMap`** for the digram index (deterministic iteration).
  Avoid `Rc<RefCell<…>>`: use an index-based arena for the doubly-linked list.
- **Algorithm source:** the 1997 paper only (the algorithm is unencumbered); do
  **not** vendor the reference-site code (licence unverified) or the unrelated
  crates.io `sequitur` crate — [`library-survey.md`](../library-survey.md) §C.5.

## Acceptance criteria

- [ ] `run_sequitur` is public and documented (crate denies missing docs if
      configured — check `src/lib.rs`).
- [ ] **Digram uniqueness** holds on the produced grammar: no adjacent symbol pair
      appears more than once across all rule bodies (assertable by scanning the
      emitted `Grammar`).
- [ ] **Rule utility** holds: every generated rule is referenced more than once
      (no rule with a single use survives).
- [ ] On the paper's canonical example input (`"abcabdabcabd"` or equivalent), the
      output exhibits the expected shared sub-rule hierarchy (a rule for the
      repeated block), and the grammar **derives exactly the input** (recall 1.0 on
      the input string via [`D1`](./D1-inference-evaluation-harness.md)).
- [ ] Compression is real: for an input with repetition, `size_symbols(grammar)`
      (from [`D1`](./D1-inference-evaluation-harness.md)) is smaller than the flat
      input length.
- [ ] Determinism: a fixed input sequence yields a byte-identical `Grammar`
      (BTreeMap-ordered digram handling; stable generated rule names `R1, R2, …`).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic /
      nursery `warn` per `Cargo.toml` `[lints.clippy]`), `cargo test --all-features`
      pass; `rust-script scripts/check-no-src-tests.rs` passes (tests under
      `tests/`).

## Tests

- `tests/unit/` (new `inference_sequitur` module):
  - Canonical paper example: assert the shared sub-rule appears and the grammar
    derives exactly the input.
  - Both invariants: scan the emitted `Grammar` and assert digram-uniqueness and
    rule-utility hold.
  - No repetition ⇒ a single flat start rule and **no** generated sub-rules
    (rule-utility never triggers).
  - Nested repetition (e.g. `"aabaab"` then a longer self-similar string) ⇒
    multi-level hierarchy; assert rule nesting depth > 1.
  - Compression: `size_symbols` < input length for a repetitive input.
  - Determinism: repeated runs are byte-identical.
- Pure in-process, no network. Fixtures inline or under
  `tests/fixtures/grammar/sequitur/`.

## References

- Nevill-Manning & Witten, "Identifying Hierarchical Structure in Sequences: A
  Linear-Time Algorithm (Sequitur)," *JAIR* 7:67–82, 1997
  ([arXiv cs/9709102](https://arxiv.org/abs/cs/9709102)) — the algorithm and its
  two invariants; [`literature-review.md`](../literature-review.md) §3.
- Stolcke & Omohundro, Bayesian model merging, ICGI 1994 — the MDL objective that
  consumes Sequitur's compression in [`D7`](./D7-generalization-mdl-minimization.md);
  [`literature-review.md`](../literature-review.md) §3.
- Sequitur is unencumbered; reference-code licence unverified, port from the
  paper — [`library-survey.md`](../library-survey.md) §C.5;
  [`solution-plans.md`](../solution-plans.md) §3 row D4 (Addresses P-2).
