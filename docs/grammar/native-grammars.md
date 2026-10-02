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

Status: one catalog language, JSON, has a native merged grammar. Its default
parse still runs tree-sitter-json until the grammar has recovery rules for
invalid input; see [current limits](#current-limits).

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
- A comment is an extra row with flag `X`.
- Leading trivia belongs before the node it precedes.
- A node spans its first to last non-trivia leaf.
- The root starts at its first visible leaf and ends at the end of the input.

[`rust/tests/unit/issue_195_grammar_native_json.rs`](../../rust/tests/unit/issue_195_grammar_native_json.rs)
projects the Rust trees the same way.
[`js/tests/issue-195-grammar-native-json.test.js`](../../js/tests/issue-195-grammar-native-json.test.js)
checks the JavaScript trees and that the oracle still gives the fixture rows.
Both suites record evidence for the `I195-GRAMMAR-NATIVE-JSON` ledger row.

Regenerate the fixtures after changing a grammar or a corpus:

```sh
cd js
node scripts/generate-native-grammar-fixtures.mjs
```

`npm run check:native-grammars` runs the generator with `--check`. It fails
when a fixture is stale, and CI runs it.

## Current limits

- Invalid input is rejected instead of recovered. The default JSON parse still
  uses tree-sitter-json, and `tree-sitter-json` stays a production dependency
  until the native grammar has recovery rules.
- No other catalog language has a native merged grammar yet. The open rows are
  `I195-GRAMMAR-NATIVE-MERGED`, `I195-GRAMMAR-LANGUAGE-CATALOG` and
  `I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED` in the
  [ledger](../issue-195-requirement-ledger.md).
