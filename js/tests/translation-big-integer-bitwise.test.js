// Future controls only; no execution is authorized before the complete draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
const fixture = readProjectData(new URL('../../parity/fixtures/big-integer-bitwise.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixture.versions.JavaScript);
const values = fixture.valueSources.map(source => runInNewContext(source));
const shifts = fixture.shiftSources.map(source => runInNewContext(source));
for (const operation of fixture.operations) test(operation.name + ': original BigInt operators and both codecs', () => {
  const source = operation.source + operation.assignmentSource, program = checkProgram(parseJavaScript(source));
  const packet = encodeTypedProgramLinks(program);
  for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const original = emitTypedProgramLinks(packet.network, packet.root, target);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      assert.deepEqual(emitTypedProgramLinks(network, packet.root, target), original);
    }
  }
  for (const name of [operation.name, operation.assignmentName].filter(Boolean)) {
    const original = runInNewContext(source.replace(/^export /gmu, '') + '\n' + name + ';');
    const translated = runInNewContext(emitJavaScript(program).text.replace(/^export /gmu, '') + '\n' + name + ';');
    const rightValues = operation.symbol === '~' ? [0n] : operation.symbol === '<<' || operation.symbol === '>>' ? shifts : values;
    for (const left of values) for (const right of rightValues) assert.equal(translated(left, right), original(left, right));
  }
});
for (const source of fixture.refusals) test('mixed and unsigned domains retain explicit obligations: ' + source, () => {
  assert.throws(() => runInNewContext(source.replace(/^export /gmu, '') + '\ncompute();'), error => error.name === 'TypeError');
  assert.throws(() => checkProgram(parseJavaScript(source)));
});
