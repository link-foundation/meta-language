import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { tokenize } from '../src/translation/lexer.js';
import { referenceMethodOperation, referenceOperations } from '../src/translation/reference-model.js';
import { hasStructuralIdentity, readSurfaceReferenceMutatingMethods, requiresReferenceSemantics } from '../src/translation/reference-analysis.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createSurfaceReferenceColumns, createSurfaceReferenceFacts } from '../src/translation/surface-reference-facts.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import frontendCases from './fixtures/translation-cases/frontend-javascript.mjs';
import { readSurfaceReferenceSourceModule } from '../scripts/surface-reference-source-module.mjs';
import {readSurfaceReferenceEmissionOperation, readSurfaceReferenceEmissionRefusal} from '../src/translation/surface-reference-analysis-rules.js';

const originalSource = readFileSync(new URL('./support/surface-reference-analysis-original.txt', import.meta.url), 'utf8');
assert.equal(createHash('sha256').update(originalSource).digest('hex'), '14490e98696240c8441f3dfc447e7654deedbc25427fad4d69a7224465692ac4', 'the original complete analysis oracle retains its authenticated bytes');
const original = new Function('tokenize', 'referenceMethodOperation',
  originalSource.replace(/^import[^\n]*\n/gmu, '').replace(/^export /gmu, '')
    + '\nreturn { requiresReferenceSemantics, hasStructuralIdentity };')(tokenize, referenceMethodOperation);

const scalar = { kind: 'float' };
const array = { kind: 'array', element: scalar };
const name = value => ({ k: 'name', path: [value] });
const equality = (left, right) => ({ k: 'binary', op: 'eq', left, right });
const parameter = (value, type) => ({ name: value, type });
const program = (body, params = [], extra = {}) => ({ items: [{ k: 'fn', name: 'probe', params, ret: { kind: 'bool' }, body }], main: { effects: [] }, ...extra });

test('token-window analysis agrees with the authenticated original on source syntax and every linked method alias', () => {
  const sources = [
    'function same(value){return value===1;}',
    'function scale(value){const double=number=>number*2;return double(value);}',
    'function counter(){let count=0;const next=()=>{count+=1;return count;};return next();}',
    'const f=function(){return 1;};let value=0;value++;',
    'function f(){let value=0;value++;return value;}',
    'function f(value){value[0]=1;value.name+=1;return value;}',
    'function f(value){++value.name;--value[0];return value;}',
    'function f(value){value?.class;return value.class;}',
    'class Example {}',
    'const label="class => values.push(0)"; /* class Example {} */',
    'const value={push(){return 1;}};',
  ];
  for (const operation of referenceOperations()) if (operation.method) sources.push('function probe(value){return value.' + operation.method + '();}');
  for (const [, [, source]] of Object.entries(frontendCases).filter(([key]) => key.includes('-accept-')).slice(0, 24)) sources.push(source);
  for (const source of sources) assert.equal(requiresReferenceSemantics(source), original.requiresReferenceSemantics(source), source);
  for (const operation of referenceOperations()) {
    if (!operation.method) continue;
    const selected = ['array', 'map', 'set', 'record'].some(kind => referenceMethodOperation(kind, operation.method)?.mutates);
    assert.equal(readSurfaceReferenceMutatingMethods().includes(operation.method), selected, operation.id);
  }
});

