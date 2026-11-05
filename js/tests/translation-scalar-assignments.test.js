// Future complete controls. Never run during the whole requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createCallableSourceProgramLinks } from '../src/callable-source-program.js';
import { createScalarAssignmentProgramLinks } from '../src/scalar-assignment-program.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { planScalarAssignmentExpression, readScalarAssignmentOperation } from '../src/translation/scalar-assignment-rules.js';
const fixture = readProjectData(new URL('../../parity/fixtures/scalar-assignments.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node);
const observe = call => { try { return { kind: 'value', value: String(call()) }; } catch (error) { return { kind: 'abort', value: error.message }; } };
test('scalar assignment values writes and operand order survive codecs four targets and native execution', () => {
  const program = checkProgram(parseJavaScript(fixture.source)), packet = encodeTypedProgramLinks(program), js = emitJavaScript(program);
  for (const actual of [packet, createScalarAssignmentProgramLinks(), createCallableSourceProgramLinks()]) for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
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
  assert.ok(emitRust(program).text.includes('.borrow_mut()'));
});
test('the construction plan keeps earlier operands and separate logical continuations',()=>{
  for (const operator of ['=','+=','-=','*=','/=','%=','**=','&&=','||=','??=','++','--','&=','|=','^=','<<=','>>=','>>>=']) {
    const plan=planScalarAssignmentExpression(operator,false);
    assert.ok(readScalarAssignmentOperation(operator));assert.equal(plan.length%4,0);
    for(let at=0;at<plan.length;at+=4){
      const [code,first,second,third]=plan.slice(at,at+4),index=at/4;
      const references=code===2||code===10?[first]:code===4||code===5||code===7?[second,third]:code===6?[first,second]:code===8||code===9?[first,second,third]:[];
      for(const reference of references)assert.ok(Number.isSafeInteger(reference)&&reference>=0&&reference<index,operator+' actual preceding operand');
    }
  }
  // Exact former **= refusal retained in sequence-expressions.lino with its
  // original input/assertion and the normative assignment-operator correction.
  for (const operator of ['unknown',''])assert.deepEqual(planScalarAssignmentExpression(operator,false),[]);
  for (const operator of ['=','+=','&&=','??='])assert.deepEqual(planScalarAssignmentExpression(operator,true),[]);
  for (const operator of ['&&=','||='])assert.ok(planScalarAssignmentExpression(operator,false).filter((_,index)=>index%4===0).includes(9));
});
