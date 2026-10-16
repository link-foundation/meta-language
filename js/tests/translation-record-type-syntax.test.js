import test from 'node:test';
import assert from 'node:assert/strict';
import { format } from 'node:util';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordTypeSyntaxRulesPolicy, readRecordTypeSyntaxSourceModule } from '../scripts/generate-record-type-syntax-rules.mjs';
import { splitType, readCompositeType } from '../src/translation/javascript-bindings.js';
import { unsupported } from '../src/translation/diagnostics.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';

const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const observe = source => {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines;
};

test('record type field projection preserves the complete original composite reader and exact child parse order', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-type-syntax.lino', import.meta.url));
  assert.deepEqual(recordTypeSyntaxRulesPolicy(), policy);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/record-type-syntax.lino', import.meta.url)), policy);
  const original = vm.runInNewContext(policy.original.source.replace('export function ', 'function ') + '\nreadCompositeType;', { splitType, unsupported });
  const range = { start: 3, end: 29 };
  for (const name of ['{}', '{a:bigint,z:string}', '{ z : [bigint,string] , a?:number }', '{a?:number,a:bigint}',
    '{\u{10000}:number,\uE000:string}', '{\u00A0x?\u00A0:number}', '{outer:{child:number},next:(x:bigint)=>string}',
    '{ :number}', '{a:number;b:string}', '{missing}', '{a:number,missing}', '{a:}', '{,a:number,,}']) {
    const evaluate = reader => {
      const calls = [];
      const parser = { type(source, span) { calls.push([source, span]); return { kind: 'probe', source: source.trim() }; } };
      try { return JSON.parse(JSON.stringify({ value: reader(parser, name, range), calls })); }
      catch (error) { return JSON.parse(JSON.stringify({ kind: error.kind, reason: error.reason, span: error.span, calls })); }
    };
    assert.deepEqual(evaluate(readCompositeType), evaluate(original), name);
  }
});

test('all three actual record type decisions and six required-column definitions translate through checked Links', () => {
  const names = ['acceptRecordTypeBoundary', 'readRecordTypeFieldColon', 'acceptRecordTypeOptionalName'];
  const helpers = ['readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact', 'refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact'];
  const surface = parseJavaScript(readRecordTypeSyntaxSourceModule(), { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name).sort(), [...names, ...helpers].sort());
  const checked = checkProgram(surface);
  const saved = encodeTypedProgramLinks(checked);
  const originalRust = emitRust(checked).text;
  for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
    const reloaded = decodeTypedProgramLinks(network, saved.root);
    const rust = emitRust(reloaded);
    assert.equal(rust.text, originalRust);
    assert.ok(rust.definitions.length >= names.length + helpers.length);
    for (const definition of rust.definitions) assert.ok(rust.text.includes(definition));
    const api = vm.runInNewContext(emitJavaScript(reloaded).text + '\n({' + names.join(',') + '});', {}, { timeout: 1000 });
    assert.equal(api.acceptRecordTypeBoundary(123, 125), true);
    assert.equal(api.acceptRecordTypeBoundary(123, 93), false);
    assert.equal(api.readRecordTypeFieldColon(units('outer:{child:number}')), 5);
    assert.equal(api.readRecordTypeFieldColon(units('missing')), -1);
    assert.equal(api.acceptRecordTypeOptionalName(63), true);
    assert.equal(api.acceptRecordTypeOptionalName(NaN), false);
  }
});

test('declared record parameters retain canonical field identities and source-free executable value edits in all targets', () => {
  const source = '/** @param {{z:bigint,a:bigint}} value @returns {bigint} */function select(value){return value.z;}function choose(){return select({a:2n,z:3n});}console.log("%s",String(choose()));';
  assert.deepEqual(observe(source), ['3']);
  const saved = createTypedProgramLinks(source, 'JavaScript');
  const original = decodeTypedProgramLinks(saved.network, saved.root);
  const emitters = [emitJavaScript, emitRust, emitLean, emitRocq];
  const expected = emitters.map(emit => emit(original).text);
  const param = original.declarations.get('select').params[0].type;
  assert.equal(param.kind, 'record');
  assert.deepEqual(param.fields.map(field => [field.name, field.type.kind]), [['a', 'int'], ['z', 'int']]);
  for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
    const reloaded = decodeTypedProgramLinks(network, saved.root);
    emitters.forEach((emit, index) => assert.equal(emit(reloaded).text, expected[index]));
    assert.deepEqual(observe(emitJavaScript(reloaded).text), ['3']);
  }
  const seen = new WeakSet(), literals = [];
  const walk = value => {
    if (!value || typeof value !== 'object' || seen.has(value)) return;
    seen.add(value);
    if (value.k === 'lit' && value.type.kind === 'int' && value.value === '3') literals.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) walk(child);
  };
  walk(original);
  assert.equal(literals.length, 1);
  literals[0].value = '7';
  const edited = encodeTypedProgramLinks(original);
  const reloaded = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited.network)), edited.root);
  assert.deepEqual(observe(emitJavaScript(reloaded).text), ['7']);
  emitters.forEach((emit, index) => assert.notEqual(emit(reloaded).text, expected[index]));
});
