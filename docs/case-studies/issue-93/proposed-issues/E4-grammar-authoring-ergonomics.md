# E4 — Grammar authoring ergonomics

> **Epic:** E — Tooling, integration, benchmarking · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`A2`](./A2-grammar-surface-syntax.md) · **Blocks:** —
> **Requirements:** P-1 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic E (E4),
> [`requirements.md`](../requirements.md) P-1.

## Context

Requirement **P-1** is "allow to **easily develop** grammars inherited from the
meta-language" ([`requirements.md`](../requirements.md) P-1). [`A1`](./A1-grammar-ir.md)
gives us a `Grammar` value and [`A2`](./A2-grammar-surface-syntax.md) lets a human
*write* one as meta-notation-derived surface text, but neither tells the author
*what is wrong* with a grammar they just wrote. A hand-written grammar typically
has a few recurring, easy-to-miss defects: a non-terminal referenced but never
defined (a typo), a rule that left-recurses (which loops forever under the PEG /
recursive-descent execution model [`E2`](./E2-inferred-grammar-runtime-parser.md)
targets), a rule no path from the start symbol can reach (dead code), and rules
that match the empty string inside a repetition (which loops). Today nothing
detects any of these — [`A1`](./A1-grammar-ir.md) ships only
`referenced_nonterminals()` as a raw building block, and there is no diagnostic
type at all.

This issue makes authoring *ergonomic*: a single `validate(&Grammar)` pass that
returns structured, span-bearing diagnostics so the author (or the
[`E1`](./E1-cli-grammar-subcommands.md) CLI, or an editor) can point at the exact
rule and explain the problem and the fix. It is the P-1 "easy to develop" half of
Epic E, paired with [`A2`](./A2-grammar-surface-syntax.md)'s authoring surface.

## Goal

Provide a pure, dependency-free **grammar validator** —
`validate(grammar: &Grammar) -> Vec<GrammarDiagnostic>` — that detects the common
hand-authoring defects (undefined references, left recursion, unreachable rules,
empty/nullable repetition, duplicate rules, unused captures) and reports each as a
structured `GrammarDiagnostic` carrying a kind, a human message, a severity, and a
source span pointing at the offending rule, so authors get friendly, actionable
errors.

## Scope

**In scope**
- A new public module `src/grammar/validate.rs` (re-exported from `src/lib.rs`).
- `GrammarDiagnostic`, `DiagnosticKind`, `Severity`, and a `RuleSpan` location type.
- `validate(&Grammar) -> Vec<GrammarDiagnostic>` plus one focused checker per
  `DiagnosticKind` (private fns), composed by `validate`.
