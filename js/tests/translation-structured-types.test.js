import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import test from 'node:test';
import { checkProgram, coerce } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { renderPortableTypeIdentity, acceptOptionalAbsence, mergeOptionalAbsence, readRustOptionalPrelude, renderRustCompoundType, renderRustReferenceType, wrapPortableReferenceIdentity } from '../src/translation/structured-type-rules.js';
import { BOOL, option, sameType, typeKey } from '../src/translation/types.js';

const fixture = readProjectData(new URL('../../parity/fixtures/translation-structured-types.json', import.meta.url));

test('structural identities preserve named fields, nesting and UTF-16 field order', () => {
  for (const entry of fixture.cases) assert.equal(typeKey(entry.type), entry.identity, entry.name);
  const record = fixture.cases.find(({ name }) => name === 'sorted record').type;
  assert.equal(sameType(record, { ...record, fields: [...record.fields].reverse() }), true);
  assert.equal(sameType(record, { ...record, open: true }), false);
  assert.deepEqual(option(option(BOOL, 'null'), 'undefined'), { kind: 'option', element: BOOL, absence: 'both' });
  assert.deepEqual(option(option(BOOL, 'null'), 'null'), { kind: 'option', element: BOOL, absence: 'null' });
  const sharedArray = fixture.cases.find(({ name }) => name === 'reference array').type;
  assert.equal(sameType(sharedArray, { ...sharedArray, reference: false }), false);
  assert.equal(wrapPortableReferenceIdentity('array:bool', true), 'reference:array:bool');
  assert.equal(wrapPortableReferenceIdentity('array:bool', false), 'array:bool');
});

test('absence admission and normalization retain null and undefined separately', () => {
  for (const expected of ['null', 'undefined', 'both']) for (const actual of ['null', 'undefined', 'both']) {
    assert.equal(acceptOptionalAbsence(expected, actual), expected === 'both' || expected === actual);
    assert.equal(mergeOptionalAbsence(expected, actual), expected === actual ? expected : 'both');
  }
  const none = { k: 'none', absence: 'undefined', type: option(BOOL, 'undefined') };
  assert.equal(coerce(none, option(BOOL, 'both'), 'JavaScript').type.absence, 'both');
  assert.throws(() => coerce(none, option(BOOL, 'null'), 'JavaScript'), /expected null absence/);
});

test('compound signatures are emitted through shared rules with exact target source', () => {
  for (const entry of fixture.emissionCases) assert.equal(emitRust(checkProgram(entry.surface)).text, entry.rust, entry.name);
  assert.equal(readRustOptionalPrelude(), fixture.optionalPrelude);
  assert.equal(renderRustCompoundType('tuple', '', []), '()');
  assert.equal(renderRustCompoundType('record', '', ['bool']), '(bool,)');
  assert.equal(renderRustCompoundType('function', 'bool', []), 'std::rc::Rc<dyn Fn() -> bool>');
  assert.equal(renderRustCompoundType('cell', '', ['bool']), 'std::rc::Rc<std::cell::RefCell<bool>>');
  assert.equal(renderRustReferenceType('Vec<bool>', true), 'std::rc::Rc<std::cell::RefCell<Vec<bool>>>');
  assert.equal(renderRustReferenceType('Vec<bool>', false), 'Vec<bool>');
});

test('data traits inspect every nested compound and reject callable structural fields', () => {
  for (const entry of fixture.traitCases) {
    assert.equal(emitRust(checkProgram(entry.surface)).text, entry.rust, entry.name);
    const derive = entry.totalEquality ? '#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Box' : '#[derive(Clone, Debug, PartialEq)]\npub enum Box';
    assert.ok(entry.rust.includes(derive), entry.name);
  }
  assert.throws(() => emitRust(checkProgram(fixture.callableData)), /callable data field/);
});

test('pure targets refuse shared reference signatures until their state model is represented', () => {
  for (const entry of fixture.emissionCases.filter(({ name }) => name.startsWith('reference ') || name.includes('cell'))) {
    for (const emit of [emitLean, emitRocq]) {
      assert.throws(() => emit(checkProgram(entry.surface)), /mutable structural value: shared reference state needs an explicit target state model/, entry.name);
    }
  }
});

test('shared type identities retain distinct unresolved inference variables', () => {
  for (const entry of fixture.inferenceIdentities) assert.equal(renderPortableTypeIdentity(entry.kind, entry.detail, entry.children), entry.identity);
  assert.equal(typeKey({ kind: 'var', id: 0 }), 'var:0');
  assert.equal(typeKey({ kind: 'var', id: 1 }), 'var:1');
  assert.equal(sameType({ kind: 'array', element: { kind: 'var', id: 0 } }, { kind: 'array', element: { kind: 'var', id: 1 } }), false);
});
