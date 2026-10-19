import assert from 'node:assert/strict';
import vm from 'node:vm';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { TYPED_PROGRAM_LINKS_SCHEMA, createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptProgramContainerFields } from '../src/typed-program-link-rules.js';

const source = 'fn add(a: i32, b: i32) -> i32 { a + b } fn main() { println!("{}", add(2, 3)); }';
const reload = archive => ({ ...archive, network: LinkNetwork.fromLino(archive.network.toLino()) });

for (const kind of ['array', 'map']) test(`named field relations on a program ${kind} are refused before materialization`, () => {
  let archive = reload(createTypedProgramLinks(source, 'Rust'));
  const container = archive.network.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.containers[kind]);
  const string = archive.network.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string);
  assert.ok(container);
  assert.ok(string);
  archive.network.insertFieldRelation(container.id(), 'unexpected field', string.id());
  archive = reload(archive);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /ordered program container contains named fields/u);
  for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), /ordered program container contains named fields/u);
});

test('valid positional lists and mappings retain behavior after reload with record fields and root provenance', () => {
  let archive = reload(createTypedProgramLinks(source, 'Rust', { provenance: { project: 'container shape regression', stages: ['source', 'links'] } }));
  const record = archive.network.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.containers.record);
  const string = archive.network.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string);
  archive.network.insertFieldRelation(record.id(), 'review note', string.id());
  archive = reload(archive);
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.ok(program.declarations instanceof Map);
  assert.ok(Array.isArray(program.items));
  assert.equal(typeof program.reviewNote, 'string');
  assert.ok(archive.network.fieldRelations().some(relation => relation.parent.equals(archive.root) && relation.label === 'provenance'));
  const output = [];
  vm.runInNewContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  assert.deepEqual(output, ['5']);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /fn add/u);
});

test('the generated shared container decision admits named record fields and only positional list/map content', () => {
  for (const kind of Object.keys(TYPED_PROGRAM_LINKS_SCHEMA.containers)) {
    assert.equal(acceptProgramContainerFields(kind, false), true, kind);
    assert.equal(acceptProgramContainerFields(kind, true), kind === 'record', kind);
  }
  assert.equal(acceptProgramContainerFields('unknown', false), false);
  assert.equal(acceptProgramContainerFields('unknown', true), false);
});
