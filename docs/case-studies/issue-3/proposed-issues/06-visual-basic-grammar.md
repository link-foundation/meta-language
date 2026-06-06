---
title: "Acquire a Visual Basic grammar (biggest parser gap)"
labels: enhancement
---

## Context

Visual Basic is TIOBE #7 but has **no official tree-sitter grammar and no
maintained Rust parser** — the single biggest parser gap in the target set. See
[`rust-libraries-survey.md`](../rust-libraries-survey.md) §A and
[`solution-plans.md`](../solution-plans.md) Solution 2.

## Scope (in priority order)

1. Evaluate third-party grammars: `arborium-vb` and CodeAnt-AI's `tree-sitter-vb`
   fork. Assess coverage, license, and maintenance.
2. If one is adequate: vendor/depend on it, harden against our fixture set, wire it
   through the [#7](https://github.com/link-foundation/meta-language/issues/7)
   adapter.
3. If none is adequate: author a tree-sitter grammar for the VB.NET subset we need,
   starting from the official language reference.

## Acceptance criteria

- [ ] A Visual Basic sample parses with real grammar coverage and reconstructs
      byte-for-byte.
- [ ] An error fixture produces `is_error`/`is_missing` links and still round-trips.
- [ ] The grammar's provenance and license are documented; license is compatible
      with this repo's Unlicense.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §A
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 2
- Requirement: LANG-PL (Visual Basic)
- Blocked by: [#7](https://github.com/link-foundation/meta-language/issues/7)
  (adapter contract)
