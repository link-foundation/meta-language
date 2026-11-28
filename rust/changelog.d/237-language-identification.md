---
bump: major
---

### Changed

- `identify_language` now returns a report with ranked confidence scores and
  UTF-8 spans for each script run. Read `.language` for the optional verdict or
  use `language_catalog::identify_language_with_threshold` to set a threshold. Unsupported
  Cyrillic text can return an unknown verdict.

### Added

- Exported the matching `identifyLanguage` API from the npm package root,
  including TypeScript declarations.
