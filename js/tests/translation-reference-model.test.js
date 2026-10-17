import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { LinkType } from '../src/primitives.js';
import {
  ReferenceHeap, ReferenceValue, referenceMethodOperation, referenceOperation,
  referenceOperationNetwork, referenceOperations,
} from '../src/translation/reference-model.js';

function method(heap, kind, name, receiver, ...arguments_) {
  return heap.execute(referenceMethodOperation(kind, name), receiver, ...arguments_);
}

function value(heap, result) { return result instanceof ReferenceValue ? heap.read(result) : result; }

test('reference operation aliases resolve to linked executable programs shared with Rust data', () => {
  assert.equal(readFileSync(new URL('../src/data/reference-operations.lino', import.meta.url), 'utf8'),
    readFileSync(new URL('../../rust/src/data/reference-operations.lino', import.meta.url), 'utf8'));
  const network = referenceOperationNetwork();
  const reloaded = LinkNetwork.fromLino(network.toLino());
  for (const operation of referenceOperations()) {
    assert.equal(network.link(operation.link).metadata().term, operation.id);
    assert.equal(reloaded.link(operation.link).metadata().term, operation.id);
    assert.ok(operation.program.length > 0);
    assert.equal(operation.program.at(-1)[0], 'return-value');
    if (operation.method) assert.equal(referenceMethodOperation(operation.kind, operation.method), operation);
  }
  assert.equal(referenceMethodOperation('array', 'invented'), undefined);
  assert.equal(referenceMethodOperation('map', 'push'), undefined);
});

test('reference allocation, aliases, reads, writes and identity execute their linked instructions', () => {
  const heap = new ReferenceHeap();
  const reference = heap.execute('allocate-reference-cell', undefined, 'array', [1, 2]);
  const alias = heap.alias(reference);
  const other = heap.allocate('array', [1, 2]);
  assert.equal(heap.execute('compare-reference-identity', reference, alias), true);
  assert.equal(heap.execute('compare-reference-identity', reference, other), false);
  assert.equal(heap.identity(heap.execute('read-reference-cell', alias), reference), true);
  const live = heap.read(alias);
  assert.equal(heap.execute('write-reference-cell', reference, [3]), reference);
  assert.equal(heap.read(reference), live);
  assert.deepEqual(heap.read(alias), [3]);
  assert.throws(() => heap.read(new ReferenceHeap().allocate('array')), /different heap/);
  assert.throws(() => heap.execute('not-an-operation', reference), /unknown reference operation/);
  assert.throws(() => method(heap, 'map', 'set', reference, 'a', 1), /expected map/);
});

test('execution reads editable linked instructions rather than a detached descriptor program', () => {
  const network = referenceOperationNetwork();
  const operation = referenceOperation('append-array-elements');
  const program = network.link(operation.link).references().map((id) => network.link(id)).find((link) => (
    network.link(link.references()[0]).metadata().term === 'program'
  ));
  const instruction = network.link(program.references().at(-1));
  const original = instruction.references();
  const receiverPoint = network.links().find((link) => link.metadata().term === 'receiver').id();
  instruction.setReferences([original[0], receiverPoint]);
  try {
    const heap = new ReferenceHeap();
    const reference = heap.allocate('array', [1]);
    const result = heap.execute(operation, reference, 2);
    assert.equal(result, reference);
    assert.deepEqual(heap.read(reference), [1, 2]);
  } finally {
    instruction.setReferences(original);
  }
});

