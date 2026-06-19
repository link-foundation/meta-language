# C7 — tree-sitter `grammar.js` emitter

> **Epic:** C — Emitters & codegen · **Blocked by:** A1 · **Blocks:** —
> **Requirements:** P-9 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §A.10, §B PART 1.

## Context

Requirement P-9 ("translate to … other languages") includes the tree-sitter
ecosystem, which this crate already depends on as a *front-end*: the adapter wires
~30 tree-sitter grammars (`src/tree_sitter_adapter.rs:135-212`) and each is a
golden CST oracle ([`existing-capabilities.md`](../existing-capabilities.md) §1).
But the crate can only **consume** compiled tree-sitter grammars; it cannot
**emit** one. This issue emits a tree-sitter `grammar.js` from the
[`A1`](./A1-grammar-ir.md) IR, widening ecosystem reach (Neovim/Zed/Helix/Emacs/
GitHub all consume tree-sitter grammars — [`library-survey.md`](../library-survey.md)
§A.10) and **pairing with [`B5`](./B5-tree-sitter-json-importer.md)** (the
`grammar.json` importer) so a grammar can round-trip IR → `grammar.js` → (via
`tree-sitter generate`) `grammar.json` → IR.

## Goal

Emit, from any A1 `Grammar`, a valid tree-sitter `grammar.js` — a JavaScript
module of the form `module.exports = grammar({ name, rules: { … } })` whose rule
bodies use the tree-sitter DSL (`seq`, `choice`, `repeat`, `repeat1`, `optional`,
`prec`/`prec.left`/`prec.right`, `token`, `field`, `alias`). Emission is
deterministic (pure IR → text, no inference, no `tree-sitter generate` call).

## Scope

**In scope**
- A new module `src/grammar/emit/tree_sitter.rs` (under the `src/grammar/emit/`
  tree from [`C1`](./C1-bnf-ebnf-abnf-emitters.md)/[`C4`](./C4-rust-parser-codegen.md);
  create `src/grammar/emit/mod.rs` if neither has landed and note it).
- A public `emit_tree_sitter_grammar_js(grammar: &Grammar) -> String` lowering each
  `GrammarExpr` to the DSL per the table below.
- A grammar `name` derived from the start rule / a caller hint.

**Out of scope** (owned elsewhere)
- The IR, builder, links round-trip → [`A1`](./A1-grammar-ir.md).
- The `grammar.json` *importer* (the round-trip partner) → [`B5`](./B5-tree-sitter-json-importer.md).
- Running `tree-sitter generate` or compiling to C — that is a downstream
  toolchain step; C7 only emits the JS source.
- JS *parser* codegen (Peggy) → [`C5`](./C5-javascript-parser-codegen.md); C7
  emits a *grammar definition* for tree-sitter, not a runnable parser.

## Design / specification

The tree-sitter grammar DSL ([`library-survey.md`](../library-survey.md) §B PART 1
"tree-sitter `grammar.js`") is a JS object: `grammar({ name, rules })`, where each
rule is an arrow function `$ => <body>` and bodies compose the combinators below.
A1's algebra maps onto it cleanly because both are CFG-shaped (tree-sitter is GLR,
so A1's *unordered* `Choice` is a natural fit — unlike the PEG targets in
[`C4`](./C4-rust-parser-codegen.md)/[`C5`](./C5-javascript-parser-codegen.md),
this target loses *less*).

### `GrammarExpr` → `grammar.js`

