---
bump: patch
---

### Added
- Generate native scanner operations from reusable delimiter and string-content
  descriptors, with shared JavaScript and Rust concrete-tree fixtures; use the
  content family in the shipped native Rust string scanner.

### Fixed
- Preserve constant-binding metadata and the nullable parameter-default field
  in the Rust translation stage representation.
- Read Unicode string escapes and fill omitted trailing JavaScript parameter
  defaults in the Rust translation pipeline.
