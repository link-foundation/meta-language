#!/usr/bin/env node
// Bounded actual linked premises, shared by JavaScript and native regressions.
import assert from 'node:assert/strict';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId, LinkMetadata, LinkType } from '../src/primitives.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeFormalConceptDefinition, proveFormalConceptEquivalence, readFormalConceptDefinition, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';

const portable = ({ network, root }) => ({ links: network.toLino(), root: LinkId.from(root).asU64() });
const reload = (archive) => ({ network: LinkNetwork.fromLino(archive.links), root: archive.root });
const definition = portable(encodeFormalConceptDefinition(undefined, { model: 'source text value' }));
const recursive = portable(encodeFormalConceptDefinition(createTypedProgramLinks('function factorial(n){return n<=1?1:n*factorial(n-1)} console.log(factorial(4));', 'JavaScript')));
const changes = ['label namespace', 'label meaning', 'label kind', 'label named role', 'label self reference', 'label arity', 'label term', 'edge namespace', 'edge meaning', 'edge term', 'edge named role', 'edge arity', 'dangling label', 'separate same-spelling label identity'];

function mutate(archive, change) {
  const value = reload(archive);
  const relations = value.network.fieldRelations();
  const relation = change === 'separate same-spelling label identity'
    ? relations.find(({ label }) => label === (relations.some((entry) => entry.label === 'construct kind') ? 'construct kind' : 'version'))
    : relations[0];
  assert.ok(relation, change);
  const labelId = relation.link.references()[1];
  const label = value.network.link(labelId);
  if (change === 'label namespace') label.setMetadata(label.metadata().withLanguage('other formal namespace'));
  else if (change === 'label meaning') label.setMetadata(label.metadata().withDefinition('another relation interpretation'));
  else if (change === 'label kind') label.setMetadata(label.metadata().withLinkType(LinkType.Concept));
  else if (change === 'label named role') label.setMetadata(label.metadata().withNamed(false));
  else if (change === 'label self reference') label.setReferences([relation.parent]);
  else if (change === 'label arity') label.setReferences([labelId, relation.parent]);
  else if (change === 'label term') label.setMetadata(label.metadata().clone({ term: undefined }));
  else if (change === 'edge namespace') relation.link.setMetadata(relation.link.metadata().withLanguage('other formal namespace'));
  else if (change === 'edge meaning') relation.link.setMetadata(relation.link.metadata().withDefinition('another relation interpretation'));
  else if (change === 'edge term') relation.link.setMetadata(relation.link.metadata().withTerm('other relation'));
  else if (change === 'edge named role') relation.link.setMetadata(relation.link.metadata().withNamed(true));
  else if (change === 'edge arity') relation.link.setReferences([...relation.link.references(), relation.child]);
  else if (change === 'dangling label') relation.link.setReferences([relation.parent, 999999, relation.child]);
  else {
    const cloned = value.network.insertLink([], label.metadata());
    value.network.link(cloned).setReferences([cloned]);
    relation.link.setReferences([relation.parent, cloned, relation.child]);
  }
  return portable(value);
}

const definitions = [definition, recursive].map((archive, index) => ({ name: index === 0 ? 'declared text primitive' : 'recursive checked program', archive, prepared: readFormalConceptDefinition(archive) }));
const rejections = definitions.flatMap(({ name, archive }, first) => changes.map((change) => {
  const altered = mutate(archive, change);
  assert.throws(() => readFormalConceptDefinition(altered), undefined, `${name}: ${change}`);
  assert.equal(proveFormalConceptEquivalence(archive, altered), null, change);
  return { name: `${name}: ${change}`, first, archive: altered };
}));
const certificate = portable(proveFormalConceptEquivalence(definition, definition));
const invalidCertificates = changes.map((change) => {
  const altered = mutate(certificate, change);
  assert.equal(verifyFormalConceptEquivalence(definition, definition, altered), false, change);
  return { name: change, certificate: altered };
});
const disconnected = reload(definition);
const parent = disconnected.network.insertLink([], LinkMetadata.new());
const child = disconnected.network.insertLink([], LinkMetadata.new());
const foreign = disconnected.network.insertFieldRelation(parent, 'unrelated namespaced relation', child);
const foreignLabel = disconnected.network.link(disconnected.network.link(foreign).references()[1]);
foreignLabel.setMetadata(foreignLabel.metadata().withLanguage('other formal namespace'));
disconnected.network.insertFieldRelation(parent, 'unrelated duplicate relation', child);
disconnected.network.insertFieldRelation(parent, 'unrelated duplicate relation', child);
const disconnectedArchive = portable(disconnected);
assert.deepEqual(readFormalConceptDefinition(disconnectedArchive), definitions[0].prepared);
assert.ok(proveFormalConceptEquivalence(definition, disconnectedArchive));
const fixture = { schemaVersion: 1, definitions, rejections, invalidCertificates, disconnectedArchive };
const path = new URL('../../parity/fixtures/formal-concept-relation-identities.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(path, fixture);
else writeProjectData(path, fixture);
console.log(`Verified ${definitions.length} premises, ${rejections.length} changed identities and ${invalidCertificates.length} counterfeit certificates.`);
