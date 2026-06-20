# C5 — JavaScript parser codegen (IR → runnable JS parser)

> **Epic:** C — Emitters & codegen · **Blocked by:** A1 · **Blocks:** E5
> **Requirements:** P-9 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §A, §D.1.

## Context

Requirement P-9 names **JavaScript** alongside Rust as a codegen target. The
crate has **no JavaScript codegen anywhere** ([`existing-capabilities.md`](../existing-capabilities.md)
§3, "No parser codegen … no JS codegen anywhere"); `reconstruct_text_as`
(`src/reconstruction.rs:14`) targets natural-language and document formats, not
parser source. This issue emits a JavaScript parser from the
[`A1`](./A1-grammar-ir.md) IR, mirroring the Rust path in
[`C4`](./C4-rust-parser-codegen.md) so the two emitters share the same
`GrammarExpr`-walk discipline and snapshot-test pattern.

## Goal

Emit, from any A1 `Grammar`, a **Peggy** (PEG.js successor) grammar string —
i.e. a `.pegjs`/`.peggy` source that Peggy compiles to a standalone JS parser —
together with a tiny generated JS module that runs it. Keep emission
deterministic (pure IR → text, no inference, no network calls).

## Scope

**In scope**
- A new module `src/grammar/emit/javascript.rs` (under the `src/grammar/emit/`
  tree from [`C1`](./C1-bnf-ebnf-abnf-emitters.md)/[`C4`](./C4-rust-parser-codegen.md);
  create `src/grammar/emit/mod.rs` if neither has landed and note it).
- A public `emit_peggy(grammar: &Grammar) -> String` lowering each `GrammarExpr`
  to Peggy PEG syntax.
- A public `emit_javascript_parser(grammar: &Grammar) -> JsParserArtifacts`
  bundling the Peggy grammar plus a small ESM wrapper (`import peggy …; export
  const parser = peggy.generate(GRAMMAR);`).

**Out of scope** (owned elsewhere)
- The IR, builder, links round-trip → [`A1`](./A1-grammar-ir.md).
- Rust codegen → [`C4`](./C4-rust-parser-codegen.md).
- Grammar-format text emitters (BNF/EBNF/ABNF/GBNF) → [`C1`](./C1-bnf-ebnf-abnf-emitters.md),
  [`C3`](./C3-gbnf-emitter.md).
- The end-to-end "examples → infer → emit JS → re-parse" wiring → [`E5`](./E5-end-to-end-integration-examples.md).

## Design / specification

### Target choice — Peggy (PEG.js)

PEG.js / its maintained successor **Peggy** is the natural JS target: it is a PEG
generator, so A1's PEG algebra (ordered choice, predicates, repetition) maps
**1-to-1**, exactly as for `pest` in [`C4`](./C4-rust-parser-codegen.md). The
emitter only produces a **grammar string**; Peggy's `generate()` compiles it to a
parser at runtime in Node or the browser — no build step in this crate. This also
matches the wider ecosystem: `meta-notation`'s own JS/TS implementation uses a
**PEG.js grammar** ([`library-survey.md`](../library-survey.md) §D.1), so emitting
Peggy keeps the generated JS parser idiomatic to the inheritance root. (A
dependency-free standalone recursive-descent JS emitter was considered as a
fallback — see Reuse — but Peggy removes the need to hand-emit backtracking JS.)

### `GrammarExpr` → Peggy

| `GrammarExpr` | Peggy fragment |
|---|---|
| `Empty` | `""` |
| `Terminal("fn")` | `"fn"` (JSON-escaped) |
| `TerminalInsensitive("fn")` | `"fn"i` |
| `CharRange('a','z')` | `[a-z]` |
| `CharClass { negated:false, items }` | `[a-df-h…]` |
| `CharClass { negated:true, items }` | `[^a-df-h…]` |
| `AnyChar` | `.` |
| `NonTerminal("expr")` | `expr` |
| `Choice { ordered:true, alts }` | `a / b / c` (PEG ordered choice) |
| `Choice { ordered:false, alts }` | `a / b / c` + emit a `// unordered in source` comment (Peggy is ordered-only; record lossy mapping for F2) |
| `Sequence([a,b,c])` | `a b c` |
| `Optional(e)` | `e?` |
| `ZeroOrMore(e)` | `e*` |
| `OneOrMore(e)` | `e+` |
| `Repeat { e, min, max: Some(n) }` | `e\|min..n\|` (Peggy repetition count) |
| `Repeat { e, min, max: None }` | `e\|min..\|` |
| `And(e)` | `&e` |
| `Not(e)` | `!e` |
| `Capture { label: Some(l), e }` | `l:e` (Peggy label binding) |

A1 `RuleKind` does not have a direct Peggy analogue (Peggy is scannerless with no
atomic/silent modifiers); map all kinds to `name = expr` and, for `Atomic`/
`Token` rules, optionally append a `{ return text(); }` action so the rule yields
its matched slice as a string (record this in the F2 matrix as a fidelity note).
Parenthesise sub-expressions when parent binding power is lower than the child
(sequence binds tighter than ordered choice) so the emitted grammar reparses to
the same tree.

### Emit fn signatures

