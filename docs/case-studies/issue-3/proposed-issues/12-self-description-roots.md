---
title: "Materialize self-description roots as links defined in their own terms"
labels: enhancement
---

## Context

The "meta" in meta-language (issue #1 §3.3) requires the system to **describe
itself in its own terms**: `link`, `reference`, `relation link`, `language`,
`grammar`, `type`, `concept`, `point`, `field`, `trivia`, `region`, `object` should
be actual links whose definitions are written in those same terms. The README lists
these roots and `LinkMetadata` has a `definition` field, but the roots are not yet
materialized as a seeded network. See
[`requirements.md`](../requirements.md) CORE-8/9 and
[`ecosystem-foundations.md`](../ecosystem-foundations.md) (meta-theory definitions).

## Scope

- Seed a self-description network where each root term is a link with a
  `definition` written in the controlled vocabulary (e.g. a "point" is defined as a
  link that references only itself in every position — meta-theory line 101).
- Seed the "common roots of common ontologies" from relative-meta-logic's
  `(Type: Type Type)` self-referential root.
- Expose it via the existing `describe` CLI subcommand.
- Honor NFR-1: "node" = self-referential link, "edge" = connecting link; never the
  word "graph."

## Acceptance criteria

- [ ] Each root term resolves to a link whose definition references only other
      defined root links (no undefined terms; no external vocabulary).
- [ ] `describe` emits the self-description network and it round-trips losslessly.
- [ ] A test asserts the definition closure is complete (every referenced term is
      itself defined).
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Ecosystem: [`ecosystem-foundations.md`](../ecosystem-foundations.md) (meta-theory)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 8
- Requirements: CORE-8/9
