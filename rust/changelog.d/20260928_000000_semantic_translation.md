---
bump: minor
---

### Added
- `translate_program` translates portable-core programs with the parse, check and emit pipeline. It produces a native target program that prints the same lines and restates every theorem and assertion. `ProgramTranslation::semantics` records the entry point, the observation procedure, encodings, assumptions, proof obligations and how the target discharges them, source mappings with UTF-16 spans, runtime dependencies, and provenance (source language, SHA-256 and byte length). The contract's support is `TranslationSupport::SemanticTranslation`.
- `read_translation_provenance` reads the provenance comment a semantic translation starts with and rejects artifacts without it.
- Programs outside the portable core keep the portable-encoding envelope and report why in `ProgramTranslation::diagnostic`.
