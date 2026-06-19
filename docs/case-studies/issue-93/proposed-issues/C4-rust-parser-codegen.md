# C4 — Rust parser codegen (IR → runnable Rust parser)

> **Epic:** C — Emitters & codegen · **Blocked by:** A1 · **Blocks:** E5
> **Requirements:** P-7, P-9 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §A.

## Context

Requirement P-9 ("translate to Rust, JavaScript, and other languages") and P-7
("a working implementation in Rust") together demand that a grammar held in the
[`A1`](./A1-grammar-ir.md) IR can be turned into **runnable Rust parser source**.
The existing `rust_codec` (`src/rust_codec.rs`, re-exported `src/lib.rs:83-86`)
already models Rust *data* types as links (`RustTypeShape`/`RustFieldShape`/
`RustTypeKind`) but **emits no parser and no source text** — `RustTypeShape`
describes a type's shape for the links codec, never a function body
(`src/rust_codec.rs:24-203`). This issue adds the missing parser-emission half:
`Grammar` → a `.pest` grammar file **plus** matching Rust AST `struct`/`enum`
declarations, so an inferred grammar becomes a compilable parser. For an inferred
grammar this is the concrete artefact that satisfies **P-7's "working Rust
implementation"** — `pest_derive` compiles the emitted `.pest` to a real parser.

## Goal

Emit, from any A1 `Grammar`, (a) a `pest`-compatible `.pest` grammar string and
(b) Rust source for the AST data types (reusing `RustTypeShape`), such that the
pair compiles under `pest`/`pest_derive` and parses sample inputs the grammar
accepts. Keep emission deterministic and free of inference (pure IR → text).

## Scope

**In scope**
- A new module `src/grammar/emit/rust.rs` (under the `src/grammar/emit/` tree
  introduced by [`C1`](./C1-bnf-ebnf-abnf-emitters.md); if C1 has not landed,
  create `src/grammar/emit/mod.rs` here and note it for C1 to reuse).
- A public `emit_pest(grammar: &Grammar) -> String` lowering each `GrammarExpr`
  to `.pest` syntax.
- A public `emit_rust_parser(grammar: &Grammar) -> RustParserArtifacts` bundling
  the `.pest` text, the `#[derive(Parser)]` struct stub, and the AST type
  declarations.
- Reuse of `RustTypeShape`/`RustFieldShape` to derive **one AST type per rule**
  and render them to Rust `struct`/`enum` source.

**Out of scope** (owned elsewhere)
- The IR itself, its builder, and links round-trip → [`A1`](./A1-grammar-ir.md).
- Plain `.pest` *grammar-format* emission as an interchange artefact (without the
  Rust derive/AST scaffolding) → [`C2`](./C2-peg-emitter.md); C4 may call C2's
  `emit_pest` if it exists, else define `emit_pest` here and let C2 reuse it.
- JavaScript codegen → [`C5`](./C5-javascript-parser-codegen.md).
- Wrapping the emitted parser as a runtime `LanguageParser` in `ParserRegistry`
  → **E2**.
- The end-to-end "examples → infer → emit Rust → re-parse" pipeline → [`E5`](./E5-end-to-end-integration-examples.md).

## Design / specification

### Target choice — `pest` (`.pest` + `pest_derive`)

`pest` is the recommended target ([`library-survey.md`](../library-survey.md) §A.1,
§A.0 table, Final-table row "pest"): **MIT OR Apache-2.0**, PEG semantics that map
**1-to-1** onto A1's algebra (A1 itself is modelled on `pest_meta::ast::Expr` —
see [`A1`](./A1-grammar-ir.md) "Design"), and codegen is **build-time via the
`pest_derive` proc-macro**, so the emitter only has to produce a **string** and a
small derive stub — no hand-written parser-combinator plumbing. This is the
lowest-effort path from A1 to a *working* Rust parser, and because A1 already
preserves PEG ordered choice, predicates, and repetition bounds, emission is a
near-mechanical pretty-print. (`winnow`/`peg` were considered — see Reuse — but
each requires emitting Rust *function bodies* rather than a grammar string, more
generated code to get right; recorded as the fallback target.)

### `GrammarExpr` → `.pest`

