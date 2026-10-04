# Lark importer parity

Checks that the JavaScript Lark importer (`js/src/grammar-importers/lark.js`)
imports the same grammars as the Rust importer
(`rust/src/grammar/import/lark.rs`) and rejects the same inputs with the same
error messages, including UTF-8 byte offsets.

`cases.json` is a list of `[name, source]` pairs: the Rust unit and integration
test inputs plus edge cases for directives (`%ignore`, `%declare`, `%import`,
`%override`), rule priorities and inline rules, templates, counted `~`
repetition, string and character-class escapes, regex terminals, comments,
CRLF line endings, Unicode whitespace and unexpected characters.

Both programs print, for every case, the source format, start rule, each
distinct rule as `<kind> <expression>` in the runtime-neutral rendering used by
the shared parity fixtures, the rule documentation and the undefined
non-terminals, or the error kind and message.

## Run

From the repository root:

```sh
node experiments/run-rust-experiment.mjs experiments/lark-parity/render.rs \
  experiments/lark-parity/cases.json > /tmp/lark-parity-rust.txt
node experiments/lark-parity/render.mjs > /tmp/lark-parity-js.txt
diff -u /tmp/lark-parity-rust.txt /tmp/lark-parity-js.txt && echo 'Lark importers agree'
```

`render.rs` is built by `experiments/run-rust-experiment.mjs` in a scratch
crate under the system temporary directory that depends on the `meta-language`
crate by path with default features disabled and reuses `rust/Cargo.lock`, so
no manifest, lockfile or build output is kept in `experiments/`.

## Known representation difference

Rust keeps rules in a list, so a Lark grammar that defines the same rule name
twice lists it twice in `rule_names()` while `Grammar::rule` returns the first
definition. The JavaScript grammar is keyed by rule name and keeps the first
definition. Both programs therefore print each distinct rule name once.
