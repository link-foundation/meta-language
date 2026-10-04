import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const MATH = `/**
 * @param {number[]} xs
 * @returns {number}
 */
function spread(xs) {
  return Math.max(...xs) - Math.min(...xs);
}
console.log(spread([3, 1]), Math.round(-0.5), Math.max(1, 2, 3), Number.isInteger(Math.PI), Math.abs(-1));
`;

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('the exactly specified Math and Number functions are a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(MATH, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
});

test('Math functions are the targets\' own where they agree with JavaScript, and generated helpers where they do not', () => {
  const rust = translateProgram(MATH, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('(crate::ml_math::max_of(&xs) - crate::ml_math::min_of(&xs))'), rust);
  assert.ok(rust.includes('crate::ml_math::round((-0.5f64))'), rust);
  assert.ok(rust.includes('crate::ml_math::max(crate::ml_math::max(1f64, 2f64), 3f64)'), rust);
  assert.ok(rust.includes('crate::ml_math::is_integer(3.141592653589793f64)'), rust);
  assert.ok(rust.includes('((-1f64)).abs()'), rust);
  const lean = translateProgram(MATH, 'JavaScript', 'Lean').code;
  assert.ok(lean.includes('((xs.foldl ml_max (-1.0 / 0.0 : Float)) - (xs.foldl ml_min (1.0 / 0.0 : Float)))'), lean);
  assert.ok(lean.includes('(ml_round (-0.5 : Float))'), lean);
  assert.ok(lean.includes('(Float.abs (-1 : Float))'), lean);
  const rocq = translateProgram(MATH, 'JavaScript', 'Rocq').code;
  assert.ok(rocq.includes('(PrimFloat.sub (List.fold_left ml_max xs PrimFloat.neg_infinity) (List.fold_left ml_min xs PrimFloat.infinity))'), rocq);
  assert.ok(rocq.includes('Definition ml_trunc (x : float) : float :=\n  match Prim2SF x with'), rocq);
  assert.ok(rocq.includes('(PrimFloat.abs (PrimFloat.opp 1%float))'), rocq);
});

test('inference makes a parameter a Math function reads a Number', () => {
  const source = 'function half(n) {\n  return Math.floor(n / 2);\n}\nconsole.log(half(7));\n';
  assert.ok(translateProgram(source, 'JavaScript', 'Rust').code.includes('pub fn half(n: f64) -> f64 {'));
});

test('the Math and Number forms that are not kept are refused with a reason', () => {
  assert.equal(refusal('console.log(Math.floor(1n));\n'), 'Math.floor of int; it takes Numbers, and converts or rejects anything else at 23..25');
  assert.equal(refusal('console.log(Math.sin(1));\n'), 'Math.sin: the JavaScript standard library is outside the portable core at 12..23');
  assert.equal(refusal('console.log(Math.abs(1, 2));\n'), 'Math.abs with 2 arguments: Math.abs takes one Number at 12..26');
  assert.equal(refusal('const xs = [1, 2];\nconsole.log(Math.abs(...xs));\n'), 'spread argument: pass Math.abs its argument at 43..45');
  assert.equal(refusal('const m = Math.max;\n'), 'function value Math.max: functions and namespaces are only portable when a function is called at 10..18');
  assert.equal(refusal('function isNaN(x) {\n  return x;\n}\n'), 'declaration of isNaN: it shadows the JavaScript global isNaN, which the translation reads as the built-in; rename it at 9..14');
});