| `GrammarExpr` | `.pest` fragment |
|---|---|
| `Empty` | `""` |
| `Terminal("fn")` | `"fn"` (escaped) |
| `TerminalInsensitive("fn")` | `^"fn"` |
| `CharRange('a','z')` | `'a'..'z'` |
| `CharClass { negated:false, items }` | `("a" \| 'd'..'f' \| …)` (silent group) |
| `CharClass { negated:true, items }` | `(!(…) ~ ANY)` |
| `AnyChar` | `ANY` |
| `NonTerminal("expr")` | `expr` |
| `Choice { ordered:true, alts }` | `a \| b \| c` (PEG ordered choice) |
| `Choice { ordered:false, alts }` | `a \| b \| c` + emit a `// NOTE: unordered in source` doc, since `.pest` is ordered-only (record lossy mapping for F2) |
| `Sequence([a,b,c])` | `a ~ b ~ c` |
| `Optional(e)` | `e?` |
| `ZeroOrMore(e)` | `e*` |
| `OneOrMore(e)` | `e+` |
| `Repeat { e, min, max: Some(n) }` | `e{min,n}` |
| `Repeat { e, min, max: None }` | `e{min,}` |
| `And(e)` | `&e` |
| `Not(e)` | `!e` |
| `Capture { label: Some(l), e }` | `e` + record `l` for the AST field name (see below) |

Rule modifiers map from A1 `RuleKind` to pest rule prefixes: `Normal` → `rule =
{ … }`, `Atomic` → `rule = @{ … }`, `Silent` → `rule = _{ … }`, `Token` →
`rule = ${ … }` (compound-atomic). Wrap each sub-expression in parentheses when
its binding power is lower than its parent (sequence binds tighter than choice)
so the emitted `.pest` reparses to the same tree.

### AST types via `RustTypeShape`

For each `GrammarRule`, build a `RustTypeShape` (`src/rust_codec.rs:107`,`:120`):
- a `Sequence`/`Capture`-shaped rule → `RustTypeShape::structure(rule_name,
  fields)` where each labelled `Capture` becomes a `RustFieldShape::new(label,
  field_type)` (`src/rust_codec.rs:66`);
- a top-level `Choice` → `RustTypeShape::enumeration(rule_name, variants)`;
- terminal-only rules → a newtype `struct Foo(String)`.

A small `render_rust_type(shape: &RustTypeShape) -> String` pretty-prints the
shape to a `#[derive(Debug, Clone)]` Rust declaration. The shapes are also valid
inputs to the existing links codec (`LinksEncoder::register_type_shape`,
`src/rust_codec.rs:399`), so the emitted AST is itself round-trippable through the
network — a free consistency check.

### Emit fn signatures

```rust
/// Bundled output of Rust parser codegen.
pub struct RustParserArtifacts {
    /// `.pest` grammar source (PEG).
    pub pest_grammar: String,
    /// `#[derive(Parser)] #[grammar_inline = "…"] struct <Name>Parser;` stub.
    pub parser_struct: String,
    /// One `#[derive(Debug, Clone)]` AST type per rule, concatenated.
    pub ast_types: String,
}

/// Emits a `.pest` PEG grammar string from the A1 IR (no Rust scaffolding).
#[must_use]
pub fn emit_pest(grammar: &Grammar) -> String;

/// Emits the full runnable-parser bundle (`.pest` + derive stub + AST types).
#[must_use]
pub fn emit_rust_parser(grammar: &Grammar) -> RustParserArtifacts;
```

### Generated-code skeleton (tiny grammar)

For the A1 grammar `sum = num ("+" num)* ;  num = '0'..'9'+ ;`
(`Sequence([NonTerminal("num"), ZeroOrMore(Sequence([Terminal("+"),
NonTerminal("num")]))])`), `emit_rust_parser` yields:

```rust
// --- pest_grammar ---
// sum = { num ~ ("+" ~ num)* }
// num = @{ '0'..'9'+ }

// --- parser_struct ---
#[derive(pest_derive::Parser)]
#[grammar_inline = "sum = { num ~ (\"+\" ~ num)* }\nnum = @{ '0'..'9'+ }\n"]
pub struct SumParser;

// --- ast_types ---
#[derive(Debug, Clone)]
pub struct Sum { pub num: Vec<Num> }
#[derive(Debug, Clone)]
pub struct Num(pub String);
```

`SumParser::parse(Rule::sum, "1+2+3")` then succeeds — the working-Rust evidence
for P-7.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | Add `pest = "2"` and `pest_derive = "2"` (pin latest `2.8.x`; **MIT OR Apache-2.0**, [`library-survey.md`](../library-survey.md) §A.1). Gate behind a `grammar-codegen` feature if optional; document it. |
| `src/grammar/emit/mod.rs` | New (if not created by [`C1`](./C1-bnf-ebnf-abnf-emitters.md)). Re-export per-target emit fns. |
| `src/grammar/emit/rust.rs` | New. `emit_pest`, `emit_rust_parser`, `RustParserArtifacts`, `render_rust_type`, the `GrammarExpr`→pest walk, and the rule→`RustTypeShape` mapping. |
| `src/lib.rs` | `pub use grammar::emit::{emit_pest, emit_rust_parser, RustParserArtifacts};` next to the `rust_codec` re-exports (`src/lib.rs:83-86`). |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_emit_rust` tests. |
| `tests/fixtures/grammar/rust/` | Expected `.pest`/AST snapshots for the fixture grammars. |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

