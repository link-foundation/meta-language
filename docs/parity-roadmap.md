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
- `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS`: popular programming languages
  immediately below the TIOBE top ten that require full grammar support.
- `NATURAL_LANGUAGE_TARGETS`: the initial ten natural-language parser targets.
- `DATA_FORMAT_TARGETS`: data-exchange / interchange formats that require full
  grammar support.
- `GRAMMAR_EMBEDDING_TARGETS`: mixed-grammar cases that must parse into one
  unified links network.
- `PARITY_FIXTURES`: executable source fixtures, one or more per parity target,
  with upstream path and license provenance, that must parse and reconstruct
  through the public API.
- `LANGUAGE_FIXTURES`: executable source fixtures for every markup,
  programming-language, second-tier-programming-language, natural-language, and
  data-exchange-format target named by the founding issue.
- `NATURAL_LANGUAGE_GRAMMAR_FIXTURES`: grammatical and ungrammatical
  natural-language pass/fail fixtures for the ten target languages, with
  provenance for the repo-authored sentence pairs and the UD-derived
  morphosyntax vocabulary.

Unit tests assert that the required projects, language groups, and executable
fixtures stay present. They also assert that every advertised parity capability
is covered by fixtures and that every language target has a lossless
parse/reconstruction fixture.

## Feature Matrix

| Project | Feature areas to match | Executable fixture gate |
|---|---|---|
| tree-sitter | Lossless concrete syntax, explicit extras/trivia, recoverable errors, mixed-language regions, query matching | Multiple ported corpus, error-corpus, query-doc, and fenced-code fixtures round-trip with provenance |
| LibCST | Python lossless parsing, trivia preservation, parser errors, metadata, query/transform, same-language reconstruction | Multiple ported round-trip, empty-line, parse-error, and transformer-style fixtures |
| Recast | JavaScript and TypeScript parse-print preservation | JavaScript comment-preservation fixture round-trips through `reconstruct_text()` |
| jscodeshift | Transform workflows over JavaScript and TypeScript syntax | JavaScript transform source fixture plus `SubstitutionRule` tests |
| Rowan | Persistent concrete syntax representation, immutable snapshots, and trivia preservation | Rust trivia fixture round-trips through `reconstruct_text()` plus snapshot version tests |
| cstree | Rust concrete syntax representation, immutable snapshots, and checkpoint behavior | Rust checkpoint fixture round-trips through `reconstruct_text()` plus snapshot version tests |
| Roslyn | C# syntax, trivia, diagnostics, query/traversal, transforms, and formatting | Multiple ported `ToFullString`, skipped-token, trivia, and replacement fixtures |
| links-notation | LiNo doublets, triplets, N-tuples, indentation, and self-reference | Ported doublet, triplet, tuple, indented-id, and nested self-reference fixtures structurally parse into relation links; provenance records the verified cross-language test comparison as Python 137, JavaScript 138, Rust 138, and C# 140 |
| link-cli | Single match-and-substitute operation | Ported create, update, delete, and swap fixtures from the `Foundation.Data.Doublets.Cli.Tests` suite plus substitution behavior tests |
| lino-objects-codec | Object encode/decode with identity and circular-reference preservation | Ported primitive round-trip, shared-reference, and circular-reference fixtures plus identity tests |
| relative-meta-logic | Dependent types, many-valued evaluation, probabilistic evaluation, paradox cases | Ported dependent-type, many-valued truth, and probabilistic liar-paradox fixtures plus `TruthValue` and `ProbabilisticTruthValue` tests |
| formal-ai | Formalization corpus and semantic reconstruction expectations | Ported fixtures from actual `data/seed/*.lino` and `data/benchmarks/*.lino` files plus concept reconstruction tests |
| meta-expression | Formalize, semantic-link, naturalize, span, and self-reference behavior | Hawaii naturalization, `1 + 1 = 2`, and liar self-reference fixtures plus the verified 351-concept semantic lexicon seed |
| ast-grep | Rule-test-style structural matching and rewrite assertions | Sampled JavaScript identifier replacement fixture with rule-test provenance |
| Semgrep | Pattern corpus matching and autofix behavior | Sampled Python pattern/autofix fixture with paired `.sgrep`-style provenance |
| Comby | Structural search-and-replace over source text | Sampled JavaScript template rewrite fixture with generic test provenance |
| GritQL | Snippet pattern matching and rewrite effects | Sampled JavaScript rewrite fixture with Grit pattern-test provenance |
| srcML | Source-to-XML markup and lossless reconstruction | Sampled XML source-markup round-trip fixture from `test/parser/testsuite` |
| difftastic | Syntax-aware before/after snapshots for structural diffing | Sampled Rust source snapshot fixture from `sample_files` |
| Babel | JavaScript parser-fixture input/output behavior | Sampled parser fixture with executable JavaScript replacement expectation |
| SWC | TypeScript parser corpus reconstruction | Sampled TypeScript round-trip fixture from parser corpus provenance |
| OpenRewrite | Java recipe before/after rewrites | Sampled Java identifier replacement fixture in `RewriteTest` style |
| Spoon | Java template and pretty-printer transformations | Sampled Java identifier replacement fixture with template-test provenance |
| JavaParser | Lexical-preserving Java source rewrites | Sampled Java identifier replacement fixture with lexical-preservation provenance |
| Rascal | In-language syntax tests and reconstruction | Sampled Rascal-style test declaration round-trip fixture |
| Stratego/Spoofax | SPT embedded-fragment parse/transform expectations | Sampled JavaScript embedded-fragment replacement fixture |
| TXL | By-example source transformation rules | Sampled C source replacement fixture with TXL example provenance |
| MPS | Projectional model serialization and self-description | Sampled XML model round-trip fixture |
| Coccinelle | C input/semantic-patch/result transform triples | Sampled C identifier replacement fixture with `.cocci` triple provenance |
| GF | Grammar parse/linearization-style formalization | Sampled English statehood linearization fixture |
| Universal Dependencies | Natural-language morphosyntax vocabulary alignment | Sampled English fixture tied to UD tag vocabulary provenance |
| LanguageTool | Negative grammar-rule examples and recoverable diagnostics | Sampled ungrammatical English fixture that must verify as recoverable |
| doublets-rs | Binary doublets storage round-trip and snapshot gates | Sampled LiNo storage fixture tied to doublets storage API provenance |

