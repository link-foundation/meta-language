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

test('shared typed graph paths stop at the semantic traversal budget', () => {
  for (const entry of fixture.resourceCases) {
    const network = LinkNetwork.fromLino(entry.lino);
    assert.ok([...network.links()].length < 1000);
    assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'JavaScript'), /resource limit/u);
  }
});

test('machine integer width edits are refused before target emission', () => {
  for (const width of fixture.invalidMachineIntegerWidths) {
    const entry = fixture.cases.find((entry) => entry.id === 'rust-add');
    const network = LinkNetwork.fromLino(entry.lino);
    const bits = network.fieldRelations().find((relation) => relation.label === 'bits').child;
    const value = network.link(bits);
    value.setMetadata(value.metadata().withDefinition(String(width)));
    assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'Rust'), /invalid machine integer type/u);
  }
});
