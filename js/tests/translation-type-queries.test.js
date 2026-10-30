import {readTypeQueryEvaluationPlan} from '../src/translation/type-query-evaluation-rules.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { execFileSync } from 'node:child_process';
import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { acceptTypeQueryOperand, readTypeQueryResult } from '../src/translation/frontend-rules.js';

const [nodeMajor,nodeMinor]=process.versions.node.split('.').map(Number);
assert.ok(nodeMajor===22&&nodeMinor>=15||nodeMajor===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');

const source = `/** @param {number} value @returns {string} */
function numberType(value) { return typeof value; }
/** @param {bigint} value @returns {string} */
function integerType(value) { return typeof value; }
/** @param {string} value @returns {string} */
function stringType(value) { return typeof value; }
/** @param {boolean} value @returns {string} */
function booleanType(value) { return typeof value; }
/** @param {number[]} value @returns {string} */
function arrayType(value) { return typeof value; }
console.log('%s', numberType(7), integerType(7n), stringType('ready'), booleanType(false), arrayType([]), typeof 7, typeof 7n, typeof false, typeof 'ready');
`;

test('type queries preserve JavaScript type names for bound values and literals', () => {
  const expected = 'number bigint string boolean object number bigint boolean string\n';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), expected);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), expected);
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const result = translateProgram(source, 'JavaScript', target);
    assert.equal(result.diagnostic, null, target);
  }
  const structural = `/** @param {{item:number}} value @returns {string} */
function recordType(value) { return typeof value; }
/** @param {[number,string]} value @returns {string} */
function tupleType(value) { return typeof value; }
/** @param {Map<string,number>} value @returns {string} */
function mapType(value) { return typeof value; }
/** @param {Set<number>} value @returns {string} */
function setType(value) { return typeof value; }
console.log('%s', recordType({item:2}), tupleType([2,'kept']), mapType(new Map([['kept',2]])), setType(new Set([2])));`;
  const structuralExpected = 'object object object object\n';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', structural], { encoding: 'utf8' }), structuralExpected);
  const checked = checkProgram(parseJavaScript(structural));
  for (const [name, kind] of [['recordType','record'], ['tupleType','tuple'], ['mapType','map'], ['setType','set']]) {
    const entry = checked.declarations.get(name);
    assert.equal(entry.params[0].type.kind, kind);
    assert.equal(entry.body.k, 'lit');
    assert.equal(entry.body.value, 'object');
    assert.equal(readTypeQueryResult(kind), 'object');
  }
  for (const kind of ['union','cell','unrepresented']) assert.equal(readTypeQueryResult(kind), '');
  for (const [target, emit] of [['JavaScript',emitJavaScript], ['Rust',emitRust], ['Lean',emitLean], ['Rocq',emitRocq]]) {
    const emitted = emit(checked);
    const functions = emitted.mappings.filter(mapping => mapping.kind === 'function');
    assert.deepEqual(functions.map(mapping => mapping.source).sort(), ['mapType','recordType','setType','tupleType'].sort(), target);
    for (const mapping of functions) {
      const targetName = mapping.target.split('.').at(-1).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
      const declaration = target === 'JavaScript' ? '^\\s*function\\s+'
        : target === 'Rust' ? '^\\s*pub\\s+(?:const\\s+)?fn\\s+'
          : target === 'Lean' ? '^\\s*(?:(?:partial|noncomputable|unsafe)\\s+)?def\\s+'
            : '^\\s*(?:Definition|Fixpoint|Function|with)\\s+';
      assert.match(emitted.text, new RegExp(declaration + targetName + '\\b', 'mu'), `${target}: ${mapping.source}`);
    }
    if (target === 'JavaScript') assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitted.text], { encoding: 'utf8' }), structuralExpected);
    else assert.match(emitted.text, /object/u, target);
  }
});

test('type queries retain former refusal operands and their actual evaluation', () => {
  // Former refusal originals stay verbatim. ECMAScript 2025 13.5.3.1
  // evaluates these operands; the new path explicitly captures their values.
  const originals = ["function fail() { throw new Error('preserve'); } console.log(typeof fail());",
    "console.log(typeof [1]);"];
  assert.equal(acceptTypeQueryOperand('call'), false);
  assert.equal(acceptTypeQueryOperand('array'), false);
  for (const [index, original] of originals.entries()) {
    for (const target of ['Rust','Lean','Rocq']) assert.equal(translateProgram(original, 'JavaScript', target).diagnostic, null, original);
    const emitted = emitJavaScript(checkProgram(parseJavaScript(original))).text;
    if (index === 0) {
      for (const text of [original, emitted]) assert.throws(() => execFileSync(process.execPath,
        ['--input-type=module','-e',text], { encoding:'utf8', stdio:['ignore','pipe','pipe'] }),
      error => /Error: preserve/u.test(String(error.stderr)));
    } else for (const text of [original, emitted]) assert.equal(execFileSync(process.execPath,
      ['--input-type=module','-e',text], {encoding:'utf8'}), 'object\n');
  }
  const source = 'function answer() { return 7; } console.log(typeof answer);';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), 'function\n');
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), 'function\n');
  assert.equal(translateProgram(source, 'JavaScript', 'Rust').diagnostic, null);
});

// Deferred complete typed-codec and four-emission controls, never run in drafting.
test('evaluated type queries keep effects through source-free typed reload', () => {
  const source = '/** @param {{value:number}} holder @returns {string} */ export function inspect(holder){return typeof update(holder)+":"+String(holder.value);} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 7;}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics:true}));
  const packet = encodeTypedProgramLinks(checked);
  const products = [checked, ...[LinkNetwork.fromLino(packet.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network =>
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram:true}))];
  const run = text => execFileSync(process.execPath,['--input-type=module','-e',text+'\nconsole.log(inspect({value:0}));'],{encoding:'utf8'});
  assert.equal(run(source),'number:9\n');
  for(const product of products) {
    assert.equal(run(emitJavaScript(product).text),'number:9\n');
    for(const emit of [emitRust,emitLean,emitRocq]) assert.ok(emit(product).definitions.length>0);
  }
});
test('type query evaluation distinguishes known, unknown and undefined domains', () => {
  for(const [input,expected] of [
    [[true,true,true,true,false],1], [[true,true,false,false,false],2],
    [[true,true,true,false,false],2], [[true,false,false,false,true],3],
    [[true,false,false,false,false],0], [[false,true,false,false,false],0],
    [[false,true,true,true,false],1], [[false,false,false,false,true],0]
  ]) assert.equal(readTypeQueryEvaluationPlan(...input),expected);
});
