# A2 — Grammar surface syntax (meta-notation-derived)

> **Epic:** A — Meta-grammar foundation · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`D6`](./D6-delimiter-structural-prior.md), [`E4`](./E4-grammar-authoring-ergonomics.md)
> **Requirements:** P-1, P-4, P-8 · **Milestone:** M1
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic A,
> [`existing-capabilities.md`](../existing-capabilities.md) §2,
> [`library-survey.md`](../library-survey.md) §D.1, §D.2.

## Context

[`A1`](./A1-grammar-ir.md) gives a `Grammar` value and a links encoding, but a human
still has no way to *write* a grammar in the meta-language itself. Requirement **P-4** is
explicit — the meta-language is **inherited from meta-notation** ([`requirements.md`](../requirements.md)
P-4 and its "Reading of the ambiguous points"). meta-notation's contribution is a
**lossless delimiter skeleton**: brackets `() {} []` (nested), quotes `'' "" \`\``
(opaque), and unquoted **text blocks** ([`existing-capabilities.md`](../existing-capabilities.md)
§2, [`library-survey.md`](../library-survey.md) §D.1). This repo already mirrors it:
`LinkNetwork::parse(text, language, configuration)` (`src/link_network.rs:350`) parses the
LiNo dialect into a links network, and `to_lino`/`from_lino` (`src/lino_serialization.rs:72,89`)
round-trip a network through links-notation text.

Missing is a concrete **textual surface for grammars** that (a) is parsed by reusing that
skeleton rather than a bespoke lexer, (b) lowers to / lifts from the A1 IR, and (c)
serialises losslessly as LiNo (P-8). This issue delivers it — realising P-4 *for the
grammar surface* — the substrate [`D6`](./D6-delimiter-structural-prior.md) feeds inference
and [`E4`](./E4-grammar-authoring-ergonomics.md) validates.

## Goal

Provide a **meta-notation-derived textual surface syntax** for authoring grammars, with
a parse pipeline `text → delimiter skeleton → A1 Grammar`, a reverse `Grammar → text`
writer, and a LiNo serialisation path, such that a hand-authored grammar round-trips
through both text and links.

## Scope

**In scope**
- A public module `src/grammar/surface/mod.rs` (re-exported from `src/lib.rs`).
- The **concrete surface syntax** (below), expressed entirely with meta-notation's
  delimiter families (`()`, `{}`, `[]`, quotes, text blocks).
- `parse_grammar_surface(text) -> Result<Grammar, GrammarSurfaceError>` and
  `write_grammar_surface(grammar) -> String` (lower / lift the A1 IR).
- A LiNo bridge reusing `LinkNetwork::to_lino`/`from_lino` (no second serializer).
- The text round-trip (`parse ∘ write`) and the LiNo round-trip as headline tests.

**Out of scope** (owned elsewhere)
- The IR itself, builder, and the `ToLinks`/`FromLinks` links encoding → [`A1`](./A1-grammar-ir.md).
- Seeding grammar-construct *concepts* / concept tags on nodes → [`A3`](./A3-grammar-concept-ontology.md).
- Importing **foreign** grammar formats (BNF/EBNF/ABNF/PEG/…) → [`B1`](./B1-bnf-importer.md)–B7.
- Emitting foreign formats or code → C1–C7.
- Feeding the delimiter skeleton into inference as a structural prior → [`D6`](./D6-delimiter-structural-prior.md).
- Validation diagnostics / friendly errors beyond a parse error type → [`E4`](./E4-grammar-authoring-ergonomics.md).

## Design / specification

### Surface concrete syntax

The surface is a **superset of the meta-notation delimiter skeleton**: every token is
either a delimiter group (`(...)`, `{...}`, `[...]`), an opaque quoted string
(`"..."`, `'...'`, `` `...` ``), or a text block (a bare identifier / operator run).
A grammar is a sequence of **rule definitions**, each a LiNo-style named relation
`(name: <expr>)` so the surface is itself valid links-notation. Operators inside an
expression mirror PEG/EBNF and map 1-to-1 onto A1 `GrammarExpr` variants.

