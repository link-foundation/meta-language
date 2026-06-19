# D2 — Tokenisation / lexical-class inference

> **Epic:** D — Inference engine · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** (feeds [`D3`](./D3-state-merging-regular-inference.md), [`D5`](./D5-blackbox-cfg-inference.md))
> **Requirements:** P-2 · **Milestone:** M3
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic D & §3 (row D2),
> [`literature-review.md`](../literature-review.md) §2, §4.

## Context

Every black-box CFG-inference system in the competition line works over a stream
of **tokens**, not raw characters: TreeVada/Arvada bubble *token sequences* into
non-terminals, and GLADE generalises *substrings* into regular fragments
([`literature-review.md`](../literature-review.md) §4). Before the structural
passes ([`D3`](./D3-state-merging-regular-inference.md),
[`D4`](./D4-sequitur-compression.md), [`D5`](./D5-blackbox-cfg-inference.md)) can
find hierarchy, the example texts must be cut into tokens and the recurring
literals generalised into **lexical classes** (identifiers, numbers, whitespace,
punctuation), which lower into [`A1`](./A1-grammar-ir.md)'s `CharRange` /
`CharClass` / `Terminal` expressions. This issue delivers that lexer layer: the
bridge from "a `Vec<&str>` of example texts" to "a token alphabet + a set of
terminal-class rules in the A1 IR" that the CFG layer consumes.

Nothing in the crate does this today: the existing tree-sitter and LiNo parsers
tokenise with *hand-written* grammars; there is no inference of token classes
from examples ([`existing-capabilities.md`](../existing-capabilities.md) §3, row
"No grammar inference of any kind").

## Goal

Given a corpus of example texts (positive only, P-2/P-3), infer (a) a **token
boundary segmentation** of each text, (b) a small set of **lexical classes**
generalised from the observed literals, and (c) the corresponding terminal rules
in the [`A1`](./A1-grammar-ir.md) IR (`CharRange`/`CharClass`/`Terminal`), so the
downstream CFG inference operates on a clean token stream rather than raw bytes.

## Scope

**In scope**
- A new module `src/grammar/inference/lexical.rs` (under the `inference::`
  namespace introduced by [`D1`](./D1-inference-evaluation-harness.md)).
- **Character-category classification** (Unicode-aware) of every char in the
  corpus into coarse categories.
- **Token-boundary detection** by category-change + maximal-munch segmentation.
- **Lexical-class generalisation**: cluster observed tokens into classes
  (keyword/literal vs. open class) and lower each class to an `A1` expression.
- A `LexicalModel` result type (the inferred alphabet + classes + a tokeniser fn)
  and `infer_lexical_classes(corpus) -> LexicalModel`.

**Out of scope** (owned elsewhere)
- Structural / hierarchical grammar over the token stream →
  [`D3`](./D3-state-merging-regular-inference.md) (regular),
  [`D4`](./D4-sequitur-compression.md) (compression),
  [`D5`](./D5-blackbox-cfg-inference.md) (CFG).
- The delimiter/bracket structural prior (that is meta-notation's job) →
  [`D6`](./D6-delimiter-structural-prior.md). D2 must, however, *preserve* delimiter
  characters as their own single-char tokens so D6 can key off them.
- Scoring inferred grammars → [`D1`](./D1-inference-evaluation-harness.md).
- Stochastic token modelling (frequencies) beyond what class-formation needs —
  probabilistic regular inference is [`D3`](./D3-state-merging-regular-inference.md)'s
  ALERGIA path.

## Design / specification

### Step 1 — Character-category classification

Map each `char` in the corpus to a coarse category (Unicode-aware via
`char::is_*` methods — `is_alphabetic`, `is_numeric`, `is_whitespace`,
`is_alphanumeric`; no external Unicode-tables crate needed for the coarse split):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CharCategory {
    Letter,        // is_alphabetic
    Digit,         // is_ascii_digit / is_numeric
    Whitespace,    // is_whitespace
    Delimiter,     // one of ( ) [ ] { } and the quote chars — kept atomic for D6
    Punctuation,   // other ASCII punctuation / symbol
    Other,         // everything else (control, emoji, …)
}
pub fn categorise(c: char) -> CharCategory;
```

The `Delimiter` set is exactly meta-notation's skeleton characters
(`() {} [] ' " `` ` ``) ([`existing-capabilities.md`](../existing-capabilities.md)
§2; [`library-survey.md`](../library-survey.md) §D.1) so D6 can recognise them
later — each delimiter is always its own token.

