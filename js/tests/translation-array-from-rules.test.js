import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { format } from 'node:util';
import vm from 'node:vm';
import { test } from 'node:test';
import { readArrayFromSourceModule } from '../scripts/generate-array-from-rules.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { translateProgram } from '../src/program-translation.js';
import { typeError, unsupported } from '../src/translation/diagnostics.js';
import { array, BOOL, FLOAT, STRING, sameType, typeKey } from '../src/translation/types.js';
import { readArrayFromArgumentError, readArrayFromMapCallbackError, readArrayFromReceiverError, renderArrayFromMapping } from '../src/translation/array-from-rules.js';
import { readArrayFromCallbackConstraints, readArrayFromExpectedElement, readArrayFromInferenceInputs, readArrayFromIterableType, readArrayFromMapCallback } from '../src/translation/array-from.js';
import { renderStringPointIteration } from '../src/translation/array-value-rules.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const oracles = readProjectData(new URL('./fixtures/array-from-original-oracles.lino', import.meta.url));
assert.equal(oracles.originalPayloadSha256, 'f5acea4921002a50008087829601931fa40794deb019546c3649e1770ec4870e');
assert.equal(createHash('sha256').update(JSON.stringify(oracles.snapshot, null, 2) + '\n').digest('hex'), oracles.originalPayloadSha256, 'the inspectable Links records reconstruct the complete authenticated original payload bytes');
const original = oracles.snapshot.parts;
for (const [name, source] of Object.entries(original)) assert.equal(createHash('sha256').update(source).digest('hex'), oracles.partSha256[name], name + ': original source bytes');
const mapOracle = oracles.mapSource;
assert.equal(createHash('sha256').update(mapOracle).digest('hex'), '615d52168b1061e3cb726d161f747a7947da85091cac300090d7d3a05b7e3b8d', 'the original complete method checker remains the map rederivation oracle');

function section(file, start, end) {
  const source = readFileSync(new URL('../src/translation/' + file, import.meta.url), 'utf8');
  return source.slice(source.indexOf(start), source.indexOf(end, source.indexOf(start)));
}

function caseFunction(source) {
  return new Function('typeError', 'unsupported', 'array', 'STRING', 'FLOAT',
    'readArrayFromArgumentError', 'readArrayFromReceiverError', 'readArrayFromIterableType', 'readArrayFromExpectedElement', 'readArrayFromInferenceInputs',
    'return function(node, env, path, expected){switch(node.k){' + source + '}}')(
    typeError, unsupported, array, STRING, FLOAT, readArrayFromArgumentError, readArrayFromReceiverError,
    readArrayFromIterableType, readArrayFromExpectedElement, readArrayFromInferenceInputs);
}

function observe(source) {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines;
}

test('Array.from checker extraction preserves receiver identity, callback expectations and exact original diagnostics', () => {
  const before = caseFunction(original.checker);
  const after = caseFunction(section('check.js', "      case 'arrayFrom': {", "      case 'forOfIterable':"));
  const element = { kind: 'float' }, key = { kind: 'string' }, value = { kind: 'bool' };
  const mapper = { k: 'lambda', type: { kind: 'function', params: [], ret: value } };
  const expected = { kind: 'array', element: key };
  for (const kind of ['array', 'string', 'set', 'map', 'float']) {
    const object = { k: 'var', name: 'values', type: { kind, element, key, value, reference: true } };
    for (const args of [[], [object], [object, mapper], [object, null], [object, mapper, object]]) {
      const node = { k: 'arrayFrom', args, span: { start: 10, end: 50 } };
      const run = implementation => {
        const trace = [];
        const host = { expr: (...args) => { trace.push(args); return args[0]; } };
        try { return { value: implementation.call(host, node, 'environment', 'path', expected), trace }; }
        catch (error) { return { error: { kind: error.kind, reason: error.reason, span: error.span }, trace }; }
      };
      const left = run(before), right = run(after);
      assert.deepEqual(right, left, kind + ':' + args.length);
      if (right.value && args.length === 1 && kind === 'array') assert.equal(right.value, object);
      if (right.value?.k === 'method') {
        assert.equal(right.value.args[0], mapper);
        assert.equal(right.trace[1][3].params[1], FLOAT);
        assert.equal(right.trace[1][3].ret, key);
      }
    }
  }
});

