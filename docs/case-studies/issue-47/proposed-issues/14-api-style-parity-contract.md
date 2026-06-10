---
title: "API-style parity contract: every operation via direct, fluent, and substitution styles"
labels: enhancement
---

## Context

Issue #47: "other traditional API adapters, like chaining, direct OOP methods
and so on ... All of APIs styles should support all the same operations up on
the languages described in meta-language." Operations are unevenly reachable
today: queries have builder + S-expression forms, substitutions only direct
methods; nothing guarantees style parity. See
[`requirements.md`](../requirements.md) **R-12**/**R-13**/**R-14** and
[`solution-plans.md`](../solution-plans.md) **S-14**.

Rust precedent ([`formats-storage-apis.md`](../formats-storage-apis.md)
Part C): many styles over one executor (sqlx/diesel/sea-orm), core trait +
default-implemented extension trait (doublets-rs itself), syn's
`Visit`/`VisitMut` split.

**Blocked by:** `#06` (the storage trait the styles layer over) and `#10`
(translation operations must exist before the matrix can include them).

## Scope

- Declare the operation inventory as data: an `API_OPERATIONS` registry
  (parse, query, transform, substitute, serialize, snapshot, translate,
  verify, ...) × styles (direct method, fluent chain, link-cli substitution
  text, S-expression/LiNo text) - the registry-plus-gate pattern already used
  for languages and parity targets.
- Implement the missing styles: a fluent pipeline
  (`network.find(q).replace(r).reconstruct()`, jscodeshift precedent) as a
  default-implemented extension trait; link-cli-style text operations
  (create `() ((1 1))`, delete `((1 1)) ()`, update same-index, read identity
  - verified semantics of link-cli 0.2.7) for every operation where the style
  applies; documented N/A entries where a style genuinely cannot apply.
- A unit test iterates `API_OPERATIONS` and asserts each operation has an
  executable fixture per applicable style - the parity gate (R-14).

## Acceptance criteria

- [ ] `API_OPERATIONS` registry exists and is gated by a per-style fixture
      test; N/A cells are explicit, not absent.
- [ ] Fluent pipeline covers parse → query → transform → reconstruct in one
      chain with a test.
- [ ] link-cli-style text surface covers the operations where applicable.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-12, R-13, R-14
- Solution: [`solution-plans.md`](../solution-plans.md) S-14
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
