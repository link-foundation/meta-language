// Future normative controls; no execution during the complete source draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { acceptResourceDescriptionIdentifierText } from '../src/resource-description-identifiers.js';
import { importResourceDescriptionNTriples, readResourceDescriptionNTriplesSource, emitResourceDescriptionNTriples } from '../src/resource-description-ntriples.js';
import { decodeResourceDescriptionGraph } from '../src/resource-description-graphs.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
const fixture = readProjectData(new URL('../../parity/fixtures/resource-description-addresses.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixture.versions.JavaScript);
for (const value of fixture.valid) test('the RFC address grammar preserves ' + value, () => {
  assert.equal(acceptResourceDescriptionIdentifierText(value), true);
  const source = '<' + value + '> <urn:predicate> "value" .\n';
  const original = importResourceDescriptionNTriples(source, { sourceIdentifier: 'address-fixture' });
  for (const network of [original.network, LinkNetwork.fromLino(original.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network))]) {
    assert.equal(readResourceDescriptionNTriplesSource(network, original.syntaxRoot), source);
    assert.equal(decodeResourceDescriptionGraph(network, original.root).triples[0][0].value, value);
    assert.equal(emitResourceDescriptionNTriples(network, original.root).source, source);
  }
});
for (const value of fixture.invalid) test('the RFC address grammar refuses ' + value, () => {
  assert.equal(acceptResourceDescriptionIdentifierText(value), false);
  assert.throws(() => importResourceDescriptionNTriples('<' + value + '> <urn:predicate> "value" .\n', { sourceIdentifier: 'address-fixture' }));
});
