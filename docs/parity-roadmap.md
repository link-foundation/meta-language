# Parity Implementation

This document keeps the comparison scope explicit and ties each named
competitor or ecosystem project to executable fixtures in this crate.

The rule for this repository is that every imported idea must become links in a
single network. External projects may use their own terminology; adapters must
translate that terminology at the boundary.

## Implemented Tracking Surface

The Rust API exposes these registries:

- `PARITY_TARGETS`: competitor and ecosystem projects whose features and tests
  must be tracked.
- `MARKUP_LANGUAGE_TARGETS`: document-container languages that require full
  grammar support.
- `PROGRAMMING_LANGUAGE_TARGETS`: the initial ten programming-language parser
  targets.
- `NATURAL_LANGUAGE_TARGETS`: the initial ten natural-language parser targets.
- `GRAMMAR_EMBEDDING_TARGETS`: mixed-grammar cases that must parse into one
  unified links network.
- `PARITY_FIXTURES`: executable source fixtures, one or more per parity target,
  that must parse and reconstruct through the public API.
- `LANGUAGE_FIXTURES`: executable source fixtures for every markup,
  programming-language, and natural-language target named by the founding issue.

Unit tests assert that the required projects, language groups, and executable
fixtures stay present. They also assert that every advertised parity capability
is covered by fixtures and that every language target has a lossless
parse/reconstruction fixture.

## Feature Matrix

| Project | Feature areas to match | Executable fixture gate |
|---|---|---|
| tree-sitter | Lossless concrete syntax, recoverable errors, mixed-language regions, query matching | Markdown fenced Rust fixture plus query and recovery tests |
| LibCST | Python lossless parsing, trivia preservation, metadata, same-language reconstruction | Python indentation fixture round-trips through `reconstruct_text()` |
| Recast | JavaScript and TypeScript parse-print preservation | JavaScript comment-preservation fixture round-trips through `reconstruct_text()` |
| jscodeshift | Transform workflows over JavaScript and TypeScript syntax | JavaScript transform source fixture plus `SubstitutionRule` tests |
| Rowan | Persistent concrete syntax representation, immutable snapshots, and trivia preservation | Rust trivia fixture round-trips through `reconstruct_text()` plus snapshot version tests |
| cstree | Rust concrete syntax representation, immutable snapshots, and checkpoint behavior | Rust checkpoint fixture round-trips through `reconstruct_text()` plus snapshot version tests |
| Roslyn | C# syntax, trivia, diagnostics, and formatting | C# diagnostic fixture plus recovery tests |
| links-notation | LiNo doublets, triplets, N-tuples, indentation, and self-reference | LiNo tuple fixture plus self-reference tests |
| link-cli | Single match-and-substitute operation | Create, update, delete, and swap substitution tests |
| lino-objects-codec | Object encode/decode with identity and circular-reference preservation | Shared and circular object fixture plus identity tests |
| relative-meta-logic | Dependent types, many-valued evaluation, probabilistic evaluation, paradox cases | Dependent-type fixture plus `TruthValue` tests |
| formal-ai | Formalization corpus and semantic reconstruction expectations | Formalization source fixture plus concept reconstruction tests |
| meta-expression | Formalize, semantic-link, naturalize, span, and self-reference behavior | Naturalization span fixture plus concept reconstruction tests |

## Executable Fixture Gates

`tests/unit/link_network.rs` enforces that every `PARITY_TARGETS` entry has a
matching `PARITY_FIXTURES` entry. Each fixture is parsed with
`LinkNetwork::parse` and reconstructed with `LinkNetwork::reconstruct_text`; the
expected reconstruction must match exactly. Capability assertions ensure
fixtures only claim capabilities advertised by their target and that every
capability advertised by every target is exercised by at least one fixture for
that target.

The same test file enforces `LANGUAGE_FIXTURES` coverage for every entry in
`MARKUP_LANGUAGE_TARGETS`, `PROGRAMMING_LANGUAGE_TARGETS`, and
`NATURAL_LANGUAGE_TARGETS`. These fixtures include UTF-8 natural-language
samples so lossless reconstruction covers non-ASCII byte ranges.

