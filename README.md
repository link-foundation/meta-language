# meta-language

A Rust foundation for a universal, self-describing meta language backed by a
links network. The initial crate focuses on the common structural substrate:
links, references, source spans, parse status metadata, configurable trivia
attachment, self-description roots, and verification that a parsed region is
clean.

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
- `projected_links()` for viewing the same lossless network as concrete syntax,
  abstract syntax, or semantic-only data by stripping lower-level preservation
  links from the view.
- `NetworkSnapshot` and `MutableNetworkSnapshot` for immutable versioned
  snapshots, editable forks, provenance, and forward commits.
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
- Concept-to-language syntax mappings for cross-language reconstruction.
- `reconstruct_text_as()` for semantic cross-language reconstruction and
  configurable formalization levels.
- `seed_common_concept_ontology()` for importing meta-expression's 351-concept
  semantic lexicon as shared concept links, plus structural programming-language
  concepts such as function, binding, application, sequence, branch, and loop.
- Object-identity links and many-valued `TruthValue` semantics.
- A testable parity registry and upstream-provenanced `PARITY_FIXTURES` for
  executable competitor and ecosystem feature gates.
- `LANGUAGE_FIXTURES` with lossless parse/reconstruction samples for every
  required markup, programming-language, and natural-language target.
- Coverage targets for full `txt`, Markdown, and HTML support, mixed grammar
  embedding, ten programming-language parser targets, and ten natural-language
  parser targets.
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

## CLI

```bash
cargo run -- describe
cargo run -- verify --language plain-text --text "alpha beta"
```

`describe` prints the built-in self-description network as LiNo-style definition
lines that round-trip through `parse()` and `reconstruct_text()`. `verify` parses
the text with the lossless text boundary and exits successfully when the
resulting region has no error or missing links.

## Parity Implementation

The crate exposes `PARITY_TARGETS`, `MARKUP_LANGUAGE_TARGETS`,
`PROGRAMMING_LANGUAGE_TARGETS`, `NATURAL_LANGUAGE_TARGETS`, and
`GRAMMAR_EMBEDDING_TARGETS` so comparison scope is part of the tested Rust API.
It also exposes `PARITY_FIXTURES`, with executable, provenance-tracked fixtures
covering every advertised target capability, and `LANGUAGE_FIXTURES`, with a
lossless fixture for every requested language target.
The current registry tracks tree-sitter, LibCST, Recast, jscodeshift, Rowan,
cstree, Roslyn, links-notation, link-cli, lino-objects-codec,
relative-meta-logic, formal-ai, and meta-expression.

See [docs/parity-roadmap.md](docs/parity-roadmap.md) for the feature matrix,
executable fixture gates, and language coverage targets.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-features
rust-script scripts/check-no-src-tests.rs
```

This repository uses changelog fragments in `changelog.d/`; code changes should
include a fragment with the intended semantic-version bump.