- Friendly, fix-suggesting messages (e.g. *"rule `expr` is left-recursive via
  `expr → term → expr`; rewrite as repetition or use `|` factoring"*).
- A convenience `Grammar::validate(&self)` shim and a `GrammarDiagnostic::is_error`
  helper so a caller can fail fast on `Severity::Error`.

**Out of scope** (owned elsewhere)
- The `Grammar` IR, `referenced_nonterminals()`, and spans on rules →
  [`A1`](./A1-grammar-ir.md) (this issue *consumes* them).
- The surface parser and its `GrammarSurfaceError` parse-time errors →
  [`A2`](./A2-grammar-surface-syntax.md) (E4 validates a *parsed* `Grammar`, after
  A2 has produced it; A2's `UndefinedReference` is a parse-time fast path, E4 is the
  full semantic pass).
- Wiring diagnostics into a CLI command / exit codes → [`E1`](./E1-cli-grammar-subcommands.md).
- Rewriting / repairing a grammar (e.g. auto-eliminating left recursion) — report
  only; transformation belongs to inference generalization
  ([`D7`](./D7-generalization-mdl-minimization.md)).
- Inference-quality metrics (precision/recall/MDL) → [`D1`](./D1-inference-evaluation-harness.md).

## Design / specification

`validate` runs each checker over the `Grammar` and concatenates their
diagnostics, sorted by source position so output is deterministic. Every checker
is pure (no IO, no network) and operates only on the [`A1`](./A1-grammar-ir.md)
`Grammar`/`GrammarExpr` tree.

```rust
/// Where a diagnostic applies — the rule and (when A1 carries it) the byte span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleSpan {
    pub rule: String,
    /// Byte range in the surface source, when the grammar was parsed from text
    /// (A2 records spans on rules); `None` for in-code / inferred grammars.
    pub span: Option<core::ops::Range<usize>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity { Error, Warning }

/// The classes of authoring defect E4 detects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// A non-terminal reference whose name no rule defines (the cycle-free
    /// complement of A1 `referenced_nonterminals()` vs `rule_names()`).
    UndefinedNonTerminal { name: String, referenced_in: String },
    /// A rule reachable from itself without consuming input, e.g. `a = a b`.
    /// `cycle` is the rule chain that proves it (`["expr","term","expr"]`).
    LeftRecursion { cycle: Vec<String> },
    /// A rule not reachable from the start symbol (dead code).
    UnreachableRule { name: String },
    /// A rule whose body can match the empty string when that is almost
    /// certainly unintended (e.g. an all-optional rule, or `e*` where `e` is
    /// nullable — a non-terminating repetition).
    NullableRepetition { rule: String, detail: String },
    /// Two rules defined with the same name (the later one shadows / conflicts).
    DuplicateRule { name: String },
    /// A `Capture { label: Some(_), .. }` whose label is never otherwise
    /// meaningful for this grammar (informational; Warning).
    UnusedCapture { rule: String, label: String },
}

/// One validation finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarDiagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    /// Friendly, fix-suggesting message (the text an author reads).
    pub message: String,
    pub location: RuleSpan,
}

impl GrammarDiagnostic {
    #[must_use]
    pub fn is_error(&self) -> bool { self.severity == Severity::Error }
}

/// Runs every checker and returns all diagnostics, sorted by source position.
#[must_use]
pub fn validate(grammar: &Grammar) -> Vec<GrammarDiagnostic>;
```

### Checker algorithms (each pure, over the A1 tree)

1. **Undefined non-terminals** (`Error`). Compute
   `grammar.referenced_nonterminals()` ([`A1`](./A1-grammar-ir.md)) minus
   `grammar.rule_names()`; emit one `UndefinedNonTerminal` per leftover, recording
   the rule it was referenced in. (For grammars produced by
   [`B3`](./B3-abnf-importer.md) etc. the importer's injected core rules are already
   present, so they will not be flagged.)
2. **Left recursion** (`Error`). Build the rule-reference graph restricted to
   references reachable **without first consuming a terminal** — i.e. follow the
   leftmost branch of `Sequence`, every branch of `Choice`, and through
   `Optional`/`ZeroOrMore`/`Repeat{min:0,..}` (which can match empty), but *stop* at
   any `Terminal`/`CharRange`/`CharClass`/`AnyChar`/`OneOrMore`(of a non-nullable).
   Run a DFS for back-edges; each back-edge yields a `LeftRecursion` with the cycle
   path. This is the defect that makes a PEG / recursive-descent grammar
   ([`E2`](./E2-inferred-grammar-runtime-parser.md)) loop, so it is the headline
   check. Reuse a small `nullable(expr)` predicate (below) to decide which edges are
   "non-consuming".
3. **Unreachable rules** (`Warning`). BFS/DFS the *full* reference graph from the
   start symbol (`grammar.start_rule()`); any rule not visited is `UnreachableRule`.
4. **Nullable repetition** (`Warning`). Using `nullable(expr)` (a rule body is
   nullable if it can derive ε: `Empty`, `Optional`, `ZeroOrMore`, `Repeat{min:0}`,
   a `Choice` with any nullable branch, a `Sequence` of all-nullable parts, or a
   `NonTerminal` to a nullable rule — computed by fixpoint), flag any
   `ZeroOrMore(e)`/`OneOrMore(e)`/`Repeat{expr:e,..}` whose `e` is nullable (an
   infinite-loop hazard) and any rule whose entire body is nullable.
5. **Duplicate rules** (`Error`). Group `grammar.rules()` by name; any name with >1
   definition is `DuplicateRule`. (Importers that legitimately merge `=/`
   alternatives, e.g. [`B3`](./B3-abnf-importer.md), have already merged before E4
   sees the grammar, so this only fires on genuine author duplicates.)
6. **Unused captures** (`Warning`). Walk for `Capture { label: Some(l), .. }`; this
   is informational scaffolding for editors and is the only `Warning`-by-default
   *and* lowest-priority check — keep it cheap and last.

The `nullable` fixpoint and the two graph builds are shared helpers so the module
stays small. No checker mutates the grammar; `validate` borrows `&Grammar`.

### Friendly messages

Each diagnostic's `message` names the rule, states the problem in one sentence,
and suggests a fix — modeled on the tone of `LanguageProfileViolation`'s `Display`
(`src/language_profile.rs:426`). Examples:

- `UndefinedNonTerminal { name: "experssion", .. }` → *"rule `expr` references
  undefined non-terminal `experssion` — did you mean `expression`? Define it or fix
  the spelling."* (nearest-name hint via simple edit distance over `rule_names()`).
- `LeftRecursion { cycle: ["expr","expr"] }` → *"rule `expr` is left-recursive
  (`expr → expr`); a recursive-descent/PEG parser will not terminate. Rewrite using
  repetition (`term ("+" term)*`) or factor the common prefix."*

## File-level plan

| File | Change |
|---|---|
| `src/grammar/validate.rs` | New. `GrammarDiagnostic`, `DiagnosticKind`, `Severity`, `RuleSpan`, `validate`, the per-kind checkers, and the shared `nullable` + reference-graph helpers. |
| `src/grammar/mod.rs` | Add `pub mod validate;` (module tree created by [`A1`](./A1-grammar-ir.md)). |
| `src/lib.rs` | `pub use grammar::validate::{validate, GrammarDiagnostic, DiagnosticKind, Severity, RuleSpan};` next to the A1/A2 re-exports (`src/lib.rs:44`, alongside the `grammar::` exports A1/A2 add). |
| `tests/unit/mod.rs` | Register a `grammar_validate` unit-test module (`mod grammar_validate;`). |
| `tests/fixtures/grammar/invalid/` | A handful of deliberately-broken `.mlg` surface grammars (one per diagnostic kind). |
| `changelog.d/` | Add a fragment (see `scripts/check-changelog-fragment.rs` / `scripts/create-changelog-fragment.rs`). |

## Reuse

- [`A1`](./A1-grammar-ir.md) `Grammar`/`GrammarExpr`: `referenced_nonterminals()`,
  `rule_names()`, `rules()`, `start_rule()` — the validator is pure analysis over
  these; it adds no new IR.
- [`A2`](./A2-grammar-surface-syntax.md) `parse_grammar_surface` produces the
  `Grammar` (and the per-rule spans `RuleSpan` reads); the validate fixtures are A2
  surface files.
- `LanguageProfileViolation` (`src/language_profile.rs:406-436`) — the precedent for
  a small, `Display`-able diagnostic type with a friendly message; match its tone
  and `Error`/`Display` shape (do not depend on it).
- No third-party dependency: edit-distance for the "did you mean" hint is a ~15-line
  Levenshtein helper kept private to the module (clippy pedantic/nursery clean).

## Acceptance criteria

- [ ] `validate`, `GrammarDiagnostic`, `DiagnosticKind`, `Severity`, `RuleSpan` are
      public and documented (doc-comment on each public item).
- [ ] Each `DiagnosticKind` is produced by at least one fixture and asserted:
      `UndefinedNonTerminal`, `LeftRecursion` (direct `a = a b` **and** indirect
      `a = b`, `b = a`), `UnreachableRule`, `NullableRepetition`, `DuplicateRule`,
      `UnusedCapture`.
- [ ] A **valid** grammar (e.g. the A2 arithmetic example) yields **zero** `Error`
      diagnostics (a clean grammar passes).
- [ ] `LeftRecursion` reports the proving cycle path; `UndefinedNonTerminal` reports
      a nearest-name suggestion when one exists within edit distance ≤ 2.
- [ ] `validate` output is deterministic (sorted by `RuleSpan`) and never panics on
      any input grammar, including an empty grammar and a single-rule cycle.
- [ ] Every diagnostic carries a non-empty, fix-suggesting `message` and a
      `location` whose `rule` is a real rule name.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml:103-106`), and
      `cargo test --all-features` all pass; `rust-script scripts/check-no-src-tests.rs`
      passes (tests live under `tests/`, not `src/`).

## Tests

- `tests/unit/grammar_validate.rs`:
  - one test per `DiagnosticKind`, each building the offending `Grammar` via the
    [`A1`](./A1-grammar-ir.md) builder **or** parsing an `invalid/*.mlg` fixture via
    [`A2`](./A2-grammar-surface-syntax.md), asserting the expected kind, severity,
    and `location.rule`.
  - left-recursion: assert detection of direct (`a = a b`), indirect
    (`a = b; b = a`), and *non*-detection of right recursion (`a = b a` is fine) and
    of recursion guarded by a leading terminal (`a = "x" a` is fine).
  - nullable repetition: `a = b*` with `b = "x"?` flags; `a = b*` with
    `b = "x"` does not.
  - a clean grammar (A2 arithmetic example) → `validate(&g)` has no `Error`.
  - determinism: `validate` called twice returns identical `Vec`.
  - "did you mean": a one-character typo in a reference produces a suggestion.
- Pure in-process, no IO/network; fixtures inline or under
  `tests/fixtures/grammar/invalid/`.

## References

- [`requirements.md`](../requirements.md) P-1; [`solution-plans.md`](../solution-plans.md)
  §Epic E (E4), §3 issue table (E4 row), §4 DAG (`A2 → E4`).
- [`A1`](./A1-grammar-ir.md) (`referenced_nonterminals`, `rule_names`, `start_rule`),
  [`A2`](./A2-grammar-surface-syntax.md) (surface + spans),
  [`E1`](./E1-cli-grammar-subcommands.md) (consumes diagnostics),
  [`E2`](./E2-inferred-grammar-runtime-parser.md) (the execution model left recursion
  breaks).
- Diagnostic-type precedent: `src/language_profile.rs:406-436`
  (`LanguageProfileViolation` + `Display`).
- Left-recursion / nullability background: standard recursive-descent/PEG analysis
  (Ford, POPL '04 — see [`library-survey.md`](../library-survey.md) §B PART 1, PEG).
