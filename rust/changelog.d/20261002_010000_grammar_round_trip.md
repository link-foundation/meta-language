---
bump: minor
---

### Added
- Check a grammar importer and emitter pair with `check_grammar_round_trip` (JavaScript `checkGrammarRoundTrip`). The guard adds a marker alternative to the start rule before export and requires the marker in the exported text and in the re-imported grammar. It also checks independent accept and reject samples, so a pair that re-emits a saved source, or two halves that are wrong in ways that cancel out, are reported as broken. Malformed sources fail with the importer's error. `parity/fixtures/grammar-importers.json` now lists malformed sources for every shared format.
