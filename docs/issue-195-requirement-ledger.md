# Issue 195 requirement and coverage ledger

This ledger preserves the full delivery target from
[issue #195](https://github.com/link-foundation/meta-language/issues/195) and the
[maintainer clarification on PR #196](https://github.com/link-foundation/meta-language/pull/196#issuecomment-5795832509).
It is an implementation checklist, not a substitute for implementation. A row
is complete only when both runtime packages have executable evidence for the
requested behavior. Documenting an omission, retaining bytes, or returning an
unsupported descriptor does not complete that row.

Statuses mean:

- **Complete** — implemented in Rust and JavaScript with the required shared evidence.
- **Partial** — useful implementation exists, but one or more stated acceptance conditions remain open.
- **Open** — the requested implementation/evidence has not been delivered.

## Authoritative language inventory

[`parity/language-grammar-inventory.json`](../parity/language-grammar-inventory.json)
is the versioned, machine-readable inventory. It currently contains 57
language/format targets, their aliases, runtime backends, a parse fixture, and
separate Rust/JavaScript status. Its status vocabulary deliberately distinguishes
real grammar CSTs from specialized structure, lexical CSTs, and lossless text.

Current dispatch audit:

| Runtime | Real grammar CST | Specialized structure | Lexical CST | Lossless text only |
| --- | ---: | ---: | ---: | ---: |
| JavaScript | 43 | 1 | 0 | 13 |
| Rust | 39 | 14 | 2 | 2 |

Those counts are a progress measurement, not an acceptance threshold. Every
non-grammar row remains in scope. Grammar importers, embedded-language paths,
natural-language representations, document formats, aliases, and SQL dialect
profiles are recorded alongside the ordinary programming-language dispatch.

## Default CST clarification

| Requirement | Status | Current implementation and evidence | Work still required |
| --- | --- | --- | --- |
| One authoritative, versioned inventory reconciled with dispatch, aliases, extensions, dialects, importers, embeddings, and fixtures | **Partial** | Shared inventory exists; both runtime conformance tests consume every row marked `grammar-cst` and exercise every listed alias. | Add file-extension dispatch metadata; automatically compare every parser/target registry against the inventory; complete importer, natural-language, document, and embedded-path fixtures. |
| Ordinary parse selects a real grammar and returns a complete lossless CST for every language | **Partial** | Rust already uses tree-sitter for 39 inventory rows. JavaScript now uses real grammars for 43 rows, retains grammar node kinds, child order, named fields, spans, error/missing/extra flags, tokens, and trivia, and reconstructs source exactly. | Replace every `specialized-structure`, `lexical-cst`, and `lossless-text` row with a complete grammar CST. In particular: Rust Rocq, JSON5, and Markdown; JavaScript PDF, DOCX, natural languages, and plain-text/document structures. Package the JavaScript grammar artifacts so an ordinary first parse does not depend on the language pack's download cache. Validate far beyond one fixture per grammar. |
| Project-aware syntax extensions, notation, macros, dialects, imports, and surface-to-expansion traces | **Open** | Parser registries can replace dispatch explicitly; surface syntax is retained. | Define project/context inputs, load imports/plugins, expand language-defined syntax, retain both surface and expanded forms, and trace mappings in both runtimes. |
| Equivalent JavaScript/Rust default capabilities | **Partial** | A shared inventory and shared four-language/default-CST tests now prevent the previous six reproduced JavaScript fallbacks. | Eliminate all per-runtime status differences and compare normalized full CSTs, fields, flags, trivia, diagnostics, and embeddings. |
| Missing grammar/constructs remain visibly incomplete | **Complete** | Inventory status definitions and this ledger explicitly reject lexical/text fallback as completed CST support. | Keep this invariant as rows are implemented; do not remove targets to improve counts. |

## Four-language representation and transformation

| Requirement | Status | Current implementation and evidence | Work still required |
| --- | --- | --- | --- |
| Full JavaScript syntax CST | **Partial** | Both runtimes use tree-sitter. Shared regressions cover regular-expression literals, template interpolation, invalid syntax, exact reconstruction, and syntax-backed replacement. | Run ECMAScript conformance corpus and representative projects; cover modules, classes, private fields, JSX where applicable, decorators/proposals, recovery, and edition claims. |
| Full Rust syntax CST | **Partial** | Both runtimes use tree-sitter and retain concrete nodes/tokens. | Run Rust grammar/rustc corpora; cover attributes, macros/token trees, editions, modules, types, unsafe/effects, malformed input, and representative projects. |
| Full Lean syntax CST | **Partial** | Both runtimes now use Lean tree-sitter grammars and preserve the grammar `module` below the compatibility `file` root. | Cover the Lean corpus and projects; add project imports, scoped notation, macros, commands, term/tactic syntax, elaboration, universes, and surface/expansion traces. Reconcile the two upstream grammar versions. |
| Full Rocq/Coq syntax CST | **Partial** | JavaScript uses the pinned `tree-sitter-rocq` grammar and retains its `ident` nodes while exposing compatible semantic identifier/type leaves. | Replace the Rust lexical frontend with an ABI-compatible complete Rocq grammar; cover modules, notations, vernacular commands, terms, tactics, plugins, universes, and project context in both runtimes. |
| Imports/modules, scopes/bindings, recursive definitions, types/universes, effects, attributes, macros/notation, proof terms/tactics | **Open** | Grammar CSTs preserve surface nodes where the selected grammar recognizes them. Capability APIs still truthfully report resolution/elaboration as unavailable. | Implement shared semantic layers without collapsing language-specific distinctions, with project-aware resolution and executable coverage for every construct. |
| Query, insert, delete, replace, move, clone, and construction with regenerated valid source independent of the original buffer | **Partial** | Query and token-backed replacement work; emission uses retained token links after the original input string is discarded. | Add structure-aware insert/delete/move/clone/build APIs, recompute tree/spans/mappings/diagnostics, and reparse generated code to the intended structure. |
| Binding-aware rename and capture avoidance | **Open** | Existing tests prove only spelling-level syntax-node replacement that avoids comments/literals. | Implement symbol identity, nested scopes, shadowing, qualified names, Unicode, macro/proof binders, capture avoidance, and semantic-refactoring diagnostics. |
| Reusable shared concepts with explicit language-specific distinctions; RML policy remains downstream | **Partial** | Existing generic link types and capability boundaries avoid claiming equivalent types/effects/proofs. | Define and test reusable binding, typing, module, proof-syntax, provenance, and translation concepts without moving RML foundations or proof authority upstream. |

## Directed translations

All 12 ordered pairs remain required:

| Source | Required targets | Status |
| --- | --- | --- |
| JavaScript | Rust, Lean, Rocq | **Open** |
| Rust | JavaScript, Lean, Rocq | **Open** |
| Lean | JavaScript, Rust, Rocq | **Open** |
| Rocq | JavaScript, Rust, Lean | **Open** |

The current `UnsupportedObligation` contracts correctly prevent source text
from being relabelled, but they are unfinished hooks. Each pair still needs an
actual translator or explicit supported encoding/runtime, observation and
preservation contracts, assumptions, provenance/source mappings, target parse
and native validation, and behavior/type/proof-preservation tests. Similar
keywords or identifiers never establish semantic equivalence.

## Evidence, integration, and release

| Requirement | Status | Current evidence | Work still required |
| --- | --- | --- | --- |
| Shared positive/negative grammar and transformation corpus | **Partial** | Both packages consume the same four-language regression corpus and language inventory; malformed JavaScript and formal-language cases retain source and report errors. | Add upstream conformance suites, representative projects, all four language-specific equivalents, sequences of edits, normalized full-tree comparisons, and semantic/translation expectations. |
| Preserve real node kinds, named fields, order, trivia, spans, errors/missing nodes, and embeddings | **Partial** | JavaScript grammar conversion now preserves those tree-sitter properties and emits field links; Rust tree-sitter adapter already does so. Inventory tests assert nontrivial CSTs and spans. | Assert normalized relationships/fields/flags/trivia for all fixtures and fully integrate embedded regions in JavaScript grammar hosts. |
| Native compiler/prover validation separated from internal authority | **Open** | Runtime requirements are named by the unsupported contracts only. | Add JavaScript hosts, `rustc`, Lean, and Rocq validation jobs and preservation oracles without treating native success as RML proof authority. |
| Published npm/crates packages and downstream RML installation | **Open** | Source packages pass local tests; a freshly packed npm tarball installs with npm 10 and passes JavaScript, Lean, Rocq, and Python grammar smoke tests. | Resolve #171; remove the JavaScript first-use grammar download and git-sourced native-build dependency; make the lock/install path portable across supported npm versions and platforms; publish matching versions; install those exact artifacts in clean environments; and run the four-language/RML integration examples. |
| PR text retains full target and does not close issue early | **Partial** | This ledger preserves the clarified target and the PR remains a draft. | Keep the description synchronized with delivered evidence, remove close directives while any row is open, and mark ready only after every required row is complete. |

## Regression provenance

The three concrete JavaScript failures reproduced in the clarification are now
permanent shared fixtures:

1. `/x/` is parsed as a regular-expression literal, so renaming identifier
   `x` does not rewrite its pattern text.
2. `x` inside `` `${x}` `` is a real expression identifier and is renamed.
3. `const = ;` retains its exact source and carries grammar diagnostics instead
   of being reported as a clean lexical scan.

The six default-path probes (Python, TypeScript, Java, Go, JSON, and HTML) are
also shared exact-node tests. The broader inventory test extends dispatch and
losslessness coverage to every alias currently marked `grammar-cst`.