### Step 2 — Token-boundary detection (segmentation)

Maximal-munch segmentation driven by category runs:
1. Walk the text left to right. Start a new token whenever the `CharCategory`
   changes, **except** that a `Letter` may be followed by `Digit` within the same
   token (so `var123` is one identifier-like token) — this single, documented
   merge rule captures the common "identifier" lexeme without a learned automaton.
2. Each `Delimiter` and each `Punctuation` char is emitted as its own
   single-character token (operators are handled as multi-char only if the same
   punctuation run recurs identically across the corpus — see Step 3).
3. Whitespace runs become `Whitespace` tokens (kept, not discarded — losslessness;
   a later pass may mark them `Silent`/skippable via `A1`'s `RuleKind::Silent`).

The output per text is a `Vec<Token>` where `Token { text: String, category: CharCategory, span: ByteRange }`. Reuse `ByteRange` from `src/source.rs` (re-exported
`src/lib.rs` `ByteRange, Point, SourceSpan`) for spans so tokens align with the
lossless links model.

### Step 3 — Lexical-class generalisation

Cluster the observed tokens into **lexical classes**, deciding per cluster
whether it is a *closed* class (a fixed literal — a keyword/operator that recurs
verbatim) or an *open* class (a generalisable pattern — identifiers, numbers):

1. Group tokens by `CharCategory`.
2. Within each group, count distinct surface forms. A surface form whose
   frequency is high and whose set of forms is *small and stable* (e.g. `if`,
   `else`, `==`, `;`) is a **closed/literal** class → keep as `Terminal`.
3. A group with *many* distinct forms over a regular character set (e.g. dozens
   of identifiers, all `Letter (Letter|Digit)*`) is an **open** class → generalise
   to a `CharClass`/`CharRange`-based pattern:
   - Compute the union of characters seen in each position-class and emit the
     smallest covering `A1` expression: e.g. identifiers →
     `Sequence([CharClass{letters}, ZeroOrMore(CharClass{letters ∪ digits})])`;
     integers → `OneOrMore(CharRange('0','9'))`.
4. The "closed vs. open" threshold is a single documented parameter
   (`max_closed_forms`, default small, e.g. 12) chosen so punctuation/keywords
   stay literal while identifier/number classes generalise. It must be a field on
   a `LexicalConfig` so callers/tests can tune it deterministically.

### Step 4 — Lower to the A1 IR

Each inferred class becomes a `GrammarRule` whose `expr` is the generalised
expression and whose `kind` is `RuleKind::Token` (matching A1's lexer-rule
modifier). Closed-class literals may instead be inlined as `Terminal` at use
sites. The result is attached to a `LexicalModel`:

```rust
pub struct LexicalModel {
    pub alphabet: Vec<char>,            // distinct chars seen, sorted
    pub classes: Vec<GrammarRule>,      // RuleKind::Token rules in the A1 IR
    pub config: LexicalConfig,
}
impl LexicalModel {
    /// Re-tokenise a (possibly unseen) text against the inferred classes.
    pub fn tokenize(&self, text: &str) -> Vec<Token>;
}

pub struct LexicalConfig { pub max_closed_forms: usize, /* … */ }

pub fn infer_lexical_classes(corpus: &[&str], config: &LexicalConfig) -> LexicalModel;
```

`tokenize` lets D3/D5 re-segment new strings (e.g. samples drawn by
[`D1`](./D1-inference-evaluation-harness.md)) consistently with the inferred
classes. Determinism (P-7): no randomness; given the same corpus + config the
`LexicalModel` is byte-identical.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/inference/mod.rs` | Add `pub mod lexical;` (create the module if [`D1`](./D1-inference-evaluation-harness.md) has not yet). |
| `src/grammar/inference/lexical.rs` | New. `CharCategory`, `categorise`, `Token`, `LexicalConfig`, `LexicalModel`, `infer_lexical_classes`, `tokenize`, the segmentation + generalisation logic. |
| `src/lib.rs` | Add `pub use grammar::inference::lexical::{CharCategory, Token, LexicalModel, LexicalConfig, infer_lexical_classes};`. |
| `tests/unit/mod.rs` | Register a new `inference_lexical` unit-test module (mirror `grammar_parsing`). |
| `changelog.d/` | Add a fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- **[`A1`](./A1-grammar-ir.md) IR** — `GrammarExpr::{Terminal, CharRange, CharClass}`,
  `CharClassItem`, `GrammarRule`, `RuleKind::Token`, and the builder are exactly
  the targets every inferred class lowers into; reuse them rather than inventing a
  lexical representation.
- **`ByteRange`/`Point`/`SourceSpan`** (`src/source.rs`, re-exported in
  `src/lib.rs`) for token spans, keeping tokens aligned with the lossless links
  model used everywhere else in the crate.
- **`src/parity.rs` example corpora** — `LANGUAGE_FIXTURES`
  (`src/parity.rs:545-568`, accessor `source()`) provide real multi-language
  source strings to exercise category classification and class formation;
  re-exported at `src/lib.rs:66-72`.
- **`std::char` predicates** for Unicode-coarse categorisation — no external crate
  (avoid pulling a Unicode-tables dependency for a coarse split).

## Acceptance criteria

- [ ] `CharCategory`, `categorise`, `Token`, `LexicalConfig`, `LexicalModel`,
      `infer_lexical_classes`, and `LexicalModel::tokenize` are public and
      documented (crate denies missing docs if configured — check `src/lib.rs`).
- [ ] Delimiter characters (`() {} [] ' " `` ` ``) are always single-character
      tokens categorised `Delimiter` (so [`D6`](./D6-delimiter-structural-prior.md)
      can key off them).
- [ ] On a simple corpus (identifiers + integers + keywords + punctuation):
      identifiers and integers become **open** classes lowered to
      `CharClass`/`CharRange` expressions; keywords/operators stay **closed**
      `Terminal`s; `infer_lexical_classes` is deterministic for a fixed corpus +
      config.
- [ ] `LexicalModel::tokenize` re-segments an unseen string consistently with the
      inferred classes (round-trip: concatenating token `text` reproduces the
      input — losslessness).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic /
      nursery `warn` per `Cargo.toml` `[lints.clippy]`), `cargo test --all-features`
      pass; `rust-script scripts/check-no-src-tests.rs` passes (tests under
      `tests/`).

