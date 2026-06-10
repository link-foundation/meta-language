---
title: "Enrich the query/transform algebra (rule composition, ellipsis, templates, strategies)"
labels: enhancement
---

## Context

Issue #47's title demands "the richest features set" against all competitors.
The code-tooling survey ([`competitors-code-tooling.md`](../competitors-code-tooling.md))
identifies query/transform operators that competitors ship and meta-language
lacks. `LinkQuery` covers type/term/language/named/S-expressions/captures;
`ReplacementRule`/`SubstitutionRule` cover replace-and-substitute. See
[`requirements.md`](../requirements.md) **R-11** (extension) and
[`solution-plans.md`](../solution-plans.md) **S-12**.

## Scope

Operators to add over links (each with competitor provenance):

- Relational, composable rule algebra: `inside`, `has`, `precedes`,
  `follows`, `all`/`any`/`not`, named reusable sub-rules (ast-grep).
- Ellipsis (`...`) gap matching and kind-constrained (typed) metavariables
  (Semgrep, Coccinelle).
- Grammar-less fallback matching over plain-text token links for unwired
  languages (Comby).
- Quasiquote replacement templates with placeholder safety and
  parenthesization-conservative reprinting (Babel template, Recast).
- Traversal-strategy combinators: `topdown`, `bottomup`, `innermost`,
  `fixpoint` (Stratego, Rascal).
- A valid/invalid snapshot test harness for rules (ast-grep) so rule suites
  are self-verifying.

## Acceptance criteria

- [ ] Each operator has unit tests plus at least one parity fixture ported
      from the competitor that motivated it.
- [ ] S-expression syntax extended (or a documented alternative added) so the
      new operators are reachable from text queries, not just builders.
- [ ] Existing query/transform tests pass unchanged.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-11 (extension), issue title
- Solution: [`solution-plans.md`](../solution-plans.md) S-12
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
