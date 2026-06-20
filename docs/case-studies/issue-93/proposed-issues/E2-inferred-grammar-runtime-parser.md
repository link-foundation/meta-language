# E2 — Inferred-grammar runtime parser (registry)

> **Epic:** E — Tooling, integration, benchmarking · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** [`D10`](./D10-active-learning-oracle.md)
> **Requirements:** P-1, P-7 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic E,
> [`existing-capabilities.md`](../existing-capabilities.md) §1 (the `ParserRegistry`
> extension point).

## Context

A1 makes a grammar a first-class value, and B1–B7 / D5 produce grammars (imported
or inferred). But a `Grammar` value is inert: today the only way to parse text is
through the fixed [`BuiltInLanguageParser`](../../../../src/language_parser.rs)
dispatch (lino → pdf/docx → data formats → tree-sitter → lossless text,
`src/language_parser.rs:21-47`). There is **no path from "I just inferred/imported
a grammar" to "that grammar now parses new input"** — which is exactly what P-1
("easy to develop new grammars") and P-7 ("working Rust implementation") demand:
a developed grammar must *run*.

The crate already has the seam. `ParserRegistry` (`src/parser_registry.rs:50-159`)
is a pluggable dispatch table where a user registration *shadows* the built-in set
for a language key (`src/parser_registry.rs:8-19, 71-91`), and `LanguageParser`
(`src/language_parser.rs:7-15`) is the single-method boundary
(`fn parse_source(&self, text, language, configuration) -> LinkNetwork`). This
issue plugs an A1 grammar into that seam: a `LanguageParser` that *interprets* a
`Grammar` to parse arbitrary text into a `LinkNetwork`, registered under a chosen
language key.

A parser that answers "does this grammar accept this string?" is also a
**membership oracle** — exactly the input the optional active-learning path
(D10, L\*/TTT) needs, which is why E2 **blocks D10**
([`solution-plans.md`](../solution-plans.md) §4: `E2 ── D10`).

## Goal

Provide `GrammarParser`, a `LanguageParser` (`src/language_parser.rs:7-15`) that
interprets any A1 `Grammar` to parse input text into a lossless `LinkNetwork`,
plus an ergonomic registration into `ParserRegistry`
(`src/parser_registry.rs:50-159`) so an **inferred or imported grammar
immediately parses new input** through the existing `parse_with_registry` path
(`src/parser_registry.rs:144-159`). Expose a boolean `accepts(&self, text)`
membership check so the same object serves as the D10 oracle.

## Scope

**In scope**
- A new module `src/grammar/runtime/mod.rs` (re-exported from `src/lib.rs`).
- `struct GrammarParser { grammar: Grammar, … }` implementing `LanguageParser`.
- A recursive-descent / packrat interpreter over `GrammarExpr` (A1) with PEG
  semantics for ordered `Choice`, and a documented strategy for unordered
  `Choice { ordered: false }` (CFG alternation) so imported BNF/EBNF grammars run.
- `GrammarParser::accepts(&self, text: &str) -> bool` (full-input membership) and
  a `register_grammar(&mut ParserRegistry, key, Grammar)` / `with_grammar(...)`
  convenience that wraps `Arc::new(GrammarParser::new(grammar))`.
- Producing a `LinkNetwork` whose links round-trip (`reconstruct_text()` equals
  the consumed input) and carry parse structure as `LinkType::Grammar`-aligned
  nodes where the grammar rule structure is recoverable.

**Out of scope** (owned elsewhere)
- Constructing the grammar (import → **B1–B7**; inference → **D5**).
- The CLI surface that calls this (`infer`/`import-grammar` → **E1**).
- The L\*/TTT learner that *uses* `accepts` as its oracle → **D10** (E2 only
  exposes the oracle method).
- Parser *codegen* to standalone Rust/JS source → **C4/C5**. E2 is an in-process
  interpreter, not a code generator.
- Authoring diagnostics / friendly grammar errors → **E4**.

## Design / specification

### Trait impl sketch

```rust
use std::sync::Arc;
use crate::grammar::Grammar;                       // A1
use crate::language_parser::LanguageParser;        // src/language_parser.rs:7
use crate::{LinkNetwork, ParseConfiguration, ParserRegistry};

/// A `LanguageParser` that interprets an A1 [`Grammar`] at runtime.
#[derive(Clone, Debug)]
pub struct GrammarParser {
    grammar: Grammar,
}

impl GrammarParser {
    #[must_use]
    pub fn new(grammar: Grammar) -> Self { Self { grammar } }

    /// Membership query: does the grammar accept all of `text`?
    /// This is the oracle D10's active learner consumes.
    #[must_use]
    pub fn accepts(&self, text: &str) -> bool {
        match self.parse_from_start(text) {
            Some(consumed) => consumed == text.len(), // whole input matched
            None => false,
        }
    }

    // Internal: parse from the start rule, returning bytes consumed.
    fn parse_from_start(&self, text: &str) -> Option<usize> { /* see interpreter */ }
}

impl LanguageParser for GrammarParser {
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        // Walk the start rule over `text`; on full match, build a links network
        // mirroring the parse tree. On partial/failed match, fall back to the
        // lossless text boundary so the pipeline never panics (mirrors
        // BuiltInLanguageParser's lossless fallback, src/language_parser.rs:44-45).
        match self.try_parse_network(text, language, configuration) {
            Some(network) => network,
            None => LinkNetwork::parse_lossless_text(text, language, configuration),
        }
    }
}
```

