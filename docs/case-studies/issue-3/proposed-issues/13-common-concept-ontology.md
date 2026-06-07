---
title: "Seed the common concept ontology from meta-expression's semantic lexicon"
labels: enhancement
---

## Context

Cross-language transformation is only meaningful if two languages sharing a concept
reference the **same** concept link (issue #1 §3.4, CORE-10). `LinkType::Concept`
and the `Semantic` projection exist, but no concept ontology is populated.
meta-expression already ships `semantic-lexicon.json` (**351 concepts**, en/hi/ru/zh
— verified, correcting issue #1's "328"). See
[`ecosystem-foundations.md`](../ecosystem-foundations.md) (meta-expression) and
[`solution-plans.md`](../solution-plans.md) Solution 8.

## Scope

- Import meta-expression's `semantic-lexicon.json` as concept links (anchored to
  Wikidata QIDs where present, e.g. Hawaii Q782, U.S. state Q35657).
- Seed a shared set of structural concepts for functional/procedural languages
  (function, binding, application, sequence, branch, loop, …).
- Map each concept to per-language concrete syntax for the initial language set;
  enforce dedup (one concept link shared across languages, not duplicated).

## Acceptance criteria

- [ ] The lexicon imports as concept links; the count matches the source file.
- [ ] Two languages expressing the same concept reference the **same** concept link
      (asserted by a test).
- [ ] The `Semantic` projection surfaces the concept layer.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Ecosystem: [`ecosystem-foundations.md`](../ecosystem-foundations.md)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 8
- Requirement: CORE-10
- Blocked by: [#16](https://github.com/link-foundation/meta-language/issues/16)
