import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';
import { checkRuntimeLock } from '../scripts/build-web-tree-sitter-runtime.mjs';
import { normalize, renderNetwork } from './support/cst-sexpression.js';

const recovery = JSON.parse(readFileSync(
  new URL('../../parity/fixtures/issue-195-conformance/error-recovery.json', import.meta.url),
));

test('the vendored UTF-8 web-tree-sitter runtime matches its lock and the pinned npm package', async () => {
  assert.deepEqual(await checkRuntimeLock(), []);
});

test('malformed input recovers to the same tree as the native tree-sitter runtime', () => {
  for (const fixture of recovery.cases) {
    const network = LinkNetwork.parse(fixture.source, fixture.language);
    assert.equal(normalize(renderNetwork(network, fixture.language)), fixture.expected, fixture.id);
    assert.notEqual(fixture.expected, fixture.utf16Tree, fixture.id);
    assert.equal(network.reconstructText(), fixture.source, fixture.id);
    assert.equal(network.verifyFullMatch().isClean(), false, fixture.id);
  }
});
