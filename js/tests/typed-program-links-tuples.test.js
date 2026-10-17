import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities, TYPED_PROGRAM_LINKS_SCHEMA as schema } from '../src/typed-program-links.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { applyTupleProgramEdits, runTupleJavaScript, tupleProjectionSyntaxWitness, typedProgramTupleFixture } from '../scripts/generate-typed-program-tuple-fixtures.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/typed-program-tuples.lino', import.meta.url));
const graph = case_ => ({ network: LinkNetwork.fromLino(case_.lino), root: LinkId.fromU64(case_.root) });
function field(network, parent, label) {
  const relation = network.fieldRelations().find(value => value.parent.equals(parent) && value.label === label);
  assert.ok(relation, `missing ${label}`); return relation.child;
}
const scalar = (network, id) => network.link(id).metadata().definition;
function records(network, kind) {
  return network.fieldRelations().filter(relation => relation.label === schema.fields.k && scalar(network, relation.child) === kind).map(relation => relation.parent);
}
function identities(network) {
  return network.fieldRelations().filter(({ label }) => Object.values(schema.references).includes(label))
    .map(relation => [relation.parent.asU64(), relation.label, relation.child.asU64()]);
}

test('independent JavaScript and Rust tuple sources produce deterministic source-free archives', () => {
  assert.deepEqual(typedProgramTupleFixture(), fixture);
  assert.equal(fixture.cases.length, 10);
  for (const case_ of fixture.cases) {
    assert.deepEqual(runTupleJavaScript(case_.oracleSource), case_.output, case_.id);
    const { network, root } = graph(case_);
    assert.equal(decodeTypedProgramLinks(network, root).sourceLanguage, case_.language);
    const javascript = emitTypedProgramLinks(network, root, 'JavaScript').code;
    assert.equal(javascript, case_.javascript);
    assert.deepEqual(runTupleJavaScript(javascript), case_.output, case_.id);
    assert.equal(emitTypedProgramLinks(network, root, 'Rust').code, case_.rust);
    assert.ok([...network.links()].every(link => !link.metadata().definition?.includes(case_.source)), 'the operational graph retains no original source envelope');
  }
});

test('tuple layouts, immutable value models and lexical identities survive LiNo reload', () => {
  for (const case_ of fixture.cases) {
    const { network, root } = graph(case_);
    const program = decodeTypedProgramLinks(network, root);
    const group = program.declarations.get('group');
    if (group) assert.equal(group.ret.kind, 'float', 'parentheses preserve the scalar type');
    if (case_.program === 'unit-and-group') {
      assert.equal(program.declarations.get('unit').ret.kind, 'unit', 'Rust unit remains distinct from an anonymous tuple');
      assert.equal(records(network, 'tuple').length, 0);
      continue;
    }
    const make = program.declarations.get('make');
    assert.equal(make.ret.kind, 'tuple');
    assert.equal(make.ret.elements.length, case_.program === 'one-and-group' ? 1 : case_.program === 'triple-copy' ? 3 : 2);
    for (const tuple of records(network, 'tuple')) {
      assert.equal(scalar(network, field(network, tuple, schema.fields.frozen)), 'true');
      assert.equal(scalar(network, field(network, tuple, schema.fields.productModel ?? 'product model')), 'immutable value product');
      const values = network.link(field(network, tuple, schema.fields.items ?? 'items')).references();
      assert.ok(values.length > 0, 'the empty JS array has no unit-product admission');
    }
    const owned = readTypedProgramBindingIdentities(network, root);
    const bindings = network.fieldRelations().filter(relation => relation.label === schema.references.binding);
    assert.ok(bindings.length > 0);
    assert.ok(bindings.every(relation => owned.some(id => id.equals(relation.child))), 'every product projection refers to its actual lexical binder');
    assert.ok(network.fieldRelations().some(relation => relation.label === schema.references.declaration), 'known closed callers retain function declaration identities');
    assert.ok(records(network, 'tupleIndex').length > 0);
  }
});

test('a semantic tuple literal edit changes both observations without source replay', () => {
  assert.equal(fixture.mutations.length, 1);
  for (const mutation of fixture.mutations) {
    const case_ = fixture.cases.find(value => value.id === mutation.case);
    const { network, root } = graph(case_);
    const before = identities(network);
    applyTupleProgramEdits(network, mutation.edits);
    const reloaded = LinkNetwork.fromLino(network.toLino());
    assert.deepEqual(identities(reloaded), before);
    assert.deepEqual(runTupleJavaScript(mutation.oracleSource), ['false', 'false']);
    const javascript = emitTypedProgramLinks(reloaded, root, 'JavaScript').code;
    assert.equal(javascript, mutation.javascript);
    assert.deepEqual(runTupleJavaScript(javascript), mutation.output);
    assert.equal(emitTypedProgramLinks(reloaded, root, 'Rust').code, mutation.rust);
  }
});

test('malformed tuple layouts, index types and contradictory immutable models refuse all targets', () => {
  assert.equal(fixture.invalid.length, 9);
  for (const mutation of fixture.invalid) {
    const case_ = fixture.cases.find(value => value.id === mutation.case);
    const { network, root } = graph(case_);
    applyTupleProgramEdits(network, mutation.edits);
    const reloaded = LinkNetwork.fromLino(network.toLino());
    assert.throws(() => decodeTypedProgramLinks(reloaded, root), undefined, mutation.id);
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(reloaded, root, target), undefined, `${mutation.id}: ${target}`);
  }
});

test('observable arrays, external callers, borrowing and non-Copy products refuse source admission', () => {
  assert.equal(fixture.refusals.length, 23);
  for (const case_ of fixture.refusals) assert.throws(() => createTypedProgramLinks(case_.source, case_.language), undefined, case_.id);
});

test('Lean and Rocq products retain the explicit singleton arity limitation', () => {
  for (const case_ of fixture.cases) for (const [target, emit] of [['Lean', emitLean], ['Rocq', emitRocq]]) {
    const { network, root } = graph(case_);
    const program = decodeTypedProgramLinks(network, root);
    if (case_.program === 'one-and-group') assert.throws(() => emit(program), /immutable tuple arity/u, `${case_.id}: ${target}`);
    else assert.ok(emit(program).text.length > 0, `${case_.id}: ${target}`);
  }
});

test('chained Rust product projections retain both exact UTF-16 spans after a Unicode comment', () => {
  assert.deepEqual(tupleProjectionSyntaxWitness(), fixture.syntaxWitness);
  assert.equal(fixture.syntaxWitness.source.slice(fixture.syntaxWitness.inner.start, fixture.syntaxWitness.inner.end), 'pair.0');
  assert.equal(fixture.syntaxWitness.source.slice(fixture.syntaxWitness.outer.start, fixture.syntaxWitness.outer.end), 'pair.0.1');
});
