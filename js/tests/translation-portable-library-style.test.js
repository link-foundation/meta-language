import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {parseJavaScript} from '../src/translation/javascript.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles, partitionPortableRustLibrary} from '../scripts/portable-source-library-style.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {DecoratorSet} from '../src/decorators.js';

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

test('actual scalar UTF16 text loans preserve owned results, comparisons and borrowed calls', () => {
  const source = '/** @param {string} text @returns {string} */ function retained(text){return text;}'
    + '/** @param {string} text @returns {string} */ function selected(text){return retained(text);}'
    + '/** @param {string} left @param {string} right @returns {boolean} */ function compared(left,right){return left===right;}'
    + '/** @param {string} text @returns {string} */ function leading(text){return text.slice(0,1);}'
    + '/** @param {string} text @returns {string} */ function spelling(text){return "text.clone() String::from(\\\"\\\")";}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const raw = emitRust(checked, {arrayConsumption: true}).definitions.join('\n\n').replaceAll('crate::ml_string::Text', 'self::ml_string::Text');
  const loaned = readPortableSourceParameterStyles(raw).source;
  assert.ok(loaned.includes('text: &self::ml_string::Text'));
  assert.ok(readSourceTextRustBlock(loaned, 'fn', 'retained').includes('text.clone()'), 'an owned result still copies the borrowed Text value');
  const selected = readSourceTextRustBlock(loaned, 'fn', 'selected');
  assert.ok(selected.includes('crate::retained(text)'));
  assert.equal(selected.includes('text.clone()'), false, 'a borrowed callee does not receive an owned copy');
  const compared = readSourceTextRustBlock(loaned, 'fn', 'compared');
  assert.ok(compared.includes('(*left)'));
  assert.ok(compared.includes('(*right)'));
  assert.equal(loaned.includes('text.to_vec()'), false, 'scalar Text never uses the vector copy operation');
  const format = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {input: loaned, encoding: 'utf8'});
  assert.equal(format.status, 0, format.stderr);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  assert.equal(context.selected('\ud800'), '\ud800');
  assert.equal(context.leading('😀'), '\ud83d');
  assert.equal(context.compared('\ud800', '\ud800'), true);
  assert.equal(context.compared('\ud800', '\ufffd'), false);
  assert.equal(context.spelling('ignored'), 'text.clone() String::from("")');
  const borrowed = readPortableSourceParameterStyles('pub fn prefixed(text: self::ml_string::Text, prefix: self::ml_string::Text) -> bool { text.starts_with(&(prefix)) }\n'
    + 'pub fn sliced(text: self::ml_string::Text) -> bool { text.starts_with(&(text.slice(0f64, 1f64))) }').source;
  assert.ok(borrowed.includes('text.starts_with((prefix))'));
  assert.ok(borrowed.includes('text.starts_with(&(text.slice(0f64, 1f64)))'), 'a compound owned result retains its actual loan');
  const called = readPortableSourceParameterStyles('pub fn consume(text: self::ml_string::Text) -> bool { text.length() > 0f64 }\n'
    + 'pub fn forwarding(text: self::ml_string::Text) -> bool { self::consume((text)) == true }',
  {ownedParameters: {consume: ['text']}}).source;
  assert.ok(called.includes('self::consume((text.clone())) == true'), 'call parentheses cannot turn an owned argument into a moved dereference');
  assert.equal(called.includes('(*text)'), false);
});

test('complete sibling placement imports only actual sibling declarations', () => {
  const source = 'pub const fn first() -> bool {\n    true\n}\n'
    + 'pub const fn second() -> bool {\n    self::first()\n}\n'
    + 'pub fn third() -> bool {\n    let first = "second first";\n    // self::first() is a comment\n    first.is_empty()\n}\n';
  const parts = partitionPortableRustLibrary(source, {directory: 'explicit_siblings', maximumLines: 6});
  assert.equal(parts.length, 4);
  assert.ok(parts[2].source.includes('use super::first;'));
  assert.equal(parts[1].source.includes('use super::'), false);
  assert.equal(parts[3].source.includes('use super::'), false, 'local bindings, strings and comments do not import sibling items');
  for (const part of parts) assert.equal(part.source.includes('use super::*;'), false);
  assert.equal(parts[2].source.match(/self::first\(\)/gu)?.length, 1, 'the emitted reference remains unchanged');
  const numbered = [10, 8, 2].map(value => `pub fn continuation${value}() -> bool {\n    true\n}\n`).join('')
    + 'pub fn selected() -> bool {\n    self::continuation10() && self::continuation8() && self::continuation2()\n}\n';
  const selected = partitionPortableRustLibrary(numbered, {directory: 'numbered_siblings', maximumLines: 6}).at(-1).source;
  const imports = selected.slice(0, selected.indexOf('\n\n') + 1);
  assert.deepEqual(imports.trim().split('\n'), ['use super::continuation2;', 'use super::continuation8;', 'use super::continuation10;']);
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {input: selected, encoding: 'utf8'});
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, selected, 'new sibling imports already follow the active 2024 formatter');
});

