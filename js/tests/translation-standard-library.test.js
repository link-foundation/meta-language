import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { codePointUnits, indexUnits, sliceUnits, splitUnits, stringUnits, unitsString } from '../src/translation/string-runtime.js';

test('UTF-16 reference helpers match JavaScript at surrogate boundaries and unusual offsets', () => {
  const texts = ['', 'a😀b😀c', 'é漢字', '\ud800a\udfff', 'abab'];
  const positions = [-Infinity, -12, -1.9, -0, 0, 1.2, 2, 10, Infinity, NaN];
  const searches = ['', '😀', 'a', 'b', '\ud83d', '\ude00', 'unmatched'];
  for (const text of texts) {
    const units = stringUnits(text);
    assert.equal(unitsString(units), text);
    for (const start of positions) {
      assert.equal(codePointUnits(units, start), text.codePointAt(start));
      for (const end of positions) assert.equal(unitsString(sliceUnits(units, start, end)), text.slice(start, end));
      for (const search of searches) {
        assert.equal(indexUnits(units, stringUnits(search), start), text.indexOf(search, start));
        assert.equal(indexUnits(units, stringUnits(search), start, true), text.lastIndexOf(search, start));
      }
    }
    for (const separator of searches) for (const limit of [0, 1, 2, -1, 4294967296, Infinity, NaN]) {
      assert.deepEqual(splitUnits(units, stringUnits(separator), limit).map(unitsString), text.split(separator, limit));
    }
  }
});

test('callbacks, short-circuiting, nullable reads and immutable array methods preserve JavaScript behavior', () => {
  const source = `/** @param {number[]} values @returns {number[]} */
function mapped(values) { return values.filter(value => value > 0).map((value, index) => value + index); }
/** @param {number[]} values @returns {number} */
function reduced(values) { return values.reduce((sum, value) => sum + value, 0); }
/** @param {number[]} values @returns {number} */
function found(values) { return values.find(value => value > 3) ?? -1; }
/** @param {number[]} values @returns {number} */
function read(values) { return values[0] ?? 42; }
/** @param {number[]} values @returns {boolean} */
function some(values) { return values.some(value => value > 3); }
/** @param {number[]} values @returns {boolean} */
function every(values) { return values.every(value => value > 3); }
/** @param {number[]} values @returns {number[]} */
function sliced(values) { return values.slice(-2, Infinity); }
`;
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  const names = '[mapped, reduced, found, read, some, every, sliced]';
  const original = runInNewContext(source + names);
  const translated = runInNewContext(emitted + names);
  for (const values of [[], [-2, 0, 1, 4, 7]]) {
    for (let index = 0; index < original.length; index += 1) assert.equal(JSON.stringify(translated[index](values)), JSON.stringify(original[index](values)));
  }
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.equal(rust.semantics.assumptions.length, 0);
  assert.match(rust.code, /optional_at/u);
  assert.match(rust.code, /find_map/u);
  assert.match(rust.code, /\.fold\(/u);
});

test('string operations emit an operational UTF-16 Rust representation', () => {
  const source = `/** @param {string} text @returns {string} */
function sliced(text) { return text.slice(1, 2).padStart(3, '😀').replaceAll('', '$&!'); }
/** @param {string} text @returns {number} */
function point(text) { return text.codePointAt(1) ?? -1; }
/** @param {string} text @returns {string[]} */
function split(text) { return text.split('', 5); }
/** @param {string} text @returns {number} */
function search(text) { return text.indexOf('😀', 1) + text.lastIndexOf('😀'); }
`;
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  const names = '[sliced, point, split, search]';
  const original = runInNewContext(source + names);
  const translated = runInNewContext(emitted + names);
  for (const text of ['', 'a😀b😀']) for (let index = 0; index < original.length; index += 1) assert.equal(JSON.stringify(translated[index](text)), JSON.stringify(original[index](text)));
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.match(rust.code, /pub struct Text\(pub Vec<u16>\)/u);
  assert.ok(rust.semantics.encodings.some(({ id }) => id === 'utf16-strings'));
  for (const target of ['Lean', 'Rocq']) assert.match(translateProgram(source, 'JavaScript', target).diagnostic.message, /UTF-16 operational string encoding/u);
});

test('collection constructors retain insertion order, SameValueZero and nullable map values', () => {
  const source = `function distinct() { return Array.from(new Set([NaN, -0, NaN, 1])); }
function lookup() { return new Map([['a', 1], ['a', 2], ['b', 3]]).get('a') ?? 4; }
function absent() { return new Map([['a', null]]).get('a') ?? 'default'; }
function entries() { return Array.from(new Map([['a', 1], ['a', 2], ['b', 3]])); }
function mapped() { return Array.from(new Set(['a', 'b', 'a']), (value, index) => value + index); }
`;
  const original = runInNewContext(source + '[distinct, lookup, absent, entries, mapped]');
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  const translated = runInNewContext(emitted + '[distinct, lookup, absent, entries, mapped]');
  for (let index = 0; index < original.length; index += 1) assert.equal(JSON.stringify(translated[index]()), JSON.stringify(original[index]()));
  const values = translated[0]();
  assert.ok(Number.isNaN(values[0]));
  assert.equal(Object.is(values[1], -0), false);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
  const mutation = translateProgram("function changed(){ const values = new Set(['a']); values.add('b'); return values.size; }", 'JavaScript', 'Rust');
  assert.ok(mutation.diagnostic);
});
