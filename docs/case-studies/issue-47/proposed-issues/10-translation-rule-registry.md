---
title: "Configurable translation-rule registry for to/from meta-language translation"
labels: enhancement
---

## Context

Issue #47 requires that "our API is expandable and all missing translation
from meta language and to meta language are expandable and configurable by end
user ... cross language rules based translation, with full freedom of
configuration". To-meta translation (parse) is total, but from-meta
cross-language output is gated on the hard-coded statehood proposition
(`has_statehood_proposition` in `src/reconstruction.rs`), and users cannot add
or replace translation rules. See [`requirements.md`](../requirements.md)
**R-15**/**R-17** and [`solution-plans.md`](../solution-plans.md) **S-10**.

**Blocked by:** `#04` (registry pattern for user-supplied components) and
`#09` (generalized concept space that rules map through).

## Scope

- `TranslationRuleSet`: ordered, named rules with LinkQuery-shaped match sides
  and per-language syntax templates (generalizing `insert_concept_mapping`).
- Quasiquote-style templates with placeholders (Babel `template` / GritQL
  precedent in
  [`competitors-code-tooling.md`](../competitors-code-tooling.md)) instead of
  string concatenation.
- Rule sets are values: loadable from LiNo text, registrable and replaceable
  at runtime; the statehood demo becomes one rule set among others.
- `reconstruct_text_as` consults the active rule set; missing-rule failures
  produce diagnostic links naming the unmatched structure (so users know what
  to configure).

## Acceptance criteria

- [ ] At least one translation pair runs entirely from user-supplied rules
      with no hard-coded gate.
- [ ] The statehood demo is re-expressed as a loadable rule set with existing
      tests still passing.
- [ ] Missing-rule diagnostics are tested.
- [ ] Rule sets round-trip through LiNo.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-15, R-17
- Solution: [`solution-plans.md`](../solution-plans.md) S-10
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