test('selected array element and record field writes preserve aliases, holes and evaluation order', () => {
  const heap = new ReferenceHeap();
  const array = heap.allocate('array', [1]);
  const alias = heap.alias(array);
  const native = [1];
  native[3] = 4;
  assert.equal(heap.setElement(alias, 3, 4), 4);
  assert.deepEqual(heap.read(array), native);
  assert.equal(heap.getElement(array, 2), undefined);
  assert.equal(Object.hasOwn(heap.read(array), 2), false);
  heap.setElement(array, -1, 5);
  native[-1] = 5;
  assert.deepEqual(heap.read(array), native);
  const before = heap.getElement(array, 0);
  const right = (() => { heap.setElement(alias, 0, 10); return 2; })();
  heap.setElement(array, 0, before + right);
  assert.equal(heap.getElement(alias, 0), 3);
  const record = heap.allocate('record', { first: 1, second: 2 });
  const recordAlias = heap.alias(record);
  assert.equal(heap.setField(recordAlias, 'third', 3), 3);
  assert.deepEqual(Object.keys(heap.read(record)), ['first', 'second', 'third']);
  assert.equal(heap.getField(recordAlias, 'first'), 1);
  assert.throws(() => heap.setField(record, '__proto__', {}), /prototype setters/);
  assert.throws(() => heap.setElement(array, Symbol('index'), 1), /property keys/);
  const reload = ReferenceHeap.fromLino(heap.toLino());
  assert.deepEqual(reload.read(new ReferenceValue(reload, array.id)), heap.read(array));
  assert.deepEqual(reload.read(new ReferenceValue(reload, record.id)), heap.read(record));
});

test('array mutation results, sparse slots, argument omissions and aliases match native JavaScript', () => {
  const native = [4, , 2, undefined, 1];
  native.note = 'kept';
  const heap = new ReferenceHeap();
  const reference = heap.allocate('array', native);
  const alias = heap.alias(reference);
  const operations = [
    ['push', [7, 8]], ['pop', []], ['unshift', [9, 10]], ['shift', []],
    ['reverse', []], ['splice', [-3, 2, 11, 12]], ['splice', []], ['splice', [2]],
    ['pop', []], ['pop', []], ['push', []],
  ];
  for (const [name, arguments_] of operations) {
    const expected = Reflect.apply(Array.prototype[name], native, arguments_);
    const actual = method(heap, 'array', name, alias, ...arguments_);
    if (expected === native) assert.equal(heap.identity(actual, reference), true);
    else assert.deepEqual(value(heap, actual), expected);
    assert.deepEqual(heap.read(reference), native);
  }
  const empty = heap.allocate('array');
  assert.equal(method(heap, 'array', 'pop', empty), undefined);
  assert.equal(method(heap, 'array', 'shift', empty), undefined);
});

test('stable sort preserves element identities, undefined values and holes', () => {
  const original = [{ key: 2, name: 'a' }, { key: 1, name: 'b' }, { key: 2, name: 'c' }, undefined, , { key: 1, name: 'd' }];
  const native = original.slice();
  const heap = new ReferenceHeap();
  const reference = heap.allocate('array', original);
  const first = heap.reference(original[0]);
  const nativeComparisons = [];
  const heapComparisons = [];
  native.sort((left, right) => { nativeComparisons.push([left.name, right.name]); return left.key - right.key; });
  const result = method(heap, 'array', 'sort', reference, (left, right) => {
    heapComparisons.push([left.name, right.name]); return left.key - right.key;
  });
  assert.equal(heap.identity(result, reference), true);
  assert.deepEqual(heap.read(reference), native);
  assert.deepEqual(heapComparisons, nativeComparisons);
  assert.deepEqual(heap.read(reference).slice(0, 4).map((element) => element.name), ['b', 'd', 'a', 'c']);
  assert.equal(heap.identity(heap.reference(heap.read(reference)[2]), first), true);
  assert.equal(Object.hasOwn(heap.read(reference), 4), true);
  assert.equal(Object.hasOwn(heap.read(reference), 5), false);
  const lexical = heap.allocate('array', [2, 11, 3]);
  method(heap, 'array', 'sort', lexical);
  assert.deepEqual(heap.read(lexical), [11, 2, 3]);
});

