---
title: "Adopt ecosystem test corpora (links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, meta-expression)"
labels: enhancement
---

## Context

The link-foundation / linksplatform projects are "internal competitors" whose
corpora this crate must subsume (issue #1 §6, I3-2). APIs, test shapes, and verified
corrections are documented in
[`ecosystem-foundations.md`](../ecosystem-foundations.md). All are
Unlicense/permissive.

## Scope

- **links-notation** — port the doublet/triplet/N-tuple/indented/self-reference
  parsing tests (cross-language identity; ~138 tests/language binding).
- **link-cli** — create `() ((1 1))`, update `((1: 1 1)) ((1: 1 2))`, delete
  `((1 1)) ()`, swap `((($index: $source $target)) (($index: $target $source)))`.
- **lino-objects-codec** — `decode(encode(x)) == x`, shared + circular references.
- **relative-meta-logic** — dependent types, many-valued, liar-paradox cases →
  `TruthValue`.
- **formal-ai** — replay `data/seed/*.lino` + `data/benchmarks/*.lino` as a
  no-regression gate (cite the actual files, **not** issue #1's unverified "706").
- **meta-expression** — formalize→naturalize round-trip ("Hawaii is a state." →
  "Гавайи это штат."), "1 + 1 = 2", "this statement is false"; cite the verified
  **351**-concept lexicon (not "328").

## Acceptance criteria

- [ ] Each ecosystem project contributes ported fixtures with provenance comments.
- [ ] The formal-ai corpus runs as a regression gate against the actual `.lino`
      files.
- [ ] Verified figures (351 concepts, ~138 tests/lang, doublets crate name) are used
      in comments — not the founding-issue estimates.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Ecosystem: [`ecosystem-foundations.md`](../ecosystem-foundations.md)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 9
- Requirements: I3-2, PAR-7…12
