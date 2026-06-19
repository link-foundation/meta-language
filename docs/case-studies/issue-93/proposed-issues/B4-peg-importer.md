# B4 — PEG (`.pest`) importer

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with C1)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B,
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 names **PEG** first ("we also should include PEG, BNF and other
languages, to be parsed *as* meta-language"). [`B1`](./B1-bnf-importer.md)
established the importer pattern (the shared `GrammarImportError` and
`import_<fmt>` fns in `src/grammar/import/mod.rs`); [`B2`](./B2-ebnf-importer.md)
and [`B3`](./B3-abnf-importer.md) added EBNF and ABNF. This issue adds the **PEG**
importer for `.pest` grammars. It is the **easiest** importer to make faithful:
[`A1`](./A1-grammar-ir.md)'s algebra was *modelled on* `pest_meta::ast::Expr`
(the proven 18-variant PEG algebra — see [`A1`](./A1-grammar-ir.md) §Design and
[`library-survey.md`](../library-survey.md) §A.2), so the lowering is nearly a
variant-for-variant rename. It is also the **reference importer for ordered choice
and the `&`/`!` predicates** — constructs BNF/EBNF/ABNF do not have.

## Goal

Parse `.pest` grammar text into a `Grammar` (A1 IR) by reusing `pest_meta` to
produce its `ast::Rule`/`ast::Expr`, then lowering that AST to A1 — losslessly
enough to re-emit equivalent `.pest` (round-trip with C2), and register it so
`import-grammar --format pest` / `--format peg` (E1) and the `ParserRegistry`
path can reach it. Reuse [`B1`](./B1-bnf-importer.md)'s `GrammarImportError` and
module layout; add only the PEG-specific lowering.

## Scope

**In scope**
- A `pest_meta::ast` → `Grammar` lowering using `pest_meta` (MIT OR Apache-2.0 —
  [`library-survey.md`](../library-survey.md) §A.2).
- A public `import_pest(text: &str) -> Result<Grammar, GrammarImportError>`.
- Mapping all 18 `ast::Expr` variants and all `ast::RuleType` modifiers.
- `source_format = Some(GrammarFormat::Peg)` on the produced grammar.

**Out of scope**
- BNF/EBNF/ABNF → B1–B3; tree-sitter/ANTLR/Lark/GBNF → B5–B7.
- Emitting `.pest` → **C2**; the text round-trip *test* lives with **F2** once C2
  exists. (Note: the table below pairs with C2, not C1.)
- The shared `GrammarImportError` is **owned by [`B1`](./B1-bnf-importer.md)** —
  reuse it, do not redefine it.
- pest's built-in tokens (`ASCII_DIGIT`, `EOI`, `SOI`, …) lower as plain
  `NonTerminal`s; the stack-machine-only variants (`Skip`, `Push`, `PeekSlice`)
  lower to `Unsupported` (see the table) rather than being faked.

## Design / specification

A `.pest` grammar is a list of rules `name = modifier? { expression }`, where the
expression algebra is PEG (ordered choice `|`, sequence `~`, repetition
`* + ? {n} {n,} {,m} {n,m}`, predicates `&` `!`, grouping `()`). `pest_meta`
parses this into `Vec<ast::Rule>` where `Rule { name: String, ty: RuleType, expr:
Expr }`. The lowering is a direct structural map of the 18 `Expr` variants:

| `pest_meta::ast::Expr` | meaning | `GrammarExpr` |
|---|---|---|
| `Str(s)` | exact string literal | `Terminal(s)` |
| `Insens(s)` | case-insensitive literal `^"…"` | `TerminalInsensitive(s)` |
| `Range(a, b)` | char range `'a'..'z'` | `CharRange(a, b)` |
| `Ident(name)` | rule / token reference | `NonTerminal(name)` |
| `PosPred(e)` | positive lookahead `&e` | `And(box e)` |
| `NegPred(e)` | negative lookahead `!e` | `Not(box e)` |
| `Seq(a, b)` | sequence `a ~ b` (binary) | `Sequence([a, b])` (flatten nested `Seq`) |
| `Choice(a, b)` | ordered choice `a \| b` (binary) | `Choice { ordered: true, alternatives: [a, b] }` (flatten nested `Choice`) |
| `Opt(e)` | optional `e?` | `Optional(box e)` |
| `Rep(e)` | zero-or-more `e*` | `ZeroOrMore(box e)` |
| `RepOnce(e)` | one-or-more `e+` | `OneOrMore(box e)` |
| `RepExact(e, n)` | exactly n `e{n}` | `Repeat { expr, min: n, max: Some(n) }` |
| `RepMin(e, n)` | at least n `e{n,}` | `Repeat { expr, min: n, max: None }` |
| `RepMax(e, m)` | at most m `e{,m}` | `Repeat { expr, min: 0, max: Some(m) }` |
| `RepMinMax(e, n, m)` | n..m `e{n,m}` | `Repeat { expr, min: n, max: Some(m) }` |
| `PeekSlice(start, end)` | stack-slice peek `PEEK[..]` | `GrammarImportError::Unsupported { construct: "PeekSlice" }` |
| `Skip(strings)` | skip-until (atomic helper) | `GrammarImportError::Unsupported { construct: "Skip" }` |
| `Push(e)` | push match onto stack `PUSH(e)` | `GrammarImportError::Unsupported { construct: "Push" }` |

The wildcard `.` (any char) appears in `.pest` as the built-in token `ANY`
(an `Ident("ANY")`); lower `Ident("ANY")` to `AnyChar`. pest has **no labelled
capture** in the A1 `Capture` sense (captures are stack ops), so A1's `Capture`
is not produced by this importer.

**Rule modifiers → `RuleKind`** (`pest_meta::ast::RuleType`):

| `ast::RuleType` | `.pest` syntax | A1 `RuleKind` |
|---|---|---|
| `Normal` | `rule = { … }` | `RuleKind::Normal` |
| `Atomic` | `rule = @{ … }` | `RuleKind::Atomic` |
| `CompoundAtomic` | `rule = ${ … }` | `RuleKind::Atomic` (compound-atomic collapses to Atomic in A1; note the loss for C2) |
| `NonAtomic` | `rule = !{ … }` | `RuleKind::Normal` (non-atomic ≈ normal in A1) |
| `Silent` | `rule = _{ … }` | `RuleKind::Silent` |

A1's `RuleKind` is `{ Normal, Atomic, Silent, Token }` (see [`A1`](./A1-grammar-ir.md)
§Design). pest has no exact `Token` analogue and A1 has no exact
`CompoundAtomic`/`NonAtomic`; the table above documents the lossy collapses (C2
should preserve what it can — record the loss in the F2 fidelity matrix).

