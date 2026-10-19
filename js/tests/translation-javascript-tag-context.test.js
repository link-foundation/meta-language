import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readJavaScriptTagOwnerPosition, readJavaScriptTagReceiverForm,
  readJavaScriptTagBindingStep, acceptJavaScriptTagHeaderFact,
  readJavaScriptTagDeclaredOwner } from '../src/translation/javascript-tag-context-rules.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const input = "/** @typedef {{ $: 'malformed' } | { $: 'unsupported', end: number, escapeLength: number } | { $: 'scalar', code: number, end: number }} Input */\n";
const output = "/** @typedef {{ $: 'malformed', at: number, end: number } | { $: 'nonScalar', at: number, end: number } | { $: 'value', code: number }} Output */\n";
const make = "/** @param {number} n @returns {Input} */function readInput(n){if(n===0)return {$:'malformed'};if(n===1)return {$:'unsupported',end:7,escapeLength:6};return {$:'scalar',code:65,end:8};}\n";
const convert = "/** @param {number} n @returns {Output} */function convert(n){const state=readInput(n);if(state.$==='malformed')return {$:'malformed',at:1,end:2};if(state.$==='unsupported')return {$:'nonScalar',at:3,end:state.end};return {$:'value',code:state.code};}\n";
const expected = '[{"$":"malformed","at":1,"end":2},{"$":"nonScalar","at":3,"end":7},{"$":"value","code":65}]';
const observe = (source, expression = '[convert(0),convert(1),convert(2)]') =>
  runInNewContext(source + '\nJSON.stringify(' + expression + ');', {}, { timeout: 1000 });

function refusal(source) {
  let caught;
  try { parseJavaScript(source); } catch (error) { caught = error; }
  assert.ok(caught, 'the complete unchanged source must refuse');
  return { name: caught.name, kind: caught.kind, reason: caught.reason, span: caught.span };
}

test('actual tag receiver owners retain distinct input and returned payloads through both source-free archives', () => {
  const makers = [make, make.replace('function readInput(n){', 'const readInput = function readInput(n){'),
    make.replace('function readInput(n){', 'const readInput = (n)=>{')];
  for (const declarations of [input + output, output + input]) for (const maker of makers) for (const bodies of [maker + convert, convert + maker]) {
    const source = declarations + bodies;
    assert.equal(observe(source), expected, 'the actual original JavaScript is the independent execution oracle');
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    assert.equal(observe(emitJavaScript(checked).text), expected);
    assert.deepEqual(checked.declarations.get('Input').ctors.find(ctor => ctor.name === 'malformed').fields, []);
    assert.deepEqual(checked.declarations.get('Output').ctors.find(ctor => ctor.name === 'malformed').fields.map(field => field.name), ['at', 'end']);
    const rust = emitRust(checked).text;
    const packet = encodeTypedProgramLinks(checked);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const replayed = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.equal(observe(emitJavaScript(replayed).text), expected);
      assert.equal(emitRust(replayed).text, rust);
    }
  }
});

