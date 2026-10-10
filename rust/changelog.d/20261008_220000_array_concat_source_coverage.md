---
bump: minor
---

### Added

- Translate homogeneous JavaScript array concatenation through checked array spreads in both runtimes, preserving order, nested arrays and zero-argument copies. Scalar arguments remain unsupported.

### Fixed

- Retain a childless native parser error root as a carried syntax item instead of omitting its source. Restore a digest-verified original prefix followed only by generated whitespace, including sources without a final newline; executable edits prevent restoration. Parser resource limits and acceptance gates are unchanged.
