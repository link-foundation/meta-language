import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { createConceptViews, readConceptOntology, readConceptTaxonomy } from '../src/concept-ontology.js';
import { LinkNetwork } from '../src/network.js';
import { LinkMetadata } from '../src/primitives.js';
import { readFormalConceptDefinition, proveFormalConceptEquivalence } from '../src/formal-concept-definition.js';

const fixture = readProjectData(new URL('../../parity/fixtures/concept-views.lino', import.meta.url));
const reload = value => ({ network: LinkNetwork.fromLino(value.links), root: value.root });
const role = (network, name) => network.links().filter(link => link.metadata().language === 'meta-language concept views' && link.metadata().term === name);

test('paired entry points expose orphan definitions without caller grammar registration', () => {
  for (const value of [fixture.orphanOnly, fixture.withBase]) {
    const { network, root } = reload(value);
    const ontology = readConceptOntology(network, root); const taxonomy = readConceptTaxonomy(network, root);
    assert.deepEqual(ontology, value.expectedOntology); assert.deepEqual(taxonomy, value.expectedTaxonomy);
    for (const concept of ontology.concepts) {
      assert.equal(taxonomy.concepts.find(value => value.identity === concept.identity).link, concept.link);
      assert.ok(network.link(concept.link));
      assert.ok(concept.definitionRoots.every(id => network.link(id)));
      assert.ok(concept.provenanceRoots.every(id => network.link(id)));
    }
    const recursion = ontology.concepts.find(value => value.identity === 'fixture.orphan.recursion');
    assert.equal(network.link(recursion.definitionRoots[0]).metadata().term, 'formal concept definition');
    assert.equal(readFormalConceptDefinition({ network, root: recursion.definitionRoots[0] }).model, 'checked typed program', 'the public root is an actual interpretable definition in the view graph');
    assert.deepEqual(recursion.provenance, { source: 'independent linked program', edition: '1' });
    assert.notEqual(ontology.concepts.find(value => value.identity === 'fixture.orphan.text').link,
      ontology.concepts.find(value => value.identity === 'fixture.orphan.second-text').link, 'equal definitions do not automatically merge declared identities');
    const unknown = ontology.concepts.find(value => value.identity === 'fixture.orphan.unknown');
    assert.equal(network.link(unknown.definitionRoots[0]).metadata().language, 'uninterpreted selected model');
  }
});

test('multiple root links are independently selectable and type roles stay declared', () => {
  const value = fixture.withBase; const { network, root } = reload(value);
  assert.deepEqual(readConceptOntology(network, root, { selectedRoot: value.expectedOntology.roots[0].link }), value.selectedOntology);
  assert.deepEqual(readConceptTaxonomy(network, root, { selectedRoot: value.expectedTaxonomy.roots[1].link }), value.selectedTaxonomy);
  const ontology = readConceptOntology(network, root); const taxonomy = readConceptTaxonomy(network, root);
  assert.ok(ontology.roots.every(value => network.link(value.link).metadata().term === 'concept view root'));
  assert.equal(ontology.roots[0].typeModel, null); assert.notEqual(ontology.roots[1].typeModel, null);
  assert.ok(taxonomy.roots.every(value => value.typeModel === null), 'root membership alone never declares a type');
  assert.deepEqual(ontology.relations.map(value => value.kind), ['association', 'axiom']);
  assert.deepEqual(taxonomy.relations.map(value => value.kind), ['classification', 'subsumption']);
  const unknown = ontology.concepts.find(value => value.identity === 'fixture.orphan.unknown').link;
  assert.ok(value.selectedOntology.reachable.includes(unknown));
  assert.ok(!value.selectedTaxonomy.reachable.includes(unknown), 'associative relations do not become hierarchy edges');
  assert.throws(() => readConceptOntology(network, root, { selectedRoot: taxonomy.roots[0].link }), /unknown selected root/u);
});

test('actual formal graph sharing and cycles survive paired views', () => {
  const formalFixture = readProjectData(new URL('../../parity/fixtures/formal-concept-definitions.lino', import.meta.url));
  const definition = reload(formalFixture.definitions[4].archive);
  const packet = createConceptViews(null, { orphans: [
    { identity: 'shared.first', definition }, { identity: 'shared.second', definition },
  ] });
  const { network, root } = reload({ links: packet.network.toLino(), root: packet.root });
  const ontology = readConceptOntology(network, root);
  assert.equal(ontology.concepts[0].definitionRoots[0], ontology.concepts[1].definitionRoots[0], 'the actual same source network is copied once');
  const archiveGroup = role(network, 'concept view archive').find(group => {
    const inventory = network.link(group.references()[0]).references();
    return inventory.some(id => id.asU64() === ontology.concepts[0].definitionRoots[0]);
  });
  const inventory = network.link(archiveGroup.references()[0]).references();
  const source = new LinkNetwork();
  for (const id of inventory) { const link = network.link(id); source.insertLinkWithOptionalId(id, link.references(), link.metadata()); }
  const interpreted = readFormalConceptDefinition({ network: source, root: ontology.concepts[0].definitionRoots[0] });
  assert.deepEqual(interpreted, formalFixture.definitions[4].prepared);
  assert.equal(proveFormalConceptEquivalence(fixture.options.orphans[2].definition, fixture.options.orphans[2].definition), null, 'unknown interpretation is retained without a proof claim');
});

test('view creation preserves deterministic graphs and rejects ambiguous declarations', () => {
  for (const [base, value] of [[null, fixture.orphanOnly], [reload(fixture.base), fixture.withBase]]) {
    const packet = createConceptViews(base, fixture.options);
    assert.equal(packet.network.toLino(), value.links); assert.equal(packet.root.asU64(), value.root);
  }
  assert.throws(() => createConceptViews(null, { orphans: [fixture.options.orphans[0], fixture.options.orphans[0]] }), /colliding declared identity/u);
  assert.throws(() => createConceptViews(null, { ...fixture.options, taxonomy: { relations: [fixture.options.ontology.relations[0]] } }), /selected view/u);
  const baseIdentity = fixture.withBase.expectedOntology.concepts.find(value => !value.identity.startsWith('fixture.orphan.')).identity;
  assert.throws(() => createConceptViews(reload(fixture.base), { orphans: [{ ...fixture.options.orphans[0], identity: baseIdentity }] }), /colliding declared identity/u);
});

test('foreign membership, model and base graph edits cannot counterfeit views', () => {
  for (const edit of [
    network => role(network, 'concept view relation')[2].setMetadata(role(network, 'concept view relation')[2].metadata().withDefinition('association')),
    network => { const root = role(network, 'taxonomy view')[0]; network.link(root.references()[0]).setReferences([root.id()]); },
    network => { const root = role(network, 'concept view root')[0]; const view = role(network, 'ontology view')[0]; network.link(view.references()[1]).setReferences([root.id(), root.id()]); },
    network => { const definition = role(network, 'concept view definition')[0]; definition.setReferences([role(network, 'ontology view')[0].id(), definition.references()[1]]); },
    network => { const root = network.links().find(link => link.metadata().term === 'discovered concept graph'); root.setMetadata(root.metadata().withDefinition('unknown version')); },
    network => network.insertLink([], LinkMetadata.new()),
  ]) {
    const { network, root } = reload(fixture.withBase); edit(network);
    assert.throws(() => readConceptOntology(network, root));
    assert.throws(() => readConceptTaxonomy(network, root));
  }
});
