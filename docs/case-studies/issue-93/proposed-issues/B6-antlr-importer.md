# B6 — ANTLR v4 (`.g4`) importer

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with a future ANTLR emitter)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B (B6),
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") means every grammar notation must lower into the A1
IR. ANTLR v4 (`.g4`) is the most widely used parser-generator grammar format and
has a huge public corpus ([antlr/grammars-v4](https://github.com/antlr/grammars-v4)),
so importing it brings many real grammars into the links network. This issue
delivers the **ANTLR4** importer following the pattern
[`B1`](./B1-bnf-importer.md) established (the `import_<fmt>` fn + shared
`GrammarImportError` in `src/grammar/import/`).

**No reusable parser crate exists — clean-room required.** Per
[`library-survey.md`](../library-survey.md) §B Part 2, `antlr-rust` (BSD-3-Clause)
is an **ANTLR4 runtime only**: the Java ANTLR tool generates Rust from a `.g4`
ahead of time, so there is **no runtime `.g4`→AST parser in Rust** (the survey's
"Gaps" paragraph states this directly: *"no runtime `.g4` parser"*). Therefore
B6 **specifies a clean-room hand-written parser for the subset of the ANTLR4
meta-grammar below** — we do not vendor or depend on any `.g4`-parsing crate, and
we do not require the Java toolchain. (Stated explicitly per the grounding rule.)

## Goal

Parse an ANTLR4 `.g4` grammar into a `Grammar` (A1 IR), faithfully enough to
re-emit an equivalent grammar later (round-trip deferred to F2), and register it
so `import-grammar --format antlr` (E1) and the `ParserRegistry` path can reach it.

## Scope

**In scope**
- A clean-room recursive-descent parser for the ANTLR4 meta-grammar subset in the
  table below, plus the lowering to A1 IR.
- A public `import_antlr(text: &str) -> Result<Grammar, GrammarImportError>`.
- The lexer-rule (UPPERCASE) vs parser-rule (lowercase) split →
  `RuleKind::Token` vs `RuleKind::Normal`; `fragment` lexer rules → `RuleKind::Silent`.
- `source_format = Some(GrammarFormat::Antlr)` on the produced grammar.

**Out of scope**
- BNF/EBNF/ABNF/PEG/tree-sitter/Lark/GBNF → **B1–B5, B7** (same pattern).
- Embedded target-language actions `{ … }` / semantic predicates `{ … }?` —
  recognise and **drop** them with a recorded note (they are Java/target code, not
  grammar structure); do not attempt to execute them.
- `grammar`/`lexer grammar`/`parser grammar` headers, `options { … }`, `import`,
  `tokens { … }`, `mode`/lexical modes — parse-and-skip with a note (capture the
  `grammar <Name>;` name as the grammar name if a name field exists; otherwise
  drop).
- Emitting `.g4` → a future emitter; round-trip *test* lands with F2.
- A shared importer trait — reuse `GrammarImportError` from
  [`B1`](./B1-bnf-importer.md).

## Design / specification

An ANTLR4 grammar is a header (`grammar X;`) followed by rules
`ruleName : alt1 | alt2 | … ;`. The rule-name's **first letter case** decides its
kind: UPPERCASE → lexer (token) rule, lowercase → parser rule. `fragment NAME : …;`
is a helper token rule never emitted as a token. Lowering:

| ANTLR4 construct | `GrammarExpr` (or rule attribute) |
|---|---|
| `name : …;` where `name` is lowercase | `GrammarRule { name, kind: Normal, .. }` |
| `NAME : …;` where `NAME` is UPPERCASE | `GrammarRule { name, kind: Token, .. }` |
| `fragment NAME : …;` | `GrammarRule { name, kind: Silent, .. }` (helper; not a standalone token) |
| reference `name` / `NAME` (RHS) | `NonTerminal(name)` |
| `'lit'` (string literal) | `Terminal(lit)` (ANTLR string literals are single-quoted) |
| ` a b c ` (juxtaposition) | `Sequence([a,b,c])` |
| `a \| b` | `Choice { ordered: false, alternatives:[a,b] }` (ANTLR is ALL(*) — **treat alternation as unordered**, see note) |
| `( … )` | the inner expression (grouping; no IR node) |
| `e?` | `Optional(e)` |
| `e*` | `ZeroOrMore(e)` |
| `e+` | `OneOrMore(e)` |
| `e??` / `e*?` / `e+?` (non-greedy) | same as greedy `Optional`/`ZeroOrMore`/`OneOrMore` + `Capture { label: Some("non_greedy"), expr }` to preserve the marker (CFG IR has no greediness; note it) |
| `[a-z0-9_]` (lexer char set) | `CharClass { negated: false, items: [Range('a','z'), Range('0','9'), Char('_')] }` |
| `~x` / `~[…]` (negation) | `Not(x)` for a single element; `CharClass { negated: true, .. }` for a negated set |
| `.` (any char) | `AnyChar` |
| `'a'..'z'` (lexer range) | `CharRange('a','z')` |
| `label=e` / `label+=e` (element labels) | `Capture { label: Some(label), expr: e }` |
| `-> skip` / `-> channel(HIDDEN)` / `-> type(X)` (lexer commands) | drop from the expression; carry on the rule as `doc: Some("-> skip")` etc. (preserve as a comment for round-trip; see note) |
| `{ … }` action / `{ … }?` predicate | dropped, recorded in a per-import "dropped constructs" note |
| `// …` / `/* … */` comment | attach to the next rule's `doc` when adjacent; else drop |

**Ordering note (load-bearing).** ANTLR uses ALL(*) (adaptive LL(*)), which is
*not* PEG ordered choice — alternatives are not first-match-wins in the PEG sense.
Lower `|` to `Choice { ordered: false, .. }` to match CFG semantics. Record this
in the importer docs so a future `.g4` emitter knows the ordering is not
significant.

**Lexer-command note.** `-> skip`, `-> channel(name)`, `-> type(name)`,
`-> mode(name)`, `-> pushMode/popMode` follow the rule body. They affect tokenisation,
not phrase structure, so they are **dropped from the `GrammarExpr`** but **carried
on the rule's `doc`** verbatim (e.g. `doc: Some("-> skip")`) so C-side emitters can
re-attach them; this is the documented fidelity edge for B6 (mirrors how
[`B5`](./B5-tree-sitter-json-importer.md) carries `prec`/`token` markers and
[`B7`](./B7-lark-gbnf-importer.md) carries `%ignore`).

```rust
/// Reuses `GrammarImportError` from `src/grammar/import/mod.rs` (defined in B1).
pub fn import_antlr(text: &str) -> Result<Grammar, GrammarImportError>;
```

**Lowering steps:** (1) strip comments/whitespace with a small clean-room lexer
that tokenises identifiers, string literals `'…'`, char sets `[…]`, and the
operators `: ; | ( ) ? * + ~ . = += -> ..`; (2) parse the header and skip
`options`/`tokens`/`import`/`mode` blocks (recording skips); (3) recursive-descent
parse each rule `ruleName : altList ;` into an expression tree, classifying the
rule by first-letter case (and the `fragment` keyword) into `RuleKind`;
(4) lower each parsed node into a `GrammarExpr` via the A1 builder per the table;
(5) assemble the `Grammar` with the first parser rule as start (or the first rule
overall if none), `source_format = Antlr`; (6) any unparseable construct →
`GrammarImportError::Parse { message, .. }`; an explicitly-unsupported-but-parsed
construct → `GrammarImportError::Unsupported { construct }`.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | **No new dependency** — clean-room parser (no `.g4`-parsing crate exists; [`library-survey.md`](../library-survey.md) §B Part 2). If a regex helper is wanted for char-set tokenising, `regex` is already a **dev**-dependency (`Cargo.toml:97`); prefer hand-rolled char scanning to avoid a new runtime dep. |
| `src/grammar/import/mod.rs` | Re-export `import_antlr` (module + `GrammarImportError` from [`B1`](./B1-bnf-importer.md)). |
| `src/grammar/import/antlr.rs` | New. The clean-room lexer + recursive-descent parser + lowering. |
| `src/lib.rs` | `pub use grammar::import::import_antlr;` |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register tests. |
| `tests/fixtures/grammar/antlr/` | A few `.g4` inputs (see Tests). |
| `changelog.d/` | Fragment. |

## Reuse

- A1 `Grammar`/`GrammarExpr`/`CharClassItem`/`RuleKind`/builder for construction
  ([`A1`](./A1-grammar-ir.md)).
- `GrammarImportError` + the `import_<fmt>` pattern from
  [`B1`](./B1-bnf-importer.md).
- Public ANTLR corpus ([antlr/grammars-v4](https://github.com/antlr/grammars-v4))
  for fixtures — **check each file's header licence** (per-grammar; the survey
  notes Java is BSD-3-Clause, others MIT/Apache); prefer hand-written tiny
  fixtures so no third-party grammar is vendored.
- Importers may later register through `ParserRegistry`
  (`src/parser_registry.rs:50-159`) — coordinate with E2, do not duplicate.

## Acceptance criteria

- [ ] `import_antlr` parses each fixture into a `Grammar` with the expected rules,
      start symbol, and `source_format = Antlr`.
- [ ] UPPERCASE rules → `RuleKind::Token`; lowercase → `RuleKind::Normal`;
      `fragment` → `RuleKind::Silent`.
- [ ] Every construct in the table maps to the documented `GrammarExpr`; `|`
      lowers to **unordered** `Choice`; char sets and `~` negation lower correctly.
- [ ] Lexer commands (`-> skip`/`channel`/`type`) are carried on `doc`, not in the
      expression; actions/predicates `{…}`/`{…}?` are dropped and recorded.
- [ ] Malformed `.g4` yields `GrammarImportError::Parse`; an explicitly-unsupported
      construct yields `Unsupported` — never a panic.
- [ ] Non-terminal references resolve; a reference to an undefined symbol is
      reported via A1 `referenced_nonterminals()`.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn`, `Cargo.toml:103-106`), `cargo test --all-features`, and
      `rust-script scripts/check-no-src-tests.rs` all pass (tests live under
      `tests/`, not `src/`); confirm in the PR description that **no new dependency
      was added** (clean-room).

## Tests

Fixtures live under `tests/fixtures/grammar/antlr/`.
- Unit: a small arithmetic `.g4` (parser rules `expr`/`term`/`factor`, lexer rules
  `INT`/`ID`, `WS : [ \t\r\n]+ -> skip;`) → assert rule kinds, the `WS` rule's
  `doc == "-> skip"`, and a spot-checked expression tree.
- Unit: a grammar exercising `?`/`*`/`+` (and one non-greedy `*?`), `(…)`
  grouping, `|`, `[a-z]` char set, `~[…]` negation, `.` any, `'a'..'z'` range, and
  a `label=e` element label.
- Unit: a grammar with a `fragment` rule and an embedded action `{…}` → assert the
  fragment is `Silent` and the action is dropped.
- Unit: malformed input (missing `;`, unbalanced `(`) → `Err(Parse)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`).
- (Deferred to F2 once an ANTLR emitter exists: `.g4` text round-trip.)

## References

- ANTLR4: <https://www.antlr.org/> · ALL(*) report:
  <https://www.antlr.org/papers/allstar-techreport.pdf> · corpus:
  <https://github.com/antlr/grammars-v4>
- `antlr-rust` (runtime only, not a `.g4` parser):
  <https://github.com/rrevenantt/antlr4rust>
- [`library-survey.md`](../library-survey.md) §B Part 2 ("no runtime `.g4`
  parser" — clean-room), [`solution-plans.md`](../solution-plans.md) §Epic B (B6).
</content>
</invoke>
