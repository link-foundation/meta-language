import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { FLOAT, INT, NAT, STRING } from '../src/translation/types.js';

const signature = (items, name) => {
  const item = items.find((candidate) => candidate.k === 'fn' && candidate.name === name);
  return item && [item.params.map((param) => param.type), item.ret];
};

test('arrow functions and function expressions bound to constants are functions', () => {
  const { items } = parseJavaScript(`const inc = x => x + 1;
/** @param {number} a @param {number} b @returns {number} */
const avg = (a, b) => (a + b) / 2;
const fact = function (n) { if (n < 0n) throw new RangeError('negative'); return n === 0n ? 1n : n * fact(n - 1n); };
const pow = function pow(b, n) { return n === 0n ? 1n : b * pow(b, n - 1n); };
export const greet = (name) => { return 'hello ' + name; };
console.log(inc(avg(1, 2)));
console.log(pow(2n, fact(3n)));
console.log(greet('world'));
`);
  assert.deepEqual(signature(items, 'inc'), [[FLOAT], FLOAT]);
  assert.deepEqual(signature(items, 'avg'), [[FLOAT, FLOAT], FLOAT]);
  assert.deepEqual(signature(items, 'fact'), [[NAT], INT]);
  assert.deepEqual(signature(items, 'pow'), [[INT, INT], INT]);
  assert.deepEqual(signature(items, 'greet'), [[STRING], STRING]);
});

test('const functions JavaScript could not call as translated are rejected', () => {
  const cases = [
    [
      'console.log(1);\nconst f = x => x;',
      'function after a top-level statement: the statements before const f could call it before it is initialised; declare every function first at 16..22',
    ],
    ['const f = function g(x) { return x; };', 'function expression g: its own name is visible only inside it; call it f at 19..20'],
    ['const f = function* () {};', 'generator function: generators are outside the portable core at 0..18'],
    ['const f = (...xs) => xs;', 'rest parameter: functions take a fixed number of arguments at 11..14'],
  ];
  for (const [source, message] of cases) {
    assert.throws(() => parseJavaScript(source), (error) => error.message === message, source);
  }
});

test('a program of arrow functions is a semantic translation in every target', () => {
  const source = 'const inc = x => x + 1; console.log(inc(41));';
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    assert.equal(translateProgram(source, 'JavaScript', target).contract.support, 'semantic-translation', target);
  }
});
