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

### Documentation
- Documented the seven wired grammars (crate, version, license, root node) and
  the explicit CSV/JSON5 deferral — both still pin `tree-sitter ~0.20` — in
  `docs/parity-roadmap.md` so the coverage gap is recorded, not silent.
