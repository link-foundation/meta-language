# Grammar import and export

This page describes the current Rust grammar IR and earlier implementation
stages. [The vision](../vision.md#grammar-import-conversion-and-reverse-conversion)
requires the complete import, export and round-trip system in this pull request
and in both runtime packages. The stage links below are historical references,
not ownership or permission to defer unfinished work.

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [B1](../case-studies/issue-93/proposed-issues/B1-bnf-importer.md)
through [B7](../case-studies/issue-93/proposed-issues/B7-lark-gbnf-importer.md)
for importers, [C1](../case-studies/issue-93/proposed-issues/C1-bnf-ebnf-abnf-emitters.md)
through [C3](../case-studies/issue-93/proposed-issues/C3-gbnf-emitter.md) for
emitters, and [F2](../case-studies/issue-93/proposed-issues/F2-grammar-format-fidelity-matrix.md)
for the fidelity matrix.

Importers translate external grammar notations into the
[`Grammar` IR](architecture.md). Emitters translate the IR back into a target
notation. The current branch already has the IR and native surface helpers; the
format-specific importer and emitter APIs are incomplete. The required APIs and
command-line tools must be implemented and tested in both packages here.

## Minimal example

```text
BNF, EBNF, ABNF, PEG, ANTLR, Lark, GBNF, or tree-sitter JSON
  -> importer
  -> Grammar { source_format: Some(...) }
  -> emitter
  -> target grammar notation
```

The importer must preserve source order where possible, set `GrammarFormat`, and
represent every source construct and its semantics as inspectable native links.
The emitter must consume only those links rather than re-reading the original
text. If a target format cannot express a feature directly, export needs a
faithful executable lowering or runtime encoding, with explicit reconstruction
metadata; a lossy report does not complete the conversion.

## Format table

| Format | Import owner | Emit owner | IR source tag |
| --- | --- | --- | --- |
| BNF | [B1](../case-studies/issue-93/proposed-issues/B1-bnf-importer.md) | [C1](../case-studies/issue-93/proposed-issues/C1-bnf-ebnf-abnf-emitters.md) | `GrammarFormat::Bnf` |
| EBNF | [B2](../case-studies/issue-93/proposed-issues/B2-ebnf-importer.md) | [C1](../case-studies/issue-93/proposed-issues/C1-bnf-ebnf-abnf-emitters.md) | `GrammarFormat::Ebnf` |
| ABNF | [B3](../case-studies/issue-93/proposed-issues/B3-abnf-importer.md) | [C1](../case-studies/issue-93/proposed-issues/C1-bnf-ebnf-abnf-emitters.md) | `GrammarFormat::Abnf` |
| PEG | [B4](../case-studies/issue-93/proposed-issues/B4-peg-importer.md) | [C2](../case-studies/issue-93/proposed-issues/C2-peg-emitter.md) | `GrammarFormat::Peg` |
| tree-sitter JSON | [B5](../case-studies/issue-93/proposed-issues/B5-tree-sitter-json-importer.md) | [C7](../case-studies/issue-93/proposed-issues/C7-tree-sitter-grammar-js-emitter.md) | `GrammarFormat::TreeSitter` |
| ANTLR | [B6](../case-studies/issue-93/proposed-issues/B6-antlr-importer.md) | required in this pull request | `GrammarFormat::Antlr` |
| Lark and GBNF | [B7](../case-studies/issue-93/proposed-issues/B7-lark-gbnf-importer.md) | [C3](../case-studies/issue-93/proposed-issues/C3-gbnf-emitter.md) for GBNF | `GrammarFormat::Lark`, `GrammarFormat::Gbnf` |

## Fidelity

Fidelity is not a boolean property of the importer alone. It is the result of a
round trip through the IR:

```text
source notation -> Grammar -> target notation -> Grammar
```

[F2](../case-studies/issue-93/proposed-issues/F2-grammar-format-fidelity-matrix.md)
owns the detailed [fidelity matrix](fidelity.md). This page only records the
stage contract: each importer and emitter must preserve the relevant syntax and
meaning. An unsupported diagnostic or approximation is an honest indication of
unfinished work, never passing round-trip evidence.

## See also

- [Authoring](authoring.md) for the native surface syntax.
- [Translation](translation.md) for concept-aligned conversion between
  notations.
- [`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs) for the `GrammarFormat`
  variants shared by importers and emitters.
