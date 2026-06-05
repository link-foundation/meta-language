# Competitor & Reference Test-Suite Survey

> Compiled 2026-06-05 from live inspection of each upstream repository (GitHub
> Contents API + raw file fetches). Every claim is paired with a source URL.
> This document supports the requirement in
> [issue #3](https://github.com/link-foundation/meta-language/issues/3) to
> "copy all the tests from competitors and make sure we support all the features
> similar projects already support."

The crate's founding vision ([issue #1](https://github.com/link-foundation/meta-language/issues/1))
names six lossless-CST / transform projects as feature suites to satisfy. This
survey records where each project's tests live, the test format, the license
(so adapted test data is license-compatible), and a concrete adaptation plan.

## 1. tree-sitter — `github.com/tree-sitter/tree-sitter`

- **License:** MIT (core) — <https://github.com/tree-sitter/tree-sitter/blob/master/LICENSE>.
  Grammar repos are individually MIT — <https://github.com/tree-sitter/tree-sitter-python/blob/master/LICENSE>.
- **Core engine tests (Rust):** `crates/cli/src/tests/` — `corpus_test.rs`,
  `parser_test.rs`, `query_test.rs` (the largest, ~182 KB), `highlight_test.rs`,
  `tags_test.rs`, `node_test.rs`, `tree_test.rs`, `pathological_test.rs`, etc.
  <https://github.com/tree-sitter/tree-sitter/tree/master/crates/cli/src/tests>
- **Engine fixtures:** `test/fixtures/` with `error_corpus/` (`c_errors.txt`,
  `javascript_errors.txt`, `python_errors.txt`, …), `template_corpus/`,
  `grammars/`. <https://github.com/tree-sitter/tree-sitter/tree/master/test/fixtures>
- **Per-grammar corpus tests** (the bulk of real tests) live in each grammar repo
  under `test/corpus/*.txt`. JS: `statements.txt`, `expressions.txt`,
  `destructuring.txt`, `literals.txt`, `semicolon_insertion.txt`, `injectables.txt`;
  highlight tests `test/highlight/*.js`; queries `queries/*.scm`
  (`highlights.scm`, `injections.scm`, `locals.scm`, `tags.scm`).
  <https://github.com/tree-sitter/tree-sitter-javascript/tree/master/test>

**Corpus format (verified, exact):**

```
============================================
Test Name
============================================

<source code>

---

(program
  (expression_statement
    (assignment_expression
      (identifier)
      (number))))
```

Header attributes start with `:` — including `:error`, `:skip`, `:fail-fast`,
`:platform(...)`, `:language(LANG)`, and `:cst` (emit the full CST rather than an
abstract S-expression). Run with `tree-sitter test`; auto-update with
`tree-sitter test -u`.
<https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests.html>

**Highlight test format (verified):** Sublime-style assertions in comments using
`<-` (assert at the comment column) and `^` (assert at the caret column).
<https://raw.githubusercontent.com/tree-sitter/tree-sitter-javascript/master/test/highlight/keywords.js>

**Key features to match:** incremental GLR parsing; lossless concrete syntax with
byte ranges and row/column; robust error recovery producing `ERROR`/`MISSING`
nodes; the S-expression query language with captures `@name` and predicates
(`#eq?`, `#match?`); language injection / mixed-language embedding;
highlighting and tags built atop queries. These map directly onto the crate's
`LosslessParsing`, `ErrorRecovery`, `MixedLanguageRegions`, and `QueryMatching`
parity capabilities.

**Adaptation:** Take the first case in JS `statements.txt` (`a = 0;\nvar b = 0;`):
assert `network.reconstruct_text() == input` byte-for-byte and
`verify_full_match().is_clean()`; port `(identifier) @x` into a `LinkQuery` for
link type `identifier`; adapt an `error_corpus` truncation (`if (`) to assert an
error/missing link is produced and survives round-trip; add a `var`→`let`
`SubstitutionRule`.

## 2. LibCST — `github.com/Instagram/LibCST`

- **License:** MIT — <https://github.com/Instagram/LibCST/blob/main/LICENSE>.
  Caveat: files under `libcst/_parser/parso/...` are dual MIT/PSF; prefer copying
  test data from `libcst/_nodes/tests/` (plain MIT).
- **Tests:** `libcst/_nodes/tests/` (one file per node kind — `test_funcdef.py`
  ~97 KB, `test_atom.py`, `test_lambda.py`, `test_import.py`, `test_try.py`,
  `test_match.py`, …); `libcst/_parser/tests/` (`test_parse_errors.py`,
  `test_whitespace_parser.py`, …); `libcst/tests/` (`test_roundtrip.py`,
  `test_fuzz.py`, `test_e2e.py`, …). Round-trip fixtures are real `.py` files at
  `native/libcst/tests/fixtures/`.
  <https://github.com/Instagram/LibCST/tree/main/libcst/_nodes/tests>
- **Canonical round-trip (verified):** `test_roundtrip.py` parses each fixture and
  asserts `self.assertEqual(mod.code, src)` — regenerated code equals the original
  bytes — then runs a no-op transformer and re-asserts equality.
  <https://raw.githubusercontent.com/Instagram/LibCST/main/libcst/tests/test_roundtrip.py>

**Key features to match:** fully lossless Python CST where every whitespace/comment
is an explicit node (`EmptyLine`, `TrailingWhitespace`, `Comment`,
`SimpleWhitespace`); `module.code` regenerates source exactly; visitor/transformer
API; a metadata provider system; a codemod framework. Maps to `LosslessParsing`,
`TriviaPreservation`, `SameLanguageReconstruction`.

**Adaptation:** lift a comment + blank-line input and assert
`reconstruct_text() == input` under all three `ParseConfiguration` trivia
policies; replicate the no-op transformer identity, then a real parameter rename
asserting only the targeted bytes change.

## 3. Recast — `github.com/benjamn/recast`

- **License:** MIT — <https://github.com/benjamn/recast/blob/master/package.json>.
- **Tests:** Mocha + Node `assert`, under `test/` (TypeScript) — `printer.ts`
  (~70 KB, the core), `comments.ts`, `lines.ts`, `typescript.ts`, `babel.ts`,
  `parens.ts`, `jsx.ts`, `identity.ts`, … plus a `test/data/` fixtures dir.
  <https://github.com/benjamn/recast/tree/master/test>
- **Canonical round-trip (verified):** `test/identity.ts` does
  `recast.parse(source)` → `recast.print(ast).code` and asserts
  `assert.strictEqual(source, code)`, plus
  `types.astNodesAreEquivalent.assert(ast.original, ast)`.
  <https://raw.githubusercontent.com/benjamn/recast/master/test/identity.ts>

**Key feature to match:** recast reprints *only modified subtrees*, leaving
untouched code byte-identical (it tracks `.original` and reuses original source
for unchanged nodes). This is exactly the crate's `SameLanguageReconstruction`
("without losing unchanged regions").

**Adaptation (the differentiating transform):** parse `const a = 1; const b = 2;`,
apply a `SubstitutionRule` changing only `1`→`42`, assert output equals input with
just those bytes changed (all other bytes, including whitespace, preserved). This
is the strictest cross-language transform guarantee.

## 4. jscodeshift — `github.com/facebook/jscodeshift`

- **License:** MIT — <https://github.com/facebook/jscodeshift/blob/main/package.json>.
- **Tests:** Jest with `__tests__` + `__testfixtures__` input/output pairs and
  snapshots — `src/__tests__/` (`Collection-test.js`, `core-test.js`,
  `matchNode-test.js`, `template-test.js`, …) and `sample/` (the
  `reverse-identifiers.js` end-to-end codemod). `src/testUtils.js` exposes
  `defineTest`/`applyTransform` that diff `<name>.input.js` against
  `<name>.output.js`.
  <https://github.com/facebook/jscodeshift/tree/main/src/__tests__>

**Key features to match:** the codemod model — select nodes via a jQuery-like
`Collection` API (`find`/`filter`/`replaceWith`), transform, re-serialize
(delegating to recast for format preservation). Maps to `QueryMatching` +
`TransformBySubstitution` + `SameLanguageReconstruction`.

**Adaptation:** the input/output fixture pair is the template for a transform
fixture — input `const foo = bar;`, transform = identifier-reversal
`SubstitutionRule`, expected output `const oof = rab;`; assert the `LinkQuery`
for identifier links finds exactly `foo`, `bar`, and that surrounding bytes are
preserved.

## 5a. Rowan — `github.com/rust-analyzer/rowan`

- **License:** dual Apache-2.0 / MIT —
  <https://raw.githubusercontent.com/rust-analyzer/rowan/master/README.md>.
- **Tests:** `tests/` holds only `tidy.rs`. The functional "tests" are runnable
  examples with inline `#[test]`s: `examples/s_expressions.rs` (~12.6 KB, a full
  lossless S-expression parser) and `examples/math.rs`.
  <https://github.com/rust-analyzer/rowan/tree/master/examples>
- **Verified API:** `src/lib.rs` re-exports `GreenNode`, `GreenNodeData`,
  `GreenToken`, `GreenNodeBuilder`, and `Checkpoint` — confirming the green/red
  split, the `start_node`/`token`/`finish_node` builder, and `checkpoint()` for
  retroactively wrapping nodes (`start_node_at`).

**Key features to match:** immutable green trees (deduplicated, position-independent)
+ lazily-built red trees (parent pointers, absolute offsets); full-fidelity
round-trip; `checkpoint()` for left-recursion/precedence wrapping. Maps to
`LosslessParsing`, `TriviaPreservation`, `SameLanguageReconstruction`, and (via
immutable green nodes) `SnapshotVersioning`.

**Adaptation:** port `s_expressions.rs` — feed `(+ 1 (* 2 3))` with original
whitespace, assert `reconstruct_text()` is byte-identical; take a `NetworkSnapshot`,
fork a `MutableNetworkSnapshot`, edit one atom, commit, and assert the original
snapshot still reconstructs the old bytes while the fork reconstructs the new.

## 5b. cstree — `github.com/domenicquirl/cstree`

- **License:** dual Apache-2.0 / MIT — <https://github.com/domenicquirl/cstree>.
- **Tests:** a dedicated `test_suite/` workspace crate — `test_suite/tests/`
  contains `derive.rs`, `ui.rs`, and a trybuild `ui/` directory; additional unit
  tests inline in the `cstree/` crate.
  <https://github.com/domenicquirl/cstree/tree/master/test_suite/tests>
- **Verified differentiators (README):** a fork of rowan with (1) a persistent/cached
  red tree, (2) `Send + Sync` red nodes, (3) reference-returning traversal,
  (4) no mutability API, (5) `#[no_std]` support, (6) string interning so identical
  identifier text shares storage.
  <https://raw.githubusercontent.com/domenicquirl/cstree/master/README.md>

**Adaptation:** same lossless parse→reconstruct fixture as rowan, plus an
interning/identity assertion — parse `foo foo foo` and assert the three
occurrences resolve to the same interned term/link (mirrors the crate's
"field labels as explicit links" / dedup design).

## 6. Roslyn — `github.com/dotnet/roslyn`

- **License:** MIT — <https://github.com/dotnet/roslyn/blob/main/License.txt>.
- **Tests:** xUnit under `src/Compilers/`. C# parsing tests in
  `src/Compilers/CSharp/Test/Syntax/Parsing/` — **71 files**, many enormous
  (`DeclarationParsingTests.cs` ~840 KB, `CollectionExpressionParsingTests.cs`
  ~733 KB, `ExpressionParsingTests.cs` ~394 KB). Tree-shape DSL utilities in
  `src/Compilers/Test/Utilities/CSharp/CSharpTestBase.cs`.
  <https://github.com/dotnet/roslyn/tree/main/src/Compilers/CSharp/Test/Syntax/Parsing>
- **Test format (verified):** `[Fact]` methods use `UsingExpression(...)` /
  `UsingTree(...)` to parse, then a nested sequence of `N(SyntaxKind.X)` calls
  walks and asserts the exact node/token sequence, terminated by `EOF()`. Round-trip
  via `Assert.Equal(text, expr.ToString())`; clean-parse via
  `Assert.Equal(0, expr.Errors().Length)`.
  <https://github.com/dotnet/roslyn/blob/main/src/Compilers/CSharp/Test/Syntax/Parsing/ExpressionParsingTests.cs>

**Key features to match:** full-fidelity syntax trees where `ToFullString()`
reproduces the exact original text (including leading/trailing trivia); first-class
trivia attached to tokens; rich diagnostics with error nodes / missing-token
recovery. Maps to `LosslessParsing`, `TriviaPreservation`, `ErrorRecovery`,
`SameLanguageReconstruction`, and `SelfDescription` (the `SyntaxKind` enum ≈
self-described link types).

**Adaptation:** take a trivia-heavy expression (interpolated/verbatim string),
assert `reconstruct_text() == input` (the `ToFullString()` analog); translate the
`N(SyntaxKind.X)` sequence into ordered `LinkQuery` assertions over projected CST
link types; adapt a `_MissingIdentifiers` case to assert an `is_missing`/`is_error`
link is produced and survives reconstruction.

## Test-adoption strategy (four recurring pillars)

Four patterns recur across **every** project and should become the structural
pillars of `PARITY_FIXTURES` / `LANGUAGE_FIXTURES`:

1. **Lossless round-trip (universal).** Every project has one canonical assertion
   that *parse → serialize == original bytes*: tree-sitter's `:cst` corpus output,
   LibCST `assertEqual(mod.code, src)`, Recast `assert.strictEqual(source, code)`,
   rowan/cstree red-tree text, Roslyn `ToFullString()`. Make
   `reconstruct_text() == input` the mandatory first assertion of every fixture,
   paired with `verify_full_match().is_clean()`.
2. **Trivia made explicit.** LibCST (`EmptyLine`/`Comment` nodes), Recast (comment
   re-attachment), rowan/cstree (trivia tokens in the green tree), Roslyn (leading/
   trailing trivia) all preserve whitespace/comments as first-class data. Each
   fixture should include a comment + blank-line case under all three
   `ParseConfiguration` trivia policies.
3. **Error recovery / partial trees.** tree-sitter (`error_corpus`, `:error`),
   Roslyn (`_MissingIdentifiers`, error nodes), LibCST (`test_parse_errors.py`),
   cstree (incomplete/erroneous trees) all test malformed input. Include negative
   fixtures asserting `is_error`/`is_missing`/`has_error` links are produced *and*
   that reconstruction still round-trips the broken source.
4. **Query-match + transform (codemod).** tree-sitter's S-expr queries +
   jscodeshift's `Collection.find/replaceWith` + Roslyn's `N(SyntaxKind)` walk all
   express "select nodes structurally"; jscodeshift/Recast/LibCST all express
   "transform and re-serialize preserving unchanged bytes." Pair every fixture with
   (a) a `LinkQuery` selecting a target link, and (b) a `SubstitutionRule` that
   rewrites only that target and asserts every other byte is preserved.

**Licensing note for copying tests:** all six upstreams are permissively licensed
(tree-sitter MIT, LibCST MIT, Recast MIT, jscodeshift MIT, rowan + cstree dual
Apache-2.0/MIT, Roslyn MIT), so adapting their test *inputs and expected outputs*
into the crate's fixtures is compatible with this repository's `Unlicense`. Caveat:
within LibCST, prefer `libcst/_nodes/tests/` (plain MIT) over
`libcst/_parser/parso/...` (dual MIT/PSF). When copying verbatim test data, retain
a short provenance comment with the upstream path and license.
