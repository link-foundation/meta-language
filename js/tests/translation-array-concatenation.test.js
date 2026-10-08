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
