---
title: "Binary doublets storage backend (doublets-rs) behind a storage trait"
labels: enhancement
---

## Context

Issue #47 requires storage as "binary links as in
https://github.com/linksplatform/doublets-rs and
https://github.com/linksplatform/doublets-web". No binary links storage exists
today; `Cargo.toml` has no `doublets` dependency. See
[`requirements.md`](../requirements.md) **R-8** and
[`solution-plans.md`](../solution-plans.md) **S-6**.

Research verdict ([`formats-storage-apis.md`](../formats-storage-apis.md)
Part B): viable - `doublets` 0.4.0 (published 2026-05-29) builds on stable
Rust 1.85, is Unlicense like this repo, provides file-mapped binary
persistence, and itself uses a three-layer API (raw `Links<T>` slice-query
ops, `Doublets<T>` ergonomic defaults, `DoubletsExt` iterators). formal-ai -
our heaviest planned user - already defines `LinkStoreBackend
{ LinoProjection, DoubletsRs, DoubletsWeb }`.

**Blocked by:** `#01` (readonly/mutable access semantics for the storage
trait) and `#05` (shared id discipline and the text↔binary bridge fixtures).

## Scope

- Extract a storage trait with `&self` reads / `&mut self` writes covering
  create/read/update/delete/search over links; the in-memory `LinkNetwork`
  becomes the default implementation.
- Add a Cargo-feature-gated backend implementing the trait over `doublets`
  0.4 file-mapped storage.
- Round-trip bridge tests: network → doublets store → network, and LiNo text
  ↔ binary equivalence via `#05`.
- Document doublets-web as a WASM exchange target through the same binary
  layout (no direct dependency), matching formal-ai's backend enum.

## Acceptance criteria

- [ ] Storage trait extracted with the in-memory implementation passing the
      existing test suite unchanged.
- [ ] `doublets` feature builds and round-trips parsed language fixtures
      through file-mapped storage.
- [ ] Readonly access mode from `#01` is honored by both backends.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-8, R-18
- Solution: [`solution-plans.md`](../solution-plans.md) S-6
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
