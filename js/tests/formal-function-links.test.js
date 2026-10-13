import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { encodeFormalFunctionDefinition } from '../src/formal-function-links.js';
import { encodeFormalConceptDefinition, proveFormalConceptEquivalence, readFormalConceptDefinition, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';

const fixture = readProjectData(new URL('../../parity/fixtures/formal-function-definitions.json', import.meta.url));
const archive = value => ({ network: LinkNetwork.fromLino(value.links), root: value.root });
const program = value => ({ network: LinkNetwork.fromLino(value.programLinks), root: value.programRoot });
function directory(network, root) {
  const body = network.link(root).references()[0];
  const mapping = network.fieldRelations().find(relation => relation.parent.equals(body) && relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.declarations).child;
  return new Map(network.link(mapping).references().map(id => {
    const [key, declaration] = network.link(id).references();
    return [network.link(key).metadata().definition, declaration];
  }));
}

test('a recursive function has the same checked definition across unrelated program contexts', () => {
  const first = program(fixture.cases[0]); const second = program(fixture.cases[1]);
  assert.equal(proveFormalConceptEquivalence(encodeFormalConceptDefinition(first), encodeFormalConceptDefinition(second)), null);
  for (const [first, second] of fixture.equivalent) {
    const one = archive(fixture.cases[first]); const other = archive(fixture.cases[second]);
    const proof = proveFormalConceptEquivalence(one, other);
    assert.ok(proof);
    assert.equal(verifyFormalConceptEquivalence(one, other, archive({ links: proof.network.toLino(), root: proof.root })), true);
  }
});

test('source-free selection preserves original recursive declaration and lexical binding identities', () => {
  for (const value of fixture.cases) {
    const input = program(value); const ids = directory(input.network, input.root);
    const definition = encodeFormalFunctionDefinition(input, ids.get(value.declaration), { provenance: { source: value.name, sourceLanguage: value.language } });
    assert.deepEqual(readFormalConceptDefinition(definition), value.expected);
    const typedRoot = definition.network.link(definition.root).references()[0];
    const selected = decodeTypedProgramLinks(definition.network, typedRoot);
    assert.ok(!selected.declarations.has('unused'));
    assert.equal(selected.sourceLanguage, value.language);
    assert.equal(selected.main.effects.length, 0);
    const wrapper = selected.declarations.get('selected_function_definition');
    assert.equal(wrapper.body.fn, value.declaration);
    const pointer = directory(definition.network, typedRoot).get(value.declaration);
    assert.ok(pointer.equals(ids.get(value.declaration)));
    for (const relation of input.network.fieldRelations().filter(relation => [TYPED_PROGRAM_LINKS_SCHEMA.references.declaration, TYPED_PROGRAM_LINKS_SCHEMA.references.binding].includes(relation.label))) {
      assert.deepEqual(definition.network.link(relation.link.id()).references(), relation.link.references());
    }
  }
});

test('operator, public identity, selected mutual member and source-language premises remain distinct', () => {
  for (const [first, second] of fixture.distinct) assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[first]), archive(fixture.cases[second])), null);
  const mutual = archive(fixture.cases[4]);
  const root = mutual.network.link(mutual.root).references()[0];
  const declarations = directory(mutual.network, root);
  assert.ok(declarations.has('even') && declarations.has('odd'));
  const references = mutual.network.fieldRelations().filter(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.references.declaration);
  assert.ok(references.some(relation => relation.child.equals(declarations.get('even'))));
  assert.ok(references.some(relation => relation.child.equals(declarations.get('odd'))));
});

test('foreign selectors and forged recursive targets cannot create a function meaning', () => {
  const input = program(fixture.cases[0]);
  assert.throws(() => encodeFormalFunctionDefinition(input, input.root), /owned function declaration/u);
  assert.throws(() => encodeFormalFunctionDefinition(input, 'not declared'), /owned function declaration/u);
  const collision = createTypedProgramLinks('/** @returns {number} */ function selected_function_definition(){return 1;} /** @returns {number} */ function keep(){return 2;}', 'JavaScript');
  assert.throws(() => encodeFormalFunctionDefinition(collision, 'keep'), /reserved function definition entry name/u);
  const changed = program(fixture.cases[0]);
  const reference = changed.network.fieldRelations().find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.references.declaration);
  const values = reference.link.references(); values[2] = changed.root; reference.link.setReferences(values);
  assert.throws(() => encodeFormalFunctionDefinition(changed, 'factorial'), /reference|declaration|structural|kind|owned/iu);
});

test('the source-free entry wrapper executes the selected recursive function', () => {
  const expected = [120, 120, 16, 120, false, true, false, 120];
  for (const [index, value] of fixture.cases.entries()) {
    const definition = archive(value);
    const root = definition.network.link(definition.root).references()[0];
    const code = emitTypedProgramLinks(definition.network, root, 'JavaScript').code;
    assert.equal(vm.runInNewContext(`${code}\nselected_function_definition(5);`, { process: { argv: [] } }, { timeout: 1000 }), expected[index]);
  }
});