test('actual template tokens and empty EOF operator text retain original reference selection', () => {
  const cases = [
    ['function label(value){return `class => values.push(0) ${value}`;}', false],
    ['function label(){return ``;}', false],
    ['function label(){return `label`.slice(0);}', false],
    ['const next=()=>`label`;let value=0;value++;', true],
    ['function update(value){value[0]=`done`;return value;}', true],
  ];
  for (const [source, expected] of cases) {
    const {tokens} = tokenize(source, 'JavaScript');
    const templates = tokens.filter(token => token.kind === 'template');
    assert.ok(templates.length > 0, 'the actual template token is observed');
    for (const token of templates) assert.equal(Object.hasOwn(token, 'value'), false, 'template text remains structured parts');
    assert.equal(tokens.at(-1).kind, 'eof');
    assert.equal(tokens.at(-1).value, '');
    assert.equal(original.requiresReferenceSemantics(source), expected, source + ': original');
    assert.equal(requiresReferenceSemantics(source), expected, source);
  }
  assert.equal(original.requiresReferenceSemantics(''), false);
  assert.equal(requiresReferenceSemantics(''), false);
  const checked = checkProgram(parseJavaScript('/** @returns {string} */ function label(){return `label`;}'));
  assert.equal(checked.declarations.get('label').ret.kind, 'string');
});

test('flat recursive scope and type flow preserve original structural identity decisions', () => {
  const cases = [
    ['scalar', program(equality(name('value'), name('value')), [parameter('value', scalar)]), false],
    ['array parameter', program(equality(name('value'), name('value')), [parameter('value', array)]), true],
    ['explicit absence', program(equality(name('value'), { k: 'none', absence: 'null' }), [parameter('value', { kind: 'option', element: array })]), false],
    ['tuple value', program(equality(name('value'), name('value')), [parameter('value', { kind: 'tuple', elements: [scalar] })]), false],
    ['lexical shadow', program({ k: 'let', name: 'value', value: { k: 'num', value: '1' }, body: equality(name('value'), name('value')) }, [parameter('value', array)]), false],
    ['inferred local array', program({ k: 'let', name: 'value', value: { k: 'array', items: [] }, body: equality(name('value'), name('value')) }), true],
    ['record field', program(equality({ k: 'field', object: name('value'), field: 'items' }, name('other')), [parameter('value', { kind: 'record', fields: [{ name: 'items', type: array }] }), parameter('other', scalar)]), true],
    ['nested element', program(equality({ k: 'index', object: name('value'), index: { k: 'num', value: '0' } }, name('other')), [parameter('value', { kind: 'array', element: array }), parameter('other', scalar)]), true],
    ['conditional fallback', program(equality({ k: 'if', cond: { k: 'bool', value: true }, then: name('unknown'), else: { k: 'array', items: [] } }, name('other'))), true],
    ['lambda scope', program({ k: 'lambda', params: [parameter('value', array)], body: equality(name('value'), name('value')), ret: { kind: 'bool' } }), true],
    ['ignored provenance', program({ k: 'bool', value: true, span: equality({ k: 'array', items: [] }, { k: 'array', items: [] }), data: equality({ k: 'array', items: [] }, { k: 'array', items: [] }) }), false],
  ];
  for (const kind of ['record', 'set', 'map', 'data']) cases.push([kind, program(equality(name('value'), name('value')), [parameter('value', { kind })]), true]);
  for (const [label, input, expected] of cases) {
    assert.equal(original.hasStructuralIdentity(input), expected, label + ': original');
    assert.equal(hasStructuralIdentity(input), expected, label);
    assert.equal(hasStructuralIdentity(structuredClone(input)), original.hasStructuralIdentity(structuredClone(input)), label + ': clone');
  }
});

test('module signatures external overrides and main binding order keep their original priority', () => {
  const call = { k: 'app', fn: { k: 'name', path: ['crate', 'nested', 'make'] }, args: [] };
  const probe = program(equality(call, call));
  probe.items.push({ k: 'module', name: 'nested', items: [{ k: 'fn', name: 'make', params: [], ret: array, body: { k: 'array', items: [] } }] });
  assert.equal(original.hasStructuralIdentity(probe), true);
  assert.equal(hasStructuralIdentity(probe), true);
  probe.externals = [{ k: 'fn', name: 'nested.make', params: [], ret: scalar }];
  assert.equal(original.hasStructuralIdentity(probe), false);
  assert.equal(hasStructuralIdentity(probe), false);
  probe.externals = [];
  probe.items[1].items.push({ k: 'fn', name: 'make', params: [], ret: scalar, body: { k: 'num', value: '1' } });
  assert.equal(original.hasStructuralIdentity(probe), false);
  assert.equal(hasStructuralIdentity(probe), false);
  const main = { items: [], main: { effects: [
    { k: 'let', name: 'first', value: { k: 'array', items: [] } },
    { k: 'let', name: 'second', value: name('first') },
    { k: 'print', expr: equality(name('first'), name('second')) },
  ] } };
  assert.equal(original.hasStructuralIdentity(main), true);
  assert.equal(hasStructuralIdentity(main), true);
});

