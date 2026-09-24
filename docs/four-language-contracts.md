# Four-language representation contracts

This document defines schema revision 2 of the JavaScript, Rust, Lean, and Rocq
surface shared by the Rust crate and JavaScript package. It is a truthful
snapshot of work in progress toward the full delivery target preserved in the
[issue #195 requirement ledger](issue-195-requirement-ledger.md), not a reduced
definition of that target. Callers can inspect the contract before attempting a
transform or translation.

## Supported profiles

| Canonical name | Parser aliases | Declared release | Edition/surface | Extensions |
| --- | --- | --- | --- | --- |
| JavaScript | `javascript`, `js`, `ecmascript` | ECMAScript 2026 | ECMA-262, 17th edition | `.js`, `.mjs`, `.cjs` |
| Rust | `rust`, `rs` | Rust 1.98.1 | 2024 | `.rs` |
| Lean | `lean`, `lean4` | Lean 4.33.1 | Lean 4 | `.lean` |
| Rocq | `rocq`, `coq` | Rocq 9.2 | Vernacular | `.v` |

Extensions are declared metadata. Parsing is selected by canonical name or
alias, or through `ParserRegistry`; it is not inferred from a filename.

Both runtimes use registered grammar frontends for all four languages,
including the pinned Rocq grammar. Grammar-backed paths retain the returned
CST, named fields, child order, exact spans, recovery flags, and source tokens.
The program representation then adds project context, scopes, bindings,
module/dependency facts, types and universes, extensions, proofs, and
surface-to-representation mappings. Missing dependency context produces an
explicit diagnostic instead of fabricated resolution.

## Stable pipeline

The contract is:

```text
source text -> registered parser -> links network -> structural transform
            -> ordered source-token emitter -> source text
```

Each source token retains its text, language, UTF-8 byte range, start/end point,
and recovery flags. Syntax links reference tokens or child syntax links.
Emission sorts the retained source tokens by span, so it remains available
after the caller discards the original source buffer. Structured identifier
edits update the captured token rather than searching the raw source; strings
and comments therefore do not match an `(identifier)` query.

`LANGUAGE_REPRESENTATION_SCHEMA_VERSION` is `2` in both packages. The public
`language_support`/`languageSupport` APIs describe fidelity, and the Rust and
JavaScript parser registries permit explicit extension without silently
changing built-in dispatch.

## Construct and fidelity inventory

The table reports the common capability available in **both** runtime packages.
“Resolved” means declarations and references carry stable scoped identities;
“elaborated” means surface facts are retained with their representation phase.
Proof syntax is explicitly not applicable to JavaScript and Rust.

| Construct or layer | JavaScript | Rust | Lean | Rocq | Evidence |
| --- | --- | --- | --- | --- | --- |
| Complete UTF-8 source and trivia | preserved | preserved | preserved | preserved | shared conformance corpus and reconstruction tests |
| UTF-8 byte spans and line/column points | concrete | concrete | concrete | concrete | parser implementations and corpus tests |
| Comments, strings, numbers, identifiers, keywords, delimiters | grammar CST | grammar CST | grammar CST | grammar CST | real grammar nodes/tokens plus shared corpus |
| Nested grammar hierarchy and recovery | concrete | concrete | concrete | concrete | positive and malformed corpus cases |
| Full grammar-level syntax hierarchy | available | available | available | available | grammar CST materialization in both runtimes |
| Identifier scope and binding resolution | resolved | resolved | resolved | resolved | shared symbol-identity, shadowing, and capture tests |
| Imports and module references | resolved with context | resolved with context | resolved with context | resolved with context | positive project context and missing-context diagnostics |
| Declarations and recursive bodies | resolved | resolved | resolved | resolved | program construct inventory and recursion evidence |
| Types, effects, universes, and elaboration | elaborated | elaborated | elaborated | elaborated | phase-tagged facts and shared semantic corpus |
| Attributes, macros, notation, and plugins | resolved | resolved | resolved | resolved | language-specific extension facts and project extensions |
| Proof and tactic syntax | not applicable | not applicable | elaborated | elaborated | proof/tactic facts retain source mappings |
| Unknown/control syntax | diagnostic and retained | diagnostic and retained | diagnostic and retained | diagnostic and retained | recovery flags plus exact reconstruction |
| Source generation after mutation | ordered token emission | ordered token emission | ordered token emission | ordered token emission | reconstruction and identifier-edit tests |

The inventory prevents a fallback token stream from being described as
grammar-complete, resolved, or elaborated syntax.

## Translation contracts

Both packages expose all 12 directed source/target pairs. At schema revision 2
each pair emits a target-language artifact containing the versioned portable
source encoding; none relabels or passes through source text as target code.
Every result names the source and target, the observable input layer, required
target runtime, registered encoding, assumptions, and a precise obligation.

| Target | Required validator/runtime |
| --- | --- |
| JavaScript | ECMAScript 2026 host |
| Rust | Rust 1.98.1, edition 2024 |
| Lean | Lean 4.33.1 kernel and project environment |
| Rocq | Rocq 9.2 kernel and project environment |

The observation is exact source bytes and the resolved source representation
after decoding. The encoding is UTF-8 represented as lowercase hexadecimal in
a native comment, next to a valid target declaration. It preserves constructs
that have no direct target equivalent without equating their semantics. The
declared assumption is that a consumer decodes the envelope before executing
the source language; target parsing and native validation are tested
separately from exact decoding and representation preservation.

## Conformance and boundaries

[`parity/fixtures/four-language-conformance.json`](../parity/fixtures/four-language-conformance.json)
is consumed by both runtime suites. It covers versions, editions, aliases,
extensions, Unicode identifiers, exact reconstruction, syntax roots,
identifier classification, comments, malformed input, regular expressions,
template interpolation, and grammar diagnostics. The versioned
[`language-grammar-inventory.json`](../parity/language-grammar-inventory.json)
additionally audits every known language target and tests all aliases currently
marked as real-grammar paths. The parity manifest no longer exempts
`language_parser` or `parser_registry` from the JavaScript implementation.

RML remains responsible for RML syntax, selectable foundations, logic,
execution semantics, and proof authority. Reusable scope, type, module,
proof-syntax, provenance, and translation concepts remain upstream here.

## Distribution status

The changelog fragment for this work requests the next minor release in the
existing release workflow. Published npm/crates installation evidence can only
be recorded after merge and release. The continuing npm distribution concern
is tracked by [issue #171](https://github.com/link-foundation/meta-language/issues/171);
source-level parity must not be mistaken for published-package parity.
