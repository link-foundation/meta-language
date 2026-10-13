import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustSourceHeaderEmissionFixture } from '../scripts/generate-rust-source-header-emission-fixtures.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, encodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptRustSourceHeaderLifetimeCharacter } from '../src/translation/rust-source-header-emission-rules.js';
const fixture = readProjectData(new URL('../../parity/fixtures/rust-source-header-emission.lino', import.meta.url));
const capture = { rustSourceSignatureModel: fixture.sourceSignatureModel };
const options = { rustSourceHeaderEmissionModel: fixture.model };
const reload = value => ({ network: LinkNetwork.fromLino(value.links), root: value.root });
const scalar = value => value === '-0' ? -0 : value === 'NaN' ? NaN : value === 'Infinity' ? Infinity : value;
const run = (code, name, args) => vm.runInNewContext(`${code}\n${name}(...argumentsList)`, { argumentsList: args.map(scalar) }, { timeout: 1000 });
const field = (network, parent, name) => network.fieldRelations().find(value => value.parent.equals(parent) && value.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name));

test('twelve independently specified headers retain reference/lifetime order with checked scoped copies after source-free reload', () => {
  assert.deepEqual(rustSourceHeaderEmissionFixture(), fixture);
  assert.equal(fixture.cases.length, 12);
  for (const value of fixture.cases) {
    const { network, root } = reload(value);
    const checked = decodeTypedProgramLinks(network, root);
    const before = network.toLino();
    const emitted = emitTypedProgramLinks(network, root, 'Rust', options);
    assert.ok(emitted.code.includes(`${value.header} {`), value.id);
    for (const copy of value.copies) assert.ok(emitted.code.includes(`    ${copy}`), value.id);
    assert.equal(network.toLino(), before, 'emission preserves the actual archive');
    const definitions = emitRust(checked, { sourceHeaderModel: fixture.model }).definitions.join('\n');
    const reimported = createTypedProgramLinks(`${definitions}\nfn main(){}`, 'Rust', capture);
    const restored = LinkNetwork.fromLino(reimported.network.toLino());
    const reread = decodeTypedProgramLinks(restored, reimported.root);
    assert.deepEqual(reread.declarations.get(value.functionName).sourceSignature, checked.declarations.get(value.functionName).sourceSignature, value.id);
    const code = emitTypedProgramLinks(restored, reimported.root, 'JavaScript').code;
    for (let index = 0; index < value.inputs.length; index += 1) assert.ok(Object.is(run(code, value.functionName, value.inputs[index]), scalar(value.expected[index])), `${value.id}:${index}`);
  }
});

test('existing captured-profile defaults remain exact and explicit unknown models or wrong targets refuse', () => {
  for (const value of fixture.cases) {
    const { network, root } = reload(value);
    const program = decodeTypedProgramLinks(network, root);
    assert.equal(emitTypedProgramLinks(network, root, 'Rust').code, emitRust(program).text, value.id);
    assert.equal(emitTypedProgramLinks(network, root, 'Rust', {}).code, emitRust(program).text, value.id);
  }
  const value = reload(fixture.cases[1]);
  assert.match(emitTypedProgramLinks(value.network, value.root, 'Rust').code, /value: f64/u);
  assert.throws(() => emitTypedProgramLinks(value.network, value.root, 'Rust', { rustSourceHeaderEmissionModel: 'unknown' }), /Rust source header emission model/u);
  assert.throws(() => emitTypedProgramLinks(value.network, value.root, 'JavaScript', options), /requires Rust target/u);
  const absent = createTypedProgramLinks(fixture.cases[1].source, 'Rust');
  assert.throws(() => emitTypedProgramLinks(absent.network, absent.root, 'Rust', options), /owned source signature facet/u);
});

test('eight unsupported source interfaces or effects retain honest selected-profile refusals', () => {
  assert.equal(fixture.refusals.length, 8);
  for (const value of fixture.refusals) {
    const { network, root } = reload(value);
    assert.throws(() => emitTypedProgramLinks(network, root, 'Rust', options), /Rust source header emission/u, value.id);
  }
});

