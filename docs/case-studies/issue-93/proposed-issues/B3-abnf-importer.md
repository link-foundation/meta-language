# B3 — ABNF (RFC 5234) importer

> **Epic:** B — Grammar-format importers · **Blocked by:** A1 · **Blocks:** F2 (with C1)
> **Requirements:** P-10 · **Milestone:** M2
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic B,
> [`library-survey.md`](../library-survey.md) §B.

## Context

Requirement P-10 ("we also should include PEG, BNF and other languages, to be
parsed *as* meta-language") requires every grammar notation to import into the
[`A1`](./A1-grammar-ir.md) IR. [`B1`](./B1-bnf-importer.md) established the
importer pattern (the shared `GrammarImportError` and `import_<fmt>` fns in
`src/grammar/import/mod.rs`); [`B2`](./B2-ebnf-importer.md) added EBNF. This issue
adds **ABNF** — **RFC 5234** (STD 68) plus the RFC 7405 case-sensitive
`%s`/`%i` extension — the notation the IETF uses for protocol grammars. ABNF is
the first format that exercises A1's counted-repetition (`Repeat { min, max }`),
numeric/hex terminals (`%x`), and character ranges.

## Goal

Parse ABNF text into a `Grammar` (A1 IR), losslessly enough to re-emit equivalent
ABNF (round-trip with C1), and register it so `import-grammar --format abnf` (E1)
and the `ParserRegistry` path can reach it. Reuse [`B1`](./B1-bnf-importer.md)'s
`GrammarImportError` and module layout; add only the ABNF-specific lowering.

## Scope

**In scope**
- An `abnf` → `Grammar` lowering using the `abnf` crate (MIT OR Apache-2.0 — see
  [`library-survey.md`](../library-survey.md) §B, PART 2 table).
- A public `import_abnf(text: &str) -> Result<Grammar, GrammarImportError>`.
- Mapping every ABNF construct (incl. `%x`/`%d`/`%b` numeric terminals & ranges,
  `*`/`n*m` repetition, `=/` incremental alternatives) to the right `GrammarExpr`.
- The 16 RFC 5234 Appendix-B **core rules** (`ALPHA`, `DIGIT`, `HEXDIG`, `BIT`,
  `CR`, `LF`, `CRLF`, `SP`, `HTAB`, `WSP`, `DQUOTE`, `CHAR`, `CTL`, `OCTET`,
  `VCHAR`, `LWSP`) made resolvable.
- `source_format = Some(GrammarFormat::Abnf)` on the produced grammar.

**Out of scope**
- BNF → [`B1`](./B1-bnf-importer.md); EBNF → [`B2`](./B2-ebnf-importer.md);
  PEG/tree-sitter/ANTLR/Lark/GBNF → B4–B7.
- Emitting ABNF → **C1**; the text round-trip *test* lives with **F2** once C1 exists.
- The shared `GrammarImportError` is **owned by [`B1`](./B1-bnf-importer.md)** —
  reuse it, do not redefine it.
- `prose-val` (`< … >` free-text placeholders) is lowered to an `Unsupported`
  error, not silently dropped.

## Design / specification

ABNF grammar: a list of rules `name = elements` (no angle brackets), `name =/
elements` for incremental alternatives, where elements are `/`-separated
alternatives of space-separated concatenations; repetition prefixes a term.
Lowering (`abnf` crate `Node` enum → A1 `GrammarExpr`):

| ABNF construct | `abnf` crate node | `GrammarExpr` |
|---|---|---|
| `name = …` (rule head) | `Rule { name(), node(), kind() }` | `GrammarRule { name, .. }` |
| `name =/ …` (incremental alt.) | second `Rule`, `kind() == Incremental` | merge into existing rule's `Choice { ordered: false, .. }` |
| rulename reference | `Node::Rulename(s)` | `NonTerminal(s)` |
| `a / b` (alternation) | `Node::Alternatives(vec)` | `Choice { ordered: false, alternatives }` |
| `a b c` (concatenation) | `Node::Concatenation(vec)` | `Sequence([a,b,c])` |
| `*a`, `n*m a`, `n a` | `Node::Repetition { repeat, node }` | see repetition mapping below |
| `[ a ]` (optional) | `Node::Optional(box)` | `Optional(a)` |
| `( a )` (grouping) | `Node::Group(box)` | inner expr (grouping is structural only) |
| `"lit"` (case-insensitive) | `Node::String(s)` | `TerminalInsensitive(s)` |
| `%s"lit"` (case-sensitive, RFC 7405) | `Node::String` w/ sensitivity flag | `Terminal(s)` |
| `%i"lit"` (explicit insensitive) | `Node::String` w/ flag | `TerminalInsensitive(s)` |
| `%x41` / `%d65` / `%b01000001` (single value) | `Node::TerminalValues(..)` (single) | `Terminal(<char>)` |
| `%x41-5A` etc. (value **range**) | `Node::TerminalValues(..)` (range) | `CharRange(lo, hi)` |
| `%x41.42.43` (concatenated values) | `Node::TerminalValues(..)` (series) | `Sequence([Terminal,…])` or a multi-char `Terminal` |
| `< prose >` | `Node::Prose(s)` | `GrammarImportError::Unsupported { construct: "prose-val" }` |

**Repetition mapping** (RFC 5234 §3.6–3.7) — `Node::Repetition { repeat, node }`:

| ABNF | semantics | `GrammarExpr` |
|---|---|---|
| `*a` | 0 or more | `ZeroOrMore(a)` (or `Repeat { min: 0, max: None }`) |
| `1*a` | 1 or more | `OneOrMore(a)` (or `Repeat { min: 1, max: None }`) |
| `n*a` (n>1) | at least n | `Repeat { expr: a, min: n, max: None }` |
| `*m a` | at most m | `Repeat { expr: a, min: 0, max: Some(m) }` |
| `n*m a` | between n and m | `Repeat { expr: a, min: n, max: Some(m) }` |
| `n a` (exact) | exactly n | `Repeat { expr: a, min: n, max: Some(n) }` |

Prefer the dedicated `ZeroOrMore`/`OneOrMore`/`Optional` variants when the bounds
match them exactly (so the IR is canonical and round-trips through C1 cleanly);
otherwise use `Repeat { min, max }`. Numeric terminal radices: `%x` hex, `%d`
decimal, `%b` binary — decode the digits to a `char` (validate it is a valid
Unicode scalar; reject otherwise with `Unsupported`).

> **Read the exact `abnf` crate API from `docs.rs/abnf` before coding.**
> [`library-survey.md`](../library-survey.md) §B lists `Vec<Rule>` with
> `Rule{name(),node(),kind()}` and a `Node` enum
> (`Alternatives/Concatenation/Repetition/Rulename/Group/Optional/String/TerminalValues/Prose`)
> — treat the variant/method names as indicative and match on the real types.
> Entry point is `abnf::rulelist(text)` (or the crate's documented parse fn).

```rust
// Defined by B1 in src/grammar/import/mod.rs; reused here unchanged:
//   pub enum GrammarImportError { Parse { format, message }, Unsupported { format, construct } }
pub fn import_abnf(text: &str) -> Result<Grammar, GrammarImportError>;
```

Lowering walk (`lower_node(node) -> Result<GrammarExpr, GrammarImportError>`):
1. Parse with the `abnf` crate → `Vec<Rule>`; for each, `lower_node(node())` per
   the two tables above. Map any parse failure to `Parse { format: Abnf, message }`.
2. Group rules by name. A `kind() == Incremental` (`=/`) rule **appends** its
   lowered expression as another branch of the base rule's unordered `Choice`
   (create the `Choice` if the base was a single expression).
3. `Prose` → `Err(Unsupported { format: Abnf, construct: "prose-val" })`.
4. **Inject the core rules**: for every `NonTerminal` whose name is an unresolved
   RFC 5234 Appendix-B core rule, add the corresponding A1 rule (e.g.
   `ALPHA = CharClass { negated: false, items: [Range('A','Z'), Range('a','z')] }`,
   `DIGIT = CharRange('0','9')`, …), built from the [`A1`](./A1-grammar-ir.md)
   builder in a small `core_rules()` helper. Then run `referenced_nonterminals()`
   and report any *remaining* undefined references.

## File-level plan

| File | Change |
|---|---|
| `Cargo.toml` | Add `abnf = "0.13"` (pin the latest; confirm **MIT OR Apache-2.0**). Optionally add `abnf-core` (MIT OR Apache-2.0) if its core-rule parsers help; otherwise hand-build the core rules. Reuse the `grammar-import` feature gate if [`B1`](./B1-bnf-importer.md) introduced one. |
| `src/grammar/import/mod.rs` | Add `mod abnf; pub use abnf::import_abnf;` (reuse the existing `GrammarImportError`). |
| `src/grammar/import/abnf.rs` | New. `import_abnf`, the `lower_node` walk, and the `core_rules()` helper. |
| `src/lib.rs` | Extend the import re-export: `pub use grammar::import::import_abnf;`. |
| `tests/unit/mod.rs` + `tests/integration/mod.rs` | Register the new tests. |
| `tests/fixtures/grammar/abnf/` | A few `.abnf` inputs (see Tests). |
| `changelog.d/` | Fragment. |

## Reuse

- `abnf` crate (MIT OR Apache-2.0) — parsing. See [`library-survey.md`](../library-survey.md)
  §B (PART 2 table: `abnf` 0.13.0, the de-facto ABNF parser); optional `abnf-core`
  for core-rule parsers.
- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`/builder (esp. `Repeat`,
  `CharRange`, `CharClass`, `Terminal`, `TerminalInsensitive`);
  `referenced_nonterminals()` for undefined-reference checks.
- [`B1`](./B1-bnf-importer.md) `GrammarImportError` and `src/grammar/import/` layout.
- Pattern parallel: existing `LanguageParser` adapters dispatch in
  `src/language_parser.rs:7-47`; importers may later register through
  `ParserRegistry` (`src/parser_registry.rs:50-159`) — coordinate with **E2**.

## Acceptance criteria

- [ ] `import_abnf` parses each fixture into a `Grammar` with the expected rules,
      start symbol (first rule unless specified), and `source_format = Abnf`.
- [ ] Repetition lowers per the repetition table: `*x`→`ZeroOrMore`,
      `1*x`→`OneOrMore`, `3x`→`Repeat{3,Some(3)}`, `2*4x`→`Repeat{2,Some(4)}`.
- [ ] `%x41`→`Terminal("A")`, `%x30-39`→`CharRange('0','9')`, and `%d`/`%b`
      radices decode correctly.
- [ ] Case sensitivity is honored: bare `"abc"` and `%i"abc"` →
      `TerminalInsensitive`; `%s"abc"` → `Terminal` (RFC 7405).
- [ ] `=/` incremental alternatives merge into the base rule's unordered `Choice`.
- [ ] Referenced core rules (`ALPHA`/`DIGIT`/…) are injected and resolve; any
      remaining undefined reference is reported (reuse A1 `referenced_nonterminals()`).
- [ ] `prose-val` (`< … >`) yields `GrammarImportError::Unsupported`; malformed
      ABNF yields `GrammarImportError::Parse`; neither panics.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (pedantic +
      nursery are `warn` per `Cargo.toml:105-106`), and `cargo test --all-features`
      all pass; `rust-script scripts/check-no-src-tests.rs` passes; each new
      dependency's licence is recorded in the PR description.

## Tests

- Unit (`tests/unit/`, new `grammar_import_abnf` module): each fixture → assert
  rule count, names, and a spot-checked expression tree (e.g. `postal-address`
  uses `*`/`1*`/`%d` and resolves `ALPHA`/`DIGIT`).
- Unit: every repetition form and every numeric radix (`%x`/`%d`/`%b`, single &
  range) round-trips to the documented `GrammarExpr`.
- Unit: `%s`/`%i` case-sensitivity table; an `=/` incremental-alt input merges.
- Unit: a `prose-val` input → `Err(Unsupported)`; a broken input → `Err(Parse)`.
- Integration: import a fixture, then assert it survives the A1 links round-trip
  (`from_links(to_links(g)) == g`, per [`A1`](./A1-grammar-ir.md)).
- Fixtures under `tests/fixtures/grammar/abnf/`:
  - `postal-address.abnf` — the canonical RFC 5234 §1.3 example
    (`postal-address = name-part street zip-part`, with `*`/`1*` repetition and
    core-rule references).
  - `numeric-terminals.abnf` — rules exercising `%x`/`%d`/`%b` single values,
    ranges (`%x30-39`), and concatenated series (`%x41.42.43`).
  - `incremental.abnf` — a base rule plus a `=/` incremental alternative.
- (Deferred to **F2** once C1 lands: `import_abnf` ∘ `emit_abnf` text round-trip.)

## References

- `abnf` crate: <https://docs.rs/abnf> · ABNF: **RFC 5234**
  <https://www.rfc-editor.org/rfc/rfc5234> (core rules in Appendix B) · case
  sensitivity: **RFC 7405** <https://www.rfc-editor.org/rfc/rfc7405>
- [`library-survey.md`](../library-survey.md) §B (PART 1 ABNF, PART 2 `abnf`/`abnf-core` rows),
  [`solution-plans.md`](../solution-plans.md) §Epic B, [`A1`](./A1-grammar-ir.md),
  [`B1`](./B1-bnf-importer.md).
