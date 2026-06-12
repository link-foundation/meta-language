---
title: "Natural-language grammatical-correctness parsing (morphology + syntax, no semantics)"
labels: enhancement
---

## Context

Issue #47 requires "natural languages (including actual parsing of their
grammar with no semantic checks ... grammatical correctness, syntax
correctness and so on - should be fully supported)". Current support is
identification, segmentation, normalization, and script annotations
(`src/natural_language.rs`) - nothing parses grammar, so correctness cannot be
checked. See [`requirements.md`](../requirements.md) **R-5** and
[`solution-plans.md`](../solution-plans.md) **S-8**.

Research ([`competitors-natural-language.md`](../competitors-natural-language.md)):
Grammatical Framework + Resource Grammar Library is the only surveyed system
doing exactly this job (parse-or-reject, no semantics, covers the ten target
languages, LGPL/BSD); UD supplies the morphosyntax vocabulary and corpora;
Wikidata lexemes (CC0) and UniMorph (CC BY-SA) supply word forms; LanguageTool
(LGPL, proven portable to Rust by nlprule) supplies explainable negative
checks.

## Scope (staged)

1. Adopt UD's UPOS/UFeats/deprel inventory as link-type vocabulary for
   morphosyntax links; import CoNLL-U fixtures (e.g. via `rs-conllu`) with
   per-treebank license provenance.
2. Word-level correctness: morphological lexica seeded from Wikidata lexeme
   Forms (CC0; UniMorph optional second source); unknown/ill-formed tokens
   become recoverable `is_error` links, reusing the parse-recovery contract.
3. Sentence-level correctness: integrate GF RGL grammars compiled to PGF
   (start with the `gf-core` crate; evaluate a native PMCFG reader over the
   links network as the long-term path); grammatical sentences parse clean,
   ungrammatical ones carry error links - `verify_full_match()` thereby
   answers "is this grammatical?".
4. Explainable negatives: port a starter set of LanguageTool-style pass/fail
   rule sentences per target language as fixtures (DELPH-IN "mal-rule" model
   for explanations).

## Acceptance criteria

- [ ] For each of the ten `NATURAL_LANGUAGE_TARGETS`: at least one grammatical
      fixture parses with a clean `verify_full_match()` and one ungrammatical
      fixture surfaces error links - while both reconstruct byte-for-byte.
- [ ] Morphosyntax links use the UD-derived vocabulary and are queryable.
- [ ] All imported data carries license provenance in fixtures.
- [ ] Staging is allowed (language-by-language), but each landed language is
      fully gated; remaining ones are tracked in the roadmap.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-5
- Solution: [`solution-plans.md`](../solution-plans.md) S-8
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
