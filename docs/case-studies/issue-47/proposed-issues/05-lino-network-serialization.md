---
title: "Serialize any links network to links-notation text (full LiNo round-trip)"
labels: enhancement
---

## Context

Issue #47 requires "storage (presenting as links notation (text based as in
https://github.com/link-foundation/links-notation) ...)". LiNo parses in
(`src/lino_parser.rs`), but arbitrary networks cannot be written back out as
links-notation text - only `self_description_text` emits LiNo-style lines. See
[`requirements.md`](../requirements.md) **R-7** and
[`solution-plans.md`](../solution-plans.md) **S-5**.

The reference Rust parser is the `links-notation` crate 0.13.0 (verified on
crates.io 2026-06-10), which formal-ai also pins
([`formats-storage-apis.md`](../formats-storage-apis.md) Part B).

## Scope

- Implement `LinkNetwork::to_lino()` and `LinkNetwork::from_lino()` covering
  every link kind (references, names, metadata) so any network round-trips.
- Property test: `from_lino(to_lino(n))` is isomorphic to `n` across parsed
  and hand-built networks.
- Align the emitted dialect with the `links-notation` 0.13 crate so other
  ecosystem parsers can consume the output; record divergences as parity
  fixtures.
- Use a doublets-style id discipline for unnamed links so text and (future)
  binary storage share one addressing scheme.

## Acceptance criteria

- [ ] `to_lino`/`from_lino` round-trip property test passes on language
      fixtures and synthetic networks.
- [ ] Output of `to_lino` is accepted by the `links-notation` crate parser in
      a test.
- [ ] links-notation `PARITY_FIXTURES` gain a serialization (output-side)
      fixture, not just parse-side.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-7
- Solution: [`solution-plans.md`](../solution-plans.md) S-5
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
