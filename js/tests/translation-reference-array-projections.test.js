// Future complete controls. Never run during the whole requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createReferenceArrayProjectionProgramLinks } from '../src/reference-array-projection-program.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { acceptReferenceArrayProjectionIndex, readReferenceArrayProjection, readReferenceArrayProjectionSuffix } from '../src/translation/reference-array-projection-rules.js';
const fixture = readProjectData(new URL('../../parity/fixtures/reference-array-projections.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node);
const observe = call => { try { return { kind: 'value', value: String(call()) }; } catch (error) { return { kind: 'abort', value: error.message }; } };
test('source array reads preserve receiver/key order, aliases and runtime errors across codecs and four targets', () => {
  const program = checkProgram(parseJavaScript(fixture.source)), packet = encodeTypedProgramLinks(program), js = emitJavaScript(program);
  for (const actual of [packet, createReferenceArrayProjectionProgramLinks()]) for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const expected = emitTypedProgramLinks(actual.network, actual.root, target);
    for (const network of [LinkNetwork.fromLino(actual.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])
      assert.deepEqual(emitTypedProgramLinks(network, actual.root, target), expected);
  }
  const names = fixture.cases.map(row => js.mappings.find(mapping => mapping.kind === 'function' && mapping.source === row.entry).target);
  const functions = runInNewContext('"use strict";\n' + js.text.replace(/^export /gmu, '') + '\n[' + names.join(',') + '];');
  for (const [index, row] of fixture.cases.entries()) {
    const expected = { kind: row.abort === null ? 'value' : 'abort', value: row.abort ?? row.value };
    assert.deepEqual(observe(() => runInNewContext('"use strict";\n' + row.source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 })), expected, row.name);
    assert.deepEqual(observe(() => functions[index]()), expected, row.name);
  }
  const rust = emitRust(program);
  assert.ok(rust.text.includes('.borrow().len()'));
  assert.ok(rust.text.includes('&*ml_reference_array'));
});
test('effectful typed keys keep the existing native payload-read path and fresh names cannot capture operands', () => {
  for (const kind of ['call', 'apply', 'cellRead', 'index', 'method', 'referenceRead', 'if'])
    assert.equal(acceptReferenceArrayProjectionIndex(kind, 'float', []), false, kind);
  assert.equal(acceptReferenceArrayProjectionIndex('binary', 'float', [true, false]), false);
  assert.equal(readReferenceArrayProjection('optionalIndex', 'referenceRead', 'array', true, 'option', false), '');
  assert.equal(readReferenceArrayProjection('length', 'referenceRead', 'string', true, 'float', true), '');
  assert.equal(readReferenceArrayProjectionSuffix(1, ['ml_reference_array1 + ml_reference_position2 + ml_reference_projection3']), 4);
});
