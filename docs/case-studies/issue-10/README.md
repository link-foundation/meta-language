# Issue 10: Visual Basic Grammar

Issue #10 asked for a Visual Basic grammar candidate evaluation and a
grammar-backed parser path that can still reconstruct source byte-for-byte.

## Candidate Evaluation

### `arborium-vb`

- Crate: <https://crates.io/crates/arborium-vb/2.18.0>
- Repository: <https://github.com/bearcove/arborium>
- License: MIT
- Maintenance signal: current `2.18.0` release from the Arborium grammar set.
- Result: not selected. The crate declares `edition = "2024"` and
  `rust-version = "1.85"`, which would raise this crate's Rust floor beyond the
  tree-sitter binding refresh needed for Visual Basic.

### CodeAnt-AI `tree-sitter-vb-dotnet` Fork

- Repository: <https://github.com/CodeAnt-AI/tree-sitter-vb-dotnet>
- License: no GitHub `licenseInfo` and no root `LICENSE` file was exposed by
  the repository contents API when evaluated on 2026-06-07.
- Maintenance signal: repository metadata showed recent updates, but the
  missing repository license metadata makes direct vendoring or Git dependency
  use unsuitable.
- Result: not selected directly.

### Published `tree-sitter-vb-dotnet`

- Crate: <https://crates.io/crates/tree-sitter-vb-dotnet/0.1.0>
- Repository: <https://github.com/jamie8johnson/tree-sitter-vb-dotnet>
- License: MIT, with an included `LICENSE` file. MIT is a permissive license
  compatible with this repository's Unlicense distribution model.
- Coverage signal: the crate README advertises VB.NET module, class,
  structure, interface, sub/function, property, delegate, event, and common
  control-flow coverage; LINQ/XML literals are marked planned, and error
  recovery is marked basic.
- Result: selected. It exposes the same safe `LanguageFn` API used by current
  tree-sitter grammar crates and satisfies the issue's clean parse and recovery
  fixture requirements.

## Integration

The adapter now recognizes `Visual Basic`, `vb`, `vb.net`, and `vbnet` labels
and loads the `tree-sitter-vb-dotnet` grammar. The existing Visual Basic
language fixture now participates in the grammar-backed fixture test, and a
malformed Visual Basic fixture verifies both explicit error and missing links
while preserving byte-exact reconstruction.
