import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { readProjectData } from '../scripts/project-data.mjs';
import cases from './fixtures/translation-cases/frontend-javascript.mjs';

const stages = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));

test('all original frontend diagnostics and the empty namespace refusal retain their exact sources and errors', () => {
  const names = [
    ...['namespace-shorthand', 'namespace-unclosed', 'arrow-token-value', 'method-call',
      'object-spread', 'object-method', 'object-without-tag', 'new-expression', 'function-expression']
      .map(name => 'frontend-javascript-error-' + name),
    'frontend-javascript-accept-namespace-empty',
  ];
  for (const name of names) {
    const source = cases[name][1];
    const fixture = stages.programs.find(program => program.name === 'frontend-javascript/' + name);
    assert.equal(fixture.source, source, name);
    assert.throws(() => parseJavaScript(source), error => {
      assert.deepEqual({ kind: error.kind, message: error.message }, fixture.parse.error, name);
      return true;
    });
  }
});

test('actual object declarations distinguish empty values identifier methods and string named methods', () => {
  const empty = parseJavaScript('const A = {};');
  assert.deepEqual(empty.items, []);
  assert.equal(empty.main.effects[0].k, 'let');
  assert.equal(empty.main.effects[0].value.k, 'record');
  assert.deepEqual(empty.main.effects[0].value.entries, []);
  assert.doesNotThrow(() => checkProgram(empty));

  const namespace = parseJavaScript('const A = { f(){return 1n;} };');
  assert.equal(namespace.items[0].k, 'module');
  assert.equal(namespace.items[0].items[0].name, 'f');
  assert.deepEqual(namespace.main.effects, []);

  const nested = parseJavaScript('const A = { B: { C: { f(){return 1n;} } } };');
  assert.equal(nested.items[0].k, 'module');
  assert.equal(nested.items[0].items[0].k, 'module');
  assert.equal(nested.items[0].items[0].items[0].items[0].name, 'f');
  const laterMethod = parseJavaScript('const A = { B: {}, f(){return 1n;} };');
  assert.equal(laterMethod.items[0].k, 'module');
  assert.equal(laterMethod.items[0].items[1].name, 'f');
  for (const modifier of ['get', 'set', 'async', 'static']) {
    assert.throws(() => parseJavaScript(`const A = { ${modifier} f(){return 1n;} };`), error => error.kind === 'unsupported'
      && error.details.construct === `${modifier} member`
      && error.reason === `${modifier} member: accessors and asynchronous methods are outside the portable core`);
  }

  const named = parseJavaScript("const A = { 'f'(){return 1n;} };");
  assert.deepEqual(named.items, []);
  assert.equal(named.main.effects[0].value.k, 'record');
  assert.equal(named.main.effects[0].value.entries[0].name, 'f');
  assert.equal(named.main.effects[0].value.entries[0].value.k, 'lambda');
  assert.doesNotThrow(() => checkProgram(named));

  const computed = parseJavaScript("const A = { ['f'](){return 1n;} };");
  assert.deepEqual(computed.items, []);
  assert.equal(computed.main.effects[0].value.entries[0].value.k, 'lambda');
  assert.doesNotThrow(() => checkProgram(computed));
});

test('ordinary record entries retain duplicate evaluations while tagged constructors retain their own checks', () => {
  const ordinary = parseJavaScript('const A = { value: 1, value: 2 };');
  assert.deepEqual(ordinary.main.effects[0].value.entries.map(entry => [entry.name, entry.value.value]), [['value', '1'], ['value', '2']]);
  assert.doesNotThrow(() => checkProgram(ordinary));

  for (const [source, kind, observation, expected] of [
    ['function read(){return 2;}const A = { value: read() };', 'name', 'console.log(A.value);', ['2']],
    ['const A = { value: (number) => number + 1 };console.log(A.value(2));', 'lambda', '', ['3']],
    ['function read(){return 2;}const A = { nested: { value: read() } };', 'record', 'console.log(A.nested.value);', ['2']],
    ['const A = { value: [1,2].slice(1), other: 3 };', 'method', 'console.log(A.value[0]);console.log(A.other);', ['2', '3']],
  ]) {
    const parsed = parseJavaScript(source);
    const record = parsed.main.effects[0].value;
    assert.equal(record.k, 'record', source);
    assert.equal(record.entries[0].value.k, kind, source);
    // The ordinary frontend represents a zero-argument named call by its
    // resolved name, rather than an app with an empty argument vector.
    if (kind === 'name') assert.deepEqual(record.entries[0].value.path, ['crate', 'read']);
    if (kind === 'record') assert.deepEqual(record.entries[0].value.entries[0].value.path, ['crate', 'read']);
    assert.doesNotThrow(() => checkProgram(parsed), source);
    const observed = source + observation;
    for (const text of [observed, emitJavaScript(checkProgram(parseJavaScript(observed))).text]) {
      const lines = [];
      runInNewContext(text, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 });
      assert.deepEqual(lines, expected, source);
    }
  }
  const once = "function read(){console.log('read');return 2;}const A = { value: read() };console.log(A.value);";
  for (const text of [once, emitJavaScript(checkProgram(parseJavaScript(once))).text]) {
    const lines = [];
    runInNewContext(text, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 });
    assert.deepEqual(lines, ['read', '2']);
  }
  assert.throws(() => parseJavaScript('const A = { value: /f\\(x\\),[{}]/ };'), error => error.kind === 'unsupported'
    && error.details.construct === 'regular expression');
  assert.throws(() => parseJavaScript('function key(){return "value";}const A = { [key()]: 1 };'), error => error.kind === 'unsupported'
    && error.details.construct === 'computed object key');

  const prefix = "/** @typedef {{ $: 'leaf', value: bigint }} Tree */\n/** @param {Tree} tree @returns {Tree} */\n";
  assert.throws(() => parseJavaScript(prefix + "function copy(tree){return { $: 'leaf', value: 1n, value: 2n, ...tree };}"), error => error.kind === 'type' && error.reason === 'duplicate field value');
  assert.throws(() => parseJavaScript(prefix + "function copy(tree){return { $: 'leaf', value: 1n, ...tree };}"), error => error.kind === 'unsupported' && error.details.construct === 'tagged object spread');
  assert.throws(() => parseJavaScript(prefix + "function copy(tree){return { $: 'leaf', value: 1n, ...missing };}"), error => error.kind === 'unsupported' && error.details.construct === 'function value missing');
});

test('object declaration order is checked before a later literal selects its namespace or value representation', () => {
  for (const value of ['{}', '{ value: 1 }', '{ f(){return 1;} }']) {
    const source = "console.log('first');const Later=" + value + ';';
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported'
      && error.details.construct === 'namespace after a top-level statement'
      && error.reason === 'namespace after a top-level statement: the statements before const Later would run before it is initialised; declare every namespace first');
  }
});
