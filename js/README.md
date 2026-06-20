# @link-foundation/meta-language

JavaScript implementation of the `meta-language` links-network core.

The package mirrors the Rust operation families used by the parity registry:
parse, query, transform, substitute, serialize, snapshot, translate, and verify.
It is intentionally dependency-light and uses `links-notation` for LiNo and
link-cli-style substitution text plus `peggy` for generated parser modules.

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

## Development

```bash
npm ci
npm test
npm run check:parity
npm pack --dry-run
```