```rust
/// Bundled output of JavaScript parser codegen.
pub struct JsParserArtifacts {
    /// Peggy (PEG.js) grammar source.
    pub peggy_grammar: String,
    /// ESM wrapper that calls `peggy.generate(...)` and exports `parser`.
    pub module: String,
}

/// Emits a Peggy PEG grammar string from the A1 IR.
#[must_use]
pub fn emit_peggy(grammar: &Grammar) -> String;

/// Emits the Peggy grammar plus a small ESM wrapper module.
#[must_use]
pub fn emit_javascript_parser(grammar: &Grammar) -> JsParserArtifacts;
```

### Generated-code skeleton (tiny grammar)

For the A1 grammar `sum = num ("+" num)* ;  num = [0-9]+ ;`, `emit_javascript_parser`
yields:

```javascript
// --- peggy_grammar ---
sum = num ("+" num)*
num = [0-9]+

// --- module (ESM) ---
import peggy from "peggy";
const GRAMMAR = "sum = num (\"+\" num)*\nnum = [0-9]+\n";
export const parser = peggy.generate(GRAMMAR);
// parser.parse("1+2+3") succeeds.
```

## File-level plan

| File | Change |
|---|---|
| `src/grammar/emit/mod.rs` | New (if not created by [`C1`](./C1-bnf-ebnf-abnf-emitters.md)/[`C4`](./C4-rust-parser-codegen.md)). Re-export per-target emit fns. |
| `src/grammar/emit/javascript.rs` | New. `emit_peggy`, `emit_javascript_parser`, `JsParserArtifacts`, the `GrammarExpr`→Peggy walk. |
| `src/lib.rs` | `pub use grammar::emit::{emit_peggy, emit_javascript_parser, JsParserArtifacts};`. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register `grammar_emit_javascript` tests. |
| `tests/fixtures/grammar/javascript/` | Expected `.peggy` snapshots + (optional) a Node smoke-test script. |
| `changelog.d/` | Fragment (`rust-script scripts/create-changelog-fragment.rs`). |

> **No new Rust crate dependency.** Emission is pure string building over the A1
> IR; Peggy is a JS toolchain artefact consumed downstream, never linked into the
> crate. (If a JS smoke test is wired in CI, it uses the already-present Node
> toolchain — see Tests — not a Cargo dependency.)

## Reuse

- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/`RuleKind`/accessors
  (`rules()`, `start_rule()`).
- The `GrammarExpr`-walk skeleton and snapshot-test discipline from
  [`C4`](./C4-rust-parser-codegen.md) (`emit_pest`) — same recursion, different
  target table; keep the two emitters structurally parallel.
- Peggy / PEG.js as the JS codegen target; `meta-notation`'s PEG.js precedent —
  [`library-survey.md`](../library-survey.md) §A.0 (tree-sitter row notes JS),
  §D.1.
- Fallback: a dependency-free standalone recursive-descent JS emitter — recorded,
  not built unless Peggy interop proves unworkable.

## Acceptance criteria

- [ ] `emit_peggy` lowers every `GrammarExpr` variant per the table; the result is
      a syntactically valid Peggy grammar (validated per Tests).
- [ ] `emit_javascript_parser` returns a `JsParserArtifacts` whose `module`
      references `peggy.generate` and embeds the grammar; emission never panics on
      any A1 grammar.
- [ ] Lossy mappings (unordered `Choice` → ordered PEG; `RuleKind` collapse) are
      recorded as comments in the output and noted for the F2 fidelity matrix.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:103-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes (tests live
      under `tests/`, not `src/`).

## Tests

- Unit (`tests/unit/`, new `grammar_emit_javascript` module): build each fixture
  `Grammar` via the A1 builder, call `emit_peggy`, assert the emitted string
  matches a committed snapshot. (Pure Rust, no toolchain — this is the primary,
  always-on gate.)
- Unit: `emit_javascript_parser` for the `sum`/`num` skeleton above → assert the
  `module` contains `peggy.generate(` and the escaped grammar; assert label
  bindings (`l:e`) and repetition counts (`e|min..n|`) render correctly.
- **Lightweight JS validation (CI-light):** a `tests/fixtures/grammar/javascript/`
  Node smoke-test script `smoke.mjs` that imports the emitted `module`, calls
  `parser.parse(...)` on a handful of accept/reject samples, and exits non-zero on
  mismatch. An integration test shells out to `node` **only when `node` and
  `peggy` are available** (gated by a `which node` check; skipped with a logged
  notice otherwise, so CI without a JS toolchain stays green). This keeps the
  default `cargo test` fast while still proving the emitted JS actually parses
  sample inputs when a Node environment is present. Document the
  `npm i peggy` / `node tests/fixtures/grammar/javascript/smoke.mjs` invocation in
  the fixture README so [`E5`](./E5-end-to-end-integration-examples.md) can wire
  it into the full pipeline.
- Assert the emitted grammar parses the same accept/reject set as the Rust parser
  from [`C4`](./C4-rust-parser-codegen.md) for the shared `sum`/`num` fixture
  (cross-target consistency).

## References

- Peggy: <https://peggyjs.org/> · PEG.js: <https://pegjs.org/> ·
  PEG (Ford, POPL '04): see [`library-survey.md`](../library-survey.md) §B PART 1.
- [`library-survey.md`](../library-survey.md) §A.0, §D.1;
  [`existing-capabilities.md`](../existing-capabilities.md) §3 (no JS codegen);
  [`solution-plans.md`](../solution-plans.md) §Epic C.
- [`A1`](./A1-grammar-ir.md), [`C4`](./C4-rust-parser-codegen.md),
  [`E5`](./E5-end-to-end-integration-examples.md).