test('sort comparator callbacks observe aliased mutations and preserve effects before exceptions', () => {
  const native = [3, 2, 1];
  let nativeMutated = false;
  native.sort((left, right) => { if (!nativeMutated) { nativeMutated = true; native.push(9); } return left - right; });
  const heap = new ReferenceHeap();
  const reference = heap.allocate('array', [3, 2, 1]);
  const alias = heap.alias(reference);
  let heapMutated = false;
  method(heap, 'array', 'sort', reference, (left, right) => {
    assert.equal(heap.read(alias), heap.read(reference));
    if (!heapMutated) { heapMutated = true; method(heap, 'array', 'push', alias, 9); }
    return left - right;
  });
  assert.deepEqual(heap.read(reference), native);
  const failure = new Error('comparison failed');
  assert.throws(() => method(heap, 'array', 'sort', reference, () => {
    method(heap, 'array', 'push', alias, 10); throw failure;
  }), (error) => error === failure);
  assert.equal(heap.read(reference).at(-1), 10);
  const reload = ReferenceHeap.fromLino(heap.toLino());
  assert.deepEqual(reload.read(new ReferenceValue(reload, reference.id)), heap.read(reference));
});

test('Map and Set use SameValueZero and reference identity rather than structural equality', () => {
  const heap = new ReferenceHeap();
  const map = heap.allocate('map');
  const set = heap.allocate('set');
  const firstObject = { key: 1 };
  const secondObject = { key: 1 };
  const first = heap.allocate('record', firstObject);
  const second = heap.allocate('record', secondObject);
  assert.equal(heap.identity(method(heap, 'map', 'set', map, NaN, 'a'), map), true);
  method(heap, 'map', 'set', map, NaN, 'b');
  method(heap, 'map', 'set', map, -0, 'zero');
  method(heap, 'map', 'set', map, first, 'first');
  method(heap, 'map', 'set', map, second, 'second');
  assert.equal(heap.read(map).size, 4);
  assert.equal(method(heap, 'map', 'get', map, NaN), 'b');
  assert.equal(method(heap, 'map', 'get', map, +0), 'zero');
  assert.equal(method(heap, 'map', 'get', map, firstObject), 'first');
  assert.equal(method(heap, 'map', 'get', map, second), 'second');
  assert.equal(method(heap, 'map', 'has', map, {}), false);
  assert.equal(Object.is([...heap.read(map).keys()][1], -0), false);
  for (const member of [NaN, NaN, -0, +0, first, firstObject, second]) {
    assert.equal(heap.identity(method(heap, 'set', 'add', set, member), set), true);
  }
  assert.equal(heap.read(set).size, 4);
  assert.equal(method(heap, 'set', 'has', set, firstObject), true);
  assert.equal(method(heap, 'set', 'delete', set, first), true);
  assert.equal(method(heap, 'set', 'delete', set, first), false);
  assert.equal(method(heap, 'map', 'delete', map, first), true);
  assert.equal(method(heap, 'map', 'delete', map, first), false);
  assert.equal(method(heap, 'set', 'clear', set), undefined);
  assert.equal(method(heap, 'map', 'clear', map), undefined);
  assert.equal(heap.read(set).size, 0);
  assert.equal(heap.read(map).size, 0);
});

test('array, Map and Set iterators stay live through deletion, reinsertion and clearing', () => {
  const heap = new ReferenceHeap();
  const array = heap.allocate('array', [1, 2]);
  const arrayIterator = heap.iterate(array);
  assert.deepEqual(arrayIterator.next(), { value: 1, done: false });
  method(heap, 'array', 'push', array, 3);
  assert.deepEqual([...arrayIterator], [2, 3]);
  for (const kind of ['map', 'set']) {
    const reference = heap.allocate(kind, kind === 'map' ? new Map([['a', 1], ['b', 2], ['c', 3]]) : new Set(['a', 'b', 'c']));
    const iterator = heap.iterate(reference, 'keys');
    assert.equal(iterator.next().value, 'a');
    method(heap, kind, 'delete', reference, 'b');
    if (kind === 'map') method(heap, kind, 'set', reference, 'a', 4);
    else method(heap, kind, 'add', reference, 'a');
    assert.equal(iterator.next().value, 'c');
    method(heap, kind, 'delete', reference, 'a');
    method(heap, kind, kind === 'map' ? 'set' : 'add', reference, 'a', 5);
    assert.equal(iterator.next().value, 'a');
    method(heap, kind, 'clear', reference);
    method(heap, kind, kind === 'map' ? 'set' : 'add', reference, 'd', 6);
    assert.equal(iterator.next().value, 'd');
    assert.equal(iterator.next().done, true);
  }
});

