---
title: "Rust traits/types representation: encode arbitrary Rust values and type shapes as links"
labels: enhancement
---

## Context

Issue #47 requires "also Rust traits/types representation" as a storage form.
`LinkType::Object` and `LinkNetwork::insert_object` cover identity and
circular references, and lino-objects-codec fixtures are gated, but arbitrary
user structs/enums cannot be encoded to links and decoded back, and Rust type
*shapes* are not represented. See [`requirements.md`](../requirements.md)
**R-9** and [`solution-plans.md`](../solution-plans.md) **S-7**.

Ecosystem parity target: `lino-objects-codec` 0.2.1 (Rust) / 0.4.0 (npm) -
object encode/decode with identity and circular-reference preservation
([`formats-storage-apis.md`](../formats-storage-apis.md) Part B).

**Blocked by:** `#05` (the LiNo serializer makes encodings inspectable and
provides the codec's text form).

## Scope

- `ToLinks` / `FromLinks` traits with implementations for primitives, `Vec`,
  `Option`, maps; decide between a `#[derive(Links)]` proc-macro and a serde
  Serializer/Deserializer adapter (serde reuses the whole ecosystem at zero
  macro cost) - record the decision and rationale.
- Preserve shared references and cycles via object-identity links, porting
  lino-objects-codec's shared-reference and circular-reference cases as
  fixtures.
- Represent Rust type declarations (struct/enum/trait shape) as links using
  the existing self-description roots (`type`, `Type`, `field`) so type shape
  is queryable data.

## Acceptance criteria

- [ ] A user struct with nesting, sharing, and a cycle round-trips through
      links and through LiNo text.
- [ ] Type-shape links are queryable via `LinkQuery`.
- [ ] lino-objects-codec parity fixtures extended to the new codec path.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-9
- Solution: [`solution-plans.md`](../solution-plans.md) S-7
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
