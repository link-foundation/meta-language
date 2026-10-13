import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustBorrowedCallInterfaceFixture } from '../scripts/generate-rust-borrowed-call-interface-fixtures.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, encodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { emitRust } from '../src/translation/emit-rust.js';
const fixture = readProjectData(new URL('../../parity/fixtures/rust-borrowed-call-interface.lino', import.meta.url));
const capture = { rustSourceSignatureModel: fixture.sourceSignatureModel };
const options = { rustSourceHeaderEmissionModel: fixture.model };
const leafOptions = { rustSourceHeaderEmissionModel: 'primitive borrowed Rust function header 1' };
const reload = value => ({ network: LinkNetwork.fromLino(value.links), root: value.root });
const scalar = value => value === '-0' ? -0 : value === 'NaN' ? NaN : value === 'Infinity' ? Infinity : value;
const run = (code, name, args) => vm.runInNewContext(`${code}\n${name}(...argumentsList)`, { argumentsList: args.map(scalar) }, { timeout: 1000 });
const replay = definitions => createTypedProgramLinks(`${definitions.join('\n')}\nfn main(){}`, 'Rust', capture);
const field = (network, parent, name) => network.fieldRelations().find(value => value.parent.equals(parent) && value.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name));

test('twelve actual leaf/caller programs retain headers and ordered once-only scalar bindings after source-free reload', () => {
  assert.deepEqual(rustBorrowedCallInterfaceFixture(), fixture);
  assert.equal(fixture.cases.length, 12);
  for (const value of fixture.cases) {
    const { network, root } = reload(value);
    const checked = decodeTypedProgramLinks(network, root);
    const before = network.toLino();
    const emitted = emitTypedProgramLinks(network, root, 'Rust', options);
    for (const header of value.headers) assert.ok(emitted.code.includes(`${header} {`), value.id);
    const definition = emitRust(checked, { sourceHeaderModel: fixture.model }).definitions.find(text => text.includes(`pub fn ${value.caller}`));
    let after = 0;
    for (const binding of value.bindings) {
      const index = definition.indexOf(binding);
      assert.ok(index >= after, `${value.id}: argument materialization order`);
      assert.equal(definition.split(binding).length - 1, 1, `${value.id}: one materialization`);
      after = index + binding.length;
    }
    assert.ok(definition.indexOf(value.invocation) >= after, `${value.id}: borrow after evaluation`);
    assert.equal(definition.split(value.invocation).length - 1, 1, `${value.id}: one invocation`);
    assert.equal(network.toLino(), before, `${value.id}: emission preserves the original archive`);
  }
});

test('actual emitted definitions reimport with both owned signatures and execute equivalent primitive caller results', () => {
  for (const value of fixture.cases) {
    const { network, root } = reload(value);
    const checked = decodeTypedProgramLinks(network, root);
    const emitted = emitRust(checked, { sourceHeaderModel: fixture.model });
    const imported = replay(emitted.definitions);
    const restored = LinkNetwork.fromLino(imported.network.toLino());
    const reread = decodeTypedProgramLinks(restored, imported.root);
    for (const [name, declaration] of checked.declarations) assert.deepEqual(reread.declarations.get(name).sourceSignature, declaration.sourceSignature, `${value.id}:${name}`);
    const originalCode = emitTypedProgramLinks(network, root, 'JavaScript').code;
    const replayedCode = emitTypedProgramLinks(restored, imported.root, 'JavaScript').code;
    for (let index = 0; index < value.inputs.length; index += 1) {
      const expected = scalar(value.expected[index]);
      assert.ok(Object.is(run(originalCode, value.caller, value.inputs[index]), expected), `${value.id}:${index}: actual source`);
      assert.ok(Object.is(run(replayedCode, value.caller, value.inputs[index]), expected), `${value.id}:${index}: emitted source`);
    }
  }
});

