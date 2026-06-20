# B2 — EBNF importer (parse EBNF *as* meta-language)

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with C1)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B,
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") means every grammar notation must be importable into
the [`A1`](./A1-grammar-ir.md) IR so it becomes a first-class grammar in the links
network. [`B1`](./B1-bnf-importer.md) shipped the **BNF** importer and established
the importer pattern (`import_<fmt>` fns + the shared `GrammarImportError` in
`src/grammar/import/mod.rs`). This issue adds the **EBNF** importer — ISO/IEC
14977 (Wikipedia/EBNF-Evaluator dialect) — the next-simplest notation, which adds
the `{ }` / `[ ]` / `( )` repetition-and-grouping operators BNF lacks.

## Goal

Parse EBNF text into a `Grammar` (A1 IR), losslessly enough to re-emit equivalent
EBNF (round-trip with C1), and register it so `import-grammar --format ebnf` (E1)
and the `ParserRegistry` path can reach it. Reuse [`B1`](./B1-bnf-importer.md)'s
`GrammarImportError` and module layout; add only the EBNF-specific lowering.

## Scope

**In scope**
- An `ebnf` → `Grammar` lowering using the `ebnf` crate (MIT — see
  [`library-survey.md`](../library-survey.md) §B, PART 2 table).
- A public `import_ebnf(text: &str) -> Result<Grammar, GrammarImportError>`.
- Mapping every EBNF construct to the right `GrammarExpr` variant.
- `source_format = Some(GrammarFormat::Ebnf)` on the produced grammar.

**Out of scope**
- BNF → [`B1`](./B1-bnf-importer.md); ABNF/PEG/tree-sitter/ANTLR/Lark/GBNF →
  B3–B7 (same pattern).
- Emitting EBNF → **C1**. The text round-trip *test* lives with **F2** once C1 exists.
- The shared `GrammarImportError` is **owned by [`B1`](./B1-bnf-importer.md)** —
  reuse it, do not redefine it. If a shared importer trait emerges, note it; don't block.
- The W3C-EBNF (XML 1.0) dialect and Lark's EBNF derivative — distinct notations,
  not covered here (Lark → **B7**).

## Design / specification

EBNF grammar: a list of rules `name = expression ;`, where an expression is
`|`-separated alternatives, each a `,`-separated concatenation of terms; terms may
be quoted terminals, rule references, or one of the bracket operators. Lowering
(`ebnf` crate `Node` enum → A1 `GrammarExpr`):

| EBNF construct | `ebnf` crate node | `GrammarExpr` |
|---|---|---|
| `name = …` (rule head) | `Expression { lhs, rhs }` | `GrammarRule { name: lhs, .. }` |
| identifier on RHS | `Node::Terminal(name)` *(rule ref)* / `RegexString` | `NonTerminal(name)` |
| `"lit"` / `'lit'` | `Node::String(s)` | `Terminal(s)` |
| `a , b , c` (concatenation) | `Node::Multiple(vec)` | `Sequence([a,b,c])` |
| `a \| b` (alternation, **unordered**) | `Node::RegexExt`/alternation node | `Choice { ordered: false, alternatives: [a,b] }` |
| `{ a }` (zero-or-more repetition) | `Node::Repeat(box)` | `ZeroOrMore(a)` |
| `[ a ]` (optional) | `Node::Optional(box)` | `Optional(a)` |
| `( a )` (grouping) | `Node::Group(box)` | the inner expr (grouping is structural only) |
| empty alternative | — | `Empty` |
| `? prose ?` (special sequence) | `Node::RegexString`/unsupported | `GrammarImportError::Unsupported { construct }` |

Walk the `ebnf` crate's parse output (`ebnf::get_grammar(text)` → `Grammar` of
`Expression { lhs, rhs: Node }`) and recurse over each `Node`, building the A1 IR
via the [`A1`](./A1-grammar-ir.md) builder. The first rule is the start symbol.
Surface parse failures as `GrammarImportError::Parse`.

> **Note on the `ebnf` crate's `Node` variant names.** [`library-survey.md`](../library-survey.md)
> §B lists `String/Regex/Terminal/Multiple/Group/Optional/Repeat/…`; treat that as
> indicative. **Read the exact public variants from `docs.rs/ebnf` and match on the
> real enum** — do not invent variants. If the crate's parser proves too narrow
> (e.g. it rejects `;` terminators or the `( )` grouping), fall back to a
> **clean-room recursive-descent EBNF parser** in `src/grammar/import/ebnf.rs`
> (tokenize on `= ; | , { } [ ] ( ) " '`, then a precedence-climbing parse:
> alternation > concatenation > term), lowering directly into the same table —
> and record that decision (no new dependency) in the PR.

```rust
// Defined by B1 in src/grammar/import/mod.rs; reused here unchanged:
//   pub enum GrammarImportError { Parse { format, message }, Unsupported { format, construct } }
pub fn import_ebnf(text: &str) -> Result<Grammar, GrammarImportError>;
```

