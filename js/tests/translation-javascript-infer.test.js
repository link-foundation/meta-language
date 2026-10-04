import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { BOOL, FLOAT, INT, NAT, STRING } from '../src/translation/types.js';

const TREE = "/**\n * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree\n */\n";

function signature(items, name) {
  for (const item of items) {
    if (item.k === 'fn' && item.name === name) return [item.params.map((param) => param.type), item.ret];
    if (item.k === 'module') {
      const found = signature(item.items, name);
      if (found) return found;
    }
  }
  return null;
}

const withoutSpan = (type) => (type.kind === 'named' ? { kind: 'named', path: type.path } : type);

test('types JSDoc leaves out are inferred from the uses', () => {
  const { items } = parseJavaScript(`${TREE}function answer() { return 42; }
function fact(n) { if (n < 0n) throw new RangeError('negative'); return n === 0n ? 1n : n * fact(n - 1n); }
function label(n) { return n + '!'; }
function pick(flag, a, b) { return flag ? a : b; }
function sum(t) { switch (t.$) { case 'leaf': return 0n; case 'node': return sum(t.left) + t.value + sum(t.right); } }
const M = { sq(x) { return x * x; }, two(x) { return M.sq(x) + M.sq(x); } };
console.log(answer());
console.log(label(fact(3n)));
console.log(pick(true, 'a', 'b'));
console.log(M.two(1.5));
`);
  assert.deepEqual(signature(items, 'answer'), [[], FLOAT]);
  assert.deepEqual(signature(items, 'fact'), [[NAT], INT]);
  assert.deepEqual(signature(items, 'label'), [[INT], STRING]);
  assert.deepEqual(signature(items, 'pick'), [[BOOL, STRING, STRING], STRING]);
  const [params, ret] = signature(items, 'sum');
  assert.deepEqual([params.map(withoutSpan), ret], [[{ kind: 'named', path: ['crate', 'Tree'] }], INT]);
  assert.deepEqual(signature(items, 'sq'), [[FLOAT], FLOAT]);
  assert.deepEqual(signature(items, 'two'), [[FLOAT], FLOAT]);
});

test('uses that need different types are type errors', () => {
  const cases = [
    [
      "function id(x) { return x; }\nconsole.log(id(1));\nconsole.log(id('a'));",
      'one value is used as a string and as a number; declare the types of the function with JSDoc at 64..67',
    ],
    ['function f(x) { return x + 1n; }\nconsole.log(f(1));', 'a BigInt is used as a number; JavaScript does not mix BigInt with other types at 23..29'],
    ['function f(x) { return !x; }\nconsole.log(f(1));', 'one value is used as a number and as a boolean; declare the types of the function with JSDoc at 43..44'],
  ];
  for (const [source, message] of cases) {
    assert.throws(() => parseJavaScript(source), (error) => error.kind === 'type' && error.message === message, source);
  }
});

test('an unannotated program is a semantic translation in every target', () => {
  const source = 'function answer() { return 42; } console.log(answer());';
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    assert.equal(translateProgram(source, 'JavaScript', target).contract.support, 'semantic-translation', target);
  }
});
