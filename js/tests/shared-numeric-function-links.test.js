import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { deserializeGrammar } from '../src/grammar.js';
import { encodeSharedNumericFunctionDefinition } from '../src/shared-numeric-function-links.js';
import { proveFormalConceptEquivalence, readFormalConceptDefinition, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';
import { decodeTypedProgramLinks, emitTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';
import { acceptSharedNumericBinary, acceptSharedNumericContext, acceptSharedNumericUnary } from '../src/shared-numeric-function-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/shared-numeric-function-definitions.json', import.meta.url));
const archive = value => ({ network: LinkNetwork.fromLino(value.links), root: value.root });
const program = value => ({ network: LinkNetwork.fromLino(value.programLinks), root: value.programRoot });
const restore = value => archive({ links: value.network.toLino(), root: value.root });
const sources = () => fixture.globalSources.map(source => ({ ...source, grammar: deserializeGrammar(JSON.stringify(source.grammar)) }));
const run = (code, argumentsList) => vm.runInNewContext(`${code}\nselected_function_definition(...argumentsList);`, { argumentsList, process: { argv: [] } }, { timeout: 1000 });

test('the explicit numeric model proves actual Rust and JavaScript recursive value definitions after source-free reload', () => {
  for (const value of fixture.cases) {
    const selected = encodeSharedNumericFunctionDefinition(program(value), value.declaration, { provenance: value.provenance });
    assert.deepEqual(readFormalConceptDefinition(restore(selected)), value.expected);
    const typedRoot = selected.network.link(selected.root).references()[0];
    assert.equal(decodeTypedProgramLinks(selected.network, typedRoot).sourceLanguage, value.language, 'language is retained in the actual linked source context');
    const provenance = selected.network.fieldRelations().find(field => field.parent.equals(selected.root) && field.label === 'provenance');
    assert.ok(provenance);
    assert.ok(selected.network.fieldRelations().some(field => field.parent.equals(provenance.child) && field.label === 'sourceLanguage' && selected.network.link(field.child).metadata().definition === value.language));
  }
  for (const [first, second] of fixture.equivalent) {
    const one = archive(fixture.cases[first]); const two = archive(fixture.cases[second]);
    const proof = proveFormalConceptEquivalence(one, two);
    assert.ok(proof);
    assert.equal(verifyFormalConceptEquivalence(one, two, restore(proof)), true);
    const originalOne = archive({ links: fixture.cases[first].conservativeLinks, root: fixture.cases[first].conservativeRoot });
    const originalTwo = archive({ links: fixture.cases[second].conservativeLinks, root: fixture.cases[second].conservativeRoot });
    assert.equal(proveFormalConceptEquivalence(originalOne, originalTwo), null, 'the existing source-language model is unchanged');
  }
});

test('closed numeric meanings ground an existing global identity while a changed operation remains distinct', () => {
  const definitions = { 'javascript:function': fixture.cases[0], 'rust:function': fixture.cases[1] };
  const space = createGlobalConceptSpace(sources(), { records: [], formalDefinitions: definitions });
  assert.equal(space.mappings['javascript:function'], space.mappings['rust:function']);
  assert.equal(space.concepts.length, 1);
  assert.deepEqual(space.concepts[0].origins.map(value => value.language), ['JavaScript', 'Rust']);
  assert.ok(space.proofs.length);
  const reloaded = readGlobalConceptSpaceLinks(globalConceptSpaceToLinks(space), { records: [] });
  assert.deepEqual(reloaded, space);
  const changed = createGlobalConceptSpace(sources(), { records: [], formalDefinitions: { ...definitions, 'rust:function': fixture.cases.at(-1) } });
  assert.notEqual(changed.mappings['javascript:function'], changed.mappings['rust:function']);
  for (const [first, second] of fixture.distinct) assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[first]), archive(fixture.cases[second])), null);
});

test('independent JavaScript sources and both source-free interpretations agree on finite, infinite, NaN and signed zero observations', () => {
  for (const [first, second] of fixture.equivalent) {
    const one = fixture.cases[first]; const two = fixture.cases[second];
    const original = `${one.source}\nfunction selected_function_definition(...argumentsList){return ${one.declaration}(...argumentsList);}`;
    const emitted = [one, two].map(value => { const definition = archive(value); return emitTypedProgramLinks(definition.network, definition.network.link(definition.root).references()[0], 'JavaScript').code; });
    const inputs = [...one.inputs];
    if (['add', 'less', 'rem', 'divide'].includes(one.declaration)) inputs.push([NaN, 2], [Infinity, Infinity], [-0, 1]);
    for (const values of inputs) {
      const expected = run(original, values);
      for (const code of emitted) assert.ok(Object.is(run(code, values), expected), `${one.declaration}(${values})`);
    }
  }
});

test('unsupported types, defaults, effects and undeclared operations cannot acquire the numeric basis', () => {
  for (const refusal of [...fixture.refusals, ...fixture.originRefusals]) assert.throws(() => encodeSharedNumericFunctionDefinition(program(refusal), refusal.declaration), /numeric|Boolean|binary64|guard|default|supported|outside|output|effect/iu, refusal.name);
  for (const invalid of fixture.invalidDefinitions) {
    assert.throws(() => readFormalConceptDefinition(archive(invalid)));
    assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[0]), archive(invalid)), null);
  }
  const changed = archive(fixture.cases[0]);
  const typedRoot = changed.network.link(changed.root).references()[0];
  const body = changed.network.link(typedRoot).references()[0];
  const language = changed.network.fieldRelations().find(field => field.parent.equals(body) && field.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceLanguage);
  changed.network.link(language.child).setMetadata(changed.network.link(language.child).metadata().withDefinition('Lean'));
  assert.throws(() => readFormalConceptDefinition(changed), /numeric|outside|Rust|JavaScript/iu);
  assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[0]), changed), null);
  const forged = archive(fixture.cases[0]);
  const primitive = forged.network.link(forged.root).references()[1];
  const meaning = forged.network.link(primitive).references()[0];
  const interpretation = forged.network.fieldRelations().find(field => field.parent.equals(meaning) && field.label === 'sourceLanguageRole');
  forged.network.link(interpretation.child).setMetadata(forged.network.link(interpretation.child).metadata().withDefinition('ignore every language unconditionally'));
  assert.throws(() => readFormalConceptDefinition(forged), /primitive model differs/u);
  assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[0]), forged), null);
});

test('generated admission premises preserve the domain, operation and source-context conjunctions', () => {
  assert.equal(acceptSharedNumericContext('JavaScript', true), false);
  assert.equal(acceptSharedNumericContext('Rust', false), true);
  assert.equal(acceptSharedNumericBinary('eq', 'float', 'checked'), false);
  assert.equal(acceptSharedNumericBinary('eq', 'bool', ''), true);
  assert.equal(acceptSharedNumericBinary('rem', 'float', 'ieee'), true);
  assert.equal(acceptSharedNumericUnary('neg', 'float', ''), false);
});
