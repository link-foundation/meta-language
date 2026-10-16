import assert from 'node:assert/strict';
import test from 'node:test';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
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

  const named = parseJavaScript("const A = { 'f'(){return 1n;} };");
  assert.deepEqual(named.items, []);
  assert.equal(named.main.effects[0].value.k, 'record');
  assert.equal(named.main.effects[0].value.entries[0].name, 'f');
  assert.equal(named.main.effects[0].value.entries[0].value.k, 'lambda');
  assert.doesNotThrow(() => checkProgram(named));
});

test('ordinary record entries retain duplicate evaluations while tagged constructors retain their own checks', () => {
  const ordinary = parseJavaScript('const A = { value: 1, value: 2 };');
  assert.deepEqual(ordinary.main.effects[0].value.entries.map(entry => [entry.name, entry.value.value]), [['value', '1'], ['value', '2']]);
  assert.doesNotThrow(() => checkProgram(ordinary));

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
