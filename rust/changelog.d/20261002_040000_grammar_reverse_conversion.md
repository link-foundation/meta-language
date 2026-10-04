---
bump: minor
---

### Added
- Check reverse conversion with `check_grammar_reverse_conversion` (JavaScript `checkGrammarReverseConversion`). The source grammar is written as native links by `render_grammar_links` and read back by `parse_grammar_links`, and the emitter is given only the grammar read back from those links. The export is re-imported and compared rule by rule (names, start rule, kinds, definitions and documentation), and both grammars are run on independent accept and reject samples.
- Add a lossless mode for abnf, antlr, bnf, ebnf, gbnf, lark, pest and tree-sitter-json. `import_grammar_lossless` returns the grammar and a `GrammarLayout` with the definition texts, comments and spacing of the source. `render_grammar_layout_links` and `parse_grammar_layout_links` write and read the layout as links. `emit_grammar_lossless` rebuilds the source byte for byte and writes only changed or new rules fresh. `parity/fixtures/grammar-importers.json` has a `reverse` section with one commented source per format, recorded for both runtimes.

### Fixed
- The GBNF emitter writes rule documentation as `#` comments instead of dropping it.
- JavaScript grammars rebuilt by renames, merges and start rule mutations keep their rule documentation, as the Rust ones already did.
- The EBNF importer skips ISO `(* ... *)` comments outside string literals, as the JavaScript importer does.
