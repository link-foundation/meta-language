import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { typeSyntaxRulesPolicy, readTypeSyntaxSourceModule } from '../scripts/generate-type-syntax-rules.mjs';
import { splitType } from '../src/translation/javascript-bindings.js';
import { readTypeSeparatorPositions } from '../src/translation/type-syntax-rules.js';
import { typeKey, sameType } from '../src/translation/types.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const observe = source => {
  const lines = [];
  vm.runInNewContext(source, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 });
  return lines;
};

test('all four actual type syntax functions and six required-column helpers survive automatic checked translation', () => {
  const names = ['readTypeSeparatorPositions', 'acceptUnionTypeField', 'acceptUnionTypeCount', 'readUnionTargetRefusal'];
  const helpers = ['readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact', 'refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact'];
  const surface = parseJavaScript(readTypeSyntaxSourceModule(), { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name).sort(), [...names, ...helpers].sort());
  const checked = checkProgram(surface);
  const packet = encodeTypedProgramLinks(checked);
  const expectedRust = emitRust(checked).text;
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const program = decodeTypedProgramLinks(network, packet.root);
    const rust = emitRust(program);
    assert.equal(rust.text, expectedRust);
    assert.ok(rust.definitions.length >= names.length + helpers.length);
    for (const definition of rust.definitions) assert.ok(rust.text.includes(definition));
    const api = vm.runInNewContext(emitJavaScript(program).text + '\n({' + names.join(',') + '});', {}, { timeout: 1000 });
    assert.deepEqual(Array.from(api.readTypeSeparatorPositions(units('() => string | bigint'), 124)), [13]);
    assert.equal(api.acceptUnionTypeField('alternatives'), true);
    assert.equal(api.acceptUnionTypeField('borrowEscape'), false);
    assert.equal(api.acceptUnionTypeCount(2), true);
    assert.equal(api.acceptUnionTypeCount(1), false);
    assert.equal(api.readUnionTargetRefusal('Rust'), 'no Rust type for union');
    assert.equal(api.readUnionTargetRefusal('JavaScript'), '');
  }
});

test('portable separators preserve actual nesting arrow and UTF16 positions against the entire original scanner', () => {
  assert.deepEqual(readTypeSeparatorPositions(units('a|b'), 124), [1]);
  assert.deepEqual(readTypeSeparatorPositions(units('(a|b)|c'), 124), [5]);
  assert.deepEqual(readTypeSeparatorPositions(units('() => string | bigint'), 124), [13]);
  assert.deepEqual(readTypeSeparatorPositions(units('\u{10000}|string'), 124), [2]);
  const policy = readProjectData(new URL('../src/translation/data/type-syntax.lino', import.meta.url));
  const old = vm.runInNewContext(policy.original.source.replace('export function ', 'function ') + '\nsplitType;');
  const sources = ['', 'bigint | string', '[bigint,string]|number', '{a:bigint|number,b:[string,number]}|boolean', '(a:bigint,b:string)=>number|boolean', 'function([bigint,string]):(number|boolean)', '(bigint|string)[]', '\u{10000}|string', 'a)>|b', 'a<|b'];
  for (const source of sources) for (const separator of ['|', ',', ':', '', '||', '\u{10000}', null, undefined]) {
    assert.deepEqual(splitType(source, separator), Array.from(old(source, separator)), `${source}/${String(separator)}`);
  }
  assert.deepEqual(typeSyntaxRulesPolicy(), policy);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/type-syntax.lino', import.meta.url)), policy);
});

test('union identities preserve every ordered alternative rather than collapsing distinct union records', () => {
  const integer = { kind: 'int' }, string = { kind: 'string' }, boolean = { kind: 'bool' };
  const first = { kind: 'union', alternatives: [integer, string] };
  assert.equal(typeKey(first), 'union:[int,string]');
  assert.equal(sameType(first, { kind: 'union', alternatives: [integer, boolean] }), false);
  assert.equal(sameType(first, { kind: 'union', alternatives: [string, integer] }), false);
  assert.equal(typeKey({ kind: 'union', alternatives: [integer, integer, string] }), 'union:[int,int,string]');
  assert.equal(first.alternatives[0], integer);
  assert.equal(first.alternatives[1], string);
});

test('unused union parameters retain source-free ordered type alternatives and exact unsupported target boundaries', () => {
  const source = '/** @param {bigint | string} value @returns {bigint} */function ignore(value){return 1n;}console.log("ready");';
  assert.deepEqual(observe(source), ['ready']);
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const program = decodeTypedProgramLinks(network, packet.root);
    assert.deepEqual(program.declarations.get('ignore').params[0].type, { kind: 'union', alternatives: [{ kind: 'int' }, { kind: 'string' }] });
    assert.deepEqual(observe(emitJavaScript(program).text), ['ready']);
    for (const [target, emit] of [['Rust', emitRust], ['Lean', emitLean], ['Rocq', emitRocq]]) {
      assert.throws(() => emit(program), error => error.message === `no ${target} type for union`);
    }
    program.declarations.get('ignore').params[0].type.alternatives[1] = { kind: 'bool' };
    const edited = encodeTypedProgramLinks(program);
    const reloaded = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited.network)), edited.root);
    assert.equal(typeKey(reloaded.declarations.get('ignore').params[0].type), 'union:[int,bool]');
    assert.deepEqual(observe(emitJavaScript(reloaded).text), ['ready']);
  }
});

test('union header refusal retains its actual ordering before an independently unsupported frozen body', () => {
  const source = '/** @typedef {{ $: "one" } | { $: "two" }} Tag */\n/** @param {bigint | string} unused @returns {Tag} */function make(unused){return Object.freeze({$:"one"});}console.log("ready");';
  assert.deepEqual(observe(source), ['ready']);
  const saved = createTypedProgramLinks(source, 'JavaScript');
  for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
    const program = decodeTypedProgramLinks(network, saved.root);
    assert.deepEqual(observe(emitJavaScript(program).text), ['ready']);
    for (const [target, emit] of [['Rust', emitRust], ['Lean', emitLean], ['Rocq', emitRocq]]) {
      assert.throws(() => emit(program), error => error.message === `no ${target} type for union`);
    }
  }
  const withoutUnion = createTypedProgramLinks(source.replace('@param {bigint | string}', '@param {bigint}'), 'JavaScript');
  const checked = decodeTypedProgramLinks(withoutUnion.network, withoutUnion.root);
  for (const emit of [emitRust, emitLean, emitRocq]) {
    assert.throws(() => emit(checked), /explicit data object freeze/u);
  }
});

test('linked union alternatives reject malformed layouts without granting a union argument coercion', () => {
  const source = '/** @param {bigint | string} value @returns {bigint} */function ignore(value){return 1n;}console.log("ready");';
  for (const alternatives of [[], [{ kind: 'int' }], null, {}, [undefined, { kind: 'int' }], new Array(2), [{ kind: 'foreign' }, { kind: 'int' }]]) {
    const packet = createTypedProgramLinks(source, 'JavaScript');
    const program = decodeTypedProgramLinks(packet.network, packet.root);
    program.declarations.get('ignore').params[0].type.alternatives = alternatives;
    assert.throws(() => encodeTypedProgramLinks(program), /malformed ordered alternative layout|malformed type|unsupported type foreign/u);
  }
  for (const value of ['1n', '"text"']) {
    const called = source.replace('console.log("ready");', `console.log("%s",String(ignore(${value})));`);
    assert.throws(() => createTypedProgramLinks(called, 'JavaScript'), error => error.kind === 'type' && error.message.includes('union'));
  }
});
