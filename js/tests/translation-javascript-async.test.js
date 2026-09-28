import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { FLOAT, INT } from '../src/translation/types.js';

const signature = (items, name) => {
  const item = items.find((candidate) => candidate.k === 'fn' && candidate.name === name);
  return item && [item.params.map((param) => param.type), item.ret];
};

test('an async function whose every call is awaited is the function its body computes', () => {
  const { items, main } = parseJavaScript(`/** @param {bigint} n @returns {Promise<bigint>} */
async function square(n) { return n * n; }
async function twice(n) { return await square(await square(n)); }
const inc = async (x) => x + 1;
export async function again(x) { return inc(x); }
console.log(await twice(2n));
console.log(await again(41));
`);
  assert.deepEqual(signature(items, 'square'), [[INT], INT]);
  assert.deepEqual(signature(items, 'twice'), [[INT], INT]);
  assert.deepEqual(signature(items, 'inc'), [[FLOAT], FLOAT]);
  assert.deepEqual(signature(items, 'again'), [[FLOAT], FLOAT]);
  assert.equal(main.sequentialAsync, true);
  assert.equal(parseJavaScript('console.log(1);').main.sequentialAsync, undefined);
});

test('a Promise the program could observe is rejected where it is made', () => {
  const cases = [
    [
      'async function f() { return 1; }\nconsole.log(f());',
      'call of async function f without await: the Promise it returns is outside the portable core; await it where it is called at 45..48',
    ],
    [
      'async function f() { return 1; }\nfunction g() { return f(); }\nconsole.log(g());',
      'call of async function f without await: the Promise it returns is outside the portable core; await it where it is called at 55..58',
    ],
    [
      'function f(n) { return await n; }',
      'await outside an async function: await is only valid in async functions and at the top level of a module at 23..28',
    ],
    ['async () => 1;', 'async function: only async functions declared by name or bound to a top-level constant are portable at 0..5'],
  ];
  for (const [source, message] of cases) {
    assert.throws(() => parseJavaScript(source), (error) => error.message === message, source);
  }
});

test('a program of awaited async functions is a semantic translation in every target, which records the encoding', () => {
  const source = 'async function answer() { return 42; }\nconsole.log(await answer());\n';
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translation = translateProgram(source, 'JavaScript', target);
    assert.equal(translation.contract.support, 'semantic-translation', target);
    assert.ok(translation.semantics.encodings.some((encoding) => encoding.id === 'sequential-async'), target);
  }
});
