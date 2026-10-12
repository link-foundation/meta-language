import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';

import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import frontendCases from './fixtures/translation-cases/frontend-javascript.mjs';

function observe(source, inspect) {
  const lines = [];
  const value = runInNewContext(`${source}\n${inspect}`, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return { value: JSON.parse(JSON.stringify(value)), lines };
}

function preserves(source, inspect, expected) {
  const original = observe(source, inspect);
  assert.deepEqual(original, expected);
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  assert.deepEqual(observe(emitted, inspect), original);
}

const ownProperty = `(() => {
  const item = make(7);
  return [Object.hasOwn(item, '__proto__'), item.__proto__, Object.getPrototypeOf(item) === Object.prototype,
    Object.getOwnPropertyDescriptor(item, '__proto__')];
})()`;
const descriptor = { value: 7, writable: true, enumerable: true, configurable: true };
const primitiveFixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/translation-primitive-values.json', import.meta.url), 'utf8'));

test('prototype-definition object entries are refused without treating them as data properties', () => {
  for (const [literal, nullPrototype, inherited] of [
    ['{__proto__: null}', true, false],
    ['{__proto__: 7}', false, false],
    ['{"__proto__": {inherited: 1}}', false, true],
    ['{"__pr\\u006fto__": null}', true, false],
  ]) {
    const source = `function make() { return ${literal}; }`;
    assert.deepEqual(observe(source, "(() => { const item = make(); return [Object.hasOwn(item, '__proto__'), Object.getPrototypeOf(item) === null, item.inherited === 1]; })()").value,
      [false, nullPrototype, inherited], literal);
    const parsed = parseJavaScript(source);
    const definition = parsed.items.find(item => item.name === 'make').body.prototypeDefinition;
    assert.ok(definition.value);
    assert.equal(definition.index, 0);
    assert.throws(() => checkProgram(parsed), error => error.kind === 'unsupported'
      && error.details.construct === 'object prototype definition' && /prototype-definition syntax/u.test(error.message)
      && source.slice(error.span.start, error.span.end).includes('__pr'), literal);
  }
  assert.throws(() => parseJavaScript('const Namespace = {__proto__: {read() { return 1; }}, own() { return 2; }};'),
    error => error.kind === 'unsupported' && error.details.construct === 'object prototype definition');
});

test('computed and shorthand __proto__ keys preserve own data properties and descriptors', () => {
  for (const source of [
    'function make(__proto__) { return {__proto__}; }',
    'function make(value) { return {["__proto__"]: value}; }',
    'function make(value) { return {["__pr\\u006fto__"]: value}; }',
    'function make(__proto__) { const base = {__proto__}; return {...base}; }',
  ]) preserves(source, ownProperty, { value: [true, 7, true, descriptor], lines: [] });
});

test('a __proto__ method is an own callable data property', () => {
  preserves('function make() { return {__proto__() { return 7; }}; }',
    "(() => { const item = make(); return [Object.hasOwn(item, '__proto__'), item.__proto__(), Object.getPrototypeOf(item) === Object.prototype]; })()",
    { value: [true, 7, true], lines: [] });
});

test('tagged constructor shorthand and computed __proto__ fields remain own properties', () => {
  const declaration = "/** @typedef {{ $: 'one', __proto__: number } | { $: 'two' }} Item */\n/** @param {number} __proto__ @returns {Item} */\n";
  for (const fields of ['__proto__', '["__proto__"]: __proto__']) {
    preserves(`${declaration}function make(__proto__) { return {$: 'one', ${fields}}; }`, ownProperty,
      { value: [true, 7, true, descriptor], lines: [] });
  }
});

test('shared native source fixtures preserve tagged own properties and explicit freezing', () => {
  for (const { source, frozen } of primitiveFixture.objectDataSources) {
    preserves(source, ownProperty, { value: [true, 7, true, { ...descriptor, writable: !frozen, configurable: !frozen }], lines: [] });
  }
});

test('an explicitly frozen tagged object preserves its source property descriptors', () => {
  const source = "/** @typedef {{ $: 'one', __proto__: number } | { $: 'two' }} Item */\n/** @param {number} value @returns {Item} */\nfunction make(value) { return Object.freeze({$: 'one', ['__proto__']: value}); }";
  assert.equal(parseJavaScript(source).items.find(item => item.name === 'make').body.frozen, true);
  assert.equal(checkProgram(parseJavaScript(source)).declarations.get('make').body.frozen, true);
  preserves(source, ownProperty, { value: [true, 7, true, { ...descriptor, writable: false, configurable: false }], lines: [] });
});

test('overwriting computed __proto__ fields retains each value evaluation', () => {
  preserves("function value(n) { console.log(n); return n; }\nfunction make() { return {['__proto__']: value(1), ['__proto__']: value(2)}; }",
    "(() => { const item = make(); return [Object.hasOwn(item, '__proto__'), item.__proto__]; })()",
    { value: [true, 2], lines: ['1', '2'] });
});

test('nonliteral computed names remain a precise source admission gap', () => {
  assert.throws(() => parseJavaScript("function make(key) { return {[key]: 7}; }"),
    error => error.kind === 'unsupported' && error.details.construct === 'computed object key');
});


test('the original accepted constructor fixture retains its prototype source expression', () => {
  const source = frontendCases['frontend-javascript-accept-ctor-proto-field'][1];
  const body = parseJavaScript(source).items.find(item => item.name === 'f').body;
  assert.equal(body.k, 'ctorObject');
  assert.equal(body.tag, 'leaf');
  assert.deepEqual(Object.keys(body.fields), []);
  assert.equal(body.prototypeDefinition.index, 1);
  assert.equal(body.prototypeDefinition.value.k, 'ctorObject');
  assert.equal(body.prototypeDefinition.value.tag, 'leaf');
  assert.equal(source.slice(body.prototypeDefinition.span.start, body.prototypeDefinition.span.end), '__proto__');
  assert.deepEqual(observe(source, "(() => { const value = f(0n); return [Object.hasOwn(value, '__proto__'), Object.getPrototypeOf(value).$]; })()").value, [false, 'leaf']);
  assert.throws(() => checkProgram(parseJavaScript(source)), error => error.kind === 'unsupported' && error.details.construct === 'object prototype definition');
});

test('unrepresented Object.freeze operands retain their behavior by refusing translation', async () => {
  const { selfTranslate } = await import('../src/self-translation.js');
  for (const source of primitiveFixture.freezeSources) {
    assert.equal(observe(source, 'Object.isFrozen(make())').value, true, source);
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported' && error.details.construct === 'Object.freeze operand', source);
    const carried = selfTranslate(source, 'JavaScript', 'Rust');
    assert.equal(carried.items.find(item => source.slice(item.start, item.end).includes('Object.freeze')).status, 'carried', source);
    assert.equal(selfTranslate(carried.code, 'Rust', 'JavaScript').code, source);
  }
  for (const source of [
    'const VALUES = Object.freeze([value()]); function value() { return 7; }',
    'const VALUES = Object.freeze([1]); function make() { return Object.freeze([2]); }',
    'const Namespace = Object.freeze({read() { return 1; }});',
  ]) assert.throws(() => parseJavaScript(source, { readonlyModuleConstants: true }),
    error => error.kind === 'unsupported' && error.details.construct === 'Object.freeze operand', source);
});
