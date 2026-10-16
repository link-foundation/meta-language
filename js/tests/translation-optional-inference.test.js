import assert from 'node:assert/strict';
import { test } from 'node:test';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { sameType } from '../src/translation/types.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { readCallableFunctionConstraints } from '../src/translation/callable-source.js';
import { createSurfaceReferenceFacts } from '../src/translation/surface-reference-facts.js';
import { readOptionalRuleSources } from '../scripts/generate-optional-rules.mjs';
import {
  readInferenceTermIdentity, readOptionalAdditionalAbsence, readOptionalCoalescingConstraints, readOptionalFieldInference,
  readOptionalJoinConstraints, readOptionalTypeConstraints,
} from '../src/translation/optional-inference.js';

const number = { kind: 'float' };
const text = { kind: 'string' };
const unit = { kind: 'unit' };

function observations(source) {
  const lines = [];
  runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines.join('\n') + '\n';
}

test('optional and callable constraints share actual child identities and preserve pair order', () => {
  const first = { kind: 'var', id: 0 };
  const second = { kind: 'var', id: 1 };
  const leftFunction = { kind: 'function', params: [first], ret: number };
  const rightFunction = { kind: 'function', params: [second], ret: number };
  const left = { kind: 'option', element: leftFunction, absence: 'null' };
  const right = { kind: 'option', element: rightFunction, absence: 'undefined' };
  const plan = readOptionalTypeConstraints(left, right);
  assert.equal(plan.pair[0], leftFunction);
  assert.equal(plan.pair[1], rightFunction);
  assert.equal(plan.absenceOwners[0], left);
  assert.equal(plan.absenceOwners[1], right);
  assert.equal(left.absence, 'null', 'planning does not prematurely join policies');
  assert.equal(right.absence, 'undefined');
  const functionPairs = readCallableFunctionConstraints(...plan.pair);
  assert.equal(functionPairs[0][0], first);
  assert.equal(functionPairs[0][1], second);
  assert.equal(functionPairs[1][0], number);
  assert.equal(functionPairs[1][1], number);
  assert.deepEqual(readOptionalTypeConstraints(left, text).pair, [leftFunction, text]);
  const reversed = readOptionalTypeConstraints(text, right);
  assert.equal(reversed.pair[0], rightFunction, 'the optional element stays first when only the right side is optional');
  assert.equal(reversed.pair[1], text);
  assert.deepEqual(reversed.absenceOwners, []);
  assert.equal(readOptionalTypeConstraints(number, text), null);
});

test('coalescing defers unresolved terms and forces the original variable without flattening its fallback', () => {
  const original = { kind: 'var', id: 0 };
  const resolved = { kind: 'var', id: 1 };
  const fallback = { kind: 'option', element: text, absence: 'null' };
  assert.equal(readOptionalCoalescingConstraints(original, resolved, fallback), null);
  const forced = readOptionalCoalescingConstraints(original, resolved, fallback, true);
  assert.equal(forced[0], original);
  assert.equal(forced[1].element, fallback, 'inference retains a nested term until ordinary surface normalization');
  assert.equal(forced[1].absence, 'both');
  const present = { kind: 'option', element: number, absence: 'both' };
  const optional = readOptionalCoalescingConstraints(original, present, fallback);
  assert.equal(optional[0], number);
  assert.equal(optional[1], fallback);
  const concrete = readOptionalCoalescingConstraints(original, number, fallback);
  assert.equal(concrete[0], original);
  assert.equal(concrete[1], fallback);
});

test('optional fields preserve the original receiver and allocate a fresh element only for an unresolved receiver', () => {
  const original = { kind: 'var', id: 0 };
  const unresolved = { kind: 'var', id: 1 };
  const fresh = readOptionalFieldInference(original, unresolved);
  assert.equal(fresh.element, undefined);
  assert.equal(fresh.bind, original);
  const optional = readOptionalFieldInference(original, { kind: 'option', element: text, absence: 'null' });
  assert.equal(optional.element, text);
  assert.equal(optional.bind, undefined);
  const concrete = readOptionalFieldInference(original, text);
  assert.equal(concrete.element, original);
  assert.equal(concrete.bind, undefined);
});

test('absent branch joins reuse exact fixed point identities and keep nested options until normalization', () => {
  const current = { kind: 'option', element: number, absence: 'undefined' };
  const stable = readOptionalJoinConstraints(unit, number, unit, number, current);
  assert.equal(stable.value, current);
  assert.equal(stable.pair, null);
  const different = readOptionalJoinConstraints(unit, number, unit, number, { ...current, element: { ...number } });
  assert.notEqual(different.value, current);
  assert.equal(different.value.element, number, 'structural equality does not substitute a different inference variable identity');
  const inner = { kind: 'option', element: text, absence: 'null' };
  const nested = readOptionalJoinConstraints(inner, unit, inner, unit, number);
  assert.equal(nested.value.element, inner);
  assert.equal(nested.value.absence, 'undefined');
  const merged = readOptionalJoinConstraints(number, inner, number, inner, current);
  assert.equal(merged.value, inner);
  assert.equal(merged.pair[0], number);
  assert.equal(merged.pair[1], inner);
});

