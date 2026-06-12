---
title: "Readonly or mutable engine as per user configuration"
labels: enhancement
---

## Context

Issue #47 opens with: "We need to make sure our engine works as readonly or
mutable as per user configuration." Today `LinkNetwork` is always mutable
(`insert_link`, `set_references`, `set_span`, `set_flags`, `apply_substitution`
are unconditional), and `NetworkSnapshot` / `MutableNetworkSnapshot`
(`src/snapshots.rs`) offer immutability only as an opt-in versioning pattern -
there is no configuration that enforces a read-only engine. See
[`requirements.md`](../requirements.md) **R-1** and
[`solution-plans.md`](../solution-plans.md) **S-1**.

Competitor precedent: Roslyn's red/green trees and rowan/cstree persistent
trees are immutable-by-default with explicit mutable derivations
([`competitors-code-tooling.md`](../competitors-code-tooling.md)).

## Scope

- Add an `AccessMode { ReadOnly, Mutable }` setting to `ParseConfiguration`.
- Add `LinkNetwork::freeze()` / `as_read_only()` yielding a read-only view
  type that exposes only `&self` operations (query, project, reconstruct,
  verify, serialize); mutators are unreachable at compile time on the view.
- Parsing with `AccessMode::ReadOnly` returns the frozen form; mutation
  attempts at runtime API boundaries fail with a clear diagnostic.
- Reuse `NetworkSnapshot`'s `Arc<LinkNetwork>` sharing for the frozen form so
  this composes with snapshot versioning instead of duplicating it.

## Acceptance criteria

- [ ] `ParseConfiguration` carries an access mode; default remains `Mutable`
      (no behavior change for existing callers).
- [ ] A read-only view supports every non-mutating public operation and none
      of the mutating ones.
- [ ] Tests cover: freeze → query/reconstruct works; mutation is impossible
      (compile-fail or error contract, documented); snapshot interop.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-1
- Solution: [`solution-plans.md`](../solution-plans.md) S-1
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
