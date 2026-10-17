// Focused JavaScript and source-replay controls; native execution belongs to CI.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {acceptRustDiagnosticAttributeOffset, planRustDiagnosticAttribute, readRustMustUseAttributeEnd,
  planRustDiagnosticAttributePrefix} from '../src/translation/rust-diagnostic-attribute-rules.js';
import {readRustDiagnosticAttributeRuleSource} from '../scripts/rust-diagnostic-attribute-policy.mjs';
import {prepareRustDiagnosticAttributeTranslation, readRustDiagnosticAttributeEmissionData} from '../scripts/generate-rust-diagnostic-attribute-rules.mjs';
import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';

import {tokenize} from '../src/translation/lexer.js';
const columns = source => {
  const {tokens, comments} = tokenize(source, 'Rust');
  return {tokens, comments, kinds: tokens.map(token => token.kind), values: tokens.map(token => token.value)};
};
const basic = '#[expect(clippy::fn_params_excessive_bools, reason = "why")]';
const positive = [
  [basic, 0, [13, 2, 4, 6, 8, 10]],
  ['#[expect(clippy::fn_params_excessive_bools, reason = "",)]', 0, [14, 2, 4, 6, 8, 10]],
  ['/* 😀 */\n#[ expect (clippy /* owner */ :: fn_params_excessive_bools,\nreason = "why" ,\n)]', 0, [14, 2, 4, 6, 8, 10]],
  ['#[expect(clippy::fn_params_excessive_bools, reason = r###" ] custom!(x) "###)]', 0, [13, 2, 4, 6, 8, 10]],
  ['#[expect(clippy::fn_params_excessive_bools, reason = "\\u{1F600} é")]', 0, [13, 2, 4, 6, 8, 10]],
  ['prefix ' + basic + ' suffix', 1, [14, 3, 5, 7, 9, 11]],
  [basic + '\n#[must_use]\npub const fn keep(a:bool)->bool{a}fn main(){}', 0, [13, 2, 4, 6, 8, 10]],
];
const negative = [
  '#![expect(clippy::fn_params_excessive_bools, reason="why")]',
  '#[allow(clippy::fn_params_excessive_bools, reason="why")]', '#[must_use]',
  '#[cfg(feature="x")]', '#[cfg_attr(x,expect(clippy::fn_params_excessive_bools,reason="x"))]',
  '#[derive(Clone)]', '#[inline]', '#[no_mangle]', '#[export_name="name"]', '#[target_feature(enable="x")]',
  '#[custom::expect(clippy::fn_params_excessive_bools,reason="why")]',
  '#[expect(other::fn_params_excessive_bools, reason="why")]', '#[expect(clippy::unknown,reason="why")]',
  '#[expect(clippy::fn_params_excessive_bools)]', '#[expect(clippy::fn_params_excessive_bools,)]',
  '#[expect(clippy::fn_params_excessive_bools, reason=1)]',
  '#[expect(clippy::fn_params_excessive_bools, reason=concat!("why"))]',
  '#[expect(clippy::fn_params_excessive_bools, reason=("why"))]',
  '#[expect(clippy::fn_params_excessive_bools, reason="why", reason="again")]',
  '#[expect(clippy::fn_params_excessive_bools, clippy::fn_params_excessive_bools, reason="why")]',
  '#[expect(clippy::fn_params_excessive_bools, reason="why", extra)]',
  '#[expect(clippy::fn_params_excessive_bools, reason="why",,)]',
  '#[expect(clippy::fn_params_excessive_bools reason="why")]',
  '#[expect(clippy::fn_params_excessive_bools, reason="why"])]',
  '#[expect{clippy::fn_params_excessive_bools, reason="why"}]',
  '#[expect(clippy::fn_params_excessive_bools, reason="why")}',
];

test('private diagnostic token plan consumes actual outer occurrences with exact absolute token positions', () => {
  for (const [source, start, expected] of positive) {
    const input = columns(source), before = structuredClone(input);
    assert.deepEqual(planRustDiagnosticAttribute(input.kinds, input.values, start), expected);
    assert.equal(input.tokens[expected[5]].kind, 'string');
    assert.equal(source.slice(input.tokens[start].start, input.tokens[expected[0] - 1].end).at(-1), ']');
    assert.deepEqual(input, before);
  }
  const input = columns(positive[2][0]);
  assert.equal(input.tokens[0].start, 9, 'the non-BMP comment contributes two UTF16 code units');
  assert.equal(input.tokens[0].start, positive[2][0].indexOf('#'), 'actual UTF16 source offsets remain lexer-owned');
  assert.equal(input.comments.length, 2);
  const unicode = columns(positive[4][0]);
  assert.equal(unicode.tokens[10].value, '😀 é');
  assert.equal(unicode.tokens[10].value.length, 4);
  assert.equal(unicode.tokens[10].raw, '"\\u{1F600} é"');
});

