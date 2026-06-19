# B5 — tree-sitter `grammar.json` importer

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with C7)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B (B5),
> [`library-survey.md`](../library-survey.md) §B Part 2,
> [`existing-capabilities.md`](../existing-capabilities.md) §1.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") means every grammar notation must lower into the A1
IR. tree-sitter is **already a central front-end of this crate**: the adapter at
`src/tree_sitter_adapter.rs:136-207` wires ~30 grammars (Python, Java, C/C++, C#,
JS/TS, Rust, Go, Ruby, SQL, HTML, CSS, JSON, YAML, TOML, XML, INI, protobuf,
GraphQL, PHP, Swift, Kotlin, Scala, Lua, Perl, Pascal, VB). Each tree-sitter
grammar crate ships a machine-readable **`grammar.json`** (the deterministic
output of `tree-sitter generate`; the `grammar.js` JavaScript DSL must be
JS-evaluated first, so there is **no pure-Rust `grammar.js` parser** —
[`library-survey.md`](../library-survey.md) §B Part 2 confirms this and says to
"target `grammar.json` instead"). This issue imports that JSON DSL into the A1
IR, turning all ~30 already-wired grammars into ready import fixtures and golden
oracles. It pairs with the C7 `grammar.js` emitter for round-trip (F2).

## Goal

Parse a tree-sitter `grammar.json` document into a `Grammar` (A1 IR), faithfully
enough to re-emit an equivalent tree-sitter grammar (round-trip with
[`C7`](./C7-tree-sitter-grammar-js-emitter.md)), and register it so
`import-grammar --format tree-sitter` (E1) and the `ParserRegistry` path can reach
it. Use `serde_json` (already a dependency — `Cargo.toml:55`); add **no** new crate.

## Scope

**In scope**
- A `grammar.json` → `Grammar` lowering, parsing the JSON via `serde_json`.
- A public `import_tree_sitter_json(text: &str) -> Result<Grammar, GrammarImportError>`.
- Mapping every documented node type (`SYMBOL`, `STRING`, `PATTERN`, `SEQ`,
  `CHOICE`, `REPEAT`, `REPEAT1`, `PREC`/`PREC_LEFT`/`PREC_RIGHT`/`PREC_DYNAMIC`,
  `TOKEN`, `IMMEDIATE_TOKEN`, `FIELD`, `ALIAS`, `BLANK`) to a `GrammarExpr`.
- `source_format = Some(GrammarFormat::TreeSitter)` on the produced grammar.

**Out of scope**
- BNF/EBNF/ABNF/PEG/ANTLR/Lark/GBNF → **B1–B4, B6, B7** (same pattern;
  [`B1`](./B1-bnf-importer.md) defines `GrammarImportError`).
- Parsing the `grammar.js` JavaScript DSL — out of scope (no Rust evaluator);
  consume the generated `grammar.json` only (note this explicitly in docs).
- Emitting `grammar.js` → [`C7`](./C7-tree-sitter-grammar-js-emitter.md). Round-trip *test*
  lives with F2 once C7 exists.
- A shared importer trait — reuse the minimal `GrammarImportError` from
  [`B1`](./B1-bnf-importer.md); if a shared trait emerges, refactor there.

## Design / specification

