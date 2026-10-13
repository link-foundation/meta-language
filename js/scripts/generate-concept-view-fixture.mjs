#!/usr/bin/env node
// Small direct graphs; no catalog scan or upstream corpus execution.
import assert from 'node:assert/strict';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { Grammar } from '../src/grammar.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeFormalConceptDefinition } from '../src/formal-concept-definition.js';
import { createDiscoveredConceptLinks } from '../src/discovered-concept-links.js';
import { createConceptViews, readConceptOntology, readConceptTaxonomy } from '../src/concept-ontology.js';

const portable = ({ network, root }) => ({ links: network.toLino(), root: LinkId.from(root).asU64() });
const primitive = portable(encodeFormalConceptDefinition(undefined, { model: 'source text value' }));
const recursive = portable(encodeFormalConceptDefinition(createTypedProgramLinks('function factorial(n){return n<=1?1:n*factorial(n-1)} console.log(factorial(4));', 'JavaScript')));
const unknownNetwork = LinkNetwork.fromLino(primitive.links);
unknownNetwork.link(primitive.root).setMetadata(unknownNetwork.link(primitive.root).metadata().withLanguage('uninterpreted selected model'));
const unknown = portable({ network: unknownNetwork, root: primitive.root });
const options = {
  orphans: [
    { identity: 'fixture.orphan.recursion', definition: recursive, provenance: { source: 'independent linked program', edition: '1' } },
    { identity: 'fixture.orphan.text', definition: primitive, provenance: { source: 'declared text primitive' } },
    { identity: 'fixture.orphan.unknown', definition: unknown, provenance: { source: 'uninterpreted model retained' } },
    { identity: 'fixture.orphan.second-text', definition: primitive, provenance: { source: 'separate declared identity' } },
  ],
  ontology: { roots: [{ identity: 'fixture.orphan.recursion' }, { identity: 'fixture.orphan.text', typeModel: unknown }], relations: [
    { source: 'fixture.orphan.recursion', target: 'fixture.orphan.text', kind: 'association', model: primitive },
    { source: 'fixture.orphan.text', target: 'fixture.orphan.unknown', kind: 'axiom', model: unknown },
  ] },
  taxonomy: { roots: [{ identity: 'fixture.orphan.text' }, { identity: 'fixture.orphan.recursion' }], relations: [
    { source: 'fixture.orphan.text', target: 'fixture.orphan.recursion', kind: 'classification', model: unknown },
    { source: 'fixture.orphan.recursion', target: 'fixture.orphan.second-text', kind: 'subsumption', model: unknown },
  ] },
};
const base = createDiscoveredConceptLinks([{ id: 'bounded source', language: 'bounded language', grammar: new Grammar('root', [
  ['root', { kind: 'normal', expression: { kind: 'literal', value: 'x' } }],
]) }], { formalDefinitions: { 'bounded source:root': primitive } });
const make = source => {
  const packet = createConceptViews(source, options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  const ontology = readConceptOntology(network, packet.root); const taxonomy = readConceptTaxonomy(network, packet.root);
  assert.equal(ontology.roots.length, 2); assert.equal(taxonomy.roots.length, 2);
  assert.deepEqual(ontology.concepts.map(value => value.link), taxonomy.concepts.map(value => value.link));
  assert.equal(ontology.roots[0].typeModel, null); assert.equal(taxonomy.roots[0].typeModel, null);
  assert.notEqual(ontology.roots[1].typeModel, null);
  return { ...portable(packet), ontologyRoot: packet.ontology.asU64(), taxonomyRoot: packet.taxonomy.asU64(),
    expectedOntology: ontology, expectedTaxonomy: taxonomy,
    selectedOntology: readConceptOntology(network, packet.root, { selectedRoot: ontology.roots[0].link }),
    selectedTaxonomy: readConceptTaxonomy(network, packet.root, { selectedRoot: taxonomy.roots[1].link }) };
};
const fixture = { schemaVersion: 1, options, base: portable(base), orphanOnly: make(null), withBase: make(base) };
const path = new URL('../../parity/fixtures/concept-views.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(path, fixture); else writeProjectData(path, fixture);
console.log('Verified paired views, four orphan identities, recursive definitions and independently selected roots.');
