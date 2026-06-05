---
title: "Cross-language reconstruction + configurable formalization/naturalization"
labels: enhancement
---

## Context

The crate's deepest novelty (issue #1 §3.7, CORE-16/17): reconstruct a network into
a **different** target language via the common concept layer, and make text→network
and network→text **configurable** — the exact knob formal-ai and meta-expression
need (formalization levels 1–4, naturalization). `reconstruct_text()` is
same-language only today. See
[`ecosystem-foundations.md`](../ecosystem-foundations.md) (meta-expression) and
[`solution-plans.md`](../solution-plans.md) Solution 8.

## Scope

- Implement cross-language reconstruction: walk concept links (`#13`) → target-language
  concrete syntax.
- Validate with meta-expression's worked example: "Hawaii is a state." (Q782 /
  Q35657) → "Гавайи это штат." (en → ru).
- Add configurable (de)formalization to `ParseConfiguration`: formalization levels
  1–4, naturalization direction; expose without leaking consumer internals.
- Keep `TruthValue` (many-valued/paradox) available for the semantic-match path
  (relative-meta-logic).

## Acceptance criteria

- [ ] A network parsed from English reconstructs into Russian for the Hawaii fixture
      (and the reverse).
- [ ] Same-language reconstruction stays byte-exact for unchanged regions.
- [ ] Formalization level is configurable and changes output as documented.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Ecosystem: [`ecosystem-foundations.md`](../ecosystem-foundations.md)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 8
- Requirements: CORE-16/17, PAR-12
- Blocked by: `#13`
