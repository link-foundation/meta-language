import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const ARRAYS = `const xs = [1, 2];
console.log(xs[1] + xs.length);
function sum(values) {
  let s = 0n;
  for (const v of values) s += v;
  return s;
}
console.log(sum([1n, ...[2n]]));
`;

const IN_BOUNDS = 'the translation agrees with the source on executions whose array reads are in bounds; JavaScript reads undefined at an index outside an array, where the target aborts';

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('arrays the program reads are a semantic translation in every target, assuming reads in bounds', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(ARRAYS, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
    assert.ok(translated.contract.assumptions.includes(IN_BOUNDS), target);
  }
});

test('an array is a Rust Vec, a Lean Array and a Rocq list, read through a bounds-checked helper', () => {
  const rust = translateProgram(ARRAYS, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('let xs = vec![1f64, 2f64];'), rust);
  assert.ok(rust.includes('crate::ml_array::at(&xs, crate::ml_array::number_index(1f64)) + (xs.len() as f64)'), rust);
  assert.ok(rust.includes('crate::ml_array::append(vec![crate::ml::Big::from_i128(1)], vec![crate::ml::Big::from_i128(2)])'), rust);
  const lean = translateProgram(ARRAYS, 'JavaScript', 'Lean').code;
  assert.ok(lean.includes('let xs := (#[(1 : Float), (2 : Float)] : (Array Float))'), lean);
  assert.ok(lean.includes('(ml_array_at_float xs (1 : Float)) + (Float.ofNat xs.size)'), lean);
  assert.ok(lean.includes('((#[(1 : Int)] : (Array Int)) ++ (#[(2 : Int)] : (Array Int)))'), lean);
  const rocq = translateProgram(ARRAYS, 'JavaScript', 'Rocq').code;
  assert.ok(rocq.includes('let xs := (1%float :: 2%float :: @nil float) in'), rocq);
  assert.ok(rocq.includes('(ml_list_at xs (ml_float_index 1%float) 0%float)'), rocq);
  assert.ok(rocq.includes('(List.app (1%Z :: @nil Z) (2%Z :: @nil Z))'), rocq);
});

test('inference fixes the element type of an empty array from its later use', () => {
  const source = 'function range(n) {\n  let out = [];\n  for (let i = 0n; i < n; i++) out = [...out, i];\n  return out;\n}\nconsole.log(range(3n).length);\n';
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.ok(rust.code.includes('Vec::<crate::ml::Big>::new()'), rust.code);
});

test('a @typedef field may hold an array, of data too', () => {
  const source = "/** @typedef {{ $: 'leaf', items: bigint[] } | { $: 'node', kids: Array<Tree> }} Tree */\nfunction size(t) {\n  if (t.$ === 'leaf') return t.items.length;\n  let n = 0;\n  for (const kid of t.kids) n += size(kid);\n  return n;\n}\nconsole.log(size({ $: 'node', kids: [{ $: 'leaf', items: [1n, 2n] }] }));\n";
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
  // Lean derives no DecidableEq through an Array of data, and Rocq would warn of every nested definition.
  assert.ok(translateProgram(source, 'JavaScript', 'Lean').code.includes('  deriving Repr, Inhabited'));
  assert.ok(translateProgram(source, 'JavaScript', 'Rocq').code.includes('#[warnings="-register-all"]\nInductive Tree'));
});

test('the array forms that are not kept are refused with a reason', () => {
  assert.equal(refusal('const xs = [1, 2];\nconsole.log(xs);\n'), 'text of an array: console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one at 31..33');
  assert.equal(refusal('const xs = [1, 2];\nxs[0] = 3;\n'), 'assignment of an array element: the portable core reads arrays and does not mutate them; build a new array with [...xs, value] at 19..24');
  assert.equal(refusal('const xs = [1, 2];\nconsole.log(xs === xs);\n'), 'comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40');
  assert.equal(refusal("const s = 'ab';\nconsole.log(s.length);\n"), undefined);
  assert.equal(refusal('const xs = [1, , 2];\n'), refusal('const xs = [1, , 2];\n'));
  assert.match(refusal('const xs = [1, , 2];\n'), /^array hole/u);
  assert.match(refusal('for (const k in { a: 1 }) console.log(k);\n'), /^for…in loop/u);
});
