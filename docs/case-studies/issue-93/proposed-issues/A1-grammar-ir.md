# A1 — Grammar IR / expression algebra (the keystone)

> **Epic:** A — Meta-grammar foundation · **Blocked by:** none · **Blocks:** A2, A3, B1–B7, C1–C7, D1–D10, E1, E2, F1, F2
> **Requirements:** P-1, P-5, P-8 · **Milestone:** M1
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic A,
> [`existing-capabilities.md`](../existing-capabilities.md) §3.

## Context

Today `LinkType::Grammar` exists (`src/link_network.rs:51`) and the self-description
roots tie `grammar → language → concept` (`src/self_description.rs:25-34`), **but
nothing constructs grammar links and there is no type that models a grammar**.
Every other issue in this initiative (import, emit, infer, translate, run)
operates on "a grammar," so the in-memory representation and its links encoding
must exist first. This issue defines that representation. It is the single
keystone the dependency DAG is rooted at.

## Goal

Provide a Rust **grammar intermediate representation (IR)** — an expression
algebra rich enough to losslessly hold PEG, (E)BNF, ABNF, and the meta-language's
own grammar — together with a **links encoding** so a grammar is a first-class,
round-trippable value in the existing `LinkNetwork`, tagged `LinkType::Grammar`.

## Scope

**In scope**
- A new public module `src/grammar/mod.rs` (re-exported from `src/lib.rs`).
- The expression algebra enum, the rule type, the grammar type.
- Constructors, accessors, structural equality, `Debug`/`Clone`.
- A links encoding (`ToLinks`/`FromLinks`) with a proven round-trip.
- A small builder API for ergonomic in-code construction (used heavily by tests
  and by every importer/inference issue).

**Out of scope** (owned elsewhere)
- Textual surface syntax / parsing a grammar from meta-notation → **A2**.
- Seeding grammar-construct *concepts* into the ontology → **A3**.
- Importing any concrete format (BNF/PEG/…) → **B1–B7**.
- Emitting any format or code → **C1–C7**.
- Any inference → **D\***.

## Design / specification

Model the algebra on `pest_meta::ast::Expr` (the proven 18-variant PEG algebra)
and `ungrammar`, **normalised** so EBNF/BNF/ABNF all lower into it without loss.

```rust
/// One node of the grammar expression algebra.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarExpr {
    /// Matches the empty string (ε).
    Empty,
    /// Literal string terminal, e.g. "fn".
    Terminal(String),
    /// Case-insensitive literal terminal.
    TerminalInsensitive(String),
    /// Inclusive character range, e.g. 'a'..='z'.
    CharRange(char, char),
    /// Explicit set of characters / ranges (a character class).
    CharClass { negated: bool, items: Vec<CharClassItem> },
    /// The "any character" wildcard (PEG `.`).
    AnyChar,
    /// Reference to another rule by name (non-terminal).
    NonTerminal(String),
    /// Ordered (PEG) or unordered (CFG) alternation; `ordered` distinguishes them.
    Choice { ordered: bool, alternatives: Vec<GrammarExpr> },
    /// Concatenation.
    Sequence(Vec<GrammarExpr>),
    /// `e?`
    Optional(Box<GrammarExpr>),
    /// `e*`
    ZeroOrMore(Box<GrammarExpr>),
    /// `e+`
    OneOrMore(Box<GrammarExpr>),
    /// Counted repetition `e{min[,max]}`; `max: None` means unbounded.
    Repeat { expr: Box<GrammarExpr>, min: usize, max: Option<usize> },
    /// Positive lookahead `&e` (PEG and-predicate).
    And(Box<GrammarExpr>),
    /// Negative lookahead `!e` (PEG not-predicate).
    Not(Box<GrammarExpr>),
    /// Labelled capture / binding `label:e` (label optional).
    Capture { label: Option<String>, expr: Box<GrammarExpr> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharClassItem { Char(char), Range(char, char) }

/// How a rule participates in parsing (mirrors pest's rule modifiers).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleKind { Normal, Atomic, Silent, Token }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRule {
    pub name: String,
    pub expr: GrammarExpr,
    pub kind: RuleKind,
    /// Optional concept-ontology alignment (populated by A3 / inference D9).
    pub concept: Option<String>,
    /// Optional free-text doc/comment carried for round-trip fidelity.
    pub doc: Option<String>,
}

/// Origin format, so emitters/round-trip tests know the source dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarFormat {
    MetaLanguage, Bnf, Ebnf, Abnf, Peg, Antlr, Lark, Gbnf, TreeSitter, Inferred,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grammar {
    rules: Vec<GrammarRule>,          // order-preserving
    start: Option<String>,            // start symbol (defaults to first rule)
    source_format: Option<GrammarFormat>,
}
```

