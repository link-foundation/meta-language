---
title: "Acquire a Delphi/Object Pascal grammar"
labels: enhancement
---

## Context

Delphi/Object Pascal is TIOBE `#10`. Only a **generic** `tree-sitter-pascal`
(Isopod/maxxnino) exists — not Delphi-specific. See
[`rust-libraries-survey.md`](../rust-libraries-survey.md) §A.

## Scope

- Adopt `tree-sitter-pascal` and wire it through the
  [#7](https://github.com/link-foundation/meta-language/issues/7) adapter.
- Document the Delphi-specific constructs the generic grammar misses (units,
  properties, attributes, generics, inline variables).
- Decide whether to extend/fork for Delphi coverage or accept generic-Pascal scope
  for now (record the decision).

## Acceptance criteria

- [ ] A Pascal/Delphi sample parses with the grammar and reconstructs byte-for-byte.
- [ ] Gaps vs full Delphi documented.
- [ ] Grammar license recorded; compatible with Unlicense.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §A
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 2
- Requirement: LANG-PL (Delphi/Object Pascal)
- Blocked by: [#7](https://github.com/link-foundation/meta-language/issues/7)
