---
bump: minor
---

### Changed
- Parse plain text and natural-language text through versioned built-in grammar CSTs (`text_document` of `line` nodes, `natural_language_document` of `sentence` nodes, with `word`, `punctuation`, whitespace extras and `ERROR` nodes for control characters) in both packages.
- Record Grammar provenance for the built-in LiNo (links-notation 0.13.0), PDF COS, plain-text and natural-language grammars; the language catalog lists them with the SHA-256 of their specifications in `parity/grammars/`.