`Grammar` methods (non-exhaustive): `new`, `with_rule`, `rule(&self, name)`,
`rules()`, `start_rule()`, `set_start`, `rule_names()`, `referenced_nonterminals()`
(for undefined-reference checks), and a `builder()` returning a fluent
`GrammarBuilder`/`ExprBuilder` (e.g. `seq([...])`, `choice([...])`, `rep0(e)`,
`term("fn")`, `nt("expr")`). Keep the builder small but complete — every
importer, emitter, inference, and test issue will use it.

### Links encoding

Implement `ToLinks for Grammar` and `FromLinks for Grammar` (the existing
`rust_codec` traits, `src/rust_codec.rs:19`, re-exported at `src/lib.rs:83-86`).
Encoding rules:
- Every grammar/rule/expression node is a `Link` whose metadata `link_type` is
  `LinkType::Grammar` (`src/link_network.rs:51`).
- A rule node references its name token and its expression-root node.
- Each `GrammarExpr` variant maps to a node whose term encodes the variant tag
  (e.g. `grammar::expr::sequence`) and whose children are its sub-expressions, in
  order. Terminals/ranges store their payload as a value/token link.
- Use the same `rust::type::` / term-prefix convention `rust_codec` already uses
  (`src/rust_codec.rs:21-22`) — add a `grammar::` prefix family for the tags.
- **Round-trip invariant:** `Grammar::from_links(&g.to_links(&mut net), ...)` must
  equal `g` for every grammar (structural `PartialEq`). This is the headline test.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/mod.rs` | New. `GrammarExpr`, `CharClassItem`, `RuleKind`, `GrammarRule`, `GrammarFormat`, `Grammar`, builder. |
| `src/grammar/links.rs` | New. `ToLinks`/`FromLinks` impls + the `grammar::` tag constants. |
| `src/lib.rs` | Add `pub mod grammar;` and `pub use grammar::{Grammar, GrammarExpr, GrammarRule, RuleKind, GrammarFormat, CharClassItem};` next to the existing re-exports (`src/lib.rs:1-36, 60`). |
| `tests/unit/mod.rs` | Register a new `grammar_ir` unit-test module. |
| `changelog.d/` | Add a fragment (see CONTRIBUTING / `README.md` changelog section). |

## Reuse

- `rust_codec::{ToLinks, FromLinks, LinksEncoder, LinksDecoder, LinksCodecError}` — encoding plumbing (`src/rust_codec.rs:19`, `src/lib.rs:83-86`).
- `link_network::{LinkNetwork, Link, LinkId, LinkType, LinkMetadata}` — node storage (`src/lib.rs:60`).
- Algebra reference: `pest_meta::ast::Expr` (18 variants) and `ungrammar` — see [`library-survey.md`](../library-survey.md) §A.
- Do **not** add a parser dependency here; A1 is pure data + links.

## Acceptance criteria

- [ ] `GrammarExpr`, `GrammarRule`, `Grammar`, `RuleKind`, `GrammarFormat`,
      `CharClassItem` are public and documented (doc-comment on each public item;
      crate denies missing docs if configured — check `src/lib.rs` lints).
- [ ] A `Grammar` can be built in code via the builder for at least: a literal,
      a char class, a sequence, an ordered choice, `*`/`+`/`?`, counted repetition,
      `&`/`!` predicates, a labelled capture, and a recursive rule.
- [ ] `ToLinks`/`FromLinks` implemented; **round-trip equality** holds for every
      fixture grammar (property-style test over a handful of hand-built grammars).
- [ ] All grammar links carry `LinkType::Grammar`.
- [ ] `referenced_nonterminals()` lets a caller detect references to undefined rules.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (new `grammar_ir` module):
  - build each `GrammarExpr` variant; assert accessors and `Display`/`Debug` shape.
  - round-trip: `from_links(to_links(g)) == g` for ~6 hand-built grammars,
    including a deeply recursive one (e.g. arithmetic expr with precedence) and one
    using every variant at least once.
  - `referenced_nonterminals()` returns the right set; undefined-reference detection.
- No network/IO; pure in-process. Keep fixtures inline or under `tests/fixtures/grammar/`.

## References

- pest_meta `ast::Expr`: <https://docs.rs/pest_meta> · ungrammar: <https://docs.rs/ungrammar>
- [`library-survey.md`](../library-survey.md) §A (IR design), [`existing-capabilities.md`](../existing-capabilities.md) §1, §3.
- Existing codec contract: `src/rust_codec.rs:1-22`.
</content>