test('complete mechanical library expressions preserve opaque macros and lazy refusal paths', () => {
  const source = 'pub fn refuse() -> f64 { panic!("missing") }\n'
    + 'pub fn select(value: Option<f64>, invalid: bool) -> f64 {\n'
    + '    if invalid { panic!("{}", "invalid") } else { value.unwrap_or_else(|| self::refuse()) }\n}\n'
    + 'pub fn identity(value: Option<bool>) -> bool { match value { Some(present) => present, None => false } }\n'
    + 'pub fn empty() -> String { String::from("") }\n'
    + 'pub fn formatted() -> String { format!("{}{}", String::from("prefix"), format!("{}", String::from("suffix"))) }\n'
    + 'pub fn opaque() -> String {\n'
    + '    // String::from("") || self::refuse()\n'
    + '    demo! { String::from("") }\n'
    + '    String::from("String::from(\\\"\\\") || self::refuse()")\n}\n';
  const styled = readPortableSourceLintStyles(source).source;
  const selected = readSourceTextRustBlock(styled, 'fn', 'select');
  assert.ok(selected.includes('assert!(!(invalid), "{}", "invalid")'));
  assert.ok(selected.includes('.unwrap_or_else(self::refuse)'));
  assert.equal(selected.includes('else {'), false);
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'identity').includes('.unwrap_or_default()'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'empty').includes('String::new()'));
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'formatted').includes('String::from("prefixsuffix")'));
  const opaque = readSourceTextRustBlock(styled, 'fn', 'opaque');
  assert.ok(opaque.includes('demo! { String::from("") }'));
  assert.ok(opaque.includes('// String::from("") || self::refuse()'));
  assert.ok(opaque.includes('String::from("String::from(\\\"\\\") || self::refuse()")'));
  const unsupported = 'pub fn binding(value: Option<f64>) -> f64 { if let None = value { panic!("missing") } else { 1.0 } }\n'
    + 'pub fn attributed() -> bool { #[example(if true { panic!("x") } else { String::from("") })] true }\n'
    + 'pub fn macro_body() -> bool { demo! { if true { panic!("x") } else { String::from("") } } true }';
  const guarded = readPortableSourceLintStyles(unsupported).source;
  assert.ok(guarded.includes('if let None = value'));
  assert.ok(guarded.includes('#[example(if true { panic!("x") } else { String::from("") })]'));
  assert.ok(guarded.includes('demo! { if true { panic!("x") } else { String::from("") } }'));
});

test('known String emptiness and numeric Option copies retain unknown ownership and comments', () => {
  const source = 'pub mod ml_math { pub fn is_integer(value: f64) -> bool { value.trunc() == value } }\n'
    + 'pub fn text() -> String { String::from("value") }\n'
    + 'pub fn empty(value: &str) -> bool { let found = self::text(); (value == "") || (found != "") }\n'
    + 'pub fn bound() -> String { { let text = String::from("kept"); text } }\n'
    + 'pub fn numeric(values: &[f64]) -> bool { let unit = self::ml_array::optional_at(values, Some(0)); '
    + '(match unit.clone() { Some(value) => self::ml_math::is_integer(value), None => false }) '
    + '&& ((match unit.clone() { Some(value) => value, None => f64::NAN }) > 0f64) }\n'
    + 'pub fn owned(values: &[String]) -> (Option<String>, Option<String>) { let unit = self::ml_array::optional_at(values, Some(0)); (unit.clone(), unit.clone()) }\n'
    + 'pub fn unknown(value: Unknown) -> bool { value == "" }\n'
    + 'pub fn commented() -> String { let text = String::from(/*keep*/ ""); /*still here*/ text }\n'
    + '#[must_use]\n#[expect(clippy::cast_possible_truncation, reason = "existing scoped source attribution")]\n'
    + 'pub fn grouped(value: f64) -> f64 { value }\n'
    + 'pub mod nested { #[must_use] #[inline] pub fn value(value: f64) -> f64 { value } }\n';
  const styled = readPortableSourceLintStyles(source).source;
  const empty = readSourceTextRustBlock(styled, 'fn', 'empty');
  assert.ok(empty.includes('(value).is_empty()'));
  assert.ok(empty.includes('!(found).is_empty()'));
  const bound = readSourceTextRustBlock(styled, 'fn', 'bound');
  assert.equal(bound.includes('let text'), false);
  assert.ok(bound.includes('String::from("kept")'));
  const numeric = readSourceTextRustBlock(styled, 'fn', 'numeric');
  assert.equal(numeric.includes('unit.clone()'), false);
  assert.ok(numeric.includes('.is_some_and(self::ml_math::is_integer)'));
  assert.ok(numeric.includes('.unwrap_or(f64::NAN)'));
  const owned = readSourceTextRustBlock(styled, 'fn', 'owned');
  assert.equal(owned.match(/unit\.clone\(\)/gu)?.length, 2, 'owned Option strings preserve both required copies');
  assert.ok(readSourceTextRustBlock(styled, 'fn', 'unknown').includes('value == ""'));
  const commented = readSourceTextRustBlock(styled, 'fn', 'commented');
  assert.ok(commented.includes('String::from(/*keep*/ "")'));
  assert.ok(commented.includes('/*still here*/'));
  const grouped = styled.slice(styled.indexOf('#[expect(clippy::cast_possible_truncation'), styled.indexOf('pub fn grouped'));
  assert.equal(grouped.includes('#[must_use]'), false, 'an intervening scoped expectation does not duplicate its attached must_use attribute');
  assert.ok(styled.includes('#[must_use]\n#[expect(clippy::cast_possible_truncation'));
  assert.ok(styled.includes('#[must_use] #[inline] pub fn value'), 'nested runtime prefixes keep their complete existing attribute group');
});

