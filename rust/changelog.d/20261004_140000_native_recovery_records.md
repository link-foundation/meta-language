---
bump: patch
---

### Changed
- Where a language parses with its native grammar, a conformance or generative case whose native recovery differs from the tree-sitter oracle is now recorded in `parity/fixtures/native-recovery.json` (written by `js/scripts/generate-native-recovery.mjs`) with the digests and repair sites of both trees and a category whose justification the file gives. The JS and Rust suites accept such a case only if it matches its record, both trees are malformed, the native tree is lossless and consistent with its diagnostics, and the category follows from the two trees; a record whose case matches the oracle, has a clean oracle or meets no case fails the suites.
