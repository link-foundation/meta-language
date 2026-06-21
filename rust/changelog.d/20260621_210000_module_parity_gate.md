---
bump: minor
---

### Added
- Added a module-level Rust/JavaScript parity gate: every `pub mod` in
  `rust/src/lib.rs` must now be classified in `parity/language-features.json`
  (`rustModules`) as either `ported` (naming an implemented feature row) or
  `rust-only` (with a justification). The JavaScript checker
  (`js/scripts/check-js-rust-parity.mjs`) and the Rust test
  (`rust/tests/unit/parity_manifest.rs`) both fail when the public Rust surface
  and the manifest drift apart, so a new Rust module can no longer slip in
  without an explicit JavaScript parity decision.
- Ported four previously Rust-only modules to JavaScript with full test
  coverage and parity manifest rows: read-only access (`ReadOnlyNetwork`,
  `EngineNetwork`, `AccessMode`), embedded-region detection (`EmbeddedRegion`,
  `detectEmbeddedRegions`, `RegionDetectionPolicy`), language profiles
  (`LanguageProfile`, `LanguageProfileLinks`, `LanguageProfileViolation`), and
  the link-rule query algebra (`LinkRule`, `LinkRuleRegistry`,
  `TraversalStrategy`, and the rule snapshot suite).
- Added `LinkMetadata.withDefinition`/`definition` to the JavaScript primitives
  for parity with the Rust `LinkMetadata` definition field, and recorded
  `parse-configuration` and `link-flags` feature rows that already existed in
  both languages but were previously untracked.