test('actual nullable inference preserves falsy values field shapes and recursive return joins', () => {
  const cases = [
    ["function choose(value){return value??'fallback';} console.log(`${choose(undefined)} ${choose(null)} ${choose('')} ${choose('kept')}`);", 'fallback fallback  kept\n'],
    ["function length(value){return value?.length??0;} console.log(length(undefined),length(null),length(''),length('abc'));", '0 0 0 3\n'],
    ["function field(value){return value?.item??'missing';} console.log(`${field(undefined)} ${field(null)} ${field({item:''})} ${field({item:'kept'})}`);", 'missing missing  kept\n'],
    ["function f(value){return value?g(false):undefined;} function g(value){return value?f(false):1;} console.log(f(false),f(true),g(false),g(true));", 'undefined 1 1 undefined\n'],
    ["function inspect(value){console.log(`${value===null} ${value===undefined} ${!!value} ${typeof value} ${value+1}`);return value||5;}console.log(inspect(null));console.log(inspect(undefined));console.log(inspect(0));console.log(inspect(2));", 'true false false object 1\n5\nfalse true false undefined NaN\n5\nfalse false false number 1\n5\nfalse false true number 3\n2\n'],
  ];
  for (const [source, expected] of cases) {
    assert.equal(observations(source), expected, 'the original JavaScript supplies the observable oracle');
    const surface = parseJavaScript(source);
    const checked = checkProgram(surface);
    assert.equal(observations(emitJavaScript(checked).text), expected);
  }
  const inferred = parseJavaScript(cases[0][0]).items.find(item => item.name === 'choose');
  assert.deepEqual(inferred.params[0].type, { kind: 'option', element: text, absence: 'both' });
  assert.deepEqual(inferred.ret, text);
});

test('optional inference keeps incompatible values and recursive types as checked failures', () => {
  assert.throws(() => parseJavaScript("function choose(value){return value??'fallback';} choose(1);"),
    error => error.kind === 'type' && /number.*string|string.*number/u.test(error.message));
  assert.throws(() => parseJavaScript('function recursive(value){return value??[value];}'),
    error => error.kind === 'type' && /recursive inferred type/u.test(error.message));
});

test('nested absence widening retains declared annotations and inspects function results without changing parameters', () => {
  const actual = { kind: 'option', element: number, absence: 'undefined' };
  assert.equal(readOptionalAdditionalAbsence(actual, number).additional, true);
  assert.equal(readOptionalAdditionalAbsence(actual, { ...actual, absence: 'both' }).additional, false);
  const functionResult = readOptionalAdditionalAbsence({ kind: 'function', params: [actual], ret: number }, { kind: 'function', params: [number], ret: number });
  assert.equal(functionResult.additional, false);
  assert.deepEqual(functionResult.pairs, [[number, number]], 'parameter optionality does not widen a return annotation');
  const record = readOptionalAdditionalAbsence({ kind: 'record', fields: [{ name: 'later', type: text }, { name: 'value', type: actual }] }, { kind: 'record', fields: [{ name: 'value', type: number }, { name: 'later', type: text }] });
  assert.equal(record.pairs[0][0], text);
  assert.equal(record.pairs[0][1], text);
  assert.equal(record.pairs[1][0], actual);
  assert.equal(record.pairs[1][1], number);
  const surface = parseJavaScript('/** @param {number[]} values @returns {number} */ function first(values){return values[0];}');
  const first = surface.items.find(item => item.name === 'first');
  assert.deepEqual(first.ret, number);
  assert.deepEqual(first.semanticRet, actual);
  const checked = checkProgram(surface);
  assert.deepEqual(checked.declarations.get('first').annotatedRet, number);
  assert.deepEqual(checked.declarations.get('first').ret, actual);
  const forged = structuredClone(surface);
  forged.items.find(item => item.name === 'first').ret = { kind: 'bool' };
  assert.throws(() => checkProgram(forged), error => error.kind === 'type' && /annotated value types/u.test(error.message));
  const runtime = {};
  runInNewContext(emitJavaScript(checked).text, runtime, { timeout: 1000 });
  assert.equal(runtime.first([]), undefined);
  assert.equal(runtime.first([2]), 2);
});

