import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork } from '../src/network.js';
import { LinkMetadata } from '../src/primitives.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { encodeFormalConceptDefinition, readFormalConceptDefinition, proveFormalConceptEquivalence, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const reload = ({ network, root }) => ({ network: LinkNetwork.fromLino(network.toLino()), root });
const encode = (source) => encodeFormalConceptDefinition(createTypedProgramLinks(source, 'JavaScript'));
const scalar = (network, label, text) => network.fieldRelations().find((relation) => relation.label === label && network.link(relation.child).metadata().definition === text)?.child;
const output = (code) => {
  const values = [];
  vm.runInNewContext(code, { console: { log: (...args) => values.push(args.map((value) => value && typeof value === 'object' ? JSON.stringify(value) : String(value)).join(' ')) }, process: { argv: [] } }, { timeout: 1_000 });
  return values;
};

test('formal definitions refuse forged structured text conversion and undeclared Rust float display', () => {
  const scalarProgram = () => checkProgram(parseJavaScript('function value(){return String(1)}'));
  const structured = scalarProgram();
  structured.declarations.get('value').body.arg = { k: 'array',
    items: [{ k: 'lit', value: '1', type: { kind: 'float' } }],
    type: { kind: 'array', element: { kind: 'float' } } };
  assert.throws(() => encodeFormalConceptDefinition(structured), /text conversion requires a supported scalar/u);
  const unmodeled = scalarProgram();
  unmodeled.sourceLanguage = 'Rust';
  assert.throws(() => encodeFormalConceptDefinition(unmodeled), /Rust Float display/u);
  const supported = encodeFormalConceptDefinition(scalarProgram());
  assert.equal(readFormalConceptDefinition(reload(supported)).model, 'checked typed program');
});

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

test('owned unused local binders normalize while public names and types stay explicit', () => {
  for (const [language, firstSource, secondSource] of [
    ['JavaScript', 'function keep(value,unused){return value;} console.log(keep(1,2));', 'function keep(value,discarded){return value;} console.log(keep(1,2));'],
    ['JavaScript', 'function keep(){const unused=1;return 2;} console.log(keep());', 'function keep(){const discarded=1;return 2;} console.log(keep());'],
    ['Rust', 'fn keep(value:i32,unused:i32)->i32{value} fn main(){println!("{}",keep(1,2));}', 'fn keep(value:i32,discarded:i32)->i32{value} fn main(){println!("{}",keep(1,2));}'],
    ['Rust', 'fn keep()->i32{let unused=1;2} fn main(){println!("{}",keep());}', 'fn keep()->i32{let discarded=1;2} fn main(){println!("{}",keep());}'],
  ]) {
    const first = encodeFormalConceptDefinition(createTypedProgramLinks(firstSource, language));
    const second = encodeFormalConceptDefinition(createTypedProgramLinks(secondSource, language));
    const proof = proveFormalConceptEquivalence(first, second);
    assert.ok(proof, firstSource);
    assert.equal(verifyFormalConceptEquivalence(reload(first), reload(second), reload(proof)), true, secondSource);
    const emit = (definition) => emitTypedProgramLinks(definition.network, definition.network.link(definition.root).references()[0], 'JavaScript').code;
    assert.deepEqual(output(emit(first)), output(emit(second)), 'source-free operational interpretation agrees after local renaming');
    if (language === 'JavaScript') assert.deepEqual(output(emit(first)), output(firstSource), 'the independent source behavior is retained');
  }
  const first = encodeFormalConceptDefinition(createTypedProgramLinks('fn keep(value:i32,unused:i32)->i32{value} fn main(){println!("{}",keep(1,2));}', 'Rust'));
  const changedType = encodeFormalConceptDefinition(createTypedProgramLinks('fn keep(value:i32,unused:i64)->i32{value} fn main(){println!("{}",keep(1,2));}', 'Rust'));
  assert.equal(proveFormalConceptEquivalence(first, changedType), null, 'an unused parameter type remains a public signature constraint');
});

test('disconnected binding decoys cannot hide observable record field names', () => {
  const first = encode('console.log({left:1});');
  const second = encode('console.log({right:1});');
  assert.notDeepEqual(output('console.log({left:1});'), output('console.log({right:1});'));
  assert.equal(proveFormalConceptEquivalence(first, second), null);
  for (const archive of [first, second]) {
    for (const { parent } of archive.network.fieldRelations().filter(({ label }) => label === 'name')) {
      const decoy = archive.network.insertLink([], LinkMetadata.new());
      archive.network.insertFieldRelation(decoy, 'binding reference', parent);
    }
  }
  assert.equal(proveFormalConceptEquivalence(first, second), null, 'unreachable edges are not evidence of a lexical binder');
});

