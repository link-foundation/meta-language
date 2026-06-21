# @link-foundation/meta-language (JavaScript)

[![JavaScript](https://github.com/link-foundation/meta-language/actions/workflows/js.yml/badge.svg)](https://github.com/link-foundation/meta-language/actions/workflows/js.yml)
[![npm](https://img.shields.io/npm/v/@link-foundation/meta-language?label=npm&style=flat)](https://www.npmjs.com/package/@link-foundation/meta-language)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D20-blue.svg)](https://nodejs.org/)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org/)

JavaScript implementation of the `meta-language` links-network core. It is the
JavaScript half of the [meta-language](../README.md) multi-language project; the
Rust crate lives in [`../rust`](../rust).

The package mirrors the Rust operation families used by the parity registry:
parse, query, transform, substitute, serialize, snapshot, translate, and verify.
It is intentionally dependency-light and uses `links-notation` for LiNo and
link-cli-style substitution text plus `peggy` for generated parser modules.

Feature parity with the Rust crate is enforced by
[`../parity/language-features.json`](../parity/language-features.json) and the
`npm run check:parity` gate (see [Parity](#parity) below).

## Usage

```js
import {
  LinkNetwork,
  LinkQuery,
  ParseConfiguration,
  ReplacementRule,
} from '@link-foundation/meta-language';

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
