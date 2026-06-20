---
bump: minor
---

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.