> **Read the exact `pest_meta` API from `docs.rs/pest_meta` before coding.** Use
> the lowest-loss entry point: prefer `pest_meta::parser::parse(Rule::grammar_rules, text)`
> + `pest_meta::parser::consume_rules(pairs)` to get the **unoptimized**
> `Vec<ast::Rule>` (so the IR mirrors the author's source). Do **not** use
> `parse_and_optimize` / `optimizer::optimize` for import — optimization rewrites
> the tree (e.g. collapses choices) and would lose round-trip fidelity with C2.
> `Expr` is binary for `Seq`/`Choice`; **flatten** nested `Seq`/`Choice` into A1's
> n-ary `Sequence`/`Choice` during the walk.

```rust
// Defined by B1 in src/grammar/import/mod.rs; reused here unchanged:
//   pub enum GrammarImportError { Parse { format, message }, Unsupported { format, construct } }
pub fn import_pest(text: &str) -> Result<Grammar, GrammarImportError>;
```

Lowering walk (`lower_expr(expr) -> Result<GrammarExpr, GrammarImportError>`):
1. Parse `text` with `pest_meta` to `Vec<ast::Rule>`; map any parse/validation
   error to `Parse { format: Peg, message }`. (`pest_meta`'s `validator` already
   reports undefined-rule and duplicate-name errors — surface them verbatim.)
2. For each `ast::Rule { name, ty, expr }`: `lower_expr(expr)`, map `ty` via the
   `RuleType` table, push `GrammarRule { name, expr, kind, .. }`.
3. `lower_expr` matches the 18-variant table; for `Seq`/`Choice`, recurse into both
   operands and **flatten** same-variant children into one n-ary node.
   `Ident("ANY")` → `AnyChar`, other `Ident` → `NonTerminal`;
   `PeekSlice`/`Skip`/`Push` → `Err(Unsupported)`.
4. The start symbol is the first rule (pest has no declared start). After building,
   call A1 `referenced_nonterminals()` to report any residual undefined references.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | Add `pest_meta = "2.8"` (pin the latest 2.8.x; confirm **MIT OR Apache-2.0**). Reuse the `grammar-import` feature gate if [`B1`](./B1-bnf-importer.md) introduced one. (Note: `pest_meta` may already be a dev/reference dep per [`A1`](./A1-grammar-ir.md)'s algebra note — promote it to a normal dependency here.) |
| `src/grammar/import/mod.rs` | Add `mod pest; pub use pest::import_pest;` (reuse the existing `GrammarImportError`). |
| `src/grammar/import/pest.rs` | New. `import_pest`, the `lower_expr` walk, the `RuleType`→`RuleKind` map, and `Seq`/`Choice` flattening. |
| `src/lib.rs` | Extend the import re-export: `pub use grammar::import::import_pest;`. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register the new tests. |
| `tests/fixtures/grammar/peg/` | A few `.pest` inputs (see Tests). |
| `changelog.d/` | Fragment. |

## Reuse

- `pest_meta` (MIT OR Apache-2.0) — parsing/validation. See
  [`library-survey.md`](../library-survey.md) §A.2 ("the standout … strongest
  introspectable, programmatically-targetable grammar IR") and the PART 2 table.
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/`RuleKind`/builder — the
  algebra was modelled on `ast::Expr`, so most variants map 1:1;
  `referenced_nonterminals()` for residual undefined-reference checks.
- [`B1`](./B1-bnf-importer.md) `GrammarImportError` and `src/grammar/import/` layout.
- Pattern parallel: existing `LanguageParser` adapters dispatch in
  `src/language_parser.rs:7-47`; the existing tree-sitter adapter
  (`src/tree_sitter_adapter.rs:136-207`) is the precedent for a format adapter.
  Importers may later register through `ParserRegistry`
  (`src/parser_registry.rs:50-159`) — coordinate with **E2**.

## Acceptance criteria

- [ ] `import_pest` parses each fixture into a `Grammar` with the expected rules,
      start symbol (first rule), and `source_format = Peg`.
- [ ] All 18 `ast::Expr` variants map per the table; `Seq`/`Choice` are flattened
      to n-ary A1 nodes; choice is `ordered: true` (PEG ordered choice).
- [ ] `Ident("ANY")` → `AnyChar`; `&e`/`!e` → `And`/`Not`; `e{n,m}` family →
      `Repeat`/`ZeroOrMore`/`OneOrMore`/`Optional` as documented.
- [ ] All five `RuleType` modifiers map per the `RuleKind` table (incl. the
      documented `CompoundAtomic`→`Atomic` and `NonAtomic`→`Normal` collapses).
- [ ] `PeekSlice`/`Skip`/`Push` yield `GrammarImportError::Unsupported`, never a
      panic; malformed `.pest` yields `GrammarImportError::Parse` (surfacing
      `pest_meta`'s validator message), never a panic.
- [ ] Undefined-rule references are reported (via `pest_meta`'s validator and/or
      A1 `referenced_nonterminals()`).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:105-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes; the
      `pest_meta` licence is recorded in the PR description.

## Tests

- Unit (`tests/unit/`, new `grammar_import_pest` module): each fixture → assert
  rule count, names, `RuleKind` per rule, and a spot-checked expression tree
  (e.g. that `expr = { term ~ ("+" ~ term)* }` lowers to a `Sequence` whose tail
  is `ZeroOrMore(Sequence([Terminal("+"), NonTerminal("term")]))`).
- Unit: a variant-coverage fixture exercising **every** mapped `Expr` variant and
  every `RuleType`; assert the lowered tree.
- Unit: a `.pest` using `PUSH`/`PEEK[..]` → `Err(Unsupported)`; a syntactically
  invalid `.pest` → `Err(Parse)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`, per [`A1`](./A1-grammar-ir.md)).
- Fixtures under `tests/fixtures/grammar/peg/`:
  - `arithmetic.pest` — precedence-climbing arithmetic (`~`, ordered `|`, `*`,
    grouping) — the recursive, ordered-choice workhorse.
  - `json.pest` — a small JSON grammar exercising `@{…}` atomic rules, `_{…}`
    silent whitespace, `^"…"` case-insensitive literals, char ranges, and `ANY`.
  - `predicates.pest` — rules using `&` / `!` lookahead and the `{n,m}` repetition
    family for variant coverage.
- (Deferred to **F2** once C2 lands: `import_pest` ∘ `emit_pest` text round-trip.)

## References

- `pest_meta` `ast::{Rule, Expr, RuleType}`: <https://docs.rs/pest_meta>
  (`enum.Expr`: <https://docs.rs/pest_meta/latest/pest_meta/ast/enum.Expr.html>) ·
  PEG (Ford, POPL '04): <https://bford.info/pub/lang/peg/> · pest book:
  <https://pest.rs/book/>
- [`library-survey.md`](../library-survey.md) §A.2 (`pest_meta` / 18-variant `Expr`),
  §B (PART 2 `pest_meta` row), [`solution-plans.md`](../solution-plans.md) §Epic B,
  [`A1`](./A1-grammar-ir.md) (algebra modelled on `ast::Expr`), [`B1`](./B1-bnf-importer.md).
