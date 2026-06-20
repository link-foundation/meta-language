# meta-language

A Rust foundation for a universal, self-describing meta language backed by a
links network. The initial crate focuses on the common structural substrate:
links, references, source spans, parse status metadata, configurable trivia
attachment, self-description roots, and verification that a parsed region is
clean.

**Website:** <https://link-foundation.github.io/meta-language> — project
description, an interactive WebAssembly demo, and the full
[Rust API documentation](https://link-foundation.github.io/meta-language/api/).

## What Is Implemented

- A mutable `LinkNetwork` where every item is a link.
- Self-referential point links, so a point is represented without introducing a
  separate primitive.
- Relation links as ordinary links with ordered references to other links.
- Field labels as explicit links instead of side-table metadata.
- Source metadata: link type, named/anonymous flag, byte range, row/column
  points, and `is_error`, `has_error`, `is_missing`, `is_extra` flags.
- `verify_full_match()` for reporting error and missing links in a selected
  source region.
- `parse()` as the default lossless parse entry point; the explicit
  `parse_lossless_text()` boundary remains available.
- `reconstruct_text()` for byte-for-byte reconstruction from non-missing token
  links ordered by source span.
- `insert_source_token()`, `insert_syntax_node()`, and `render_source()` for
  emitting target-language source from programmatically constructed syntax
  networks whose token leaves do not come from a prior parse.
- `projected_links()` for viewing the same lossless network as concrete syntax,
  abstract syntax, or semantic-only data by stripping lower-level preservation
  links from the view.
- `NetworkSnapshot` and `MutableNetworkSnapshot` for immutable versioned
  snapshots, editable forks, provenance, and forward commits.
- `AccessMode` for a read-only or mutable engine per user configuration:
  `freeze()` / `as_read_only()` yield a `ReadOnlyNetwork` view whose mutators
  are unreachable at compile time, and `parse_engine()` returns an
  `EngineNetwork` that rejects mutation with a clear diagnostic under
  `AccessMode::ReadOnly`; the frozen form reuses snapshot `Arc` sharing.
- `LinkStore` and `EngineLinkStore` for storage-backed create/read/update/delete
  and search operations: reads take `&self`, writes take `&mut self`, the
  default store is the in-memory `LinkNetwork`, and read-only access mode
  rejects writes through the same storage boundary.
- Optional `doublets` feature support for a file-mapped `DoubletsLinkStore`
  using `doublets-rs` 0.4, with lossless network round trips and a documented
  `doublets-web` backend label for browser/WASM exchange of the same binary
  graph layout.
- `ParseConfiguration` with containment-link, token-link, or combined trivia
  attachment policies.
- Mixed-region links for Markdown fenced code and HTML regions, plus HTML
  script, style, and style-attribute regions, with `txt` fallback for prose
  regions that content sniffing cannot classify.
- `LinkQuery` for structural matching by link type, term, language, named flag,
  tree-sitter-query-like S-expressions, captures, and host predicates.
- `find()` / `replace()` for codemod-style query transforms over captured links
  while preserving unchanged source bytes.
- `SubstitutionRule` / `apply_substitution()` for the link-cli-style
  match-and-substitute operation.
- `apply_edit()` for incremental source reparsing with stable outside-edit link
  ids, snapshot fork sharing for unchanged links, and structural diff sets for
  changed, added, and removed links.
- Concept-to-language syntax mappings for cross-language reconstruction.
- `reconstruct_text_as()` for semantic cross-language reconstruction and
  configurable formalization levels.
- Exact-match concept interning with language-bound expression links,
  queryable external-id aliases, LiNo concept-set import, and
  `seed_common_concept_ontology()` for the default 351-concept semantic
  lexicon plus structural programming-language concepts.
- Object-identity links, many-valued `TruthValue` semantics, and fixed-point
  `ProbabilisticTruthValue` confidence semantics.
- A testable parity registry and upstream-provenanced `PARITY_FIXTURES` for
  executable competitor and ecosystem feature gates.
- Structural LiNo parsing for links-notation doublets, triplets, named links,
  simple indented definitions, and self-references.
- `LANGUAGE_FIXTURES` with lossless parse/reconstruction samples for every
  required markup, programming-language, and natural-language target.
- `NATURAL_LANGUAGE_GRAMMAR_FIXTURES` with pass/fail grammaticality fixtures
  for the ten natural-language targets, including provenance for the
  repo-authored sentences and UD-derived tag vocabulary.
- Coverage targets for full `txt`, Markdown, HTML, PDF, and DOCX support, mixed
  grammar embedding, ten programming-language parser targets, and ten
  natural-language parser targets.
- A documented text PDF profile (issue #84): `render_pdf_document()` /
  `parse_pdf_document()` map a language-free formatting document onto a valid,
  uncompressed single-page PDF (marked content for heading/paragraph/list, font
  resources for bold/italic), `parse("…", "pdf", …)` builds a byte-exact lossless
  network with additive concept-tagged structure links, and
  `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through
  the shared concept layer. See `docs/pdf-fidelity.md` for the round-trip
  fidelity matrix.
- A documented DOCX (OOXML) profile (issue #85): `render_docx_document()` /
  `parse_docx_document()` map a language-free formatting document onto
  WordprocessingML `word/document.xml` (`<w:pStyle>` headings, `<w:numPr>` lists,
  `<w:b/>`/`<w:i/>` bold/italic runs), a binary OPC layer
  (`render_docx_package()` / `parse_docx_package()`) assembles a valid `.docx`
  ZIP with no new dependencies, `parse("…", "docx", …)` builds a byte-exact
  lossless network with additive concept-tagged structure links, and
  `reconstruct_text_as("DOCX", …)` renders a structurally equivalent DOCX through
  the shared concept layer. See `docs/docx-fidelity.md` for the round-trip
  fidelity matrix.
- Cross-format document reconstruction (issue #86): `reconstruct_text_as("txt" |
  "Markdown" | "HTML" | "PDF" | "DOCX", …)` reconstructs a document parsed from
  any supported format into any other over the shared concept layer (same-format
  targets stay byte-exact), with `txt` added as a first-class format and the lossy
  fallback floor. Per-format capability profiles (`document_format_profile`,
  `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`) report, for every concept, either
  native support or a documented `LanguageProfile` fallback. See
  `docs/cross-format-fidelity.md` for the cross-format entry point and the
  per-format fidelity matrix.
- Self-description roots for `link`, `reference`, `relation link`, `language`,
  `grammar`, `type`, `Type`, `concept`, `point`, `field`, `trivia`, `region`,
  and `object`.
- A lossless text parser boundary that preserves tokens, trivia, recovery
  markers, and mixed-region metadata behind the same representation.

## Usage

```rust
use meta_language::{LinkNetwork, ParseConfiguration};

let network = LinkNetwork::parse("alpha beta", "plain-text", ParseConfiguration::default());
let report = network.verify_full_match(None);

assert!(report.is_clean());
assert_eq!(network.reconstruct_text(), "alpha beta");
```

The repository also includes a native ESM JavaScript package in `js/`:

```bash
cd js
npm ci
npm test
```

```js
import { LinkNetwork, ParseConfiguration } from '@link-foundation/meta-language';

const network = LinkNetwork.parse('alpha beta', 'txt', ParseConfiguration.default());

console.log(network.reconstructText());
```

The JavaScript package is checked against the Rust feature surface through
`parity/language-features.json` and `npm run check:parity`.

The default parse path is lossless. Callers that need a narrower view can use a
projection without mutating the original network:

```rust
use meta_language::{LinkNetwork, NetworkProjection, ParseConfiguration};

let network = LinkNetwork::parse("alpha beta", "plain-text", ParseConfiguration::default());
let abstract_links = network
    .projected_links(NetworkProjection::AbstractSyntax)
    .count();

assert!(abstract_links < network.len());
```

Construct source directly as a syntax network when code should be generated
before validation:

```rust
use meta_language::{LinkNetwork, ParseConfiguration};

let mut network = LinkNetwork::new();
let tokens = [
    network.insert_source_token("JavaScript", "const answer = "),
    network.insert_source_token("JavaScript", "42"),
    network.insert_source_token("JavaScript", ";\n"),
];
let declaration = network.insert_syntax_node("JavaScript", "lexical_declaration", tokens);
network.insert_syntax_node("JavaScript", "program", [declaration]);

let source = network.render_source("JavaScript");
assert_eq!(source, "const answer = 42;\n");
assert!(LinkNetwork::parse(&source, "JavaScript", ParseConfiguration::default())
    .verify_full_match(None)
    .is_clean());
```

Configure the engine read-only when a parsed network must never be mutated. The
frozen view exposes every read operation but no mutators (calling one is a
compile error), and the `EngineNetwork` boundary rejects mutation at runtime:

```rust
use meta_language::{AccessMode, LinkNetwork, ParseConfiguration};

let configuration = ParseConfiguration::default().with_access_mode(AccessMode::ReadOnly);
let mut engine = LinkNetwork::parse_engine("alpha beta", "plain-text", configuration);

assert!(engine.is_read_only());
assert_eq!(engine.reconstruct_text(), "alpha beta");
assert!(engine.as_mutable().is_err()); // read-only engine rejects mutation
```

Use the storage trait directly when links need to move between in-memory and
binary stores. The optional doublets backend is enabled with
`--features doublets`:

```rust
use meta_language::{
    DoubletsLinkStore, LinkNetwork, LinkStore, LinkStoreQuery, ParseConfiguration,
};

let network = LinkNetwork::parse("const answer = 42;\n", "JavaScript", ParseConfiguration::default());
let mut store = DoubletsLinkStore::create_file("network.doublets").expect("create doublets store");
store.replace_with_network(&network).expect("write network");

let restored = DoubletsLinkStore::open_file("network.doublets")
    .expect("open doublets store")
    .to_network()
    .expect("read network");
assert_eq!(restored.to_lino(), network.to_lino());

let links = LinkStore::search(&restored, &LinkStoreQuery::new()).expect("search links");
assert_eq!(links.len(), restored.len());
```

`LinkStoreBackend::DoubletsWeb` names the WASM/browser exchange target for this
binary graph representation; native code uses `DoubletsLinkStore`, while a
browser host can map the same logical records through `doublets-web`.

Codemod-style transforms can select links with an S-expression query and replace
only captured source ranges:

```rust
use meta_language::{LinkNetwork, LinkQuery, ParseConfiguration, ReplacementRule};

let mut network = LinkNetwork::parse(
    "const oldName = call(oldName);\n",
    "JavaScript",
    ParseConfiguration::default(),
);
let query = LinkQuery::from_sexpression(
    r#"
    (identifier) @target
    (#eq? @target "oldName")
    "#,
)
.expect("query parses");
let captures = network.find(&query);

network.replace(
    &captures,
    &ReplacementRule::captured_text("target", "newName"),
);

assert_eq!(network.reconstruct_text(), "const newName = call(newName);\n");
```

Cross-language reconstruction can naturalize a parsed semantic proposition into
another target language, or expose progressively more formal representations:

```rust
use meta_language::{FormalizationLevel, LinkNetwork, ParseConfiguration};

let network = LinkNetwork::parse(
    "Hawaii is a state.\n",
    "English",
    ParseConfiguration::default(),
);

assert_eq!(
    network.reconstruct_text_as("Russian", ParseConfiguration::default()),
    "Гавайи это штат.\n"
);
assert_eq!(
    network.reconstruct_text_as(
        "Russian",
        ParseConfiguration::default().with_formalization_level(FormalizationLevel::Concept),
    ),
    "statehood(Q782, Q35657)\n"
);
```

The same `reconstruct_text_as` entry point translates a document parsed from one
markup/document format into another over the shared formatting concept layer.
Same-format targets stay byte-exact; cross-format targets carry the document's
heading/paragraph/list and bold/italic/link structure, degrading through the
documented per-format fallbacks (see `docs/cross-format-fidelity.md`):

```rust
use meta_language::{LinkNetwork, ParseConfiguration};

let network = LinkNetwork::parse(
    "# Status Report\n\nThe system is **ready** for *launch*.",
    "Markdown",
    ParseConfiguration::default(),
);

let html = network.reconstruct_text_as("HTML", ParseConfiguration::default());
assert!(html.contains("<h1>Status Report</h1>"));
assert!(html.contains("<strong>ready</strong>"));

// txt is the lossy fallback floor: prose survives, markup is dropped.
let txt = network.reconstruct_text_as("txt", ParseConfiguration::default());
assert!(txt.contains("The system is ready for launch"));
assert!(!txt.contains('#'));
```

## CLI

```bash
cargo run -- describe
cargo run -- verify --language plain-text --text "alpha beta"
cargo run -- infer examples/json-samples/ --format bnf --metrics
cargo run -- import-grammar --format bnf tests/fixtures/grammar/bnf/arithmetic.bnf
cargo run -- emit-grammar --format gbnf grammar.lino --out grammar.gbnf
cargo run -- translate-grammar grammar.lino --from-language en --to-language ru
```

`describe` prints the built-in self-description network as LiNo-style definition
lines that round-trip through `parse()` and `reconstruct_text()`. `verify` parses
the text with the lossless text boundary and exits successfully when the
resulting region has no error or missing links.

The grammar subcommands expose the grammar IR pipeline from the shell. `infer`
reads one or more example files or directories, infers a grammar, and writes
LiNo by default; `--format bnf|ebnf|abnf|peg|gbnf|tree-sitter` renders another
supported notation, `--metrics` writes precision/recall/F1/runtime to stderr,
and `--out <path>` writes the grammar to a file. `import-grammar` reads
`bnf|ebnf|abnf|peg|antlr|lark|gbnf|tree-sitter` input and renders LiNo by
default or another `--to` notation. `emit-grammar` reads LiNo or native grammar
surface input and emits `bnf|ebnf|abnf|peg|gbnf|tree-sitter`. Unsupported target
formats report a clear non-zero CLI error. `translate-grammar` rewrites
concept-aligned rule names and documentation to the requested target language
while preserving grammar structure.

## Grammar subsystem

The grammar layer stores authored, imported, inferred, translated, and generated
grammars as first-class links instead of side data. Its IR is built from
`Grammar`, `GrammarRule`, `GrammarExpr`, `RuleKind`, and `GrammarFormat`, lowers
through `ToLinks` / `FromLinks` as `LinkType::Grammar`, and is documented in
[docs/grammar](docs/grammar/README.md).

Start with the [architecture overview](docs/grammar/architecture.md) for the IR,
links encoding, and pipeline. The stage guides cover
[authoring](docs/grammar/authoring.md),
[import and export](docs/grammar/import-export.md),
[code generation](docs/grammar/codegen.md),
[inference](docs/grammar/inference.md),
[translation](docs/grammar/translation.md), and
[CLI/runtime integration](docs/grammar/cli-and-runtime.md).

## Parity Implementation

The crate exposes `PARITY_TARGETS`, `MARKUP_LANGUAGE_TARGETS`,
`PROGRAMMING_LANGUAGE_TARGETS`, `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS`,
`NATURAL_LANGUAGE_TARGETS`, `DATA_FORMAT_TARGETS`, and
`GRAMMAR_EMBEDDING_TARGETS` so comparison scope is part of the tested Rust API.
It also exposes `PARITY_FIXTURES`, with executable, provenance-tracked fixtures
covering every advertised target capability, and `LANGUAGE_FIXTURES`, with a
lossless fixture for every requested language target.
Grammar notation round-trip fidelity is tracked in
[docs/grammar/fidelity.md](docs/grammar/fidelity.md) through the
`grammar_format_profile` API.
The current registry tracks tree-sitter, LibCST, Recast, jscodeshift, Rowan,
cstree, Roslyn, links-notation, link-cli, lino-objects-codec,
relative-meta-logic, formal-ai, and meta-expression.
Internal ecosystem fixtures now structurally parse links-notation doublet,
triplet, tuple, indented, and self-reference cases; link-cli create/update/delete/swap
substitutions; lino object round-trip, shared-reference, and circular-reference
cases; relative-meta-logic dependent, many-valued, probabilistic, and
liar-paradox cases;
formal-ai seed and benchmark `.lino` corpora; and meta-expression formalize and
naturalize examples backed by the verified 351-concept lexicon.

See [docs/parity-roadmap.md](docs/parity-roadmap.md) for the feature matrix,
executable fixture gates, and language coverage targets.

## Website

The project website at <https://link-foundation.github.io/meta-language> is
built and deployed by the `Deploy Website` job in
[`.github/workflows/release.yml`](.github/workflows/release.yml). It is assembled
by [`scripts/build-site.rs`](scripts/build-site.rs) into `_site/`:

| Path | Contents |
| --- | --- |
| `/` | Landing page (description, interactive demo, docs links) — [`docs/site/`](docs/site) |
| `/demo/pkg/` | WebAssembly demo built from the [`web/`](web) crate (wraps `links-notation`) |
| `/api/` | `rustdoc` output, with a root redirect so `/api/` does not 404 |

Build and preview the site locally:

```bash
# Requires the wasm32-unknown-unknown target and wasm-pack:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-pack
rust-script scripts/build-site.rs
python3 -m http.server -d _site 8000   # open http://localhost:8000
```

Deploying `cargo doc` output directly used to make the Pages root URL return
HTTP 404 because `cargo doc` writes `target/doc/<crate>/index.html` but no root
`index.html` (see [issue #90](https://github.com/link-foundation/meta-language/issues/90)
and [docs/case-studies/issue-90](docs/case-studies/issue-90)). The assembled
site puts the landing page at the root and keeps the API docs under `/api/`.

**One-time setup:** in the repository's *Settings → Pages*, set *Source* to
*GitHub Actions*. Without this, the first deploy fails on `actions/deploy-pages`
with "Get Pages site failed". This cannot be configured from a workflow.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-features
rust-script scripts/check-no-src-tests.rs
```

The WebAssembly demo crate in [`web/`](web) is standalone (not part of the
published crate). Check it with:

```bash
cd web && cargo test && cargo clippy --all-targets
```

This repository uses changelog fragments in `changelog.d/`; code changes should
include a fragment with the intended semantic-version bump.
