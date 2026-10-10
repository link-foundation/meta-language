---
bump: patch
---

### Fixed
- Keep JavaScript regular expression character classes, quoted bodies and escaped delimiters in one token, preserving the regular expression unsupported diagnostic.
- Distinguish division from regular expression literals using shared frontend decisions translated into Rust.
- Decode NUL and common control escapes in template text and reject legacy octal template escapes.
- Refresh the JavaScript export parity manifest after merging the release branch.
- Preserve Dart type identifiers and nullable suffix alias reductions in generated native grammars, and refresh the Nix corpus fixture.
- Treat doubled punctuation at character-class edges as literal members rather than binary set operators, retaining PowerShell operator grammar generation.
