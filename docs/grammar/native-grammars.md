# Native merged grammars

> This document is subordinate to the authoritative
> [vision and architecture specification](../vision.md). It describes the
> native merged grammars that exist today
> ([native merged grammars](../vision.md#native-merged-grammars)). Where the
> two disagree, the vision is the contract and this page is a defect to fix.

A native merged grammar is a Links Notation grammar in
[`parity/grammars/native/`](../../parity/grammars/native/). It runs on the
native executors with no parser generator: `compileGrammar` in JavaScript and
`compile_feature_grammar` in Rust (see [feature union](feature-union.md)). Each
grammar is checked against the tree-sitter grammar that still backs its
language's default parse. That grammar is an oracle; the native grammar does
not embed it, and no foreign grammar text is stored in the native file.

Status: two catalog languages, JSON and INI, have a native merged grammar.
Their default parses still run tree-sitter-json and tree-sitter-ini until the
grammars have recovery rules for invalid input; see
[current limits](#current-limits).

## Format

Each file holds one link per line, in the canonical form `renderGrammarLinks`
and `render_grammar_links` write:

```text
(grammar (start document))
(extra (class plain (char %20) ...))
(extra (ref comment))
(rule document normal (seq (optional (alias byte_order_mark (immediateToken (literal %EF%BB%BF)))) (repeat0 (ref _value))))
```

`parseGrammarLinks` followed by `renderGrammarLinks` gives back the file byte
for byte. Literals are percent-encoded UTF-8, so a byte order mark is
`%EF%BB%BF`.

## Merging sources

A merged grammar accepts the union of what its sources accept. For JSON the
sources are [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259),
[ECMA-404](https://ecma-international.org/publications-and-standards/standards/ecma-404/)
and [tree-sitter-json 0.24.8](https://github.com/tree-sitter/tree-sitter-json/blob/v0.24.8/grammar.js):

- From tree-sitter-json: comments, a number ending in `.` (`1.`), several
  top-level values, and an escape of fewer than four hex digits (`"\u12"`).
- From RFC 8259: a plus sign in an exponent (`1e+5`), which tree-sitter-json
  0.24.8 rejects.
- A leading byte order mark is a named leaf, `byte_order_mark`, so the tree
  keeps every byte. tree-sitter-json skips it as whitespace; RFC 8259
  section 8.1 lets a parser ignore it.

For INI the sources are
[tree-sitter-ini 1.4.0](https://github.com/justinmk/tree-sitter-ini/blob/v1.4.0/grammar.js)
and the
[Python configparser file structure](https://docs.python.org/3/library/configparser.html#supported-ini-file-structure):

- From tree-sitter-ini: the tree shape. A setting value is everything after
  `=` up to the line feed, leading spaces and a carriage return included, and
  a comment is an extra with a `text` child.
- From configparser: a last comment line without a line break (`; c`), which
  tree-sitter-ini 1.4.0 rejects.
- Blank lines, line breaks and comment markers are named leaves
  (`blank_space`, `newline`, `comment_marker`), so the tree keeps every byte.

## Checking against the oracle

[`js/scripts/generate-native-grammar-fixtures.mjs`](../../js/scripts/generate-native-grammar-fixtures.mjs)
lists each native grammar with its corpus and writes
[`parity/fixtures/native-grammars/<id>.json`](../../parity/fixtures/native-grammars/).
The corpus has three parts:

| Part | Meaning |
| --- | --- |
| `matches` | Sources whose native rows equal the oracle rows. |
| `divergences` | Sources one merged source accepts and the oracle does not. The native grammar accepts them, and the fixture keeps their rows and the reason. |
| `rejections` | Invalid sources. The oracle recovers with error nodes; the native grammar rejects them. |

The generator fails when a match differs from the oracle, when the oracle
accepts a divergence or a rejection, or when the native grammar accepts a
rejection.

Rows have the shape of
[`parity/fixtures/default-cst-expected.json`](../../parity/fixtures/default-cst-expected.json):
`[depth, field, kind, named, startByte, endByte, flags]`.
[`js/scripts/native-grammar-rows.mjs`](../../js/scripts/native-grammar-rows.mjs)
projects a native tree the way tree-sitter places nodes:

- Whitespace trivia and the fixture's `hidden` kinds are not rows.
- The fixture's `anonymous` kinds are not rows but count in the spans, as
  tree-sitter keeps a regular expression token such as a line break.
- A comment is an extra row with flag `X`, and so is a node of the fixture's
  `extras` kinds.
- Leading trivia belongs before the node it precedes.
- A node spans its first to last non-trivia leaf.
- The root starts at its first visible leaf and ends at the end of the input.

[`rust/tests/unit/issue_195_native_grammar_rows.rs`](../../rust/tests/unit/issue_195_native_grammar_rows.rs)
projects the Rust trees the same way. Each language has a JavaScript suite,
which checks the JavaScript trees and that the oracle still gives the fixture
rows, and a Rust suite. Both record evidence for the language's ledger row:

| Language | JavaScript suite | Rust suite | Ledger row |
| --- | --- | --- | --- |
| JSON | [`issue-195-grammar-native-json.test.js`](../../js/tests/issue-195-grammar-native-json.test.js) | [`issue_195_grammar_native_json.rs`](../../rust/tests/unit/issue_195_grammar_native_json.rs) | `I195-GRAMMAR-NATIVE-JSON` |
| INI | [`issue-195-grammar-native-ini.test.js`](../../js/tests/issue-195-grammar-native-ini.test.js) | [`issue_195_grammar_native_ini.rs`](../../rust/tests/unit/issue_195_grammar_native_ini.rs) | `I195-GRAMMAR-NATIVE-INI` |

Regenerate the fixtures after changing a grammar or a corpus:

```sh
cd js
node scripts/generate-native-grammar-fixtures.mjs
```

`npm run check:native-grammars` runs the generator with `--check`. It fails
when a fixture is stale, and CI runs it.

## Current limits

- Invalid input is rejected instead of recovered. The default JSON and INI
  parses still use tree-sitter-json and tree-sitter-ini, which stay production
  dependencies until the native grammars have recovery rules.
- No other catalog language has a native merged grammar yet. The open rows are
  `I195-GRAMMAR-NATIVE-MERGED`, `I195-GRAMMAR-LANGUAGE-CATALOG` and
  `I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED` in the
  [ledger](../issue-195-requirement-ledger.md).
