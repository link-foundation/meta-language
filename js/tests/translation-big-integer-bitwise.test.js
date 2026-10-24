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
// Corrected with ECMAScript ApplyStringOrNumericBinaryOperator and BigInt
// unsignedRightShift: these original supported inputs throw during execution.
// Original inputs and the old "checkProgram throws" assertion remain recorded.
for (const source of fixture.refusals) test('retained mixed and unsigned input has faithful runtime error: ' + source, () => {
  const correction=readProjectData(new URL('../../parity/fixtures/numeric-operation-errors.lino',import.meta.url)).cases.find(row=>row.source===source);
  assert.equal(correction.previousExpectation,'checkProgram throws');
  const observe=code=>{try{runInNewContext(code.replace(/^export /gmu,'')+'\ncompute();');assert.fail('source must throw');}catch(error){return {name:error.name,message:error.message,present:Object.hasOwn(error,'message')};}};
  const expected={name:correction.errorClass,message:correction.abort,present:correction.messagePresent};
  assert.deepEqual(observe(source),expected);
  const program=checkProgram(parseJavaScript(source)),packet=encodeTypedProgramLinks(program);
  for(const target of ['JavaScript','Rust','Lean','Rocq']){
    const expected=emitTypedProgramLinks(packet.network,packet.root,target);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
  }
  assert.deepEqual(observe(emitJavaScript(program).text),expected);
});
