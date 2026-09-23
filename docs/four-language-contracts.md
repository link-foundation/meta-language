# Four-language representation contracts

This document defines schema revision 1 of the JavaScript, Rust, Lean, and Rocq
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
| Lean | `lean`, `lean4` | Lean 4.34.0 | Lean 4 | `.lean` |
| Rocq | `rocq`, `coq` | Rocq 9.3.0 | Vernacular | `.v` |

Extensions are declared metadata. Parsing is selected by canonical name or
alias, or through `ParserRegistry`; it is not inferred from a filename.

Both runtimes now use tree-sitter grammars for JavaScript, Rust, and Lean. The
JavaScript package also uses the pinned `tree-sitter-rocq` grammar; Rust still
uses the recovery-aware lexical Rocq frontend while an ABI-compatible complete
Rust grammar is integrated. Grammar-backed paths retain the full returned CST,
named fields, child order, exact spans, recovery flags, and source tokens.
Project notation, plugins, name resolution, and elaboration are not supplied by
these context-free grammars and remain required work rather than permanent
exclusions.

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

`LANGUAGE_REPRESENTATION_SCHEMA_VERSION` is `1` in both packages. The public
`language_support`/`languageSupport` APIs describe fidelity, and the Rust and
JavaScript parser registries permit explicit extension without silently
changing built-in dispatch.

## Construct and fidelity inventory

The table reports the common capability available in **both** runtime packages.
“Concrete” means parsed source structure; “opaque” means retained source whose
meaning requires a language project or plugin; “unavailable” means no semantic
claim is made.

| Construct or layer | JavaScript | Rust | Lean | Rocq | Evidence |
| --- | --- | --- | --- | --- | --- |
| Complete UTF-8 source and trivia | preserved | preserved | preserved | preserved | shared conformance corpus and reconstruction tests |
| UTF-8 byte spans and line/column points | concrete | concrete | concrete | concrete | parser implementations and corpus tests |
| Comments, strings, numbers, identifiers, keywords, delimiters | grammar CST | grammar CST | grammar CST | grammar CST in JS; lexical in Rust | real grammar nodes/tokens plus shared corpus |
| Nested grammar hierarchy and recovery | concrete | concrete | concrete | concrete in JS; delimiter recovery in Rust | positive and malformed corpus cases |
| Full grammar-level syntax hierarchy | available | available | available | JS only | tree-sitter CST materialization; Rust Rocq remains open |
| Identifier scope and binding resolution | unavailable | unavailable | unavailable | unavailable | capability report returns `Unavailable` |
| Imports and module references | parsed, unresolved | parsed, unresolved | parsed, unresolved | parsed in JS, lexical in Rust | retained without project resolution |
| Declarations and recursive bodies | parsed, unresolved | parsed, unresolved | parsed, unresolved | parsed in JS, lexical in Rust | retained without semantic lowering |
| Types, effects, universes, and elaboration | unavailable | unavailable | unavailable | unavailable | capability report returns `Unavailable` |
| Attributes, macros, notation, and plugins | opaque | opaque | opaque | opaque | source retained; project expansion is not attempted |
| Proof and tactic syntax | unavailable | unavailable | opaque | opaque | source retained without kernel or tactic interpretation |
| Unknown/control syntax | diagnostic and retained | diagnostic and retained | diagnostic and retained | diagnostic and retained | recovery flags plus exact reconstruction |
| Source generation after mutation | ordered token emission | ordered token emission | ordered token emission | ordered token emission | reconstruction and identifier-edit tests |

The inventory prevents a fallback token stream from being described as
grammar-complete, resolved, or elaborated syntax. Schema revision 1 remains an
intermediate foundation for issue #195, not its full-coverage completion.

## Translation contracts

Both packages expose all 12 directed source/target pairs. At schema revision 1
each pair returns `UnsupportedObligation`; none relabels or passes through text.
Every result names the source and target, the observable input layer, required
target runtime, registered encoding, assumptions, and a precise obligation.

| Target | Required validator/runtime |
| --- | --- |
| JavaScript | ECMAScript 2026 host |
| Rust | Rust 1.98.1, edition 2024 |
| Lean | Lean 4.34.0 kernel and project environment |
| Rocq | Rocq 9.3.0 kernel and project environment |

The current observation is source bytes plus the concrete syntax supplied by
the source runtime; it does not include resolved semantics. No
semantic-preservation proof or non-native encoding is registered, and the
assumption list is empty. This fail-closed result is the only valid outcome
until a pair defines shared semantics, preservation tests, and validation by
the relevant native runtime. Similar spelling never establishes equivalent
effects, types, universes, or proofs.

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
execution semantics, and proof authority. Future meta-language work can add
reusable scope, type, module, proof-syntax, and translation concepts, but must
raise the relevant capability only when common conformance evidence exists.

## Distribution status

The changelog fragment for this work requests the next minor release in the
existing release workflow. Published npm/crates installation evidence can only
be recorded after merge and release. The continuing npm distribution concern
is tracked by [issue #171](https://github.com/link-foundation/meta-language/issues/171);
source-level parity must not be mistaken for published-package parity.
