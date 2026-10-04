---
bump: minor
---

### Added
- `emit_tree_sitter_json` renders a grammar as tree-sitter `grammar.json`, and the JavaScript package now exports `emitAbnf`, `emitBnf`, `emitEbnf`, `emitPest`, and `emitTreeSitterJson`, so every importer has a same-format emitter in both packages.
- Shared importer corpus `parity/fixtures/grammar-importers.json`: each ABNF, BNF, EBNF, pest, and tree-sitter JSON case is imported, run through the generated JavaScript and Rust parsers, re-emitted, re-imported, and serialized in both packages.

### Fixed
- EBNF import decodes `\t \b \n \r \f \/ \\` and quote escapes, gives concatenation precedence over alternation (ISO 14977), and accepts the empty literal `""`; EBNF emission escapes terminals to match.
- BNF emission no longer writes backslash escapes, which classic BNF does not have: terminals containing a double quote are single-quoted, and terminals containing both quote kinds are split into quoted runs.
- JavaScript BNF/EBNF import maps an empty terminal to the empty expression, as Rust does, and treats BNF backslashes literally.
- The pest compound-atomic modifier `$` imports as a token rule in both packages, so it survives an import/emit round trip.
- Rust ABNF import appends referenced core rules in first-reference order, matching JavaScript.
- Generated Rust AST types no longer shadow types the generated module uses: rules named `string`, `vec`, `option`, `box`, `result`, `rule`, or `self` get a `Node` suffix (`StringNode`), so their parsers compile.
