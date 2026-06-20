# C3 — GBNF emitter (LLM-constraint interop)

> **Epic:** C — Emitters & codegen · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`F2`](./F2-grammar-format-fidelity-matrix.md)
> **Requirements:** P-9, P-12 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic C,
> [`library-survey.md`](../library-survey.md) §E (GBNF / grammar-constrained
> decoding), [`competitive-analysis.md`](../competitive-analysis.md) §2.

## Context

This emitter renders an [`A1`](./A1-grammar-ir.md) `Grammar` as **GBNF** (GGML
BNF, llama.cpp's grammar dialect). GBNF is the de-facto portable
grammar-constraint format for LLM decoding: native to **llama.cpp**, adopted by
**XGrammar** (the default backend of **vLLM** and **SGLang**), and ingestible by
**Guidance** via its `gbnf()` constructor
([`library-survey.md`](../library-survey.md) §E). One GBNF emitter therefore lets
*any* inferred or imported meta-grammar directly constrain LLM generation across
that whole stack.

This is a **P-12 secondary-metric win that no competitor reports.** The CFG
inference line (GLADE → Arvada → TreeVada → Kedavra → NatGI) infers grammars but
does not interoperate with structured-generation engines:
[`competitive-analysis.md`](../competitive-analysis.md) §2 lists "**LLM-constraint
readiness** — emit GBNF (C3) usable directly by llama.cpp/vLLM/XGrammar" as
**"Unique to this project,"** and §4 records "(none — competitors stop at one
grammar)" for the cross-format/codegen/GBNF row. Shipping C3 scores this metric by
construction (competitors score 0). Stress this framing in the issue body.

GBNF is a BNF dialect — `nonterminal ::= sequence...`, the **`root` rule is the
start symbol**, and it adds regex-like operators `| () * + ? {m,n} [...] [^...]`
([`library-survey.md`](../library-survey.md) §B PART 1, §E). So the emitter is a
[`C1`](./C1-bnf-ebnf-abnf-emitters.md)-style walk with GBNF's operator set, reusing
the same `TranslationTemplate` engine (`src/translation_rules.rs:300-366`,
re-exported `src/lib.rs:101-104`) per [`solution-plans.md`](../solution-plans.md)
§Epic C. It pairs with the **B7 GBNF importer** for the round-trip scored in
[`F2`](./F2-grammar-format-fidelity-matrix.md).

## Goal

Provide `emit_gbnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>`
that renders an A1 `Grammar` as a valid `.gbnf` string consumable directly by
llama.cpp / vLLM / SGLang / XGrammar / Guidance, with the start symbol emitted as
the mandatory `root` rule, and PEG-only constructs handled by documented
approximation or rejection.

## Scope

**In scope**
- `src/grammar/emit/gbnf.rs` — the `GrammarExpr → GBNF` walk and `emit_gbnf`.
- The mapping table below, with `root`-rule handling for the start symbol.
- Documented handling of PEG-only constructs (the predicates `And`/`Not`) by
  approximation where faithful, else `GrammarEmitError::Unsupported`.
- Deterministic output (rule order from `Grammar::rules()`, `root` first).

**Out of scope** (owned elsewhere)
- BNF/EBNF/ABNF → [`C1`](./C1-bnf-ebnf-abnf-emitters.md); PEG → [`C2`](./C2-peg-emitter.md).
- The B7 GBNF/Lark **importer** (the round-trip partner).
- **JSON Schema → GBNF** conversion and a JSON-Schema emitter — a documented
  *secondary* LLM-interop target ([`library-survey.md`](../library-survey.md) §E
  recommends GBNF primary, JSON Schema secondary). Note it as future work; do not
  implement here.
- Actually *running* an LLM with the grammar (out of repo scope); C3 emits text.
- The full round-trip matrix → [`F2`](./F2-grammar-format-fidelity-matrix.md);
  C3 ships an emit-then-reimport smoke test only.

## Design / specification

For each `GrammarRule` in `grammar.rules()`, emit `{name} ::= {body}` via a
`TranslationTemplate` (`{name}`/`{body}` placeholders), where `{body}` is
`emit_expr(&rule.expr)`. The start rule **must** be named `root` in the output:
emit `grammar.start_rule()` first under the literal name `root`, and if a
user rule is already called `root`, rename the start rule and reference it from a
synthesised `root ::= <start>` (record the rename in `EmitReport`).

### `GrammarExpr → GBNF`

GBNF operators: alternation `|`, grouping `()`, repetition `* + ? {m,n}`,
character classes `[...]` and negated classes `[^...]`, and string/char-range
syntax inside classes ([`library-survey.md`](../library-survey.md) §B PART 1, §E).