| `GrammarExpr` | `grammar.js` fragment |
|---|---|
| `Empty` | `optional(seq())` *(or `blank()`; record the chosen form)* |
| `Terminal("fn")` | `"fn"` (JSON-escaped) |
| `TerminalInsensitive("fn")` | `/[Ff][Nn]/` (tree-sitter has no native case-insensitive literal; emit a regex and note it for F2) |
| `CharRange('a','z')` | `/[a-z]/` |
| `CharClass { negated:false, items }` | `/[a-df-h…]/` |
| `CharClass { negated:true, items }` | `/[^a-df-h…]/` |
| `AnyChar` | `/./` |
| `NonTerminal("expr")` | `$.expr` |
| `Choice { ordered:_, alts }` | `choice(a, b, c)` (tree-sitter `choice` is unordered/GLR — A1 ordered & unordered both map here; record that ordering is not preserved) |
| `Sequence([a,b,c])` | `seq(a, b, c)` |
| `Optional(e)` | `optional(e)` |
| `ZeroOrMore(e)` | `repeat(e)` |
| `OneOrMore(e)` | `repeat1(e)` |
| `Repeat { e, min:0, max:None }` | `repeat(e)` |
| `Repeat { e, min:1, max:None }` | `repeat1(e)` |
| `Repeat { e, min, max }` (other) | desugar to `seq(e×min, optional(e)×(max-min))` / `seq(e×min, repeat(e))`; record the expansion |
| `And(e)` / `Not(e)` | tree-sitter has **no syntactic predicates**; emit `Unsupported` → `GrammarEmitError::Unsupported { construct: "predicate" }` (record for F2) |
| `Capture { label: Some(l), e }` | `field("l", e)` |

Rule modifiers from A1 `RuleKind`: `Token` → wrap the body in `token(...)`;
`Atomic` → `token(...)` as well (single lexical unit); `Silent` → emit the rule
but add its name to the grammar's `extras`/inline list as appropriate (or alias it
away with `alias($.rule, $.rule)` — pick one and record it); `Normal` → plain
rule. Use `prec`/`prec.left`/`prec.right` only when the IR carries precedence
(A1's algebra does not encode precedence directly; if a future field/inference
pass supplies it, thread it through — otherwise omit and rely on `choice`). The
DSL primitives `prec`, `prec.left`, `prec.right`, `token`, `field`, and `alias`
are all in scope for emission so the table is complete even where current A1
grammars do not yet exercise them.

### Fn signature

```rust
#[derive(Debug)]
pub enum GrammarEmitError {
    /// An A1 construct has no tree-sitter DSL counterpart (e.g. a predicate).
    Unsupported { construct: String },
}

/// Emits a tree-sitter `grammar.js` module from the A1 IR.
pub fn emit_tree_sitter_grammar_js(
    grammar: &Grammar,
) -> Result<String, GrammarEmitError>;
```

### Generated-code skeleton (tiny grammar)

For the A1 grammar `sum = num ("+" num)* ;  num = [0-9]+ ;`
(`num` is `RuleKind::Token`), `emit_tree_sitter_grammar_js` yields:

```javascript
module.exports = grammar({
  name: "sum",
  rules: {
    sum: $ => seq($.num, repeat(seq("+", $.num))),
    num: $ => token(repeat1(/[0-9]/)),
  },
});
```

`tree-sitter generate` then turns this into `grammar.json` + `parser.c`; feeding
that `grammar.json` back through [`B5`](./B5-tree-sitter-json-importer.md) must
reproduce a structurally equivalent A1 grammar (the round-trip test).

## File-level plan