test('existing defaults and leaf-model refusals remain exact; explicit model and target errors remain precise', () => {
  for (const value of fixture.cases) {
    const { network, root } = reload(value);
    const program = decodeTypedProgramLinks(network, root);
    assert.equal(emitTypedProgramLinks(network, root, 'Rust').code, emitRust(program).text, value.id);
    assert.equal(emitTypedProgramLinks(network, root, 'Rust', {}).code, emitRust(program).text, value.id);
    assert.throws(() => emitTypedProgramLinks(network, root, 'Rust', leafOptions), /calls and effects require a distinct interface transformation proof/u, value.id);
  }
  const value = reload(fixture.cases[0]);
  assert.throws(() => emitTypedProgramLinks(value.network, value.root, 'JavaScript', options), /requires Rust target/u);
  assert.throws(() => emitTypedProgramLinks(value.network, value.root, 'Rust', { rustSourceHeaderEmissionModel: 'unknown' }), /Rust source header emission model/u);
  const absent = createTypedProgramLinks(fixture.cases[0].source, 'Rust');
  assert.throws(() => emitTypedProgramLinks(absent.network, absent.root, 'Rust', options), /owned source signature facet/u);
});

test('twelve actual unsupported programs refuse static target borrows, escapes, effects and wider call graphs', () => {
  assert.equal(fixture.refusals.length, 12);
  for (const value of fixture.refusals) {
    const { network, root } = reload(value);
    assert.throws(() => emitTypedProgramLinks(network, root, 'Rust', options), /Rust (?:borrowed call interface|source header emission)/u, value.id);
  }
});

test('source-free scalar edits change caller behavior while both owned source interfaces remain intact', () => {
  const { network, root } = reload(fixture.cases[0]);
  const before = decodeTypedProgramLinks(network, root);
  const scalarLink = network.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string && link.metadata().definition === before.declarations.get('leaf').body.right.value);
  assert.ok(scalarLink);
  scalarLink.setMetadata(scalarLink.metadata().withDefinition('2.0'));
  const restored = LinkNetwork.fromLino(network.toLino());
  const edited = decodeTypedProgramLinks(restored, root);
  for (const [name, declaration] of before.declarations) assert.deepEqual(edited.declarations.get(name).sourceSignature, declaration.sourceSignature, name);
  const imported = replay(emitRust(edited, { sourceHeaderModel: fixture.model }).definitions);
  assert.equal(run(emitTypedProgramLinks(imported.network, imported.root, 'JavaScript').code, 'caller', [3]), 5);
});

test('source-free nested-reference and ordered-lifetime edits change the actual target header and call borrowing together', () => {
  const value = reload(fixture.cases[0]);
  const program = decodeTypedProgramLinks(value.network, value.root);
  const signature = program.declarations.get('leaf').sourceSignature;
  signature.parameters[0].wrappers.push('reference');
  signature.parameters[0].lifetimes = ['outer', 'inner'];
  signature.lifetimeBindings = ['outer', 'inner'];
  const edited = encodeTypedProgramLinks(program);
  const network = LinkNetwork.fromLino(edited.network.toLino());
  const emitted = emitTypedProgramLinks(network, edited.root, 'Rust', options);
  assert.match(emitted.code, /pub fn leaf<'outer, 'inner>\(value: &'outer &'inner f64\)/u);
  assert.ok(emitted.code.includes('crate::leaf(&(&(ml_borrowed_argument)))'));
  const imported = replay(emitRust(decodeTypedProgramLinks(network, edited.root), { sourceHeaderModel: fixture.model }).definitions);
  assert.deepEqual(decodeTypedProgramLinks(imported.network, imported.root).declarations.get('leaf').sourceSignature, signature);
  assert.equal(run(emitTypedProgramLinks(imported.network, imported.root, 'JavaScript').code, 'caller', [3]), 4);
});

