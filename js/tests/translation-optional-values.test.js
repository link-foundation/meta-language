import { observeThreadedOptionalProgram } from './helpers/translation-output-observer.js';
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { readOptionalRuleSources } from '../scripts/generate-optional-rules.mjs';
import { threadOutput } from '../src/translation/output.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import {
  acceptOptionalLiteralAbsence, acceptOptionalPrimitiveKind, readOptionalLogicalOperation, readOptionalComparisonOperation, readOptionalOutputAbsences,
  readOptionalCallBinding, readOptionalCoercionOperation, readOptionalFieldOperation,
  readOptionalJoinBinding, readOptionalNumericTerm, requireOptionalResultBinding,
  renderOptionalAbsentValue, renderOptionalCoalescing, renderOptionalFlattening,
  renderOptionalMatch, renderOptionalPresentValue, renderOptionalProjection,
} from '../src/translation/optional-value-rules.js';

test('optional coercion plans retain distinct absence checks present wrapping and discarded unit effects', () => {
  assert.equal(readOptionalCoercionOperation('JavaScript', 'option', 'option', 'none', false, true, false), 'absence-error');
  assert.equal(readOptionalCoercionOperation('Rust', 'option', 'option', 'none', true, false, false), 'none');
  assert.equal(readOptionalCoercionOperation('JavaScript', 'option', 'unit', 'unit', false, true, false), 'unit');
  assert.equal(readOptionalCoercionOperation('JavaScript', 'option', 'unit', 'call', false, true, false), 'discard-unit');
  assert.equal(readOptionalCoercionOperation('Rust', 'option', 'unit', 'call', false, true, false), '');
  assert.equal(readOptionalCoercionOperation('JavaScript', 'option', 'string', 'var', false, true, true), 'some');
  assert.equal(readOptionalCoercionOperation('JavaScript', 'string', 'string', 'var', true, true, true), '');
  for (const [kind, field] of [['string', 'length'], ['array', 'length'], ['set', 'size'], ['map', 'size']]) assert.equal(readOptionalFieldOperation('option', kind, field), 'project');
  assert.equal(readOptionalFieldOperation('option', 'record', 'value'), 'optional-field');
  assert.equal(readOptionalFieldOperation('record', '', 'value'), 'field');
  assert.equal(readOptionalFieldOperation('option', 'float', 'value'), 'unsupported');
});

test('optional rendering preserves both absence tags lazy fallbacks and nested flattening in every target', () => {
  assert.equal(renderOptionalAbsentValue('Rust', 'both', 'null', ''), 'crate::MlOptional::Null');
  assert.equal(renderOptionalAbsentValue('Rust', 'both', 'undefined', ''), 'crate::MlOptional::Undefined');
  assert.equal(renderOptionalAbsentValue('Lean', 'both', 'undefined', ''), '(some none)');
  assert.equal(renderOptionalAbsentValue('Rocq', 'both', 'null', 'float'), '(@None (option float))');
  assert.equal(renderOptionalAbsentValue('Rocq', 'both', 'undefined', 'float'), '(Some (@None float))');
  const javascript = renderOptionalCoalescing('JavaScript', 'both', 'left()', '', 'right()');
  for (const [value, expected, calls] of [[0, 0, 0], [false, false, 0], ['', '', 0], [null, 7, 1], [undefined, 7, 1]]) {
    let leftCalls = 0;
    let rightCalls = 0;
    const actual = runInNewContext(javascript, { left() { leftCalls += 1; return value; }, right() { rightCalls += 1; return 7; } });
    assert.equal(actual, expected);
    assert.equal(leftCalls, 1);
    assert.equal(rightCalls, calls);
  }
  for (const format of ['Rust', 'Lean', 'Rocq']) {
    const present = renderOptionalPresentValue(format, 'both', 'value');
    assert.ok(present.includes('value'));
    const match = renderOptionalMatch(format, 'both', 'source', 'item', 'present', 'isNull', 'isUndefined');
    assert.ok(match.includes('isNull') && match.includes('isUndefined'));
    const flattened = renderOptionalFlattening(format, 'both', 'null', 'both', 'source');
    assert.ok(flattened.includes('ml_nested') && flattened.includes('ml_present'));
    const projection = renderOptionalProjection(format, 'both', 'source', '', 'item', 'field', '');
    assert.ok(projection.includes('field'));
  }
});