test('malformed owned source facts and redirected body binder identities refuse before header rendering', () => {
  const original = readProjectData(new URL('../../parity/fixtures/rust-source-signatures.lino', import.meta.url));
  for (const value of original.malformed) assert.throws(() => emitTypedProgramLinks(LinkNetwork.fromLino(value.archive), value.root, 'Rust', options), undefined, value.id);
  const archive = createTypedProgramLinks('fn subtract(first:&f64,second:&f64)->f64{*first-*second}fn other(first:&f64)->f64{*first}fn main(){}', 'Rust', capture);
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const directory = field(network, network.link(archive.root).references()[0], 'declarations').child;
  const declarations = new Map(network.link(directory).references().map(id => {
    const [name, value] = network.link(id).references(); return [network.link(name).metadata().definition, value];
  }));
  const subtraction = field(network, declarations.get('subtract'), 'body').child;
  const left = field(network, subtraction, 'left').child;
  const binding = field(network, left, TYPED_PROGRAM_LINKS_SCHEMA.references.binding).link;
  const otherParameter = network.link(field(network, declarations.get('other'), 'params').child).references()[0];
  const references = binding.references(); references[2] = otherParameter; binding.setReferences(references);
  assert.throws(() => emitTypedProgramLinks(network, archive.root, 'Rust', options), /the linked variable targets a declaration outside its scope/u);
});

test('source-free value edits change emitted behavior while primitive reference facts remain intact', () => {
  const archive = createTypedProgramLinks('fn add_one(value:&f64)->f64{*value+1.0}fn main(){}', 'Rust', capture);
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const before = decodeTypedProgramLinks(network, archive.root).declarations.get('add_one').sourceSignature;
  const literal = decodeTypedProgramLinks(network, archive.root).declarations.get('add_one').body.right.value;
  const one = network.links().find(value => value.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string && value.metadata().definition === literal);
  assert.ok(one);
  one.setMetadata(one.metadata().withDefinition('2.0'));
  const restored = LinkNetwork.fromLino(network.toLino());
  const program = decodeTypedProgramLinks(restored, archive.root);
  assert.deepEqual(program.declarations.get('add_one').sourceSignature, before);
  const definitions = emitRust(program, { sourceHeaderModel: fixture.model }).definitions.join('\n');
  assert.match(definitions, /value: &f64/u);
  const reimported = createTypedProgramLinks(`${definitions}\nfn main(){}`, 'Rust', capture);
  const code = emitTypedProgramLinks(reimported.network, reimported.root, 'JavaScript').code;
  assert.equal(run(code, 'add_one', [3]), 5);
});

test('custom/qualified primitive owners, generated functions and resource context flags cannot bypass source admission', () => {
  const value = reload(fixture.cases[1]);
  for (const owner of [{ kind: 'declaration', path: ['Alias'], arguments: [] }, { kind: 'primitive', path: ['core', 'f64'], arguments: [] }]) {
    const program = decodeTypedProgramLinks(value.network, value.root);
    program.declarations.get('keep').sourceSignature.parameters[0].owner = owner;
    assert.throws(() => encodeTypedProgramLinks(program), /source|primitive|value type/u);
  }
  const original = checkProgram(parseRust(fixture.cases[1].source, { sourceSignatureModel: fixture.sourceSignatureModel }));
  for (const flag of ['outputThreaded', 'abortsThreaded', 'referenceSemantics', 'utf16Strings']) {
    const program = { ...original, [flag]: true };
    assert.throws(() => emitRust(program, { sourceHeaderModel: fixture.model }), /Rust source header emission/u, flag);
  }
  const generated = decodeTypedProgramLinks(value.network, value.root);
  generated.declarations.get('keep').generated = true;
  assert.throws(() => emitRust(generated, { sourceHeaderModel: fixture.model }), /original leaf/u);
});

test('editable lifetime identifiers cannot inject syntax through the selected header renderer', () => {
  const value = reload(fixture.cases.find(value => value.id === 'named-lifetime'));
  for (const text of ['', 'a>\nfn injected()', '1bad', 'non ASCII α']) {
    const program = decodeTypedProgramLinks(value.network, value.root);
    const signature = program.declarations.get('keep').sourceSignature;
    signature.lifetimeBindings[0] = text;
    signature.parameters[0].lifetimes[0] = text;
    const encoded = encodeTypedProgramLinks(program);
    const network = LinkNetwork.fromLino(encoded.network.toLino());
    assert.throws(() => emitTypedProgramLinks(network, encoded.root, 'Rust', options), /ASCII identifier/u, text);
  }
});

test('the declared ASCII lifetime boundary keeps digits separate from first-position letters and underscore', () => {
  for (const [character, first, expected] of [[65, true, true], [90, true, true], [97, true, true], [122, true, true], [95, true, true], [48, true, false], [57, false, true], [47, false, false], [58, false, false], [64, true, false], [91, true, false], [96, true, false], [123, true, false], [945, false, false], [NaN, false, false], [Infinity, false, false], [-0, true, false]]) assert.equal(acceptRustSourceHeaderLifetimeCharacter(character, first), expected);
});