## Tests

- `tests/unit/` (new `inference_lexical` module):
  - `categorise` maps representative chars to the right `CharCategory` (letters,
    digits, whitespace, each delimiter, punctuation, a non-ASCII char).
  - Segmentation: `"foo123 + bar;"` → expected token sequence with categories and
    spans; `var123` is a single token (Letter-then-Digit merge rule).
  - Class formation: a corpus of many identifiers + a few keywords yields one open
    identifier class (a `CharClass` expr) and the keywords as literals; tune
    `max_closed_forms` and assert the closed/open decision flips at the boundary.
  - Determinism: same corpus + config ⇒ byte-identical `LexicalModel`.
  - Lossless re-tokenisation: `tokenize(s)` token texts concatenate back to `s`.
- Pure in-process, no network. Fixtures inline or under
  `tests/fixtures/grammar/lexical/`.

## References

- TreeVada / Arvada / GLADE operate on token/substring streams — motivates a lexer
  layer feeding the CFG passes; [`literature-review.md`](../literature-review.md)
  §4, [`competitive-analysis.md`](../competitive-analysis.md) §1.
- de la Higuera, *Grammatical Inference*, 2010 — regular/lexical inference context;
  [`literature-review.md`](../literature-review.md) §0, §2.
- meta-notation delimiter skeleton (the `Delimiter` set) —
  [`library-survey.md`](../library-survey.md) §D.1,
  [`existing-capabilities.md`](../existing-capabilities.md) §2.
- [`solution-plans.md`](../solution-plans.md) §3 row D2 (Addresses P-2).