test('Array.from inference extraction retains original terms, lazy expected elements and every argument evaluation', () => {
  const before = caseFunction(original.inference);
  const after = caseFunction(section('javascript-infer.js', "      case 'arrayFrom': {", "      case 'arrayIsArray':"));
  const args = [{ kind: 'var', id: 1 }, { kind: 'var', id: 2 }, { kind: 'var', id: 3 }];
  for (const expected of [undefined, { kind: 'float' }, { kind: 'array', element: args[2] }, { kind: 'array' }]) {
    for (const count of [0, 1, 2, 3]) {
      const node = { k: 'arrayFrom', args: args.slice(0, count), span: { start: 1, end: 8 } };
      const fresh = { kind: 'var', id: 99 };
      const run = implementation => {
        const trace = [], iterables = [], collections = new Map();
        const host = { iterables, collections, expr: arg => { trace.push(arg); return arg; }, fresh: () => { trace.push('fresh'); return fresh; } };
        return { result: implementation.call(host, node, 'environment', '', expected), trace, iterables, collections };
      };
      const left = run(before), right = run(after);
      assert.deepEqual(right, left);
      assert.deepEqual(right.trace.slice(0, count), args.slice(0, count));
      assert.equal(right.result.element, expected?.kind === 'array' ? expected.element : fresh);
      assert.equal(right.collections.get(node), right.result);
      if (count) { assert.equal(right.iterables[0].object, args[0]); assert.equal(right.iterables[0].mapper, args.slice(0, count)[1]); }
    }
  }
});

test('callback plans preserve fewer supplied arguments, original pair identities and the shared unifier order', () => {
  const before = new Function('typeError', 'return ({' + original.callback + '}).callback;')(typeError);
  const after = new Function('typeError', 'readArrayFromCallbackConstraints',
    'return ({' + section('javascript-infer.js', '  callback(term, params, ret, where) {', '  prop(prop, env) {') + '}).callback;')(typeError, readArrayFromCallbackConstraints);
  const provided = [{ kind: 'float' }, { kind: 'float' }];
  const ret = { kind: 'string' };
  const term = { kind: 'var', id: 1 };
  for (const count of [0, 1, 2, 3]) {
    const declared = { kind: 'function', params: [term, ret, provided[0]].slice(0, count), ret: term };
    for (const selected of [declared, term]) {
      const run = implementation => {
        const pairs = [];
        const host = { resolve: () => selected, unify: (...pair) => pairs.push(pair) };
        try { implementation.call(host, term, provided, ret, { start: 2, end: 7 }); return { pairs }; }
        catch (error) { return { pairs, reason: error.reason, span: error.span }; }
      };
      const left = run(before), right = run(after);
      assert.deepEqual(right, left);
      right.pairs.forEach((pair, index) => { assert.equal(pair[0], left.pairs[index][0]); if (selected.kind === 'function') assert.equal(pair[1], left.pairs[index][1]); });
      if (selected === term) { assert.equal(right.pairs[0][1].params, provided); assert.equal(right.pairs[0][1].ret, ret); }
    }
  }
  assert.equal(readArrayFromCallbackConstraints(term, { kind: 'function', params: [], ret }, provided, ret).pairs[0][0], ret);
});

test('iterable inference plans preserve ordered Map children and the original pending constraint behavior', () => {
  const before = new Function('STRING', 'FLOAT', 'return function(){let progress=false;' + original.iterable + 'return progress;};')(STRING, FLOAT);
  const after = new Function('STRING', 'FLOAT', 'readArrayFromIterableType',
    'return function(){let progress=false;' + section('javascript-infer.js', '      for (const constraint of [...this.iterables]) {', '      for (const constraint of [...this.indices]) {') + 'return progress;};')(STRING, FLOAT, readArrayFromIterableType);
  const key = { kind: 'var', id: 1 }, value = { kind: 'var', id: 2 }, result = { kind: 'array', element: key };
  for (const kind of ['array', 'string', 'set', 'map', 'var']) for (const mapper of [undefined, value]) {
    const object = { kind, element: value, key, value };
    const constraint = { object, mapper, result, where: { start: 0, end: 1 } };
    const run = implementation => {
      const trace = [], iterables = [constraint];
      const host = { iterables, resolve: term => term, callback: (...args) => trace.push(['callback', ...args]), unify: (...args) => trace.push(['unify', ...args]) };
      return { progress: implementation.call(host), trace, iterables };
    };
    assert.deepEqual(run(after), run(before));
    const selected = readArrayFromIterableType(object);
    if (kind === 'map') { assert.equal(selected.element.elements[0], key); assert.equal(selected.element.elements[1], value); }
    if (kind === 'array' || kind === 'set') assert.equal(selected.element, value);
  }
});

