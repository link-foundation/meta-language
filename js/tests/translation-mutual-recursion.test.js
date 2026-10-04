import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const EVEN_ODD = `/** @param {bigint} n @returns {boolean} */
function isEven(n) { if (n === 0n) return true; return isOdd(n - 1n); }
/** @param {bigint} n @returns {boolean} */
function isOdd(n) { if (n === 0n) return false; return isEven(n - 1n); }
console.log(isEven(10n));
`;

test('mutually recursive functions are a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(EVEN_ODD, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
});

test('a Lean mutual group is a mutual block of partial defs', () => {
  const translated = translateProgram(EVEN_ODD, 'JavaScript', 'Lean');
  assert.ok(translated.semantics.encodings.some((encoding) => encoding.id === 'mutual-recursion'));
  assert.match(translated.code, /\nmutual\n\npartial def isEven \(n : Int\) : Bool :=\n[^]*\n\npartial def isOdd \(n : Int\) : Bool :=\n[^]*\n\nend\n/u);
});

test('a Rocq mutual group is one ml_fix over the sum of the parameter tuples, projected to each function', () => {
  const translated = translateProgram(`/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function first(n, acc) { if (n === 0n) return acc; return second(n - 1n, acc + 1n); }
/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function second(n, acc) { if (n === 0n) return acc; return third(n - 1n, acc + 10n); }
/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function third(n, acc) { if (n === 0n) return acc; return first(n - 1n, acc + 100n); }
console.log(first(10n, 0n));
`, 'JavaScript', 'Rocq');
  assert.ok(translated.semantics.encodings.some((encoding) => encoding.id === 'mutual-recursion'));
  const code = translated.code;
  assert.ok(code.includes('Definition ml_mutual_first : ((Z * Z) + ((Z * Z) + (Z * Z))) -> (Z + (Z + Z)) :=\n  ml_fix 64 (fun (ml_rec : ((Z * Z) + ((Z * Z) + (Z * Z))) -> (Z + (Z + Z))) (ml_args : ((Z * Z) + ((Z * Z) + (Z * Z)))) =>\n    match ml_args with\n    | (inl (n, acc)) => '), code);
  assert.ok(code.includes('(match ml_rec (inr (inl ((Z.sub n 1%Z), (Z.add acc 1%Z)))) with (inr (inl ml_r)) => ml_r | _ => 0%Z end)'), code);
  assert.ok(code.includes('(fun ml_args => match ml_args with (inl _) => (inl 0%Z) | (inr (inl _)) => (inr (inl 0%Z)) | (inr (inr _)) => (inr (inr 0%Z)) end).'), code);
  assert.ok(code.includes('Definition third (n : Z) (acc : Z) : Z :=\n  match ml_mutual_first (inr (inr (n, acc))) with (inr (inr ml_r)) => ml_r | _ => 0%Z end.'), code);
});