test('declared tag parameters and immutable aliases preserve branch effects and zero argument forward results', () => {
  const source = input + output + make
    + "/** @param {Input} input @returns {Output} */function convertInput(input){const state=input;if(state.$==='malformed'){console.log('first');return {$:'malformed',at:4,end:5};}if(state.$==='unsupported'){console.log('second');return {$:'nonScalar',at:6,end:state.end};}console.log('last');return {$:'value',code:state.code};}\n"
    + "/** @param {number} n @returns {Output} */function convert(n){console.log('argument',n);return convertInput(readInput(n));}\n";
  const effects = text => {
    const lines = [];
    const result = runInNewContext(text + '\nJSON.stringify([convert(0),convert(1),convert(2)]);',
      { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
    return { lines, result };
  };
  const oracle = effects(source);
  assert.deepEqual(oracle.lines, ['argument 0', 'first', 'argument 1', 'second', 'argument 2', 'last']);
  assert.deepEqual(effects(emitJavaScript(checkProgram(parseJavaScript(source, { referenceSemantics: false }))).text), oracle);
  const types = "/** @typedef {{ $: 'shared', input: number } | { $: 'first', input: number }} First */\n"
    + "/** @typedef {{ $: 'shared', output: boolean } | { $: 'second', output: boolean }} Second */\n";
  const maker = "/** @returns {First} */function make(){return {$:'shared',input:3};}\n";
  const inspect = "/** @returns {number} */function inspect(){const one=make();const two=one;const three=two;if(three.$==='shared')return three.input;return three.input;}\n";
  for (const declaration of [maker, maker.replace('function make(){', 'const make=()=>{')]) for (const functions of [declaration + inspect, inspect + declaration]) {
    const source = types + functions;
    assert.equal(observe(source, 'inspect()'), '3');
    assert.equal(observe(emitJavaScript(checkProgram(parseJavaScript(source, { referenceSemantics: false }))).text, 'inspect()'), '3');
  }
});

test('tag context refuses unknown optional asynchronous mutable shadowed namespace and captured owners', () => {
  const unknown = "/** @returns {Output} */function read(state){if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'value',code:0};}";
  const sources = [input + output + unknown,
    input + output + "/** @param {Input} state @returns {Output} */function outer(state){const read=(state)=>{if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'value',code:0};};return read(state);}",
    input + output + make + "/** @param {Input} state @returns {Output} */function read(state){{let state=readInput(0);if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'value',code:0};}}",
    input + output + "/** @param {Input|undefined} state @returns {Output} */function read(state){if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'value',code:0};}",
    input + output + make.replace('@returns {Input}', '@returns {Input|undefined}') + convert,
    input + output + "/** @returns {Promise<Input>} */async function readInput(){return {$:'malformed'};}\n/** @returns {Promise<number>} */async function read(){const state=await readInput();if(state.$==='malformed')return 0;return 1;}",
    input + output + make + make.replace('@returns {Input}', '@returns {Output}').replace("if(n===0)return {$:'malformed'};if(n===1)return {$:'unsupported',end:7,escapeLength:6};return {$:'scalar',code:65,end:8};", "return {$:'malformed',at:1,end:2};") + convert,
    input + output + make + "/** @returns {number} */function read(readInput){const state=readInput();if(state.$==='malformed')return 0;return 1;}",
    input + output + make + "const api={readInput(){return {$:'malformed',at:1,end:2};}};\n" + convert.replace('readInput(n)', 'api.readInput(n)'),
    input + output + "/** @param {Input} state @returns {number} */function read(state){const captured=()=>{if(state.$==='malformed')return 0;return 1;};return captured();}",
  ];
  for (const source of sources) {
    const diagnostic = refusal(source);
    assert.equal(diagnostic.kind, 'type', source);
    assert.equal(diagnostic.reason, 'tag malformed is ambiguous between data types', source);
    const start = source.indexOf("state.$==='malformed'");
    assert.deepEqual(diagnostic.span, { start, end: start + "state.$==='malformed'".length }, source);
  }
  const actualOptional = input + output
    + "/** @param {boolean} present @returns {Input} */function readInput(present){if(present)return {$:'malformed'};}\n"
    + "/** @param {boolean} present @returns {number} */function read(present){const state=readInput(present);if(state.$==='malformed')return 0;return 1;}console.log(read(false));";
  assert.throws(() => runInNewContext(actualOptional, { console: { log() {} } }, { timeout: 1000 }), error => error.name === 'TypeError');
  assert.throws(() => checkProgram(parseJavaScript(actualOptional, { referenceSemantics: false })), error => ['type', 'unsupported'].includes(error.kind),
    'an annotated owner cannot erase actual semantic absence');
});

test('selected tag contexts retain mandatory payload checking and exact unique-owner refusals', () => {
  const ambiguousResult = "/** @param {Input} state */function unknownResult(state){if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'malformed',at:3,end:4};}";
  assert.throws(() => checkProgram(parseJavaScript(input + output + ambiguousResult)), error => ['type', 'unsupported'].includes(error.kind));
  for (const altered of [convert.replace("{$:'malformed',at:1,end:2}", "{$:'malformed'}"),
    convert.replace("{$:'malformed',at:1,end:2}", "{$:'malformed',at:true,end:2}"), convert.replace('end:state.end', 'end:state.escapeLength+state.code')]) {
    assert.throws(() => checkProgram(parseJavaScript(input + output + make + altered)), error => ['type', 'unsupported'].includes(error.kind));
  }
  const unique = input + make
    + "/** @param {Input} state @returns {number} */function read(state){if(state.$==='malformed')return -1;if(state.$==='unsupported')return -2;return state.code+state.end;}\n";
  assert.equal(observe(unique, '[read(readInput(0)),read(readInput(1)),read(readInput(2))]'), '[-1,-2,73]');
  assert.equal(observe(emitJavaScript(checkProgram(parseJavaScript(unique))).text, '[read(readInput(0)),read(readInput(1)),read(readInput(2))]'), '[-1,-2,73]');
  const bad = input + "/** @param {Input} state @returns {boolean} */function read(state){return state.$==='absent';}";
  const start = bad.indexOf("state.$==='absent'");
  assert.deepEqual(refusal(bad), { name: 'TranslationError', kind: 'type', reason: 'no @typedef has tag absent',
    span: { start, end: start + "state.$==='absent'".length } });
});

test('failed speculative tag headers retain the original ambiguity span and ordinary declaration error precedence', () => {
  const types = "/** @typedef {{ $: 'shared', input: number } | { $: 'first', input: number }} First */\n"
    + "/** @typedef {{ $: 'shared', output: boolean } | { $: 'second', output: boolean }} Second */\n";
  const caller = "function inspect(){const state=make();return state.$==='shared';}\n";
  for (const [maker, kind, reason] of [
    ["/** @param {number} @returns {First} */function make(n){return {$:'shared',input:3};}\n", 'syntax', '@param needs a type and a name'],
    ["/** @returns {unknown} */function make(){return {$:'shared',input:3};}\n", 'unsupported', 'JSDoc type {unknown}: portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types'],
  ]) {
    const source = types + caller + maker;
    const start = source.indexOf("state.$==='shared'");
    assert.deepEqual(refusal(source), { name: 'TranslationError', kind: 'type', reason: 'tag shared is ambiguous between data types',
      span: { start, end: start + "state.$==='shared'".length } });
    const commentStart = types.length;
    assert.deepEqual(refusal(types + maker + caller), { name: 'TranslationError', kind, reason,
      span: { start: commentStart, end: commentStart + maker.indexOf('*/') + 2 } });
  }
});

test('shared tag receiver and lexical decisions preserve exact owner identity and nearest unknown shadows', () => {
  for (const [kind, functionKind, path, expected] of [
    ['app', 'name', ['crate', 'make'], 1], ['name', '', ['crate', 'make'], 1], ['name', '', ['state'], 2],
    ['app', 'name', ['make'], 0], ['app', 'functionValue', ['crate', 'make'], 0],
    ['app', 'name', ['crate', 'api', 'make'], 0], ['name', '', ['crate', 'api', 'make'], 0],
    ['name', '', ['other', 'make'], 0], ['method', 'name', ['crate', 'make'], 0], ['name', '', [], 0],
  ]) assert.equal(readJavaScriptTagReceiverForm(kind, functionKind, path, 'crate'), expected);
  for (const boundary of [0, 1, 2]) assert.equal(readJavaScriptTagBindingStep(true, boundary), 1,
    'a nearest binding stops lookup even when its host has no declared fact');
  assert.equal(readJavaScriptTagBindingStep(false, 0), 0);
  for (const boundary of [1, 2, -1, 3]) assert.equal(readJavaScriptTagBindingStep(false, boundary), 2);
  assert.equal(readJavaScriptTagDeclaredOwner('data', [], 'Input', 'crate'), 'Input');
  assert.equal(readJavaScriptTagDeclaredOwner('named', ['crate', 'Input'], '', 'crate'), 'Input');
  for (const [kind, path, name] of [['named', ['other', 'Input'], ''], ['named', ['crate', 'api', 'Input'], ''],
    ['option', ['crate', 'Input'], 'Input'], ['union', ['crate', 'Input'], 'Input'], ['array', ['crate', 'Input'], 'Input'], ['', [], 'Input']]) {
    assert.equal(readJavaScriptTagDeclaredOwner(kind, path, name, 'crate'), '');
  }
  for (const hasType of [false, true]) for (const hasHeader of [false, true]) for (const asynchronous of [false, true]) {
    assert.equal(acceptJavaScriptTagHeaderFact(hasType, hasHeader, asynchronous), !hasType && hasHeader && !asynchronous);
  }
  for (const owner of ['Input', 'Output', 'quoted|owner', 'λOwner']) for (const context of ['', owner, 'unrelated']) {
    assert.equal(readJavaScriptTagOwnerPosition([owner], context), 0);
  }
  assert.equal(readJavaScriptTagOwnerPosition(['Input', 'Output'], 'Input'), 0);
  assert.equal(readJavaScriptTagOwnerPosition(['Input', 'Output'], 'Output'), 1);
  assert.equal(readJavaScriptTagOwnerPosition(['Output', 'Input'], 'Input'), 1);
  for (const context of ['', 'Unknown', 'crate.Input']) assert.equal(readJavaScriptTagOwnerPosition(['Input', 'Output'], context), -1);
  assert.equal(readJavaScriptTagOwnerPosition(['Input', 'Input'], 'Input'), -1);
  assert.equal(readJavaScriptTagOwnerPosition([], 'Input'), -1);
});