test('the actual complete optional value kernel checks emits to four targets and executes its generated JavaScript', () => {
  const source = readOptionalRuleSources().find(source => source.name === 'optional_value_rules').source;
  const checked = checkProgram(parseJavaScript(source));
  const javascript = emitJavaScript(checked);
  const functions = [...checked.declarations.values()].filter(entry => entry.k === 'fn').map(entry => entry.fullName).sort();
  for (const [target, emitted] of [['JavaScript', javascript], ['Rust', emitRust(checked, { arrayConsumption: true })], ['Lean', emitLean(checked)], ['Rocq', emitRocq(checked)]]) {
    assert.ok(emitted.text.length > 0, target);
    const mappings = emitted.mappings.filter(mapping => mapping.kind === 'function');
    assert.deepEqual(mappings.map(mapping => mapping.source).sort(), functions, target + ': every actual checked function is emitted');
    if (target === 'JavaScript' || target === 'Rust') assert.ok(emitted.definitions.length >= functions.length, target);
    else for (const mapping of mappings) {
      const name = mapping.target.split('.').at(-1).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
      const declaration = target === 'Lean' ? '^\\s*(?:(?:partial|noncomputable|unsafe)\\s+)?def\\s+' : '^\\s*(?:Definition|Fixpoint|Function|with)\\s+';
      assert.match(emitted.text, new RegExp(declaration + name + '\\b', 'mu'), target + ':' + mapping.source);
    }
  }
  const runtime = {};
  runInNewContext(javascript.text, runtime, { timeout: 1000 });
  for (const format of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    for (const absence of ['null', 'undefined']) {
      assert.equal(runtime.renderOptionalAbsentValue(format, 'both', absence, 'element'), renderOptionalAbsentValue(format, 'both', absence, 'element'));
    }
    assert.equal(runtime.renderOptionalPresentValue(format, 'both', 'child'), renderOptionalPresentValue(format, 'both', 'child'));
    assert.equal(runtime.renderOptionalCoalescing(format, 'both', 'left', 'receiver', 'right'), renderOptionalCoalescing(format, 'both', 'left', 'receiver', 'right'));
  }
});


test('shared optional inference bindings preserve refusal identity and recursive alias decisions', () => {
  assert.equal(requireOptionalResultBinding(true, false, 'option', 'var', false, false), false);
  assert.equal(requireOptionalResultBinding(true, true, 'option', 'var', false, false), true);
  assert.equal(requireOptionalResultBinding(false, false, 'var', 'var', false, false), false);
  assert.equal(requireOptionalResultBinding(false, false, 'option', 'var', false, true), false);
  assert.equal(readOptionalCallBinding('var', 'option', 'float', '', 'null', false), 'bind');
  assert.equal(readOptionalCallBinding('var', 'option', 'float', '', 'null', true), '');
  assert.equal(readOptionalCallBinding('var', 'option', 'option', 'undefined', 'null', false), 'merge');
  assert.equal(readOptionalCallBinding('var', 'option', 'option', 'both', 'null', false), 'elements');
  assert.equal(readOptionalJoinBinding(true, false, false), 'identity');
  assert.equal(readOptionalJoinBinding(false, true, false), 'changes');
  assert.equal(readOptionalJoinBinding(false, false, true), 'identity');
  assert.equal(readOptionalJoinBinding(false, false, false), 'bind');
  assert.equal(readOptionalNumericTerm('option'), 'element');
  assert.equal(readOptionalNumericTerm('unit'), 'float');
  assert.equal(readOptionalNumericTerm('var'), 'identity');
  assert.equal(readOptionalLogicalOperation('JavaScript', 'option', 'float', true, false, false, 'or'), 'unwrap-left');
  assert.equal(readOptionalLogicalOperation('JavaScript', 'option', 'float', true, false, false, 'and'), 'wrap-right');
  assert.equal(readOptionalLogicalOperation('JavaScript', 'float', 'option', false, true, false, 'or'), 'wrap-left');
  assert.equal(readOptionalLogicalOperation('Rust', 'float', 'float', false, false, true, 'or'), 'language-error');
  assert.equal(readOptionalComparisonOperation('JavaScript', 'option', 'float', 'eq', true, true), 'strict');
  assert.equal(readOptionalComparisonOperation('JavaScript', 'option', 'record', 'eq', true, false), 'object-error');
  assert.equal(readOptionalComparisonOperation('JavaScript', 'option', 'float', 'lt', true, true), 'number');
  for (const kind of ['bool', 'float', 'fixed', 'int', 'nat', 'string', 'unit']) assert.equal(acceptOptionalPrimitiveKind(kind), true);
  for (const kind of ['option', 'array', 'record', 'function', 'cell', 'map', 'set']) assert.equal(acceptOptionalPrimitiveKind(kind), false);
  assert.deepEqual(readOptionalOutputAbsences('both'), ['null', 'undefined']);
  assert.deepEqual(readOptionalOutputAbsences('null'), ['null']);
  assert.deepEqual(readOptionalOutputAbsences('undefined'), ['undefined']);
});