test('the complete optional planning module is checked before all four target emissions', () => {
  const source = readOptionalRuleSources().find(source => source.name === 'optional_inference_rules').source;
  const checked = checkProgram(parseJavaScript(source));
  const javascript = emitJavaScript(checked);
  const functions = [...checked.declarations.values()].filter(entry => entry.k === 'fn').map(entry => entry.fullName).sort();
  for (const [target, emitted] of [['JavaScript', javascript], ['Rust', emitRust(checked, { arrayConsumption: true })], ['Lean', emitLean(checked)], ['Rocq', emitRocq(checked)]]) {
    assert.equal(typeof emitted.text, 'string');
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
  const optional = { kind: 'option', element: number, absence: 'null' };
  const current = { kind: 'option', element: number, absence: 'undefined' };
  const facts = createSurfaceReferenceFacts({ optional, number, unit, current });
  const identity = name => {
    for (let at = facts.starts[facts.root]; at < facts.starts[facts.root] + facts.counts[facts.root]; at += 1) {
      if (facts.labels[at] === name) return facts.targets[at];
    }
    throw new Error('a complete execution fixture is missing ' + name);
  };
  const arrays = [facts.texts, facts.starts, facts.counts, facts.labels, facts.targets];
  assert.deepEqual(Array.from(runtime.planOptionalTypeConstraints(...arrays, identity('optional'), identity('number'))), [identity('number'), identity('number'), -1, -1]);
  assert.deepEqual(Array.from(runtime.planOptionalCoalescingConstraints(...arrays, identity('optional'), identity('optional'), identity('number'), false)), [0, identity('number'), identity('number')]);
  assert.deepEqual(Array.from(runtime.planOptionalFieldInference(...arrays, identity('optional'), identity('optional'))), [identity('number')]);
  assert.deepEqual(Array.from(runtime.planOptionalJoinConstraints(...arrays, identity('unit'), identity('number'), identity('unit'), identity('number'), identity('current'))), [1, identity('current')]);
});


test('shared inference identity matches complete logical type keys without collapsing unresolved variables', () => {
  const first = { kind: 'var', id: 0 };
  const second = { kind: 'var', id: 1 };
  const values = [number, text, first, second,
    { kind: 'array', element: first }, { kind: 'array', element: second },
    { kind: 'array', element: first, reference: true },
    { kind: 'option', element: { kind: 'option', element: first, absence: 'null' }, absence: 'undefined' },
    { kind: 'option', element: first, absence: 'both' },
    { kind: 'option', element: first, absence: 'null' },
    { kind: 'record', fields: [{ name: 'z', type: first }, { name: 'a', type: second }] },
    { kind: 'record', fields: [{ name: 'a', type: second }, { name: 'z', type: first }] },
    { kind: 'record', fields: [{ name: 'a', type: second }, { name: 'z', type: first }], open: true },
    { kind: 'record', fields: [{ name: 'a', type: first }, { name: 'a', type: second }] },
    { kind: 'record', fields: [{ name: 'a', type: second }, { name: 'a', type: first }] },
    { kind: 'function', params: [first], ret: { kind: 'option', element: second, absence: 'null' } },
    { kind: 'function', params: [second], ret: { kind: 'option', element: first, absence: 'null' } },
    { kind: 'union', alternatives: [first, number] }, { kind: 'union', alternatives: [number, first] },
    { kind: 'map', key: first, value: second }, { kind: 'set', element: first },
    { kind: 'cell', element: first }, { kind: 'tuple', elements: [first, second] }];
  const compare = (left, right) => {
    const pairs = readInferenceTermIdentity(left, right);
    return pairs !== null && pairs.every(([value, expected]) => compare(value, expected));
  };
  for (const left of values) for (const right of values) {
    assert.equal(compare(left, right), sameType(left, right), JSON.stringify([left, right]));
  }
  assert.equal(compare({ kind: 'tuple', elements: [first] }, { kind: 'tuple', elements: [second] }), false);
  const source = "/** @param {string} format @returns {string} */ function captured(format){const prefix=format==='Rust'?'':'| ';const separator=format==='Rust'?', ':' ';let value=prefix;let first=0;while(first<2){let second=0;while(second<2){value+=separator;second+=1;}first+=1;}return value;}console.log(captured('Rust'));console.log(captured('Lean'));";
  const surface = parseJavaScript(source);
  const checked = checkProgram(surface);
  assert.equal(observations(emitJavaScript(checked).text), observations(source), 'deferred branch values reach parameters captured by nested generated loops');
  for (const fn of surface.items.filter(item => item.k === 'fn' && item.generated)) {
    for (const param of fn.params.filter(param => param.name === 'prefix' || param.name === 'separator')) assert.equal(param.type.kind, 'string');
  }
});
