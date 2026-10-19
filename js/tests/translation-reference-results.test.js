import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const fixtures = [
  ['binding', '/** @param {number[]} values @param {boolean} selected @returns {number[]} */ function choose(values,selected){const alias=values;if(selected)return alias;return [];}', true],
  ['conditional', '/** @param {number[]} values @param {boolean} selected @returns {number[]} */ function choose(values,selected){return selected?values:[];}', true],
  ['switch', '/** @param {number[]} values @param {number} selected @returns {number[]} */ function choose(values,selected){switch(selected){case 1:return values;default:return [];}}', 1],
  ['lifted loop', '/** @param {number[]} values @returns {number[]} */ function choose(values){for(let index=0;index<3;index+=1){if(index===1)return values;}return [];}', true],
];

test('declared array reference results retain alias identity through control flow and lifted loops', () => {
  for (const [name, source, selected] of fixtures) {
    const original = {};
    const replay = {};
    runInNewContext(source, original);
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics: true }));
    runInNewContext(emitJavaScript(checked).text, replay);
    assert.equal(checked.items[0].ret.reference, true, name);
    for (const implementation of [original, replay]) {
      const values = [1, 2];
      const alias = implementation.choose(values, selected);
      assert.equal(alias, values, name);
      alias.push(3);
      assert.deepEqual(values, [1, 2, 3], `${name}: returned alias mutates the input`);
      values[0] = 9;
      assert.deepEqual(alias, [9, 2, 3], `${name}: caller mutation remains visible through the result`);
    }
  }
});

test('fresh conditional arrays retain a separate identity and nested arrays retain child aliases', () => {
  const source = '/** @param {number[]} values @param {boolean} selected @returns {number[]} */ function choose(values,selected){return selected?values:[];}'
    + '/** @param {number[]} values @returns {number[][]} */ function nested(values){return [values];}';
  const replay = {};
  runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source, { referenceSemantics: true }))).text, replay);
  const values = [1, 2];
  const first = replay.choose(values, false);
  const second = replay.choose(values, false);
  assert.notEqual(first, values);
  assert.notEqual(first, second);
  first.push(7);
  assert.deepEqual(Array.from(second), []);
  assert.deepEqual(values, [1, 2]);
  const outer = replay.nested(values);
  assert.equal(outer[0], values);
  outer[0].push(3);
  assert.deepEqual(values, [1, 2, 3]);
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: true }));
  assert.equal(checked.items[1].ret.reference, true);
  assert.equal(checked.items[1].ret.element.reference, true);
});

test('collector diagnostics reach unsupported mutation without discarding annotated array references', () => {
  for (const returns of ['', ' @returns {number[]}']) {
    const source = `/** @param {number[]} units${returns} */ export function scan(units){const result=[];for(let i=0;i<units.length;i+=1){result.push(units[i]??-1);}return result;}`;
    const surface = parseJavaScript(source, { referenceSemantics: true });
    assert.equal(surface.items[0].params[0].type.reference, true);
    assert.equal(surface.items[0].ret.reference, true);
    assert.throws(() => checkProgram(surface), /\.push\(\).*has no checked operation/u);
  }
});

test('scalar and immutable value-array result contracts remain distinct', () => {
  const source = '/** @param {number[]} values @param {boolean} selected @returns {number[]} */ function choose(values,selected){return selected?values:[];}'
    + '/** @param {boolean} selected @returns {number} */ function scalar(selected){return selected?1:2;}';
  for (const referenceSemantics of [false, true]) {
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics }));
    assert.equal(Boolean(checked.items[0].ret.reference), referenceSemantics);
    assert.equal(checked.items[1].ret.kind, 'float');
    const replay = {};
    runInNewContext(emitJavaScript(checked).text, replay);
    assert.equal(replay.scalar(true), 1);
    assert.equal(replay.scalar(false), 2);
  }
});
