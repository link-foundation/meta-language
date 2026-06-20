# C2 — PEG (`.pest`) emitter

> **Epic:** C — Emitters & codegen · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`F2`](./F2-grammar-format-fidelity-matrix.md)
> **Requirements:** P-9 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §A.1–A.2, [`existing-capabilities.md`](../existing-capabilities.md) §1.

## Context

Requirement P-9 wants a meta-grammar emitted to runnable grammar formats. **PEG**
is the highest-value text target because the [`A1`](./A1-grammar-ir.md) IR was
modelled on `pest_meta::ast::Expr` (the 18-variant PEG algebra —
[`library-survey.md`](../library-survey.md) §A.2), so the `GrammarExpr → .pest`
mapping is **near-isomorphic**: ordered choice, the predicates `&`/`!`, and `*`/
`+`/`?`/`{m,n}` all have native `.pest` syntax. A `.pest` file is also directly
compilable to a Rust parser by `pest_derive` at build time
([`library-survey.md`](../library-survey.md) §A.1), so this emitter is the bridge
into the C4 Rust-codegen path as well as a standalone format target.

This emitter pairs with the **B4 PEG importer** (which lowers `pest_meta`'s
`ast::Expr` into A1) to give a PEG round-trip, scored in
[`F2`](./F2-grammar-format-fidelity-matrix.md). As with
[`C1`](./C1-bnf-ebnf-abnf-emitters.md), the per-rule production line is rendered
through the existing `TranslationTemplate` engine
(`src/translation_rules.rs:300-366`, re-exported at `src/lib.rs:101-104`) rather
than ad-hoc string building ([`solution-plans.md`](../solution-plans.md) §Epic C).

## Goal

Provide `emit_pest(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>`
that renders an A1 `Grammar` as a valid `.pest` grammar string, faithful enough
that B4's `import_pest` re-imports a structurally equal `Grammar` (the F2
round-trip). Optionally note a `peg`/`winnow`-ready variant (see Scope).

## Scope

**In scope**
- `src/grammar/emit/pest.rs` — the `GrammarExpr → .pest` walk and `emit_pest`.
- The mapping table below, including `RuleKind → .pest` rule modifiers
  (`@`/`_`/`$`/`!`).
- Deterministic output (rule order from `Grammar::rules()`).
- `source_format`-aware start handling: emit `grammar.start_rule()` first.

**Out of scope** (owned elsewhere)
- BNF/EBNF/ABNF → [`C1`](./C1-bnf-ebnf-abnf-emitters.md); GBNF → [`C3`](./C3-gbnf-emitter.md).
- **Compiling** the emitted `.pest` to a Rust parser (that is C4, via
  `pest_derive`/`pest_generator`). C2 stops at valid `.pest` text.
- The B4 PEG **importer** (the round-trip partner).
- The full round-trip matrix → [`F2`](./F2-grammar-format-fidelity-matrix.md);
  C2 ships an emit-then-reimport smoke test only.
- An optional **`peg`/`winnow`-ready variant**: the IR maps cleanly to the
  `peg::parser!{}` inline-PEG dialect and to winnow combinator calls
  ([`library-survey.md`](../library-survey.md) §A.4, §A.8). Note the mapping in a
  doc-comment / `## References`-level pointer, but **do not implement it here** —
  combinator/`peg!` Rust codegen is C4's responsibility; flag it, don't block.

## Design / specification

The walk mirrors [`C1`](./C1-bnf-ebnf-abnf-emitters.md): for each
`GrammarRule` in `grammar.rules()`, emit a production line
`{modifier}{name} = {{ {body} }}` via a `TranslationTemplate`, where `{body}` is
`emit_expr(&rule.expr)`. The `{{`/`}}` literal braces match `.pest`'s rule-body
braces and are produced through the template's brace-escaping
(`src/translation_rules.rs:330-360`). Because PEG is the IR's native shape, almost
every variant maps 1:1 and **no helper-rule synthesis is needed** (unlike BNF).

### `GrammarExpr → .pest`

