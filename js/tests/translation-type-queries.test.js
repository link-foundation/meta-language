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
import { readTypeQueryResult } from '../src/translation/frontend-rules.js';

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

test('type queries refuse operands whose evaluation could be discarded', () => {
  for (const [source, reason] of [
    ["function fail() { throw new Error('preserve'); } console.log(typeof fail());", /typeof operand/u],
    ["console.log(typeof [1]);", /typeof operand/u],
  ]) assert.match(translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message ?? '', reason, source);
  const source = 'function answer() { return 7; } console.log(typeof answer);';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), 'function\n');
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), 'function\n');
  assert.equal(translateProgram(source, 'JavaScript', 'Rust').diagnostic, null);
});
