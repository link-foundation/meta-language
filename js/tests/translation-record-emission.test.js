import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordEmissionRulesPolicy, readRecordEmissionSourceModule } from '../scripts/generate-record-emission-rules.mjs';
import { renderRecordField, renderRecordProductField, renderRecordType, renderRecordValue } from '../src/translation/record-emission-rules.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

test('the complete five-function renderer and six required-column definitions translate through checked Links', () => {
  const names = ['renderRecordValue', 'renderRecordField', 'renderRecordProductField', 'renderRecordType'];
  const dependencyNames = ['readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact', 'refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact'];
  const surface = parseJavaScript(readRecordEmissionSourceModule(), { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name).sort(), [...names, 'joinRecordItems', ...dependencyNames].sort());
  const checked = checkProgram(surface);
  const packet = encodeTypedProgramLinks(checked);
  const expectedRust = emitRust(checked, { arrayConsumption: true }).text;
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const program = decodeTypedProgramLinks(network, packet.root);
    const rust = emitRust(program, { arrayConsumption: true });
    assert.equal(rust.text, expectedRust);
    assert.ok(rust.definitions.length >= names.length + 1 + dependencyNames.length);
    for (const definition of rust.definitions) assert.ok(rust.text.includes(definition));
    const api = vm.runInNewContext(emitJavaScript(program).text + '\n({' + names.join(',') + '});', {}, { timeout: 1000 });
    assert.equal(api.renderRecordValue('JavaScript', ['"a"', '"z"'], ['left', 'right']), '({["a"]: left, ["z"]: right})');
    assert.equal(api.renderRecordValue('Rust', ['"a"'], ['value']), '(value,)');
    assert.equal(api.renderRecordValue('Lean', [], []), '()');
    assert.equal(api.renderRecordValue('Rocq', [], []), 'tt');
    assert.equal(api.renderRecordField('Rust', 'value', '', '2'), '(value).2');
    assert.equal(api.renderRecordProductField('Lean', 'value', 2, 4), '(value).2.2.1');
    assert.equal(api.renderRecordProductField('Rocq', 'value', 2, 4), '(snd (fst value))');
    assert.equal(api.renderRecordType('Lean', ['Int', 'String']), '(Int × String)');
    assert.equal(api.renderRecordType('Rocq', ['Z', 'string']), '(Z * string)%type');
    assert.throws(() => api.renderRecordValue('JavaScript', [], ['value']), /invalid required text fact column position/u);
  }
});

test('record syntax preserves empty singleton and ordered values in every existing target representation', () => {
  const cases = [
    { labels: [], values: [], expected: ['({})', '()', '()', 'tt'] },
    { labels: ['"only"'], values: ['value'], expected: ['({["only"]: value})', '(value,)', '(value)', '(value)'] },
    { labels: ['"a"', '"z"'], values: ['left', 'right'], expected: ['({["a"]: left, ["z"]: right})', '(left, right)', '(left, right)', '(left, right)'] },
  ];
  const targets = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
  for (const entry of cases) targets.forEach((target, index) => assert.equal(renderRecordValue(target, entry.labels, entry.values), entry.expected[index]));
  assert.equal(renderRecordType('Lean', []), 'Unit');
  assert.equal(renderRecordType('Rocq', []), 'unit');
  assert.equal(renderRecordType('Lean', ['Int']), '(Int)');
  assert.equal(renderRecordType('Rocq', ['Z']), '(Z)%type');
  assert.equal(renderRecordType('Lean', ['Int', 'String']), '(Int × String)');
  assert.equal(renderRecordType('Rocq', ['Z', 'string']), '(Z * string)%type');
});

test('record field projections retain exact product association and source property labels', () => {
  assert.equal(renderRecordField('JavaScript', 'value', '"a\\b"', ''), '(value)["a\\b"]');
  assert.equal(renderRecordField('Rust', 'value', '', '2'), '(value).2');
  assert.equal(renderRecordProductField('Lean', 'value', 0, 1), 'value');
  assert.equal(renderRecordProductField('Rocq', 'value', 0, 1), 'value');
  const lean = ['(value).1', '(value).2.1', '(value).2.2.1', '(value).2.2.2'];
  const rocq = ['(fst (fst (fst value)))', '(snd (fst (fst value)))', '(snd (fst value))', '(snd value)'];
  for (let index = 0; index < 4; index += 1) {
    assert.equal(renderRecordProductField('Lean', 'value', index, 4), lean[index]);
    assert.equal(renderRecordProductField('Rocq', 'value', index, 4), rocq[index]);
  }
});

test('record emitters retain complete source-free target output through text binary and value edits', () => {
  const source = 'function choose(){const value={z:1n,a:2n};return value.z;}console.log("%s",String(choose()));';
  const packet = createTypedProgramLinks(source, 'JavaScript');
  const original = decodeTypedProgramLinks(packet.network, packet.root);
  const emitters = [emitJavaScript, emitRust, emitLean, emitRocq];
  const expected = emitters.map(emit => emit(original).text);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const reloaded = decodeTypedProgramLinks(network, packet.root);
    emitters.forEach((emit, index) => assert.equal(emit(reloaded).text, expected[index]));
  }
  const visited = new WeakSet();
  const literals = [];
  const visit = value => {
    if (!value || typeof value !== 'object' || visited.has(value)) return;
    visited.add(value);
    if (value.k === 'lit' && value.type.kind === 'int' && value.value === '1') literals.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) visit(child);
  };
  visit(original);
  assert.equal(literals.length, 1);
  literals[0].value = '7';
  const edited = encodeTypedProgramLinks(original);
  const reloaded = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited.network)), edited.root);
  emitters.forEach((emit, index) => {
    const text = emit(reloaded).text;
    assert.notEqual(text, expected[index]);
    assert.equal(text, emit(original).text);
  });
});

test('both packages share the entire actual record emission source', () => {
  const policy = recordEmissionRulesPolicy();
  assert.deepEqual(readProjectData(new URL('../src/translation/data/record-emission.lino', import.meta.url)), policy);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/record-emission.lino', import.meta.url)), policy);
});
