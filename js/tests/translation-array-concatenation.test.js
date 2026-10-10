import assert from 'node:assert/strict';
import { test } from 'node:test';
import { execFileSync } from 'node:child_process';
import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const source = `/** @param {number[]} left @param {number[]} right @returns {number} */
function combinedSize(left, right) { let order = 0; for (const value of left.concat(right, [])) order = order * 10 + value; return order; }
/** @param {number[][]} left @param {number[][]} right @returns {number} */
function nestedSize(left, right) { return left.concat(right).length; }
/** @param {number[]} values @returns {number} */
function copySize(values) { return values.concat().length; }
console.log(combinedSize([1, 2], [3]), nestedSize([[1, 2]], [[3]]), copySize([1, 2]));
`;

test('array concatenation preserves order, one-level spreading and zero arguments', () => {
  const expected = '123 2 2\n';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), expected);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), expected);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
});

test('scalar concat arguments and string receivers keep explicit diagnostics', () => {
  for (const source of [
    'function append() { return [1].concat(2).length; }',
    "function append() { return 'a'.concat('b'); }",
  ]) assert.ok(translateProgram(source, 'JavaScript', 'Rust').diagnostic, source);
});

test('zero-argument slicing copies immutable arrays without changing order or nesting', () => {
  const source = `/** @param {number[]} values @returns {number} */
function copyOrder(values) { let order = 0; for (const value of values.slice()) order = order * 10 + value; return order; }
/** @param {number[][]} values @returns {number} */
function copyNested(values) { let size = 0; for (const row of values.slice()) size = size + row.length; return size; }
/** @param {string[]} values @returns {string} */
function copyText(values) { let text = ''; for (const value of values.slice()) text = text + value; return text; }
/** @param {boolean[]} values @returns {number} */
function copyFlags(values) { let count = 0; for (const value of values.slice()) if (value) count = count + 1; return count; }
/** @returns {number[]} */
function observe() { console.log('once'); return [4, 5]; }
console.log(copyOrder([1, 2, 3]), copyOrder([]), copyNested([[1, 2], [], [3]]), copyText(['café', '😀']), copyFlags([true, false, true]), observe().slice().length);
`;
  const expected = 'once\n123 0 3 café😀 2 2\n';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), expected);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), expected);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
});

test('indexed slices and non-array copy receivers retain explicit diagnostics', () => {
  for (const source of [
    'function copy() { return [1].slice(0).length; }',
    'function copy() { return [1].slice(-1, 2).length; }',
    "function copy() { return 'a'.slice(); }",
    'function copy() { return 3.slice(); }',
  ]) assert.ok(translateProgram(source, 'JavaScript', 'Rust').diagnostic, source);
});

test('Array.from and Array.of preserve homogeneous construction, nesting and single evaluation', () => {
  const source = `/** @param {number[]} values @returns {number} */
function copyOrder(values) { let order = 0; for (const value of Array.from(values)) order = order * 10 + value; return order; }
/** @returns {number[][]} */
function constructNested() { return Array.of([1, 2], [], [3]); }
/** @returns {string[]} */
function constructText() { return Array.of('café', '😀'); }
/** @returns {boolean[]} */
function constructEmpty() { return Array.of(); }
/** @returns {number[]} */
function observe() { console.log('once'); return [4, 5]; }
console.log(copyOrder([1, 2, 3]), copyOrder([]), constructNested().length, constructText().length, constructEmpty().length, Array.from(observe()).length);
`;
  const expected = 'once\n123 0 3 2 0 2\n';
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), expected);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', emitJavaScript(checkProgram(parseJavaScript(source))).text], { encoding: 'utf8' }), expected);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
});

test('array construction refuses array-like objects, string iterators, mapping and mixed elements', () => {
  for (const source of [
    'function copy() { return Array.from().length; }',
    'function copy() { return Array.from([1], 2).length; }',
    "function copy() { return Array.from('café😀').length; }",
    'function copy() { return Array.from(2).length; }',
    "function copy() { return Array.of(1, 'a').length; }",
  ]) assert.ok(translateProgram(source, 'JavaScript', 'Rust').diagnostic, source);
});
