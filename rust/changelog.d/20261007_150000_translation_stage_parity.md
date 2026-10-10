---
bump: patch
---

### Fixed
- Preserve JavaScript constant-binding and parameter-default metadata in Rust translation stage serialization.
- Decode Unicode escapes and fill omitted JavaScript arguments from parameter defaults in Rust, using frontend decisions translated automatically from JavaScript.
- Install the shared JavaScript report dependencies before Rust tests and fresh-merge checks.
- Use portable module paths and verify the complete reports before recording self-translation reporting evidence.
