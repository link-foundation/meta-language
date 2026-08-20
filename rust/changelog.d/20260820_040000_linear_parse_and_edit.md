---
bump: patch
---

### Fixed
- Resolve source byte offsets to `(row, column)` points through a line index
  built once per parse instead of rescanning the source for every node and
  token, which made parsing quadratic in file size: a 38 KB Rust module went
  from 27.6 s to 0.28 s, producing the same 44 142 links.
- Match the links of an incremental `apply_edit` against the previous network
  through an index instead of searching every old link for every reparsed one,
  which was quadratic in network size: an edit to an 8 KB Rust module went from
  2.31 s to 27 ms.

### Added
- Add a complexity guard that measures parsing and incremental edits at two
  input sizes and fails when the cost per byte grows, so neither path can fall
  back to quadratic unnoticed.
