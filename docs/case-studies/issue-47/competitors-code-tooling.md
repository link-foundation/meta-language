# Competitor Case Study: Source-Code Syntax Tooling

Research date: **2026-06-10**. Scope: tools that parse, query, and transform source code syntax — the
closest competitors to meta-language's lossless links network with CST/AST/semantic projections,
`LinkQuery`, and `find()`/`replace()` transforms.

Method: every repository below was inspected via the GitHub API on 2026-06-10 (license, activity,
directory layout of the test corpus); doc claims were checked against the projects' official
documentation. Facts that could not be verified are marked **(unverified)**.

Legend for each section: (a) what it is / primary languages, (b) read-only vs mutable syntax model,
(c) lossless / trivia preservation, (d) query language, (e) transform/codemod support,
(f) multi-language breadth, (g) extensibility, (h) license, (i) test-suite location and shape,
(j) standout feature meta-language lacks.

---

## tree-sitter

- (a) Incremental parsing system for programming tools, written in Rust/C (core) with grammars
  compiled from JS DSL to C. Repo: <https://github.com/tree-sitter/tree-sitter> (25.7k stars, active
  as of 2026-06-10).
- (b) **Read-only** CST. Trees are immutable; "editing" means telling the parser about source edits
  and re-parsing incrementally — there is no tree mutation API.
- (c) Lossless in the sense that every byte is covered by the tree (named + anonymous nodes, `extra`
  nodes for comments/whitespace handled by grammar), with `ERROR`/`MISSING` recovery nodes. The tree
  stores spans into the original text rather than owning text.
- (d) **Tree-sitter query language**: S-expression patterns with field names (`left:`), negated
  fields (`!field`), wildcards (`_`/`(_)`), quantifiers, anchors, captures (`@name`), predicates
  (`#eq?`, `#match?`), and `(ERROR)`/`(MISSING)` matching. Docs:
  <https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html>.
- (e) No transform support — queries are match-only; rewriting is left to consumers (ast-grep,
  difftastic, editors).
- (f) Hundreds of community grammars (one repo per language under the `tree-sitter` org and beyond).
- (g) Extensible by writing a new grammar (`grammar.js` + optional external C scanner for
  context-sensitive lexing).
- (h) MIT.
- (i) Tests: core runtime tests in `test/fixtures/` (subdirs `error_corpus/`, `template_corpus/`,
  `grammars/`, `test_grammars/`, `fixtures.json`) — e.g. `test/fixtures/error_corpus/javascript_errors.txt`.
  Each grammar repo has the canonical **corpus format**: `test/corpus/*.txt` files where each case is
  `=== name ===`, source code, `---`, then the expected S-expression tree (see
  <https://github.com/tree-sitter/tree-sitter-javascript/tree/master/test/corpus> —
  `destructuring.txt`, `expressions.txt`, `semicolon_insertion.txt`, … — and the format docs at
  <https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests.html>). Grammar repos
  also carry `test/highlight/` and `test/tags/` assertion files.
- (j) **Incremental re-parsing** after edits, error-recovery corpora, and the de-facto-standard
  corpus test format (meta-language already mimics the query syntax; the corpus format and
  incremental reparse are the gaps).

## ast-grep

- (a) CLI for structural search, lint, and rewriting built on tree-sitter, written in Rust. Repo:
  <https://github.com/ast-grep/ast-grep> (14.4k stars, active).
- (b) Read-only tree-sitter CSTs; edits are produced as text replacements computed from matches, not
  by mutating the tree.
- (c) Inherits tree-sitter's full-coverage CST; rewrites splice replacement text into original
  source, preserving untouched bytes.
- (d) Two query surfaces: **pattern code** (write the code you want to match, with `$META` /
  `$$$MULTI` metavariables) and a **YAML rule language** composing atomic rules (`pattern`, `kind`,
  `regex`), relational rules (`inside`, `has`, `precedes`, `follows`), and composite rules
  (`all`/`any`/`not`/`matches`), plus reusable `utils`. Docs: <https://ast-grep.github.io/>.
- (e) Yes: `fix:` rewrites in rules, `rewriters` for nested transformations, interactive `ast-grep
  scan --interactive` codemod mode; also Node/Python APIs (`@ast-grep/napi`, `pyo3` crates).
- (f) Any language with a tree-sitter grammar; ~25 built in, more via dynamic libraries.
- (g) Custom languages via dynamically loaded tree-sitter parsers (`crates/dynamic`,
  `fixtures/json-linux.so` in-repo); language bindings via napi/pyo3/wasm crates.
