# Grammar authoring

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [A2](../case-studies/issue-93/proposed-issues/A2-grammar-surface-syntax.md)
for the native surface syntax and
[E4](../case-studies/issue-93/proposed-issues/E4-grammar-authoring-ergonomics.md)
for expanded validation and friendly diagnostics.

Authoring starts with the meta-language grammar surface and lowers into the
[`Grammar` IR](architecture.md). The current public entry points are
`parse_grammar_surface`, `write_grammar_surface`, `grammar_to_lino`, and
`grammar_from_lino` in
[`src/grammar/surface/mod.rs`](../../rust/src/grammar/surface/mod.rs).

## Minimal example

```text
(expr: term (( "+" / "-" ) term)*)
(term: factor (( "*" / "/" ) factor)*)
(factor: number / "(" expr ")")
(number: [0-9]+)
```

Parsing that surface text produces a `Grammar` with `GrammarFormat::MetaLanguage`
and the first rule as the default start rule. Writing the grammar returns
canonical surface text. See
[`tests/unit/grammar_surface.rs`](../../rust/tests/unit/grammar_surface.rs) for the
executable coverage of each surface form.

## Surface forms

| Surface form | IR construct |
| --- | --- |
| `"literal"` or `'literal'` | `GrammarExpr::Terminal` |
| `` `literal` `` | `GrammarExpr::TerminalInsensitive` |
| `[a-z]` | `GrammarExpr::CharRange` |
| `[^ a b]` | negated `GrammarExpr::CharClass` |
| `.` | `GrammarExpr::AnyChar` |
| `name` | `GrammarExpr::NonTerminal` |
| `a b c` | `GrammarExpr::Sequence` |
| `a / b` | ordered `GrammarExpr::Choice` |
| `a | b` | unordered `GrammarExpr::Choice` |
| `a?`, `a*`, `a+` | optional and repetition expressions |
| `a{2,4}`, `a{1,}` | counted repetition |
| `& a`, `! a` | positive and negative lookahead |
| `{ label : a }` | labelled capture |
| `()` | `GrammarExpr::Empty` |

## Validation

The current parser rejects malformed skeletons, failed lowerings, and undefined
non-terminal references through `GrammarSurfaceError`. Programmatic grammars can
also inspect `Grammar::undefined_nonterminals()` from
[`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs).

The richer authoring validator planned by
[E4](../case-studies/issue-93/proposed-issues/E4-grammar-authoring-ergonomics.md)
should build on those same checks and return diagnostics without changing the IR
contract.

## Authoring flow

```text
surface text
  -> parse_grammar_surface
  -> Grammar
  -> optional links encoding or LiNo serialization
  -> emit, translate, generate, infer against, or register
```

Use [import and export](import-export.md) when the source grammar is BNF, EBNF,
ABNF, PEG, ANTLR, Lark, GBNF, or tree-sitter JSON rather than the native surface.
