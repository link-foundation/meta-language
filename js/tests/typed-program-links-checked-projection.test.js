import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, createTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';

const floating = { kind: 'float' };
const boolean = { kind: 'bool' };
function productProgram() {
  return checkProgram({ language: 'JavaScript', items: [{ k: 'fn', name: 'make', params: [],
    ret: { kind: 'tuple', elements: [floating, boolean] },
    body: { k: 'tuple', frozen: true, productModel: 'immutable value product', items: [{ k: 'num', value: '5', type: floating }, { k: 'bool', value: true }] },
  }], main: { effects: [{ k: 'print', style: 'js-console', expr: { k: 'tupleIndex', index: 0,
    object: { k: 'app', fn: { k: 'name', path: ['make'] }, args: [] },
  } }] } });
}

test('whole linked product programs recheck with ordered elements and immutable descriptors', () => {
  const program = productProgram();
  const archive = encodeTypedProgramLinks(program);
  assert.equal(decodeTypedProgramLinks(archive.network, archive.root).declarations.get('make').body.frozen, true);
  const code = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
  const output = [];
  const descriptor = vm.runInNewContext(`${code}\n[Object.isFrozen(make()), make().length, make()[1]]`,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['5']);
  assert.deepEqual(Array.from(descriptor), [true, 2, true]);
  const product = program.declarations.get('make').body;
  product.frozen = false;
  assert.throws(() => encodeTypedProgramLinks(program), /product model.*contradicts/u);
  delete product.productModel;
  const mutable = encodeTypedProgramLinks(program);
  const changed = emitTypedProgramLinks(mutable.network, mutable.root, 'JavaScript').code;
  assert.equal(vm.runInNewContext(`${changed}\nObject.isFrozen(make())`, { console: { log() {} } }, { timeout: 1000 }), false);
});

test('tuple field, layout, facet and positional edits are checked before emission', () => {
  for (const mutate of [
    program => { program.declarations.get('make').body.items.pop(); },
    program => { program.declarations.get('make').body.type = { kind: 'tuple', elements: [boolean, floating] }; },
    program => { program.declarations.get('make').body.frozen = 'true'; },
    program => { program.declarations.get('make').body.productModel = 'mutable value product'; },
    program => { program.declarations.get('make').body.productModel = null; },
    program => { program.declarations.get('make').body.productModel = ''; },
    program => { program.declarations.get('make').body.extraProjection = true; },
    program => { program.declarations.get('make').body.type = { ...program.declarations.get('make').ret, frozen: true }; },
    program => { program.main.effects[0].expr.arg.index = 2; },
    program => { program.main.effects[0].expr.arg.index = Number.MAX_SAFE_INTEGER + 1; },
    program => { program.main.effects[0].expr.arg.index = '0'; },
    program => { program.main.effects[0].expr.arg.object.type = { kind: 'array', element: floating }; },
  ]) {
    const program = productProgram(); mutate(program);
    assert.throws(() => encodeTypedProgramLinks(program), /linked|tuple|positional/u);
  }
  const tuple = productProgram().declarations.get('make').body;
  assert.throws(() => projectLinkedProgramExpression({ ...tuple, extraProjection: true }), /unrecognized projection field/u);
});

test('complete IR comparison preserves span roles and rejects forged program semantics', () => {
  const spanned = productProgram();
  spanned.declarations.get('make').body.span = { start: 4, end: 20 };
  const archive = encodeTypedProgramLinks(spanned);
  assert.deepEqual(decodeTypedProgramLinks(archive.network, archive.root).declarations.get('make').body.span, { start: 4, end: 20 });
  for (const mutate of [
    program => { program.declarations.get('make').recursive = true; },
    program => { program.declarations.get('make').decreasing = 0; },
    program => { program.declarations.get('make').body.items[0].unexpectedPolicy = 'ignored'; },
    program => { program.main.effects[0].expr.arg.unexpectedPolicy = 'ignored'; },
    program => { program.span = null; },
  ]) {
    const program = productProgram(); mutate(program);
    assert.throws(() => encodeTypedProgramLinks(program), /projection|inconsistent types, bindings or operation semantics/u);
  }
});

test('the same recheck retains actual data declarations and match scopes across the whole program', () => {
  const base = checkProgram(parseRust('enum Choice{Empty,Value(i32)} fn read(item:Choice)->i32{match item{Choice::Empty=>0,Choice::Value(value)=>value}} fn main(){println!("{}",read(Choice::Value(7)));}'));
  const product = productProgram().declarations.get('make');
  base.items.push(product); base.declarations.set(product.fullName, product);
  assert.equal(recheckLinkedTypedProgram(base), base);
  const archive = encodeTypedProgramLinks(base);
  const output = [];
  vm.runInNewContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['7']);
  base.declarations.get('read').body.cases[1].body.type = { kind: 'fixed', bits: 64, signed: true };
  assert.throws(() => encodeTypedProgramLinks(base), /inconsistent variable type/u);
});

test('new explicit product programs refuse uncovered projections while legacy collections retain behavior', () => {
  const program = productProgram();
  const literal = program.declarations.get('make').body.items[0];
  program.declarations.get('make').body.items[0] = { k: 'collectionRead', op: 'get', type: floating,
    object: { k: 'var', name: 'unbound', type: { kind: 'map', key: floating, value: floating } }, key: literal,
  };
  assert.throws(() => recheckLinkedTypedProgram(program), /collectionRead has no surface projection/u);
  const legacy = createTypedProgramLinks('const values=new Map([[1,2]]);console.log(values.get(1));', 'JavaScript');
  const output = [];
  vm.runInNewContext(emitTypedProgramLinks(legacy.network, legacy.root, 'JavaScript').code,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['2']);
});

test('the mechanical recheck rejects structural cycles and excessively deep program records', () => {
  const cyclic = productProgram();
  cyclic.declarations.get('make').body.items.push(cyclic.declarations.get('make').body);
  assert.throws(() => recheckLinkedTypedProgram(cyclic), /structural cycle/u);
  const deep = productProgram();
  let value = {};
  for (let index = 0; index < 260; index += 1) value = { value };
  deep.metadata = value;
  assert.throws(() => recheckLinkedTypedProgram(deep), /resource limit/u);
});

test('tuple position zero is canonical while floating literal negative zero survives', () => {
  const program = productProgram();
  program.main.effects[0].expr.arg.index = -0;
  const archive = encodeTypedProgramLinks(program);
  assert.equal(Object.is(program.main.effects[0].expr.arg.index, -0), false);
  const index = archive.network.fieldRelations().find(relation => relation.label === 'index').child;
  const literal = archive.network.fieldRelations().find(relation => relation.label === 'value'
    && archive.network.link(relation.child).metadata().definition === '5').child;
  const scalar = archive.network.link(literal);
  scalar.setMetadata(scalar.metadata().withDefinition('-0'));
  for (const spelling of ['-0', '0.0', '0e0']) {
    const position = archive.network.link(index);
    position.setMetadata(position.metadata().withDefinition(spelling));
    const decoded = decodeTypedProgramLinks(archive.network, archive.root);
    assert.equal(Object.is(decoded.main.effects[0].expr.arg.index, -0), false);
    assert.equal(decoded.declarations.get('make').body.items[0].value, '-0');
    const output = [];
    vm.runInNewContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code,
      { console: { log: value => output.push(value) } }, { timeout: 1000 });
    assert.deepEqual(output, ['-0']);
  }
});
