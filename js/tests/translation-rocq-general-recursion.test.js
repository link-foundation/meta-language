import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

test('recursion Rocq cannot check terminates is a Definition over ml_fix, not a rejection', () => {
  const translated = translateProgram(`/** @param {bigint} a @param {bigint} b @returns {bigint} */
function gcd(a, b) { if (b === 0n) return a; return gcd(b, a % b); }
/** @param {bigint} n @returns {bigint} */
function collatz(n) { if (n <= 1n) return 0n; return 1n + collatz(n % 2n === 0n ? n / 2n : 3n * n + 1n); }
console.log(gcd(1071n, 462n));
console.log(collatz(27n));
`, 'JavaScript', 'Rocq');
  assert.equal(translated.diagnostic, null);
  assert.equal(translated.contract.support, 'semantic-translation');
  assert.ok(translated.semantics.encodings.some((encoding) => encoding.id === 'general-recursion'));
  assert.match(translated.code, /Fixpoint ml_fix \{A B : Type\}/u);
  assert.match(
    translated.code,
    // `a % b` throws where b is 0n, so gcd threads that abort and the output before it as a value.
    /Definition gcd \(a : Z\) \(b : Z\) \(ml_out : list string\) : ml_io1 :=\n {2}ml_fix 64 \(fun \(ml_rec : Z \* Z \* list string -> ml_io1\) \(ml_args : Z \* Z \* list string\) =>\n {4}let '\(a, b, ml_out\) := ml_args in .*\(ml_io1_abort ml_out "Division by zero"%string\) else \(ml_io1_mk ml_out \(Z\.rem a b\)\).*\(ml_rec \(b, ml_v2, ml_o1\)\).*\n {4}\(fun _ => \(ml_io1_mk \(@nil string\) 0%Z\)\) \(a, b, ml_out\)\./u,
  );
  assert.match(translated.code, /Definition collatz \(n : Z\) : Z :=\n {2}ml_fix 64 \(fun \(ml_rec : Z -> Z\) \(n : Z\) =>\n/u);
});
