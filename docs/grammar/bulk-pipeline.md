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
   A language without one has a note there instead. ANTLR has no start rule,
   so the import starts at the `<entry-point>` that the `desc.xml` beside the
   grammar names. grammars-v4 runs its own tests from that rule. Without a
   `desc.xml`, the first parser rule is the start rule.
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
- **ANTLR left recursion, surrogates and numbered channels.**
  - *Left recursion.* ANTLR resolves a left-recursive rule by alternative
    order. The importers give each alternative that starts or ends with the
    rule a `precedence` form, where earlier alternatives bind tighter.
    `<assoc=right>` makes an alternative right-associative. Other alternative
    options join the rule's doc.
  - *Surrogates.* ANTLR reads `\uD800`-`\uDFFF` in a set as UTF-16
    surrogates. Text here is code points, so a set keeps only the scalar
    values it names.
  - *Numbered channels.* `-> channel(2)` puts a token on a numbered channel.
  - *Before this,* the JavaScript, Java, TypeScript, PHP, Lua and Solidity
    grammars failed to import.
  - *Coverage.* The shared fixture
    [`precedence.g4`](../../rust/tests/fixtures/grammar/antlr/precedence.g4)
    checks all three in both importers.
- **ANTLR `caseInsensitive` and rule preludes.**
  - *Case.* `options { caseInsensitive = true; }` makes the lexer match
    literals, ranges and sets in either case. A rule's own `options` block can
    turn it off for that rule. A literal becomes a case-insensitive terminal.
    A range or set also gets the other case of each character it lists, and
    of the ASCII letters in its ranges.
  - *Preludes.* `returns [...]`, `locals [...]` and `throws A, B` declare
    target-language values. Like actions, they are added to the rule's doc.
    Other rule options are added to the doc too.
  - *Before this,* the PHP and Swift grammars failed to import.
  - *Coverage.* The shared fixture
    [`case-insensitive.g4`](../../rust/tests/fixtures/grammar/antlr/case-insensitive.g4)
    checks both in both importers.
- **grammars-v4 entry points.** The pipeline once started each grammar at its
  first parser rule. TypeScriptParser.g4 begins with
  `initializer : '=' singleExpression`, so the TypeScript sample was rejected
  at its end, where `=` was expected. Starting at the `desc.xml` entry point
  (`program`) lets the sample parse.
- **ANTLR `-> type(NAME)`.** The command gives a lexer rule's tokens the type
  `NAME`, so a parser rule that asks for `NAME` also takes them. Both
  importers add the rule as one more alternative of `NAME`; if `NAME` is only
  declared in `tokens { ... }`, they define it from the retyped rules. The
  doc of `NAME` lists each rule that retypes into it. Before this, the R
  grammar named tokens that no rule defined. The shared fixture
  [`retype.g4`](../../rust/tests/fixtures/grammar/antlr/retype.g4) checks this
  in both importers.
