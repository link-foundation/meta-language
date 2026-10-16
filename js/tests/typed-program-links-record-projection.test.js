import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { format } from 'node:util';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, readTypedProgramBindingIdentities } from '../src/typed-program-links.js';
import { readRecordProjection } from '../src/translation/record-value-rules.js';

function records(program, kind) {
  const seen = new WeakSet(), found = [];
  function visit(value) {
    if (!value || typeof value !== 'object' || seen.has(value)) return;
    seen.add(value);
    if (value.k === kind) found.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) visit(child);
  }
  visit(program);
  return found;
}

function observe(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: (...args) => output.push(format(...args)) } }, { timeout: 1000 });
  return output;
}

function archiveForms(packet) {
  return [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
}

function bindingNames(network, root) {
  return readTypedProgramBindingIdentities(network, root).map(owner => {
    const name = network.fieldRelations().find(row => row.parent.equals(owner) && row.label === 'name');
    assert.ok(name);
    return network.link(name.child).metadata().definition;
  }).sort();
}

const mark = '/** @param {string} label @param {bigint} value @returns {bigint} */function mark(label,value){console.log("%s",label);return value;}';
const sources = [
  { source: 'function empty(){return {};}const value=empty();', expected: [] },
  { source: mark + 'function make(){return {z:mark("first",1n),a:mark("second",2n),z:mark("last",3n)};}const value=make();console.log("%s",String(value.a));console.log("%s",String(value.z));', expected: ['first', 'second', 'last', '2', '3'] },
  { source: mark + 'function make(){const base={b:mark("base-b",2n),a:mark("base-a",1n)};return {...base,b:mark("override-b",4n)};}const value=make();console.log("%s",String(value.a));console.log("%s",String(value.b));', expected: ['base-b', 'base-a', 'override-b', '1', '4'] },
  { source: 'function make(){return {z:{value:7n},a:2n};}const value=make();console.log("%s",String(value.z.value));', expected: ['7'] },
];

test('whole checked record projection is idempotent without introducing new binding owners', () => {
  for (const { source, expected } of sources) {
    const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    const original = structuredClone(program);
    assert.ok(records(program, 'record').length > 0);
    assert.deepEqual(observe(source), expected);
    for (let repeat = 0; repeat < 3; repeat += 1) {
      assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
      assert.deepEqual(program, original);
    }
    const packet = encodeTypedProgramLinks(program);
    const names = bindingNames(packet.network, packet.root);
    for (const network of archiveForms(packet)) {
      const restored = decodeTypedProgramLinks(network, packet.root);
      assert.equal(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
      assert.deepEqual(restored, program);
      assert.deepEqual(bindingNames(network, packet.root), names);
      assert.deepEqual(observe(emitJavaScript(restored).text), expected);
    }
  }
});

test('whole record rechecking retains source-free value edits field order and actual binding identities', () => {
  const source = 'function make(){return {z:9n,a:2n};}const value=make();console.log("%s",String(value.a));console.log("%s",String(value.z));';
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const packet = encodeTypedProgramLinks(program);
  const originalNames = bindingNames(packet.network, packet.root);
  assert.deepEqual(observe(source), ['2', '9']);
  for (const network of archiveForms(packet)) {
    const literal = network.fieldRelations().find(row => row.label === 'value' && network.link(row.child).metadata().definition === '9');
    assert.ok(literal);
    const link = network.link(literal.child);
    link.setMetadata(link.metadata().withDefinition('17'));
    const edited = decodeTypedProgramLinks(network, packet.root);
    assert.equal(recheckLinkedTypedProgram(edited, { wholeProgram: true }), edited);
    assert.deepEqual(bindingNames(network, packet.root), originalNames);
    assert.deepEqual(records(edited, 'record').map(record => record.entries.map(entry => entry.name)), [['a', 'z']]);
    assert.equal(edited.items[0], edited.declarations.get('make'));
    const saved = encodeTypedProgramLinks(edited);
    for (const reloaded of archiveForms(saved)) {
      const restored = decodeTypedProgramLinks(reloaded, saved.root);
      assert.equal(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
      assert.deepEqual(bindingNames(reloaded, saved.root), originalNames);
      assert.deepEqual(observe(emitJavaScript(restored).text), ['2', '17']);
    }
  }
});

test('canonical record projections reject malformed layouts types and lexical targets', () => {
  const source = 'function make(){return {z:9n,a:2n};}';
  const make = () => checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const canonical = projectLinkedProgramExpression(records(make(), 'record')[0]);
  assert.equal(canonical.recordProjection, readRecordProjection());
  assert.deepEqual(canonical.entries.map(entry => entry.name), ['a', 'z']);
  assert.ok(canonical.entries.every(entry => entry.spread === false));
  for (const mutate of [
    record => record.entries.reverse(),
    record => record.entries.push(record.entries[0]),
    record => { record.entries[0].spread = true; },
    record => { record.entries[0].extra = 'discarded'; },
    record => { record.entries[0].name = 1; },
    record => { record.extra = 'discarded'; },
    record => { record.type.fields[0].type = { kind: 'bool' }; },
    record => { record.entries[0].value.type = { kind: 'bool' }; },
    record => { record.entries[0].value.name = 'unbound'; },
  ]) {
    const program = make();
    mutate(records(program, 'record')[0]);
    assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }), /record projection|record expression|unknown name|expected|inconsistent types, bindings or operation semantics/u);
  }
});

test('canonical record markers still check every child while raw records keep source-order lowering', () => {
  const integer = { kind: 'int' };
  const type = { kind: 'record', fields: [{ name: 'a', type: integer }] };
  const entry = { name: 'a', spread: false, value: { k: 'num', value: '5', type: integer } };
  const surface = body => ({ language: 'JavaScript', items: [{ k: 'fn', name: 'make', params: [], ret: type, body }], main: null });
  const canonical = { k: 'record', recordProjection: readRecordProjection(), entries: [entry] };
  const checked = checkProgram(surface(canonical));
  assert.equal(checked.declarations.get('make').body.k, 'record');
  assert.equal(checked.declarations.get('make').body.entries[0].value.value, '5');
  assert.equal(recheckLinkedTypedProgram(checked, { wholeProgram: true }), checked);
  const raw = checkProgram(surface({ k: 'record', entries: [entry] }));
  assert.equal(raw.declarations.get('make').body.k, 'let');
  assert.equal(raw.declarations.get('make').body.body.k, 'record');
  for (const mutate of [
    node => { node.recordProjection = true; },
    node => { node.recordProjection = 'source record'; },
    node => { node.entries[0].spread = true; },
    node => { delete node.entries[0].spread; },
    node => { node.entries[0].value = { k: 'bool', value: true }; },
    node => { node.entries[0].value = { k: 'name', path: ['unbound'] }; },
  ]) {
    const node = structuredClone(canonical);
    mutate(node);
    assert.throws(() => checkProgram(surface(node)), /record projection|expected|function make returns|unknown name/u);
  }
});