A `grammar.json` document is `{ "name", "rules": { <name>: <node>, … },
"extras", "externals", "precedences", "word", "conflicts", "inline",
"supertypes", "reserved" }` (verified against a real
`grammar.schema.json`-conformant file). Each `<node>` is an object tagged by a
`"type"` field; the table is the exact construct → `GrammarExpr` mapping. Lower
each top-level rule's node into a `GrammarRule`; the **first key** in `rules` is
the start symbol (tree-sitter's first rule is the implicit start), so
`set_start(first_rule_name)`.

| `grammar.json` node (`type` + payload) | `GrammarExpr` |
|---|---|
| `{type:"SYMBOL", name}` | `NonTerminal(name)` |
| `{type:"STRING", value}` | `Terminal(value)` |
| `{type:"PATTERN", value}` | `CharClass`/`Sequence` if trivially a class; else carry the regex via a `Capture { label: Some("regex"), expr: Terminal(value) }` placeholder (note: lossy — see below) |
| `{type:"BLANK"}` | `Empty` |
| `{type:"SEQ", members:[…]}` | `Sequence([…])` |
| `{type:"CHOICE", members:[…]}` | `Choice { ordered: false, alternatives:[…] }` (tree-sitter is GLR/unordered) |
| `{type:"CHOICE"}` containing a `BLANK` member | `Optional(other)` when exactly `[e, BLANK]` (canonical tree-sitter `optional()`); else `Choice` with an `Empty` alternative |
| `{type:"REPEAT", content}` | `ZeroOrMore(content)` |
| `{type:"REPEAT1", content}` | `OneOrMore(content)` |
| `{type:"PREC", value, content}` | lower `content`; carry precedence as `Capture { label: Some("prec=<v>"), expr }` |
| `{type:"PREC_LEFT"/"PREC_RIGHT"/"PREC_DYNAMIC", value, content}` | lower `content`; carry associativity/value as `Capture { label: Some("prec_left=<v>"…), expr }` |
| `{type:"TOKEN", content}` | lower `content`; mark the owning rule `RuleKind::Token` (or wrap `Capture { label: Some("token"), .. }` for inline tokens) |
| `{type:"IMMEDIATE_TOKEN", content}` | as `TOKEN` but `Capture { label: Some("immediate_token"), .. }` (no whitespace before) |
| `{type:"FIELD", name, content}` | `Capture { label: Some(name), expr: content }` |
| `{type:"ALIAS", value, named, content}` | `Capture { label: Some("alias:<value>"), expr: content }` |

`value`s for `PREC*` are signed integers (e.g. `-1`, `0`, `1`); render them into
the label verbatim. `PATTERN` holds a JS/tree-sitter regex string; **fully
lowering regex into `CharClass`/`Repeat` is out of scope here** — recognise the
trivial single character-class case (`[…]`, `[^…]`) and otherwise carry the raw
regex string so C7 can re-emit it (document this as the one lossy edge, mirroring
how [`B6`](./B6-antlr-importer.md)/[`B7`](./B7-lark-gbnf-importer.md) carry
notes). Top-level `extras`, `externals`, `word`, `conflicts`, `inline`,
`supertypes`, and `reserved` are **not** rules: capture `word`/`extras` as synthetic
rules or drop with a documented note (prefer: import `extras` members as a
synthetic `_extras` rule for fidelity; record dropped metadata in the round-trip
note). Surface JSON or schema failures as `GrammarImportError::Parse`.

```rust
#[derive(serde::Deserialize)]   // private deser structs in this module only
struct TsGrammar { name: String, rules: serde_json::Map<String, serde_json::Value>, /* … */ }

/// Reuses `GrammarImportError` from `src/grammar/import/mod.rs` (defined in B1).
pub fn import_tree_sitter_json(text: &str) -> Result<Grammar, GrammarImportError>;
```

**Lowering steps:** (1) `serde_json::from_str::<serde_json::Value>` the document,
erroring to `Parse` on failure; (2) read `rules` as an object (error if missing),
preserving key order via `serde_json::Map` (serde_json preserves insertion order
without `preserve_order` for the `Map` iteration we rely on — if order matters,
read keys from the raw text order or sort deterministically and document it);
(3) for each `(name, node)`, recursively lower `node` into a `GrammarExpr` by
matching on its `"type"` tag per the table, building with the A1 builder;
(4) wrap into `GrammarRule { name, expr, kind, … }` (`kind = Token` when the rule
body is a top-level `TOKEN`/`IMMEDIATE_TOKEN`, else `Normal`); (5) assemble the
`Grammar`, `set_start(first_rule_name)`, `source_format = TreeSitter`;
(6) an unknown `"type"` tag → `GrammarImportError::Unsupported { construct }`.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | **No new dependency** — `serde_json = "1"` already present (`Cargo.toml:55`, locked to 1.0.150). |
| `src/grammar/import/mod.rs` | Re-export `import_tree_sitter_json` (module + `GrammarImportError` created by [`B1`](./B1-bnf-importer.md)). |
| `src/grammar/import/tree_sitter_json.rs` | New. `import_tree_sitter_json` + the recursive node-lowering walk + private `serde` deser structs. |
| `src/lib.rs` | `pub use grammar::import::import_tree_sitter_json;` (next to the B1 re-exports). |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register the new tests. |
| `tests/fixtures/grammar/tree-sitter/` | A few `grammar.json` inputs (see Tests). |
| `changelog.d/` | Fragment. |

## Reuse

- `serde_json` (MIT OR Apache-2.0; already a dependency, `Cargo.toml:55`) — JSON
  parsing. Existing usage to mirror: `src/concept_ontology.rs:6,755` and
  `src/translation_rules.rs:544,565`. **No new dependency; no licence to record.**
- A1 `Grammar`/`GrammarExpr`/`RuleKind`/builder for construction
  ([`A1`](./A1-grammar-ir.md)).
- `GrammarImportError` + the `import_<fmt>` pattern from
  [`B1`](./B1-bnf-importer.md).
- The ~30 wired tree-sitter grammars (`src/tree_sitter_adapter.rs:136-207`) — each
  crate ships a `grammar.json` usable as a fixture and a golden oracle.
- Importers may later register through `ParserRegistry`
  (`src/parser_registry.rs:50-159`) — coordinate with E2, do not duplicate.

## Acceptance criteria

- [ ] `import_tree_sitter_json` parses each fixture into a `Grammar` with the
      expected rules, start symbol (first key in `rules`), and
      `source_format = TreeSitter`.
- [ ] Every node type in the table maps to the documented `GrammarExpr`; the
      `[e, BLANK]` CHOICE special-cases to `Optional`.
- [ ] Malformed JSON, a missing `rules` object, or an unknown node `"type"` yields
      `GrammarImportError::{Parse,Unsupported}` — never a panic.
- [ ] Non-terminal references (`SYMBOL`) resolve; references to undefined symbols
      are reported via A1 `referenced_nonterminals()`. (Note: tree-sitter
      `externals` are legitimately undefined here — exclude them or record them.)
- [ ] A real shipped `grammar.json` from a wired grammar (e.g. JSON or Lua, the
      smallest) imports without error.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn`, `Cargo.toml:103-106`), `cargo test --all-features`, and
      `rust-script scripts/check-no-src-tests.rs` all pass (tests live under
      `tests/`, not `src/`); no new dependency to record.

## Tests

Fixtures live under `tests/fixtures/grammar/tree-sitter/`.
- Unit: a hand-written minimal `grammar.json` exercising every node type
  (`SYMBOL`, `STRING`, `PATTERN`, `SEQ`, `CHOICE` (plain + the `[e,BLANK]`
  optional form), `REPEAT`, `REPEAT1`, `PREC*`, `TOKEN`, `IMMEDIATE_TOKEN`,
  `FIELD`, `ALIAS`, `BLANK`) → assert rule count, names, `RuleKind::Token` on the
  token rule, and a spot-checked expression tree.
- Unit: a real shipped `grammar.json` (copy the smallest wired grammar's file,
  e.g. `tree-sitter-json`) → assert it imports, has the expected start symbol, and
  resolves all internal `SYMBOL` references.
- Unit: malformed JSON, `{}` (no `rules`), and `{"rules":{"x":{"type":"WAT"}}}`
  → `Err(Parse)` / `Err(Unsupported)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`).
- (Deferred to F2 once C7 lands: `import_tree_sitter_json` ∘ `emit_grammar_js`
  round-trip across the ~30 wired grammars as golden oracles.)

## References

- tree-sitter grammar DSL: <https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html>
  · `grammar.json` schema: <https://tree-sitter.github.io/tree-sitter/assets/schemas/grammar.schema.json>
  · `tree-sitter generate`: <https://tree-sitter.github.io/tree-sitter/cli/generate.html>
- `serde_json`: <https://docs.rs/serde_json>
- [`library-survey.md`](../library-survey.md) §B Part 2 (no pure-Rust `grammar.js`
  parser; target `grammar.json`), [`solution-plans.md`](../solution-plans.md)
  §Epic B (B5), [`existing-capabilities.md`](../existing-capabilities.md) §1
  (tree-sitter adapter, `src/tree_sitter_adapter.rs:136-207`).
</content>
</invoke>
