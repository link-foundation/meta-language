# D10 — Optional active learning (L*/TTT oracle path)

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`E2`](./E2-inferred-grammar-runtime-parser.md) · **Blocks:** —
> **Requirements:** P-2 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D (D10),
> [`literature-review.md`](../literature-review.md) §1,
> [`library-survey.md`](../library-survey.md) §C.3.

## Context

The core inference deliverable is **positive-only** (P-3): reconstruct a grammar
from correct examples with **no oracle**, which is the hard, valuable, and
Gold-impossible-without-a-prior setting ([`literature-review.md`](../literature-review.md)
§0). But the project sometimes *does* have a **membership oracle**: an existing
parser registered through `ParserRegistry` (`src/parser_registry.rs:50-159`), or
one of the ~30 wired tree-sitter grammars acting as an *acceptor*
([`existing-capabilities.md`](../existing-capabilities.md) §1), reachable as a
runtime acceptor via [`E2`](./E2-inferred-grammar-runtime-parser.md). When an
oracle exists, **Angluin's L\*** ([`literature-review.md`](../literature-review.md)
§1) learns the corresponding regular language *exactly* in polynomial time, and
**TTT** (Isberner et al.) does so space-optimally in counterexample length. This
issue adds that **optional active-learning path**.

This is deliberately a **secondary, optional** capability (requirement **P-2**,
"infer a grammar from examples," in its query-based variant). It is **never on the
critical path** and **never required** for the positive-only core (P-3) — the
DAG in [`solution-plans.md`](../solution-plans.md) §4 places D10 off to the side,
needing only [`A1`](./A1-grammar-ir.md) and [`E2`](./E2-inferred-grammar-runtime-parser.md).
LearnLib is **Apache-2.0** ([`library-survey.md`](../library-survey.md) §C.3), so
the algorithms may be **ported to Rust** (clean-room idiomatic port preferred over
a JVM bridge).

## Goal

Implement **Angluin's L\*** (observation table; closedness/consistency;
counterexample handling) — and, optionally, **TTT** (discrimination-tree variant)
— to learn a **DFA from membership + equivalence queries**, then lower that DFA
into the [`A1`](./A1-grammar-ir.md) IR as a regular `Grammar`. The **membership
oracle** is any acceptor: an existing `ParserRegistry` parser or a tree-sitter
grammar wrapped via [`E2`](./E2-inferred-grammar-runtime-parser.md). The
**equivalence oracle** is approximated by sampling (since exact equivalence is
generally unavailable for a black-box parser), with the learned DFA refined on any
counterexample found.

## Scope

**In scope**
- A new module `src/grammar/infer/active.rs` (under [`D5`](./D5-blackbox-cfg-inference.md)'s
  `src/grammar/infer/`).
- An `Oracle` trait (`membership` + `equivalence`) and adapters that build a
  membership oracle from a `ParserRegistry` parser and from an
  [`E2`](./E2-inferred-grammar-runtime-parser.md) acceptor.