`parse_source` must mirror the existing trait contract exactly (signature at
`src/language_parser.rs:9-14`) and must **never panic** on malformed input — it
degrades to `LinkNetwork::parse_lossless_text` (`src/link_network.rs:364`), the
same lossless fallback `BuiltInLanguageParser` uses (`src/language_parser.rs:44-45`).

### Interpreter semantics (per `GrammarExpr` variant, A1 §Design)

A recursive matcher `match_expr(&GrammarExpr, input, pos) -> Option<usize>`
(returns new position, or `None`), memoised on `(rule, pos)` for packrat-linear
behaviour on PEG grammars:

| `GrammarExpr` | Semantics |
|---|---|
| `Empty` | succeed, consume 0. |
| `Terminal(s)` / `TerminalInsensitive(s)` | match literal at `pos` (ASCII-case-insensitive for the latter). |
| `CharRange(a,b)` | match one char in `a..=b`. |
| `CharClass { negated, items }` | match one char in/notin the set. |
| `AnyChar` | match any one char (PEG `.`). |
| `NonTerminal(name)` | look up rule by name (A1 `Grammar::rule`); recurse; report a clear error on undefined reference (reuse A1 `referenced_nonterminals()` at construction time). |
| `Sequence(es)` | match each in order; fail if any fails. |
| `Choice { ordered: true, … }` | PEG ordered choice: first alternative that matches wins (no backtracking past it). |
| `Choice { ordered: false, … }` | CFG alternation: try alternatives; pick the **longest** match (documented determinisation so unordered BNF/EBNF imports parse predictably). |
| `Optional(e)` | match `e` or consume 0. |
| `ZeroOrMore(e)` / `OneOrMore(e)` | greedy repetition (`+` requires ≥1). |
| `Repeat { e, min, max }` | counted repetition; `max: None` = unbounded. |
| `And(e)` / `Not(e)` | lookahead predicates: match `e` without consuming; `Not` succeeds iff `e` fails. |
| `Capture { label, e }` | match `e`; record a labelled span for the links output. |

Document the two known limits plainly (E4 can improve them): (1) the interpreter
is recursive-descent/PEG, so a **left-recursive** rule from an unordered import
must be detected and reported (do not loop) — detect via a rule-on-same-position
re-entry guard; (2) unordered `Choice` uses longest-match determinisation, not
full Earley/GLR ambiguity, so a genuinely ambiguous CFG parses to one tree. Both
are acceptable for M5: importers (B1–B7) and D5 produce grammars this interpreter
runs, and ambiguous-CFG parsing is explicitly a non-goal here.

### Links output

The produced `LinkNetwork` must be lossless: `reconstruct_text()`
(`src/link_network.rs:495`) equals the input. Build it by walking the successful
parse tree and emitting a node per matched rule/expression so the rule structure
is recoverable; tag structural nodes consistent with A1's `grammar::` term family
(see A1 §"Links encoding") and `LinkType::Grammar` (`src/link_network.rs:51`).
Reuse A1's links plumbing rather than hand-rolling node creation.

### Registration

```rust
/// Register `grammar` under `key`, shadowing the built-in dispatch for that key.
pub fn register_grammar(
    registry: &mut ParserRegistry,
    key: impl Into<String>,
    grammar: Grammar,
) -> &mut ParserRegistry {
    registry.register(key, Arc::new(GrammarParser::new(grammar)))
}

/// Builder-style variant mirroring `ParserRegistry::with_parser`.
#[must_use]
pub fn with_grammar(
    registry: ParserRegistry,
    key: impl Into<String>,
    grammar: Grammar,
) -> ParserRegistry {
    registry.with_parser(key, Arc::new(GrammarParser::new(grammar)))
}
```

This routes through the existing `register`/`with_parser`
(`src/parser_registry.rs:71-91`); once registered, `parse_with_registry`
(`src/parser_registry.rs:144-159`) and `registry.parse(...)`
(`src/parser_registry.rs:124-135`) dispatch new input through the inferred
grammar. No change to the dispatch core.

## File-level plan