| `GrammarExpr` | `.pest` output | Notes |
|---|---|---|
| `Empty` | *(empty body)* / `("")` | matches ε; `.pest` allows an empty alternative |
| `Terminal(s)` | `"s"` | escape `"` and `\`; `\n`/`\t` as pest escapes |
| `TerminalInsensitive(s)` | `^"s"` | pest's case-insensitive string literal |
| `CharRange(a,b)` | `'a'..'b'` | native pest range (single-quoted chars) |
| `CharClass{negated:false, items}` | nested ordered choice `("a" \| 'x'..'z' \| …)` | pest has no `[...]` class; map each `CharClassItem`: `Char(c)`→`"c"`, `Range(a,b)`→`'a'..'b'`, joined by `\|` |
| `CharClass{negated:true, items}` | `(!(<inner choice>) ~ ANY)` | negation via not-predicate + `ANY` (pest idiom) |
| `AnyChar` | `ANY` | pest built-in any-char rule |
| `NonTerminal(n)` | `n` | bare identifier |
| `Choice{ordered:true, alts}` | `a \| b \| c` | pest `\|` is **ordered** — exact match for ordered choice |
| `Choice{ordered:false, alts}` | `a \| b \| c` + Lossy note | pest cannot express *unordered* choice; emit ordered `\|` and record loss in `EmitReport` (the canonical CFG→PEG caveat) |
| `Sequence(items)` | `a ~ b ~ c` | pest sequence operator `~` |
| `Optional(e)` | `(e)?` | native |
| `ZeroOrMore(e)` | `(e)*` | native |
| `OneOrMore(e)` | `(e)+` | native |
| `Repeat{e,min,max}` | `(e){min,max}` / `(e){min,}` (`max:None`) / `(e){min}` (`min==max`) | pest counted repetition |
| `And(e)` | `&(e)` | native positive lookahead |
| `Not(e)` | `!(e)` | native negative lookahead |
| `Capture{label:Some(l), expr}` | `(e)` + push label into `EmitReport`/comment | pest has no inline label syntax (captures come from rule structure); see note |
| `Capture{label:None, expr}` | `(e)` | label-less capture is just the sub-expr |

Parenthesise any sub-expression whose precedence is below its parent (`Choice`
inside `Sequence`, a repetition applied to a multi-token sequence, a predicate on
a group) so the emitted text re-parses to the same tree. pest precedence
(tightest→loosest): postfix `* + ? {}` > prefix `& !` > sequence `~` > choice `|`.

### `RuleKind → .pest` rule modifier

A1's `RuleKind` (`Normal`/`Atomic`/`Silent`/`Token`) maps onto pest's rule
modifier sigils ([`library-survey.md`](../library-survey.md) §A.2, `RuleType`
Normal/Silent/Atomic/CompoundAtomic/NonAtomic):

| `RuleKind` | `.pest` modifier prefix | Meaning |
|---|---|---|
| `Normal` | *(none)* — `name = { … }` | ordinary rule |
| `Atomic` | `@` — `name = @{ … }` | atomic (no implicit whitespace, no inner tokens) |
| `Silent` | `_` — `name = _{ … }` | silent (produces no token/pair) |
| `Token` | `$` — `name = ${ … }` | compound-atomic (keeps inner pairs, atomic boundary) |

Document the chosen `Token → $` (compound-atomic) decision in the doc-comment; it
is the closest pest analogue and is what B4 must invert.

### Emit fn signature and walk

```rust
// Reuses GrammarEmitError / EmitReport from src/grammar/emit/mod.rs (C1).
pub fn emit_pest(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>;
```

1. For each rule in `grammar.rules()`: pick the modifier from `rule.kind`, then
   render `{modifier}{name} = {{ {body} }}` via the production-line
   `TranslationTemplate`; `{body}` = `emit_expr(&rule.expr)`.
2. `emit_expr` matches every `GrammarExpr` variant per the table, parenthesising
   by precedence.
3. Emit `grammar.start_rule()` first (pest has no mandatory start rule, but
   start-first is a readable, stable convention).
4. If `rule.doc` is `Some`, emit it as a leading `//` line comment (round-trip of
   the A1 `doc` field).
5. Unordered `Choice` and label-bearing `Capture` add `EmitReport.lossy` notes;
   nothing in pure PEG is `Unsupported` (the IR is PEG-shaped), so `emit_pest`
   only errors on internal invariant violations (e.g. an empty `Choice`).

## File-level plan

