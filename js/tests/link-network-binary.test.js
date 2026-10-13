import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../src/network.js';
import { ByteRange, LinkMetadata, LinkType } from '../src/primitives.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary, BinaryLinksError } from '../src/link-network-binary.js';
import { transcodeLinkNetworkBinary } from '../src/link-network-binary-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/link-network-binary.lino', import.meta.url));

test('complete graph reconstruction preserves IDs, cycles, metadata, exact caches and deleted allocation history', () => {
  const parse = LinkNetwork.parse;
  LinkNetwork.parse = () => { throw new Error('source parsing cannot supply a binary snapshot'); };
  let restored;
  try { restored = decodeLinkNetworkBinary(Uint8Array.from(fixture.version2Bytes)); } finally { LinkNetwork.parse = parse; }
  assert.equal(restored.toLino(), fixture.graph);
  assert.deepEqual([...restored._terms].map(([term, id]) => [term, id.asU64()]).sort(), [...fixture.termMap].sort());
  assert.deepEqual([...restored._conceptSyntax].sort(), [...fixture.selectedSyntax].sort());
  assert.equal(restored.findTerm('original').asU64(), 1);
  assert.equal(restored.findTerm('renamed'), undefined);
  assert.equal(restored.reconstructConcept('meaning', 'JavaScript'), 'selected syntax');
  assert.deepEqual([...encodeLinkNetworkBinary(restored)], fixture.version2Bytes);
  assert.equal(restored.insertLink().asU64(), fixture.nextIdentity);
});

test('legacy MLDSNP01 retains its recorded graph and registration semantics', () => {
  const restored = decodeLinkNetworkBinary(fixture.legacyVersion1Bytes);
  assert.deepEqual(restored.links(), LinkNetwork.fromLino(fixture.graph).links());
  assert.equal(restored.findTerm('original'), undefined);
  assert.equal(restored.findTerm('renamed'), undefined);
  assert.equal(restored.reconstructConcept('meaning', 'JavaScript'), undefined);
  assert.equal(restored._nextId, restored.links().at(-1).id().asU64() + 1);
  assert.equal(new TextDecoder().decode(encodeLinkNetworkBinary(restored).slice(0, 8)), 'MLDSNP02');
});

test('reloaded graph edits change byte output and survive another reconstruction', () => {
  const restored = decodeLinkNetworkBinary(fixture.version2Bytes);
  restored.link(1).setReferences([1, 2, 1]);
  restored.setTerm(2, 'edited linked relation');
  const changed = encodeLinkNetworkBinary(restored);
  assert.notDeepEqual([...changed], fixture.version2Bytes);
  assert.deepEqual(decodeLinkNetworkBinary(changed).link(1).references().map(Number), [1, 2, 1]);
  assert.equal(decodeLinkNetworkBinary(changed).link(2).metadata().term, 'edited linked relation');
});

test('unsafe numeric IDs, spans, isolated surrogates and ambiguous cache keys are refused precisely', () => {
  const network = new LinkNetwork();
  const id = network.insertTypedPoint(LinkType.Concept, 'safe');
  network._nextId = Number.MAX_SAFE_INTEGER + 1;
  assert.throws(() => encodeLinkNetworkBinary(network), /unsafe or invalid next identity/);
  network._nextId = 2;
  network.link(id).setMetadata(network.link(id).metadata().withSpan({ byteRange: new ByteRange(0, Number.MAX_SAFE_INTEGER + 1), start: { row: 0, column: 0 }, end: { row: 0, column: 0 } }));
  assert.throws(() => encodeLinkNetworkBinary(network), /unsafe span value/);
  network.link(id).setMetadata(LinkMetadata.new().withTerm('\uD800'));
  assert.throws(() => encodeLinkNetworkBinary(network), /isolated surrogate/);
  network.link(id).setMetadata(LinkMetadata.new());
  network._conceptSyntax.set('a\0b\0c', 'selected');
  assert.throws(() => encodeLinkNetworkBinary(network), /ambiguous concept-syntax/);
});

test('invalid framing, truncation, trailing bytes and hostile counts are bounded', () => {
  const bytes = Uint8Array.from(fixture.version2Bytes);
  const header = bytes.slice(); header[0] = 0;
  assert.throws(() => decodeLinkNetworkBinary(header), /invalid archive header/);
  const version = bytes.slice(); version[7] = 51;
  assert.throws(() => decodeLinkNetworkBinary(version), /unsupported archive version/);
  assert.throws(() => decodeLinkNetworkBinary(bytes.slice(0, -1)), BinaryLinksError);
  assert.throws(() => decodeLinkNetworkBinary([...bytes, 0]), /trailing bytes/);
  const count = bytes.slice(); count.fill(255, 16, 24);
  assert.throws(() => decodeLinkNetworkBinary(count), /unsafe record count/);
  assert.throws(() => decodeLinkNetworkBinary(bytes, { byteLimit: 32 }), /byte resource limit/);
  assert.throws(() => encodeLinkNetworkBinary(new LinkNetwork(), { byteLimit: 16 }), /byte resource limit/);
  assert.throws(() => encodeLinkNetworkBinary(decodeLinkNetworkBinary(bytes), { recordLimit: 1 }), /record resource limit/);
  assert.throws(() => decodeLinkNetworkBinary([77, 76, 68, 83, 78, 80, 48, 50, 256]), BinaryLinksError);
});

test('both shared schemas agree and the complete portable framing source is admitted by the checked translator', async () => {
  assert.equal(readFileSync(new URL('../src/data/link-network-binary-schema.lino', import.meta.url), 'utf8'),
    readFileSync(new URL('../../rust/src/data/link-network-binary-schema.lino', import.meta.url), 'utf8'));
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const source = readFileSync(new URL('../src/link-network-binary-rules.js', import.meta.url), 'utf8');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  assert.ok(program.items.length >= 8);
  assert.equal(transcodeLinkNetworkBinary([], true, 1, 32).startsWith('-3,'), true);
});
