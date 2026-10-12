import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import { typedProgramLinksFixture } from '../scripts/generate-typed-program-links-fixtures.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { emitTypedProgramLinks } from '../src/typed-program-links.js';
import { decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { checkProgram } from '../src/translation/check.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/typed-program-links.json', import.meta.url), 'utf8'));
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