- (h) MIT.
- (i) Tests: Rust unit tests inline in `crates/core`, `crates/config`, `crates/cli/src` (note
  `crates/cli/src/verify/` implements the user-facing test framework). The **user-level rule-test
  format** is the portable asset: a `rule-tests/` dir with `<rule>-test.yml` files containing
  `valid:` / `invalid:` code lists plus `__snapshots__/` of expected diagnostics, driven by
  `ast-grep test` (<https://ast-grep.github.io/guide/test-rule.html>).
- (j) **Composable YAML rule algebra** (relational + boolean rule composition with named utility
  rules) and a built-in **rule snapshot-testing harness** with valid/invalid/noisy/missing
  classification.

## Semgrep (syntax-pattern subset)

- (a) Lightweight static-analysis engine; patterns look like the target language's source code.
  OCaml core + Python CLI. Repo: <https://github.com/semgrep/semgrep> (15.4k stars, active).
- (b) Read-only AST (per-language parsers normalized into a generic AST in OCaml).
- (c) Not lossless — it works on a normalized AST; autofix is span-based text replacement on
  original source rather than tree printing.
- (d) Pattern syntax: `$METAVARIABLES`, `...` ellipsis (statements/args), deep-expression
  `<... e ...>`, typed metavariables, `pattern-either`/`patterns`/`pattern-not`/`pattern-inside`
  composition in YAML rules, plus `metavariable-regex`/`-comparison`. Docs:
  <https://semgrep.dev/docs/writing-rules/pattern-syntax>.
- (e) `fix:` autofix (textual / metavariable substitution) — search-first tool, weaker rewriting than
  ast-grep/OpenRewrite.
- (f) 30+ languages (the `tests/patterns/` directory enumerates bash, c, cpp, csharp, dart,
  dockerfile, go, hack, html, java, js, json, jsonnet, julia, kotlin, lua, ocaml, php, python, r,
  ruby, rust, scala, solidity, swift, terraform, move, cairo, circom, ql, promql, …).
- (g) New languages added in-core (OCaml, mostly tree-sitter based via `semgrep-grammars`); rules are
  user-extensible YAML; registry at semgrep.dev.
- (h) LGPL-2.1 (engine); some components proprietary (Semgrep AppSec Platform).
- (i) Tests: top-level `tests/` with per-purpose subtrees. The gold mine is `tests/patterns/<lang>/`:
  **paired files** `name.<ext>` (target code, with `# ruleid:`-style annotations) + `name.sgrep`
  (the pattern) — e.g. `tests/patterns/python/ac_matching_dots.py` + `.sgrep`. Also
  `tests/parsing/`, `tests/autofix/`, `tests/rules/`, `tests/taint_maturity/`.
- (j) **Ellipsis (`...`) and deep-matching operators** that abstract over irrelevant code, typed
  metavariables, and constant propagation during matching ("semantic equivalences").

## Comby

- (a) Structural search-and-replace for "~every language", written in OCaml. Repo:
  <https://github.com/comby-tools/comby> (2.6k stars; site <https://comby.dev>).
- (b) No real tree: parses **balanced-delimiter structure + string/comment syntax** per language
  family, not a grammar — matching is over the raw text.
- (c) Trivially lossless (it never leaves the original text; replacements are spliced spans).
- (d) Template syntax with holes: `:[hole]`, `:[[word]]`, `:[hole\n]`, etc., plus a small `where`
  rule language (equality, regex match, nested match) — <https://comby.dev/docs/syntax-reference>.
- (e) Yes — that's its entire purpose: `comby 'match template' 'rewrite template'`; supports in-place
  rewrites, JSON match output, custom metasyntax.
- (f) Dozens of language definitions (comment/string/delimiter conventions); `.generic` matcher for
  anything else.
- (g) Custom matchers/metasyntax definable; OCaml library `comby-kernel`.
- (h) Apache-2.0.
- (i) Tests: `test/common/` — alcotest/OCaml files per language and feature
  (`test_c.ml`, `test_go.ml`, `test_generic.ml`, `test_nested_matches.ml`, `test_match_offsets.ml`,
  `test_custom_metasyntax.ml`, …) with inline source/template/expected strings; plus
  `test/test_special_matcher_cases.ml`.
- (j) **Grammar-less structural matching** (balanced-parens model) that works on languages you have
  no parser for — a useful fallback layer meta-language could add below tree-sitter.

## GritQL (grit)

- (a) Query language + engine for searching and modifying code, Rust, tree-sitter based. Repo:
  <https://github.com/getgrit/gritql> (4.5k stars; docs <https://docs.grit.io/language/overview>).
- (b) Read-only trees with rewrite effects collected and applied to text (the engine tracks
  "effects" rather than mutating the CST).
- (c) Tree-sitter-grade source fidelity; rewrites preserve surrounding bytes and auto-fix
  indentation.
- (d) **GritQL**: declarative datalog-ish/logic-flavored language where *source snippets in
  backticks are first-class patterns* (`` `console.log($msg)` ``), with metavariables, `where`
  clauses, pattern composition (`and`/`or`/`not`/`contains`/`within`/`after`), named pattern
  definitions, list/regex predicates, and built-in functions.
- (e) Yes: `=>` rewrite operator inside patterns; `grit apply`; migration "workflows" (multi-step,
  JS-scriptable — partly in the `js`/`python` dirs).
- (f) ~20 target languages via tree-sitter (JS/TS, Python, Java, Go, Rust, Ruby, PHP, CSS, JSON,
  YAML, HCL, Solidity, SQL, …) listed in `crates/language`.
- (g) Shareable pattern libraries (`.grit/patterns`); the standard library lives at
  <https://github.com/getgrit/stdlib> (`.grit/patterns/<lang>/*.md`); custom language support
  requires engine work.
- (h) MIT (engine). stdlib has no SPDX-detected license (custom/none reported by GitHub API —
  **unverified** what applies).
- (i) Tests: `crates/core/src/test.rs` + `crates/core/src/snapshots/` (insta snapshots),
  `crates/cli_bin/tests` + `crates/cli_bin/fixtures`. The most portable corpus is **stdlib pattern
  files**: each `.grit/patterns/<lang>/<pattern>.md` is a markdown doc embedding the GritQL pattern
  plus before/after config samples that double as executable tests (`grit patterns test`).
- (j) **Patterns written as literal target-language snippets inside a logic language**, plus
  markdown-as-executable-test pattern docs and multi-step migration workflows.

## srcML

- (a) Toolkit converting source code to/from an **XML markup of the parse tree** (C/C++ heritage,
  GPL). Repo: <https://github.com/srcML/srcML> (155 stars; site <https://www.srcml.org>).
- (b) The XML document is freely mutable with any XML tooling; srcML converts both directions.
- (c) **Fully lossless by design**: all text, whitespace, and comments are preserved inside the XML;
  `srcml file.xml` regenerates the original source byte-for-byte.
- (d) **XPath** (plus XSLT/XQuery) over the srcML namespace — reuses the entire XML ecosystem.
- (e) Transformations via XSLT or DOM manipulation, then unparse back to source.
- (f) C, C++, C#, Java officially; the develop-branch test suite shows Python and Objective-C
  support in progress (`*_py.py.xml`, `*_m.m.xml` files).
- (g) New languages require new parsers in the C++ codebase; output schema is a documented XML
  vocabulary.
- (h) GPL-3.0.
- (i) Tests: `test/parser/testsuite/` — **thousands of single-construct `.xml` files** named
  `construct_lang.ext.xml` (e.g. `assert_java.java.xml`, `atomic_c.c.xml`, `block_lambda.cpp.xml`),
  each containing the source fragment and its expected markup; plus `test/client/` and
  `test/libsrcml/`.
- (j) Round-trip **source⇆document model with off-the-shelf query languages** (XPath/XSLT); the
  per-construct round-trip testsuite layout is directly portable to meta-language's
  `LANGUAGE_FIXTURES`.

## difftastic

- (a) Structural (syntax-aware) diff tool, Rust, built on tree-sitter. Repo:
  <https://github.com/Wilfred/difftastic> (25.4k stars, active).
- (b)/(c) Read-only; parses both file versions to CSTs, diffs trees (Dijkstra-based minimal edit
  graph), displays unchanged-vs-changed at token level — no rewriting.
- (d)/(e) None (not a query/transform tool).
- (f) 50+ languages via vendored tree-sitter grammars (`vendored_parsers/`); falls back to text diff.
- (g) Adding a language = vendoring a grammar + highlighting config (documented in its manual).
- (h) MIT.
- (i) Tests: `sample_files/` with **before/after pairs** `name_1.ext` / `name_2.ext` (232 files: Ada,
  Apex, asm, Kotlin, …) exercised by `tests/cli.rs` and a `justfile` regression flow against expected
  outputs (`.expected` snapshots managed via the repo's tooling — **layout of expected outputs
  unverified beyond `tests/cli.rs`**).
- (j) **Structural diffing between two versions of a tree** — an obvious application meta-language's
  snapshot/fork model could expose (diff two `NetworkSnapshot`s and print token-level changes).

## Babel

- (a) JavaScript compiler/transpiler; parser (`@babel/parser`), traversal, transforms, codegen — JS.
  Repo: <https://github.com/babel/babel> (44k stars).
- (b) **Mutable AST** — plugins mutate paths during `@babel/traverse` visitation.
- (c) Not lossless: comments are attached (leading/trailing/inner) and ranges kept, but exact
  whitespace/formatting is regenerated by `@babel/generator` (optionally `retainLines`); use Recast
  for fidelity.
- (d) No query language; visitor pattern with node-type keys and `path` predicates
  (`path.isIdentifier({name: "n"})`).
- (e) The reference codemod/transpile pipeline: plugins + presets; `@babel/template` for quasiquote
  construction.
- (f) JS/TS/JSX/Flow only (plus proposals).
- (g) Plugin architecture (syntax plugins gate parser features; transform plugins visit/mutate).
- (h) MIT.
- (i) Tests: `packages/babel-parser/test/fixtures/<area>/<feature>/<case>/` with `input.js` +
  `output.json` (expected AST, auto-generated) — areas include `es2015`…`es2026`, `jsx`, `flow`,
  `typescript`, `estree`, `comments`, `annex-b` (verified: `fixtures/es2015/arrow-functions/inner-parens-2/{input.js,output.json}`).
  Transform tests live per-package in `packages/babel-plugin-*/test/fixtures` with `input.js`/
  `output.js` pairs. Also runs against **Test262**.
- (j) **`@babel/template` quasiquoting** (build subtrees from code strings with placeholders) and the
  enormous staged-by-year parser fixture corpus.

## SWC

- (a) Rust platform for the web: EcmaScript/TypeScript parser, transformer, minifier, bundler. Repo:
  <https://github.com/swc-project/swc> (33.5k stars).
- (b) **Mutable owned AST** (`swc_ecma_ast`) transformed via `Fold`/`VisitMut` passes.
- (c) Not lossless: comments stored in a side-table keyed by byte position; output is re-printed
  (`swc_ecma_codegen`); spans preserved for source maps.
- (d) No query language (Rust visitors only).
- (e) Yes: transform plugin system, including **Wasm plugins** loadable at runtime.
- (f) ECMAScript/TypeScript core, plus `swc_html_*` and `swc_css_*` crates.
- (g) Wasm plugin ABI (`swc_plugin`), so transforms can be written in any wasm-targeting language.
- (h) Apache-2.0.
- (i) Tests: `crates/swc_ecma_parser/tests/` with per-syntax directories (`js/`, `jsx/`,
  `typescript/`, `flow/`, `errors/`, `span/`, `comments/`) of source files + `.json`/`.swc-stderr`
  snapshots, and **git submodules of conformance suites**: `tc39/test262-parser-tests` at
  `crates/swc_ecma_parser/tests/test262-parser`, the **TypeScript compiler corpus** under `tsc/`,
  and `html5lib-tests` for the HTML parser (verified via `.gitmodules`).
- (j) **Conformance-by-submodule** (test262 / tsc / html5lib) — meta-language could mount the same
  external corpora — and the wasm plugin ABI for sandboxed third-party transforms.

## Recast

- (a) JavaScript syntax-tree transformer + **nondestructive pretty-printer** with source maps; JS.
  Repo: <https://github.com/benjamn/recast> (5.2k stars; last push 2025-03).
- (b) Mutable AST (ast-types / ESTree-compatible).
- (c) **Conservative printing**: untouched nodes are printed from their original source text
  verbatim; only modified subtrees are pretty-printed — the canonical "preserve what you didn't
  change" design meta-language's `replace()` also follows.
- (d) None beyond ast-types' `visit()` visitors.
- (e) Yes — it's the printing layer for jscodeshift and many codemods; parser-pluggable (Babel, TS,
  Flow, Esprima via `parsers/`).
