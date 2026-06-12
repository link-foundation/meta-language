---
bump: minor
---

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.
