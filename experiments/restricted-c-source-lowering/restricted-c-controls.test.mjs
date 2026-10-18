// Authored controls. No native compilation or upstream corpus work.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseProgrammingLanguage} from '../../js/src/programming-language-parser.js';
import {lowerRestrictedCst, archiveRestrictedCLowering, readRestrictedCLowering} from './restricted-c-cst.mjs';

const abi = {intBits: 32, sizeBits: 64, charSigned: true};
const lower = (source, options = {}) => lowerRestrictedCst(parseProgrammingLanguage(source, 'C'), source, {abi, ...options});
const instructions = body => body.flatMap(item => [item,
  ...instructions(item.whenTrue?.instructions ?? (Array.isArray(item.whenTrue) ? item.whenTrue : [])),
  ...instructions(item.whenFalse?.instructions ?? (Array.isArray(item.whenFalse) ? item.whenFalse : [])),
  ...instructions(item.condition?.instructions ?? []), ...instructions(item.body ?? []), ...instructions(item.update ?? [])]);

test('source lowering refuses original recovered trees and retained nested missing-node flags',()=>{
  for(const source of ['int value(void) { return ; ? }','int value(void) { return 1;']) {
    const parsed=parseProgrammingLanguage(source,'C');
    assert.throws(()=>lowerRestrictedCst(parsed,source,{abi}),/recovered C CSTs cannot be silently admitted/u);
  }
  const source='int value(void) { return 1; }',parsed=parseProgrammingLanguage(source,'C');
  for(const flags of [{hasError:true},{isError:true},{isMissing:true}]) {
    const changed=structuredClone(parsed);changed.tree.children[0].flags={...changed.tree.children[0].flags,...flags};
    assert.throws(()=>lowerRestrictedCst(changed,source,{abi}),/recovered C CSTs cannot be silently admitted/u);
  }
});

test('arbitrary source names lower the same uint8 update structure with explicit promotion and write conversion', () => {
  for (const name of ['increment', 'completelyDifferent']) {
    const source = `/* 前 */ uint8_t ${name}(uint8_t value) { value++; return value; }`;
    const unit = lower(source, {roots: [name]});
    const fn = unit.functions[0], body = instructions(fn.body);
    assert.equal(fn.name, name);
    assert.equal(fn.parameters[0].type.bits, 8);
    assert.equal(body.filter(item => item.operation === 'store C lvalue').length, 1);
    assert.ok(body.some(item => item.operation === 'convert C value'
      && item.reason === 'C integer promotion' && item.target.bits === 32));
    assert.ok(body.some(item => item.operation === 'convert C value'
      && item.reason === 'C assignment conversion' && item.target.bits === 8
      && item.conversion.rule === 'modulo two to bits'));
    assert.equal(unit.sourceCst.tokens.map(token => token.text).join(''), source);
    assert.equal(unit.checkedPortableProgram, null);
  }
});

test('short circuit retains actual source calls inside the selected branch and follows ordinary declared closure', () => {
  const source = 'int effect(void) { return 1; } int choose(int input) { return input && effect(); }';
  const unit = lower(source, {roots: ['choose']});
  assert.deepEqual(unit.functions.map(fn => fn.name), ['effect', 'choose']);
  const fn = unit.functions.find(fn => fn.name === 'choose');
  const selected = fn.body.find(item => item.operation === 'C short circuit');
  assert.ok(selected);
  assert.equal(selected.result.type.bits, 32);
  assert.equal(fn.body.some(item => item.operation === 'C call'), false);
  assert.equal(selected.whenTrue.instructions.filter(item => item.operation === 'C call').length, 1);
  assert.equal(selected.whenFalse.instructions.some(item => item.operation === 'C call'), false);
  assert.equal(selected.whenTrue.instructions.find(item => item.operation === 'C call').sourceFunction, 'effect');
});

