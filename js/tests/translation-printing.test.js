import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const LOUD = `function loud(x) {
  console.log('loud');
  return x * 2n;
}
console.log(loud(5n));
`;

test('a function that prints is a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(LOUD, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
});

test('Rust prints where the source prints', () => {
  const code = translateProgram(LOUD, 'JavaScript', 'Rust').code;
  assert.ok(code.includes('pub fn loud(x: crate::ml::Big) -> crate::ml::Big {\n    {\n        println!("{}", String::from("loud"));\n        x.mul(&crate::ml::Big::from_i128(2))\n    }\n}'), code);
});

test('Lean threads the lines printed through a function that prints', () => {
  const translated = translateProgram(LOUD, 'JavaScript', 'Lean');
  assert.ok(translated.semantics.encodings.some((encoding) => encoding.id === 'output-threading'));
  const code = translated.code;
  assert.ok(code.includes('inductive ml_io1 where\n  | ml_io1_mk (output : List String) (value : Int) : ml_io1\n  deriving Inhabited'), code);
  assert.ok(code.includes('def loud (x : Int) (ml_out : List String) : ml_io1 :=\n  (let ml_o1 := ("loud" :: ml_out);'), code);
  assert.ok(code.includes('.reverse do IO.println ml_line'), code);
});

test('Rocq threads the lines printed through a function that prints', () => {
  const translated = translateProgram(LOUD, 'JavaScript', 'Rocq');
  assert.ok(translated.semantics.encodings.some((encoding) => encoding.id === 'output-threading'));
  const code = translated.code;
  assert.ok(code.includes('Definition loud (x : Z) (ml_out : list string) : ml_io1 :=\n  (let ml_o1 := ("loud"%string :: ml_out) in (ml_io1_mk ml_o1 (Z.mul x 2%Z))).'), code);
  assert.ok(code.includes('app (List.rev (match ml_run2 with | ml_io2_mk ml_o7 ml_v8 => ml_o7 end))'), code);
});

test('a program that prints only in main threads nothing', () => {
  const translated = translateProgram('console.log(1n + 2n);\n', 'JavaScript', 'Lean');
  assert.ok(!translated.semantics.encodings.some((encoding) => encoding.id === 'output-threading'));
  assert.ok(!translated.code.includes('ml_io'), translated.code);
});

test('an assertion over a mutually recursive function runs when main runs in Lean', () => {
  const translated = translateProgram(`import assert from 'node:assert/strict';
function isEven(n) { if (n === 0n) return true; return isOdd(n - 1n); }
function isOdd(n) { if (n === 0n) return false; return isEven(n - 1n); }
const odd = isOdd(3n);
assert(odd);
`, 'JavaScript', 'Lean');
  assert.equal(translated.diagnostic, null);
  assert.ok(translated.code.includes('if !odd then throw (IO.userError "assertion 1 failed")'), translated.code);
});

test('an operand that prints in a compound assertion is refused with a reason', () => {
  const translated = translateProgram(`import assert from 'node:assert/strict';
function loud(x) { console.log('loud'); return x; }
assert(loud(true) && true);
`, 'JavaScript', 'Lean');
  assert.equal(translated.diagnostic?.kind, 'unsupported');
  assert.match(translated.diagnostic.message, /^output in a compound assertion: /u);
});