| File | Change |
|---|---|
| `src/grammar/emit/pest.rs` | New. `emit_pest` + the `GrammarExpr`/`RuleKind` walk. |
| `src/grammar/emit/mod.rs` | Add `pub use pest::emit_pest;` (reuses `GrammarEmitError`/`EmitReport` defined for [`C1`](./C1-bnf-ebnf-abnf-emitters.md)). |
| `src/lib.rs` | `pub use grammar::emit::emit_pest;` next to existing re-exports (`src/lib.rs:83-104`). |
| `tests/unit/mod.rs` | Extend the `grammar_emit` module (registered for C1 at `tests/unit/mod.rs`). |
| `tests/fixtures/grammar/emit/` | Golden `.pest` outputs for a few hand-built grammars (incl. one using `&`/`!` and each `RuleKind`). |
| `changelog.d/` | Add a fragment. |

## Reuse

- **`TranslationTemplate`** (`src/translation_rules.rs:300-366`, re-exported
  `src/lib.rs:101-104`) — production-line rendering with `{{`/`}}` for pest's
  body braces. Mandated over `format!` by [`solution-plans.md`](../solution-plans.md) §Epic C.
- A1 `Grammar`/`GrammarExpr`/`RuleKind` (the algebra is `pest_meta`-shaped, so the
  mapping is direct) — [`A1`](./A1-grammar-ir.md),
  [`library-survey.md`](../library-survey.md) §A.2.
- `GrammarEmitError`/`EmitReport` from [`C1`](./C1-bnf-ebnf-abnf-emitters.md)'s
  `src/grammar/emit/mod.rs` — do not redefine.
- No new dependency: the emitted `.pest` is plain text; the optional B4 importer
  is what pulls in `pest_meta`. `unsafe` is forbidden (`Cargo.toml:100-101`);
  clippy pedantic+nursery are `warn` (`Cargo.toml:103-106`).

## Acceptance criteria

- [ ] `emit_pest` is public, documented, and deterministic (byte-stable output).
- [ ] Every `GrammarExpr` variant maps per the table; ordered `Choice → |`,
      `Sequence → ~`, `ZeroOrMore → *`, `OneOrMore → +`, `Optional → ?`,
      `Repeat → {m,n}`, `And → &`, `Not → !`, `CharRange → 'a'..'z'`,
      `AnyChar → ANY`, `NonTerminal → ident` are emitted natively.
- [ ] `RuleKind` maps to `@`/`_`/`$` modifiers (Normal has none) exactly as tabled.
- [ ] Unordered `Choice` and labelled `Capture` are emitted with a recorded
      `EmitReport.lossy` note (PEG's documented CFG-fidelity caveats); no silent loss.
- [ ] The production line is produced via `TranslationTemplate`, not ad-hoc `format!`.
- [ ] The emitted text is valid `.pest` — verified by re-importing through B4 once
      it lands (smoke test; full matrix is [`F2`](./F2-grammar-format-fidelity-matrix.md)).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery `warn`, `Cargo.toml:103-106`), `cargo test --all-features` pass;
      `rust-script scripts/check-no-src-tests.rs` passes (tests under `tests/`).

## Tests

- `tests/unit/` (`grammar_emit` module):
  - Hand-built `Grammar` covering every variant → assert output equals committed
    golden `.pest` fixture; spot-check `&`/`!`/`{m,n}`/`'a'..'z'`/`ANY` rendering.
  - Each `RuleKind` → assert the right `@`/`_`/`$`/none prefix.
  - Unordered `Choice` and labelled `Capture` → assert an `EmitReport.lossy` entry.
- Round-trip smoke (gated on B4; `#[ignore]` with a comment pointing at
  [`F2`](./F2-grammar-format-fidelity-matrix.md) until B4 exists):
  `import_pest(emit_pest(g))` is structurally equal to `g` for the fixtures,
  modulo the documented unordered-`Choice` loss.
- Pure in-process; only committed fixtures are read.

## References

- pest / `pest_meta` PEG algebra and `.pest` syntax:
  [`library-survey.md`](../library-survey.md) §A.1, §A.2.
- `peg`/`winnow` optional variants (deferred to C4):
  [`library-survey.md`](../library-survey.md) §A.4, §A.8.
- Pairing importer: B4 (PEG via `pest_meta`); [`solution-plans.md`](../solution-plans.md) §Epic B.
- IR being emitted: [`A1`](./A1-grammar-ir.md); shared emit scaffolding:
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md).
- `TranslationTemplate` engine: `src/translation_rules.rs:300-366`,
  [`existing-capabilities.md`](../existing-capabilities.md) §1.
- Round-trip fidelity matrix: [`F2`](./F2-grammar-format-fidelity-matrix.md);
  [`solution-plans.md`](../solution-plans.md) §Epic C, §3 table (row C2), §4 DAG.