- **L\***: observation table `(S, E, T)`, closedness & consistency checks,
  hypothesis-DFA construction from row signatures, counterexample processing
  (Angluin's suffix-addition; Rivest–Schapire as an option).
- A `Dfa` value type + lowering `Dfa -> Grammar` (right-linear rules → A1 IR).
- An **approximate equivalence oracle** by bounded sampling of strings the
  hypothesis accepts/rejects, checked against the membership oracle.
- **Optional TTT** (discrimination trees) behind a `ttt` sub-path, sharing the
  `Oracle`/`Dfa` plumbing; may be deferred to a follow-up if L\* lands first
  (note it, don't block).

**Out of scope** (owned elsewhere)
- The positive-only CFG core → [`D5`](./D5-blackbox-cfg-inference.md); D10 is an
  *alternative* path used only when an oracle exists, and is never required.
- Passive state-merging from labelled strings (RPNI/EDSM) → **D3** (a different
  algorithm family, [`literature-review.md`](../literature-review.md) §2).
- Wrapping/registering a grammar as a runtime parser/acceptor →
  [`E2`](./E2-inferred-grammar-runtime-parser.md); D10 *consumes* that acceptor.
- Learning full context-free / visibly-pushdown languages with an oracle — L\*/TTT
  here target **regular** acceptors (token classes, lexical structure feeding the
  CFG layer). A visibly-pushdown extension
  ([`literature-review.md`](../literature-review.md) §5, arXiv 2508.16305) is a
  noted follow-up, not this issue.
- Any LLM use → [`D9`](./D9-llm-assisted-naming-merge.md).

## Design / specification

### Oracle abstraction

```rust
/// Minimally Adequate Teacher (Angluin 1987): membership + equivalence queries.
pub trait Oracle {
    /// Is `word` in the target language?
    fn membership(&self, word: &[Symbol]) -> bool;
    /// If `hypothesis` is wrong, return a counterexample (a word on which the
    /// hypothesis and the target disagree); `None` means "accepted as equivalent".
    fn equivalence(&self, hypothesis: &Dfa) -> Option<Vec<Symbol>>;
}

/// The input alphabet (concrete bytes/chars/tokens the learner explores).
pub type Symbol = char; // or a small token id; chosen per call site

/// Membership oracle backed by an existing parser: a word is "in the language"
/// iff the parser accepts it (parses without falling back to lossless text).
pub struct ParserMembershipOracle {
    registry: ParserRegistry,         // src/parser_registry.rs:50
    language: String,                 // the registered key to dispatch to
    alphabet: Vec<Symbol>,
}

/// Membership oracle backed by an E2 acceptor (an A1 grammar wrapped as a
/// LanguageParser/acceptor and registered).
pub struct GrammarAcceptorOracle { /* wraps E2's acceptor */ }
```

`ParserMembershipOracle::membership` runs `registry.parse(word, &language, cfg)`
(`src/parser_registry.rs` `ParserRegistry::parse`) and reports acceptance via a
predicate over the resulting network — e.g. "no lossless-text fallback node / no
error node," using the projections at `src/link_network.rs:64-103`. The exact
acceptance predicate is documented and unit-tested against a known parser. The
**equivalence** query is the approximate sampler below (a black-box parser offers
no exact equivalence check).

### L\* (Angluin 1987 — [`literature-review.md`](../literature-review.md) §1)

Maintain an **observation table** `(S, E, T)`:
- `S` — a prefix-closed set of access strings (rows);
- `E` — a suffix-closed set of distinguishing experiments (columns);
- `T: (S ∪ S·Σ) × E → {0,1}` filled by membership queries.

Loop:
1. **Fill** the table with membership queries for every `(s·a, e)` and `(s, e)`.
2. **Closedness**: if some row `s·a` (lower part) has a signature absent from the
   upper part `S`, add `s·a` to `S`; refill; repeat.
3. **Consistency**: if two rows `s1, s2 ∈ S` have equal signatures but `s1·a`,
   `s2·a` differ for some `a`, add a distinguishing suffix `a·e` to `E`; refill;
   repeat.
4. When **closed and consistent**, build the **hypothesis DFA**: states = distinct
   row signatures of `S`, start = row of `ε`, accepting = rows with `T(s, ε)=1`,
   transitions `δ(row(s), a) = row(s·a)`.
5. **Equivalence query**: ask the oracle. If it returns a counterexample `w`, add
   `w` (and, in Angluin's variant, all its prefixes; or use Rivest–Schapire's
   single-suffix analysis) to refine the table, then go to 1. If `None`, the
   hypothesis is the result.

Termination is guaranteed for regular targets (the minimal DFA bounds the number
of states/refinements) — the classic L\* polynomial bound.

### Approximate equivalence oracle

Since the membership oracle is a black-box parser, exact equivalence is
unavailable. Implement `SamplingEquivalenceOracle`:
- generate up to `n` test words (bounded length, mix of strings the hypothesis
  accepts and rejects, plus random/boundary words over the alphabet);
- for each, compare hypothesis acceptance with `membership`; the first
  disagreement is the counterexample;
- if none disagree within the budget, report `None` (probably-equivalent). The
  budget and sampling strategy are configurable and deterministic given a fixed
  seed (so runs are reproducible — match the determinism criterion the other
  inference issues use).

### Optional TTT (Isberner et al. — [`literature-review.md`](../literature-review.md) §1)

TTT replaces the observation table with three coupled structures
(spanning-tree hypothesis, discrimination tree, discriminator-finalisation) to
keep discriminators short and decompose counterexamples incrementally — **space-
optimal in counterexample length**. Ship behind the same `Oracle`/`Dfa` interface;
if scope is tight, deliver L\* first and TTT as a follow-up (the survey notes TTT
is "best for long counterexamples", [`library-survey.md`](../library-survey.md)
§C.3).

### Lowering `Dfa -> Grammar`

A DFA lowers to a **right-linear** grammar in the [`A1`](./A1-grammar-ir.md) IR:
- one `GrammarRule` per state (name `q{i}`, or a concept-aligned name if
  [`A3`](./A3-grammar-concept-ontology.md)/[`D9`](./D9-llm-assisted-naming-merge.md)
  is available — but D10 does not *require* them);
- each transition `δ(q, a)=q'` becomes `Terminal(a)` followed by `NonTerminal(q')`
  inside the state's rule body (a `Sequence`); accepting states add an `Empty`
  alternative; the rule body is a `Choice { ordered: false, .. }` over the
  outgoing transitions;
- start symbol = the start state's rule; `source_format = Some(GrammarFormat::Inferred)`.
This produces a valid A1 `Grammar` that round-trips through the links encoding
([`A1`](./A1-grammar-ir.md)) and can be emitted/translated by the C\* issues.

### Types and entry point

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dfa {
    pub alphabet: Vec<Symbol>,
    pub states: usize,
    pub start: usize,
    pub accepting: Vec<bool>,
    pub delta: Vec<Vec<usize>>,   // delta[state][symbol_index] = next state
}

#[derive(Clone, Debug)]
pub struct ActiveLearningConfig {
    pub max_word_len: usize,
    pub equivalence_samples: usize,
    pub seed: u64,                 // deterministic sampling
    pub use_ttt: bool,             // false => L*
}

/// Learn a DFA via L*/TTT against `oracle`, then lower it to an A1 Grammar.
pub fn learn_grammar(
    oracle: &dyn Oracle,
    config: &ActiveLearningConfig,
) -> Result<Grammar, ActiveLearningError>;

/// Just the automaton, for callers that want the DFA before lowering.
pub fn learn_dfa(oracle: &dyn Oracle, config: &ActiveLearningConfig)
    -> Result<Dfa, ActiveLearningError>;
```

## File-level plan

| File | Change |
|---|---|
| `src/grammar/infer/active.rs` | New. `Oracle`, `ParserMembershipOracle`, `GrammarAcceptorOracle`, `SamplingEquivalenceOracle`, observation table + L\*, optional TTT, `Dfa`, `Dfa -> Grammar` lowering, `learn_dfa`/`learn_grammar`, `ActiveLearningConfig`, `ActiveLearningError`. |
| `src/grammar/infer/mod.rs` | `pub mod active;` + re-exports (module owned by [`D5`](./D5-blackbox-cfg-inference.md); create if D10 lands first). |
| `src/lib.rs` | `pub use grammar::infer::active::{learn_grammar, learn_dfa, Oracle, Dfa, ActiveLearningConfig};` |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_infer_active` modules. |
| `tests/fixtures/grammar/active/` | Tiny reference DFAs / acceptors used as ground-truth oracles (e.g. "even number of `a`s", a small identifier lexer). |
| `changelog.d/` | Fragment. |

## Reuse

- `ParserRegistry` (`src/parser_registry.rs:50-159`, `ParserRegistry::parse`) — the
  **oracle source**: an existing registered parser answers membership queries.
- [`E2`](./E2-inferred-grammar-runtime-parser.md) — wraps an A1 grammar as a
  runtime acceptor that can also serve as a membership oracle; D10 consumes it.
- `LanguageParser` (`src/language_parser.rs:7-15`) and the network projections
  (`src/link_network.rs:64-103`) — to define the "accepts" predicate from a parse
  result.
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/builder — the lowering target;
  `GrammarFormat::Inferred` tags the result.
- Algorithm source: **Apache-2.0 LearnLib** (L\*, TTT, observation table / MAT) —
  *port, not vendor* (idiomatic Rust); see [`library-survey.md`](../library-survey.md)
  §C.3. No GPL/LGPL automata-learning dependency (avoid libalf/flexfringe linkage).

## Acceptance criteria

- [ ] `learn_dfa` against a known reference oracle (e.g. "even number of `a`s")
      returns the **minimal** DFA for that language (correct state count and
      acceptance) using only membership + (approximate) equivalence queries.
- [ ] The observation table enforces **closedness** and **consistency**, and a
      counterexample from the equivalence oracle refines the hypothesis until the
      sampler finds no disagreement within budget.
- [ ] `ParserMembershipOracle` answers membership by dispatching through
      `ParserRegistry` (`src/parser_registry.rs`) with a documented, tested
      acceptance predicate.
- [ ] `learn_grammar` lowers the learned DFA to a valid [`A1`](./A1-grammar-ir.md)
      `Grammar` (right-linear) that **round-trips** through the A1 links encoding
      and parses the same language via [`E2`](./E2-inferred-grammar-runtime-parser.md).
- [ ] Learning is **deterministic** given a fixed `seed` (reproducible sampling).
- [ ] D10 is **opt-in and isolated**: the positive-only core
      ([`D5`](./D5-blackbox-cfg-inference.md), P-3) neither depends on nor invokes
      D10; a build/test confirms the core path needs no oracle.
- [ ] (If TTT shipped) `use_ttt = true` learns the same language as L\* on the
      fixtures, with shorter discriminators on a long-counterexample case.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery `warn`, `Cargo.toml:105-106`), and `cargo test --all-features` all
      pass; `rust-script scripts/check-no-src-tests.rs` passes (tests under
      `tests/`, not `src/`).

## Tests

- Unit (`tests/unit/`, new `grammar_infer_active` module):
  - **L\* core**: a hand-coded ground-truth oracle (closure over a known DFA, e.g.
    "even count of `a`", "(ab)\*", a 3-state identifier lexer) → assert the learned
    DFA is minimal and accepts/rejects a battery of words correctly.
  - **table mechanics**: targeted tests that closedness/consistency violations are
    detected and repaired (assert the table grows the expected `S`/`E`).
  - **counterexample handling**: inject a counterexample and assert the hypothesis
    refines (Angluin and, if present, Rivest–Schapire variants).
  - **lowering**: `Dfa -> Grammar` produces the right rules and round-trips through
    the A1 links encoding (`from_links(to_links(g)) == g`).
  - **determinism**: same `seed` ⇒ identical learned DFA across runs.
- Integration (`tests/integration/`):
  - register a simple parser in a `ParserRegistry`, build a
    `ParserMembershipOracle`, and learn its (regular) language; assert the lowered
    grammar, wrapped via [`E2`](./E2-inferred-grammar-runtime-parser.md), accepts
    the same strings as the original parser on a held-out set.
  - confirm the positive-only [`D5`](./D5-blackbox-cfg-inference.md) path runs with
    **no `Oracle`** (isolation check).
- Oracles in CI are **in-process closures / registered parsers** — no network, no
  external solver, deterministic.

## References

- Angluin, "Learning Regular Sets from Queries and Counterexamples,"
  *Information and Computation* 75(2):87–106, 1987
  ([ScienceDirect](https://www.sciencedirect.com/science/article/pii/0890540187900526))
  — L\*, the MAT model, observation table, closedness/consistency, counterexample
  refinement.
- Isberner, Howar, Steffen, "The TTT Algorithm: A Redundancy-Free Approach to
  Active Automata Learning," RV 2014 — space-optimal discrimination-tree learner
  (implemented in Apache-2.0 LearnLib).
- LearnLib (Apache-2.0) — reference implementation of L\*/TTT/MAT;
  [`library-survey.md`](../library-survey.md) §C.3 (**port, do not bridge**).
- [`literature-review.md`](../literature-review.md) §1 (query-based/active
  learning; "the core deliverable (P-3) must not require it"),
  [`solution-plans.md`](../solution-plans.md) §Epic D (D10) and §4 DAG,
  [`A1`](./A1-grammar-ir.md), [`E2`](./E2-inferred-grammar-runtime-parser.md),
  [`D5`](./D5-blackbox-cfg-inference.md).
