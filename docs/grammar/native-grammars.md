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
grammar is checked against the pinned tree-sitter grammar it replaced as its
language's default parse. That grammar is an oracle; the native grammar does
not embed it, and no foreign grammar text is stored in the native file.

Status: nine catalog languages, JSON, INI, Diff, CSV, JSON5, Scheme, Racket,
C and Rust, have a native merged grammar, and it is their default parser in
both runtimes; see [default parse](#default-parse). The C grammar is the first
that the [automatic import pipeline](#imported-grammars) writes, and the Rust
grammar the first with a ported native scanner. tree-sitter-json,
tree-sitter-ini, tree-sitter-diff, tree-sitter-csv, tree-sitter-json5-orchard,
tree-sitter-scheme, tree-sitter-racket, tree-sitter-c and tree-sitter-rust
remain as pinned oracles; see [current limits](#current-limits).

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
- From configparser: a last line without a line break (`; c`, `a=1`, `[s]`).
  tree-sitter-ini 1.4.0 rejects a last comment line and completes any other
  last line with a missing line break, a recovery only the has-error flag of
  its root shows.
- Blank lines, line breaks and comment markers are named leaves
  (`blank_space`, `newline`, `comment_marker`), so the tree keeps every byte.

For unified diffs the sources are
[tree-sitter-diff 0.1.0](https://github.com/tree-sitter-grammars/tree-sitter-diff/blob/v0.1.0/grammar.js),
the [GNU diffutils unified format](https://www.gnu.org/software/diffutils/manual/html_node/Detailed-Unified.html)
and the git patch format
([`git diff -p`](https://git-scm.com/docs/git-diff#_generating_patch_text_with_p)):

- From tree-sitter-diff: the tree shape. A `diff` command line opens a
  `block` with its headers (`file_change`, `index`, `similarity`,
  `binary_change`), the `old_file` and `new_file` lines and the `hunks`; a
  hunk is a `location` field and a `changes` field of `addition`, `deletion`
  and `context` lines. Lines outside a block stand alone under the root.
- From GNU diffutils: inside a hunk the first character decides the line. A
  line that starts with a space is context even when a keyword follows
  (` new x`), and a line that starts with `---`, `+++` or `+++i;` is a deleted
  or added line. tree-sitter-diff 0.1.0 reads these as keyword lines and
  recovers.
- From git: an abbreviated object name of four to forty hex digits
  ([`core.abbrev`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreabbrev));
  tree-sitter-diff 0.1.0 needs seven.
- From the sources' line-oriented reading: a block cut at the end of the input,
  with no line break after its last line, which tree-sitter-diff 0.1.0
  completes with a missing one.
- Line breaks and blank lines are `newline` leaves, and the rest of a line is
  an `anything` leaf, as in tree-sitter-diff, so the tree keeps every byte.

For CSV the sources are [RFC 4180](https://www.rfc-editor.org/rfc/rfc4180) and
[tree-sitter-csv](https://github.com/tree-sitter-grammars/tree-sitter-csv/blob/f6bf6e35eb0b95fbadea4bb39cb9709507fcb181/common/define-grammar.js)
at revision `f6bf6e3`, with the RFC 4180 quotes patch the vendored parser
applies ([`NOTICE.md`](../../rust/vendor/tree-sitter-csv/NOTICE.md)):

- From tree-sitter-csv: the tree shape. A `document` holds `row`s of `field`s,
  and a field is a `number` (decimal or `0x` hexadecimal), a `float`, a
  `boolean` (`true` or `false`) or `text`. A typed field must end the field,
  so `1 `, `true1` and `0x` are text, as the oracle's longest-match lexer
  reads them. The spaces before a field belong to its token (` 1` is a
  number), the spaces after a closing quote are skipped, and blank lines are
  not rows.
- From RFC 4180: a quoted field holds commas, line breaks and doubled quotes,
  an unquoted field holds no quote, and the last record may end without a line
  break. An empty last field at the end of the input (`a,`, `,`) is a field of
  empty text; tree-sitter-csv recovers from it.
- A run of line breaks is one `newline` leaf, and the spaces after a closing
  quote are a `blank_space` leaf, so the tree keeps every byte.

For JSON5 the sources are the [JSON5 1.0.0 specification](https://spec.json5.org/)
and
[tree-sitter-json5-orchard 0.1.0](https://docs.rs/crate/tree-sitter-json5-orchard/0.1.0/source/grammar.js):

- From tree-sitter-json5-orchard: the tree shape. A `file` holds one value; an
  `object` holds `member`s with a `name` field (a `string` or an
  `identifier`) and a `value` field; strings, numbers, `null`, `true` and
  `false` are leaves; comments are extras. Both quote styles, trailing commas,
  hexadecimal, signed, `Infinity` and `NaN` numbers, and a raw line break in a
  string, which the specification does not allow, are accepted. A number may
  be `.` or `.e5`, as in the oracle.
- From JSON5 section 8 (White Space): NBSP, LS, PS, a byte order mark anywhere
  and every Zs space are whitespace. tree-sitter-json5-orchard 0.1.0 skips
  only the ASCII spaces, and a leading byte order mark.
- From JSON5 section 3 (Objects): a member name is an ECMAScript 5.1
  IdentifierName, so it may hold `\u` escapes, Nl letters, and Mn, Mc, Nd,
  Pc, ZWNJ and ZWJ characters after its first one.
- From JSON5 section 5.1 (Escapes): `\0` before a non-digit, a backslash
  before any character other than a digit, `x` or `u`, and a line
  continuation after a lone CR, LS or PS.
- Whitespace and comments are trivia leaves, so the tree keeps every byte.

For Scheme the sources are the
[R7RS small report](https://small.r7rs.org/attachment/r7rs.pdf) and
[tree-sitter-scheme 0.24.7](https://docs.rs/crate/tree-sitter-scheme/0.24.7/source/grammar.js):

- From tree-sitter-scheme: the tree shape and the lexical rules. A `program`
  holds data; a `list` may use `()`, `[]` or `{}`; `vector`, `byte_vector`
  (R6RS `#vu8(`), `quote`, `quasiquote`, `unquote`, `unquote_splicing`,
  `syntax`, `quasisyntax`, `unsyntax` and `unsyntax_splicing` hold their
  datum. `boolean`, `character`, `number`, `symbol` and `keyword` are leaves
  of the R5RS, R6RS and R7RS spellings the oracle unions, and a string holds
  `escape_sequence` rows. Line comments, `#;` datum comments, nested `#| |#`
  block comments and `#!` directives are rows between data, as the oracle
  declares no extras. Where a number and a symbol
  both match, the longer one wins and the number wins a tie, as the oracle's
  lexer reads `1#a` (a number and a symbol) and `1abc` (a symbol).
- From R7RS small sections 6.9 (Bytevectors) and 7.1.2 (External
  representations): a `#u8(` bytevector is a `byte_vector`;
  tree-sitter-scheme 0.24.7 reads only `#vu8(`.
- From R7RS small section 2.4 (Datum labels): `#<n>=<datum>` is a
  `datum_label` holding a `label` and its datum, and `#<n>#` is a
  `datum_reference`; tree-sitter-scheme 0.24.7 recovers from both.
- Whitespace, comment text, string text and directive names are trivia
  leaves, so the tree keeps every byte.

For Racket the sources are the
[reader chapter of the Racket Reference](https://docs.racket-lang.org/reference/reader.html)
and
[tree-sitter-racket 0.25.0](https://docs.rs/crate/tree-sitter-racket/0.25.0/source/grammar.js):

- From tree-sitter-racket: the tree shape and the lexical rules. A `program`
  holds data and may start with a byte order mark, which has no row; a `list`
  may use `()`, `[]` or `{}` and holds `dot` rows; `vector` (with the `#fl`
  and `#fx` prefixes and a length), `structure`, `hash`, `box`, `graph`,
  `quote`, `quasiquote`, `unquote`, `unquote_splicing`, `syntax`,
  `quasisyntax`, `unsyntax` and `unsyntax_splicing` hold their datum.
  `boolean`, `character`, `number` (extflonums included), `symbol` and
  `keyword` are leaves; a string or byte string holds `escape_sequence` rows,
  and `regex` holds its string. Line comments, `#;` datum comments, nested
  `#| |#` block comments and `#lang`, `#!` and `#reader` extensions are rows
  between data, as the oracle declares no extras. Where a number and a symbol
  both match, the longer one wins and the number wins a tie.
- A `here_string` replaces the oracle's external scanner with grammar
  actions: `here_terminator` stores the rest of the `#<<` line in a state
  variable, `here_line` fails on a line equal to it and `here_end` matches
  only that line, as Racket Reference section 1.3.7 (Reading Strings) reads a
  here string.
- From Racket Reference section 1.3.14 (Reading Characters): `#\` followed by
  a line feed is a `character`; tree-sitter-racket 0.25.0 reads no line feed
  after `#\` and recovers from it.
- From Racket Reference sections 1.3.1 (Delimiters and Dispatch) and 1.3.2
  (Reading Symbols): a backslash quotes the next character of a symbol or
  keyword, a line feed included; tree-sitter-racket 0.25.0 quotes any
  character but a line feed and recovers from it.
- Whitespace, comment text, string text and the here string lines are trivia
  leaves, so the tree keeps every byte.

## Imported grammars

[`js/scripts/import-native-grammars.mjs`](../../js/scripts/import-native-grammars.mjs)
is the automatic merge pipeline. It reads each pinned upstream grammar of
[`parity/grammars/sources.json`](../../parity/grammars/sources.json), which
records the package, repository, revision, path, license and SHA-256 of the
gzipped copy under
[`parity/grammars/sources/`](../../parity/grammars/sources/), and fails when
a copy does not match its hash. For each grammar it:

- translates the tree-sitter `grammar.json` into an executable native grammar
  with `importTreeSitterNative`
  ([`tree-sitter-native.js`](../../js/src/grammar-importers/tree-sitter-native.js)):
  every pattern becomes native classes, sequences, choices and repeats, and
  precedence, associativity, dynamic precedence, tokens, fields, aliases,
  extras, conflicts and the word rule keep their tree-sitter meaning in the
  executor (see [feature union](feature-union.md#executor));
- renames every rule to readable English through
  [`parity/naming/grammar-name-expansions.json`](../../parity/naming/grammar-name-expansions.json),
  which holds each word replacement and each name or concept a reviewer
  decided by hand with its reason; the tree-sitter name stays as a
  `source-names` alias;
- gives every rule a concept record, shared by name with the other native
  grammars only where the construct means the same;
- writes the merge report
  [`parity/grammars/merge-reports/<language>.json`](../../parity/grammars/merge-reports/):
  the sources, the rule count, the keywords, every rename and expanded word,
  the shared and generated concepts, and the constructs it approximated or
  could not import.

A hand edit of an imported grammar is a decision in the expansions file, not
an edit of the generated `.lino`. `npm run check:native-imports` runs the
pipeline with `--check` and fails on any drift, and CI runs it.

For C the source is the `src/grammar.json` of
[tree-sitter-c 0.24.2](https://github.com/tree-sitter/tree-sitter-c/blob/b780e47fc780ddc8da13afa35a3f4ed5c157823d/src/grammar.json)
(crates.io tree-sitter-c 0.24.2, revision
`b780e47fc780ddc8da13afa35a3f4ed5c157823d`, MIT):

- [`parity/grammars/native/c.lino`](../../parity/grammars/native/c.lino) has
  183 rules, 106 keywords and the 17 conflicts tree-sitter-c declares. The
  [merge report](../../parity/grammars/merge-reports/c.json) lists 93
  renamed rules and 24 expanded words, such as `preproc_include` to
  `preprocessor_include` and `sizeof` to `size_of`, and no approximated or
  unsupported construct. tree-sitter-c has no external scanner.
- A keyword is lexed wherever the parse admits it, as the tree-sitter lexer
  does, so `typedef` is an identifier only where no keyword may stand
  (`x = typedef;`); `typedef;` and `if;` are rejected
  ([keyword lexing](feature-union.md#executor)).
- The oracle's LR conflicts are settled as its generated parser settles them:
  a conflict between silent rules is not an ambiguity (`a;` is an expression
  statement), and a shift/reduce choice follows precedence and associativity
  (a case statement keeps the statements after it). Both executors parse every
  corpus source with no ambiguity.
- [`parity/fixtures/native-grammars/c.json`](../../parity/fixtures/native-grammars/c.json)
  holds 100 matches (the 85 cases of the upstream `test/corpus` at the same
  revision and 15 more), no divergence and 30 rejections.
- The ISO C standard is not a merged source yet: the grammar accepts what
  tree-sitter-c accepts, GNU and Microsoft extensions included.
- Whitespace, line continuations, comments, string text and the line breaks
  of preprocessor directives are leaves, so the tree keeps every byte.

## Concepts

Every rule has a readable English name and names a canonical concept record
in [`parity/naming/canonical-concepts.json`](../../parity/naming/canonical-concepts.json)
with `(concept ID)`. A rule that replaces a differently named tree-sitter kind
keeps that name as `(source-names (tree-sitter NAME))`, and the catalog's
`oracleKinds` maps the rule back to it for the oracle comparison. The record
lists the rule as a `native:<language>` source alias. A node kind that only an
alias names, such as tree-sitter-rust's `doc_comment`, has no rule to carry its
source name. It keeps that name in a
`(kind documentation_comment (source-names (tree-sitter doc_comment)))` link
before the rules, and `oracleKinds` maps it back the same way.

A construct that means the same in two or more native grammars names one
shared concept: a JSON `pair` and a JSON5 `member` are both `grammar.member`,
and a string is `grammar.string` in CSV, JSON, JSON5, Scheme, Racket and C.
Lookalikes stay apart. A Scheme `list` is `grammar.linked-list`, not the
`grammar.list` of a JSON `array`. A JSON `object` is `grammar.object`, not the
`grammar.racket.hash-table` of a Racket `hash`. A construct only one language has
names a concept in that language's namespace, such as `grammar.diff.hunk`.
`checkNativeGrammarConcepts` and `check_native_grammar_concepts` fail on any
rule without a record, and on any record alias without a rule.

`translateNativeConstruct` and `translate_native_construct` translate a rule
from one native grammar to another through its concept record alone; no rule
exists for the pair of languages. `translateNativeConstructTree` and
`translate_native_construct_tree` translate a whole construct tree, or report
every construct the target grammar has no rule for. The per-language counts of
shared and language-specific rules are in the generated
[concept reuse report](native-grammar-concept-reuse.md), which
`npm run check:concept-reuse` keeps current. The fixtures are
[`parity/fixtures/grammar-shared-concepts.json`](../../parity/fixtures/grammar-shared-concepts.json)
and
[`parity/fixtures/native-grammar-concept-reuse.json`](../../parity/fixtures/native-grammar-concept-reuse.json).

## Checking against the oracle

[`js/scripts/generate-native-grammar-fixtures.mjs`](../../js/scripts/generate-native-grammar-fixtures.mjs)
lists each native grammar with its corpus and writes
[`parity/fixtures/native-grammars/<id>.json`](../../parity/fixtures/native-grammars/).
The corpus has three parts:

| Part | Meaning |
| --- | --- |
| `matches` | Sources whose native rows equal the oracle rows. |
| `divergences` | Sources one merged source accepts and the oracle does not. The native grammar accepts them, and the fixture keeps their rows and the reason. |
| `rejections` | Invalid sources. The oracle recovers with error nodes; the native grammar rejects them, and with `errorRecovery` repairs each into its `recovered` tree. |

The generator fails when a match differs from the oracle or the oracle
recovers from it, when the oracle accepts a divergence or a rejection, or when
the native grammar accepts a rejection or does not recover from it. The
oracle recovers when a row has an error or missing flag, or when the root has
its has-error flag: tree-sitter can insert a missing anonymous token, such as
a line break at the end of the input, that no row shows. `oracleRecovers` in
[`js/scripts/native-grammar-rows.mjs`](../../js/scripts/native-grammar-rows.mjs)
checks both.

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
| Diff | [`issue-195-grammar-native-diff.test.js`](../../js/tests/issue-195-grammar-native-diff.test.js) | [`issue_195_grammar_native_diff.rs`](../../rust/tests/unit/issue_195_grammar_native_diff.rs) | `I195-GRAMMAR-NATIVE-DIFF` |
| CSV | [`issue-195-grammar-native-csv.test.js`](../../js/tests/issue-195-grammar-native-csv.test.js) | [`issue_195_grammar_native_csv.rs`](../../rust/tests/unit/issue_195_grammar_native_csv.rs) | `I195-GRAMMAR-NATIVE-CSV` |
| JSON5 | [`issue-195-grammar-native-json5.test.js`](../../js/tests/issue-195-grammar-native-json5.test.js) | [`issue_195_grammar_native_json5.rs`](../../rust/tests/unit/issue_195_grammar_native_json5.rs) | `I195-GRAMMAR-NATIVE-JSON5` |
| Scheme | [`issue-195-grammar-native-scheme.test.js`](../../js/tests/issue-195-grammar-native-scheme.test.js) | [`issue_195_grammar_native_scheme.rs`](../../rust/tests/unit/issue_195_grammar_native_scheme.rs) | `I195-GRAMMAR-NATIVE-SCHEME` |
| Racket | [`issue-195-grammar-native-racket.test.js`](../../js/tests/issue-195-grammar-native-racket.test.js) | [`issue_195_grammar_native_racket.rs`](../../rust/tests/unit/issue_195_grammar_native_racket.rs) | `I195-GRAMMAR-NATIVE-RACKET` |
| C | [`issue-195-grammar-native-c.test.js`](../../js/tests/issue-195-grammar-native-c.test.js) | [`issue_195_grammar_native_c.rs`](../../rust/tests/unit/issue_195_grammar_native_c.rs) | `I195-GRAMMAR-NATIVE-C` |
| Rust | [`issue-195-grammar-native-rust.test.js`](../../js/tests/issue-195-grammar-native-rust.test.js) | [`issue_195_grammar_native_rust.rs`](../../rust/tests/unit/issue_195_grammar_native_rust.rs) | `I195-GRAMMAR-NATIVE-RUST` |

Regenerate the fixtures after changing a grammar or a corpus:

```sh
cd js
node scripts/generate-native-grammar-fixtures.mjs
```

`npm run check:native-grammars` runs the generator with `--check`. It fails
when a fixture is stale, and CI runs it.

## Default parse

The language catalog lists the native grammar first in a native language's
`grammars`, with the SHA-256 of its file as `parserSha256`, and its
tree-sitter oracles in `oracleGrammars`. The catalog's top-level `nativeGrammars` gives each native
grammar's file, a copy of the `parity/grammars/native/` file under
`src/data/native-grammars/` in both packages, and its `hidden`, `anonymous`
and `extras` kinds. `parseNative` in
[`js/src/native-grammar-parser.js`](../../js/src/native-grammar-parser.js) and
`parse_native` in
[`rust/src/native_grammar_parser.rs`](../../rust/src/native_grammar_parser.rs)
run the grammar with automatic error recovery and project the tree the way
the fixture rows above do, so `parseProgrammingLanguage` and
`parse_programming_language` build the same lossless network they built from
the oracle. An embedded region in a native language and an incremental
reparse use the native grammar too; an incremental reparse parses the edited
text again.

[`parity/fixtures/native-default-cst-expected.json`](../../parity/fixtures/native-default-cst-expected.json)
holds each native language's default CST rows. For valid input the rows and
digests equal those of the oracle in
[`parity/fixtures/builtin-cst-expected.json`](../../parity/fixtures/builtin-cst-expected.json);
for invalid input they hold the native repair, which
[`default-cst-expectations.test.js`](../../js/tests/default-cst-expectations.test.js)
and
[`default_cst_expectations.rs`](../../rust/tests/unit/default_cst_expectations.rs)
check in both runtimes. `node scripts/generate-native-grammar-fixtures.mjs`
writes the file with the per-grammar fixtures. Its `embeddedFixtures` give
the rows of the regions in a native language of the shared embedded-language
fixtures of
[`parity/fixtures/default-cst-expected.json`](../../parity/fixtures/default-cst-expected.json):
the oracle's rows for valid input and the native repair for invalid input, at
the bounds the host grammar places. The Markdown fenced Rust region of the
recovery fixture, `fn answer() -> u32 { 42`, is repaired with a MISSING `}`.

## Current limits

- Invalid input is rejected by default. With `errorRecovery` the executor
  repairs it ([automatic recovery](feature-union.md#executor)), and every
  rejection in a fixture records its `recovered` tree, which
  [`issue-195-grammar-native-recovery.test.js`](../../js/tests/issue-195-grammar-native-recovery.test.js)
  and
  [`issue_195_grammar_native_recovery.rs`](../../rust/tests/unit/issue_195_grammar_native_recovery.rs)
  check. The default parse uses these repairs, so the ERROR and MISSING
  nodes of an invalid input are placed where the native executor repairs it,
  not where tree-sitter's LR recovery places them; the two are not compared.
- tree-sitter-json, tree-sitter-ini, tree-sitter-diff, tree-sitter-csv,
  tree-sitter-json5-orchard, tree-sitter-scheme, tree-sitter-racket and
  tree-sitter-c no longer back a default parse, so they are development files only: the Rust
  crates are `[dev-dependencies]` that the `pinned_*_oracle_gives_the_fixture`
  tests load, the vendored CSV parser is no longer compiled into or published
  with the crate, and their WebAssembly builds and licenses are in
  [`js/oracles/grammars`](../../js/oracles/grammars/NOTICE.md), outside the npm
  package, where the fixture generator and the oracle checks load them. Their
  lock entries carry `"oracle": true`, and `grammarNames`/`grammar_names` and
  `grammar_by_id` return nothing for their ids.
- tree-sitter-rust 0.24.2, patched for the oracle and vendored under
  `rust/vendor/tree-sitter-rust`, is still compiled into the Rust crate, and
  its WebAssembly build is still in the npm package, so it is not yet a
  development file like the oracles above; the Rust suite checks the
  executor against the fixture rows, not against the loaded oracle.
- Some inputs the diff oracle reads with its LR recovery are outside the
  corpus, because no source decides them: a NUL byte in a line, which
  tree-sitter-diff recovers from and the native grammar accepts as context; a
  keyword line with leading spaces outside a hunk (` new file mode 1`), which
  the oracle reads as a keyword line and the native grammar as context; and a
  block cut after a bare `---` or after `--- a`, where the oracle recovers and
  the native grammar reads the lines on their own.
- A boolean after leading spaces (` true`) is outside the CSV corpus: the
  oracle reads it as a `boolean` whose keyword spans the spaces too, which the
  native grammar's tree cannot show, and the native grammar reads it as text,
  as RFC 4180 section 2.4 counts spaces as part of a field.
- A JSON5 line comment ends only at a line feed, as in
  tree-sitter-json5-orchard 0.1.0; a lone CR, LS or PS stays in the comment.
  JSON5 section 7 ends it at any ECMAScript 5.1 LineTerminator, which would
  split the oracle's comment rows, so inputs that differ there are outside
  the corpus.
- The tree-sitter-racket 0.25.0 here string scanner packs each character in
  32 bits and compares a line with the terminator by `strcmp`, so it compares
  only the first character: it accepts `#<<EOF\nab\nEOFx`, ending the here
  string at `EOFx`. The native grammar ends a here string only at a line
  equal to its terminator, as Racket Reference section 1.3.7 requires, and
  rejects these inputs; since the oracle accepts them they are not
  rejections, and they are outside the corpus. The Racket suites check that
  the native grammar rejects them.
- The native Racket grammar follows the oracle where it accepts input the
  Racket Reference reads differently, so those inputs stay matches: `#Fl(`
  and `#Fx(` read as the boolean `#F` followed by a symbol and a list rather
  than an flvector or fxvector, and `#\nx` reads as the character `#\n`
  followed by the symbol `x`, where section 1.3.14 reads no character whose
  letter is followed by another letter.
- The C grammar has one source, tree-sitter-c; inputs the C standard and
  tree-sitter-c read differently are outside the corpus until the standard
  is merged.
- No other catalog language has a native merged grammar yet. The open rows are
  `I195-GRAMMAR-NATIVE-MERGED`, `I195-GRAMMAR-LANGUAGE-CATALOG` and
  `I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED` in the
  [ledger](../issue-195-requirement-ledger.md).
