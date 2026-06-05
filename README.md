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
- `ParseConfiguration` with containment-link, token-link, or combined trivia
  attachment policies.
- Self-description roots for `link`, `reference`, `relation link`, `language`,
  `grammar`, `type`, `concept`, and `point`.
- A minimal lossless text parser boundary that preserves tokens and trivia while
  language-specific parsers are added behind the same representation.

## Usage

```rust
use meta_language::{LinkNetwork, ParseConfiguration};

let network =
    LinkNetwork::parse_lossless_text("alpha beta", "plain-text", ParseConfiguration::default());
let report = network.verify_full_match(None);

assert!(report.is_clean());
```

## CLI

```bash
cargo run -- describe
cargo run -- verify --language plain-text --text "alpha beta"
```

`describe` prints the built-in self-description roots. `verify` parses the text
with the lossless text boundary and exits successfully when the resulting region
has no error or missing links.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-features
```

This repository uses changelog fragments in `changelog.d/`; code changes should
include a fragment with the intended semantic-version bump.
