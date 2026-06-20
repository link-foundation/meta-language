# Grammar subsystem

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md).

The grammar subsystem is the layer that lets `meta-language` describe syntax as
data in the same links network that stores parsed documents and programs. It
exists so an authored, imported, generated, inferred, or translated grammar can
move through one canonical intermediate representation instead of being trapped
inside one parser generator format. That satisfies requirement P-1 for the
grammar layer: the project can explain what the layer is, what it is for, and
how it fits into the rest of the self-describing network.

At the center is the [grammar IR](architecture.md): `Grammar`,
`GrammarRule`, `GrammarExpr`, `RuleKind`, and `GrammarFormat` in
[`src/grammar/mod.rs`](../../src/grammar/mod.rs). The IR lowers to
`LinkType::Grammar` links through the same `ToLinks` and `FromLinks` codec used
by other structured objects, so a grammar is not side data. It is part of the
network.

## Pipeline

```text
authoring/import
      |
      v
Grammar IR as links
      |
      +--> infer and evaluate
      +--> emit other grammar formats
      +--> translate through grammar concepts
      +--> generate parser code
      +--> register or run a parser
```

Every stage should enter or leave through the IR. Stages whose public API has
already landed link to concrete Rust items. Stages owned by later issues are
documented as contracts and examples in `text` fences until those APIs land and
can be promoted to rustdoc doctests.

## Documentation map

| Page | Purpose | Owning issues |
| --- | --- | --- |
| [Architecture](architecture.md) | IR, links encoding, projections, concept alignment, and the end-to-end pipeline. | [A1](../case-studies/issue-93/proposed-issues/A1-grammar-ir.md), [A3](../case-studies/issue-93/proposed-issues/A3-grammar-concept-ontology.md), [C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md) |
| [Authoring](authoring.md) | Write the native grammar surface, lower it to the IR, and understand current validation. | [A2](../case-studies/issue-93/proposed-issues/A2-grammar-surface-syntax.md), [E4](../case-studies/issue-93/proposed-issues/E4-grammar-authoring-ergonomics.md) |
| [Import and export](import-export.md) | Import external notations into the IR and emit them back out. | [B1](../case-studies/issue-93/proposed-issues/B1-bnf-importer.md)-[B7](../case-studies/issue-93/proposed-issues/B7-lark-gbnf-importer.md), [C1](../case-studies/issue-93/proposed-issues/C1-bnf-ebnf-abnf-emitters.md)-[C3](../case-studies/issue-93/proposed-issues/C3-gbnf-emitter.md), [F2](../case-studies/issue-93/proposed-issues/F2-grammar-format-fidelity-matrix.md) |
| [Fidelity](fidelity.md) | Format-by-format round-trip fidelity matrix over the grammar IR construct vocabulary. | [F2](../case-studies/issue-93/proposed-issues/F2-grammar-format-fidelity-matrix.md) |
| [Code generation](codegen.md) | Generate Rust and JavaScript parser code from the IR. | [C4](../case-studies/issue-93/proposed-issues/C4-rust-parser-codegen.md), [C5](../case-studies/issue-93/proposed-issues/C5-javascript-parser-codegen.md), [E5](../case-studies/issue-93/proposed-issues/E5-end-to-end-integration-examples.md) |
| [Inference](inference.md) | Infer grammar structure from examples and evaluate candidates. | [D1](../case-studies/issue-93/proposed-issues/D1-inference-evaluation-harness.md), [D5](../case-studies/issue-93/proposed-issues/D5-blackbox-cfg-inference.md), [D6](../case-studies/issue-93/proposed-issues/D6-delimiter-structural-prior.md) |
| [Translation](translation.md) | Translate between grammar notations through grammar concepts. | [A3](../case-studies/issue-93/proposed-issues/A3-grammar-concept-ontology.md), [C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md) |
| [CLI and runtime](cli-and-runtime.md) | Expose grammar operations on the command line and register runtime parsers. | [E1](../case-studies/issue-93/proposed-issues/E1-cli-grammar-subcommands.md), [E2](../case-studies/issue-93/proposed-issues/E2-inferred-grammar-runtime-parser.md) |

## Current entry points

- Build grammar data with `Grammar::builder()` and `Grammar::expr()` in
  [`src/grammar/mod.rs`](../../src/grammar/mod.rs).
- Encode and decode the IR as links with `ToLinks`, `FromLinks`,
  `LinksEncoder`, and `LinksDecoder`.
- Parse and write the native grammar surface with `parse_grammar_surface` and
  `write_grammar_surface` from
  [`src/grammar/surface/mod.rs`](../../src/grammar/surface/mod.rs).
- Convert grammar links to and from LiNo with `grammar_to_lino` and
  `grammar_from_lino`.
- Align grammar constructs with concepts using
  [`src/grammar/concepts.rs`](../../src/grammar/concepts.rs).

See [architecture](architecture.md) first when extending the subsystem, then
follow the stage page for the boundary you are implementing.
