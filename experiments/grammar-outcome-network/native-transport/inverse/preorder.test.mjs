import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {checkOwnedOutcomePreorder} from './preorder-rules.mjs';
import {generateOwnedOutcomePreorder} from './generate-preorder.mjs';

test('complete generated preorder control preserves every independent shape and every lifted loop',()=>{
  const {fixture,files}=generateOwnedOutcomePreorder({check:true});
  assert.equal(fixture.cases.length,23);assert.equal(fixture.declarations.length,7);
  for(const row of fixture.cases)assert.deepEqual(checkOwnedOutcomePreorder(row.offsets,row.children,...row.limits),row.expected,row.name);
  const source=files.map(file=>file.source).join('\n');
  const parsed=readRustSourceItems(Buffer.from(source));
  for(const suffix of ['loop1','loop2','loop3']){
    const name='ml_check_owned_outcome_preorder_'+suffix;
    const owners=parsed.items.filter(item=>item.kind==='function_item'&&item.name===name);assert.equal(owners.length,1);
    const owner=parsed.text(owners[0].node);
    assert.match(owner,/\bloop\s*\{/u);assert.equal(owner.split(name+'(').length,2,'a lifted loop never calls itself');
  }
  for(const value of [NaN,Infinity,-Infinity]){
    assert.deepEqual(checkOwnedOutcomePreorder([0,0],[],value,1,1),[-1,0,0,0]);
    assert.deepEqual(checkOwnedOutcomePreorder([0,value,1],[2],2,1,2),[-2,0,0,0]);
    assert.deepEqual(checkOwnedOutcomePreorder([0,1,1],[value],2,1,2),[-4,0,0,0]);
  }
});

test('preorder worklists admit a deep bounded chain and retain exact predecessor refusals',()=>{
  const count=2048,offsets=Array.from({length:count+1},(_,index)=>Math.min(index,count-1));
  const children=Array.from({length:count-1},(_,index)=>index+2);
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count,count-1,count),[0,count,count-1,count]);
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count-1,count-1,count),[-2,0,0,0]);
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count,count-2,count),[-2,0,0,0]);
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count,count-1,count-1),[-5,0,0,0]);
});

test('the complete preorder source translates without carried items and restores every source byte',()=>{
  const source=readFileSync(new URL('./preorder-rules.mjs',import.meta.url),'utf8');
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(item=>item.status==='carried').length,0);
  assert.ok(translated.items.some(item=>item.status==='translated'));
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});
