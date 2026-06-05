# Parity Roadmap

This document keeps the comparison scope explicit while the implementation
grows from the initial links-network core into complete parsing,
transformation, and reconstruction support.

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

Unit tests assert that the required projects and language groups stay present.
Future fixture imports should extend those tests instead of replacing the
registry.

## Feature Matrix

| Project | Feature areas to match | Test adoption gate |
|---|---|---|
| tree-sitter | Lossless concrete syntax, recoverable errors, mixed-language regions, query matching | Port representative concrete syntax, injection, query, and recovery fixtures |
| LibCST | Python lossless parsing, trivia preservation, metadata, same-language reconstruction | Port Python parse, metadata, transform, and round-trip fixtures |
| Recast | JavaScript and TypeScript parse-print preservation | Port parse-print preservation fixtures |
| jscodeshift | Transform workflows over JavaScript and TypeScript syntax | Port transform fixtures as substitution-rule parity cases |
| Rowan | Persistent concrete syntax representation and trivia preservation | Port green/red syntax and trivia preservation fixtures as links-network cases |
| cstree | Rust concrete syntax representation and checkpoint behavior | Port Rust concrete syntax and checkpoint fixtures |
| Roslyn | C# syntax, trivia, diagnostics, and formatting | Port C# syntax, trivia, diagnostic, and formatter fixtures |
| links-notation | LiNo doublets, triplets, N-tuples, indentation, and self-reference | Port LiNo parsing and formatting fixtures |
| link-cli | Single match-and-substitute operation | Port create, update, delete, swap, trigger, and dedup substitution fixtures |
| lino-objects-codec | Object encode/decode with identity and circular-reference preservation | Port encode/decode and identity fixtures |
| relative-meta-logic | Dependent types, many-valued evaluation, probabilistic evaluation, paradox cases | Port dependent-type, many-valued, probabilistic, and paradox fixtures |
| formal-ai | Formalization corpus and semantic reconstruction expectations | Replay the formal-ai corpus as a parity gate |
| meta-expression | Formalize, semantic-link, naturalize, span, and self-reference behavior | Port formalize, naturalize, span, and self-reference fixtures |

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
