# Grammar CLI and runtime

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [E1](../case-studies/issue-93/proposed-issues/E1-cli-grammar-subcommands.md)
for grammar CLI subcommands and
[E2](../case-studies/issue-93/proposed-issues/E2-inferred-grammar-runtime-parser.md)
for registering inferred grammars as runtime parsers.

The runtime boundary makes grammar work usable outside library tests. The CLI
should expose import, emit, infer, and translate operations. The parser runtime
should let a `Grammar` become a parser registered in `ParserRegistry`.

## Current CLI baseline

The current CLI in [`src/main.rs`](../../src/main.rs) exposes:

```bash
cargo run -- describe
cargo run -- verify --language plain-text --text "alpha beta"
```

`describe` prints the built-in self-description network. `verify` parses source
with the lossless text boundary and reports whether the selected region is
clean. Grammar-specific subcommands are planned by
[E1](../case-studies/issue-93/proposed-issues/E1-cli-grammar-subcommands.md).

## Planned grammar subcommands

```text
cargo run -- infer --examples examples.txt --output grammar.lino
cargo run -- import-grammar --from bnf --input grammar.bnf --output grammar.lino
cargo run -- emit-grammar --to gbnf --input grammar.lino --output grammar.gbnf
cargo run -- translate-grammar --from ebnf --to peg --input grammar.ebnf --output grammar.peg
```

Those commands should route through the same IR described in
[architecture](architecture.md). They should not duplicate importer, emitter, or
translation logic behind CLI-only code paths.

## Runtime parser registration

[`src/parser_registry.rs`](../../src/parser_registry.rs) already provides
`ParserRegistry`, `LanguageParser`, and `LinkNetwork::parse_with_registry`.
Today, callers can register hand-written parsers that shadow built-in dispatch.
[E2](../case-studies/issue-93/proposed-issues/E2-inferred-grammar-runtime-parser.md)
extends that idea so an inferred or authored `Grammar` can become a runtime
parser.

The expected shape is:

```text
Grammar
  -> runtime parser adapter
  -> ParserRegistry::register("language-key", parser)
  -> LinkNetwork::parse_with_registry(...)
```

When the grammar comes from inference, the runtime parser should preserve the
same verification expectations recorded by the inference evaluation harness.

## See also

- [Inference](inference.md) for producing candidate grammars from examples.
- [Code generation](codegen.md) for generating standalone parser code.
- [Import and export](import-export.md) for CLI import and emit operations.
- [`README.md`](../../README.md) for the current CLI and development commands.