test('checked map rederivation preserves original callback type ownership and kind count parameter refusals', () => {
  const method = source => new Function('typeError', 'unsupported', 'FLOAT', 'BOOL', 'array', 'sameType', 'typeKey', 'readArrayFromMapCallback', 'readArrayFromMapCallbackError',
    'return ({' + source + '}).method;')(typeError, unsupported, FLOAT, BOOL, array, sameType, typeKey, readArrayFromMapCallback, readArrayFromMapCallbackError);
  const before = method(mapOracle), after = method(section('check.js', '  method(node, env, path, expected) {', '  arrayLiteral('));
  const element = { kind: 'float' }, ret = { kind: 'string' };
  const object = { k: 'var', name: 'values', type: { kind: 'array', element } };
  for (const count of [0, 1, 2, 3, 4]) for (const mismatch of [false, true]) {
    const params = [element, FLOAT, object.type, ret].slice(0, count);
    if (mismatch && params.length) params[params.length - 1] = { kind: 'string' };
    for (const callbackType of [{ kind: 'function', params, ret }, { kind: 'float' }]) {
      const callback = { k: 'lambda', type: callbackType };
      for (const args of [[], [callback], [callback, callback]]) {
        const node = { k: 'method', op: 'map', object, args, span: { start: 3, end: 12 } };
        const run = implementation => {
          const trace = [];
          const host = { language: 'JavaScript', referenceValue: value => value, expr: (...args) => { trace.push(args); return args[0]; } };
          try { return { result: implementation.call(host, node, 'environment', 'path', undefined), trace }; }
          catch (error) { return { reason: error.reason, span: error.span, trace }; }
        };
        const left = run(before), right = run(after);
        assert.deepEqual(right, left);
        if (right.result) {
          assert.equal(right.result.args[0], callback);
          assert.equal(right.result.object, object);
          assert.equal(right.result.type.element, ret);
          assert.equal(right.trace[1][3].params[0], element);
          assert.equal(right.trace[1][3].params[2], object.type);
        }
      }
    }
  }
});

test('the extracted map formatter retains all four original target bytes and child emission order', () => {
  for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) for (const count of [0, 1, 2, 3]) {
    const trace = [];
    const expression = { op: 'map', object: { type: { kind: 'array' }, text: 'receiver()' }, args: [{ type: { params: Array(count).fill(FLOAT) }, text: 'callback()' }] };
    let expected;
    if (target === 'JavaScript') {
      const formatter = new Function(original.JavaScript.replace('export ', '') + ';return renderPrimitiveMethodExpression;')();
      expected = formatter('JavaScript', 'array', 'map', 'receiver()', 'callback()');
    } else {
      const method = new Function('return ({' + original[target] + '}).method;')();
      const host = { temporaries: 6, expr: node => { trace.push(node.text); return node.text; }, floats: () => trace.push('floats') };
      expected = method.call(host, expression, 0);
      assert.deepEqual(trace, [...(target === 'Rocq' && count > 1 ? ['floats'] : []), 'receiver()', 'callback()']);
    }
    assert.equal(renderArrayFromMapping(target, 'receiver()', 'callback()', count, '7'), expected, target + ':' + count);
  }
});

