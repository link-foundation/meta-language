import assert from 'node:assert/strict';
import { test } from 'node:test';
import { parseJavaScript } from '../src/translation/javascript.js';
import { hasStructuralIdentity, requiresReferenceSemantics } from '../src/translation/reference-analysis.js';

test('linked mutation operations select reference semantics', () => {
  for (const source of [
    'function grow(values){ values.push(1); return values.length; }',
    'function update(mapping){ mapping.set("word", 1); return mapping.size; }',
    'function remove(members){ return members.delete("word"); }',
    'function update(values){ values[0] = 2; return values.length; }',
    'function update(record){ record.value += 2; return record.value; }',
    'function counter(){ let count=0; const next=()=>{count+=1;return count;};return next(); }',
  ]) assert.equal(requiresReferenceSemantics(source), true, source);
  for (const source of [
    'function same(value){ return value === 1; }',
    'function scale(value){ const double=number=>number*2; return double(value); }',
    'function size(values){ let count=0; for(const value of values) count+=1; return count; }',
  ]) assert.equal(requiresReferenceSemantics(source), false, source);
});

test('inferred container equality selects identity while scalar equality preserves value mode', () => {
  const scalar = parseJavaScript('function same(left,right){return left===right;} console.log(same(1,1));');
  assert.equal(hasStructuralIdentity(scalar), false);
  assert.equal(scalar.referenceSemantics, undefined);
  const arrays = parseJavaScript('function same(left,right){return left===right;} console.log(same([1],[1]));');
  assert.equal(hasStructuralIdentity(arrays), true);
  assert.equal(arrays.referenceSemantics, true);
  const record = parseJavaScript('function same(){const first={value:1};const second=first;return first===second;}');
  assert.equal(hasStructuralIdentity(record), true);
  assert.equal(record.referenceSemantics, true);
});

test('closure assignment projects cell allocation, reads, writes and captured identities', () => {
  const program = parseJavaScript('function counter(){let count=0;const next=()=>{count=count+1;return count;};return next();}');
  assert.equal(program.referenceSemantics, true);
  const body = program.items[0].body;
  assert.equal(body.value.k, 'cellAllocate');
  const closure = body.body.value;
  assert.deepEqual(closure.assignedCaptures, ['count']);
  assert.equal(closure.body.k, 'cellWrite');
  assert.equal(closure.body.value.left.k, 'cellRead');
  assert.deepEqual(closure.body.cell.path, ['count']);
});

test('reference writes preserve the receiver and selected field or index as separate expressions', () => {
  const array = parseJavaScript('function change(){const values=[1];values[0]=3;return values[0];}');
  const write = array.items[0].body.body.value;
  assert.equal(write.k, 'referenceWrite');
  assert.deepEqual(write.object.path, ['values']);
  assert.equal(write.index.value, '0');
  assert.equal(write.value.value, '3');
  const record = parseJavaScript('function change(){const value={count:1};const alias=value;alias.count+=2;return value.count;}');
  const field = record.items[0].body.body.body.value;
  assert.equal(field.k, 'referenceWrite');
  assert.deepEqual(field.object.path, ['alias']);
  assert.equal(field.field, 'count');
  assert.equal(field.op, '+=');
});