test('private diagnostic token plan refuses unknown duplicate semantic macro and malformed forms', () => {
  for (const source of negative) {
    const input = columns(source);
    assert.deepEqual(planRustDiagnosticAttribute(input.kinds, input.values, 0), [-1], source);
  }
  // Token recognition does not grant the pending attribute to a following
  // item; the later parser/facet authority must reject these attachments.
  for (const following of ['fn main(){}', 'mod nested{}', 'enum Other{One}', 'const X:bool=true;']) {
    const input = columns(basic + following);
    assert.deepEqual(planRustDiagnosticAttribute(input.kinds, input.values, 0), [13, 2, 4, 6, 8, 10]);
  }
});

test('private diagnostic token plan fails closed on selected kind value hole and index mutations', () => {
  const input = columns(basic), kinds = input.kinds, values = input.values;
  for (let at = 0; at < 13; at += 1) {
    const changedKinds = [...kinds]; changedKinds[at] = kinds[at] === 'punct' ? 'identifier' : 'punct';
    assert.deepEqual(planRustDiagnosticAttribute(changedKinds, values, 0), [-1]);
    const holeKinds = [...kinds]; delete holeKinds[at];
    assert.deepEqual(planRustDiagnosticAttribute(holeKinds, values, 0), [-1]);
    const holeValues = [...values]; delete holeValues[at];
    assert.deepEqual(planRustDiagnosticAttribute(kinds, holeValues, 0), [-1]);
    if (at !== 10) {
      const changedValues = [...values]; changedValues[at] = 'different';
      assert.deepEqual(planRustDiagnosticAttribute(kinds, changedValues, 0), [-1]);
    }
  }
  for (const payload of [undefined, null, 0, true, {}, new String('why')]) {
    const changed = [...values]; changed[10] = payload;
    assert.deepEqual(planRustDiagnosticAttribute(kinds, changed, 0), [-1]);
  }
  for (const start of [-1, 0.5, NaN, Infinity, -Infinity, 9007199254740992, kinds.length, true, '0']) {
    assert.deepEqual(planRustDiagnosticAttribute(kinds, values, start), [-1]);
  }
  assert.deepEqual(planRustDiagnosticAttribute(kinds, values, -0), [13, 2, 4, 6, 8, 10]);
  for (let length = 0; length < 13; length += 1) assert.deepEqual(planRustDiagnosticAttribute(kinds.slice(0, length), values.slice(0, length), 0), [-1]);
  assert.deepEqual(planRustDiagnosticAttribute(kinds.slice(1), values, 0), [-1]);
  assert.deepEqual(planRustDiagnosticAttribute(kinds, values.slice(1), 0), [-1]);
  assert.equal(readFileSync(new URL('../src/translation/rust-diagnostic-attribute-rules.js', import.meta.url), 'utf8'), readRustDiagnosticAttributeRuleSource());
  for (const value of [0, -0, 1, 9007199254740991]) assert.equal(acceptRustDiagnosticAttributeOffset(value), true);
  for (const value of [-1, 0.5, NaN, Infinity, -Infinity, 9007199254740992, null, true, '0']) assert.equal(acceptRustDiagnosticAttributeOffset(value), false);
});

test('private complete diagnostic prefix consumes every ordered legacy attribute and final simple visibility token', () => {
  const compact = source => {
    const input = columns(source), tokens = input.tokens.filter(token => token.kind !== 'eof');
    return {kinds: tokens.map(token => token.kind), values: tokens.map(token => token.value)};
  };
  for (const [source, expected] of [
    [basic, [13, 1, 1, -1]], [basic + ' pub', [14, 1, 1, 13]],
    ['#[must_use]' + basic, [17, 2, 1, -1]],
    [basic + '#[must_use] pub', [18, 2, 1, 17]],
    ['#[must_use]' + basic + '#[must_use] pub', [22, 3, 1, 21]],
    [basic + basic, [26, 2, 2, -1]],
    ['/* 😀 prefix */#[must_use]\n' + basic + ' /* é comment */ pub', [18, 2, 1, 17]],
  ]) {
    const input = compact(source);
    assert.deepEqual(planRustDiagnosticAttributePrefix(input.kinds, input.values), expected);
  }
  const legacy = compact('#[must_use]');
  assert.equal(readRustMustUseAttributeEnd(legacy.kinds, legacy.values, 0), 4);
  assert.deepEqual(planRustDiagnosticAttributePrefix(legacy.kinds, legacy.values), [-1], 'must_use-only source keeps its old path');
  for (const source of ['', 'pub', '#[must_use] pub', 'pub ' + basic, basic + ' pub(crate)',
    basic + ' pub pub', basic + ' fn wrong(){}', basic + ' const X:bool=true;', basic + '#[must_use()]',
    basic + '#[cfg(anything)]', '#![must_use]' + basic, '#[must_use(reason="x")]' + basic]) {
    const input = compact(source);
    assert.deepEqual(planRustDiagnosticAttributePrefix(input.kinds, input.values), [-1], source);
  }
  const eof = columns(basic);
  assert.deepEqual(planRustDiagnosticAttributePrefix(eof.kinds, eof.values), [-1], 'EOF is not silently discarded from a facet token inventory');
});