## Executable Fixture Gates

`tests/unit/parity_corpora.rs` enforces that every `PARITY_TARGETS` entry has a
matching `PARITY_FIXTURES` entry. Each fixture is parsed with
`LinkNetwork::parse` and reconstructed with `LinkNetwork::reconstruct_text`; the
expected reconstruction must match exactly. Each fixture records upstream path
and license provenance. Recovery fixtures must expose error, has-error, or
missing-link diagnostics while still reconstructing their original source.
Query/transform fixtures can attach an executable transform expectation that is
run through `LinkQuery` and `ReplacementRule`.

Capability assertions ensure fixtures only claim capabilities advertised by
their target and that every capability advertised by every target is exercised
by at least one fixture for that target. Additional coverage gates require
multiple fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree,
Roslyn, links-notation, link-cli, lino-objects-codec, relative-meta-logic,
formal-ai, and meta-expression so upstream corpora do not collapse back to one
illustrative case.

The ecosystem coverage gate also asserts issue-specific provenance contracts:
links-notation records the verified `TEST_CASE_COMPARISON.md` counts
`137/138/138/140` and parses its fixture surface into relation links, including
named and self-referential links; link-cli fixtures cite the C#
`Foundation.Data.Doublets.Cli.Tests` project; formal-ai fixtures cite actual
`data/seed/` and `data/benchmarks/` files instead of an unverified corpus-size
estimate; and meta-expression continues to seed the verified 351-concept
semantic lexicon.