test('a lexical binder aliased with a public type field retains its observable name', () => {
  const definitions = ['left', 'right'].map((name) => encode(`/** @param {{${name}:number}} item @param {number} ${name} */ function keep(item,${name}){return 3;} console.log(3);`));
  for (const definition of definitions) {
    const relations = definition.network.fieldRelations();
    const fn = relations.find(({ label, child }) => label === 'construct kind' && definition.network.link(child).metadata().definition === 'fn').parent;
    const field = (parent, label) => relations.find((relation) => relation.parent.asU64() === parent.asU64() && relation.label === label).child;
    const parameters = definition.network.link(field(fn, 'parameters')).references();
    const publicFields = field(field(parameters[0], 'type'), 'fields');
    definition.network.link(publicFields).setReferences([parameters[1]]);
    assert.equal(readFormalConceptDefinition(definition).model, 'checked typed program', 'the checked graph admits the shared record identity');
  }
  assert.equal(proveFormalConceptEquivalence(...definitions), null, 'binding ownership cannot erase a second public type-field role');
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

test('shared formal archives preserve primitives, recursive references and checked correspondence premises', () => {
  const fixtureFile = 'parity/fixtures/formal-concept-definitions.json';
  const fixture = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));
  let recursive = 0;
  const hasCycle = (graph) => {
    const active = new Set();
    const visited = new Set();
    function visit(node) {
      if (active.has(node)) return true;
      if (visited.has(node)) return false;
      active.add(node);
      for (let edge = graph.offsets[node]; edge < graph.offsets[node + 1]; edge += 1) if (visit(graph.edges[edge])) return true;
      active.delete(node); visited.add(node);
      return false;
    }
    return visit(0);
  };
  for (const definition of fixture.definitions) {
    const value = { network: LinkNetwork.fromLino(definition.archive.links), root: definition.archive.root };
    const wrapper = value.network.link(value.root);
    assert.equal(wrapper.metadata().term, 'formal concept definition');
    assert.equal(wrapper.references().length, 2);
    assert.equal(value.network.link(wrapper.references()[1]).metadata().term, 'formal primitive model');
    assert.deepEqual(readFormalConceptDefinition(value), definition.prepared, definition.name);
    assert.deepEqual(readFormalConceptDefinition(definition.archive), definition.prepared, 'primitive assumptions and shared references survive source-free reload');
    if (definition.name.startsWith('recursive factorial')) {
      recursive += 1;
      assert.equal(hasCycle(definition.prepared), true, 'actual declaration references remain cyclic');
      const incoming = Array(definition.prepared.shapes.length).fill(0);
      for (const target of definition.prepared.edges) incoming[target] += 1;
      assert.ok(incoming.some((count) => count > 1), 'actual shared references retain one identity');
    }
  }
  assert.equal(recursive, 2);
  for (const proof of fixture.proofs) {
    const first = fixture.definitions[proof.first].archive;
    const second = fixture.definitions[proof.second].archive;
    assert.equal(verifyFormalConceptEquivalence(first, second, proof.certificate), true, proof.name);
    const recomputed = proveFormalConceptEquivalence(first, second);
    assert.ok(recomputed, proof.name);
    assert.equal(recomputed.network.toLino(), proof.certificate.links);
    assert.equal(recomputed.root.asU64(), proof.certificate.root);
  }
  for (const rejection of fixture.rejections) {
    assert.throws(() => readFormalConceptDefinition(rejection.archive), undefined, rejection.name);
    assert.equal(proveFormalConceptEquivalence(fixture.definitions[0].archive, rejection.archive), null, rejection.name);
  }
  for (const certificate of fixture.invalidCertificates) assert.equal(verifyFormalConceptEquivalence(fixture.definitions[certificate.first].archive, fixture.definitions[certificate.second].archive, certificate.certificate), false, certificate.name);
  for (const pair of fixture.negativeEquivalences ?? []) assert.equal(proveFormalConceptEquivalence(fixture.definitions[pair.first].archive, fixture.definitions[pair.second].archive), null, pair.name);
  const assertions = {
    'I195-FORMAL-CONCEPT-DEFINITIONS': ['definitionsAreInspectableLinks', 'foundationalPrimitivesExplicit'],
    'I195-FORMAL-CONCEPT-REFERENCES': ['recursiveConceptReferencesPreserved', 'sharedAndCyclicDefinitionsReloaded', 'definitionAssumptionsPreserved'],
    'I195-FORMAL-CONCEPT-MEANING-COMPARISON': ['formalDefinitionsCompared', 'semanticObligationsChecked', 'unprovedMeaningEquivalenceRejected'],
  };
  for (const [requirementId, verified] of Object.entries(assertions)) recordIssue195Observations({ requirementId, suffix: 'behavior', fixtureId: 'formal-concept-definitions', fixtureFile, assertions: verified, testName: 'shared formal archives preserve primitives, recursive references and checked correspondence premises' });
});
