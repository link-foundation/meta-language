# B7 — Lark + GBNF importer

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (GBNF with [`C3`](./C3-gbnf-emitter.md))
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B (B7),
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") means every grammar notation must lower into the A1
IR. Two small, high-value formats remain after B1–B6: **Lark** `.lark` (the EBNF
dialect consumed by the Outlines/Guidance LLM-constraint stacks) and **GBNF**
(GGML BNF, llama.cpp's grammar format — the de-facto portable LLM-constraint
grammar adopted by XGrammar/vLLM/SGLang, see
[`library-survey.md`](../library-survey.md) §E). Both are compact, so they share
one issue with **two mapping tables and two import functions**. Importing GBNF
**closes the round-trip loop** with the [`C3`](./C3-gbnf-emitter.md) GBNF emitter
(F2 fidelity). This issue follows the pattern [`B1`](./B1-bnf-importer.md)
established (the `import_<fmt>` fn + shared `GrammarImportError`).

**No reusable parser crate exists for either format — clean-room required.** Per
[`library-survey.md`](../library-survey.md) §B Part 2: there is **no Lark `.lark`
parser crate** (the permissive EBNF crates `ebnf`/`abnf`/`pest_meta` parse other
dialects, and the table lists **no Lark entry**), and the Rust **`gbnf` crate is
an emit/convert target, not a `.gbnf`-text→AST parser** (§E lists it only as a
"JSON Schema → GBNF" converter; no GBNF *parser* is listed). Therefore B7
**specifies clean-room hand-written parsers for both formats** — we do not vendor
or depend on any Lark/GBNF-parsing crate. (Stated explicitly per the grounding
rule.)

## Goal

Parse Lark `.lark` and llama.cpp GBNF text into a `Grammar` (A1 IR), and register
both so `import-grammar --format {lark,gbnf}` (E1) and the `ParserRegistry` path
can reach them. The GBNF importer is the inverse of [`C3`](./C3-gbnf-emitter.md),
enabling the GBNF text round-trip in F2.

## Scope

**In scope**
- Two clean-room recursive-descent parsers (Lark, GBNF) + lowering to A1 IR.
- Two public fns: `import_lark(text) -> Result<Grammar, GrammarImportError>` and
  `import_gbnf(text) -> Result<Grammar, GrammarImportError>`.
- Lark rule (lowercase) vs terminal (UPPERCASE) split → `RuleKind::Normal` vs
  `RuleKind::Token`; GBNF rules are uniform (`RuleKind::Normal`).
- `source_format = Some(GrammarFormat::Lark)` / `Some(GrammarFormat::Gbnf)`.