test('private complete diagnostic module retains every owner and decision through both checked Links archives', async () => {
  const prepared = await prepareRustDiagnosticAttributeTranslation();
  assert.deepEqual(prepared.roots, ['acceptRustDiagnosticAttributeOffset', 'planRustDiagnosticAttribute',
    'readRustMustUseAttributeEnd', 'planRustDiagnosticAttributePrefix']);
  assert.deepEqual(prepared.programs.map(program => program.format), ['original', 'text', 'binary']);
  for (const emission of [prepared.rust, ...prepared.programs.map(program => program.javascript)]) {
    const before = Object.getOwnPropertyDescriptors(emission);
    assert.equal(before.definitions.enumerable, false);
    assert.equal(before.preludes.enumerable, false);
    const data = readRustDiagnosticAttributeEmissionData(emission);
    assert.deepEqual(Object.keys(data), [...Object.keys(emission), 'definitions', 'preludes']);
    assert.strictEqual(data.definitions, emission.definitions);
    assert.strictEqual(data.preludes, emission.preludes);
    assert.deepEqual(readStructuredDataLino(structuredDataToLino(data)), data,
      'source artifacts retain every actual emission field and complete non-enumerable part');
    assert.deepEqual(Object.getOwnPropertyDescriptors(emission), before, 'artifact projection leaves the emitter contract intact');
  }
  for (const {format, javascript} of prepared.programs) {
    const context = {};
    vm.runInNewContext(javascript.text, context, {timeout: 1000});
    const mappings = javascript.mappings.filter(mapping => mapping.source === 'planRustDiagnosticAttribute' && mapping.kind === 'function');
    assert.equal(mappings.length, 1);
    const invoke = input => Array.from(context[mappings[0].target](input.kinds, input.values, input.start));
    for (const [source, start, expected] of positive) assert.deepEqual(invoke({...columns(source), start}), expected, format);
    for (const source of negative) assert.deepEqual(invoke({...columns(source), start: 0}), [-1], format);
    const prefixMapping = javascript.mappings.filter(mapping => mapping.source === 'planRustDiagnosticAttributePrefix' && mapping.kind === 'function');
    assert.equal(prefixMapping.length, 1);
    for (const [source, expected] of [[basic, [13, 1, 1, -1]], ['#[must_use]' + basic + ' pub', [18, 2, 1, 17]],
      [basic + basic, [26, 2, 2, -1]], ['#[must_use]', [-1]], [basic + ' fn wrong(){}', [-1]]]) {
      const tokens = columns(source).tokens.filter(token => token.kind !== 'eof');
      assert.deepEqual(Array.from(context[prefixMapping[0].target](tokens.map(token => token.kind), tokens.map(token => token.value))), expected, format);
    }
    const offsetMapping = javascript.mappings.filter(mapping => mapping.source === 'acceptRustDiagnosticAttributeOffset' && mapping.kind === 'function');
    assert.equal(offsetMapping.length, 1);
    for (const [value, expected] of [[0, true], [-0, true], [9007199254740991, true], [-1, false], [0.5, false], [NaN, false], [Infinity, false]]) {
      assert.equal(context[offsetMapping[0].target](value), expected, format);
    }
    const legacyMapping = javascript.mappings.filter(mapping => mapping.source === 'readRustMustUseAttributeEnd' && mapping.kind === 'function');
    assert.equal(legacyMapping.length, 1);
    for (const [source, expected] of [['#[must_use]', 4], ['#[must_use()]', -1], [basic, -1]]) {
      const input = columns(source);
      assert.equal(context[legacyMapping[0].target](input.kinds, input.values, 0), expected, format);
    }
  }
});
