---
bump: patch
---

### Added
- Generative fixtures for JavaScript, Lean, Rocq and Rust in `parity/fixtures/issue-195-generative/`, built from the conformance inputs by a seeded PRNG (mulberry32 seeded with FNV-1a of `<seed>:<language>`). Each language has 16 property compositions, 32 fuzz mutations and 8 metamorphic pairs (two leading blank lines, and CRLF line ends), plus 8 four-step edit sequences and the kept reproducers of `reproducers.json`. Each input carries the tree printed by the native tree-sitter 0.27.0 CLI, which is the independent oracle. `js/scripts/generate-issue-195-generative.mjs` regenerates the fixtures, `--seed` picks another seed and `--check` verifies them.
- The JavaScript suite `js/tests/issue-195-generative.test.js` and the Rust suite `rust/tests/unit/issue_195_generative.rs` regenerate the inputs from the recorded seed and compare every public tree with the oracle. Every parse must also keep oracle-free properties: exact reconstruction; spans inside the source, on code point boundaries and with matching points; children inside their parents; and a verification report that is clean exactly when no link is flagged. The Rust suite applies edit sequences through the incremental `apply_edit` and compares each step with a fresh parse. Both suites then fuzz new inputs at run time. `ISSUE_195_GENERATIVE_SEED` and `ISSUE_195_GENERATIVE_CASES` set the run's seed and case count, and a failure prints the seed, source and edits to keep as a reproducer.

### Fixed
- `LinkNetwork::apply_edit` now gives the same network as a fresh parse of the edited text when that text is malformed. Before, tree-sitter's incremental error recovery reused subtrees of the old tree and could settle on another tree. For example, after an edit of `if (x) y;` followed by an `else` clause, an ERROR node could swallow the `else` clause, which a fresh parse and the JavaScript runtime do not produce. An incremental tree with errors is now parsed again from scratch.

### Changed
- The conformance and generative test helpers order the siblings of a Rust network by start and then end byte. After an incremental edit the link ids no longer follow tree order, so a zero-width MISSING node could be listed after a longer sibling that starts at the same byte.