## Reuse

- `rust_codec::{RustTypeShape, RustFieldShape, RustTypeKind}` for the AST types
  and `LinksEncoder::register_type_shape` for the consistency round-trip
  (`src/rust_codec.rs:88-203`,`:399`, re-exported `src/lib.rs:83-86`). The new
  `render_rust_type` is the **only** new pretty-printer; the shape vocabulary is
  reused unchanged.
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/`RuleKind`/accessors
  (`rules()`, `start_rule()`).
- `pest` + `pest_derive` (build-time PEG codegen) — [`library-survey.md`](../library-survey.md) §A.1–A.2.
- `.pest` lowering shared with [`C2`](./C2-peg-emitter.md) (PEG emitter): define
  `emit_pest` once, reuse from both.
- Fallback target `winnow`/`peg` (emit Rust combinator/`peg::parser!{}` source) —
  [`library-survey.md`](../library-survey.md) §A.4, §A.8 — recorded, not built.

## Acceptance criteria

- [ ] `emit_pest` lowers every `GrammarExpr` variant per the table; the result is
      accepted by `pest_meta::parse_and_optimize` (validate in a test, no panic).
- [ ] `emit_rust_parser` returns a `RustParserArtifacts` whose `pest_grammar`,
      `parser_struct`, and `ast_types` compile together (see Tests for how this is
      validated CI-light) and whose AST types are derived from `RustTypeShape`.
- [ ] Lossy mappings (unordered `Choice` → ordered `.pest`) are recorded as a
      doc-comment in the output and noted for the F2 fidelity matrix; emission
      never panics on any A1 grammar.
- [ ] Rule modifiers map `RuleKind` → pest prefixes (`@`,`_`,`$`) as documented.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:103-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live
      under `tests/`, not `src/`); new dependency licences recorded in the PR.

## Tests

- Unit (`tests/unit/`, new `grammar_emit_rust` module): build each fixture
  `Grammar` via the A1 builder, call `emit_pest`, assert the emitted string
  matches a committed snapshot **and** that `pest_meta::parse_and_optimize`
  accepts it (proves the grammar is well-formed without a build step).
- Unit: `emit_rust_parser` for the `sum`/`num` skeleton above → assert the AST
  `RustTypeShape`s (`Sum` struct with a `Vec<Num>` field, `Num` newtype) and that
  rendered source contains the expected `#[derive(...)]` lines.
- Integration (CI-light compile + parse check, no extra toolchain): a fixture
  under `tests/fixtures/grammar/rust/` provides the *expected* generated
  `src`; commit a tiny crate-shaped fixture whose `lib.rs` pastes a known-good
  emitted bundle and a `#[test]` calling `Parser::parse` on sample inputs, run as
  part of `cargo test --all-features`. (Generating-then-`cargo build`-ing inside
  a test is out of scope to keep CI light; the committed fixture proves the shape
  compiles and parses, and the snapshot test proves the emitter reproduces it.)
- Integration: emit AST `RustTypeShape`s, round-trip them through
  `LinksEncoder`/`LinksDecoder` (`src/rust_codec.rs:399`,`:498`) to confirm the
  emitted types are consistent with the existing codec.

## References

- `pest` / `pest_derive`: <https://docs.rs/pest> · <https://docs.rs/pest_derive> ·
  `pest_meta::ast::Expr`: <https://docs.rs/pest_meta>
- [`library-survey.md`](../library-survey.md) §A.1, §A.2, §A.4, §A.8, Final table;
  [`existing-capabilities.md`](../existing-capabilities.md) §1 (rust_codec row),
  [`solution-plans.md`](../solution-plans.md) §Epic C.
- [`A1`](./A1-grammar-ir.md), [`C2`](./C2-peg-emitter.md),
  [`C5`](./C5-javascript-parser-codegen.md), [`E5`](./E5-end-to-end-integration-examples.md).
- Existing codec: `src/rust_codec.rs:1-203`, re-exports `src/lib.rs:83-86`.