test('bounded actual frontend surfaces retain original selection after parsing', () => {
  for (const suffix of ['arithmetic', 'logical-not', 'negation', 'nested-conditional', 'to-string', 'parenthesised', 'tag-test-value', 'asi-return-value']) {
    const source = frontendCases['frontend-javascript-accept-' + suffix][1];
    const surface = parseJavaScript(source);
    assert.equal(hasStructuralIdentity(surface), original.hasStructuralIdentity(surface), suffix);
  }
  for (const source of [
    'function same(left,right){return left===right;} console.log(same([1],[1]));',
    'function same(){const first={value:1};const second=first;return first===second;}',
  ]) {
    const surface = parseJavaScript(source);
    assert.equal(surface.referenceSemantics, true);
    assert.equal(hasStructuralIdentity(surface), original.hasStructuralIdentity(surface));
  }
});

test('actual checked nested references retain allocation read and source-free JavaScript observations', () => {
  const source = '/** @param {number[]} values @returns {number[][]} */ function nested(values){return [values];}'
    + '/** @param {number[][]} values @returns {number[]} */ function first(values){return values[0];}';
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: true }));
  assert.equal(checked.declarations.get('nested').ret.reference, true);
  assert.equal(checked.declarations.get('nested').ret.element.reference, true);
  assert.equal(checked.declarations.get('nested').body.k, 'referenceAllocate');
  assert.equal(checked.declarations.get('first').body.array.k, 'referenceRead');
  const packet = createTypedProgramLinks(source, 'JavaScript', { context: { referenceSemantics: true } });
  const restored = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  const actual = {};
  runInNewContext(emitJavaScript(restored).text, actual, { timeout: 1000 });
  const values = [1, 2];
  const nested = actual.nested(values);
  assert.equal(actual.first(nested), values);
  assert.equal(nested[0], values);
  values.push(3);
  assert.deepEqual(Array.from(actual.first(nested)), [1, 2, 3]);
});

