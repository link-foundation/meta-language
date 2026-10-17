import assert from 'node:assert/strict';
import vm from 'node:vm';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { typedProgramLinksFixture } from '../scripts/generate-typed-program-links-fixtures.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { emitTypedProgramLinks } from '../src/typed-program-links.js';
import { decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { checkProgram } from '../src/translation/check.js';

const fixture = readProjectData(new URL('../../parity/fixtures/typed-program-links.json', import.meta.url));
test('bounded typed graph fixtures reload without source and preserve behavior', () => {
  assert.deepEqual(typedProgramLinksFixture(), fixture);
  for (const entry of [...fixture.cases, ...fixture.derivedCases]) {
    const network = LinkNetwork.fromLino(entry.lino);
    const emitted = emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'JavaScript');
    const output = [];
    vm.runInNewContext(emitted.code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
    assert.deepEqual(output, entry.output);
    assert.equal(emitted.code, entry.javascript);
  }
});

test('shared typed graph paths stop at the semantic traversal budget', () => {
  for (const entry of fixture.resourceCases) {
    const network = LinkNetwork.fromLino(entry.lino);
    assert.ok([...network.links()].length < 1000);
    assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'JavaScript'), /resource limit/u);
  }
});

test('machine integer width edits are refused before target emission', () => {
  for (const width of fixture.invalidMachineIntegerWidths) {
    const entry = fixture.cases.find((entry) => entry.id === 'rust-add');
    const network = LinkNetwork.fromLino(entry.lino);
    const bits = network.fieldRelations().find((relation) => relation.label === 'bits').child;
    const value = network.link(bits);
    value.setMetadata(value.metadata().withDefinition(String(width)));
    assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), 'Rust'), /invalid machine integer type/u);
  }
});

test('linked operation edits remain admitted decisions rather than host source', () => {
  for (const entry of fixture.operationMutations) {
    const network = LinkNetwork.fromLino(entry.lino);
    const operation = network.fieldRelations().find((relation) => relation.label === 'operation'
      && network.link(relation.child).metadata().definition === entry.operation).child;
    const value = network.link(operation);
    value.setMetadata(value.metadata().withDefinition(entry.replacement));
    for (const target of entry.runtimes) assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), target), /expression operation/u);
  }
});

test('division rounding edits cannot enter quoted target source', () => {
  for (const entry of fixture.roundingMutations) {
    const network = LinkNetwork.fromLino(entry.lino);
    const rounding = network.fieldRelations().find((relation) => relation.label === 'rounding').child;
    const value = network.link(rounding);
    value.setMetadata(value.metadata().withDefinition(entry.replacement));
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(entry.root), target), /rounding/u);
  }
});

test('floating-point spellings retain canonical special values', () => {
  for (const replacement of fixture.invalidFloatingPointSpellings) {
    const network = LinkNetwork.fromLino(fixture.cases[0].lino);
    const literal = network.fieldRelations().find((relation) => relation.label === 'value'
      && network.link(relation.child).metadata().definition === '2').child;
    const value = network.link(literal);
    value.setMetadata(value.metadata().withDefinition(replacement));
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(network, LinkId.fromU64(fixture.cases[0].root), target), /floating-point literal/u);
  }
  for (const entry of fixture.validFloatingPointSpellings) {
    const network = LinkNetwork.fromLino(fixture.cases[0].lino);
    const literal = network.fieldRelations().find((relation) => relation.label === 'value'
      && network.link(relation.child).metadata().definition === '2').child;
    const value = network.link(literal);
    value.setMetadata(value.metadata().withDefinition(entry.value));
    const emitted = emitTypedProgramLinks(network, LinkId.fromU64(fixture.cases[0].root), 'JavaScript');
    const output = [];
    vm.runInNewContext(emitted.code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
    assert.deepEqual(output, entry.output);
    assert.ok(emitTypedProgramLinks(network, LinkId.fromU64(fixture.cases[0].root), 'Rust').code.length > 0);
  }
});

test('constructor immutability descriptors and origin defaults survive shared archive reload', () => {
  for (const entry of fixture.constructorImmutabilityCases) {
    const network = LinkNetwork.fromLino(entry.lino);
    const root = LinkId.fromU64(entry.root);
    const code = emitTypedProgramLinks(network, root, 'JavaScript').code;
    const actual = JSON.parse(vm.runInNewContext(`${code}\n(() => {
      const value = make(); const property = Object.getOwnPropertyDescriptor(value, 'value');
      return JSON.stringify({ frozen: Object.isFrozen(value), extensible: Object.isExtensible(value),
        writable: property.writable, configurable: property.configurable, enumerable: property.enumerable,
        assigned: Reflect.set(value, 'value', 2), value: value.value });
    })()`, {}, { timeout: 1000 }));
    assert.deepEqual(actual, entry.descriptors);
    assert.equal(code, entry.javascript);
    if (entry.id !== 'mutable-absent') {
      const immutability = network.fieldRelations().find((relation) => relation.label === 'constructor immutability');
      assert.ok(immutability, 'the explicit boolean policy is inspectable linked data');
      assert.equal(network.link(immutability.child).metadata().term, 'boolean value');
    }
  }
});

function normalized(value) {
  if (value instanceof Map) return [...value].map(([name, value]) => [name, normalized(value)]);
  if (Array.isArray(value)) return value.map(normalized);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value)
    .filter(([field, value]) => value != null && !(value === false && ['generated', 'mutual', 'constant', 'console', 'sequentialAsync', 'outputThreaded', 'abortsThreaded'].includes(field)))
    .map(([field, value]) => [field, normalized(value)]));
  return value;
}

test('shared expression projection reuses the existing checker without semantic changes', () => {
  for (const entry of [...fixture.cases, ...fixture.derivedCases]) {
    const network = LinkNetwork.fromLino(entry.lino);
    const original = decodeTypedProgramLinks(network, LinkId.fromU64(entry.root));
    const surface = { language: original.sourceLanguage, items: original.items.map((entry) => ({
      k: 'fn', name: entry.name, params: entry.params, ret: entry.ret, body: projectLinkedProgramExpression(entry.body), span: entry.span, generated: entry.generated,
    })), main: original.main && { ...original.main, effects: original.main.effects.map((effect) => effect.k === 'print'
      ? { ...effect, expr: projectLinkedProgramExpression(effect.expr), style: 'js-console' }
      : { ...effect, type: effect.value.type, value: projectLinkedProgramExpression(effect.value) }) } };
    assert.deepEqual(normalized(checkProgram(surface)), normalized(original), entry.id);
  }
});