| `GrammarExpr` | GBNF output | Notes |
|---|---|---|
| `Empty` | `""` | empty string literal |
| `Terminal(s)` | `"s"` | double-quoted; escape `"`, `\`, and control chars per GBNF |
| `TerminalInsensitive(s)` | per-char class expansion, e.g. `[Ff][Nn]` + Lossy note | GBNF has no case-insensitive literal; expand each letter to a 2-element class, record loss |
| `CharRange(a,b)` | `[a-b]` | native GBNF range inside a class |
| `CharClass{negated:false, items}` | `[…]` | `Char(c)`→`c`, `Range(a,b)`→`a-b`, concatenated inside `[...]` |
| `CharClass{negated:true, items}` | `[^…]` | native GBNF negated class |
| `AnyChar` | `[^]` *(or a documented `[\x00-\U0010FFFF]`)* | GBNF "any character"; pick one and document it |
| `NonTerminal(n)` | `n` | rule reference (sanitise to GBNF identifier charset; record any rename) |
| `Choice{ordered:_, alts}` | `a \| b \| c` | GBNF `\|` is BNF-style **unordered**; ordered `Choice` emits `\|` + Lossy note (PEG-priority is lost — same caveat as the BNF target in C1) |
| `Sequence(items)` | `a b c` | space-juxtaposition |
| `Optional(e)` | `(e)?` | native |
| `ZeroOrMore(e)` | `(e)*` | native |
| `OneOrMore(e)` | `(e)+` | native |
| `Repeat{e,min,max}` | `(e){min,max}` / `(e){min,}` (`max:None`) / `(e){min}` | native GBNF counted repetition |
| `And(e)` | **approx** or `Unsupported` | GBNF has no lookahead; see "PEG-only constructs" below |
| `Not(e)` | **approx** (negated class) or `Unsupported` | see below |
| `Capture{expr,..}` | emit `expr`, drop label | GBNF has no captures |

Parenthesise sub-expressions by precedence (postfix `* + ? {}` > sequence >
`|`) so the emitted GBNF re-parses (via B7) to the same tree.

### PEG-only constructs (predicates) — documented approximation/rejection

GBNF is recogniser-grammar (BNF + regex repetition) with **no syntactic
predicates** ([`library-survey.md`](../library-survey.md) §B PART 1). Handle the
A1 PEG variants explicitly:

- `Not(CharClass{negated:false, items})` or `Not(Terminal(single_char))` followed
  in a sequence by `AnyChar` (the common "any char except X" idiom) → fold into a
  **negated class `[^…]`** (faithful; no loss). Detect this peephole during the walk.
- Any other `And(e)` / `Not(e)` (general lookahead) → `GrammarEmitError::Unsupported
  { format: Gbnf, construct: "and-predicate" | "not-predicate" }`. GBNF cannot
  approximate arbitrary lookahead without changing the accepted language, so
  rejecting is correct (silent over-/under-approximation would corrupt
  LLM-constraint behaviour). Document this clearly — it is the one place a faithful
  GBNF emit is impossible.

### Emit fn signature and walk

```rust
// Reuses GrammarEmitError / EmitReport from src/grammar/emit/mod.rs (C1).
pub fn emit_gbnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError>;
```

1. Determine the start rule via `grammar.start_rule()` (A1); it becomes `root`.
2. Emit `root ::= {body}` first; then every other rule as `{name} ::= {body}`
   via the production-line `TranslationTemplate`.
3. `emit_expr` matches every `GrammarExpr` variant per the table, applying the
   `Not(...) ~ AnyChar → [^…]` peephole and the predicate rejection rule.
4. Sanitise non-terminal names to GBNF's identifier charset
   (`[A-Za-z][A-Za-z0-9-]*` by convention); record any rename in `EmitReport` so
   B7's importer and F2 can account for it.
5. Ordered `Choice` and `TerminalInsensitive` add `EmitReport.lossy` notes;
   unsupported predicates return `Err(Unsupported)`.

The output is a single `.gbnf` document with `root` as the entry rule, ready to
pass to `llama-cli --grammar-file`, vLLM `guided_grammar`, or XGrammar/Guidance
`gbnf()` ([`library-survey.md`](../library-survey.md) §E).

## File-level plan

| File | Change |
|---|---|
| `src/grammar/emit/gbnf.rs` | New. `emit_gbnf` + the `GrammarExpr → GBNF` walk, `root` handling, predicate peephole/rejection. |
| `src/grammar/emit/mod.rs` | Add `pub use gbnf::emit_gbnf;` (reuses `GrammarEmitError`/`EmitReport` from [`C1`](./C1-bnf-ebnf-abnf-emitters.md)). |
| `src/lib.rs` | `pub use grammar::emit::emit_gbnf;` next to existing re-exports (`src/lib.rs:83-104`). |
| `tests/unit/mod.rs` | Extend the `grammar_emit` module (registered for C1). |
| `tests/fixtures/grammar/emit/` | Golden `.gbnf` outputs (incl. a JSON-shaped grammar — GBNF's canonical use case — and a `[^…]` peephole case). |
| `changelog.d/` | Add a fragment. |

## Reuse

- **`TranslationTemplate`** (`src/translation_rules.rs:300-366`, re-exported
  `src/lib.rs:101-104`) — render the `{name} ::= {body}` production line; mandated
  over `format!` by [`solution-plans.md`](../solution-plans.md) §Epic C.
- A1 `Grammar`/`GrammarExpr` and `start_rule()` (for the `root` rule) —
  [`A1`](./A1-grammar-ir.md).
- `GrammarEmitError`/`EmitReport` from [`C1`](./C1-bnf-ebnf-abnf-emitters.md)'s
  `src/grammar/emit/mod.rs` — do not redefine.
- The `reconstruct_text_as` "IR → target text" precedent (`src/reconstruction.rs:14`,
  `README.md:248`).
- No new dependency: GBNF is plain text. (`library-survey` §E also notes a Rust
  `gbnf` crate and `json-schema-to-gbnf` bridge — relevant only to the deferred
  JSON-Schema path, not needed here.) `unsafe` forbidden (`Cargo.toml:100-101`);
  clippy pedantic+nursery `warn` (`Cargo.toml:103-106`).

## Acceptance criteria

- [ ] `emit_gbnf` is public, documented, and deterministic; the start symbol is
      always emitted as the GBNF `root` rule (with a recorded rename if a user rule
      already owns the name `root`).
- [ ] Every `GrammarExpr` variant maps per the table; `[a-b]` ranges, `[...]` /
      `[^...]` classes, `* + ? {m,n}`, and `|` are emitted with GBNF syntax.
- [ ] The `Not(class) ~ AnyChar` "any-char-except" idiom folds to `[^…]`; all other
      `And`/`Not` predicates return `GrammarEmitError::Unsupported` (named), never a
      panic and never a silent language-changing approximation.
- [ ] Ordered `Choice`, `TerminalInsensitive`, and name sanitisation are recorded
      in `EmitReport.lossy`/rename log; no silent loss.
- [ ] The emitted text is valid GBNF — a fixture is checked to round-trip through
      the B7 importer (smoke test; full matrix is [`F2`](./F2-grammar-format-fidelity-matrix.md)),
      and at least one fixture is the canonical JSON-object grammar (the standard
      GBNF/structured-generation example, [`library-survey.md`](../library-survey.md) §E).
- [ ] The issue body states the **P-12 secondary-metric framing** (LLM-constraint
      readiness is unique vs all competitors — [`competitive-analysis.md`](../competitive-analysis.md) §2, §4).
- [ ] The production line is produced via `TranslationTemplate`, not ad-hoc `format!`.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery `warn`, `Cargo.toml:103-106`), `cargo test --all-features` pass;
      `rust-script scripts/check-no-src-tests.rs` passes (tests under `tests/`).

## Tests

- `tests/unit/` (`grammar_emit` module):
  - Hand-built `Grammar` covering every variant → assert output equals the committed
    golden `.gbnf` fixture; spot-check `root` placement, `[a-b]`, `[^…]`, `{m,n}`.
  - A grammar whose start rule is the *first* rule and one where it is *not* → both
    emit a correct `root`; the "user rule named `root`" case records a rename.
  - `Not(class) ~ AnyChar` → asserts a `[^…]` peephole result.
  - General `And`/`Not` → `Err(Unsupported)`.
  - Ordered `Choice`/`TerminalInsensitive` → `EmitReport.lossy` entry.
  - The canonical JSON-object grammar fixture emits valid GBNF.
- Round-trip smoke (gated on B7; `#[ignore]` with a comment pointing at
  [`F2`](./F2-grammar-format-fidelity-matrix.md) until B7 exists):
  `import_gbnf(emit_gbnf(g))` is structurally equal to `g` modulo documented loss.
- Pure in-process; only committed fixtures are read.

## References

- GBNF / grammar-constrained decoding (llama.cpp, XGrammar, vLLM, SGLang,
  Guidance): [`library-survey.md`](../library-survey.md) §E; GBNF syntax §B PART 1.
- P-12 secondary metric "LLM-constraint readiness — unique to this project":
  [`competitive-analysis.md`](../competitive-analysis.md) §2 and §4.
- Pairing importer: B7 (Lark + GBNF); [`solution-plans.md`](../solution-plans.md) §Epic B.
- IR being emitted: [`A1`](./A1-grammar-ir.md); shared emit scaffolding:
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md).
- `TranslationTemplate` engine: `src/translation_rules.rs:300-366`,
  [`existing-capabilities.md`](../existing-capabilities.md) §1.
- Round-trip fidelity matrix: [`F2`](./F2-grammar-format-fidelity-matrix.md);
  [`solution-plans.md`](../solution-plans.md) §Epic C, §3 table (row C3), §4 DAG.
