import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { checkProgram, coerce } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptOptionalAbsence, mergeOptionalAbsence, readRustOptionalPrelude, renderRustCompoundType } from '../src/translation/structured-type-rules.js';
import { BOOL, option, sameType, typeKey } from '../src/translation/types.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/translation-structured-types.json', import.meta.url), 'utf8'));

test('structural identities preserve named fields, nesting and UTF-16 field order', () => {
  for (const entry of fixture.cases) assert.equal(typeKey(entry.type), entry.identity, entry.name);
  const record = fixture.cases.find(({ name }) => name === 'sorted record').type;
  assert.equal(sameType(record, { ...record, fields: [...record.fields].reverse() }), true);
  assert.equal(sameType(record, { ...record, open: true }), false);
  assert.deepEqual(option(option(BOOL, 'null'), 'undefined'), { kind: 'option', element: BOOL, absence: 'both' });
  assert.deepEqual(option(option(BOOL, 'null'), 'null'), { kind: 'option', element: BOOL, absence: 'null' });
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
});