// Execute the actual private output program rather than evaluating generated
// target text in a different language. Its constructors carry ordered fields.

test('nullable coalescing threads fallback output and aborts only after an absent left value', () => {
  for (const aborting of [false, true]) {
    const source = "function fallback(){console.log('fallback');" + (aborting ? "throw new Error('stopped');" : 'return 7;') + '}'
      + 'function choose(value){return value??fallback();} console.log(choose(0));console.log(choose(null));console.log(choose(undefined));';
    const original = { lines: [], error: '' };
    try { runInNewContext(source, { console: { log(value) { original.lines.push(String(value)); } } }, { timeout: 1000 }); }
    catch (error) { original.error = error.message; }
    const checked = checkProgram(parseJavaScript(source));
    const actual = threadOutput(checked);
    assert.equal(actual.outputThreaded, true);
    assert.deepEqual(observeThreadedOptionalProgram(actual), original);
    assert.deepEqual(original.lines, aborting ? ['0', 'fallback'] : ['0', 'fallback', '7', 'fallback', '7']);
    assert.equal(original.error, aborting ? 'stopped' : '');
    for (const emitted of [emitLean(checked), emitRocq(checked)]) {
      assert.ok(emitted.text.includes('ml_optional'), 'the actual formal output binds the left once before testing absence');
    }
  }
});


test('concrete absent tags reject ambiguous both through checking and source-free Links while the both type policy remains valid', () => {
  const type = { kind: 'option', element: { kind: 'float' }, absence: 'both' };
  const surface = absence => ({ language: 'JavaScript', items: [{ k: 'fn', name: 'empty', params: [], ret: type, body: { k: 'none', absence } }], main: null });
  assert.equal(acceptOptionalLiteralAbsence('both'), false);
  assert.throws(() => checkProgram(surface('both')), /concrete null or undefined tag/u);
  for (const absence of ['null', 'undefined']) {
    assert.equal(acceptOptionalLiteralAbsence(absence), true);
    const checked = checkProgram(surface(absence));
    const packet = encodeTypedProgramLinks(checked);
    const restored = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    assert.deepEqual(restored.declarations.get('empty').ret, type);
    assert.equal(restored.declarations.get('empty').body.absence, absence);
    assert.equal(runInNewContext(emitJavaScript(restored).text + '\nempty();', {}, { timeout: 1000 }), absence === 'null' ? null : undefined);
    for (const emitted of [emitRust(restored), emitLean(restored), emitRocq(restored)]) assert.ok(emitted.text.length > 0);
    const forged = structuredClone(restored);
    forged.declarations.get('empty').body.absence = 'both';
    assert.throws(() => encodeTypedProgramLinks(forged), /concrete null or undefined tag/u);
    const mismatched = structuredClone(restored);
    const fn = mismatched.declarations.get('empty');
    fn.ret = { ...type, absence: absence === 'null' ? 'undefined' : 'null' };
    fn.body.type = fn.ret;
    assert.throws(() => encodeTypedProgramLinks(mismatched), /retained absence policy/u);
    assert.throws(() => checkProgram({ ...surface(absence), items: [{ ...surface(absence).items[0], ret: fn.ret }] }), /retained absence policy/u);
  }
});
