import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { applyAlgebraicProgramEdits, typedProgramAlgebraicFixture } from '../scripts/generate-typed-program-algebraic-fixtures.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/typed-program-algebraic.json', import.meta.url));
function run(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
  return output;
}
const graph = case_ => ({ network: LinkNetwork.fromLino(case_.lino), root: LinkId.fromU64(case_.root) });
function field(network, owner, label) {
  const found = network.fieldRelations().find(relation => relation.parent.equals(owner) && relation.label === label);
  assert.ok(found, `missing ${label}`); return found.child;
}

test('shared algebraic fixtures reproduce independent source behavior and deterministic linked snapshots', () => {
  assert.deepEqual(typedProgramAlgebraicFixture(), fixture);
  assert.equal(fixture.cases.length, 6);
  assert.deepEqual([...new Set(fixture.cases.map(case_ => case_.program))], ['choice', 'tree', 'natural']);
  for (const case_ of fixture.cases) {
    assert.deepEqual(run(case_.oracleSource), case_.output, case_.id);
    const archive = graph(case_);
    const program = decodeTypedProgramLinks(archive.network, archive.root);
    assert.equal(program.items[0], program.declarations.get(program.items[0].fullName));
    assert.equal(program.sourceLanguage, case_.language, 'numeric and object models retain their original language context');
    const emitted = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript');
    assert.deepEqual(run(emitted.code), case_.output, case_.id);
    assert.equal(emitted.code, case_.javascript);
    assert.equal(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, case_.rust);
    const sourceArchive = createTypedProgramLinks(case_.source, case_.language);
    assert.deepEqual(run(emitTypedProgramLinks(LinkNetwork.fromLino(sourceArchive.network.toLino()), sourceArchive.root, 'JavaScript').code), case_.output);
    assert.ok([...archive.network.links()].every(link => !link.metadata().definition?.includes(case_.source)), 'the graph retains no source envelope');
  }
});

test('datatype, constructor and case binder identities survive source-free recursive reload', () => {
  for (const case_ of fixture.cases) {
    const { network, root } = graph(case_);
    const relations = network.fieldRelations();
    const datatypes = relations.filter(relation => relation.label === 'construct kind' && network.link(relation.child).metadata().definition === 'data').map(relation => relation.parent);
    const constructors = datatypes.flatMap(datatype => network.link(field(network, datatype, 'constructors')).references());
    const datatypeReferences = relations.filter(({ label }) => label === TYPED_PROGRAM_LINKS_SCHEMA.references.datatype);
    const constructorReferences = relations.filter(({ label }) => label === TYPED_PROGRAM_LINKS_SCHEMA.references.constructor);
    assert.ok(datatypeReferences.length > 1);
    assert.ok(constructorReferences.length > 1);
    assert.ok(datatypeReferences.every(relation => datatypes.some(id => id.equals(relation.child))));
    assert.ok(constructorReferences.every(relation => constructors.some(id => id.equals(relation.child))));
    const formerLabel = Object.entries(TYPED_PROGRAM_LINKS_SCHEMA.referenceAliases).find(([, role]) => role === 'datatype')?.[0];
    assert.ok(formerLabel, 'the former data type relation spelling remains an explicit schema alias');
    const legacy = graph(case_);
    const label = datatypeReferences[0].link.references()[1];
    legacy.network.link(label).setMetadata(legacy.network.link(label).metadata().withTerm(formerLabel));
    assert.deepEqual(run(emitTypedProgramLinks(legacy.network, legacy.root, 'JavaScript').code), case_.output);
    const duplicate = graph(case_);
    duplicate.network.insertFieldRelation(datatypeReferences[0].parent, formerLabel, datatypeReferences[0].child);
    assert.throws(() => decodeTypedProgramLinks(duplicate.network, duplicate.root), undefined, 'canonical and former aliases cannot duplicate one reference role');
    const owned = readTypedProgramBindingIdentities(network, root);
    const lists = relations.filter(({ label }) => label === 'pattern bindings');
    const slots = lists.flatMap(relation => network.link(relation.child).references());
    const binders = slots.filter(id => network.link(id).metadata().term === 'program record');
    assert.ok(binders.length > 0);
    assert.ok(binders.every(id => owned.some(binding => binding.equals(id))), 'pattern bindings belong to actual checked cases');
    if (case_.program === 'choice') {
      const variables = relations.filter(({ label }) => label === TYPED_PROGRAM_LINKS_SCHEMA.references.binding);
      assert.ok(binders.some(id => variables.some(relation => relation.child.equals(id))), 'pattern body variables reference their actual case records');
      const pairSlots = lists.map(relation => network.link(relation.child).references()).find(values => values.length === 2);
      assert.ok(pairSlots && !variables.some(relation => relation.child.equals(pairSlots[1])), 'the ignored second constructor field has no body variable use');
    } else {
      assert.ok(datatypeReferences.filter(relation => relation.child.equals(datatypes[0])).length >= 3, 'recursive field and value types reuse the same declared datatype identity');
    }
  }
});

test('linked algebraic edits preserve identities and change execution without saved source', () => {
  assert.equal(fixture.mutations.length, 6);
  for (const mutation of fixture.mutations) {
    const case_ = fixture.cases.find(value => value.id === mutation.case);
    const { network, root } = graph(case_);
    const before = network.fieldRelations().filter(({ label }) => label.endsWith('reference')).map(relation => [relation.parent.asU64(), relation.label, relation.child.asU64()]);
    applyAlgebraicProgramEdits(network, mutation.edits);
    const reloaded = LinkNetwork.fromLino(network.toLino());
    if (mutation.id === 'ignored-constructor-slot') {
      const slots = reloaded.link(LinkId.fromU64(mutation.edits[0].link)).references();
      assert.equal(reloaded.link(slots[1]).metadata().term, 'null value', 'an explicit ignored slot survives reload');
    }
    assert.deepEqual(reloaded.fieldRelations().filter(({ label }) => label.endsWith('reference')).map(relation => [relation.parent.asU64(), relation.label, relation.child.asU64()]), before,
      'renames and literal edits retain all binding and declaration identities');
    assert.deepEqual(run(mutation.oracleSource), mutation.output);
    const javascript = emitTypedProgramLinks(reloaded, root, 'JavaScript').code;
    assert.equal(javascript, mutation.javascript);
    assert.deepEqual(run(javascript), mutation.output, mutation.id);
    assert.equal(emitTypedProgramLinks(reloaded, root, 'Rust').code, mutation.rust);
  }
});

test('malformed algebraic identities, bindings, types and match coverage refuse decoding and emission', () => {
  assert.equal(fixture.invalid.length, 13);
  for (const mutation of fixture.invalid) {
    const case_ = fixture.cases.find(value => value.id === mutation.case);
    const { network, root } = graph(case_);
    applyAlgebraicProgramEdits(network, mutation.edits);
    assert.throws(() => decodeTypedProgramLinks(network, root), undefined, mutation.id);
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(network, root, target), undefined, `${mutation.id}: ${target}`);
  }
});
