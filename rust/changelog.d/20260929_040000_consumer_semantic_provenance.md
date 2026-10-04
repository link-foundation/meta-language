---
bump: patch
---

### Fixed

- The clean npm and crate consumers of issue #195 accept a semantic translation by its provenance, whose source language, size and SHA-256 must name the exact source, since `console.log(42);` became a semantic translation to Rust that has no source envelope to decode; `readTranslationProvenance` and `read_translation_provenance` are declared public entry points of the delivery corpus.
