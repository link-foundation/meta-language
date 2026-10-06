# Grammar code generation

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [C4](../case-studies/issue-93/proposed-issues/C4-rust-parser-codegen.md)
for Rust parser generation, [C5](../case-studies/issue-93/proposed-issues/C5-javascript-parser-codegen.md)
for JavaScript parser generation, and
[E5](../case-studies/issue-93/proposed-issues/E5-end-to-end-integration-examples.md)
for runnable integration examples.

Code generation consumes the [`Grammar` IR](architecture.md) and emits parser
source code for a target runtime. The current branch provides the IR and links
encoding plus Rust and JavaScript parser generators.

## Minimal example

```text
Grammar
  -> generate Rust parser
  -> cargo test generated fixture

Grammar
  -> generate JavaScript parser
  -> run it on the native grammar executor
```

Generators should accept `Grammar` values rather than source-format text. If a
caller starts from BNF, EBNF, PEG, or another notation, it should first use the
[import stage](import-export.md) so every generator sees the same normalized
structure.

## Generator contract

A generator should document:

- Which `GrammarExpr` variants it supports natively.
- Which `RuleKind` values change parse output or tokenization.
- How it reports unsupported expressions.
- How generated parsers preserve source spans, missing/error markers, or
  recovery information when the runtime supports them.
- Which fixtures are compiled or executed by CI.

## Current stable input

Use [`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs) to build fixtures:

```text
let expr = Grammar::expr();
Grammar::builder()
    .source_format(GrammarFormat::MetaLanguage)
    .start("number")
    .rule("number", expr.rep1(expr.char_range('0', '9')))
    .build()
```

That snippet is shown as text here because this Markdown file is not compiled by
rustdoc. The equivalent IR-builder round trip is doctested in the module docs for
[`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs).

The existing `emit_rust_parser` produces a `.pest` grammar, `pest_derive` parser
stub and AST declarations; that external engine does not satisfy the
[native grammar execution requirement](../vision.md#native-merged-grammars), and
replacing it in Rust remains open work. In JavaScript, `emitJavascriptParser`
produces an ESM module that carries the serialized grammar and calls
`compileGrammar(deserializeGrammar(GRAMMAR))` from the `meta-language` package,
so the generated parser runs on the native executor described in
[the grammar feature union](feature-union.md); `emitPeggy` still emits Peggy
grammar text as an export format, and nothing in the package runs Peggy.

## See also

- [Architecture](architecture.md) for why generators target the IR.
- [CLI and runtime](cli-and-runtime.md) for how generated or inferred parsers
  should become runnable.
- [Import and export](import-export.md) when the generator input starts as an
  external notation.
