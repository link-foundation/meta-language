import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles, partitionPortableRustLibrary} from '../scripts/portable-source-library-style.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';

test('actual multi-line lint attributes remain attached to complete partitioned declarations', () => {
  const source = Array.from({length: 18}, (_, index) =>
    '/** @param {number} a @param {number} b @param {number} c @param {number} d @param {number} e @param {number} f @param {number} g @param {number} h @param {number} i @returns {number} */ function column' + index + '(a,b,c,d,e,f,g,h,i){return a+b+c+d+e+f+g+h+i;}').join('\n');
  const emitted = emitRust(checkProgram(parseJavaScript(source, {referenceSemantics: false})), { arrayConsumption: true });
  const styled = readPortableSourceLintStyles(emitted.definitions.join('\n\n')).source;
  const format = input => {
    const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {input, encoding: 'utf8'});
    assert.equal(result.status, 0, result.stderr);
    return result.stdout;
  };
  const formatted = format(styled);
  assert.ok(formatted.includes('#[expect(\n'), 'rustfmt expands the real checked argument attribute');
  const parts = partitionPortableRustLibrary(formatted, {directory: 'column_controls', maximumLines: 100});
  assert.ok(parts.length > 2);
  let attributes = 0;
  for (const part of parts.slice(1)) {
    format(part.source);
    attributes += (part.source.match(/#\[expect\(/gu) ?? []).length;
    assert.equal((part.source.match(/#\[expect\(/gu) ?? []).length,
      (part.source.match(/^pub fn /gmu) ?? []).length, 'each function keeps its own lint attribute');
  }
  assert.equal(attributes, 18);
});

test('actual nested primitive rows retain copies and target text through borrowed library placement', () => {
  const source = '/** @param {number[][]} rows @param {number[][]} otherRows @returns {number[][]} */ function appendRows(rows,otherRows){return rows.concat(otherRows);}'
    + '/** @param {number[][]} rows @returns {number[][]} */ function relay(rows){return appendRows(rows,[[3,4]]);}'
    + '/** @returns {string} */ function spelling(){return "rows.clone()";}'
    + '/** @returns {string} */ function digits(){return "9007199254740992";}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const raw = emitRust(checked, { arrayConsumption: true }).definitions.join('\n\n');
  const styled = readPortableSourceParameterStyles(raw, {publicNames: ['append_rows', 'relay', 'spelling']});
  assert.ok(styled.source.includes('rows: &[Vec<f64>]'));
  assert.ok(styled.source.includes('other_rows: &[Vec<f64>]'));
  assert.ok(styled.source.includes('rows.to_vec()'), 'the existing owned append operation keeps its actual row copies');
  assert.ok(styled.source.includes('other_rows.to_vec()'));
  assert.ok(styled.source.includes('String::from("rows.clone()")'), 'target-source string bytes remain unchanged');
  const reviewed = readPortableSourceLintStyles(styled.source);
  assert.ok(reviewed.source.includes('String::from("rows.clone()")'));
  assert.ok(reviewed.source.includes('String::from("9007199254740992")'));
  const parts = partitionPortableRustLibrary(reviewed.source, {directory: 'nested_row_controls'});
  for (const name of ['append_rows', 'relay', 'spelling', 'digits']) {
    assert.equal(parts.slice(1).reduce((count, part) => count + part.source.split('pub fn ' + name + '(').length - 1, 0), 1, name);
  }
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  const rows = [[1, 2]], original = structuredClone(rows);
  const result = context.relay(rows);
  assert.deepEqual(Array.from(result, row => Array.from(row)), [[1, 2], [3, 4]]);
  assert.deepEqual(rows, original);
  assert.equal(context.spelling(), 'rows.clone()');
  assert.equal(context.digits(), '9007199254740992');
});

test('actual lowered loop tuple assignments keep changing strings and nested rows owned', () => {
  const source = '/** @param {string} text @param {number} count @returns {string} */ function suffix(text,count){while(count>0){text=text+"x";count-=1;}return text;}'
    + '/** @param {number[][]} rows @param {number} count @returns {number[][]} */ function appendUntil(rows,count){while(count>0){rows=rows.concat([[count]]);count-=1;}return rows;}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const raw = emitRust(checked, { arrayConsumption: true }).definitions.join('\n\n');
  const styled = readPortableSourceParameterStyles(raw).source;
  assert.ok(styled.includes('text: &str'), 'the unchanged public input may be borrowed');
  assert.ok(styled.includes('rows: &[Vec<f64>]'));
  assert.ok(styled.includes('mut text: String'), 'the actual loop state remains owned and assignable');
  assert.ok(styled.includes('mut rows: Vec<Vec<f64>>'));
  assert.ok(/\([^()]*text[^()]*\)\s*=/u.test(styled), 'the genuine lowered text state has a tuple assignment');
  assert.ok(/\([^()]*rows[^()]*\)\s*=/u.test(styled), 'the genuine lowered row state has a tuple assignment');
  const reviewed = readPortableSourceLintStyles(styled).source;
  assert.ok(reviewed.includes('mut text: String'));
  assert.ok(reviewed.includes('mut rows: Vec<Vec<f64>>'));
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  assert.equal(context.suffix('a', 2), 'axx');
  const rows = [[1]];
  assert.deepEqual(Array.from(context.appendUntil(rows, 2), row => Array.from(row)), [[1], [2], [1]]);
  assert.deepEqual(rows, [[1]]);
});

test('whole required-column emission retains IEEE values and explicit argument state under recorded lint styles', () => {
  const source = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8')
    + '/** @param {number[]} values @param {number} a @param {number} b @param {number} c @param {number} d @param {number} e @param {number} f @param {number} g @param {number} h @returns {number} */ function select(values,a,b,c,d,e,f,g,h){return readRequiredNumberFact(values,a)+b+c+d+e+f+g+h;}'
    + '/** @param {number} value @returns {boolean} */ function interval(value){return value>=9&&value<=13;}'
    + '/** @param {number} value @returns {number} */ function rounding(value){return (value+1)-value;}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const emitted = emitRust(checked, { arrayConsumption: true });
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const loaned = readPortableSourceParameterStyles(raw).source;
  const reviewed = readPortableSourceLintStyles(loaned);
  for (const lint of ['float_cmp', 'suboptimal_flops', 'cast_precision_loss', 'cast_possible_truncation', 'cast_sign_loss']) assert.ok(reviewed.source.includes('clippy::' + lint));
  assert.ok(reviewed.source.includes('expect(clippy::too_many_arguments'));
  assert.ok(reviewed.source.includes('pub fn select('));
  assert.ok(reviewed.source.includes('9f64..=13f64'));
  assert.equal(/optional_at\([\s\S]*?\)\)\.clone\(\)/u.test(reviewed.source), false, 'fresh owned optional reads need no copied temporary');
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  assert.ok(Number.isNaN(context.select([NaN], 0, 0, 0, 0, 0, 0, 0, 0)));
  assert.ok(Object.is(context.readRequiredNumberFact([-0], 0), -0));
  assert.equal(context.rounding(1e16), 0);
  for (const [value, expected] of [[NaN, false], [-0, false], [8, false], [9, true], [13, true], [14, false]]) assert.equal(context.interval(value), expected);
  assert.throws(() => context.readRequiredNumberFact([], 0), /invalid required Number fact column position/u);
});

test('actual final array transfers preserve owned results while every required column stays borrowed', () => {
  const callable = readFileSync(new URL('../src/translation/callable-source-rules.js', import.meta.url), 'utf8');
  const actual = ['callableText', 'callableAppendName'].map(name => {
    const start = callable.indexOf('function ' + name + '(');
    assert.ok(start >= 0);
    const end = callable.indexOf('\n}', start) + 2;
    assert.ok(end > start);
    return callable.slice(callable.lastIndexOf('/**', start), end);
  }).join('\n');
  const source = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8') + actual
    + '/** @param {number[]} values @returns {number[]} */ function identity(values){return values;}'
    + '/** @param {number[]} values @returns {number[]} */ function binding(values){const inner=values;return inner;}'
    + '/** @param {number[]} values @returns {number[]} */ function consuming(values){return values;}'
    + '/** @param {number[]} values @returns {number[]} */ function forwarding(values){return consuming(values);}'
    + '/** @param {number[]} values @returns {number} */ function reading(values){return readRequiredNumberFact(values,0);}'
    + '/** @returns {string} */ function spelling(){return "values.to_vec() longer_values";}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const raw = emitRust(checked, { arrayConsumption: true }).definitions.join('\n\n');
  assert.equal(readSourceTextRustBlock(raw, 'fn', 'identity'), 'pub fn identity(values: Vec<f64>) -> Vec<f64> {\n    values\n}', 'the actual new emitter transfers its final input without a clone');
  const styled = readPortableSourceParameterStyles(raw, {ownedParameters: {consuming: ['values']}}).source;
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'identity').includes('values.to_vec()'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'binding').includes('let inner = values.to_vec();'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'forwarding').includes('crate::consuming(values.to_vec())'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'consuming').includes('values: Vec<f64>'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'callable_append_name').includes('identities.to_vec()'), 'the original append receives its actual owned left value');
  const reading = readSourceTextRustBlock(styled, 'fn', 'reading');
  assert.ok(reading.includes('values: &[f64]'));
  assert.equal(reading.includes('values.to_vec()'), false, 'actual required-column calls retain the borrowed column');
  assert.equal(reading.includes('values.clone()'), false);
  assert.ok(styled.includes('String::from("values.to_vec() longer_values")'));
  readPortableSourceLintStyles(styled);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  for (const method of ['identity', 'binding', 'forwarding']) {
    const values = [2, 3];
    assert.deepEqual(Array.from(context[method](values)), [2, 3]);
    assert.deepEqual(values, [2, 3]);
  }
  assert.deepEqual(Array.from(context.callableAppendName(['left', 'right'], [0], 1)), [0, 1]);
  assert.deepEqual(Array.from(context.callableAppendName(['left', 'right'], [0], 0)), [0]);
  assert.equal(context.reading([7]), 7);
});