test('typed field compound assignment materializes one address and one old read before the actual RHS call', () => {
  const source = 'typedef struct { uint8_t count; } Counter; int supply(void) { return 2; } uint8_t adjust(Counter *owner) { owner->count += supply(); return owner->count; }';
  const unit = lower(source, {roots: ['adjust']});
  const fn = unit.functions.find(fn => fn.name === 'adjust');
  const storeIndex = fn.body.findIndex(item => item.operation === 'store C lvalue');
  const prefix = fn.body.slice(0, storeIndex + 1);
  const addresses = prefix.filter(item => item.operation === 'address C field');
  assert.equal(addresses.length, 1);
  const address = addresses[0].result.temporary;
  assert.equal(prefix.filter(item => item.operation === 'load C lvalue' && item.address.temporary === address).length, 1);
  assert.equal(prefix.at(-1).address.temporary, address);
  assert.ok(prefix.findIndex(item => item.operation === 'load C lvalue' && item.address.temporary === address)
    < prefix.findIndex(item => item.operation === 'C call'));
  assert.equal(prefix.at(-1).value.type.bits, 8);
  assert.ok(unit.obligations.some(item => item.category === 'C memory addressing'));
  assert.ok(unit.obligations.some(item => item.category === 'C operand sequencing'));
  assert.equal(unit.checkedPortableProgram, null, 'an annotated candidate order is not yet a checked C alias/sequencing proof');
});

test('external calls and unimplemented C forms remain obligations and cannot replace a declared helper', () => {
  const source = 'void reserve(void) { void *value = malloc(8); } int width(void) { return sizeof(int); }';
  const unit = lower(source);
  assert.ok(unit.obligations.some(item => item.category === 'C external primitive' && item.detail === 'malloc'));
  assert.ok(unit.obligations.some(item => item.category === 'C expression form'
    && ['sizeof_expression', 'size_of_expression'].includes(item.detail)));
  assert.ok(instructions(unit.functions.find(fn => fn.name === 'width').body)
    .some(item => item.operation === 'unresolved C expression'));
  assert.throws(() => lower(source, {primitives: {reserve: {kind: 'C function', result: {kind: 'C void'}, parameters: []}}}),
    /source function cannot be replaced/u);
});

test('const qualification survives pointer/index lowering and invalid const stores remain explicit', () => {
  const read = lower('int inspect(const uint8_t *values) { return values[0]; }');
  const parameter = read.functions[0].parameters[0];
  assert.equal(parameter.type.kind, 'C pointer');
  assert.deepEqual(parameter.type.to.qualifiers, ['const']);
  const loaded = instructions(read.functions[0].body).find(item => item.operation === 'load C lvalue'
    && item.sourceType.kind === 'C qualified');
  assert.ok(loaded);
  assert.equal(loaded.result.type.bits, 8);
  const write = lower('void alter(const uint8_t *values) { values[0] = 1; }');
  assert.ok(write.obligations.some(item => item.category === 'C const write'));
});

test('actual CST field/span provenance and lowering obligations survive the existing shared Links codec', () => {
  const source = 'uint8_t value(uint8_t input) { while (input < 3) { input++; } return input; }';
  const unit = lower(source);
  const packet = archiveRestrictedCLowering(unit);
  assert.deepEqual(readRestrictedCLowering(packet.network, packet.root), unit);
  assert.equal(unit.sourceCst.tree.term, 'translation_unit');
  assert.ok(unit.functions[0].body.some(item => item.operation === 'C while'));
  assert.equal(unit.checkedPortableProgram, null);
  const parsed = parseProgrammingLanguage(source, 'C');
  assert.throws(() => lowerRestrictedCst(parsed, source.replace('< 3', '< 4'), {abi}), /source byte/u);
  assert.throws(() => lowerRestrictedCst({...parsed, canonical: 'JavaScript'}, source, {abi}), /repository C/u);
});

test('source-only C archive replay refuses altered instructions receipts scopes and original CST bytes',()=>{
  const source='uint8_t value(uint8_t input) { input++; return input; }',unit=lower(source);
  for(const mutate of [
    value=>{value.sourceSha256='0'.repeat(64);},
    value=>{value.sourceBytes++;},
    value=>{value.sourceCst.tokens[0].text+=' ';},
    value=>{value.functions[0].body.pop();},
    value=>{value.functions[0].parameters[0].type.bits=16;},
    value=>{value.roots=['missing'];},
    value=>{value.obligations=[];},
    value=>{delete value.loweringOptions;},
    value=>{value.checkedPortableProgram={schemaVersion:1};},
  ]) {
    const changed=structuredClone(unit);mutate(changed);const packet=archiveRestrictedCLowering(changed);
    assert.throws(()=>readRestrictedCLowering(packet.network,packet.root),TypeError);
  }
  const declared=lower('uint8_t value(uint8_t input) { return input; }',{types:{Alias:{kind:'C integer',bits:8,signed:false}}});
  const packet=archiveRestrictedCLowering(declared);
  assert.deepEqual(readRestrictedCLowering(packet.network,packet.root),declared);
  assert.equal(declared.checkedPortableProgram,null,'source consistency does not establish executable C semantics');
});
