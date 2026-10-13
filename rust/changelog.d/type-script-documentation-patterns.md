### Fixed

- Preserve executable TypeScript parameter patterns containing comments or a
  quoted comment terminator when projecting their annotations into JavaScript.
  Supplemental documentation escapes the terminator while original parameter
  source text remains unchanged.
