import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import { typedProgramLinksFixture } from '../scripts/generate-typed-program-links-fixtures.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { emitTypedProgramLinks } from '../src/typed-program-links.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/typed-program-links.json', import.meta.url), 'utf8'));
test('bounded typed graph fixtures reload without source and preserve behavior', () => {
  assert.deepEqual(typedProgramLinksFixture(), fixture);
  for (const entry of fixture.cases) {
    const network = LinkNetwork.fromLino(entry.lino);
    const emitted = emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'JavaScript');
    const output = [];
    vm.runInNewContext(emitted.code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
    assert.deepEqual(output, entry.output);
    assert.equal(emitted.code, entry.javascript);
  }
});