test('actual repeated string predicates factor only immutable standard receiver reads', () => {
  const provider = readFileSync(new URL('../src/translation/primitive-method-rules.js', import.meta.url), 'utf8');
  const start = provider.indexOf('export function readPrimitiveMethodMaximumArguments(');
  assert.ok(start >= 0);
  const source = provider.slice(provider.lastIndexOf('/**', start), provider.indexOf('\n}', start) + 2);
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const raw = emitRust(checked, {arrayConsumption: true}).definitions.join('\n\n');
  const loaned = readPortableSourceParameterStyles(raw).source;
  const reviewed = readPortableSourceLintStyles(loaned);
  const block = readSourceTextRustBlock(reviewed.source, 'fn', 'read_primitive_method_maximum_arguments');
  assert.equal((block.match(/receiver == "array"/gu) ?? []).length, 1, 'the actual merged receiver predicate is evaluated once');
  for (const operation of ['slice', 'includes', 'indexOf', 'lastIndexOf']) assert.ok(block.includes('operation == "' + operation + '"'));
  const styles = new DecoratorSet([{id: 'pure-repeated-string-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: reviewed.actions}]);
  assert.equal(styles.decorate('emitter', {format: 'Rust', source: loaned}).source, reviewed.source);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  for (const receiver of ['array', 'string', 'unknown']) for (const operation of ['slice', 'includes', 'indexOf', 'lastIndexOf', 'repeat', 'codePointAt', 'split', 'replace', 'padStart', 'unknown']) {
    const expected = receiver === 'array' ? ['slice', 'includes', 'indexOf', 'lastIndexOf'].includes(operation) ? 2 : -1
      : receiver === 'string' ? ['repeat', 'codePointAt'].includes(operation) ? 1
        : ['split', 'slice', 'indexOf', 'lastIndexOf', 'replace', 'replaceAll', 'padStart', 'padEnd'].includes(operation) ? 2 : -1 : -1;
    assert.equal(context.readPrimitiveMethodMaximumArguments(receiver, operation), expected);
  }
  const condition = '((receiver == "array") && (operation == "slice")) || ((receiver == "array") && (operation == "includes"))';
  for (const type of ['String', '&str']) {
    const predicateSource = 'pub fn pure(receiver: ' + type + ', operation: ' + type + ') -> bool { ' + condition + ' }';
    const result = readPortableSourceLintStyles(predicateSource).source;
    assert.equal((result.match(/receiver == "array"/gu) ?? []).length, 1);
    if (type === 'String') {
      const contexts = [predicateSource, result].map(candidate => {
        const replay = {};
        runInNewContext(emitJavaScript(checkProgram(parseRust(candidate + '\nfn main(){}'))).text, replay, {timeout: 1000});
        return replay;
      });
      for (const receiver of ['array', 'string', 'unknown', '', '𝌆']) for (const operation of ['slice', 'includes', 'indexOf', '', '𝌆']) {
        const expected = receiver === 'array' && (operation === 'slice' || operation === 'includes');
        assert.equal(contexts[0].pure(receiver, operation), expected, 'the original owned String predicate has the independent expected behavior');
        assert.equal(contexts[1].pure(receiver, operation), contexts[0].pure(receiver, operation), 'factoring preserves the checked Rust predicate behavior');
      }
    }
  }
  const declined = [
    'pub fn custom(receiver: Custom, operation: &str) -> bool { ' + condition + ' }',
    'struct String;\npub fn custom(receiver: String, operation: String) -> bool { ' + condition + ' }',
    'pub fn effect(receiver: &str, operation: &str) -> bool { ((receiver == "array") && self::observe()) || ((receiver == "array") && (operation == "slice")) }',
    'pub fn comment(receiver: &str, operation: &str) -> bool { ((receiver == "array") /* retain */ && (operation == "slice")) || ((receiver == "array") && (operation == "includes")) }',
    'pub fn shadow(receiver: &str, operation: &str) -> bool { let receiver = other(); ' + condition + ' }',
  ];
  for (const candidate of declined) assert.equal((readPortableSourceLintStyles(candidate).source.match(/receiver == "array"/gu) ?? []).length, 2);
  assert.ok(readPortableSourceLintStyles(declined[3]).source.includes('/* retain */'));
});

test('qualified String return proofs require the declared local module namespace', () => {
  const source = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8')
    + '/** @param {string[]} columns @param {number} size @param {number} node @returns {boolean} */ function present(columns,size,node){return readRequiredTextFact(columns,size+node)!=="";}';
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const modulePath = 'crate::translation::surface_reference_analysis_rules';
  const raw = emitRust(checked, {arrayConsumption: true}).definitions.join('\n\n')
    .replaceAll('crate::read_required_text_fact', modulePath + '::read_required_text_fact');
  const loaned = readPortableSourceParameterStyles(raw).source;
  const defaultResult = readPortableSourceLintStyles(loaned).source;
  assert.equal(readSourceTextRustBlock(defaultResult, 'fn', 'present').includes('.is_empty()'), false);
  const reviewed = readPortableSourceLintStyles(loaned, {modulePath});
  const present = readSourceTextRustBlock(reviewed.source, 'fn', 'present');
  assert.ok(present.includes('.is_empty()'));
  assert.ok(present.includes('!(' + modulePath + '::read_required_text_fact('));
  assert.equal(present.includes('!((' + modulePath + '::read_required_text_fact('), false, 'the actual qualified fact read does not reproduce the Clippy double_parens receiver');
  assert.equal((present.match(/::read_required_text_fact\(/gu) ?? []).length, 1, 'the effectful call and its arithmetic argument remain single evaluations');
  assert.ok(present.includes('size + node'));
  const unrelated = loaned.replaceAll(modulePath + '::read_required_text_fact', 'crate::unrelated::read_required_text_fact');
  assert.equal(readSourceTextRustBlock(readPortableSourceLintStyles(unrelated, {modulePath}).source, 'fn', 'present').includes('.is_empty()'), false);
  const unknown = 'pub fn value() -> Custom { self::make() }\npub fn present() -> bool { (' + modulePath + '::value()) != "" }';
  assert.equal(readPortableSourceLintStyles(unknown, {modulePath}).source.includes('.is_empty()'), false);
  const styles = new DecoratorSet([{id: 'qualified-string-return-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: reviewed.actions}]);
  assert.equal(styles.decorate('emitter', {format: 'Rust', source: loaned}).source, reviewed.source);
  const call = modulePath + '::value()';
  for (const receiver of [call, '(' + call + ')']) for (const comparison of ['==', '!=']) {
    const candidate = 'pub fn value() -> String { String::from("text") }\npub fn present() -> bool { ' + receiver + ' ' + comparison + ' "" }';
    const styled = readPortableSourceLintStyles(candidate, {modulePath});
    const body = readSourceTextRustBlock(styled.source, 'fn', 'present');
    assert.ok(body.includes((comparison === '!=' ? '!' : '') + '(' + call + ').is_empty()'), 'both grouped and ungrouped receivers keep one complete method-receiver group');
    assert.equal(body.includes('((' + call + '))'), false);
    assert.equal((body.match(/::value\(/gu) ?? []).length, 1, 'String comparison styling preserves one receiver evaluation');
    assert.equal(readPortableSourceLintStyles(styled.source, {modulePath}).source, styled.source, 'receiver grouping is idempotent');
    const receiverStyles = new DecoratorSet([{id: 'grouped-empty-string-control', level: 'emitter', order: 0,
      when: [['format', 'Rust']], actions: styled.actions}]);
    assert.equal(receiverStyles.decorate('emitter', {format: 'Rust', source: candidate}).source, styled.source);
  }
});
