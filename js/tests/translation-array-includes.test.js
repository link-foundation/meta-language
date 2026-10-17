import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

function observe(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return output;
}

test('array membership uses actual receiver and element types with SameValueZero observations', () => {
  const source = '/** @param {number[]} values @param {number} sought @returns {boolean} */function hasNumber(values,sought){return values.includes(sought);}\n'
    + '/** @param {string[]} values @param {string} sought @returns {boolean} */function hasText(values,sought){return values.includes(sought);}\n'
    + '/** @param {boolean[]} values @param {boolean} sought @returns {boolean} */function hasFlag(values,sought){return values.includes(sought);}\n'
    + 'function inferred(values,sought){return values.includes(sought);}\n'
    + "console.log(hasNumber([NaN],NaN),hasNumber([-0],0),hasNumber([0],-0),hasNumber([1],1),hasNumber([1],2));"
    + "console.log(hasText(['yes'],'yes'),hasText(['yes'],'no'),hasFlag([false],false),inferred(['x'],'x'));";
  const surface = parseJavaScript(source);
  assert.equal(surface.items[0].body.k, 'method');
  assert.equal(surface.items[1].body.k, 'method');
  const checked = checkProgram(surface);
  const expected = ['true true true true false', 'true false true true'];
  assert.deepEqual(observe(source), expected);
  assert.deepEqual(observe(emitJavaScript(checked).text), expected);
  // A distinct built-in supplies an independent SameValueZero oracle. This
  // observes its membership operation; it does not declare TypedArray source
  // construction to be supported by the portable frontend.
  assert.deepEqual(observe('const values=new Float64Array([NaN,-0,1]);console.log(values.includes(NaN),values.includes(0),values.includes(-0),values.includes(1),values.includes(2));'), ['true true true true false']);
  const rust = emitRust(checked).text;
  assert.ok(rust.includes('crate::ml_array::includes('));
  assert.ok(rust.includes('is_nan()'));
});

test('string membership keeps its raw StringTest shape and unconstrained string default', () => {
  const surface = parseJavaScript('/** @param {string} text @returns {boolean} */function typed(text){return text.includes("x");}\nfunction inferred(text){return text.includes("x");}');
  for (const fn of surface.items) {
    assert.equal(fn.body.k, 'stringTest');
    assert.equal(fn.body.op, 'includes');
    assert.deepEqual(fn.params[0].type, { kind: 'string' });
  }
  assert.doesNotThrow(() => emitRust(checkProgram(surface)));
});

test('nullable numeric membership preserves NaN equality and distinct null and undefined cases', () => {
  for (const [type, values, absent] of [
    ['number|null', '[NaN,null,-0]', 'null'],
    ['number|undefined', '[NaN,undefined,-0]', 'undefined'],
    ['number|null|undefined', '[NaN,null,undefined,-0]', 'null'],
  ]) {
    const source = '/** @param {(' + type + ')[]} values @param {' + type + '} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\n'
      + 'console.log(has(' + values + ',NaN),has(' + values + ',' + absent + '),has(' + values + ',0));';
    const checked = checkProgram(parseJavaScript(source));
    assert.deepEqual(observe(source), ['true true true']);
    assert.deepEqual(observe(emitJavaScript(checked).text), ['true true true']);
    const rust = emitRust(checked).text;
    assert.ok(rust.includes('(a.is_nan() && b.is_nan())'));
    assert.ok(rust.includes(type.includes('null|undefined') ? '(crate::MlOptional::Null, crate::MlOptional::Null)' : '(Some(a), Some(b))'));
  }
  const source = '/** @param {(number|null|undefined)[]} values @param {number|null|undefined} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\nconsole.log(has([null],undefined),has([undefined],null));';
  const checked = checkProgram(parseJavaScript(source));
  assert.deepEqual(observe(source), ['false false']);
  assert.deepEqual(observe(emitJavaScript(checked).text), ['false false']);
  assert.ok(emitRust(checked).text.includes('_ => false'));
});

test('array membership refuses incompatible element and structured identity searches through ordinary checks', () => {
  assert.throws(() => parseJavaScript('/** @param {number[]} values @returns {boolean} */function bad(values){return values.includes("x");}'), error => error.kind === 'type');
  for (const source of [
    '/** @param {number[][]} values @param {number[]} sought @returns {boolean} */function bad(values,sought){return values.includes(sought);}',
    '/** @param {(number[]|null)[]} values @param {number[]|null} sought @returns {boolean} */function bad(values,sought){return values.includes(sought);}',
  ]) {
    const surface = parseJavaScript(source);
    assert.throws(() => checkProgram(surface), error => error.kind === 'unsupported'
      && error.reason === 'array identity search: object identity must be represented before searching arrays of objects');
  }
});
