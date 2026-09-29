import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const SUM = `function sum(n) {
  let total = 0n;
  for (let i = 1n; i <= n; i++) {
    total += i;
  }
  return total;
}
console.log(sum(10n));
`;

test('a loop is a generated function of the variables it uses', () => {
  const { items } = parseJavaScript(SUM);
  const loop = items.find((item) => item.name === 'ml_sum_loop1');
  assert.equal(loop.k, 'fn');
  assert.equal(loop.generated, true);
  assert.deepEqual(loop.params.map((param) => param.name), ['n', 'total', 'i']);
  // The one variable the loop assigns and the statements after it read is its result: no result type is needed.
  assert.equal(items.some((item) => item.k === 'data'), false);
});

test('a loop that returns, or leaves live variables it assigns, returns a generated data type', () => {
  const { items } = parseJavaScript(`function divisor(n) {
  for (let d = 2n; d * d <= n; d++) {
    if (n % d === 0n) return d;
  }
  return n;
}
function gcd(a, b) {
  while (b !== 0n) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}
console.log(divisor(91n) + gcd(1071n, 462n));
`);
  const result = items.find((item) => item.name === 'ml_divisor_loop1_result');
  assert.deepEqual(result.ctors.map((ctor) => ctor.name), ['ml_divisor_loop1_done', 'ml_divisor_loop1_return']);
  // gcd's loop assigns a and b, but only a is read after it.
  assert.equal(items.some((item) => item.name === 'ml_gcd_loop2_result'), false);
});

test('mutable bindings JavaScript could not run as translated are rejected', () => {
  const cases = [
    ['function f(n) { var x = n; return x; }', 'var declaration: var bindings are hoisted to the function and shared by its blocks; use let or const at 16..19'],
    ['function f(n) { const x = n; x = 1n; return x; }', 'assignment of constant x: assigning a const binding throws a TypeError; declare it with let at 29..33'],
    ['function f(n) { let x; x = n; return x; }', 'let x without a value: an uninitialised let holds undefined, which is not a portable value; give it an initial value at 16..20'],
    ['function f(n) { let [a] = n; return a; }', 'let destructuring: declare each binding with its own let at 16..20'],
    ['function f(n) { x = 1n; let x = n; return x; }', 'assignment of x before its declaration: the binding is in its temporal dead zone, where assigning it throws a ReferenceError at 16..20'],
    ['function f(n) { break; }', 'break outside a loop or switch at 16..21'],
    ['function f(n) { outer: while (true) { break outer; } return n; }', 'label outer: labels are outside the portable core; a break or continue applies to the innermost loop at 16..21'],
    ['function f(n) { for (const x of n) {} return n; }', 'for…of loop: iteration over arrays, strings and objects is outside the portable core; count with for (let i = …; …; …) at 16..21'],
    ['function f(n) { let ml_x = n; return ml_x; }', "reserved identifier: ml_x uses the translator's reserved ml_ prefix at 20..24"],
    ['function f(n) { while (n > 0n) { if (n === 3n) return n; n--; } }', 'the function returns a bigint on one path and finishes without a return value, returning undefined, on another; return a value on every path at 54..55'],
    ['function f(n) { while (true) { return n; } return 0n; }', 'unreachable statement: statements after return, throw, break, continue or a complete if are never executed at 43..54'],
    ['function f(n) { n <<= 1n; return n; }', '<<= assignment: the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||= at 16..22'],
    ['function f(n) { for (let i = 0n; i < n; i++) { g = i; } return n; }', 'assignment of g: only local variables declared with let, and parameters, are assignable at 47..51'],
  ];
  for (const [source, message] of cases) {
    assert.throws(() => parseJavaScript(source), (error) => error.message === message, source);
  }
});

test('a program of loops is a semantic translation in every target, and Rust and JavaScript run lifted loops as loops', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    assert.equal(translateProgram(SUM, 'JavaScript', target).contract.support, 'semantic-translation', target);
  }
  assert.match(translateProgram(SUM, 'JavaScript', 'Rust').code, /pub fn ml_sum_loop1\(mut n: [^)]*\) -> crate::ml::Big \{\n {4}loop \{/u);
  // A hundred thousand iterations would overflow the stack as a hundred thousand calls.
  const long = SUM.replace('sum(10n)', 'sum(100000n)');
  const { text } = emitJavaScript(checkProgram(parseJavaScript(long)));
  assert.match(text, /for \(;;\) \{/u);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', text], { encoding: 'utf8' }), '5000050000n\n');
});
