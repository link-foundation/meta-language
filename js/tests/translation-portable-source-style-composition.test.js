import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles} from '../scripts/portable-source-library-style.mjs';
import {readPortableOwnedSourceStyles} from '../scripts/portable-owned-source-style.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';

test('actual explicit format operands retain ownership proofs before generated capture formatting', () => {
  const source = '/** @param {string} value @returns {string} */ function identity(value){return value;}\n'
    + '/** @param {string} value @param {boolean} flag @returns {string} */ function decorated(value,flag){const local=identity(value);const label="prefix:"+local;return flag?local:label;}\n'
    + '/** @param {number} count @returns {string} */ function repeated(count){let cases="";while(count>0){cases=cases+"x";count-=1;}return cases;}\n'
    + '/** @param {string} first @param {string} second @param {number} count @returns {string} */ function selected(first,second,count){const firstAbsence=identity(first);const secondAbsence=identity(second);let remaining=count;while(remaining>0){if(remaining===2)return "blocked";remaining-=1;}if(firstAbsence==="")return secondAbsence===""?"both":secondAbsence;return firstAbsence;}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const emitted = emitRust(checked, {arrayConsumption: true}).definitions.join('\n\n');
  const loaned = readPortableSourceParameterStyles(emitted).source;
  const before = readSourceTextRustBlock(loaned, 'fn', 'decorated');
  assert.match(before, /format!\(\s*"\{\}\{\}"/u, 'actual Rust emission supplies separate format argument expressions');
  assert.ok(before.includes('local.clone()'), 'the original final owned branch contains a copy');
  const selectedBefore = readSourceTextRustBlock(loaned, 'fn', 'selected');
  assert.match(selectedBefore, /match\s+ml_s[0-9]+(?:_[0-9]+)?\.clone\(\)/u, 'actual early-return loop emission has a cloned enum match temporary');
  assert.ok(selectedBefore.includes('first_absence.clone()'));
  assert.ok(selectedBefore.includes('second_absence.clone()'));
  const reviewed = readPortableSourceLintStyles(loaned);
  const decorated = readSourceTextRustBlock(reviewed.source, 'fn', 'decorated');
  assert.equal(decorated.includes('local.clone()'), false, 'ownership sees the explicit earlier read before its final branch');
  assert.ok(decorated.includes('{local}'), 'subsequent formatting may use the proven earlier local as an implicit capture');
  assert.equal(reviewed.source.includes('cases.clone()'), false, 'the actual changing string loop transfers its final owned state');
  const selected = readSourceTextRustBlock(reviewed.source, 'fn', 'selected');
  assert.equal(/match\s+ml_s[0-9]+(?:_[0-9]+)?\.clone\(\)/u.test(selected), false, 'single-use enum temporaries expose their actual owned payloads');
  assert.equal(selected.includes('first_absence.clone()'), false);
  assert.equal(selected.includes('second_absence.clone()'), false);
  const styles = new DecoratorSet([{id: 'portable-composition-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: reviewed.actions}]);
  assert.equal(styles.decorate('emitter', {format: 'Rust', source: loaned}).source, reviewed.source);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  for (const [value, flag, expected] of [['kept', true, 'kept'], ['kept', false, 'prefix:kept'], ['', false, 'prefix:']]) {
    assert.equal(context.decorated(value, flag), expected);
  }
  assert.equal(context.repeated(0), '');
  assert.equal(context.repeated(4), 'xxxx');
  for (const [first, second, count, expected] of [['', '', 0, 'both'], ['', 'undefined', 1, 'undefined'], ['null', 'undefined', 0, 'null'], ['null', 'undefined', 2, 'blocked']]) {
    assert.equal(context.selected(first, second, count), expected);
  }
});

test('pre-existing implicit format captures continue to refuse owned copy elimination', () => {
  const source = 'pub fn captured_format(value: String) -> String { let copied = value.clone(); let display = format!("{value}"); self::observe(display); copied }';
  assert.deepEqual(readPortableOwnedSourceStyles(source), {source, actions: []}, 'an opaque captured read cannot prove the earlier original value dead');
  assert.ok(readPortableSourceLintStyles(source).source.includes('value.clone()'), 'full style composition keeps the same captured original alive');
});
