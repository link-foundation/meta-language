# Grammar translation

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [A3](../case-studies/issue-93/proposed-issues/A3-grammar-concept-ontology.md)
for grammar concepts and
[C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md)
for concept-aligned grammar translation.

Grammar translation converts one notation or grammar dialect into another by
going through the shared [`Grammar` IR](architecture.md) and the grammar concept
ontology. It should not be a direct string rewrite from source notation to target
notation.

## Minimal example

```text
source grammar text
  -> importer
  -> Grammar
  -> grammar concept alignment
  -> target emitter
  -> target grammar text
```

The current branch exposes grammar concept helpers in
[`src/grammar/concepts.rs`](../../rust/src/grammar/concepts.rs): `GRAMMAR_CONCEPTS`,
`annotate_grammar_concepts`, `grammar_expr_concept_id`, and
`rule_concept_id`. The full translation API is owned by
[C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md).

## Why concepts matter

Different notations can spell the same construct differently, and some notations
make a construct implicit. Concept alignment gives translation a stable semantic
handle for constructs such as:

- grammar
- rule
- terminal
- non-terminal
- sequence
- choice
- optional
- repetition
- character class
- predicate
- capture

When a target notation cannot represent a source concept directly, the
translation should use a documented fallback instead of silently dropping the
construct. [F2](../case-studies/issue-93/proposed-issues/F2-grammar-format-fidelity-matrix.md)
owns the public matrix for those tradeoffs.

## Relationship to document translation

The document-format translation path is described in
[`docs/cross-format-fidelity.md`](../cross-format-fidelity.md): parse into a
shared concept tree, then render the target format with known fallbacks. Grammar
translation follows the same shape, but its concept set is grammar-specific and
its renderers are grammar emitters instead of document renderers.

The generic translation rule infrastructure in
[`src/translation_rules.rs`](../../rust/src/translation_rules.rs) is available when a
grammar translation needs declarative rule mappings, but C6 owns the grammar
specific public boundary.

## See also

- [Architecture](architecture.md) for the IR and links encoding.
- [Import and export](import-export.md) for importer and emitter boundaries.
- [Code generation](codegen.md) when the translation target is executable parser
  source.
