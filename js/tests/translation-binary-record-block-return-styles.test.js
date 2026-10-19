import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readBinaryRecordBlockReturnStyles} from '../scripts/binary-record-block-return-styles.mjs';
import {tokenize} from '../src/translation/lexer.js';

const review=readProjectData(new URL('../../parity/self-translation/binary-record-ci-block-return-review-116894548503.lino',import.meta.url));
const sources=()=>review.functions.map(row=>row.source);

test('binary block return review retains all actual lint and source placement findings',()=>{
  assert.equal(createHash('sha256').update(readFileSync(new URL('../../parity/self-translation/binary-record-ci-block-return-review-116894548503.lino',import.meta.url))).digest('hex'),'76da94763594f82aaab72fe3801ec4859158d5e1fabcd54e6f9520355e50eee5');
  assert.equal(review.capture.logBytes,163319);assert.equal(review.capture.head,'3624201f14da61a143635baec1b7ecc377200517');
  assert.equal(review.diagnostics.length,3);assert.equal(review.sourceTestFindings.length,5);
  assert.deepEqual(review.functions[0].edits.map(row=>row.binding),['output_5','output_10','output_11']);
  assert.equal(readBinaryRecordBlockReturnStyles(sources()).actions.length,1);
});

test('binary block return corrections preserve every returned expression and other token',()=>{
  const item=review.functions[0],action=readBinaryRecordBlockReturnStyles(sources()).actions[0],before=tokenize(item.source,'Rust').tokens;
  const removed=[];
  for(const edit of item.edits){
    const local=before.filter(token=>token.start>=edit.start&&token.end<=edit.end);
    assert.equal(local[0].value,'let');assert.equal(local.at(-1).value,local[1].value);
    removed.push(...local.slice(0,3),...local.slice(-2));
  }
  const omitted=new Set(removed.map(token=>token.start));assert.equal(omitted.size,15);
  assert.deepEqual(tokenize(action.to,'Rust').tokens.map(token=>[token.kind,token.value]),before.filter(token=>!omitted.has(token.start)).map(token=>[token.kind,token.value]));
});

test('binary block return review refuses stale overlap altered expressions and missing findings',()=>{
  for(const change of [
    value=>{value.functions[0].sha256='0'.repeat(64);},
    value=>{value.functions[0].edits[0].to='0f64';},
    value=>{value.functions[0].edits[1]={...value.functions[0].edits[0]};},
    value=>{value.diagnostics.pop();},
    value=>{value.sourceTestFindings.pop();},
    value=>{value.reviewKind='MachineApplicable';},
  ]){const value=structuredClone(review);change(value);assert.throws(()=>readBinaryRecordBlockReturnStyles(sources(),value));}
  assert.throws(()=>readBinaryRecordBlockReturnStyles([...sources(),...sources()]),/exactly once/u);
  assert.throws(()=>readBinaryRecordBlockReturnStyles(['pub fn stale() {}']),/exactly once/u);
});