Additional behavior-specific tests cover:

- recoverable missing-link diagnostics without losing original source text;
- Markdown fenced-code and HTML embedded-region detection in one links network;
- query matching by link type, term, language, and named flag;
- link-cli-style create, update, delete, and swap substitutions;
- concept-to-language reconstruction for English and Spanish syntax;
- immutable snapshots, mutable forks, provenance, and forward version commits;
- object identity and circular-reference representation through shared links;
- many-valued and paradox-compatible truth values.

## Default Parse Contract

`LinkNetwork::parse` is the default parse entry point and is lossless. A parse
must preserve enough data to reconstruct unchanged text byte-for-byte. CST-like,
AST-like, and semantic-only use cases should be projections over the same
network, not separate parse modes.

Current projections:

- `NetworkProjection::Lossless`: every link.
- `NetworkProjection::ConcreteSyntax`: syntax-preserving links, including token
  and trivia links.
- `NetworkProjection::AbstractSyntax`: lower-level token and trivia links
  stripped from the view.
- `NetworkProjection::Semantic`: semantic, concept, type, and language links.

## Language Coverage Targets

Document-container targets:

- txt
- Markdown
- HTML

Programming-language targets use the TIOBE May 2026 top-ten list:

1. Python
2. C
3. Java
4. C++
5. C#
6. JavaScript
7. Visual Basic
8. R
9. SQL family, represented by the `sql-ansi` baseline dialect fixture
10. Delphi/Object Pascal

Source: <https://www.tiobe.com/tiobe-index/>

Natural-language targets use the Ethnologue 2025 total-speaker order for the
Britannica/Ethnologue top-ten set:

1. English
2. Mandarin Chinese
3. Hindi
4. Spanish
5. Modern Standard Arabic
6. French
7. Bengali
8. Portuguese
9. Russian
10. Urdu

Source: <https://www.britannica.com/topic/languages-by-total-number-of-speakers-2228881>

## Mixed Grammar Targets

The mixed-grammar targets are:

- Markdown fenced code regions, detected by language tag and by content.
- Markdown inline or block HTML.
- HTML script elements containing JavaScript.
- HTML style elements and style attributes containing CSS.

All of these must produce one network with source spans and cross-language
links, rather than separate disconnected parse results.

## SQL Dialect Coverage

`sql-ansi` is the first registered SQL-family dialect key. It uses
`tree-sitter-sequel` 0.3.11 as the baseline SQL grammar, published under the
MIT license from <https://github.com/derekstride/tree-sitter-sql.git>.

Coverage currently includes common `SELECT`, DDL, DML, function, trigger, and
window-function syntax from a permissive general SQL grammar. The upstream
grammar references PostgreSQL, MariaDB, and SQLite syntax sources and carries
some dialect-aware productions, but this crate only advertises the adopted
`sql-ansi` baseline until separate dialect grammars such as BigQuery, SQLite,
PostgreSQL, or T-SQL are wired and tested under their own keys.

## Delphi/Object Pascal Coverage

`Delphi/Object Pascal` uses `tree-sitter-pascal` 0.10.2 from
<https://github.com/Isopod/tree-sitter-pascal>. The crate is published under
the MIT license, which is compatible with this repository's Unlicense model.

The adopted fixture parses a Delphi-style unit containing a generic class,
RTTI-style attribute, and property, then reconstructs the source byte-for-byte.
The selected grammar also parsed local probes for inline variable declarations
and constrained generics without recovery errors.

The decision for now is to accept the published generic Pascal grammar rather
than fork. It provides a useful Delphi/Object Pascal syntax baseline, but this
crate does not claim full Delphi compiler coverage: version-specific Delphi and
Free Pascal mode differences, conditional compilation variants, packages,
libraries, resource/design-time files, and semantic checks remain outside the
advertised grammar-backed scope until dedicated fixtures are added.
