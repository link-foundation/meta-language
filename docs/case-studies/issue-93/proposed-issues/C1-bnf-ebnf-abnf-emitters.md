# C1 — BNF / EBNF / ABNF text emitters

> **Epic:** C — Emitters & codegen · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** E1, [`F2`](./F2-grammar-format-fidelity-matrix.md)
> **Requirements:** P-9 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §B, [`existing-capabilities.md`](../existing-capabilities.md) §1.

## Context

Requirement P-9 ("translate the meta-grammar to … other languages") needs the
inverse of the Epic B importers: take an [`A1`](./A1-grammar-ir.md) `Grammar` and
render it back out as concrete grammar-notation text. The three classical
notations — **BNF**, **EBNF (ISO/IEC 14977)**, and **ABNF (RFC 5234)** — are the
first targets because they pair directly with the [`B1`](./B1-bnf-importer.md)
(BNF), B2 (EBNF), and B3 (ABNF) importers to give round-trip fidelity (measured
in [`F2`](./F2-grammar-format-fidelity-matrix.md)). They are also the simplest
emitters: each is a thin recursive walk of the `GrammarExpr` tree that produces a
string. This issue delivers all three in one module and establishes the emitter
pattern that [`C2`](./C2-peg-emitter.md) and [`C3`](./C3-gbnf-emitter.md) follow.

Per [`solution-plans.md`](../solution-plans.md) §Epic C, emitters **reuse the
existing `TranslationTemplate` engine** (`src/translation_rules.rs:300-366`,
re-exported at `src/lib.rs:101-104`) for the per-rule line shape instead of
hand-rolling format strings, so the textual surface is produced by the same
declarative substrate the rest of the crate uses for reconstruction
(`reconstruct_text_as`, `src/reconstruction.rs:14`, documented in `README.md:248`).

## Goal

Provide three public functions — `emit_bnf`, `emit_ebnf`, `emit_abnf` — that turn
a `Grammar` into valid text in each notation, losslessly enough that re-importing
through B1/B2/B3 yields a structurally equal `Grammar` (the F2 round-trip). Each
emitter is a `GrammarExpr` walk; the per-rule `name <sep> body` line is rendered
through a `TranslationTemplate`.

## Scope

**In scope**
- `src/grammar/emit/mod.rs` — shared `GrammarEmitError`, the `TranslationTemplate`
  helper, and re-exports of the three fns.
- `src/grammar/emit/bnf.rs`, `.../ebnf.rs`, `.../abnf.rs` — one walk each.
- The `GrammarExpr → notation` mapping tables below, including the documented
  fallback/error for variants a notation cannot express natively.
- Deterministic output (stable rule order from `Grammar::rules()`; no `HashMap`
  iteration), so byte output is reproducible for golden tests.

