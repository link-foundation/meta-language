// Deferred controls only; never import or execute while authoring the draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {executeJavaScriptCallPrimitive,readJavaScriptCallPrimitiveArityError,
 readJavaScriptCallTextReplacementWorkError} from '../src/translation/javascript-call-primitives.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
test('literal replacement admission retains all actual operands',()=>{
 for(const operation of ['replace text','replace all text']){
  assert.equal(readJavaScriptCallPrimitiveArityError(operation,3),'');
  for(const count of [0,1,2,4])assert.notEqual(readJavaScriptCallPrimitiveArityError(operation,count),'');
  for(let index=0;index<3;index+=1){const values=[text('aba'),text('b'),text('x')];values[index]={$:'undefined'};
   assert.equal(executeJavaScriptCallPrimitive(operation,values).$,'primitiveInvalid');}
 }
});
test('replacement work exhaustion stays distinct from source exceptions',()=>{
 for(const [length,search,replacement,all,reason] of [
  [1048577,1,[],false,'input'],[1,1048577,[],false,'input'],
  [1,1,Array(4097).fill(97),false,'substitution-plan'],
  [65536,1024,[97],true,'search work'],[1048576,0,[97],true,'output work'],
  [65536,0,Array.from({length:4096},(_,index)=>index%2===0?36:38),true,'substitution work']
 ])assert.ok(readJavaScriptCallTextReplacementWorkError(length,search,replacement,all).includes(reason));
 assert.equal(readJavaScriptCallTextReplacementWorkError(0,0,Array(4096).fill(97),true),'');
 assert.equal(readJavaScriptCallTextReplacementWorkError(1024,1024,[97],false),'');
 const result=executeJavaScriptCallPrimitive('replace text',[text('a'),text('a'),text('a'.repeat(4097))]);
 assert.equal(result.$,'primitiveResourceLimit');
});