test('redirected actual call declaration links cannot bypass owned leaf or callee type checks', () => {
  for (const targetName of ['caller', 'other']) {
    const source = 'fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&value)}fn other(value:&bool)->f64{if *value{1.0}else{2.0}}fn main(){}';
    const archive = createTypedProgramLinks(source, 'Rust', capture);
    const network = LinkNetwork.fromLino(archive.network.toLino());
    const directory = field(network, network.link(archive.root).references()[0], 'declarations').child;
    const declarations = new Map(network.link(directory).references().map(id => {
      const [name, value] = network.link(id).references(); return [network.link(name).metadata().definition, value];
    }));
    const call = field(network, declarations.get('caller'), 'body').child;
    const relation = field(network, call, TYPED_PROGRAM_LINKS_SCHEMA.references.declaration).link;
    const references = relation.references(); references[2] = declarations.get(targetName); relation.setReferences(references);
    const reloaded = LinkNetwork.fromLino(network.toLino());
    assert.throws(() => emitTypedProgramLinks(reloaded, archive.root, 'Rust', options), targetName === 'caller' ? /owned pure leaf/u : /call argument type/u, targetName);
  }
});

test('unrepresented call fields and malformed target interfaces refuse through actual source-free archives', () => {
  const value = reload(fixture.cases[0]);
  for (const mutation of [
    program => { program.declarations.get('caller').body.optional = false; },
    program => { program.declarations.get('caller').body.args = []; },
    program => { program.declarations.get('leaf').sourceSignature.parameters[0].lifetimes = []; },
    program => { program.declarations.get('leaf').sourceSignature.result.wrappers = ['reference']; program.declarations.get('leaf').sourceSignature.result.lifetimes = [null]; },
    program => { program.declarations.get('leaf').sourceSignature.parameters[0].owner = { kind: 'primitive', path: ['Alias'], arguments: [] }; },
    program => { program.declarations.get('leaf').params[0].optional = false; },
  ]) {
    const program = decodeTypedProgramLinks(value.network, value.root);
    mutation(program);
    assert.throws(() => {
      const archive = encodeTypedProgramLinks(program);
      emitTypedProgramLinks(LinkNetwork.fromLino(archive.network.toLino()), archive.root, 'Rust', options);
    });
  }
});

test('both selected header models refuse unrepresented caller declaration facets while defaults remain exact', () => {
  const value = reload(fixture.cases[0]);
  for (const name of ['leaf', 'caller']) {
    for (const [field, scalar] of [['async', true], ['unsafe', true], ['opaqueInterface', false]]) {
      const program = decodeTypedProgramLinks(value.network, value.root);
      const defaultCode = emitRust(program).text;
      program.declarations.get(name)[field] = scalar;
      const archive = encodeTypedProgramLinks(program);
      const network = LinkNetwork.fromLino(archive.network.toLino());
      assert.equal(emitTypedProgramLinks(network, archive.root, 'Rust').code, defaultCode);
      assert.throws(() => emitTypedProgramLinks(network, archive.root, 'Rust', options), /unrepresented source interface fields/u, `${name}:${field}`);
      assert.throws(() => emitTypedProgramLinks(network, archive.root, 'Rust', leafOptions), /unrepresented source interface fields/u);
    }
  }
});

test('both selected models refuse source-free parameter spans without dropping supported source-node spans', () => {
  const value = reload(fixture.cases[0]);
  for (const name of ['leaf', 'caller']) {
    for (const span of [null, { start: 0, end: 1 }]) {
      const program = decodeTypedProgramLinks(value.network, value.root);
      const defaultCode = emitRust(program).text;
      const sourceSpan = program.declarations.get(name).span;
      program.declarations.get(name).params[0].span = span;
      const archive = encodeTypedProgramLinks(program);
      const network = LinkNetwork.fromLino(archive.network.toLino());
      assert.deepEqual(decodeTypedProgramLinks(network, archive.root).declarations.get(name).span, sourceSpan);
      assert.equal(emitTypedProgramLinks(network, archive.root, 'Rust').code, defaultCode);
      for (const selection of [options, leafOptions]) assert.throws(() => emitTypedProgramLinks(network, archive.root, 'Rust', selection), /parameter defaults, guards and unrepresented fields/u, name);
    }
  }
});
