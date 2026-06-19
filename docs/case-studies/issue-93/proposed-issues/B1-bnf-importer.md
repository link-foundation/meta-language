# B1 — BNF importer (parse BNF *as* meta-language)

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with C1)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B,
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") means every grammar notation must be importable into
the A1 IR so it becomes a first-class grammar in the links network. This issue
delivers the **BNF** importer — the simplest classical notation — and establishes
the importer pattern that B2–B7 follow.

## Goal

Parse classic BNF text into a `Grammar` (A1 IR), losslessly enough to re-emit
equivalent BNF (round-trip with C1), and register it so `import-grammar --format bnf`
(E1) and the `ParserRegistry` path can reach it.

## Scope

**In scope**
- A `bnf` → `Grammar` lowering using the `bnf` crate (MIT).
- A public `import_bnf(text: &str) -> Result<Grammar, GrammarImportError>`.
- Mapping every BNF construct to the right `GrammarExpr` variant.
- `source_format = Some(GrammarFormat::Bnf)` on the produced grammar.

**Out of scope**
- EBNF/ABNF/PEG/ANTLR/Lark/GBNF/tree-sitter → **B2–B7** (same pattern).
- Emitting BNF → **C1**. Round-trip *test* lives with F2 once C1 exists.
- A shared importer trait abstraction — define a minimal `GrammarImportError`
  here; if a shared trait emerges, refactor in B2 (note it, don't block).

## Design / specification

BNF grammar: a list of productions `<symbol> ::= expansion`, where an expansion is
`|`-separated alternatives, each a sequence of terminals (quoted) and
non-terminals (`<angle>`). Lowering:

| BNF construct | `GrammarExpr` |
|---|---|
| `<name>` (LHS) | `GrammarRule { name, .. }` |
| `<name>` (RHS) | `NonTerminal(name)` |
| `"lit"` / `'lit'` | `Terminal(lit)` |
| ` a b c ` (juxtaposition) | `Sequence([a,b,c])` |
| `a \| b` | `Choice { ordered: false, alternatives: [a,b] }` (BNF alternation is unordered) |
| empty alternative | `Empty` |

Use the `bnf` crate to parse (`Grammar::from_str`), then walk its
`Production`/`Expression`/`Term` types and build the A1 IR. Surface parse failures
as `GrammarImportError::Parse { message, .. }`.

```rust
#[derive(Debug)]
pub enum GrammarImportError {
    Parse { format: GrammarFormat, message: String },
    Unsupported { format: GrammarFormat, construct: String },
}
pub fn import_bnf(text: &str) -> Result<Grammar, GrammarImportError>;
```

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | Add `bnf = "..."` (pin latest; confirm MIT). If it must be optional, gate behind a `grammar-import` feature and document it. |
| `src/grammar/import/mod.rs` | New. `GrammarImportError`; re-export per-format fns. |
| `src/grammar/import/bnf.rs` | New. `import_bnf` + the lowering walk. |
| `src/lib.rs` | `pub use grammar::import::{import_bnf, GrammarImportError};` |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register tests. |
| `tests/fixtures/grammar/bnf/` | A few `.bnf` inputs (small arithmetic grammar, a recursive list grammar, postal-address BNF). |
| `changelog.d/` | Fragment. |

## Reuse

- `bnf` crate (MIT) — parsing. See [`library-survey.md`](../library-survey.md) §B.1.
- A1 `Grammar`/`GrammarExpr`/builder for construction.
- Pattern parallel: the existing `LanguageParser` adapters dispatch in
  `src/language_parser.rs:7-47`; importers may later register through
  `ParserRegistry` (`src/parser_registry.rs:50-159`) — coordinate with E2, do not
  duplicate.

## Acceptance criteria

- [ ] `import_bnf` parses each fixture into a `Grammar` with the expected rules,
      start symbol (first production unless specified), and `source_format = Bnf`.
- [ ] Every BNF construct in the table maps to the documented `GrammarExpr`.
- [ ] Malformed BNF yields `GrammarImportError::Parse`, never a panic.
- [ ] Non-terminal references resolve; a reference to an undefined symbol is
      reported (reuse A1 `referenced_nonterminals()`).
- [ ] `cargo fmt --check`, `clippy --all-targets --all-features`, `test --all-features`
      pass; new dependency's licence recorded in the PR description.

## Tests

- Unit: each fixture → assert rule count, names, and a spot-checked expression tree.
- Unit: malformed input → `Err(Parse)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`).
- (Deferred to F2 once C1 lands: `import_bnf` ∘ `emit_bnf` text round-trip.)

## References

- `bnf` crate: <https://docs.rs/bnf> · BNF (RFC-style): <https://en.wikipedia.org/wiki/Backus–Naur_form>
- [`library-survey.md`](../library-survey.md) §B.1, [`solution-plans.md`](../solution-plans.md) §Epic B.
</content>
