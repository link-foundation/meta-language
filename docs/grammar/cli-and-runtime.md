# Grammar CLI and runtime

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [E1](../case-studies/issue-93/proposed-issues/E1-cli-grammar-subcommands.md)
for grammar CLI subcommands and
[E2](../case-studies/issue-93/proposed-issues/E2-inferred-grammar-runtime-parser.md)
for registering inferred grammars as runtime parsers.

The runtime boundary makes grammar work usable outside library tests. The CLI
should expose import, emit, infer, and translate operations. The parser runtime
should let a `Grammar` become a parser registered in `ParserRegistry`.

## Current CLI

The current CLI in [`src/main.rs`](../../src/main.rs) exposes:

```bash
cargo run -- describe
cargo run -- verify --language plain-text --text "alpha beta"
cargo run -- infer examples/json-samples/
cargo run -- infer a.txt b.txt c.txt --format gbnf --metrics --out grammar.gbnf
cargo run -- import-grammar --format bnf grammar.bnf
cargo run -- import-grammar --format ebnf grammar.ebnf --to bnf
cargo run -- emit-grammar --format gbnf grammar.lino --out grammar.gbnf
cargo run -- translate-grammar grammar.lino --from-language en --to-language ru
```

`describe` prints the built-in self-description network. `verify` parses source
with the lossless text boundary and reports whether the selected region is
clean.

The grammar subcommands route through the same IR described in
[architecture](architecture.md). They do not duplicate importer, emitter,
inference, or translation logic behind CLI-only code paths.

- `infer <examples>...` reads example files or directories, calls the D5 CFG
  inference entry point, and writes LiNo by default. `--format` can render
  `bnf`, `ebnf`, `abnf`, `peg`, `gbnf`, or `tree-sitter`; `--metrics` writes a
  one-line precision/recall/F1/runtime summary to stderr; `--out <path>` writes
  the grammar to a file.
- `import-grammar --format <format> <input>` imports
  `bnf`, `ebnf`, `abnf`, `peg`, `antlr`, `lark`, `gbnf`, or `tree-sitter` and
  renders LiNo by default. `--to <format>` can render another supported output
  notation.
- `emit-grammar --format <format> <input>` reads LiNo or native grammar surface
  text and emits `bnf`, `ebnf`, `abnf`, `peg`, `gbnf`, or `tree-sitter`.
- `translate-grammar <input> --from-language <lang> --to-language <lang>` reads
  LiNo or native grammar surface text, applies the built-in grammar concept
  translation rules, and writes translated LiNo.

Unreadable files, empty inference corpora, parse failures, and not-yet-emitted
target formats are reported on stderr with a non-zero exit status.

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