| Surface form | Meaning | A1 `GrammarExpr` |
|---|---|---|
| `(name: e)` | rule definition (top level) | `GrammarRule { name, expr: e, .. }` |
| `name` (bare, in RHS) | non-terminal reference | `NonTerminal(name)` |
| `"lit"` / `'lit'` | literal terminal (opaque quote) | `Terminal(lit)` |
| `` `lit` `` | case-insensitive literal | `TerminalInsensitive(lit)` |
| `[a-z]` / `['a' 'z']` | char range | `CharRange('a','z')` |
| `[a b c]` | char class | `CharClass { negated: false, items }` |
| `[^ a b]` | negated char class | `CharClass { negated: true, items }` |
| `.` | any char | `AnyChar` |
| `a b c` (juxtaposition) | sequence | `Sequence([a,b,c])` |
| `a / b` | **ordered** choice (PEG) | `Choice { ordered: true, .. }` |
| `a \| b` | **unordered** choice (CFG) | `Choice { ordered: false, .. }` |
| `e ?` | optional | `Optional(e)` |
| `e *` | zero-or-more | `ZeroOrMore(e)` |
| `e +` | one-or-more | `OneOrMore(e)` |
| `e { m , n }` | counted repetition | `Repeat { expr, min: m, max: Some(n) }` |
| `e { m , }` | unbounded repetition | `Repeat { expr, min: m, max: None }` |
| `& e` | positive predicate | `And(e)` |
| `! e` | negative predicate | `Not(e)` |
| `{ label : e }` | labelled capture/binding | `Capture { label: Some(label), expr } ` |
| `( )` empty group | epsilon | `Empty` |

A small example grammar written in the surface (arithmetic with precedence):

```text
(expr:  term ("+" / "-" term)*)
(term:  factor ("*" / "/" factor)*)
(factor: number / "(" expr ")")
(number: [0-9]+)
```

Note: `(...)` is reused both for **rule definitions** (when the first token is
`name:`) and for **grouping** inside an expression — exactly the dual role
`src/lino_parser.rs:104-156` already gives parentheses. The first rule is the start
symbol unless an explicit `(start: name)` directive is present.

### Parse pipeline (`text → delimiter skeleton → A1 Grammar`)

Three stages, each a private function in `src/grammar/surface/mod.rs`:

1. **Skeletonise.** Run the text through the LiNo delimiter pass —
   `LinkNetwork::parse(text, "grammar-surface", ParseConfiguration::default())`
   (`src/link_network.rs:350`) — yielding a network whose top-level named relations are
   rule definitions and whose nested `()`/`[]`/quote/text nodes are the skeleton. **No
   new lexer**: this is the P-4 step, reusing the parenthesis/atom handling of
   `src/lino_parser.rs:95-185`.
2. **Lower skeleton → `GrammarExpr`.** Walk each rule's sub-tree and map every skeleton
   node per the table above, applying postfix (`? * + {m,n}`) / prefix (`& !`) operators
   and choice separators (`/`, `|`). Build with the [`A1`](./A1-grammar-ir.md) builder
   (`seq`, `choice`, `rep0`, `term`, `nt`, …) so construction stays canonical.
3. **Assemble `Grammar`.** Collect rules in source order, resolve the start symbol
   (explicit `(start: name)` or first rule), set
   `source_format = Some(GrammarFormat::MetaLanguage)`.

```rust
/// Error raised while parsing grammar surface text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarSurfaceError {
    /// The delimiter skeleton could not be parsed (unbalanced bracket/quote, …).
    Skeleton { message: String },
    /// A skeleton node had no valid lowering (e.g. a dangling operator).
    Lowering { rule: Option<String>, message: String },
    /// A rule referenced a name that no rule defines (reuses A1 `referenced_nonterminals`).
    UndefinedReference { rule: String, name: String },
}

/// Parses meta-notation-derived grammar surface text into the A1 IR.
pub fn parse_grammar_surface(text: &str) -> Result<Grammar, GrammarSurfaceError>;

/// Lifts an A1 grammar back to canonical surface text.
#[must_use]
pub fn write_grammar_surface(grammar: &Grammar) -> String;
```

### Serialise-back path (`Grammar → text`, `Grammar ↔ LiNo`)

- `write_grammar_surface` emits one `(name: <expr>)` line per rule in `Grammar::rules()`
  order, choosing the canonical operator per variant (table above) and inserting
  grouping parentheses only where precedence requires, so the output reparses equal.