test('selected allocation and member reads survive complete checked surface rechecking', () => {
  const source = '/** @param {number[]} values @returns {number[][]} */ function nested(values){return [values];}'
    + '/** @param {number[]} values @returns {number} */ function size(values){return values.length;}';
  const packet = createTypedProgramLinks(source, 'JavaScript', { context: { referenceSemantics: true } });
  const restored = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  assert.equal(restored.declarations.get('nested').body.k, 'referenceAllocate');
  assert.equal(restored.declarations.get('size').body.array.k, 'referenceRead');
  assert.equal(Object.hasOwn(restored.declarations.get('size').body.array.type, 'reference'), true);
  assert.equal(restored.declarations.get('size').body.array.type.reference, false);
  assert.strictEqual(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
  const missing = structuredClone(restored);
  delete missing.declarations.get('size').body.array.type.reference;
  assert.throws(() => recheckLinkedTypedProgram(missing, {wholeProgram: true}), /inconsistent types, bindings or operation semantics/u);
});

test('the entire reference rule module checks and emits every declaration with executable original-oracle controls', () => {
  const source = readFileSync(new URL('../src/translation/surface-reference-analysis-rules.js', import.meta.url), 'utf8');
  const names = [...source.matchAll(/^(?:export )?function (\w+)\(/gmu)].map(match => match[1]);
  const priorNames = names.slice(0, -1);
  assert.equal(priorNames.slice(0, -2).length, 21, 'all prior reference declarations remain before the two appended emission guards');
  assert.equal(priorNames.length, 23);
  assert.deepEqual(priorNames.slice(-2), ['readSurfaceReferenceEmissionOperation', 'readSurfaceReferenceEmissionRefusal']);
  assert.deepEqual(names.slice(-1), ['requireSurfaceReferenceTokenAfterElimination']);
  assert.equal(names.length, 24);
  const closed = readSurfaceReferenceSourceModule(source);
  const allNames = [...closed.matchAll(/^(?:export )?function (\w+)\(/gmu)].map(match => match[1]);
  assert.equal(allNames.slice(0, -1).length, 29, 'all23 reference functions and the complete actual6 required-column functions are retained');
  assert.equal(allNames.length, 30);
  const surface = parseJavaScript(closed, {referenceSemantics: false});
  assert.equal(Object.hasOwn(surface, 'referenceSemantics'), false);
  const checked = checkProgram(surface);
  for (const name of allNames) assert.equal(checked.declarations.get(name)?.k, 'fn', name);
  const rust = emitRust(checked, { arrayConsumption: true });
  assert.ok(rust.definitions.length >= allNames.length, 'all original declarations and actual lowered continuations are emitted');
  for (const name of allNames) {
    const nativeName = name.replace(/([a-z])([A-Z])/gu, '$1_$2').toLowerCase();
    assert.ok(rust.definitions.some(definition => definition.includes('pub fn ' + nativeName + '(')), name);
  }
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  for (const input of [
    program(equality(name('value'), name('value')), [parameter('value', scalar)]),
    program(equality(name('value'), name('value')), [parameter('value', array)]),
    program(equality(name('value'), {k: 'none', absence: 'null'}), [parameter('value', {kind: 'option', element: array})]),
  ]) {
    const facts = createSurfaceReferenceFacts(input), packed = createSurfaceReferenceColumns(facts);
    assert.equal(context.hasSurfaceStructuralIdentity(packed.columns, packed.rows, packed.size, facts.root), original.hasStructuralIdentity(input));
  }
  assert.equal(context.requireSurfaceReferenceToken(['identifier', 'class', '', '', '', 'identifier', 'Example', '{'], false, false), original.requiresReferenceSemantics('class Example {}'));
  assert.equal(context.renderSurfaceReferenceOperation('JavaScript', 'referenceRead', 'owner', ''), 'owner');
  for (const [kind, receiver, allocation] of [['length', true, false], ['array', false, true], ['referenceRead', true, false], ['referenceAllocate', false, true]]) {
    const operation = readSurfaceReferenceEmissionOperation(kind, receiver, allocation);
    assert.equal(context.readSurfaceReferenceEmissionOperation(kind, receiver, allocation), operation);
    assert.equal(context.readSurfaceReferenceEmissionRefusal(operation), readSurfaceReferenceEmissionRefusal(operation));
  }
  const packet = encodeTypedProgramLinks(checked), javascript = emitJavaScript(checked);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram: true});
    assert.deepEqual([...restored.declarations.keys()], [...checked.declarations.keys()]);
    const replay = emitRust(restored, {arrayConsumption: true});
    for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(replay[field], rust[field]);
    assert.deepEqual(emitJavaScript(restored), javascript);
    const context = {};
    runInNewContext(javascript.text, context, {timeout: 1000});
    const window = ['punct', '=', 'punct', ']', 'position', 'identifier', 'value', ';'];
    assert.equal(context.requireSurfaceReferenceToken(window, false, false), true);
    assert.equal(context.requireSurfaceReferenceTokenAfterElimination(window, false, false, false), true);
    assert.equal(context.requireSurfaceReferenceTokenAfterElimination(window, false, false, true), false);
  }
});

