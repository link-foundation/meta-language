import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeFormalConceptDefinition, readFormalConceptDefinition, proveFormalConceptEquivalence, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';

const reload = ({ network, root }) => ({ network: LinkNetwork.fromLino(network.toLino()), root });
const encode = (source) => encodeFormalConceptDefinition(createTypedProgramLinks(source, 'JavaScript'));
const scalar = (network, label, text) => network.fieldRelations().find((relation) => relation.label === label && network.link(relation.child).metadata().definition === text)?.child;

test('recursive operational definitions carry a closed linked correspondence after reload', () => {
  const first = reload(encode('function factorial(n){return n<=1 ? 1 : n*factorial(n-1);} console.log(factorial(4));'));
  const second = reload(encode('function factorial(value) { return value<=1 ? 1 : value*factorial(value-1); } console.log(factorial(4));'));
  const proof = proveFormalConceptEquivalence(first, second);
  assert.ok(proof, 'binding identity permits alpha-equivalence while retaining public names');
  assert.equal(verifyFormalConceptEquivalence(first, second, reload(proof)), true);
  assert.ok(first.network.fieldRelations().some(({ label, child }) => label === 'declaration reference' && first.network.link(child).metadata().term === 'program record'));
  const prepared = readFormalConceptDefinition(first);
  assert.equal(prepared.model, 'checked typed program');
  assert.equal(prepared.shapes.length + 1, prepared.offsets.length);
  assert.ok(prepared.edges.length > prepared.shapes.length, 'sharing and recursive references are retained');
});

test('public declaration names and semantic source context remain part of the definition', () => {
  const first = encode('function add(a,b){return a+b;} console.log(add(2,3));');
  const renamed = encode('function sum(a,b){return a+b;} console.log(sum(2,3));');
  assert.equal(proveFormalConceptEquivalence(first, renamed), null);
  const rust = encodeFormalConceptDefinition(createTypedProgramLinks('fn add(a: f64,b: f64)->f64{a+b} fn main(){}', 'Rust'));
  assert.equal(proveFormalConceptEquivalence(first, rust), null, 'a source-language change alone is no operational proof');
});

test('primitive operations, type policies, binding targets and effects constrain equality', () => {
  const first = encode('function calculate(a,b){return a+b;} console.log(calculate(2,3));');
  const proof = proveFormalConceptEquivalence(first, reload(first));
  assert.ok(proof);
  for (const [label, old, replacement] of [['operation', 'add', 'mul'], ['semantics', 'ieee', 'checked'], ['console', 'true', 'false']]) {
    const changed = reload(first);
    const id = scalar(changed.network, label, old);
    assert.ok(id, label);
    changed.network.link(id).setMetadata(changed.network.link(id).metadata().withDefinition(replacement));
    assert.equal(proveFormalConceptEquivalence(first, changed), null, label);
    assert.equal(verifyFormalConceptEquivalence(first, changed, proof), false, label);
  }
  const retargeted = reload(first);
  const targets = retargeted.network.fieldRelations().filter(({ label }) => label === 'binding reference');
  const firstReference = targets[0];
  firstReference.link.setReferences([firstReference.parent, firstReference.link.references()[1], targets[1].child]);
  assert.equal(proveFormalConceptEquivalence(first, retargeted), null, 'retargeting equal types still changes binding identity');
});

test('missing, unknown and counterfeit formal primitive bases never authorize sharing', () => {
  const program = createTypedProgramLinks('console.log(3);', 'JavaScript');
  assert.equal(proveFormalConceptEquivalence(program, reload(program)), null);
  assert.throws(() => encodeFormalConceptDefinition(program, { model: 'declared by prose' }), /unsupported/);
  const first = encodeFormalConceptDefinition(undefined, { model: 'source trivia interpretation' });
  const counterfeit = reload(first);
  const basis = counterfeit.network.link(counterfeit.root).references()[1];
  counterfeit.network.link(basis).setMetadata(counterfeit.network.link(basis).metadata().withDefinition('unknown model'));
  assert.equal(proveFormalConceptEquivalence(first, counterfeit), null);
  const altered = reload(first);
  const version = scalar(altered.network, 'version', '1');
  altered.network.link(version).setMetadata(altered.network.link(version).metadata().withDefinition('2'));
  assert.equal(proveFormalConceptEquivalence(first, altered), null);
  const text = encodeFormalConceptDefinition(undefined, { model: 'source text value' });
  assert.equal(proveFormalConceptEquivalence(first, text), null, 'trivia and text values have distinct operational interpretations');
});

test('provenance remains explicit linked data outside the operational correspondence', () => {
  const first = encodeFormalConceptDefinition(undefined, { model: 'source trivia interpretation', provenance: { project: 'first project', edition: '1' } });
  const second = encodeFormalConceptDefinition(undefined, { model: 'source trivia interpretation', provenance: { project: 'second project', edition: '2' } });
  assert.ok(first.network.fieldRelations().some(({ parent, label }) => parent.asU64() === first.root.asU64() && label === 'provenance'));
  const proof = proveFormalConceptEquivalence(first, second);
  assert.ok(proof);
  assert.equal(verifyFormalConceptEquivalence(reload(first), reload(second), reload(proof)), true);
});

test('a certificate refuses removed, duplicated, retargeted or stale correspondence links', () => {
  const first = encode('function add(a,b){return a+b;} console.log(add(2,3));');
  const proof = proveFormalConceptEquivalence(first, reload(first));
  assert.ok(proof);
  for (const change of ['removed', 'duplicated', 'retargeted']) {
    const broken = reload(proof);
    const list = broken.network.link(broken.network.link(broken.root).references()[2]);
    const pairs = list.references();
    if (change === 'removed') list.setReferences(pairs.slice(1));
    if (change === 'duplicated') list.setReferences([pairs[0], ...pairs.slice(0, -1)]);
    if (change === 'retargeted') {
      const pair = broken.network.link(pairs[1]);
      pair.setReferences(broken.network.link(pairs[0]).references());
    }
    assert.equal(verifyFormalConceptEquivalence(first, reload(first), broken), false, change);
  }
  const malformed = reload(first);
  const body = malformed.network.link(malformed.root).references()[0];
  malformed.network.link(body).setReferences([999999]);
  assert.equal(proveFormalConceptEquivalence(first, malformed), null);
});
