// Small cross-runtime archives measured through the public JavaScript APIs.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeFormalConceptDefinition, readFormalConceptDefinition, proveFormalConceptEquivalence, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';

const portable = ({ network, root }) => ({ links: network.toLino(), root: Number(root.asU64()) });
const reload = (value) => ({ network: LinkNetwork.fromLino(value.links), root: LinkId.from(value.root) });
const primitive = (model) => encodeFormalConceptDefinition(undefined, { model });
const program = (source) => encodeFormalConceptDefinition(createTypedProgramLinks(source, 'JavaScript'));
const inputs = [
  ['source text value', primitive('source text value')],
  ['source trivia interpretation', primitive('source trivia interpretation')],
  ['addition definition', program('function add(a,b){return a+b;} console.log(add(2,3));')],
  ['addition alpha binding', program('function add(first,second){return first+second;} console.log(add(2,3));')],
  ['recursive factorial definition', program('function factorial(n){return n<=1 ? 1 : n*factorial(n-1);} console.log(factorial(4));')],
  ['recursive factorial alpha binding', program('function factorial(value){return value<=1 ? 1 : value*factorial(value-1);} console.log(factorial(4));')],
];
const definitions = inputs.map(([name, archive]) => ({ name, archive: portable(archive), prepared: readFormalConceptDefinition(archive) }));
const proofs = [[0, 0], [1, 1], [2, 3], [4, 5]].map(([first, second]) => {
  const proof = proveFormalConceptEquivalence(inputs[first][1], inputs[second][1]);
  assert.ok(proof, definitions[first].name);
  const certificate = portable(proof);
  assert.equal(verifyFormalConceptEquivalence(definitions[first].archive, definitions[second].archive, certificate), true);
  return { name: definitions[first].name, first, second, certificate };
});
const rejections = [];
for (const change of ['unknown model', 'changed basis', 'structural cycle', 'dangling reference', 'duplicate label']) {
  const archive = reload(definitions[0].archive);
  const [body, basis] = archive.network.link(archive.root).references();
  if (change === 'unknown model') {
    const link = archive.network.link(basis);
    link.setMetadata(link.metadata().withDefinition('unsupported prose assertion'));
  } else if (change === 'changed basis') {
    const relation = archive.network.fieldRelations().find(({ parent, label }) => parent.equals(archive.network.link(basis).references()[0]) && label === 'version');
    const link = archive.network.link(relation.child);
    link.setMetadata(link.metadata().withDefinition('2'));
  } else if (change === 'structural cycle') archive.network.link(body).setReferences([body]);
  else if (change === 'dangling reference') archive.network.link(body).setReferences([LinkId.from(999999)]);
  else {
    const relation = archive.network.fieldRelations().find(({ parent }) => parent.equals(body));
    archive.network.insertFieldRelation(body, relation.label, relation.child);
  }
  const value = portable(archive);
  assert.throws(() => readFormalConceptDefinition(value), undefined, change);
  assert.equal(proveFormalConceptEquivalence(definitions[0].archive, value), null, change);
  rejections.push({ name: change, archive: value });
}
const invalidCertificates = [];
for (const change of ['removed pair', 'duplicate pair', 'retargeted pair', 'stale copied premise']) {
  const certificate = reload(proofs[0].certificate);
  const [firstRoot, , listRoot] = certificate.network.link(certificate.root).references();
  const list = certificate.network.link(listRoot);
  const pairs = list.references();
  if (change === 'removed pair') list.setReferences(pairs.slice(1));
  else if (change === 'duplicate pair') list.setReferences([pairs[0], ...pairs.slice(0, -1)]);
  else if (change === 'retargeted pair') certificate.network.link(pairs[1]).setReferences(certificate.network.link(pairs[0]).references());
  else {
    const link = certificate.network.link(firstRoot);
    link.setMetadata(link.metadata().withDefinition('2'));
  }
  const value = portable(certificate);
  assert.equal(verifyFormalConceptEquivalence(definitions[0].archive, definitions[0].archive, value), false, change);
  invalidCertificates.push({ name: change, first: 0, second: 0, certificate: value });
}
const fixture = { schemaVersion: 1, definitions, proofs, rejections, invalidCertificates };
const path = new URL('../../parity/fixtures/formal-concept-definitions.json', import.meta.url);
const expected = `${JSON.stringify(fixture, null, 2)}\n`;
if (process.argv.includes('--check')) assert.ok(readFileSync(path, 'utf8') === expected, 'formal concept fixtures are stale');
else writeFileSync(path, expected);
assert.equal(readFileSync(new URL('../src/data/formal-concept-models.json', import.meta.url), 'utf8'), readFileSync(new URL('../../rust/src/data/formal-concept-models.json', import.meta.url), 'utf8'), 'formal primitive declarations differ between runtimes');
process.stdout.write(`Verified ${definitions.length} formal definitions, ${proofs.length} proofs, ${rejections.length} basis rejections and ${invalidCertificates.length} invalid certificates.\n`);
