---
title: "Generalize the shared concept space with an exact-match reuse discipline"
labels: enhancement
---

## Context

Issue #47 requires "advanced shared concepts space between languages, which
should be reused when only matched exactly, so it will help to simplify
automated translation between any languages". The 351-concept lexicon
(`seed_common_concept_ontology`) and `reference_for_atom` exact-`find_term`
reuse exist, but concepts are not interned across languages under a stated
exact-match discipline, and concept-mediated translation is demo-grade. See
[`requirements.md`](../requirements.md) **R-6** and
[`solution-plans.md`](../solution-plans.md) **S-9**.

Research ([`competitors-natural-language.md`](../competitors-natural-language.md)):
Wikidata's two-layer design (language-bound lexeme ↔ language-free Q-item,
all CC0) is the friction-free model; WordNet CILI institutionalizes exactly
the issue's rule - reuse an interlingual id only on exact match, otherwise
mint a new one. BabelNet is rejected (non-commercial license, API-gated).

## Scope

- Two-layer concept links: language-bound expression links connected to
  language-free concept links; reuse a concept only on exact-id match,
  otherwise mint a new concept link.
- Alias links on concepts for external ids (WordNet CILI ILI ids, Wikidata
  Q-ids) so external vocabularies can be attached without becoming load-bearing.
- Generalize `seed_common_concept_ontology` into an import surface that loads
  concept sets from LiNo files, keeping the 351-concept seed as the default.
- Tests proving the exact-match rule: same id → same link reused; near-miss
  (case, diacritics, sense) → new concept minted.

## Acceptance criteria

- [ ] Concept interning API with the exact-match rule, gated by tests
      including near-miss cases.
- [ ] External-id alias links queryable via `LinkQuery`.
- [ ] Concept import from LiNo round-trips through the ontology.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-6
- Solution: [`solution-plans.md`](../solution-plans.md) S-9
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