- (f) JS/TS family only.
- (g) Pluggable parsers; ast-types extensible node definitions.
- (h) MIT.
- (i) Tests: `test/*.ts` per concern — `printer.ts`, `comments.ts`, `lines.ts`, `parens.ts`,
  `identity.ts` (round-trip/idempotency), `mapping.ts` (source maps), with sample inputs in
  `test/data/`. The **identity and parens suites** are the valuable ports: they encode
  reprint-fidelity and parenthesization invariants.
- (j) **Parenthesization-safety logic** when splicing subtrees into new positions, and source-map
  generation from reprints.

## jscodeshift

- (a) JavaScript codemod toolkit (Facebook/Meta): runner + jQuery-like Collection API over
  recast/ast-types. Repo: <https://github.com/facebook/jscodeshift> (10k stars).
- (b) Mutable (via recast AST).
- (c) Inherits recast's conservative printing.
- (d) Collection API: `j(file).find(j.Identifier, {name: "foo"})...` — structural filters by node
  type + property patterns, plus `closestScope`, `getVariableDeclarators`, etc.
- (e) Yes — transform modules (`export default (file, api) => ...`), parallel runner over file trees,
  dry-run, `defineTest` harness.
- (f) JS/TS family.
- (g) Custom parsers, extensible collections (`registerMethods`).
- (h) MIT.
- (i) Tests: `src/__tests__/` (unit) and `src/__testfixtures__/` + the documented **`defineTest`
  convention**: `__testfixtures__/<transform>.input.js` / `<transform>.output.js` pairs auto-run by
  `jscodeshift/dist/testUtils`. Recipes in `recipes/`.
