# Self-translation

Self-translation ports meta-language's own modules between JavaScript,
TypeScript and Rust with meta-language itself. Both packages have the same
library API and the same CLI command, and their results are the same byte for
byte:

```js
import { selfTranslate } from 'meta-language';
const { code, items } = selfTranslate(source, 'JavaScript', 'Rust');
```

```rust
let translation = meta_language::self_translate(&source, "Rust", "JavaScript")?;
```

```sh
meta-language translate --to rust [--from js] [--items] module.mjs
```

A language is named by name or extension (`rust`, `rs`, `ts`, `mjs`); without
`--from`, the CLI takes it from the file extension. `--items` prints one line
per top-level item instead of the translation.

## The output

A translation starts with a header that records the source language and the
SHA-256 and length of the source:

```
// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=… bytes=508
```

Each top-level item of the source then becomes one block:

- **translated**: the item went through links into the target language. Its
  source is kept above the code as `// |` lines, with the item's own hash:
  `// meta-language:translated JavaScript export_statement items=1 sha256=…`.
- **carried**: the translator cannot express the item in the target yet (it
  has no definition, or uses a construct the portable core does not cover).
  The item is kept as `// |` lines only, with the reason:
  `// meta-language:carried JavaScript import_statement (no definition)`.
  A carried item is a gap in the translator, reported in the item list and in
  the per-module report.
- **comment**: comment groups between items are copied as comments when the
  syntax fits both languages. A comment directly before an item travels with
  that item.

The definitions the translated code needs (helpers and, for Rust, an
`#![allow(...)]` line) sit once between `// meta-language:prelude begin` and
`// meta-language:prelude end`.

## Round trips

Translating back reads the provenance:

- a translated or carried block whose code still matches its hash is restored
  to its source text exactly (status `restored`);
- an edited block drops its provenance and is translated again (status
  `translated`), so edits made in the target language carry over;
- when the header names the target language and the hash of the restored text
  equals the header's, the result is the original source byte for byte.

So JavaScript → Rust → JavaScript and Rust → JavaScript → Rust are lossless
for unedited output, and a translation into the same language returns the
source unchanged. Behavior across languages is checked by calling the
translated functions in both runtimes (see below).

## The shared corpus

`parity/self-translation/cases.lino` lists the cases and calls in Links
Notation, in the manner of
[relative-meta-logic PR #184](https://github.com/link-foundation/relative-meta-logic/pull/184):

```
(case arithmetic-to-rust (source sources/arithmetic.mjs) (from JavaScript) (to Rust) (expected expected/arithmetic-to-rust.rs))
(call arithmetic-to-rust (javascript add) (rust add) (arguments (f64 2) (f64 3.5)) (result 5.5))
```

Each case has its expected output and an `.items.lino` file with one
`(item start end term status "reason")` link per item. Both runtimes check
every case (`js/tests/self-translation.test.js`,
`rust/tests/unit/self_translation.rs`). The Rust test compiles the translated
Rust with clippy at `-D warnings` and runs each call; the JavaScript test
imports the JavaScript side and runs the same calls. Meta-language's own
modules round-trip byte for byte in both tests.

`npm run check:self-translation` checks that the expected files match the
translator; `node js/scripts/generate-self-translation-cases.mjs` rewrites
them after an intended change.

The Unicode-escape and default-parameter decisions in
`js/src/translation/frontend-rules.js` also run inside the Rust translator.
`node js/scripts/generate-frontend-rules.mjs` translates the complete module
through the portable core, applies the shared emitter decorators in
`parity/self-translation/frontend-rules-decorators.lino`, and formats the
result as `rust/src/translation/frontend_rules.rs`. The adapters supply UTF-16
code units and token records to these generated decisions. CI runs
`npm run check:frontend-rules` to reject stale generated code.

## Translated against hand-written Rust

`js/scripts/generate-self-translation-report.mjs` translates every module of
`js/src` into Rust and measures it against its hand-written counterpart
(`rust/src/<module>.rs` or `rust/src/<module>/mod.rs`): the items by status,
the Rust functions written, how many the hand-written Rust defines under the
same name and how many are identical up to whitespace, and how many translated
code lines the hand-written Rust holds too. The report also accounts for every
UTF-8 source byte using the item ranges and whitespace between them, and lists
translated, carried, comment and layout byte counts. It rejects overlapping,
out-of-bounds or split UTF-8 ranges. A module with code outside those ranges is
listed as refused, including when a parser fallback returned no items, so zero
carried items cannot conceal omitted source. Each module is measured twice: by
the generic translation and by the translation with the shared emitter
[decorators](decorators.md) of `parity/self-translation/decorators.lino`
(`--decorators` names another set), so the report shows how much of the
difference the decorators close. The whole of `js/src` takes over an hour on
one runner, so `--shard K/N` measures one of N shards that hold about the same
number of source bytes, and `--list` prints a shard's modules. CI's
`Self-Translation Report` jobs in `ci.yml` run eight shards beside the
JavaScript and Rust workflows. Each shard publishes its part of the report in
its job summary and as a `self-translation-report-<sha>-<shard>` artifact.
The JSON records the observed commit and the SHA-256 hashes of each module and
its Rust counterpart. Both acceptance suites download all eight shards and run
`--verify-reports <directory> --commit <sha>` before recording the per-module
report assertion. Verification rejects missing or duplicate modules, stale
sources, failed translations, inconsistent measurements and missing Markdown
rows without translating the whole corpus again.

Decorators edit emitted lines; they cannot add what the translator carries
untranslated (imports, classes, exports the emitters do not cover), so for
most modules the report shows the difference that is left honestly rather
than hiding it.

## Decorated translations

`selfTranslate(source, from, to, { decorators })` (JavaScript) and
`self_translate_decorated(source, from, to, &decorators)` (Rust) apply emitter
decorators to the code they write before the provenance marker is hashed, so
the decorated translation still translates back to its source. The corpus case
`arithmetic-decorated-to-rust` translates `sources/arithmetic.mjs` with
`decorators.lino` and is checked, function by function, against
`hand-written/arithmetic.rs`; removing the decorators gives the generic
translation again.
