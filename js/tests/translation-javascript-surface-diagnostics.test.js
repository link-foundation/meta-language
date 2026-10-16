import assert from 'node:assert/strict';
import test from 'node:test';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { readProjectData } from '../scripts/project-data.mjs';
import cases from './fixtures/translation-cases/frontend-javascript.mjs';

const stages = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));

test('actual constructor and closure parameter failures retain the original source and stage diagnostics', () => {
  for (const name of ['new-expression', 'arrow-token-value']) {
    const id = 'frontend-javascript-error-' + name;
    const source = cases[id][1];
    const fixture = stages.programs.find(program => program.name === 'frontend-javascript/' + id);
    assert.equal(fixture.source, source);
    assert.throws(() => parseJavaScript(source), error => {
      assert.deepEqual({ kind: error.kind, message: error.message }, fixture.parse.error);
      return true;
    });
  }
});

test('constructor argument syntax and grouped values keep their independent parsing boundaries', () => {
  assert.throws(() => parseJavaScript('function make(){return new Missing(,);}'), error => error.kind === 'syntax');
  assert.throws(() => parseJavaScript('function make(){return new Missing;}'), error =>
    error.kind === 'unsupported' && error.details.construct === 'new Missing');
  assert.doesNotThrow(() => parseJavaScript('console.log((1n));'));
  assert.doesNotThrow(() => parseJavaScript('const identity=(value)=>value;console.log(identity(2));'));
  assert.doesNotThrow(() => parseJavaScript('const A={identity(value){return value;}};console.log(A.identity(2));'));
});

test('unused documentation aliases preserve all four original malformed or scalar source fixtures', () => {
  for (const name of ['typedef-bad-key', 'typedef-missing-colon', 'typedef-not-object', 'typedef-untagged']) {
    const id = 'frontend-javascript-error-' + name;
    const source = cases[id][1];
    const fixture = stages.programs.find(program => program.name === 'frontend-javascript/' + id);
    assert.equal(fixture.source, source);
    const surface = parseJavaScript(source);
    assert.deepEqual(surface.items, []);
    assert.doesNotThrow(() => checkProgram(surface));
  }
});

test('referenced documentation aliases resolve through scalar array and tuple type readers', () => {
  const source = '/** @typedef {number} Scalar */\n'
    + '/** @typedef {Scalar[]} Values */\n'
    + '/** @typedef {[Scalar,Scalar]} Pair */\n'
    + '/** @param {Values} values @returns {Scalar} */\n'
    + 'function size(values){return values.length;}\n'
    + '/** @param {Pair} value @returns {Pair} */\n'
    + 'function identity(value){return value;}';
  const surface = parseJavaScript(source);
  assert.equal(surface.items.length, 2);
  assert.deepEqual(surface.items[0].params[0].type, { kind: 'array', element: { kind: 'float' } });
  assert.deepEqual(surface.items[0].ret, { kind: 'float' });
  const tuple = { kind: 'tuple', elements: [{ kind: 'float' }, { kind: 'float' }] };
  assert.deepEqual(surface.items[1].params[0].type, tuple);
  assert.deepEqual(surface.items[1].ret, tuple);
  assert.doesNotThrow(() => checkProgram(surface));
});

test('referenced invalid recursive and duplicate documentation aliases retain exact refusals', () => {
  const invalidPrefix = '/** @typedef {unknown} Invalid */\n';
  const invalidComment = '/** @param {Invalid} value @returns {number} */';
  const invalid = invalidPrefix + invalidComment + '\nfunction invalid(value){return 1;}';
  assert.throws(() => parseJavaScript(invalid), error => {
    assert.deepEqual({ kind: error.kind, reason: error.reason, span: error.span }, {
      kind: 'unsupported',
      reason: 'JSDoc type {unknown}: portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types',
      span: { start: invalidPrefix.length, end: invalidPrefix.length + invalidComment.length },
    });
    return true;
  });
  const recursivePrefix = '/** @typedef {Loop} Loop */\n';
  const recursiveComment = '/** @param {Loop} value @returns {Loop} */';
  const recursive = parseJavaScript(recursivePrefix + recursiveComment + '\nfunction recursive(value){return value;}');
  const recursiveRange = { start: recursivePrefix.length, end: recursivePrefix.length + recursiveComment.length };
  assert.deepEqual(recursive.items[0].params[0].type, { kind: 'named', path: ['crate', 'Loop'], span: recursiveRange });
  assert.throws(() => checkProgram(recursive), error => {
    assert.deepEqual({ kind: error.kind, reason: error.reason, span: error.span }, {
      kind: 'type', reason: 'unknown type crate.Loop', span: recursiveRange,
    });
    return true;
  });
  const duplicatePrefix = '/** @typedef {number} Value */\n';
  const duplicateComment = '/** @typedef {string} Value */';
  assert.throws(() => parseJavaScript(duplicatePrefix + duplicateComment), error => {
    assert.deepEqual({ kind: error.kind, reason: error.reason, span: error.span }, {
      kind: 'type', reason: 'duplicate @typedef Value',
      span: { start: duplicatePrefix.length, end: duplicatePrefix.length + duplicateComment.length },
    });
    return true;
  });
});
