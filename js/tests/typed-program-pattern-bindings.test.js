import assert from 'node:assert/strict';
import vm from 'node:vm';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities } from '../src/typed-program-links.js';
import { parseLean } from '../src/translation/lean.js';
import { checkProgram } from '../src/translation/check.js';

const source = 'enum Choice { Value(i64), None }\nfn pick(value: Choice, shadow: i64) -> i64 { match value { Choice::Value(shadow) => shadow, Choice::None => shadow } }\nfn main() {}';
const reload = archive => ({ root: archive.root, network: LinkNetwork.fromLino(archive.network.toLino()) });
const field = (network, parent, label) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === label)?.child;
function replaceField(network, parent, label, child) {
  const relation = network.fieldRelations().find(value => value.parent.equals(parent) && value.label === label);
  assert.ok(relation, label);
  relation.link.setReferences([parent, relation.link.references()[1], child]);
}
function shape(archive, name = 'pick') {
  const { network } = archive;
  const declaration = network.fieldRelations().find(relation => relation.label === 'full name' && network.link(relation.child).metadata().definition === name).parent;
  const parameters = network.link(field(network, declaration, 'parameters')).references();
  const match = field(network, declaration, 'body');
  const cases = network.link(field(network, match, 'cases')).references();
  const pattern = field(network, cases[0], 'pattern');
  const bindings = network.link(field(network, pattern, 'pattern bindings')).references();
  return { declaration, parameters, match, cases, pattern, bindings };
}
function call(archive, expression) {
  const context = vm.createContext({});
  vm.runInContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code, context, { timeout: 1000 });
  return vm.runInContext(expression, context, { timeout: 1000 });
}

test('constructor pattern bindings retain owned identity across shadowing and source-free rename', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const { network } = archive;
  const graph = shape(archive);
  const firstBody = field(network, graph.cases[0], 'body');
  const secondBody = field(network, graph.cases[1], 'body');
  assert.ok(field(network, firstBody, 'binding reference').equals(graph.bindings[0]));
  assert.ok(field(network, secondBody, 'binding reference').equals(graph.parameters[1]));
  assert.ok(!graph.bindings[0].equals(graph.parameters[1]));
  assert.equal(call(archive, 'pick({$:"Value",field0:3n},9n)'), 3n);
  assert.equal(call(archive, 'pick({$:"None"},9n)'), 9n);
  const name = network.link(field(network, graph.bindings[0], 'name'));
  name.setMetadata(name.metadata().withDefinition('renamed'));
  const edited = reload(archive);
  const program = decodeTypedProgramLinks(edited.network, edited.root);
  assert.deepEqual(program.declarations.get('pick').body.cases[0].pattern.binds, ['renamed']);
  assert.equal(program.declarations.get('pick').body.cases[0].body.name, 'renamed');
  assert.equal(program.declarations.get('pick').body.cases[1].body.name, 'shadow');
  assert.equal(call(edited, 'pick({$:"Value",field0:3n},9n)'), 3n);
  assert.equal(call(edited, 'pick({$:"None"},9n)'), 9n);
  assert.ok(readTypedProgramBindingIdentities(edited.network, edited.root).some(id => id.equals(graph.bindings[0])));
  assert.ok(network.links().every(link => !link.metadata().definition?.includes(source)));
});

test('pattern references cannot target an outer binding, a wrong-kind record or a dangling value', () => {
  for (const mode of ['outer', 'wrong-kind', 'dangling']) {
    const archive = reload(createTypedProgramLinks(source, 'Rust'));
    const graph = shape(archive);
    const body = field(archive.network, graph.cases[0], 'body');
    const target = mode === 'outer' ? graph.parameters[1] : mode === 'wrong-kind' ? graph.pattern : graph.bindings[0];
    replaceField(archive.network, body, 'binding reference', target);
    if (mode === 'dangling') archive.network.deleteLink(target);
    assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /scope|unbound|dangling|owned|named declaration/u, mode);
  }
});

test('pattern binder types, match result types and coverage reuse the existing checked rules', () => {
  for (const mode of ['binder', 'result', 'coverage']) {
    const archive = reload(createTypedProgramLinks(source, 'Rust'));
    const graph = shape(archive);
    if (mode === 'binder') replaceField(archive.network, graph.bindings[0], 'type', field(archive.network, graph.parameters[0], 'type'));
    if (mode === 'result') replaceField(archive.network, graph.match, 'type', field(archive.network, graph.parameters[0], 'type'));
    if (mode === 'coverage') archive.network.link(field(archive.network, graph.match, 'cases')).setReferences([graph.cases[0]]);
    assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /pattern binding type|match result type|misses None/u, mode);
  }
});

test('a pattern binder record cannot be owned by sibling cases with identical names and types', () => {
  const two = source.replace('None }', 'Other(i64) }').replace('Choice::None => shadow', 'Choice::Other(shadow) => shadow');
  const archive = reload(createTypedProgramLinks(two, 'Rust'));
  const graph = shape(archive);
  const otherPattern = field(archive.network, graph.cases[1], 'pattern');
  archive.network.link(field(archive.network, otherPattern, 'pattern bindings')).setReferences([graph.bindings[0]]);
  replaceField(archive.network, field(archive.network, graph.cases[1], 'body'), 'binding reference', graph.bindings[0]);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /multiple lexical owners/u);
});

test('natural successor bindings preserve predecessor identity and a shadowed outer name', () => {
  const program = checkProgram(parseLean('def predecessor (n : Nat) : Nat :=\n  match n with\n  | Nat.zero => 0\n  | Nat.succ n => n\n'));
  const archive = reload(encodeTypedProgramLinks(program));
  const declaration = archive.network.fieldRelations().find(relation => relation.label === 'full name' && archive.network.link(relation.child).metadata().definition === 'predecessor').parent;
  const match = field(archive.network, declaration, 'body');
  const cases = archive.network.link(field(archive.network, match, 'cases')).references();
  const pattern = field(archive.network, cases[1], 'pattern');
  const binding = field(archive.network, pattern, 'name');
  assert.ok(field(archive.network, field(archive.network, cases[1], 'body'), 'binding reference').equals(binding));
  assert.equal(call(archive, 'predecessor(0n)'), 0n);
  assert.equal(call(archive, 'predecessor(4n)'), 3n);
  const name = archive.network.link(field(archive.network, binding, 'name'));
  name.setMetadata(name.metadata().withDefinition('previous'));
  const edited = reload(archive);
  assert.equal(decodeTypedProgramLinks(edited.network, edited.root).declarations.get('predecessor').body.cases[1].body.name, 'previous');
  assert.equal(call(edited, 'predecessor(4n)'), 3n);
  assert.ok(readTypedProgramBindingIdentities(edited.network, edited.root).some(id => id.equals(binding)));
});

test('an injected effects container cannot bypass match coverage', () => {
  const archive = createTypedProgramLinks(source, 'Rust');
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  program.declarations.get('pick').body.cases.pop();
  program.declarations.get('pick').body.effects = [];
  assert.throws(() => encodeTypedProgramLinks(program), /misses None/u);
});
