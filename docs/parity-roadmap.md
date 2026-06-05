# Parity Roadmap

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

Unit tests assert that the required projects, language groups, and executable
fixtures stay present. Additional upstream fixture imports should extend those
tests instead of replacing the registry.

## Feature Matrix

| Project | Feature areas to match | Executable fixture gate |
|---|---|---|
| tree-sitter | Lossless concrete syntax, recoverable errors, mixed-language regions, query matching | Markdown fenced Rust fixture plus query and recovery tests |
| LibCST | Python lossless parsing, trivia preservation, metadata, same-language reconstruction | Python indentation fixture round-trips through `reconstruct_text()` |
| Recast | JavaScript and TypeScript parse-print preservation | JavaScript comment-preservation fixture round-trips through `reconstruct_text()` |
| jscodeshift | Transform workflows over JavaScript and TypeScript syntax | JavaScript transform source fixture plus `SubstitutionRule` tests |
| Rowan | Persistent concrete syntax representation and trivia preservation | Rust trivia fixture round-trips through `reconstruct_text()` |
| cstree | Rust concrete syntax representation and checkpoint behavior | Rust checkpoint fixture round-trips through `reconstruct_text()` |
| Roslyn | C# syntax, trivia, diagnostics, and formatting | C# diagnostic fixture plus recovery tests |
| links-notation | LiNo doublets, triplets, N-tuples, indentation, and self-reference | LiNo tuple fixture plus self-reference tests |
| link-cli | Single match-and-substitute operation | Create, update, delete, and swap substitution tests |
| lino-objects-codec | Object encode/decode with identity and circular-reference preservation | Shared and circular object fixture plus identity tests |
| relative-meta-logic | Dependent types, many-valued evaluation, probabilistic evaluation, paradox cases | Dependent-type fixture plus `TruthValue` tests |
| formal-ai | Formalization corpus and semantic reconstruction expectations | Formalization source fixture plus concept reconstruction tests |
| meta-expression | Formalize, semantic-link, naturalize, span, and self-reference behavior | Naturalization span fixture plus concept reconstruction tests |

## Executable Fixture Gate

`tests/unit/link_network.rs` enforces that every `PARITY_TARGETS` entry has a
matching `PARITY_FIXTURES` entry. Each fixture is parsed with `LinkNetwork::parse`
and reconstructed with `LinkNetwork::reconstruct_text`; the expected
reconstruction must match exactly. Capability assertions ensure fixtures only
claim capabilities advertised by their target.

Additional behavior-specific tests cover:

- recoverable missing-link diagnostics without losing original source text;
- Markdown fenced-code and HTML embedded-region detection in one links network;
- query matching by link type, term, language, and named flag;
- link-cli-style create, update, delete, and swap substitutions;
- concept-to-language reconstruction for English and Spanish syntax;
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

Full document-container targets:

- Markdown
- HTML

Initial programming-language targets use the TIOBE May 2026 top-ten list:

1. Python
2. C
3. Java
4. C++
5. C#
6. JavaScript
7. Visual Basic
8. R
9. SQL
10. Delphi/Object Pascal

Source: <https://www.tiobe.com/tiobe-index/>

Initial natural-language targets use the Britannica/Ethnologue total-speaker
top-ten list:

1. English
2. Mandarin Chinese
3. Hindi
4. Spanish
5. French
6. Modern Standard Arabic
7. Bengali
8. Russian
9. Portuguese
10. Urdu

Source: <https://www.britannica.com/topic/languages-by-total-number-of-speakers-2228881>

## Mixed Grammar Targets

The first mixed-grammar targets are:

- Markdown fenced code regions, detected by language tag and later by content.
- Markdown inline or block HTML.
- HTML script elements containing JavaScript.
- HTML style elements and style attributes containing CSS.

All of these must produce one network with source spans and cross-language
links, rather than separate disconnected parse results.
