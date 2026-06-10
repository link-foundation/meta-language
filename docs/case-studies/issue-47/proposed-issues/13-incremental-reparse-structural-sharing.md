---
title: "Incremental re-parse, snapshot structural sharing, and structural diff"
labels: enhancement
---

## Context

Issue #47's title demands "the richest features set". The competitor survey
([`competitors-code-tooling.md`](../competitors-code-tooling.md)) ranks
incremental re-parse with subtree reuse (tree-sitter) and red/green persistent
trees with structure sharing (Roslyn, rowan/Biome) as the top capabilities
meta-language lacks; difftastic adds structural snapshot diffing. Today every
edit re-parses from scratch. See
[`solution-plans.md`](../solution-plans.md) **S-13**.

## Scope

- Expose tree-sitter's native incremental parsing (`InputEdit` + previous
  tree) through `src/tree_sitter_adapter.rs`:
  `LinkNetwork::apply_edit(range, new_text)` re-parses only affected regions;
  link ids outside the edited span stay stable.
- Extend `NetworkSnapshot` so an edited fork shares unchanged links with its
  parent (rowan/cstree precedent), with a test measuring sharing.
- Structural diff between two snapshots returning changed/added/removed link
  sets (difftastic precedent) - this also gives transforms a cheap dry-run
  preview.

## Acceptance criteria

- [ ] `apply_edit` produces a network equal to a from-scratch parse of the
      edited text (byte-exact reconstruction) while preserving ids outside
      the edit.
- [ ] Snapshot fork sharing is asserted (unchanged links are not duplicated).
- [ ] Structural diff is tested on an edit fixture and exposed publicly.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: issue title ("richest features set"); [`requirements.md`](../requirements.md) R-2 context
- Solution: [`solution-plans.md`](../solution-plans.md) S-13
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