| File | Change |
|---|---|
| `src/grammar/runtime/mod.rs` | New. `GrammarParser` (the interpreter + `LanguageParser` impl + `accepts`), the `match_expr` walker with packrat memoisation, and `register_grammar`/`with_grammar`. |
| `src/lib.rs` | `pub use grammar::runtime::{GrammarParser, register_grammar, with_grammar};` next to the existing re-exports (`src/lib.rs:1-36, 60`). |
| `tests/unit/mod.rs` | Register a new `grammar_runtime` unit-test module. |
| `tests/integration/mod.rs` | Register an integration test for the import/infer → register → parse round-trip. |
| `tests/fixtures/grammar/` | Reuse A1/B1 hand-built grammars; add a tiny arithmetic grammar fixture if not already present. |
| `changelog.d/` | Add a fragment (see CONTRIBUTING / `README.md` changelog section). |

## Reuse

- `LanguageParser` trait + `BuiltInLanguageParser` fallback pattern — `src/language_parser.rs:7-47` (copy the lossless-fallback discipline at :44-45).
- `ParserRegistry::{register, with_parser, parse}` + `LinkNetwork::parse_with_registry` — `src/parser_registry.rs:71-91, 124-135, 144-159` (registration + dispatch; do **not** modify the dispatch core).
- `LinkNetwork::{parse_lossless_text, reconstruct_text, links}` — `src/link_network.rs:364, 495, 484` (lossless output + verification).
- A1 `Grammar`/`GrammarExpr`/`rule`/`referenced_nonterminals` + the `grammar::` links encoding — [`A1`](./A1-grammar-ir.md).
- `B1` (and B2–B7) supply imported grammars; `D5` supplies inferred grammars — both feed `GrammarParser::new`.

## Acceptance criteria

- [ ] `GrammarParser` implements `LanguageParser` with the exact trait signature
      (`src/language_parser.rs:9-14`) and is public + documented.
- [ ] For a hand-built arithmetic grammar (A1 builder), `GrammarParser` parses a
      valid expression into a `LinkNetwork` whose `reconstruct_text()` equals the
      input (lossless), and the rule structure is recoverable from the links.
- [ ] Every `GrammarExpr` variant in the table above is exercised by a parsing
      test (ordered vs unordered `Choice`, `*`/`+`/`?`, `Repeat`, `And`/`Not`,
      `Capture`, char classes/ranges, `AnyChar`).
- [ ] `accepts(text)` returns `true` for accepted strings and `false` for
      rejected ones (the D10 oracle contract); whole-input match only.
- [ ] Malformed input never panics: `parse_source` degrades to
      `LinkNetwork::parse_lossless_text` and a left-recursive unordered import is
      detected and reported rather than looping.
- [ ] `register_grammar` / `with_grammar` make a grammar reachable through
      `registry.parse(...)` and `LinkNetwork::parse_with_registry(...)`; a
      registration shadows the built-in dispatch for its key
      (`src/parser_registry.rs:71-91, 144-159`), and unrelated keys still fall
      through to the built-in set.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      `pedantic`/`nursery` are `warn` per `Cargo.toml:103-106`), and
      `cargo test --all-features` all pass; `rust-script scripts/check-no-src-tests.rs`
      passes (tests live under `tests/`, not `src/`).

## Tests

- `tests/unit/` (new `grammar_runtime` module):
  - one parsing test per `GrammarExpr` variant (build via A1 builder, assert
    accept/reject and consumed length).
  - `accepts` truth table for a small grammar: accepted strings → `true`,
    near-misses and superstrings → `false`.
  - lossless: `parse_source(input).reconstruct_text() == input` for several
    inputs of a recursive grammar (arithmetic with precedence).
  - non-panic: malformed input and a left-recursive unordered rule both return
    without panicking (lossless fallback / reported error).
- `tests/integration/`:
  - import a BNF fixture via `import_bnf` ([`B1`](./B1-bnf-importer.md)),
    `register_grammar(&mut reg, "my-bnf", g)`, then
    `LinkNetwork::parse_with_registry(&reg, sample, "my-bnf", cfg)` parses a
    matching sample losslessly — the headline "imported grammar immediately
    parses new input" path.
  - a registered grammar shadows the built-in dispatch for its key while another
    key still routes through the built-in set.
- No network/IO; pure in-process. Keep fixtures inline or under
  `tests/fixtures/grammar/`.

## References

- `LanguageParser` trait + registry: `src/language_parser.rs:7-47`,
  `src/parser_registry.rs:50-159`.
- A1 IR + links encoding: [`A1`](./A1-grammar-ir.md).
- D10 (consumer of the oracle): [`D10`](./D10-active-learning-oracle.md);
  L\*/TTT MAT model — [`literature-review.md`](../literature-review.md) §1.
- PEG packrat semantics (interpreter basis): Ford, *POPL '04* — see
  [`library-survey.md`](../library-survey.md) §B.1.
- [`existing-capabilities.md`](../existing-capabilities.md) §1 (extension point),
  [`solution-plans.md`](../solution-plans.md) §Epic E (E2).