- The **LiNo bridge** reuses A1's links encoding plus the network serialiser:
  `Grammar::to_links` ([`A1`](./A1-grammar-ir.md)) → `LinkNetwork::to_lino`
  (`src/lino_serialization.rs:72`) for text; `LinkNetwork::from_lino` +
  `Grammar::from_links` for the reverse. Surface text is the *human* form; LiNo is the
  *canonical storage* form (P-8). Both must round-trip — no second serializer.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/surface/mod.rs` | New. `GrammarSurfaceError`, `parse_grammar_surface`, `write_grammar_surface`, the three private pipeline stages, and the LiNo bridge helpers. |
| `src/grammar/mod.rs` | Add `pub mod surface;` (module created by [`A1`](./A1-grammar-ir.md)). |
| `src/lib.rs` | `pub use grammar::surface::{parse_grammar_surface, write_grammar_surface, GrammarSurfaceError};` next to the A1 re-exports (`src/lib.rs:44,60-61`). |
| `tests/unit/mod.rs` | Reuse/extend the existing `mod grammar_parsing;` registration, or add a `grammar_surface` module. |
| `tests/fixtures/grammar/surface/` | A few `.mlg` surface inputs (arithmetic, a recursive list, one using every operator). |
| `changelog.d/` | Add a fragment (see `scripts/check-changelog-fragment.rs` / `scripts/create-changelog-fragment.rs`). |

## Reuse

- **meta-notation delimiter model** — the mandated P-4 basis; consumed via the LiNo
  parser, not forked ([`library-survey.md`](../library-survey.md) §D.1,
  [`existing-capabilities.md`](../existing-capabilities.md) §2).
- `LinkNetwork::parse` (`src/link_network.rs:350`) + `src/lino_parser.rs:95-185` — the
  skeleton pass; do **not** write a new lexer.
- `LinkNetwork::to_lino` / `from_lino` (`src/lino_serialization.rs:72,89`;
  `LinoSerializationError` at `src/lib.rs:61`) — the LiNo round-trip.
- A1 `Grammar`/`GrammarExpr`/builder + `ToLinks`/`FromLinks` ([`A1`](./A1-grammar-ir.md))
  — construction and links encoding (no second encoder here).
- `links-notation` 0.13 (`Cargo.toml:53`) — underlying parser (Unlicense,
  [`library-survey.md`](../library-survey.md) §D.2).

## Acceptance criteria

- [ ] `parse_grammar_surface` parses each surface fixture into a `Grammar` whose rules,
      start symbol, and `source_format = Some(GrammarFormat::MetaLanguage)` match the
      documented mapping table.
- [ ] Every surface form in the table lowers to the documented `GrammarExpr` variant
      (one assertion per row, over the "every operator" fixture).
- [ ] `write_grammar_surface` is a left inverse: `parse(write(g)) == g` (structural
      `PartialEq`) for every fixture grammar and for grammars hand-built via the A1
      builder.
- [ ] LiNo round-trip holds: a grammar → links → `to_lino` → `from_lino` → links →
      `Grammar` equals the original (reuses the A1 round-trip invariant).
- [ ] Malformed surface text (unbalanced delimiter, dangling operator, undefined
      non-terminal) yields the appropriate `GrammarSurfaceError` variant — **never a
      panic**; undefined references reuse A1 `referenced_nonterminals()`.
- [ ] No new lexer is introduced; the skeleton stage calls `LinkNetwork::parse` (or the
      LiNo parser) — verifiable by inspection / a test asserting the intermediate
      network is non-empty.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (`grammar_surface`, or extend `grammar_parsing`):
  - parse each fixture; assert rule names, count, start symbol, and a spot-checked tree.
  - the "every operator" fixture: one assertion per mapping-table row.
  - text round-trip `parse(write(g)) == g` for ~5 grammars (incl. the recursive
    arithmetic example and a deeply nested one).
  - LiNo round-trip `from_lino(to_lino(g_net)) == g_net` reusing A1 helpers.
  - malformed inputs (unbalanced `(`/`[`/quote; trailing `*`; undefined symbol) → the
    right `GrammarSurfaceError`, asserting no panic.
- Fixtures inline or under `tests/fixtures/grammar/surface/`; pure in-process, no IO.

## References

- meta-notation: <https://github.com/link-foundation/meta-notation> · links-notation
  (LiNo): <https://github.com/link-foundation/links-notation>.
- [`library-survey.md`](../library-survey.md) §D.1 (inheritance root), §D.2 (LiNo);
  [`existing-capabilities.md`](../existing-capabilities.md) §2;
  [`solution-plans.md`](../solution-plans.md) §Epic A.
- LiNo serialisation contract: `src/lino_serialization.rs:1-30`; LiNo parser:
  `src/lino_parser.rs:95-185`; sibling keystone: [`A1`](./A1-grammar-ir.md).