**Out of scope** (owned elsewhere)
- PEG/`.pest` emit → [`C2`](./C2-peg-emitter.md); GBNF emit → [`C3`](./C3-gbnf-emitter.md).
- Rust/JS parser codegen → C4/C5; tree-sitter `grammar.js` → C7.
- The importers themselves → [`B1`](./B1-bnf-importer.md)/B2/B3.
- The cross-format **round-trip test matrix** → [`F2`](./F2-grammar-format-fidelity-matrix.md)
  (this issue ships an emit-then-reimport smoke test per format; the full matrix is F2's).
- A shared `GrammarEmitter` trait abstraction — define the three free fns and a
  minimal `GrammarEmitError` here; if a trait emerges, refactor in C2 (note it, don't block).

## Design / specification

Each emitter walks `Grammar::rules()` (order-preserving, `A1`) and, for each
`GrammarRule`, renders one production line, then walks `rule.expr` recursively.
A notation that has no native form for a variant uses the documented **fallback**
(an equivalent expansion) where one is faithful, or returns
`GrammarEmitError::Unsupported` where none is — never silent data loss.

### Per-rule line shape (via `TranslationTemplate`)

Build one `TranslationTemplate` per notation for the production line and call
`TranslationTemplate::render` is internal; for code-side emission use
`TranslationTemplate::source()`-style substitution with the two placeholders
`{name}` and `{body}`. Concretely, hold the template string as a constant and
substitute (the engine treats `{{`/`}}` as literal braces, so emit those when a
notation needs literal braces — relevant to EBNF `{ }`):

| Notation | Production-line template | Alternation sep | Def operator | Terminator |
|---|---|---|---|---|
| BNF | `<{name}> ::= {body}` | ` \| ` | `::=` | newline |
| EBNF | `{name} = {body} ;` | ` \| ` | `=` | `;` |
| ABNF | `{name} = {body}` | ` / ` | `=` (`=/` for incremental, out of scope) | CRLF or `\n` |

### `GrammarExpr → BNF`

BNF has no repetition/optional/predicate operators, so those lower to **named
helper rules** (the inverse of how the `bnf` crate normalises groups to `__anon*`
non-terminals — [`library-survey.md`](../library-survey.md) §B PART 2).

| `GrammarExpr` | BNF output | Notes |
|---|---|---|
| `Empty` | *(empty alternative)* | nothing between `\|` / after `::=` |
| `Terminal(s)` | `"s"` | double-quote; escape `"` and `\` |
| `TerminalInsensitive(s)` | `"s"` + `Unsupported` if strictness required | classic BNF has no case flag; emit literal and record a `GrammarEmitError::Lossy` note (see below) |
| `CharRange(a,b)` | helper rule `<range_a_b> ::= "a" \| … \| "b"` | BNF has no ranges; expand or `Unsupported` if range is huge (> 256 chars) |
| `CharClass{negated:false, items}` | `\|`-joined chars/ranges as a helper rule | expand each `CharClassItem` |
| `CharClass{negated:true, …}` | `GrammarEmitError::Unsupported` | BNF cannot express negation |
| `AnyChar` | `GrammarEmitError::Unsupported` | no `.` in BNF |
| `NonTerminal(n)` | `<n>` | |
| `Choice{alternatives}` | `a \| b \| c` | `ordered` flag ignored (BNF is unordered); see Lossy note |
| `Sequence(items)` | `a b c` | space-joined |
| `Optional(e)` | helper rule `<opt_e> ::= {emit e} \| ` (empty alt) | |
| `ZeroOrMore(e)` | recursive helper `<star_e> ::= {emit e} <star_e> \| ` | classic right-recursive star |
| `OneOrMore(e)` | recursive helper `<plus_e> ::= {emit e} <plus_e> \| {emit e}` | |
| `Repeat{e,min,max}` | expand to `min` copies + `(max-min)` optional copies; `max:None` → use `<star_e>` | |
| `And(e)` / `Not(e)` | `GrammarEmitError::Unsupported` | PEG predicates have no BNF form — documented rejection |
| `Capture{expr,..}` | emit `expr`, drop the label | BNF has no captures |

### `GrammarExpr → EBNF (ISO/IEC 14977)`

EBNF *does* have `{ }` (repeat zero-or-more), `[ ]` (optional), `( )` grouping,
and `,` concatenation ([`library-survey.md`](../library-survey.md) §B PART 1).

| `GrammarExpr` | EBNF output | Notes |
|---|---|---|
| `Empty` | `` (empty) or `''` | empty alternative |
| `Terminal(s)` | `"s"` (or `'s'` if `s` contains `"`) | ISO 14977 allows either quote |
| `TerminalInsensitive(s)` | `"s"` + Lossy note | ISO EBNF has no case-insensitive literal |
| `CharRange(a,b)` | helper rule (no native range) **or** W3C-style note | ISO 14977 lacks `[a-z]`; expand to a helper as in BNF |
| `CharClass` | helper rule | as BNF; `negated:true` → `Unsupported` |
| `AnyChar` | `? any character ?` (special-sequence) | ISO 14977 special-sequence escape hatch |
| `NonTerminal(n)` | `n` | bare identifier |
| `Choice{alternatives}` | `a \| b \| c` | |
| `Sequence(items)` | `a , b , c` | comma-concatenation per ISO |
| `Optional(e)` | `[ {emit e} ]` | native |
| `ZeroOrMore(e)` | `{ {emit e} }` | native — emit literal braces (escape as `{{`/`}}` through the template) |
| `OneOrMore(e)` | `{emit e} , { {emit e} }` | one then repeat |
| `Repeat{e,min,max}` | `min` × `e` then `(max-min)` × `[e]`; unbounded → `{ e }` | |
| `And`/`Not` | `Unsupported` | no PEG predicates in ISO EBNF |
| `Capture{expr,..}` | emit `expr`, drop label | |

### `GrammarExpr → ABNF (RFC 5234 + RFC 7405)`

ABNF natively has repetition prefixes `*` / `n*m` / `nA`, optional `[ ]`,
alternation `/`, grouping `( )`, value ranges `%xNN-MM`, and case sensitivity via
`%s` (RFC 7405) — [`library-survey.md`](../library-survey.md) §B PART 1.

| `GrammarExpr` | ABNF output | Notes |
|---|---|---|
| `Empty` | `""` | ABNF empty string |
| `Terminal(s)` | `%s"s"` (case-sensitive, RFC 7405) | ABNF strings are case-**insensitive** by default; use `%s` to be faithful to A1 |
| `TerminalInsensitive(s)` | `"s"` or `%i"s"` | native — this is ABNF's default semantics |
| `CharRange(a,b)` | `%x{a:02X}-{b:02X}` | native value range |
| `CharClass{negated:false, items}` | `( %xNN / %xMM-PP / … )` | map each `CharClassItem` to a value or `%x`-range, `/`-joined |
| `CharClass{negated:true, …}` | `Unsupported` | ABNF has no complement |
| `AnyChar` | `%x00-10FFFF` (or `OCTET` core rule) | document the chosen any-char rule |
| `NonTerminal(n)` | `n` | rulename |
| `Choice{alternatives}` | `a / b / c` | |
| `Sequence(items)` | `a b c` | space-juxtaposition |
| `Optional(e)` | `[ {emit e} ]` | native |
| `ZeroOrMore(e)` | `*( {emit e} )` | native |
| `OneOrMore(e)` | `1*( {emit e} )` | native |
| `Repeat{e,min,max}` | `{min}*{max}( e )`; `max:None` → `{min}*( e )` | native counted repetition |
| `And`/`Not` | `Unsupported` | no PEG predicates in ABNF |
| `Capture{expr,..}` | emit `expr`, drop label | |

### Emit fn signatures and the walk

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarEmitError {
    /// The target notation cannot represent this construct at all.
    Unsupported { format: GrammarFormat, construct: String },
}

/// Non-fatal fidelity notes (e.g. ordered→unordered Choice, dropped case flag).
/// Returned alongside the text so F2 can score round-trip loss precisely.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EmitReport { pub lossy: Vec<String> }

pub fn emit_bnf(grammar: &Grammar)  -> Result<(String, EmitReport), GrammarEmitError>;
pub fn emit_ebnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>;
pub fn emit_abnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>;
```

The walk (shared shape; one private `fn emit_expr(expr, out, ctx, helpers)` per
notation, dispatched by `match expr { … }` over every `GrammarExpr` variant):

1. For each rule in `grammar.rules()`: render the production line via the
   notation's `TranslationTemplate` with `{name}` and `{body}`, where `{body}` is
   `emit_expr(&rule.expr, …)`.
2. `emit_expr` recurses; when it needs a helper rule (e.g. BNF star/range), it
   appends the helper to a `helpers: Vec<GrammarRule>`-style accumulator with a
   deterministic, collision-free name (e.g. `star_<n>`, `range_a_z`), de-duped so
   identical sub-expressions reuse one helper.
3. Parenthesise sub-expressions when precedence requires (a `Choice` inside a
   `Sequence`, etc.) so the emitted text re-parses to the same tree.
4. After the main rules, append all accumulated helper rules.
5. Use `grammar.start_rule()` (A1) to emit the start symbol first where the
   notation cares (GBNF's `root` matters in C3; here start-first is a convention,
   not a requirement).

Helper-rule synthesis is the inverse of B1's note that `bnf` normalises groups to
`__anon*` non-terminals ([`library-survey.md`](../library-survey.md) §B PART 2),
so a `import_bnf(emit_bnf(g))` round-trip is structurally faithful modulo helper
renaming (F2 records this as the expected equivalence class).

## File-level plan

| File | Change |
|---|---|
| `src/grammar/emit/mod.rs` | New. `GrammarEmitError`, `EmitReport`, the per-notation production-line `TranslationTemplate` constants, `pub use` of the three fns. |
| `src/grammar/emit/bnf.rs` | New. `emit_bnf` + BNF `emit_expr` walk + helper-rule synthesis. |
| `src/grammar/emit/ebnf.rs` | New. `emit_ebnf` + EBNF walk. |
| `src/grammar/emit/abnf.rs` | New. `emit_abnf` + ABNF walk (`%x` ranges, `%s`/`%i`). |
| `src/lib.rs` | `pub use grammar::emit::{emit_bnf, emit_ebnf, emit_abnf, GrammarEmitError, EmitReport};` next to existing re-exports (`src/lib.rs:83-104`). |
| `tests/unit/mod.rs` | Register a `grammar_emit` module (alongside the existing `grammar_parsing` at `tests/unit/mod.rs:9`). |
| `tests/fixtures/grammar/emit/` | Golden `.bnf`/`.ebnf`/`.abnf` expected outputs for a few hand-built grammars. |
| `changelog.d/` | Add a fragment (see `scripts/create-changelog-fragment.rs` / `README.md` changelog section). |

## Reuse

- **`TranslationTemplate`** (`src/translation_rules.rs:300-366`, re-exported
  `src/lib.rs:101-104`) — render the `{name} <sep> {body}` production line; its
  `{{`/`}}` literal-brace handling (`src/translation_rules.rs:330-360`) is exactly
  what EBNF's `{ }` repetition needs. **Do not hand-roll `format!` templating** —
  [`solution-plans.md`](../solution-plans.md) §Epic C mandates the engine.
- A1 `Grammar`/`GrammarExpr`/`RuleKind`/`GrammarFormat` and accessors
  (`rules()`, `start_rule()`, `referenced_nonterminals()`) — the IR being walked.
- The `reconstruct_text_as` precedent (`src/reconstruction.rs:14`, `README.md:248`)
  shows the established "IR/network → target text" pattern these emitters mirror.
- No new third-party dependency is required (pure string emission). The crate
  forbids `unsafe` (`Cargo.toml:100-101`) and warns clippy pedantic+nursery
  (`Cargo.toml:103-106`); keep output construction allocation-light and lint-clean.

## Acceptance criteria

- [ ] `emit_bnf`, `emit_ebnf`, `emit_abnf` are public, documented (doc-comment per
      public item), and deterministic (identical input ⇒ byte-identical output).
- [ ] Every `GrammarExpr` variant maps per the three tables above; native EBNF/ABNF
      operators (`{ } [ ] *( ) [ ] %x`) are used where available; helper rules are
      synthesised for BNF repetition/optional/range with stable, de-duplicated names.
- [ ] PEG predicates (`And`/`Not`), negated char classes, and `AnyChar` (in BNF)
      return `GrammarEmitError::Unsupported` with the offending construct named —
      never a panic, never silent loss.
- [ ] Fidelity-reducing conversions (ordered→unordered `Choice`, dropped
      case-sensitivity, dropped capture label) are recorded in `EmitReport.lossy`.
- [ ] The per-rule production line is produced via `TranslationTemplate`, not ad-hoc
      `format!` string concatenation.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:103-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`).

## Tests

- `tests/unit/` (new `grammar_emit` module):
  - For a hand-built `Grammar` exercising every variant, assert each emitter's
    output equals the committed golden file under `tests/fixtures/grammar/emit/`.
  - Assert `And`/`Not`/negated-`CharClass`/`AnyChar`(BNF) yield `Err(Unsupported)`.
  - Assert ordered `Choice` and `TerminalInsensitive` populate `EmitReport.lossy`
    for BNF/ISO-EBNF; assert ABNF emits `%s"…"` (faithful, no lossy entry).
  - Assert EBNF emits literal `{ }`/`[ ]` for `ZeroOrMore`/`Optional`.
- Round-trip smoke (per format; the full matrix is [`F2`](./F2-grammar-format-fidelity-matrix.md)):
  once [`B1`](./B1-bnf-importer.md)/B2/B3 land, `import_*(emit_*(g))` yields a
  `Grammar` structurally equal to `g` modulo helper-rule renaming. Gate behind the
  importer features; mark `#[ignore]` (with a comment pointing at F2) until they exist.
- No network/IO beyond reading committed fixtures; pure in-process.

## References

- BNF / EBNF (ISO/IEC 14977) / ABNF (RFC 5234, RFC 7405):
  [`library-survey.md`](../library-survey.md) §B PART 1.
- Pairing importers: [`B1`](./B1-bnf-importer.md) (and B2/B3) §B PART 2.
- `TranslationTemplate` engine: `src/translation_rules.rs:300-366`,
  [`existing-capabilities.md`](../existing-capabilities.md) §1 (row "`TranslationRule`/…").
- IR being emitted: [`A1`](./A1-grammar-ir.md).
- Round-trip fidelity matrix: [`F2`](./F2-grammar-format-fidelity-matrix.md);
  [`solution-plans.md`](../solution-plans.md) §Epic C, §3 table (row C1), §4 DAG.
