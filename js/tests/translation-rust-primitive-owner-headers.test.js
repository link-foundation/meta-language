import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const capture = {
  rustPrimitiveOwnerModel: 'standard primitive Rust binary64 owner 1',
  rustSourceSignatureModel: 'Rust source function signature 1',
};
const header = { rustSourceHeaderEmissionModel: 'primitive borrowed Rust function header 1' };
const inputs = [
  ['', 'std::primitive::f64'],
  ['', 'core::primitive::f64'],
  ['use std::primitive::f64 as Real;', 'Real'],
  ['use core::primitive::f64 as Float;', 'Float'],
];
const packet = (prefix, owner, expression = '*value') => createTypedProgramLinks(
  `${prefix}fn apply(value:&${owner})->${owner}{${expression}}fn main(){}`, 'Rust', capture);
const reload = packet => ({ network: LinkNetwork.fromLino(packet.network.toLino()), root: packet.root });
const observe = (network, root, value) => vm.runInNewContext(
  emitTypedProgramLinks(network, root, 'JavaScript').code + '\napply(input)', { input: value }, { timeout: 1000 });

test('qualified source owner proofs compose with canonical borrowed header emission after Links reload', () => {
  for (const [prefix, owner] of inputs) {
    const { network, root } = reload(packet(prefix, owner));
    const before = network.toLino();
    const program = decodeTypedProgramLinks(network, root);
    assert.deepEqual(program.items[0].sourcePrimitiveOwners.parameters[0].sourcePath, owner.split('::'));
    assert.deepEqual(program.items[0].sourceSignature.parameters[0].owner.path, ['f64']);
    assert.ok(emitTypedProgramLinks(network, root, 'Rust', header).code.includes('pub fn apply(value: &f64) -> f64'));
    for (const [value, expected] of [[-0, -0], [-0.5, -0.5], [NaN, NaN], [Infinity, Infinity]]) {
      assert.ok(Object.is(observe(network, root, value), expected), owner);
    }
    assert.equal(network.toLino(), before);
  }
});

test('source-free value edits retain both owner and borrowed-interface proofs', () => {
  const original = reload(packet(inputs[2][0], inputs[2][1], '*value + 1.0'));
  const program = decodeTypedProgramLinks(original.network, original.root);
  const declaration = program.items[0];
  const owner = structuredClone(declaration.sourcePrimitiveOwners);
  const signature = structuredClone(declaration.sourceSignature);
  assert.equal(declaration.body.right.value, '1');
  declaration.body.right.value = '2';
  const edited = reload(encodeTypedProgramLinks(program));
  const restored = decodeTypedProgramLinks(edited.network, edited.root).items[0];
  assert.deepEqual(restored.sourcePrimitiveOwners, owner);
  assert.deepEqual(restored.sourceSignature, signature);
  assert.ok(emitTypedProgramLinks(edited.network, edited.root, 'Rust', header).code.includes('pub fn apply(value: &f64) -> f64'));
  assert.equal(observe(edited.network, edited.root, 3), 5);
});

test('a retained qualified-owner facet must validate before selected header emission', () => {
  const original = reload(packet(inputs[3][0], inputs[3][1]));
  const program = decodeTypedProgramLinks(original.network, original.root);
  program.items[0].sourcePrimitiveOwners.namespace.imports[0].target = ['core', 'primitive', 'f32'];
  assert.throws(() => encodeTypedProgramLinks(program), /primitive owner proof/u);
});

test('primitive owner selection preserves the separate header model operation boundary', () => {
  const { network, root } = reload(packet(inputs[3][0], inputs[3][1], '(*value).floor()'));
  assert.equal(observe(network, root, -0.5), -1);
  assert.throws(() => emitTypedProgramLinks(network, root, 'Rust', header), /distinct interface transformation proof/u);
});
