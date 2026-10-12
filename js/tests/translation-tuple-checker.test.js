import assert from 'node:assert/strict';
import { test } from 'node:test';

import { checkProgram } from '../src/translation/check.js';
import { Scope, containsKind, renameExpr } from '../src/translation/ir.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { acceptTupleIndex } from '../src/translation/primitive-value-rules.js';
import { BOOL, FLOAT, INT, NAT, STRING, UNIT, array, fixed, immutableProductType, tuple, typeKey } from '../src/translation/types.js';

const number = value => ({ k: 'num', value: String(value), type: FLOAT });
const product = (items, frozen = true) => ({ k: 'tuple', items, frozen });
const checked = (body, ret, params = [], language = 'JavaScript') => checkProgram({ language, items: [{ k: 'fn', name: 'choose', params, ret, body }], main: null }).declarations.get('choose').body;

test('immutable products keep positional types, explicit freezing and constant projections', () => {
  const type = tuple([FLOAT, BOOL]);
  const body = checked(product([number(5), { k: 'bool', value: true }]), type);
  assert.equal(body.frozen, true);
  assert.deepEqual(body.items.map(item => item.type), [FLOAT, BOOL]);
  const projection = checked({ k: 'tupleIndex', object: product([number(5), { k: 'bool', value: true }]), index: 1 }, BOOL);
  assert.equal(projection.k, 'tupleIndex');
  assert.equal(projection.type, BOOL);
  assert.equal(projection.object.frozen, true);
  assert.equal(typeKey(tuple([BOOL])), 'tuple:[bool]');
  assert.notEqual(typeKey(tuple([])), typeKey(UNIT));
});

test('product admission recurses through scalar Copy types and preserves each expected numeric type', () => {
  for (const type of [BOOL, FLOAT, fixed(32, true), UNIT, tuple([BOOL, tuple([FLOAT, UNIT])])]) assert.equal(immutableProductType(type), true, typeKey(type));
  for (const type of [STRING, INT, NAT, array(FLOAT), tuple([BOOL, STRING]), { kind: 'data', name: 'Tree' }]) assert.equal(immutableProductType(type), false, typeKey(type));
  const signed = fixed(32, true);
  const type = tuple([signed, tuple([FLOAT, UNIT])]);
  const body = checked(product([{ k: 'num', value: '7' }, product([number(5), { k: 'unit' }])]), type, [], 'Rust');
  assert.deepEqual(body.items[0].type, signed);
  assert.equal(body.items[1].frozen, true);
  assert.deepEqual(body.items[1].items.map(item => item.type), [FLOAT, UNIT]);
});

test('products refuse incorrect arity, heterogeneous types and unsupported frozen operands', () => {
  assert.throws(() => checked(product([number(5)]), tuple([FLOAT, BOOL])), /wrong number of elements/);
  assert.throws(() => checked(product([{ k: 'bool', value: false }]), tuple([FLOAT])), /expected float/);
  assert.throws(() => checked(product([{ k: 'str', value: 'owned' }]), tuple([STRING])), /recursive scalar Copy/);
  assert.throws(() => checked(product([number(5)], 'true'), tuple([FLOAT])), /frozen facet must be a boolean/);
  assert.throws(() => checked(product([product([number(5)], false)]), tuple([tuple([FLOAT])])), /mutable inner tuple/);
  assert.throws(() => checked(product([]), tuple([])), /empty positional product is distinct from a unit/);
  assert.equal(checked({ k: 'unit' }, UNIT).k, 'unit');
});

test('tuple indices are safe nonnegative integers in range, with no array coercion', () => {
  const object = product([number(5), { k: 'bool', value: true }]);
  for (const index of [-1, 0.5, 2, Number.NaN, Number.POSITIVE_INFINITY, Number.MAX_SAFE_INTEGER + 1]) {
    assert.equal(acceptTupleIndex(index, 2), false);
    assert.throws(() => checked({ k: 'tupleIndex', object, index }, FLOAT), /tuple index out of range/);
  }
  assert.equal(acceptTupleIndex(0, 2), true);
  assert.throws(() => checked({ k: 'tupleIndex', object: number(5), index: 0 }, FLOAT), /requires a tuple value/);
  assert.throws(() => checked({ k: 'index', object, index: { k: 'num', value: '1', negative: true } }, FLOAT), /tuple index out of range/);
});

test('nested product value proofs follow owned bindings and both branches rather than trusting tuple types', () => {
  const type = tuple([tuple([FLOAT])]);
  const inner = frozen => product([number(5)], frozen);
  const bound = value => ({ k: 'let', name: 'inner', value, body: { k: 'name', path: ['inner'] } });
  assert.throws(() => checked(product([bound(inner(false))]), type), /mutable inner tuple/);
  assert.throws(() => checked(product([{ k: 'if', cond: { k: 'bool', value: true }, then: inner(true), else: inner(false) }]), type), /mutable inner tuple/);
  assert.throws(() => checked(product([{ k: 'name', path: ['input'] }]), type, [{ name: 'input', type: tuple([FLOAT]) }]), /mutable inner tuple/);
  const accepted = checked(product([bound(inner(true))]), type);
  assert.equal(accepted.items[0].k, 'let');
  assert.equal(accepted.items[0].value.frozen, true);
  assert.equal(accepted.frozen, true);
});

test('ordinary contextual array tuples and Map entry tuples retain their previous value model', () => {
  const source = `/** @returns {[string, number]} */ function pair() { return ['name', 5]; }
/** @returns {Map<string, number>} */ function entries() { return new Map([['name', 5]]); }`;
  const program = checkProgram(parseJavaScript(source));
  const pair = program.declarations.get('pair').body;
  assert.equal(pair.k, 'tuple');
  assert.equal(Object.hasOwn(pair, 'frozen'), false);
  assert.deepEqual(pair.items.map(item => item.type), [STRING, FLOAT]);
  const entry = program.declarations.get('entries').body.entries.items[0];
  assert.equal(entry.k, 'tuple');
  assert.equal(Object.hasOwn(entry, 'frozen'), false);
  assert.deepEqual(entry.type, tuple([STRING, FLOAT]));
});

test('generic traversal and binder renaming retain product positions and freezing', () => {
  const body = checked(product([{ k: 'name', path: ['left'] }, { k: 'name', path: ['right'] }]), tuple([FLOAT, BOOL]), [{ name: 'left', type: FLOAT }, { name: 'right', type: BOOL }]);
  const renamed = renameExpr(body, new Map([['left', 'first'], ['right', 'second']]), new Scope(name => name));
  assert.deepEqual(renamed.items.map(item => item.name), ['first', 'second']);
  assert.equal(containsKind({ k: 'tupleIndex', object: renamed, index: 0, type: FLOAT }, 'var'), true);
  assert.equal(renamed.frozen, true);
});
