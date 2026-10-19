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
  assert.equal(sameType(option(option(STRING, 'null'), 'undefined'), option(STRING, 'both')), true);
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

test('nullish coalescing infers optional callers and evaluates the fallback only on absence', () => {
  assertSameOutput(`function choose(value) { return value ?? fallback(); }
function fallback() { console.log('fallback'); return 'value'; }
console.log(choose(''));
console.log(choose(null));`);
  assertSameOutput(`/** @param {{name: string}|null} item @returns {string} */
function label(item) { return item?.name ?? 'default'; }
console.log(label(null));
console.log(label({ name: '' }));
console.log(label({ name: 'name' }));`);
});

test('optional JSDoc parameters preserve omitted arguments and the source default', () => {
  assertSameOutput(`/** @param {number} [count] @returns {number} */
function optional(count) { return count ?? 1; }
/** @param {number} [count=1] @returns {number} */
function defaulted(count = 1) { return count; }
console.log(optional());
console.log(optional(0));
console.log(defaulted());
console.log(defaulted(0));`);
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
  assertSameOutput(`${source}
console.log(firstWithCount(['name'], 2)[0]);
console.log(firstWithCount(['name'], 2)[1]);`);
});

test('function JSDoc accepts named arrow parameters and nested callable results', () => {
  const source = `/** @param {(x: string) => ((y: number) => boolean)} test @param {string} name @returns {boolean} */
function apply(test, name) { return test(name)(2); }`;
  const fn = parseJavaScript(source).items[0];
  assert.deepEqual(fn.params[0].type, { kind: 'function', params: [STRING], ret: { kind: 'function', params: [FLOAT], ret: BOOL } });
});

test('named function callbacks retain the same parameter constraints as closures', () => {
  assertSameOutput(`function isLong(text) { return text.length > 2; }
function selected(items) { return items.filter(isLong).join(':'); }
console.log(selected(['a', 'long']));`);
});

test('readonly collection construction keeps Map insertion order and Set deduplication', () => {
  assertSameOutput(`function keys() {
  const values = new Map([['second', 2], ['first', 1], ['second', 3]]);
  return Array.from(values.keys()).join(':');
}
function count() { const values = new Set([1, 1, 2]); return values.size; }
console.log(keys());
console.log(count());`);
});

test('nullable array inference is independent of element order and survives spreads', () => {
  assertSameOutput(`function numberFirst() { return [1, null, undefined, -0].join('|'); }
function absenceFirst() { return [null, undefined, 1, -0].join('|'); }
function spreadAbsence() { return [1, ...[null, undefined], -0].join('|'); }
function nullableBigints() { return [1n, null, 3n].join('|'); }
console.log(numberFirst());
console.log(absenceFirst());
console.log(spreadAbsence());
console.log(nullableBigints());`);
});
