# meta-language (JavaScript)

[![JavaScript](https://github.com/link-foundation/meta-language/actions/workflows/js.yml/badge.svg)](https://github.com/link-foundation/meta-language/actions/workflows/js.yml)
[![npm](https://img.shields.io/npm/v/meta-language?label=npm&style=flat)](https://www.npmjs.com/package/meta-language)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D20-blue.svg)](https://nodejs.org/)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org/)

JavaScript implementation of the `meta-language` links-network core. It is the
JavaScript half of the [meta-language](../README.md) multi-language project; the
Rust crate lives in [`../rust`](../rust).

The package mirrors the Rust operation families used by the parity registry:
parse, query, transform, substitute, serialize, snapshot, translate, and verify.
It is intentionally dependency-light and uses `links-notation` for LiNo and
link-cli-style substitution text plus `peggy` for generated parser modules.
Lossless source-token links can be addressed as `LinkType.Token`, matching the
Rust `LinkType::Token` name; `LinkType.SourceToken` remains available for
existing JavaScript callers.

Feature parity with the Rust crate is enforced by
[`../parity/language-features.json`](../parity/language-features.json) and the
`npm run check:parity` gate (see [Parity](#parity) below).

See [`../docs/translation-rules.md`](../docs/translation-rules.md) for recursive
rule evaluation, captures, variadic and optional templates, rendering contexts,
and target-language fallbacks.

The shared executable query-plan IR and registry-driven GraphQL and SQL
adapters are documented in [`../docs/query-plans.md`](../docs/query-plans.md).

The package registers lossless tree-sitter CST frontends for JavaScript, Rust,
Lean, Rocq/Coq, and the broader programming/data-language inventory, including
aliases, grammar diagnostics, UTF-8 spans, named fields, and ordered token
emission. The exact version profiles and the explicit boundary between
concrete syntax, opaque extensions, unavailable semantic layers, and the 12
unfinished fail-closed translation hooks are documented in
[`../docs/four-language-contracts.md`](../docs/four-language-contracts.md). The
full delivery checklist and remaining fallback paths are tracked in
[`../docs/issue-195-requirement-ledger.md`](../docs/issue-195-requirement-ledger.md).

## Usage

```js
import {
  LinkNetwork,
  LinkQuery,
  ParseConfiguration,
  ReplacementRule,
} from 'meta-language';

const network = LinkNetwork.parse(
  'const oldName = call(oldName);\n',
  'JavaScript',
  ParseConfiguration.default(),
);
const query = LinkQuery.fromSexpression(`
  (identifier) @target
  (#eq? @target "oldName")
`);

network.replace(
  network.find(query),
  ReplacementRule.capturedText('target', 'newName'),
);

console.log(network.reconstructText());
// const newName = call(newName);
```

## Command line

The package installs a `meta-language` command whose `grammar` subcommand
imports, validates, converts, merges, renames, exports and round-trips grammars.
It prints the same output and exits with the same status as the Rust
`meta-language grammar` command:

```bash
npx meta-language grammar formats
npx meta-language grammar convert --from pest --to ebnf message.pest
npx meta-language grammar merge --source bnf:message.bnf --source ebnf:message.ebnf
npx meta-language grammar round-trip --from abnf --accept "1+2" --reject "1+x" sum.abnf
```

The same commands are available as a library through `runGrammarCommand(args,
{ readFile })`, and the steps behind them as `grammarImporter`,
`grammarEmitter`, `validateGrammar`, `renderNativeGrammar` and
`parseNativeGrammar`. `meta-language grammar help` lists every command.

`checkGrammarReverseConversion(source, { importGrammar, emitGrammar, accepts,
rejects })` checks the cycle source grammar -> native links -> exported grammar
-> native links. The emitter only sees the grammar read back from
`renderGrammarLinks`, and the re-imported export must keep every rule, kind,
definition and documentation comment and give the same answers on the accept
and reject samples. For an exact copy of the source, `importGrammarLossless(source,
format)` also returns a layout (definition texts, comments and spacing) that
`renderGrammarLayoutLinks` writes as links. `emitGrammarLossless(grammar,
layout)` then rebuilds the source byte for byte and writes only changed or new
rules fresh.

`lowerGrammar(grammar, format)` writes a grammar in a notation that cannot
express all of it (abnf, antlr, bnf, ebnf, gbnf, lark, pest or
tree-sitter-json). Each construct the target cannot write moves into a helper
rule encoded in constructs it can write. For example, an optional item becomes a
choice with the empty expression and a repetition becomes a recursive rule. The
result has two parts: the executable text, which the target's own importer
reads back, and reconstruction metadata as links. The metadata names every
helper, its construct, whether its encoding is exact or approximate and the
original expression, plus every rename, kind and documentation the target does
not keep. `reconstructGrammar(executable, metadata)` rebuilds the original from
both parts. `checkGrammarLowering(grammar, format, { accepts, rejects })`
reports every emission note, every executable that does not read back, every
feature the reconstruction lost and every sample an exact lowering disagrees
on.

`translateProgram(source, from, to)` carries every Lean and Rocq theorem as a
proof obligation in `semantics.obligations`. The obligation names the source
theorem and the target declaration that states it, so a false restatement is
rejected by the source kernel and by every target. Into Lean and Rocq the
obligation is discharged by the target kernel itself (`discharge:
'target-kernel'`). The emitted proof has no `sorry`, `admit` or axiom, and
`#print axioms` or `Print Assumptions` shows it closed without an axiom of the
translation or of another kernel. Into JavaScript and Rust a theorem is a
bounded check (`discharge: 'source-kernel'`, `check: 'bounded'`), not a proof.
Run with `--ml-check-theorems`, the program prints `theorem <name>: holds on the
bounded domain` or fails, and the proof stays with the source kernel. A
statement that holds on the bounded domain but not in general therefore passes
the check while every kernel rejects it.

## Parity

Every feature in [`../parity/language-features.json`](../parity/language-features.json)
must be implemented in both Rust and JavaScript. `npm run check:parity` validates
the manifest, confirms each cell's evidence files exist, and asserts the
JavaScript `API_OPERATIONS` registry covers every operation family and API style.
The same check runs in both `js.yml` and `rust.yml`, so a change to one language
that is not mirrored in the other fails CI.

## Development

```bash
npm ci
npm test
npm run check:parity
npm pack --dry-run
```
