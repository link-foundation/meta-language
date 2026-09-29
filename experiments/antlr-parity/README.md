# ANTLR importer parity

Checks that the JavaScript ANTLR v4 importer
(`js/src/grammar-importers/antlr.js`) produces the same grammar as the Rust
importer (`rust/src/grammar/import/antlr.rs`).

Both programs import the three Rust fixtures in
`rust/tests/fixtures/grammar/antlr/` and every case of `cases.txt` (a case
starts with a `### name` line), then print the start rule and every rule as
`rule <name>: <kind> <expression> | doc: <doc>` in the runtime-neutral
rendering of `rust/tests/unit/grammar_render.rs` and
`js/tests/support/render-grammar-expression.js`, or the error message when the
import fails.

From this directory:

    node ../run-rust-experiment.mjs render.rs > /tmp/antlr-rust.txt
    node render.mjs > /tmp/antlr-js.txt
    diff /tmp/antlr-rust.txt /tmp/antlr-js.txt && echo identical

The outputs are identical. The cases cover the unit test inputs of
`rust/tests/unit/grammar_import_antlr.rs`, headers, skipped directives,
comment documentation, suffixes and non-greedy markers, labels, negation,
empty alternatives, actions and predicates, lexer commands, string and
character-set escapes, and every lexer and parser error (including the UTF-8
byte offsets and Rust `Debug` escaping in the messages).

One difference is left out of `cases.txt` on purpose: Rust keeps rules in a
list, so a grammar that defines a rule name twice lists it twice, while the
JavaScript grammar is keyed by name and keeps only the first definition. Both
resolve the name to the first definition.