test('actual UTF16 text columns borrow vectors while preserving isolated surrogate values', () => {
  const source = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8')
    + '/** @param {string[]} values @param {number} index @returns {string} */ function selected(values,index){return readRequiredTextFact(values,index);}'
    + '/** @returns {string} */ function isolated(){return "\\ud800";}';
  assert.throws(() => parseJavaScript(source, {referenceSemantics: false}), /unicode escape \\ud800 is not a scalar value/u, 'the original literal input retains the existing lexer refusal');
  const valid = source.replace('"\\ud800"', '"😀".slice(0,1)');
  const checked = checkProgram(parseJavaScript(valid, {referenceSemantics: false}));
  const raw = emitRust(checked, { arrayConsumption: true }).definitions.join('\n\n').replaceAll('crate::ml_string::Text', 'self::ml_string::Text');
  assert.ok(raw.includes('Vec<self::ml_string::Text>'), 'the actual UTF16 emitter supplies its Text leaf');
  const styled = readPortableSourceParameterStyles(raw).source;
  for (const name of ['read_required_text_fact', 'selected']) {
    const block = readSourceTextRustBlock(styled, 'fn', name);
    assert.ok(block.includes('values: &[self::ml_string::Text]'));
    assert.equal(block.includes('values.clone()'), false);
    assert.equal(block.includes('values.to_vec()'), false, 'a selected text element never copies the complete fact column');
  }
  readPortableSourceLintStyles(styled);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  const values = ['\ud800', '𝄞', 'plain'];
  for (const [index, value] of values.entries()) assert.equal(context.selected(values, index), value);
  assert.equal(context.isolated().charCodeAt(0), 0xd83d);
  assert.deepEqual(values, ['\ud800', '𝄞', 'plain']);
});
