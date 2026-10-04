# Bulk grammar pipeline

[`js/scripts/run-grammar-bulk-pipeline.mjs`](../../js/scripts/run-grammar-bulk-pipeline.mjs)
handles every language in
[the language catalog](../../parity/language-grammar-inventory.json) that has
a grammar. That is 57 languages
([I195-GRAMMAR-BULK-PIPELINE](../vision.md#native-merged-grammars)). For each
language, in its own process with a memory limit and a timeout, it runs these
stages:

1. **tree-sitter import.** The pipeline imports the pinned tree-sitter
   `grammar.json` natively, without the generated parser:
   - A source pinned in [`parity/grammars/sources.json`](../../parity/grammars/sources.json)
     is imported the way the shipped native grammar is, with its reviewed
     names and native scanner (see [native grammars](native-grammars.md)).
   - Any other grammar comes from the crate `rust/Cargo.lock` pins, or from the
     upstream revision of a vendored parser.
2. **grammars-v4 import.** The pipeline imports the pinned
   [antlr/grammars-v4](https://github.com/antlr/grammars-v4) grammar listed in
   [`parity/grammars-v4-sources.json`](../../parity/grammars-v4-sources.json).
   A language without one has a note there instead.
3. **compile and parse.** Both grammars are compiled on the native executor
   and parse the catalog's sample source. The rows of the tree-sitter tree are
   compared with the oracle rows in
   [`parity/fixtures/default-cst-expected.json`](../../parity/fixtures/default-cst-expected.json).
4. **merge.** The grammars that imported are merged with `mergeGrammars`.

A failing language does not fail the run. It becomes a matrix row that lists
its missing features, each with the stage it failed in:

- unsupported constructs;
- undefined rules;
- compile errors;
- rejected samples, with the position and the expected tokens;
- rows that differ from the oracle;
- stages that timed out.

CI's `Bulk Grammar Pipeline` job in [`ci.yml`](../../.github/workflows/ci.yml)
runs every language. It writes the Markdown matrix to the job summary and
uploads both the JSON and the Markdown matrix as the `grammar-bulk-matrix`
artifact. The job runs alongside the JavaScript and Rust workflows and does
not delay either of them.

```sh
cargo fetch --manifest-path rust/Cargo.toml   # the registry grammar.json files
node js/scripts/run-grammar-bulk-pipeline.mjs --out-dir grammar-bulk --jobs 4
node js/scripts/run-grammar-bulk-pipeline.mjs --out-dir grammar-bulk --only JSON,INI
```

Upstream grammars are fetched at their pinned revisions into
`GRAMMAR_BULK_CACHE`, which defaults to a directory under the system temporary
directory. [`js/tests/grammar-bulk-pipeline.test.js`](../../js/tests/grammar-bulk-pipeline.test.js)
checks two things without downloading anything:

- every catalog language has a pinned source;
- a row runs every stage on the pinned C grammar, and the matrix lists every
  missing feature.

## What the matrix has driven so far

- **A token over an extra.** Under `(matching longest)`, a `token(...)` and a
  terminal are lexed over a separator or a silent extra rule that matches the
  same text. tree-sitter does the same: it shifts a valid token before it
  reduces the same text to an extra. This lets the tree-sitter INI and CSV
  grammars take their line breaks. Both runtimes do this, and
  [`parity/fixtures/grammar-token-over-extra.json`](../../parity/fixtures/grammar-token-over-extra.json)
  records the cases.
- **ANTLR alternative labels.** In `expr # Label`, the label names the context
  class ANTLR generates for the alternative; it is not syntax. Both importers
  keep the alternative and add `alternative Label` to the rule's doc. Before
  this, every grammars-v4 grammar that labels its alternatives (Rust among
  them) failed to import.
- **ANTLR `~` over a set.** `~` complements a set and matches one character
  outside it. The set can be a character class, a one-character literal, a
  range, or an alternation of these. `~` over anything else stays a negative
  lookahead. Before this, `~'"'` imported as a zero-width lookahead, so the
  grammars-v4 CSV `STRING` rule rejected every quoted field. The shared fixture
  [`set-complement.g4`](../../rust/tests/fixtures/grammar/antlr/set-complement.g4)
  checks this in both importers.