Wave two adds sampled gates for ast-grep, Semgrep, Comby, GritQL, srcML,
difftastic, Babel, SWC, OpenRewrite, Spoon, JavaParser, Rascal,
Stratego/Spoofax, TXL, MPS, Coccinelle, GF, Universal Dependencies,
LanguageTool, and doublets-rs. These fixtures intentionally port representative
assertion shapes, not whole upstream suites: pattern/autofix rewrites,
before/after recipe transforms, XML/source-markup round trips, syntax-aware
snapshots, projectional model serialization, grammar linearization, UD
vocabulary alignment, recoverable grammar-rule negatives, and doublets storage
round-trip gates.

The remaining "copy all competitor tests" goal is tracked by the open parent
comparison issue [#47](https://github.com/link-foundation/meta-language/issues/47).
Issue [#63](https://github.com/link-foundation/meta-language/issues/63) is the
ratcheted wave-two slice: it raises the executable target surface and records a
coverage floor without claiming that every upstream corpus has been imported.
Known concrete deferrals remain split out where the blocker is narrower: CSV
and JSON5 grammar bindings are tracked by
[#50](https://github.com/link-foundation/meta-language/issues/50), and Perl's
tree-sitter runtime mismatch is tracked by
[#70](https://github.com/link-foundation/meta-language/issues/70). SQL dialect
keys and Delphi-specific compiler coverage are not advertised as implemented;
until they receive dedicated fixtures, they remain under the open #47 parent
scope rather than silent roadmap promises.

The same test file enforces `LANGUAGE_FIXTURES` coverage for every entry in
`MARKUP_LANGUAGE_TARGETS`, `PROGRAMMING_LANGUAGE_TARGETS`,
`SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS`, `NATURAL_LANGUAGE_TARGETS`, and
`DATA_FORMAT_TARGETS`. These fixtures include UTF-8 natural-language samples,
UTF-8 source-string literals in the second-tier programming fixtures, and UTF-8
data-format values so lossless reconstruction covers non-ASCII byte ranges.

`tests/unit/natural_language_grammar.rs` enforces the starter grammaticality gate
for `NATURAL_LANGUAGE_GRAMMAR_FIXTURES`: every target has one grammatical
fixture whose parse verifies cleanly, one registered ungrammatical fixture that
emits a recoverable error link while reconstructing byte-for-byte, and queryable
UD-style `upos:*`, `ufeat:*`, and `deprel:*` morphosyntax links. Unregistered
natural-language text remains lossless and clean during this starter stage, so
existing formalization fixtures are not rejected just because the starter
lexicon is intentionally small. The sentences are repo-authored and no UD
treebank sentence data is imported; UD currently supplies the shared tag
vocabulary. GF/RGL or a native PMCFG reader remains the long-term path for
broad-coverage natural-language grammars.

Additional behavior-specific tests cover:

- recoverable missing-link diagnostics without losing original source text;
- Markdown fenced-code and HTML embedded-region detection in one links network;
- query matching by link type, term, language, and named flag;
- link-cli-style create, update, delete, and swap substitutions;
- concept-to-language reconstruction for English and Spanish syntax;
- immutable snapshots, mutable forks, provenance, and forward version commits;
- object identity and circular-reference representation through shared links;
- many-valued, probabilistic, and paradox-compatible truth values.

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

Second-tier programming-language targets (`SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS`)
cover popular languages immediately below the TIOBE top ten:

1. PHP
2. Swift
3. Kotlin
4. Scala
5. Lua

Perl is part of the same wave but is **deferred**; see
[Second-Tier Programming Language Coverage](#second-tier-programming-language-coverage).

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

## Natural-Language Grammar Coverage

The first grammar-correctness layer is intentionally fixture-gated and
deterministic. Natural-language parses emit a `natural-language:sentence`
syntax link plus UD-style `upos:*`, `ufeat:*`, and `deprel:*` links for known
starter forms. For the accepted fixture pattern, `verify_full_match()` stays
clean. For registered ungrammatical starter fixtures, unknown forms or
sentence-pattern mismatches emit recoverable `natural-language:error:*` syntax
links with `is_error` flags, so `verify_full_match()` answers the staged
grammaticality question while `reconstruct_text()` remains byte-exact.

This is not yet a bundled GF/RGL runtime or imported UD treebank corpus. The
starter registry keeps the ten target languages gated and preserves provenance
while leaving broad-coverage grammar assets to later staged work.

Data-exchange / interchange format targets (`DATA_FORMAT_TARGETS`):

1. JSON
2. YAML
3. TOML
4. XML
5. INI
6. protobuf (Protocol Buffers)
7. GraphQL

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

## Go Coverage

`Go` (also accepted as `go` and `golang`) uses the official `tree-sitter-go`
0.25.0 grammar from <https://github.com/tree-sitter/tree-sitter-go>, published
under the MIT license. The grammar's root rule is `source_file` and its
`LANGUAGE` symbol is wired the same way as the other grammar-backed targets, so
`LinkNetwork::parse(source, "Go", ParseConfiguration::default())` produces real
`LinkType::Syntax` concrete-syntax links and reconstructs the source
byte-for-byte. Go is not part of the curated TIOBE top-ten programming targets,
but the grammar is available for downstream consumers (for example
`link-assistant/formal-ai`) that need a real Go CST/AST.

## Data-Exchange Format Coverage

`DATA_FORMAT_TARGETS` lists the seven data-exchange / interchange formats wired
through `src/tree_sitter_adapter.rs`. Each parses through
`LinkNetwork::parse(source, format, ParseConfiguration::default())`, emits real
`LinkType::Syntax` concrete-syntax links, and reconstructs the source
byte-for-byte. Every target has a UTF-8 `LANGUAGE_FIXTURES` round-trip entry,
and `tests/unit/grammar_parsing.rs` additionally covers case-insensitive label
aliases, recovery diagnostics (a malformed JSON fixture), and a mixed-region
case where a ` ```json ` fence inside Markdown parses into the same links
network as the host document.

| Format | Labels (case-insensitive) | Crate | Version | License | Grammar root |
|---|---|---|---|---|---|
| JSON | `JSON` | [`tree-sitter-json`](https://github.com/tree-sitter/tree-sitter-json) | 0.24.8 | MIT | `document` |
| YAML | `YAML`, `yml` | [`tree-sitter-yaml`](https://github.com/tree-sitter-grammars/tree-sitter-yaml) | 0.7.2 | MIT | `stream` |
| TOML | `TOML` | [`tree-sitter-toml-ng`](https://github.com/tree-sitter-grammars/tree-sitter-toml) | 0.7.0 | MIT | `document` |
| XML | `XML` (`DTD` also wired) | [`tree-sitter-xml`](https://github.com/tree-sitter-grammars/tree-sitter-xml) | 0.7.0 | MIT | `document` |
| INI | `INI` | [`tree-sitter-ini`](https://github.com/justinmk/tree-sitter-ini) | 1.4.0 | Apache-2.0 | `document` |
| Protocol Buffers | `protobuf`, `proto`, `Protocol Buffers` | [`tree-sitter-proto`](https://github.com/coder3101/tree-sitter-proto) | 0.4.0 | MIT | `source_file` |
| GraphQL | `GraphQL`, `gql` | [`tree-sitter-graphql`](https://github.com/joowani/tree-sitter-graphql) | 0.1.0 | MIT | `source_file` |

All seven crates use the modern `tree-sitter-language` ABI binding (they list
`tree-sitter` only as a dev-dependency), so they link cleanly against the
project's `tree-sitter 0.25.8` front end. The Apache-2.0 license on
`tree-sitter-ini` is compatible with this repository's Unlicense model.

### CSV and JSON5: explicit deferral

CSV and JSON5 are **not** wired. Their crates.io grammar bindings still declare a
*normal* dependency on `tree-sitter ~0.20`, which is incompatible with the
project's `tree-sitter 0.25.x` toolchain as published:

- **CSV** — [`tree-sitter-csv`](https://crates.io/crates/tree-sitter-csv) 1.2.0
  pins `tree-sitter ~0.20.10`. The maintained repo at
  `tree-sitter-grammars/tree-sitter-csv` still pins `~0.20.10` on master; no
  fixed release exists. Adopting it requires vendoring the generated `parser.c`
  behind `tree-sitter-language`, or a hand-rolled RFC 4180 lossless lexer.
- **JSON5** — [`tree-sitter-json5`](https://crates.io/crates/tree-sitter-json5)
  0.1.0 pins `tree-sitter ~0.20.0`. Upstream
  [`Joakker/tree-sitter-json5`](https://github.com/Joakker/tree-sitter-json5)
  already targets `tree-sitter = "0.25"` on master and is usable as a git or
  vendored dependency once published.

Both are tracked for a follow-up once compatible bindings are published or
vendored; see issue
[#50](https://github.com/link-foundation/meta-language/issues/50) and
`docs/case-studies/issue-47/formats-storage-apis.md` Part A for the verified
binding-compatibility research.

## Second-Tier Programming Language Coverage

`SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` lists the popular programming
languages immediately below the TIOBE top ten that are wired through
`src/tree_sitter_adapter.rs`. Each parses through
`LinkNetwork::parse(source, language, ParseConfiguration::default())`, emits real
`LinkType::Syntax` concrete-syntax links, and reconstructs the source
byte-for-byte. Every target has a UTF-8 `LANGUAGE_FIXTURES` round-trip entry, and
`tests/unit/grammar_parsing.rs` additionally covers case-insensitive label
aliases and a per-language recovery fixture whose malformed source still
reconstructs while exposing error/missing diagnostics.

| Language | Labels (case-insensitive) | Crate | Version | License | Grammar root |
|---|---|---|---|---|---|
| PHP | `PHP` | [`tree-sitter-php`](https://github.com/tree-sitter/tree-sitter-php) | 0.24.2 | MIT | `program` |
| Swift | `Swift` | [`tree-sitter-swift`](https://github.com/alex-pinkus/tree-sitter-swift) | 0.7.3 | MIT | `source_file` |
| Kotlin | `Kotlin`, `kt` | [`tree-sitter-kotlin-ng`](https://github.com/tree-sitter-grammars/tree-sitter-kotlin) | 1.1.0 | MIT | `source_file` |
| Scala | `Scala` | [`tree-sitter-scala`](https://github.com/tree-sitter/tree-sitter-scala) | 0.25.1 | MIT | `compilation_unit` |
| Lua | `Lua` | [`tree-sitter-lua`](https://github.com/tree-sitter-grammars/tree-sitter-lua) | 0.2.0 | MIT | `chunk` |

All five crates use the modern `tree-sitter-language` ABI binding (they list
`tree-sitter` only as a dev-dependency), so they link cleanly against the
project's `tree-sitter 0.25.x` front end. `tree-sitter-php` is wired through its
`LANGUAGE_PHP` symbol (the full PHP-with-template grammar) rather than the
`LANGUAGE_PHP_ONLY` variant. `tree-sitter-scala` is pinned to 0.25.1 and
`tree-sitter-lua` to 0.2.0 — both generated against a `tree-sitter 0.25`/`0.23`
CLI — so the emitted parser ABI loads under the project's 0.25.x runtime; the
newer 0.26-generated releases of those crates are deferred until the runtime is
upgraded.

### Perl: explicit deferral

Perl is **not** wired. Its only published binding,
[`tree-sitter-perl`](https://crates.io/crates/tree-sitter-perl) 1.1.2, declares a
*normal* dependency on `tree-sitter ^0.26.3`, which would force the whole project
off its `tree-sitter 0.25.x` front end. Unlike the five wired second-tier
grammars — which expose only `tree-sitter-language` as a normal dependency — the
Perl crate couples the parser directly to the newer runtime.

It is tracked for a follow-up once the project upgrades to `tree-sitter 0.26.x`,
the binding is vendored behind `tree-sitter-language`, or an upstream release
lists `tree-sitter` only as a dev-dependency; see issue
[#70](https://github.com/link-foundation/meta-language/issues/70).
