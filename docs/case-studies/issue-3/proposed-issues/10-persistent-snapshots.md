---
title: "Persistent immutable snapshots (green/red model; optional doublets substrate)"
labels: enhancement
---

## Context

`src/snapshots.rs` provides `NetworkSnapshot` + `MutableNetworkSnapshot` with
provenance and forward version commits, but without persistent structural sharing.
Issue #1 §3.5 requires immutable snapshots that are "persistent, structurally
shared." The rowan/cstree green/red model is the reference. See
[`rust-libraries-survey.md`](../rust-libraries-survey.md) §B and
[`solution-plans.md`](../solution-plans.md) Solution 6.

## Scope

- Implement structural sharing for `NetworkSnapshot` along the green/red model
  (deduplicated, position-independent green links; lazily-resolved absolute
  offsets).
- Evaluate **string interning** (cstree-style) so identical terms share storage —
  consistent with "identical terms are the same link."
- Optionally evaluate the ecosystem-native **`doublets`** crate (Unlicense) as a
  file-backed substrate for large networks (`Doublets` trait).
- Keep both trivia attachment policies working across snapshots (NFR-5).

## Acceptance criteria

- [ ] Forking a `MutableNetworkSnapshot`, editing one link, and committing leaves
      the original snapshot reconstructing the **old** bytes while the fork
      reconstructs the **new** bytes.
- [ ] Unchanged subtrees are structurally shared (no full deep copy) — demonstrated
      by a test/benchmark.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §B, §E
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 6
- Requirements: CORE-11/12, PAR-5
