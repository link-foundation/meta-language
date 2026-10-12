import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { BOOL, FLOAT, STRING, callable, map, option, record, sameType, set, tuple } from '../src/translation/types.js';

function printed(text) {
  const lines = [];
  runInNewContext(text, { console: { log: (...args) => lines.push(args.map(String).join(' ')) } });
  return lines;
}

function assertSameOutput(source) {
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  assert.deepEqual(printed(emitted), printed(source));
}

test('structural type identity keeps field names, element types and distinct absences', () => {
  const first = record([{ name: 'name', type: STRING }, { name: 'count', type: FLOAT }]);
  const reordered = record([{ name: 'count', type: FLOAT }, { name: 'name', type: STRING }]);
  assert.equal(sameType(first, reordered), true);
  assert.equal(sameType(first, record([{ name: 'name', type: STRING }])), false);
  assert.equal(sameType(option(STRING, 'null'), option(STRING, 'undefined')), false);
  assert.equal(sameType(tuple([STRING, FLOAT]), tuple([FLOAT, STRING])), false);
  assert.equal(sameType(map(STRING, FLOAT), map(FLOAT, STRING)), false);
  assert.equal(sameType(set(STRING), set(FLOAT)), false);
  assert.equal(sameType(callable([STRING], BOOL), callable([FLOAT], BOOL)), false);
});

test('nullable inference preserves null and undefined under strict equality', () => {
  const source = `function absent(flag) { if (flag) return null; return undefined; }
console.log(absent(true) === null);
console.log(absent(true) === undefined);
console.log(absent(false) === null);
console.log(absent(false) === undefined);`;
  const fn = parseJavaScript(source).items[0];
  assert.equal(fn.ret.kind, 'option');
  assert.equal(fn.ret.absence, 'both');
  assertSameOutput(source);
});

test('record spreads infer their full input shape from callers and keep later overrides', () => {
  assertSameOutput(`function entry(name, count) { return { name, count }; }
function updated(item) { return { ...item, count: 2 }; }
console.log(updated(entry('name', 1)).name);
console.log(updated(entry('name', 1)).count);`);
});

test('callbacks infer record fields and distinguish the mapped result element type', () => {
  const source = `function names(items) { return items.map(item => item.name).join(':'); }
console.log(names([{ name: 'first' }, { name: 'second' }]));`;
  const fn = parseJavaScript(source).items[0];
  assert.equal(fn.params[0].type.element.kind, 'record');
  assertSameOutput(source);
  assertSameOutput(`function doubled(items) { return items.map(item => item * 2); }
console.log(doubled([1, 2]).length);
console.log(doubled([1, 2])[1]);`);
});

test('closure parameter shadowing and captured block bindings keep their source scope', () => {
  assertSameOutput(`function scoped(n) {
  let value = n;
  { let value = n + 1; const next = value => value + 1; value = next(value); }
  return value;
}
console.log(scoped(2));`);
});

test('uninitialized bindings keep undefined when it is read before assignment', () => {
  assertSameOutput(`function before(n) {
  let value;
  const wasUndefined = value === undefined;
  value = n;
  return wasUndefined;
}
console.log(before(17));`);
});

test('declared tuples supply a distinct expected type for every literal element', () => {
  const source = `/** @param {string[]} names @param {number} count @returns {[string, number]} */
function firstWithCount(names, count) { return [names[0], count]; }`;
  const fn = parseJavaScript(source).items[0];
  assert.equal(fn.ret.kind, 'tuple');
  assert.deepEqual(fn.ret.elements, [STRING, FLOAT]);
  const checked = checkProgram(parseJavaScript(source));
  assert.equal(checked.declarations.get('firstWithCount').body.k, 'tuple');
});

test('function JSDoc accepts named arrow parameters and nested callable results', () => {
  const source = `/** @param {(x: string) => ((y: number) => boolean)} test @param {string} name @returns {boolean} */
function apply(test, name) { return test(name)(2); }`;
  const fn = parseJavaScript(source).items[0];
  assert.deepEqual(fn.params[0].type, { kind: 'function', params: [STRING], ret: { kind: 'function', params: [FLOAT], ret: BOOL } });
});