- (j) The **input/output fixture convention + `defineTest`** harness for codemods (cheap to port as a
  transform-test convention for meta-language's `find()/replace()`).

## LibCST

- (a) Python **concrete syntax tree** parser/serializer (Instagram/Meta), Python API with a Rust
  parser (`native/`). Repo: <https://github.com/Instagram/LibCST> (1.9k stars; docs
  <https://libcst.readthedocs.io>).
- (b) **Immutable nodes with functional update** — `node.with_changes(...)`, transformer visitors
  return replacement nodes (`CSTTransformer`).
- (c) **Lossless**: whitespace/comments are modeled as typed fields on nodes; round-trips
  byte-identical (enforced by `native/roundtrip.sh` and `parser_roundtrip.rs`).
- (d) **Matchers DSL** (`libcst.matchers`): declarative node patterns with `OneOf`, wildcards,
  `MatchIfTrue` predicates, used as decorators or `m.matches(node, pattern)`.
- (e) Yes: codemod framework (`libcst.codemod`) with context, metadata providers
  (scope/type/position via `libcst.metadata`), batched transforms.
- (f) Python only (multiple grammar versions).
- (g) Metadata providers as plugins; matchers composable; codemod CLI.
- (h) MIT (some PSF-derived files dual MIT+PSF, per `LICENSE`).
- (i) Tests: Python side `libcst/_nodes/tests/`, `libcst/codemod/tests/`, `libcst/matchers/tests/`
  (node tests assert code⇆node equivalence both directions); Rust side
  `native/libcst/tests/fixtures/*.py` — **adversarial round-trip fixtures** with self-describing
  names (`dangling_indent.py`, `mixed_newlines.py`, `just_a_comment_without_nl.py`,
  `spacious_spaces.py`, `malicious_match.py`).
- (j) **Typed whitespace fields on every node** (not generic trivia bags) plus **metadata providers**
  layering scope/type info onto a lossless CST — the cleanest analogue of meta-language's projection
  idea, but with static typing per construct.

## RedBaron (+ Baron)

- (a) Python refactoring API on top of **Baron**, a "Full Syntax Tree" (FST) parser. Repos:
  <https://github.com/PyCQA/redbaron> (725 stars, last push 2022 — **effectively unmaintained**),
  <https://github.com/PyCQA/baron>.
- (b) Mutable, with string-assignment ergonomics (`node.value = "new_code"` re-parses in place).
- (c) **Lossless**: Baron guarantees `fst_to_code(code_to_fst(src)) == src`; the FST is JSON-
  serializable (formatting stored as `formatting` keys in the JSON).
- (d) `find()`/`find_all()` by node type + attribute, plus path/position queries
  (`at()`, bounding-box).
- (e) Mutation-as-API (no rule language).
- (f) Python only (grammar ≤3.7 — stale).
- (g) Minimal.
- (h) Baron LGPL-3.0; RedBaron repo has no SPDX-detected license file (PyPI metadata says LGPL-3.0 —
  **unverified in-repo**).
- (i) Tests: `tests/test_*.py` — notably `test_initial_parsing.py`, `test_render.py` (FST→code),
  `test_setter.py`, `test_indentation.py`, `test_position.py`, `test_proxy_list.py` (formatting-
  preserving list edits). Baron's own `tests/` cover the FST grammar.
- (j) **JSON-serializable FST** and "assign a string, get a parsed subtree" ergonomics; its
  proxy-list handling of comma/newline-separated sequences during insert/remove is a hard problem
  meta-language will hit too.

## Roslyn

- (a) The .NET compiler platform: C#/VB compilers exposing full syntax+semantic APIs. Repo:
  <https://github.com/dotnet/roslyn> (20.5k stars).
- (b) **Immutable red/green trees**: persistent green nodes + on-demand red wrappers with parents;
  updates via `With*()`/`ReplaceNode()` produce new trees with structure sharing.
- (c) **Full fidelity**: every token carries leading/trailing **trivia** (whitespace, comments,
  preprocessor directives, skipped tokens); `tree.ToFullString()` reproduces the source exactly, even
  with errors (docs: <https://learn.microsoft.com/en-us/dotnet/csharp/roslyn-sdk/work-with-syntax>).
- (d) No pattern language; typed APIs + LINQ over nodes (`DescendantNodes().OfType<...>()`);
  semantic queries via `SemanticModel`.
- (e) `SyntaxFactory`, `SyntaxRewriter` (CSharpSyntaxRewriter), Workspaces + `Formatter`/
  `Simplifier`/`Renamer`; analyzers + code-fix providers ship as NuGet plugins.
- (f) C# and VB only.
- (g) Analyzer/CodeFix/source-generator plugin model loaded by compiler and IDE.
- (h) MIT.
- (i) Tests: `src/Compilers/CSharp/Test/Syntax/` — `Parsing/` holds 71 files of construct-focused
  suites (`DeclarationParsingTests.cs`, `AsyncParsingTests.cs`,
  `CollectionExpressionParsingTests.cs`, …) using `UsingTree(...)`/`N(SyntaxKind...)` expected-tree
  assertions plus incremental-parsing tests; sibling `Semantic/`, `Symbol/`, `IOperation/`, `Emit*`
  dirs. Massive but extractable per construct.
- (j) **Red/green persistent trees with structure sharing** (cheap snapshots — directly relevant to
  `NetworkSnapshot`), trivia attached to tokens as leading/trailing, and `GetChanges()`/incremental
  parse APIs.

## OpenRewrite

- (a) Automated mass refactoring platform (Moderne); JVM-based. Repo:
  <https://github.com/openrewrite/rewrite> (3.5k stars, very active).
- (b) Immutable-ish **Lossless Semantic Trees (LSTs)** transformed by visitors that produce modified
  trees; recipes compose visitors.
- (c) **Lossless + type-attributed**: whitespace before/after each element is stored in the tree so
  printing "reconstitute[s] the original source code without clobbering formatting"; every node
  carries resolved type info
  (<https://docs.openrewrite.org/concepts-and-explanations/lossless-semantic-trees>).
- (d) No standalone query language; "search recipes" + `JavaTemplate` matching; declarative YAML
  recipe composition with preconditions.
- (e) The flagship: thousands of **recipes** (framework migrations, dependency upgrades), YAML
  recipe pipelines, `JavaTemplate` for tree construction, scanning recipes across whole repos.
- (f) Per-language modules in one repo: `rewrite-java` (8/11/17/21/25), `rewrite-kotlin`,
  `rewrite-groovy`, `rewrite-csharp`, `rewrite-javascript`, `rewrite-python`, `rewrite-go`,
  `rewrite-xml`, `rewrite-yaml`, `rewrite-json`, `rewrite-properties`, `rewrite-toml`,
  `rewrite-hcl`, `rewrite-protobuf`, `rewrite-docker`, `rewrite-maven`, `rewrite-gradle`.
- (g) Recipes as plain Java/YAML artifacts distributed via Maven; new languages = new LST module.
- (h) Apache-2.0 (some Moderne-proprietary modules elsewhere).
- (i) Tests: `rewrite-java-tck/` — an explicit **Technology Compatibility Kit** of parser/printer
  tests (`src/main/java/org/openrewrite/java/tree/…`, with `MinimumJava11/17/21/25` gates) that any
  alternate Java parser implementation must pass; per-language `*-test` modules use
  `RewriteTest`'s `rewriteRun(java("before","after"))` inline before/after style.
- (j) **Type attribution inside a lossless tree** and the **TCK pattern**: a parser-independent
  conformance suite — exactly the artifact meta-language should imitate (and can port from).

## Spoon

- (a) Java metaprogramming library (INRIA): analyze + transform Java source via a well-designed AST
  ("CtModel"). Repo: <https://github.com/INRIA/spoon> (1.9k stars).
- (b) **Mutable** AST with setters/factories; processors visit and mutate.
- (c) Default pretty-printer reformats; the **`SniperJavaPrettyPrinter`**
  (`src/main/java/spoon/support/sniper/SniperJavaPrettyPrinter.java`, verified) preserves original
  formatting of unmodified elements — "sniper mode" prints only what changed.
- (d) Typed queries: `model.getElements(new TypeFilter<>(CtMethod.class))`, composable `Filter`s and
  `CtQuery` chains; also `spoon-smpl` (SmPL/Coccinelle-style semantic patches for Java).
- (e) Processors, templates (`CtTemplate`), pattern matching/generation (`spoon.pattern`) with
  parameterized snippets.
- (f) Java only (+ Javadoc module, control/dataflow companions).
- (g) Processor plugin model; Maven plugin.
- (h) Dual CeCILL-C / MIT (verified: `LICENSE-CECILL-C.txt`, `LICENSE-MIT.txt`).
- (i) Tests: `src/test/java/spoon/test/<feature>/` (hundreds of feature packages) with inputs under
  `src/test/resources/`; sniper-printer round-trip tests are the interesting port target
  (`src/test/java/spoon/test/prettyprinter/`).
- (j) **Pattern objects built from compilable template code** (type-checked templates) and sniper
  printing as a pluggable printer strategy.

## JavaParser

- (a) Java 1–25 parser + AST with symbol resolution (JavaSymbolSolver). Repo:
  <https://github.com/javaparser/javaparser> (6.1k stars).
- (b) Mutable AST (observable nodes).
- (c) Optional **`LexicalPreservingPrinter`**
  (`javaparser-core/src/main/java/com/github/javaparser/printer/lexicalpreservation/`, verified):
  tracks tokens/whitespace and computes diffs (`Difference.java`, `DifferenceElementCalculator.java`)
  so unmodified code keeps formatting; default printer reformats.
- (d) Visitors + `Node.findAll(Class)`; no pattern language.
- (e) Direct AST mutation; no rule/codemod framework.
- (f) Java only.
- (g) Library-level; symbol solver pluggable type solvers.
- (h) Triple-licensed Apache-2.0 / LGPL / GPL (verified: `LICENSE.APACHE`, `LICENSE.LGPL`,
  `LICENSE.GPL`).
- (i) Tests: `javaparser-core-testing/src/test/java/com/github/javaparser/` — note
  `LexicalPreservingPrinterTest` area, `TokenRangeTest.java`, `PositionMappingTest.java`,
  `ParseErrorRecoveryTest.java`, plus issue-numbered regression tests (`Issue1017Test.java`, …) and
  `ast/` construct tests; BDD suite in `javaparser-core-testing-bdd`.
- (j) **Lexical preservation as a retrofit**: a token-diff algorithm bolted onto a non-lossless AST —
  meta-language avoids this by being lossless-first, but its `Difference*` test cases are great
  adversarial inputs.

## Rascal MPL

- (a) A complete **meta-programming language** (CWI/UseTheSource) for source analysis and
  transformation: built-in grammars, parsing, traversal, rewriting. Repo:
  <https://github.com/usethesource/rascal> (458 stars; site <https://www.rascal-mpl.org>).
- (b) Immutable values; transformation via rewriting (`visit` returns new trees).
- (c) Parse trees from Rascal's generalized parser are **fully concrete** (layout/comments are part
  of the tree per grammar's layout definitions); ASTs via `implode` lose trivia.
- (d) **Concrete-syntax pattern matching**: match parse trees with patterns written in the object
  language inside quotes (`` (Expr)`<Expr a> + <Expr b>` ``), plus abstract patterns,
  `visit`/`switch` strategies, and relational/comprehension queries over extracted facts.
- (e) Yes: `visit` with insert, string templates, full GPL around it.
- (f) Any language you define a grammar for (built-in grammar formalism, GLL parsing); library
  grammars for Java (M3), C, etc.
- (g) Define grammars + libraries in Rascal itself; Eclipse/VS Code IDE generation.
- (h) BSD-2-Clause (verified from `LICENSE`; some older files Eclipse-licensed).
- (i) Tests: `test/org/rascalmpl/` (JUnit drivers) but the bulk are **in-language test functions**
  (`test bool ...`) under `src/org/rascalmpl/library/lang/**/tests/`, runnable by the interpreter;
  also `test/org/rascalmpl/benchmark`.
- (j) **Concrete-syntax quoting/antiquoting in patterns** for arbitrary user-defined grammars and
  built-in relational analysis (M3 model) — the strongest academic analogue to meta-language's goal.

## Spoofax / Stratego

- (a) Language workbench (TU Delft/MetaBorg): SDF3 declarative syntax, **Stratego** rewriting
  language, NaBL2/Statix name/type analysis. Repos: <https://github.com/metaborg/spoofax> (Apache-2.0
  runtime), Spoofax 3 at <https://github.com/metaborg/spoofax-pie> (**unverified contents**), site
  <https://spoofax.dev>.
- (b) Terms (ATerms) are immutable; Stratego rewriting produces new terms.
- (c) SGLR parse trees retain layout; typical pipelines work on ASTs with origin tracking
  ("origin terms" link AST nodes back to source for layout-preserving unparse — **partially
  unverified**, based on Spoofax docs).
- (d)/(e) **Stratego**: rewrite rules + programmable **strategies** (`innermost`, `topdown`,
  `try`, …) — full strategic term rewriting; concrete-syntax patterns supported in rules.
- (f) Any language defined in SDF3.
- (g) The whole point: define syntax, analysis, transforms, editor services declaratively.
- (h) Apache-2.0.
- (i) Tests: **SPT (Spoofax Testing language)** — a dedicated DSL where `.spt` files embed program
  fragments and expectations (`parse succeeds`, `parse to <AST>`, resolution/type expectations);
  framework at <https://github.com/metaborg/spt> (Apache-2.0, verified); JUnit-style tests in
  `org.metaborg.core.test`.
- (j) **A DSL for syntax tests** (SPT) with first-class expectations like `parse to` /
  `resolve #1 to #2`, and **strategy combinators** for ordering rewrites — meta-language's
  transforms have no traversal-strategy algebra yet.

## TXL

- (a) "Source Transformation by Example": grammar + by-example rewrite rules, a 30+-year-old
  transformation language (Queen's University). Site: <https://www.txl.ca> — current release FreeTXL
  10.8b (July 2022).
- (b)/(c) Functional rewriting over parse trees; whole files are re-unparsed (formatting is governed
  by grammar formatting cues, not preserved verbatim).
- (d)/(e) TXL rules: patterns and replacements written in the **object language's own syntax**
  ("by example"), with parameterized rules, guards (`where`), and grammar **overrides** to extend a
  base grammar per-transformation.
- (f) The TXL grammar collection covers many languages (C, Java, COBOL, …) —
  <https://www.txl.ca/txl-resources.html>.
- (g) Grammar redefinition/override mechanism.
- (h) FreeTXL is free and freely distributable (binary; compiler/interpreter); **not** OSI
  open-source in the usual sense — source availability **unverified**.
- (i) Test suite not publicly browsable on GitHub (no canonical public repo found) — **nothing to
  port directly**; example transforms ship with the distribution.
- (j) **Grammar overrides**: locally extending a language's grammar just for one transformation
  (e.g., adding markup constructs) — a powerful idea for meta-language's per-language grammars.

## JetBrains MPS

- (a) Projectional language workbench: programs are edited **directly as the AST** (no parsing);
  editors project the model as text/tables/diagrams. Repo: <https://github.com/JetBrains/MPS>
  (1.6k stars; docs <https://www.jetbrains.com/mps/>).
- (b) Fully mutable model (the model *is* the persistence format, XML on disk).
- (c) Lossless trivially — there is no concrete text to lose; but **cannot ingest arbitrary existing
  source text** without separate importers.
- (d) Model queries via generator/BaseLanguage query API and editor "find usages"; no textual query
  language.
- (e) Generators (model-to-model + model-to-text templates), refactorings, intentions.
- (f) Any language built in MPS; composition of languages is a headline feature (embedding languages
  without grammar conflicts).
- (g) Define languages (structure/editor/typesystem/generator aspects); plugin ecosystem.
- (h) Apache-2.0.
- (i) Tests: in-repo MPS modules — `languages.test/languageDesign`, `testbench/`, plus MPS's own
  test languages for editor/generator tests; tests are MPS models (XML), not portable text fixtures.
- (j) **Grammar-conflict-free language composition/embedding** and projectional (multi-notation)
  editing — meta-language's mixed-region links are the parsing-world cousin; MPS shows the
  AST-first endgame.

## Coccinelle

- (a) Program matching/transformation for **C** (Inria), driven by **SmPL "semantic patches"** —
  patch-like specs with metavariables; used heavily by the Linux kernel. Repo:
  <https://github.com/coccinelle/coccinelle> (797 stars; OCaml).
- (b)/(c) Internal C AST + control-flow graph; output is generated as a minimal patch, so untouched
  code is preserved (patch-based, not reprint-based).
- (d) **SmPL**: looks like a unified diff with `-`/`+` lines, metavariable declarations
  (`@@ expression E; @@`), `...` for control-flow paths (matching is along CFG paths, not just
  syntax), position variables, and rule dependencies.
- (e) Yes — the semantic patch *is* the transform; `spatch` applies it across a codebase.
- (f) C (plus experimental C++ — `cpptests/` dir in repo).
- (g) Python/OCaml scripting hooks inside rules.
- (h) GPL-2.0.
- (i) Tests: `tests/` with ~1000 files in **triples**: `name.c` (input), `name.cocci` (semantic
  patch), `name.res` (expected output) — verified (`a.c`/`a.cocci`/`a.res`, `62.*`, …). The single
  most portable transform-test corpus format in this list.
- (j) **Control-flow-aware `...` matching** (match along execution paths, not sibling lists) and
  diff-shaped transform specs; also the input/patch/expected triple test format.

---

## Other notable projects (adjacent, verified 2026-06-10)

- **Biome** (<https://github.com/biomejs/biome>, Apache-2.0, 25k stars): web toolchain whose CST is a
  rust-analyzer-style **red/green lossless tree** (`biome_rowan`); error-resilient parsing where even
  broken code round-trips. Test corpus: per-crate `tests/specs/**` snapshot files. Closest Rust-native
  lossless-CST competitor.
- **rowan** (<https://github.com/rust-analyzer/rowan>, Apache-2.0): the standalone red/green
  lossless syntax-tree library underlying rust-analyzer — untyped `GreenNode` + typed AST layer is a
  proven architecture for "one substrate, many projections".
- **dave/dst** (<https://github.com/dave/dst>, MIT per repo LICENSE; GitHub shows NOASSERTION):
  Go "Decorated Syntax Tree" — manipulates Go source "with perfect fidelity" by attaching comments
  and spacing as decorations on nodes, fixing `go/ast`'s position-based comment problem.
- **parso** (<https://github.com/davidhalter/parso>): Python parser powering jedi; round-trippable
  trees and **error recovery**; tests include round-trip + fuzz cases.
- **Uber Piranha** (<https://github.com/uber/piranha>, Apache-2.0): tree-sitter-based
  multi-language structural rewriting (feature-flag cleanup) with rules in TOML + graph of rule
  dependencies.
- **CodeQL / Glean / Kythe** — semantic code indexing/query (datalog-style); out of syntax-tooling
  scope but relevant to meta-language's semantic-projection ambitions.

---

## Feature ideas meta-language should match

Concrete capabilities, each tagged with its source:

1. **Incremental re-parse after edits** — reuse unchanged subtrees instead of full re-parse
   [tree-sitter].
2. **Corpus test format** (`=== name ===` / source / `---` / expected S-expression) and an
   `error_corpus` of malformed inputs with expected recovery trees [tree-sitter].
3. **Composable rule algebra**: relational (`inside`/`has`/`precedes`/`follows`) and boolean
   (`all`/`any`/`not`) combinators over patterns, with named reusable `utils` [ast-grep].
4. **Rule snapshot-testing harness** with `valid:`/`invalid:` cases and
   reported/validated/noisy/missing classification [ast-grep].
5. **Ellipsis / deep-match operators** (`...`, `<... e ...>`) and typed metavariables so patterns
   abstract over irrelevant code [Semgrep].
6. **Grammar-less fallback matcher** based on balanced delimiters + string/comment conventions, for
   languages without a parser [Comby].
7. **Source snippets as first-class pattern literals** inside a logic/query language, with `=>`
   rewrites and shareable pattern libraries whose markdown docs are executable tests [GritQL].
8. **Export to a standard document model** (XML/JSON) so XPath/XSLT/jq ecosystems can query and
   transform the network [srcML; Baron's JSON FST].
9. **Structural diff between two snapshots** with token-level change display — natural fit for
   `NetworkSnapshot` forks [difftastic; Roslyn `GetChanges()`].
10. **Quasiquote templates with placeholders** for building subtrees from code strings
    [Babel `@babel/template`; Spoon patterns; Rascal concrete-syntax quoting].
11. **Sandboxed plugin ABI (Wasm)** so third parties ship transforms safely [SWC].
12. **Conservative reprint + parenthesization safety**: when a transform splices a subtree into a
    new precedence context, auto-insert/drop parens; idempotent reprint ("identity") tests
    [Recast `test/parens.ts`, `test/identity.ts`].
13. **Codemod fixture convention**: `<transform>.input.x` / `<transform>.output.x` pairs with a
    one-line `defineTest` [jscodeshift].
14. **Typed trivia**: model whitespace/comments as typed fields per construct (not just generic
    trivia links), plus **metadata providers** layering scope/type/position over the lossless tree
    [LibCST].
15. **Red/green persistent trees with structure sharing** for cheap snapshots and full-fidelity
    `ToFullString()` even on error trees [Roslyn; rowan/Biome].
16. **Type attribution in the lossless tree** and a parser-independent **TCK** other
    implementations must pass [OpenRewrite `rewrite-java-tck`].
17. **Traversal-strategy combinators** (`topdown`, `innermost`, `try`, fixpoint) as a transform
    ordering algebra [Stratego/Spoofax; also Rascal `visit`].
18. **A test DSL for syntax expectations** (`parse succeeds`, `parse to <tree>`, name-resolution
    expectations) embedded next to fragments [Spoofax SPT].
19. **Grammar overrides**: locally extend a language grammar for one transformation [TXL].
20. **Control-flow-path `...` matching** and diff-shaped transform specs producing minimal patches
    [Coccinelle SmPL].
21. **Language composition without grammar conflicts** and non-text projections of the same model
    [JetBrains MPS] — validates meta-language's mixed-region direction.
22. **Mounting external conformance corpora as submodules** (test262, tsc, html5lib) [SWC; Babel].

### Most portable competitor test corpora (for meta-language's parity fixtures)

| Corpus | Path | Shape |
|---|---|---|
| tree-sitter grammar corpora | `tree-sitter-<lang>/test/corpus/*.txt` | source + expected S-expression, per construct |
| tree-sitter error corpus | `tree-sitter/test/fixtures/error_corpus/*.txt` | malformed source + recovery trees |
| Coccinelle | `coccinelle/tests/{*.c,*.cocci,*.res}` | ~1000 input/patch/expected triples |
| Semgrep patterns | `semgrep/tests/patterns/<lang>/{*.ext,*.sgrep}` | paired target+pattern, 30+ languages |
| srcML | `srcML/test/parser/testsuite/*.xml` | per-construct lossless round-trip docs |
| LibCST round-trip | `LibCST/native/libcst/tests/fixtures/*.py` | adversarial whitespace/newline fixtures |
| Babel parser | `babel/packages/babel-parser/test/fixtures/<era>/<feature>/<case>/{input.js,output.json}` | staged-by-year AST fixtures |
| OpenRewrite TCK | `rewrite/rewrite-java-tck/src/main/java/org/openrewrite/java/**` | parser/printer conformance kit |
| difftastic | `difftastic/sample_files/{*_1,*_2}.ext` | before/after pairs, 50+ languages |
| ast-grep rule tests | user projects' `rule-tests/*-test.yml` + `__snapshots__/` | valid/invalid + snapshot |