test('actual Array.from strings and mappers retain code points, callback arities and evaluation order after source free reload', () => {
  const source = `/** @returns {string} */
function text(){console.log('receiver');return 'café😀';}
/** @returns {number} */
function probe(){const points=Array.from(text());const zero=Array.from([1,2],()=>7);const one=Array.from([2,4],value=>value*2);const two=Array.from([2,4],(value,index)=>value+index);return points.length+zero[0]+one[1]+two[1];}
console.log(probe(),Array.from('').length);`;
  const expected = ['receiver', '25 0'];
  assert.deepEqual(observe(source), expected);
  const checked = checkProgram(parseJavaScript(source));
  assert.deepEqual(observe(emitJavaScript(checked).text), expected);
  const iterator = renderStringPointIteration('Rust', false, '');
  assert.ok(emitRust(checked).text.includes(iterator), 'the existing scalar-string runtime selects its chars code-point iterator');
  const saved = createTypedProgramLinks(source, 'JavaScript');
  for (const binary of [false, true]) {
    const network = binary ? decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network)) : LinkNetwork.fromLino(saved.network.toLino());
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.deepEqual(observe(emitTypedProgramLinks(network, saved.root, 'JavaScript').code), expected);
    assert.ok(emitTypedProgramLinks(network, saved.root, 'Rust').code.includes(iterator));
  }
  const scalar = checkProgram(parseJavaScript('/** @returns {number[]} */ function values(){return Array.from([1,2],(value,index)=>value+index);}'));
  assert.ok(emitLean(scalar).text.includes('ml_values.toList.zipIdx.map'));
  assert.ok(emitRocq(scalar).text.includes('List.map (fun ml_pair'));
});

test('Array.from keeps collection receiver selection and existing source refusals', () => {
  const source = "function sizes(){return Array.from(new Set([1,1,2])).length+Array.from(new Map([[1,2],[3,4]])).length;}console.log(sizes());";
  assert.deepEqual(observe(source), ['4']);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(source))).text), ['4']);
  for (const source of [
    'function copy(){return Array.from().length;}',
    'function copy(){return Array.from([1],2).length;}',
    'function copy(){return Array.from(2).length;}',
    'function copy(){return Array.from([1],(value,index,extra)=>value).length;}',
    'function copy(){return Array.from([1],value=>value,[2]).length;}',
  ]) assert.ok(translateProgram(source, 'JavaScript', 'Rust').diagnostic, source);
});

test('the complete Array.from source and actual dependencies check and self translate every declaration', () => {
  const source = readArrayFromSourceModule();
  const surface = parseJavaScript(source, { referenceSemantics: false });
  const declared = [...source.matchAll(/^(?:export )?function ([A-Za-z][A-Za-z0-9]*)\(/gmu)].map(match => match[1]);
  assert.equal(declared.length, 26, 'thirteen Array.from declarations plus the actual six fact and seven iterable declarations');
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name), declared);
  const checked = checkProgram(surface);
  const functions = [...checked.declarations.values()].filter(declaration => declaration.k === 'fn');
  assert.equal(functions.length, surface.items.filter(item => item.k === 'fn').length, 'all original and compiler-created continuation functions remain checked');
  const emitted = emitRust(checked, { arrayConsumption: true });
  for (const name of ['plan_array_from_iterable_type', 'plan_array_from_inference_inputs', 'plan_array_from_callback_constraints', 'render_array_from_mapping', 'read_required_number_fact', 'read_array_iterable_form', 'render_string_point_iteration']) {
    assert.ok(emitted.definitions.some(definition => definition.includes('pub fn ' + name + '(')), name);
  }
  for (const fn of functions) {
    const name = fn.name.replace(/([a-z0-9])([A-Z])/gu, '$1_$2').toLowerCase();
    assert.ok(emitted.definitions.some(definition => definition.includes('pub fn ' + name + '(')), fn.name);
  }
  assert.ok(emitted.definitions.length >= functions.length);
  const context = {};
  vm.runInNewContext(emitJavaScript(checked).text.replace(/^export /gmu, ''), context, { timeout: 1000 });
  for (const count of [0, 1, 2, 3]) assert.equal(context.readArrayFromArgumentError(count), readArrayFromArgumentError(count));
  for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) assert.equal(context.renderArrayFromMapping(target, 'values', 'next', 2, '9'), renderArrayFromMapping(target, 'values', 'next', 2, '9'));
});
