---
title: "Single-language restriction profiles (e.g. JavaScript-to-JavaScript mode)"
labels: enhancement
---

## Context

Issue #47: "if we want to keep working with single language for example for
javascript to javascript transformation - we just restrict ourselves with
using only features of meta language that JavaScript supports exactly". No
symbol in `src/` models per-language capability profiles today. See
[`requirements.md`](../requirements.md) **R-16** and
[`solution-plans.md`](../solution-plans.md) **S-11**.

**Blocked by:** `#10` (a profile is effectively the domain of a translation
rule set).

## Scope

- `LanguageProfile` links: per-language capability sets naming the concepts,
  link types, and translation rules a target language supports - profiles are
  links, hence queryable and user-editable like everything else.
- A profile can be declared by the user or computed from a `#10` rule set's
  domain.
- `ParseConfiguration::with_profile(...)` (or a transform-time argument):
  operations that would leave the profile fail with a diagnostic link naming
  the unsupported feature instead of producing untranslatable output.
- Ship a JavaScript profile as the reference case with a JS→JS transform test
  that stays within profile and one that is correctly rejected.

## Acceptance criteria

- [ ] Profiles are representable as links and queryable.
- [ ] In-profile JS→JS transform passes; out-of-profile operation yields the
      documented diagnostic.
- [ ] Profiles compose with `#10` rule sets (declared or computed domain).
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-16
- Solution: [`solution-plans.md`](../solution-plans.md) S-11
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
