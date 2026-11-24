// Deferred admission controls. Independent source originals exercise behavior.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readCheckedJavaScriptIntegerResult} from '../src/translation/javascript-call-integer-lowering.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
assertVersions();
const originals=[
 ['text','/** @param {string} input @returns {bigint} */ export function inspect(input){return BigInt(input);}','convert explicit integer'],
 ['number','/** @param {number} input @returns {bigint} */ export function inspect(input){return BigInt(input);}','convert explicit integer'],
 ['division','/** @param {bigint} left @param {bigint} right @returns {bigint} */ export function inspect(left,right){return left/right;}','divide integers'],
 ['remainder','/** @param {bigint} left @param {bigint} right @returns {bigint} */ export function inspect(left,right){return left%right;}','read integer remainder'],
 ['power','/** @param {bigint} left @param {bigint} right @returns {bigint} */ export function inspect(left,right){return left**right;}','raise integer power']
];
function findResult(node){
 if(!node||typeof node!=='object')return null;
 if(readCheckedJavaScriptIntegerResult(node))return node;
 for(const value of Object.values(node)){if(Array.isArray(value)){for(const entry of value){const found=findResult(entry);if(found)return found;}}else{const found=findResult(value);if(found)return found;}}
 return null;
}
for(const [name,source,operation]of originals)test('complete integer result admission '+name,()=>{
 const checked=checkProgram(parseJavaScript(source));
 const result=findResult(checked.declarations.get('inspect'));
 assert.ok(result,'the actual checked frontend protocol must be recognized');
 assert.equal(readCheckedJavaScriptIntegerResult(result).operation,operation);
 for(const change of [
  node=>{node.body.cond.right.value='99';},
  node=>{node.body.cond.left.array.name+='different';},
  node=>{node.body.then.right.value='1';},
  node=>{node.body.then.left.array.name+='different';},
  node=>{node.body.then.left.index.value='99';},
  node=>{node.body.else.op='raiseJavaScriptEvalError';},
  node=>{node.body.else.args[1].value=false;},
  node=>{node.body.else.args[0]={k:'lit',type:{kind:'string'},value:'different error'};},
  node=>{node.body.else.args=null;},
  node=>{node.value.ignoredEffect={k:'call',fn:'sideEffect'};},
  node=>{node.body.ignoredEffect=true;}
 ]){
  const changed=structuredClone(result);change(changed);
  assert.equal(readCheckedJavaScriptIntegerResult(changed),null,'a changed observable success/error protocol must refuse fusion');
 }
});
