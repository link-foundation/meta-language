---
bump: patch
---

### Added

- The merge quality evidence of the native grammars
  (`docs/grammar/merge-quality-evidence.md`): every native grammar against the
  pinned tree-sitter grammar it was merged from, on that grammar's upstream
  corpus, with coverage, preserved features, correctness, recovery, shared
  reuse, and the time and memory of the native executor and the oracle in both
  runtimes. `rust/tests/merge_quality.rs` measures the Rust executor with a
  counting allocator.