test('link serialization retains cycles, duplicated references, sparse arrays and primitive distinctions', () => {
  const shared = { number: NaN, negativeZero: -0, infinity: Infinity, big: 12345678901234567890n, text: '\ud800\u0000🙂', absent: undefined, nil: null };
  const array = [shared, , shared];
  const record = Object.assign(Object.create(null), { first: shared, second: shared, array });
  record.self = record;
  record.map = new Map([[record, shared], [shared, record]]);
  record.set = new Set([record, shared]);
  Object.defineProperty(record, '__proto__', { value: shared, writable: false, enumerable: false, configurable: false });
  const heap = new ReferenceHeap();
  const reference = heap.allocate('record', record);
  const source = heap.toLino();
  const actualNetwork = LinkNetwork.fromLino(source);
  assert.ok(actualNetwork.links().some((link) => link.metadata().linkType === LinkType.Reference));
  assert.ok(actualNetwork.links().some((link) => link.references().some((id) => Number(id) === Number(reference.id))));
  assert.equal(source.includes('JSON'), false);
  const reload = ReferenceHeap.fromLino(source);
  const reloadedReference = new ReferenceValue(reload, reference.id);
  const result = reload.read(reloadedReference);
  assert.equal(result.self, result);
  assert.equal(result.first, result.second);
  assert.equal(result.array[0], result.first);
  assert.equal(result.array[2], result.first);
  assert.equal(Object.hasOwn(result.array, 1), false);
  assert.equal(result.map.get(result), result.first);
  assert.equal(result.map.get(result.first), result);
  assert.equal(result.set.has(result), true);
  assert.equal(Object.getPrototypeOf(result), null);
  assert.deepEqual(Object.getOwnPropertyDescriptor(result, '__proto__'), Object.getOwnPropertyDescriptor(record, '__proto__'));
  assert.deepEqual(result.first, shared);
  assert.equal(Object.is(result.first.negativeZero, -0), true);
  assert.equal(reload.identity(reload.reference(result.self), reloadedReference), true);
  method(reload, 'array', 'push', reload.reference(result.array), result.first);
  assert.equal(result.array.at(-1), result.first);
});

test('unsupported heap values fail before allocation and readonly descriptors retain native failures', () => {
  const heap = new ReferenceHeap();
  const before = heap.network.links().length;
  for (const value of [{ fn() {} }, { symbol: Symbol('x') }, Object.create({ custom: true }), new Date(), { get field() { throw new Error('getter must not execute'); } }]) {
    assert.throws(() => heap.allocate('record', value), /serialize|supports/);
    assert.equal(heap.network.links().length, before);
  }
  const array = heap.allocate('array', Object.freeze([1, 2]));
  assert.throws(() => method(heap, 'array', 'push', array, 3), TypeError);
  assert.deepEqual(heap.read(array), [1, 2]);
  const reload = ReferenceHeap.fromLino(heap.toLino());
  const reference = new ReferenceValue(reload, array.id);
  assert.throws(() => method(reload, 'array', 'pop', reference), TypeError);
  assert.deepEqual(reload.read(reference), [1, 2]);
  const closed = heap.allocate('array', Object.preventExtensions([1, 2]));
  assert.throws(() => method(heap, 'array', 'push', closed, 3), TypeError);
  assert.equal(Object.isExtensible(heap.read(closed)), false);
  const closedReload = ReferenceHeap.fromLino(heap.toLino());
  assert.throws(() => method(closedReload, 'array', 'push', new ReferenceValue(closedReload, closed.id), 3), TypeError);
  assert.throws(() => ReferenceHeap.fromLino(''), /exactly one root/);
});