test('forged checked receiver and allocation flags retain the exact prior Rust emission refusals', () => {
  const checked = checkProgram(parseJavaScript('/** @param {number[]} values @returns {number} */ function size(values){return values.length;} function numbers(){return [1];}'));
  const receiver = structuredClone(checked);
  receiver.declarations.get('size').body.array.type.reference = true;
  assert.throws(() => emitRust(receiver), error => error.kind === 'unsupported'
    && error.reason === 'reference operation: this operation needs the shared reference and slot runtime before target emission');
  const allocation = structuredClone(checked);
  allocation.declarations.get('numbers').body.type.reference = true;
  assert.throws(() => emitRust(allocation), error => error.kind === 'unsupported'
    && error.reason === 'reference allocation: this allocation needs the shared reference and slot runtime before target emission');
  for (const kind of ['index', 'optionalIndex', 'length', 'recordField', 'tupleIndex', 'method', 'collectionSize', 'collectionRead', 'collectionItems']) {
    assert.equal(readSurfaceReferenceEmissionOperation(kind, true, false), 'reference operation');
    assert.equal(readSurfaceReferenceEmissionOperation(kind, false, false), '');
  }
  for (const kind of ['array', 'record', 'map', 'set', 'append']) assert.equal(readSurfaceReferenceEmissionOperation(kind, false, true), 'reference allocation');
  assert.equal(readSurfaceReferenceEmissionOperation('referenceRead', true, false), '');
  assert.equal(readSurfaceReferenceEmissionOperation('referenceAllocate', false, true), '');
});

test('default reference selection excludes only actual eliminated private append operations', () => {
  const untyped = '/** @returns {number[]} */ function build(){const values=[];values.push(2);return values;}';
  assert.throws(() => checkProgram(parseJavaScript(untyped, {referenceSemantics: false})), {
    message: 'empty array of an unknown type: nothing fixes the type of the elements of this empty array; declare it with JSDoc at 57..59',
  }, 'the original empty literal retains its preexisting element-inference boundary');
  const source = untyped.replace('const values=[];', 'const values=[1];');
  const surface = parseJavaScript(source);
  assert.equal(Object.hasOwn(surface, 'referenceSemantics'), false);
  const checked = checkProgram(surface);
  assert.equal(checked.referenceSemantics, undefined);
  assert.ok(emitRust(checked).text.includes('crate::ml_array::append('));
  const packet = createTypedProgramLinks(source, 'JavaScript');
  const replay = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  assert.strictEqual(recheckLinkedTypedProgram(replay, {wholeProgram: true}), replay);
  const context = {};
  runInNewContext(emitJavaScript(replay).text, context, {timeout: 1000});
  assert.deepEqual(Array.from(context.build()), [1, 2]);
  assert.equal(parseJavaScript(source, {referenceSemantics: true}).referenceSemantics, true);

  const operation = {start: source.indexOf('.push'), end: source.indexOf('(2)') + 1};
  assert.equal(requiresReferenceSemantics(source), original.requiresReferenceSemantics(source));
  assert.equal(requiresReferenceSemantics(source, [operation]), false);
  for (const suffix of [
    'class Stateful {}',
    'function mutate(other){other[0]=1;return other;}',
    'function mutate(other){other.push(3);return other;}',
    'let count=0;const next=()=>{count+=1;return count;};',
  ]) assert.equal(requiresReferenceSemantics(source + suffix, [operation]), true, suffix);

  const nested = source.replace('values.push(2)', 'values.push(other.push(2))');
  const beforeArgument = {start: nested.indexOf('.push'), end: nested.indexOf('other.push')};
  assert.equal(requiresReferenceSemantics(nested, [beforeArgument]), true, 'a nested argument mutation is retained');
  const equality = source.replace('@returns {number[]}', '@returns {boolean}')
    .replace('return values;', 'const alias=values;return values===alias;');
  assert.equal(parseJavaScript(equality).referenceSemantics, true, 'late actual structural identity selection is retained');
});
