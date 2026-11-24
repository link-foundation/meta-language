import assert from 'node:assert/strict';
import test from 'node:test';
import {createOriginalLinkRawProfileCache} from '../../experiments/original-link-complete-table-projection/raw-profile-cache.mjs';

test('raw profile reuse requires every complete input byte and retains independent caller values', () => {
  const select = createOriginalLinkRawProfileCache();
  const inputs = {artifact: Buffer.from('complete Ω artifact'), producer: Buffer.from('whole producer')};
  let calls = 0;
  const verify = () => {calls++; return {declarations: ['actual checked declaration'], profile: 'current'};};
  const first = select('current', inputs, verify);
  first.declarations.push('caller edit');
  const second = select('current', {...inputs, artifact: Buffer.from(inputs.artifact)}, verify);
  assert.equal(calls, 1); assert.deepEqual(second.declarations, ['actual checked declaration']);
  second.declarations.push('another caller edit');
  assert.deepEqual(select('current', inputs, verify).declarations, ['actual checked declaration']);
  const failure = new Error('original verifier refuses');
  for (const changed of [
    {...inputs, artifact: Buffer.concat([inputs.artifact, Buffer.from('\n')])},
    {...inputs, producer: Buffer.from('different producer')},
    {producer: inputs.producer, artifact: inputs.artifact},
    {artifact: inputs.artifact}, {...inputs, extra: Buffer.from('extra input')},
  ]) {
    assert.throws(() => select('current', changed, () => {throw failure;}), error => error === failure);
  }
  assert.throws(() => select('historical', inputs, () => {throw failure;}), error => error === failure);
  assert.equal(calls, 1);
  inputs.producer[0] = 0;
  assert.throws(() => select('current', inputs, () => {throw failure;}), error => error === failure);
});

test('raw profile failures and input mutation cannot acquire cached authority', () => {
  const select = createOriginalLinkRawProfileCache(), inputs = {source: Buffer.from('original')};
  for (let attempt = 0; attempt < 2; attempt++) {
    assert.throws(() => select('current', inputs, () => {throw new Error('failed verification');}), /failed verification/u);
  }
  assert.throws(() => select('current', inputs, () => {inputs.source[0] = 0; return {}; }), /inputs changed/u);
  assert.throws(() => select('current', inputs, () => {inputs.extra = Buffer.from('added'); return {}; }), /inputs changed/u);
  let calls = 0;
  const result = select('current', inputs, () => {calls++; return {checked: true};});
  assert.deepEqual(result, {checked: true}); assert.equal(calls, 1);
  assert.throws(() => select('current', {source: 'not bytes'}, () => result));
});
