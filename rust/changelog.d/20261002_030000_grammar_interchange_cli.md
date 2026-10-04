---
bump: minor
---

### Added
- Add a `meta-language grammar` command with `formats`, `import`, `validate`, `convert`, `export`, `merge`, `rename`, `round-trip` and `help`. The JavaScript package installs the same `meta-language` command, and both commands give the same output and exit status for every case in `parity/fixtures/grammar-importers.json`. The library entry point is `run_grammar_command` (JavaScript `runGrammarCommand`). It reads and writes abnf, antlr, bnf, ebnf, gbnf, lark, pest, tree-sitter-json and `native`. The `native` format is a line-per-rule listing of the grammar model, read by `parse_native_grammar` and written by `render_native_grammar`.
- Add `validateGrammar` and the ANTLR and Lark emitters to the JavaScript package so it matches the Rust grammar diagnostics and exporters.
