# meta-language (JavaScript)

[![JavaScript](https://github.com/link-foundation/meta-language/actions/workflows/js.yml/badge.svg)](https://github.com/link-foundation/meta-language/actions/workflows/js.yml)
[![npm](https://img.shields.io/npm/v/meta-language?label=npm&style=flat)](https://www.npmjs.com/package/meta-language)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D22-blue.svg)](https://nodejs.org/)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org/)

JavaScript implementation of the `meta-language` links-network core. It is the
JavaScript half of the [meta-language](../README.md) multi-language project; the
Rust crate lives in [`../rust`](../rust).

The package mirrors the Rust operation families used by the parity registry:
parse, query, transform, substitute, serialize, snapshot, translate, and verify.
It is intentionally dependency-light and uses `links-notation` for LiNo and
link-cli-style substitution text. Grammars run on the package's own executor
([grammar feature union](../docs/grammar/feature-union.md)); `peggy` is only a
development dependency of the tests that compare against it.
Lossless source-token links can be addressed as `LinkType.Token`, matching the
Rust `LinkType::Token` name; `LinkType.SourceToken` remains available for
existing JavaScript callers.

Feature parity with the Rust crate is enforced by
[`../parity/language-features.lino`](../parity/language-features.lino) and the
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

The package requires Node.js 22 or later.

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

### Parse HTML with source positions

`LinkNetwork.parse(source, 'HTML')` returns the HTML concrete syntax tree as a
`LinkNetwork`. The native grammar is identified by `native-html` in
`network.parseGrammars()`. npm version `0.46.0` predates this grammar. To try it
from a checkout containing the grammar, run `npm ci --prefix js` and import from
`./js/src/index.js` in a script at the repository root.

```js
import { LinkNetwork, LinkType } from 'meta-language';

const source = '<article lang="fr">été <em>🙂</em></article>';
const network = LinkNetwork.parse(source, 'HTML');
if (!network.parseGrammars().some(({ id }) => id === 'native-html')) {
  throw new Error('This package version needs the native HTML grammar');
}

const isHtmlNode = (link) => {
  const { linkType, language, named } = link.metadata();
  return linkType === LinkType.Syntax && language === 'HTML' && named;
};
const children = (link) => link.references().map((id) => network.link(id)).filter(isHtmlNode);
const kind = (link) => link.metadata().term;
const raw = (link) => network.capturedText(link.id());

const document = network.links().filter(isHtmlNode).find((link) => kind(link) === 'document');
const article = children(document).find((link) => kind(link) === 'element');
const opening = children(article).find((link) => kind(link) === 'start_tag');
const tag = raw(children(opening).find((link) => kind(link) === 'tag_name'));
const attributes = children(opening).filter((link) => kind(link) === 'attribute').map(raw);
const content = children(article).filter((link) => ['text', 'element'].includes(kind(link)));
console.log(tag, attributes, content.map(raw));
// article [ 'lang="fr"' ] [ 'été', '<em>🙂</em>' ]

const { start, end } = content[0].metadata().span.byteRange;
const bytes = Buffer.from(source, 'utf8');
const toStringOffset = (offset) => bytes.subarray(0, offset).toString('utf8').length;
console.log([start, end], [toStringOffset(start), toStringOffset(end)]);
// [ 19, 24 ] [ 19, 22 ]
```

Each node is a `Link` with `id()`, ordered `references()` to child link IDs,
and `metadata()`. The metadata gives the node kind as `term`, the `language`,
whether it is `named`, recovery `flags`, and its `span`. Resolve each child
with `network.link(id)`. Named children of `document` include elements and
other top-level syntax. An ordinary `element` has a `start_tag`, ordered content,
and an `end_tag` when one is present. Void elements can have just a `start_tag`.
An `attribute` has an `attribute_name` and, if assigned, an `attribute_value`
or a `quoted_attribute_value` that can contain an inner `attribute_value`.
Traverse nested elements in the same way. The tree can also contain
`script_element`, `style_element`, `raw_text`, `entity` and `comment` nodes.

`span.byteRange` uses half-open UTF-8 byte offsets. `span.start` and `span.end`
each contain a zero-based `row` and UTF-8 byte `column`. Convert byte boundaries
to UTF-16 code-unit offsets before using `source.slice()`, as shown above.
`capturedText(id)` returns the original spelling within a selected node's span,
including entity spelling and attribute quotes when applicable. Named-child
traversal omits punctuation and some whitespace. Use `network.reconstructText()`
for the complete source and `network.verifyFullMatch().isClean()` to check for
parse and recovery issues before importing the document.

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

Every feature in [`../parity/language-features.lino`](../parity/language-features.lino)
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
