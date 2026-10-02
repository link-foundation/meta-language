---
bump: minor
---

### Added
- Lower a grammar into a less expressive notation with `lower_grammar` (JavaScript `lowerGrammar`) for abnf, antlr, bnf, ebnf, gbnf, lark, pest and tree-sitter-json. Each construct the target cannot write moves into a helper rule encoded in constructs it can write, and the lowering is `exact` or `approximate`. The result is the executable text, which the target's own importer reads back, plus reconstruction metadata as links. The metadata names every helper, its construct, encoding and original expression, plus every rename, rule kind and documentation the target does not keep. `render_lowering_metadata` and `parse_lowering_metadata` write and read the metadata, and `reconstruct_grammar` rebuilds the original grammar from the executable and the metadata.
- Check a lowering with `check_grammar_lowering`, which reports lossy emission, an executable that does not read back as the lowered grammar, every feature the reconstruction lost (`dropped_grammar_features`) and, for an exact lowering, every sample it disagrees on. `parity/fixtures/grammar-importers.json` has a `lowering` section that records the executable, metadata and reconstruction of two grammars in every target, for both runtimes.