| File | Change |
|---|---|
| `src/grammar/emit/mod.rs` | New (if not created by [`C1`](./C1-bnf-ebnf-abnf-emitters.md)/[`C4`](./C4-rust-parser-codegen.md)). Re-export per-target emit fns; define `GrammarEmitError` here for emitters to share (mirrors how [`B1`](./B1-bnf-importer.md) owns `GrammarImportError`). |
| `src/grammar/emit/tree_sitter.rs` | New. `emit_tree_sitter_grammar_js` + the `GrammarExpr`→DSL walk. |
| `src/lib.rs` | `pub use grammar::emit::{emit_tree_sitter_grammar_js, GrammarEmitError};`. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_emit_tree_sitter` tests. |
| `tests/fixtures/grammar/tree-sitter/` | Expected `grammar.js` snapshots (+ a Node smoke-test if a JS toolchain is available). |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

> **No new Rust crate dependency.** Emission is pure string building; the
> `tree-sitter` crate already present (`Cargo.toml`, `src/tree_sitter_adapter.rs`)
> is a *runtime* consumer and is not needed to emit JS text.

## Reuse

- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/`RuleKind`/accessors
  (`rules()`, `start_rule()`).
- The `GrammarExpr`-walk skeleton + snapshot discipline from
  [`C4`](./C4-rust-parser-codegen.md)/[`C5`](./C5-javascript-parser-codegen.md) —
  same recursion, tree-sitter DSL table.
- Round-trip partner [`B5`](./B5-tree-sitter-json-importer.md) (`grammar.json`
  importer) — the existing tree-sitter adapter and its ~30 wired grammars
  (`src/tree_sitter_adapter.rs:135-212`) are the reference for the DSL surface and
  a source of real `grammar.json` to diff against.
- tree-sitter DSL reference — [`library-survey.md`](../library-survey.md) §A.10,
  §B PART 1.

## Acceptance criteria

- [ ] `emit_tree_sitter_grammar_js` lowers every supported `GrammarExpr` variant
      per the table; predicates (`And`/`Not`) yield `GrammarEmitError::Unsupported`,
      never a panic.
- [ ] The output is a syntactically valid `module.exports = grammar({ name, rules })`
      module (validated per Tests).
- [ ] `RuleKind` maps to `token`/`extras`/`alias` as documented; counted
      repetition is desugared; lossy mappings (ordered→unordered choice,
      case-insensitive→regex) are recorded for the F2 fidelity matrix.
- [ ] Round-trip with [`B5`](./B5-tree-sitter-json-importer.md): for fixtures that
      avoid predicates, emit `grammar.js`, (where a tree-sitter toolchain is
      available) generate `grammar.json`, import via B5, and assert structural
      equality with the source grammar.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:103-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live
      under `tests/`, not `src/`).

## Tests

- Unit (`tests/unit/`, new `grammar_emit_tree_sitter` module): build each fixture
  `Grammar` via the A1 builder, call `emit_tree_sitter_grammar_js`, assert the
  emitted string matches a committed snapshot. (Pure Rust, always-on gate.)
- Unit: a grammar containing a predicate (`Not`/`And`) →
  `Err(Unsupported { construct: "predicate" })`.
- Unit: the `sum`/`num` skeleton → assert `seq($.num, repeat(...))` and
  `token(repeat1(/[0-9]/))` appear; assert `field(...)` for a labelled capture.
- **Lightweight JS validation (CI-light):** a `tests/fixtures/grammar/tree-sitter/`
  Node smoke-test that `require`s the emitted module and asserts
  `module.exports.name` and that `rules` is an object of functions — gated on
  `node` being present (skipped with a logged notice otherwise, keeping
  toolchain-free CI green). Full `tree-sitter generate` + B5 round-trip runs only
  where the tree-sitter CLI is available (documented in the fixture README).
- Integration (round-trip with [`B5`](./B5-tree-sitter-json-importer.md)): commit
  a known-good `grammar.json` produced from an emitted `grammar.js`, import it via
  B5, and assert structural equality with the source A1 grammar — proving the
  IR → `grammar.js` → `grammar.json` → IR loop without requiring the JS toolchain
  in CI.

## References

- tree-sitter DSL: <https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html>
  · `tree-sitter generate`: <https://tree-sitter.github.io/tree-sitter/cli/generate.html>
- [`library-survey.md`](../library-survey.md) §A.10, §B PART 1 (tree-sitter row);
  [`existing-capabilities.md`](../existing-capabilities.md) §1 (tree-sitter adapter
  row, `src/tree_sitter_adapter.rs:135-212`);
  [`solution-plans.md`](../solution-plans.md) §Epic C (C7), §Epic B (B5).
- [`A1`](./A1-grammar-ir.md), [`B5`](./B5-tree-sitter-json-importer.md),
  [`C4`](./C4-rust-parser-codegen.md), [`C5`](./C5-javascript-parser-codegen.md).
