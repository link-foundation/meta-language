import assert from 'node:assert/strict';
import { test } from 'node:test';
import { execFileSync } from 'node:child_process';
import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

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
});

test('type queries refuse operands whose evaluation could be discarded', () => {
  for (const [source, reason] of [
    ["function fail() { throw new Error('preserve'); } console.log(typeof fail());", /typeof operand/u],
    ["function answer() { return 7; } console.log(typeof answer);", /function value/u],
    ["console.log(typeof [1]);", /typeof operand/u],
  ]) assert.match(translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message ?? '', reason, source);
});
