// Authored enum controls for the source-only lowering experiment.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseProgrammingLanguage} from '../../js/src/programming-language-parser.js';
import {lowerRestrictedCst, archiveRestrictedCLowering, readRestrictedCLowering} from './restricted-c-cst.mjs';

const abi = {intBits: 32, sizeBits: 64, charSigned: true};
const lower = source => lowerRestrictedCst(parseProgrammingLanguage(source, 'C'), source, {abi});

test('implicit source enumerators supply exact typed scanner-index values without invented storage', () => {
  for (const end of ['CLOSE', 'EntirelyDifferent']) {
    const source = `typedef enum { UNUSED, OPEN, ${end} } Token; int select(const uint8_t *valid) { return valid[${end}]; }`;
    const unit = lower(source), body = unit.functions[0].body;
    assert.deepEqual(unit.enums[0].entries.map(entry => [entry.name, entry.value]), [['UNUSED', 0], ['OPEN', 1], [end, 2]]);
    const constant = body.find(item => item.sourceEnumerator === `enum:${end}`);
    assert.equal(constant.operation, 'C integer literal');
    assert.equal(constant.value, '2');
    assert.equal(constant.spelling, end);
    assert.deepEqual(constant.result.type, {kind: 'C integer', bits: 32, signed: true});
    assert.equal(Buffer.from(source).subarray(constant.span.start, constant.span.end).toString(), end);
    const index = body.find(item => item.operation === 'address C element');
    assert.equal(index.index.temporary, constant.result.temporary);
    assert.equal(body.some(item => item.operation === 'address C binding' && item.binding.startsWith('enum:')), false);
    assert.ok(unit.obligations.some(item => item.category === 'C memory addressing'));
    assert.ok(unit.obligations.some(item => item.category === 'C execution boundary'));
    assert.equal(unit.sourceCst.tokens.map(token => token.text).join(''), source);
    assert.equal(unit.checkedPortableProgram, null);
    const packet = archiveRestrictedCLowering(unit);
    assert.deepEqual(readRestrictedCLowering(packet.network, packet.root), unit);
  }
});

test('ordinary local C shadowing wins over a global enum name while its source declaration remains intact', () => {
  const unit = lower('typedef enum { OPEN, CLOSE } Token; int select(void) { int CLOSE = 7; return CLOSE; }');
  const body = unit.functions[0].body;
  const local = body.find(item => item.operation === 'declare C binding' && item.binding.name === 'CLOSE');
  assert.ok(local);
  assert.equal(unit.enums[0].entries[1].value, 1);
  assert.equal(body.some(item => item.sourceEnumerator === 'enum:CLOSE'), false);
  assert.ok(body.some(item => item.operation === 'address C binding' && item.binding === local.binding.identity));
  assert.ok(body.some(item => item.operation === 'load C lvalue'));
  assert.ok(body.some(item => item.operation === 'C integer literal' && item.value === '7'));
  assert.equal(unit.checkedPortableProgram, null);
});

test('explicit unresolved enum expressions and attempts to address or assign an enumerator remain obligations', () => {
  const unit = lower('typedef enum { FIRST, EXPLICIT = 5, AFTER } Token; int select(void) { return EXPLICIT + AFTER; }');
  assert.deepEqual(unit.enums[0].entries.map(entry => entry.value), [0, null, null]);
  const unknown = unit.functions[0].body.filter(item => item.operation === 'unresolved C constant');
  assert.deepEqual(unknown.map(item => item.sourceEnumerator), ['enum:EXPLICIT', 'enum:AFTER']);
  assert.equal(unknown.some(item => Object.hasOwn(item, 'value')), false);
  assert.ok(unit.obligations.some(item => item.category === 'C constant evaluation'));
  assert.equal(unit.functions[0].body.some(item => item.operation === 'address C binding' && item.binding.startsWith('enum:')), false);
  const invalid = lower('typedef enum { CHOICE } Token; void misuse(void) { CHOICE = 1; &CHOICE; }');
  assert.equal(invalid.obligations.filter(item => item.category === 'C enum constant lvalue').length, 2);
  assert.equal(invalid.functions[0].body.filter(item => item.operation === 'unresolved C address' && item.sourceEnumerator === 'enum:CHOICE').length, 2);
  assert.equal(invalid.functions[0].body.some(item => item.operation === 'address C binding' && item.binding.startsWith('enum:')), false);
  assert.equal(unit.checkedPortableProgram, null);
  assert.equal(invalid.checkedPortableProgram, null);
});

test('enum value and source-identity corruption cannot pass complete source-only archive replay', () => {
  const unit = lower('typedef enum { OPEN, CLOSE } Token; int select(void) { return CLOSE; }');
  for (const mutate of [
    item => { item.value = '0'; },
    item => { item.sourceEnumerator = 'enum:OPEN'; },
    item => { item.spelling = 'OPEN'; },
    item => { item.result.type.bits = 16; },
  ]) {
    const changed = structuredClone(unit);
    const constant = changed.functions[0].body.find(item => item.sourceEnumerator === 'enum:CLOSE');
    assert.ok(constant);
    mutate(constant);
    const packet = archiveRestrictedCLowering(changed);
    assert.throws(() => readRestrictedCLowering(packet.network, packet.root), /complete actual source lowering/u);
  }
  const missing = parseProgrammingLanguage('int value(void) { return UNDECLARED; }', 'C');
  const unknown = lowerRestrictedCst(missing, 'int value(void) { return UNDECLARED; }', {abi});
  assert.ok(unknown.obligations.some(item => item.category === 'C binding' && item.detail === 'UNDECLARED'));
  assert.equal(unknown.functions[0].body.some(item => item.operation === 'C integer literal'), false);
});