Lowering walk (`lower_node(node) -> Result<GrammarExpr, GrammarImportError>`):
1. For each top-level `Expression { lhs, rhs }`, `lower_node(rhs)` and push a
   `GrammarRule { name: lhs, expr, kind: RuleKind::Normal, .. }`.
2. Alternation node → collect each branch via `lower_node`, wrap in
   `Choice { ordered: false, alternatives }` (EBNF alternation is set-like / unordered).
3. Concatenation / `Multiple` → `Sequence(children.map(lower_node))`.
4. `Repeat` → `ZeroOrMore(box lower_node(inner))`; `Optional` → `Optional(box …)`;
   `Group` → just `lower_node(inner)` (no IR node for parentheses).
5. String literal → `Terminal(s)`; identifier/rule reference → `NonTerminal(name)`.
6. Any node with no A1 counterpart (special-sequence prose, inline regex) →
   `Err(Unsupported { format: Ebnf, construct: "<name>" })`.
7. After building, call A1 `referenced_nonterminals()` and report references to
   undefined rules.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | Add `ebnf = "0.1"` (pin the latest `0.1.x`; confirm **MIT**). If imports must be optional, gate behind the same `grammar-import` feature [`B1`](./B1-bnf-importer.md) introduces. |
| `src/grammar/import/mod.rs` | Add `mod ebnf; pub use ebnf::import_ebnf;` (reuse the existing `GrammarImportError`; do **not** redefine it). |
| `src/grammar/import/ebnf.rs` | New. `import_ebnf` + the `lower_node` walk above. |
| `src/lib.rs` | Extend the import re-export: `pub use grammar::import::import_ebnf;` (next to `import_bnf`). |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register the new tests (mirror `grammar_parsing` registration). |
| `tests/fixtures/grammar/ebnf/` | A few `.ebnf` inputs (see Tests). |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- `ebnf` crate (MIT) — parsing. See [`library-survey.md`](../library-survey.md) §B
  (PART 2 table: `ebnf` 0.1.4, MIT, `Grammar`/`Expression{lhs,rhs}` + `Node` enum).
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/builder for construction;
  `referenced_nonterminals()` for undefined-reference checks.
- [`B1`](./B1-bnf-importer.md) `GrammarImportError` and `src/grammar/import/` layout.
- Pattern parallel: existing `LanguageParser` adapters dispatch in
  `src/language_parser.rs:7-47`; importers may later register through
  `ParserRegistry` (`src/parser_registry.rs:50-159`) — coordinate with **E2**, do
  not duplicate.

## Acceptance criteria

- [ ] `import_ebnf` parses each fixture into a `Grammar` with the expected rules,
      start symbol (first rule unless specified), and `source_format = Ebnf`.
- [ ] Every EBNF construct in the table maps to the documented `GrammarExpr`
      (`{}`→`ZeroOrMore`, `[]`→`Optional`, `()`→inner, `,`→`Sequence`,
      `|`→unordered `Choice`, quoted→`Terminal`).
- [ ] Unsupported constructs (special-sequence prose) yield
      `GrammarImportError::Unsupported`, never a panic.
- [ ] Malformed EBNF yields `GrammarImportError::Parse`, never a panic.
- [ ] Non-terminal references resolve; a reference to an undefined symbol is
      reported (reuse A1 `referenced_nonterminals()`).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:105-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live under
      `tests/`, not `src/`); the dependency choice (`ebnf` crate **or** clean-room
      parser) and its licence are recorded in the PR description.

## Tests

- Unit (`tests/unit/`, new `grammar_import_ebnf` module): each fixture → assert
  rule count, names, and a spot-checked expression tree (e.g. that `{ digit }`
  lowers to `ZeroOrMore(NonTerminal("digit"))` and `[ sign ]` to `Optional(…)`).
- Unit: a special-sequence input → `Err(Unsupported)`; a syntactically broken
  input → `Err(Parse)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`, per [`A1`](./A1-grammar-ir.md)).
- Fixtures under `tests/fixtures/grammar/ebnf/`:
  - `arithmetic.ebnf` — expression grammar exercising `( )`, `|`, `,`.
  - `number.ebnf` — `integer = digit, { digit } ;` (repetition) and
    `signed = [ "-" ], integer ;` (optional).
  - `iso14977-sample.ebnf` — the syntax example from ISO/IEC 14977 (a recursive
    list grammar) to exercise the dialect end-to-end.
- (Deferred to **F2** once C1 lands: `import_ebnf` ∘ `emit_ebnf` text round-trip.)

## References

- `ebnf` crate: <https://docs.rs/ebnf> · EBNF (ISO/IEC 14977):
  <https://www.iso.org/standard/26153.html> · <https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form>
- [`library-survey.md`](../library-survey.md) §B (PART 1 EBNF, PART 2 `ebnf` row),
  [`solution-plans.md`](../solution-plans.md) §Epic B, [`A1`](./A1-grammar-ir.md),
  [`B1`](./B1-bnf-importer.md).
