---
title: "Unified query + transform surface over links"
labels: enhancement
---

## Context

Once the tree-sitter adapter (`#03`) and the enriched matcher (`#09`) exist, the
codemod story (jscodeshift/Recast parity, PAR-3/PAR-4) needs a single ergonomic
surface: select links with a query, transform with a substitution rule, and
re-serialize preserving every unchanged byte. See
[`competitor-test-suites.md`](../competitor-test-suites.md) (Recast/jscodeshift) and
[`solution-plans.md`](../solution-plans.md) Solutions 1 & 7.

## Scope

- A `find(query) -> captures` + `replace(rule)` API over the network, delegating
  matching to `#09`'s S-expression matcher and rewriting via `apply_substitution`.
- Guarantee that a transform touching one target leaves all other bytes identical
  (the Recast `strictEqual(source, code)` / jscodeshift `__testfixtures__`
  guarantee).
- Port one jscodeshift-style input/output fixture pair (e.g. identifier rename) and
  one Recast-style "change only the literal" fixture.

## Acceptance criteria

- [ ] A query selects exactly the intended links; a transform rewrites only those.
- [ ] Output equals input except for the intended bytes (whitespace/comments of
      unchanged regions preserved).
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Suites: [`competitor-test-suites.md`](../competitor-test-suites.md) §3, §4
- Solution: [`solution-plans.md`](../solution-plans.md) Solutions 1, 7
- Requirements: CORE-14, PAR-1/3/4/8
- Blocked by: `#03`, `#09`
