---
title: "Enrich the substitution/query matcher (S-expressions, by-type matching)"
labels: enhancement
---

## Context

The transform engine adopts link-cli's single match→substitute operation
(`src/substitution.rs`: create/update/delete/swap). The matcher today
(`src/query.rs` `LinkQuery`) matches only by link type / term / language / named
flag. Issue #1 §3.6 requires **advanced matching** by syntax, meaning, and type.
See [`ecosystem-foundations.md`](../ecosystem-foundations.md) (link-cli) and
[`solution-plans.md`](../solution-plans.md) Solution 7.

## Scope

- Confirm create/update/delete/swap parity against link-cli's own test forms
  (including variable binding `$index`/`$source`/`$target`).
- Add a **tree-sitter-query-like S-expression** matcher over links: quantifiers,
  alternation, anchors, captures, fields, negated fields → lowered to `LinkQuery`.
- Add **by-type** matching (match on `LinkType`/type links).
- Keep predicates a **pluggable host-evaluated layer** (engine binds captures; host
  runs regex/eq/semantic) — mirror tree-sitter's engine/host split (CORE-15).
- (Match-by-meaning depends on the concept layer
  [#17](https://github.com/link-foundation/meta-language/issues/17) — out of
  scope here; leave the hook.)

## Acceptance criteria

- [ ] create/update/delete/swap fixtures match link-cli's documented behavior.
- [ ] An S-expression query selects a target link by structure with captures.
- [ ] A by-type query selects links of a given `LinkType`.
- [ ] A host predicate hook is exercised by a fixture.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Ecosystem: [`ecosystem-foundations.md`](../ecosystem-foundations.md) (link-cli)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 7
- Requirements: CORE-13/14/15, PAR-8