**Out of scope**
- BNF/EBNF/ABNF/PEG/tree-sitter/ANTLR → **B1–B6** (same pattern).
- Lark's parser-engine directives that don't affect structure (`%declare`,
  `tree`-shaping `?`-inline beyond the IR's capture, priority `.N` on rules) —
  parse-and-record with a note; do not model parser-backend selection.
- Emitting GBNF → [`C3`](./C3-gbnf-emitter.md); emitting Lark → a future emitter.
  Round-trip *tests* land with F2 (GBNF round-trip is the headline once C3 exists).
- A shared importer trait — reuse `GrammarImportError` from
  [`B1`](./B1-bnf-importer.md).

## Design / specification

### (a) Lark `.lark` → A1 IR

A Lark grammar is a list of `rule: expansion` (rules **lowercase**) and
`TERMINAL: expansion` (terminals **UPPERCASE**), with `|` alternation, EBNF
operators, regex terminals `/…/`, and `%`-directives. Lowering:

| Lark construct | `GrammarExpr` (or attribute) |
|---|---|
| `name : …` (lowercase LHS) | `GrammarRule { name, kind: Normal, .. }` |
| `NAME : …` (UPPERCASE LHS) | `GrammarRule { name, kind: Token, .. }` |
| `?name : …` (inline-if-single-child) | `GrammarRule { name, kind: Silent, .. }` (Lark's `?` rule prefix; record the inline hint as `doc`) |
| reference `name` / `NAME` (RHS) | `NonTerminal(name)` |
| `"lit"` (string literal) | `Terminal(lit)` |
| `/regex/` (regex terminal) | `Capture { label: Some("regex"), expr: Terminal(body) }` (carry the raw regex; trivial `[…]` classes may lower to `CharClass` — see note) |
| ` a b c ` (juxtaposition) | `Sequence([a,b,c])` |
| `a \| b` | `Choice { ordered: false, alternatives:[a,b] }` (Lark CFG alternation is unordered) |
| `( … )` | inner expression (grouping; no IR node) |
| `[ … ]` (Lark optional group) | `Optional(inner)` |
| `e?` | `Optional(e)` |
| `e*` | `ZeroOrMore(e)` |
| `e+` | `OneOrMore(e)` |
| `e ~ n` | `Repeat { expr: e, min: n, max: Some(n) }` (exact count) |
| `e ~ m..n` | `Repeat { expr: e, min: m, max: Some(n) }` |
| `[a-z]` char set (in a terminal) | `CharClass { negated: false, items: [Range('a','z')] }` |
| `%ignore X` | top-level directive: record as a synthetic `_ignore` note on the grammar (carry for round-trip; mirrors B6 `-> skip`) |
| `%import …` | resolve if the imported module is bundled; otherwise record as `Unsupported`-style note and continue (document the limitation) |
| `// …` comment | attach to next rule's `doc` when adjacent; else drop |

### (b) GBNF (llama.cpp) → A1 IR

A GBNF grammar is a list of `name ::= sequence`, with the **`root` rule as the
start symbol**, `|` alternation, the EBNF operators, char classes `[…]`/`[^…]`,
grouping `(…)`, and string literals. Lowering:

| GBNF construct | `GrammarExpr` (or attribute) |
|---|---|
| `name ::= …` | `GrammarRule { name, kind: Normal, .. }`; if `name == "root"`, `set_start("root")` |
| reference `name` (RHS) | `NonTerminal(name)` |
| `"lit"` (string literal) | `Terminal(lit)` |
| ` a b c ` (juxtaposition) | `Sequence([a,b,c])` |
| `a \| b` | `Choice { ordered: false, alternatives:[a,b] }` (GBNF is BNF-derived; unordered) |
| `( … )` | inner expression (grouping; no IR node) |
| `e?` | `Optional(e)` |
| `e*` | `ZeroOrMore(e)` |
| `e+` | `OneOrMore(e)` |
| `e{m,n}` / `e{m,}` / `e{m}` | `Repeat { expr: e, min: m, max: Some(n)/None }` |
| `[a-z0-9_]` (char class) | `CharClass { negated: false, items: [Range('a','z'), Range('0','9'), Char('_')] }` |
| `[^…]` (negated class) | `CharClass { negated: true, items: [...] }` |
| `\x41` / `é` / `\n` (escapes in class/literal) | decode to the `char`; build `Char`/`Range` items or `Terminal` text |
| `# …` comment | attach to next rule's `doc` when adjacent; else drop |

**Regex/char-class note (load-bearing, both formats).** A1 has first-class
`CharClass`/`CharRange`/`Repeat`, so the common cases lower exactly. Lark's `/…/`
regex terminals can hold arbitrary regex; **fully lowering arbitrary regex is out
of scope** — recognise the trivial single-class case and otherwise carry the raw
regex string in a `Capture { label: Some("regex"), .. }` so a future Lark emitter
re-emits it (the one documented fidelity edge, mirroring
[`B5`](./B5-tree-sitter-json-importer.md)/[`B6`](./B6-antlr-importer.md)). GBNF has
no free-form regex (only char classes + the listed operators), so GBNF import is
lossless and the C3 round-trip (F2) should be exact.

```rust
/// Both reuse `GrammarImportError` from `src/grammar/import/mod.rs` (defined in B1).
pub fn import_lark(text: &str) -> Result<Grammar, GrammarImportError>;
pub fn import_gbnf(text: &str) -> Result<Grammar, GrammarImportError>;
```

**Lowering steps (each fn):** (1) tokenise with a small clean-room lexer
(identifiers, string literals, char classes `[…]`, regex `/…/` for Lark, the
operators `::=`/`:`/`|`/`(`/`)`/`[`/`]`/`?`/`*`/`+`/`~`/`{m,n}` as applicable, and
comments `//` for Lark / `#` for GBNF); (2) recursive-descent parse each
production into an expression tree, classifying Lark rules by first-letter case
(and the `?` prefix) into `RuleKind`; (3) lower each node into a `GrammarExpr` via
the A1 builder per the matching table; (4) assemble the `Grammar` — Lark start =
first rule (or a conventional `start` rule if present); GBNF start = `root` —
setting `source_format`; (5) any unparseable construct →
`GrammarImportError::Parse { message, .. }`; an explicitly-unsupported construct
(e.g. unresolved `%import`) → `GrammarImportError::Unsupported { construct }`.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | **No new dependency** — clean-room parsers (no Lark/GBNF parser crate exists; [`library-survey.md`](../library-survey.md) §B Part 2). Prefer hand-rolled char scanning over `regex` (a dev-dependency only, `Cargo.toml:97`). |
| `src/grammar/import/mod.rs` | Re-export `import_lark`, `import_gbnf` (module + `GrammarImportError` from [`B1`](./B1-bnf-importer.md)). |
| `src/grammar/import/lark.rs` | New. Clean-room Lark lexer + parser + lowering. |
| `src/grammar/import/gbnf.rs` | New. Clean-room GBNF lexer + parser + lowering. |
| `src/lib.rs` | `pub use grammar::import::{import_lark, import_gbnf};` |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register tests. |
| `tests/fixtures/grammar/lark/` + `tests/fixtures/grammar/gbnf/` | Inputs (see Tests). |
| `changelog.d/` | Fragment. |

## Reuse

- A1 `Grammar`/`GrammarExpr`/`CharClassItem`/`RuleKind`/builder for construction
  ([`A1`](./A1-grammar-ir.md)).
- `GrammarImportError` + the `import_<fmt>` pattern from
  [`B1`](./B1-bnf-importer.md).
- [`C3`](./C3-gbnf-emitter.md) GBNF emitter — `import_gbnf` is its inverse; the two
  together give the F2 GBNF text round-trip. Coordinate the char-class/escape
  encoding so `import_gbnf ∘ emit_gbnf` is identity.
- Importers may later register through `ParserRegistry`
  (`src/parser_registry.rs:50-159`) — coordinate with E2, do not duplicate.

## Acceptance criteria

- [ ] `import_lark` and `import_gbnf` each parse their fixtures into a `Grammar`
      with the expected rules, start symbol (Lark: first/`start`; GBNF: `root`),
      and the correct `source_format`.
- [ ] Lark: lowercase → `RuleKind::Normal`, UPPERCASE → `RuleKind::Token`, `?name`
      → `RuleKind::Silent`; `~ n` / `~ m..n` lower to `Repeat`.
- [ ] GBNF: `root` becomes the start symbol; `{m,n}`/`{m,}`/`{m}` lower to
      `Repeat`; `[…]`/`[^…]` lower to `CharClass` (negated correctly).
- [ ] Every construct in both tables maps to the documented `GrammarExpr`; `|`
      lowers to **unordered** `Choice` in both.
- [ ] Malformed input (either format) yields `GrammarImportError::Parse`; an
      explicitly-unsupported construct yields `Unsupported` — never a panic.
- [ ] Non-terminal references resolve in both; references to undefined symbols are
      reported via A1 `referenced_nonterminals()`.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn`, `Cargo.toml:103-106`), `cargo test --all-features`, and
      `rust-script scripts/check-no-src-tests.rs` all pass (tests live under
      `tests/`, not `src/`); confirm in the PR description that **no new dependency
      was added** (clean-room).

## Tests

Fixtures live under `tests/fixtures/grammar/lark/` and `tests/fixtures/grammar/gbnf/`.
- Unit (Lark): a small grammar with rules + UPPERCASE terminals, `|`, `[…]`
  optional group, `?`/`*`/`+`, a `~ 3` exact repeat, a `/regex/` terminal, and
  `%ignore WS` → assert rule kinds, the `~` → `Repeat`, the `%ignore` note, and a
  spot-checked tree.
- Unit (GBNF): the canonical llama.cpp arithmetic / JSON `root ::= …` grammar →
  assert `root` is start, `[…]`/`[^…]` and `{m,n}` lower correctly.
- Unit: malformed input for each (`:` / `::=` missing, unbalanced `[`) →
  `Err(Parse)`; unresolved `%import` → `Err(Unsupported)` (or recorded note per the
  documented choice).
- Integration: import a fixture of each format, then assert it survives the A1
  links round-trip (`from_links(to_links(g)) == g`).
- (Deferred to F2 once [`C3`](./C3-gbnf-emitter.md) lands: the **GBNF text
  round-trip** `import_gbnf ∘ emit_gbnf == identity` — the headline fidelity test
  for this issue.)

## References

- Lark grammar reference:
  <https://lark-parser.readthedocs.io/en/latest/grammar.html>
- GBNF (llama.cpp):
  <https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md>
- [`library-survey.md`](../library-survey.md) §B Part 2 (no Lark/GBNF parser crate
  — clean-room) and §E (GBNF as the portable LLM-constraint target),
  [`solution-plans.md`](../solution-plans.md) §Epic B (B7),
  [`C3`](./C3-gbnf-emitter.md) (the GBNF emitter this closes the loop with).
</content>
</invoke>
