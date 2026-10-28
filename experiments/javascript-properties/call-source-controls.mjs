// Deferred actual frontend -> shared compiler -> executor and original source.
import assert from 'node:assert/strict';
import {createContext,runInContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {lowerCheckedJavaScriptCallModule} from '../../js/src/translation/javascript-call-lowering.js';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine} from '../../js/src/translation/javascript-call-machine.js';
import {encodeJavaScriptCallSource,encodeJavaScriptCallSourceCompilation} from './call-source-codecs.mjs';
import {decodeJavaScriptSemanticValue,encodeJavaScriptSemanticValue,encodeJavaScriptCallProgram,encodeJavaScriptCallMachineTransition,encodeJavaScriptOrdinaryHeap} from './call-machine-codecs.mjs';
import {assertVersions} from './controls.mjs';
import {createOrdinaryHostContext,assertOrdinaryHostSnapshot} from './heap-controls.mjs';
export {assertVersions};
export const callSourceFixture=readProjectData(new URL('../../parity/fixtures/javascript-call-source.lino',import.meta.url));
function registry(row,checked){const names=[...row.functions,...[...checked.declarations].filter(([name,entry])=>entry.k==='fn'&&!row.functions.includes(name)).map(([name])=>name)];return {functions:names.map((name,index)=>({name,identity:10n+BigInt(index),realm:1000n})),realms:[1000n],exoticKey:{$:'symbol',identity:18446744073709551615n}};}
function decodeObject(object){return {...object,identity:BigInt(object.identity),prototype:decodeJavaScriptSemanticValue(object.prototype),properties:object.properties.map(property=>({...property,key:decodeJavaScriptSemanticValue(property.key),descriptor:{...property.descriptor,value:decodeJavaScriptSemanticValue(property.descriptor.value),getter:decodeJavaScriptSemanticValue(property.descriptor.getter),setter:decodeJavaScriptSemanticValue(property.descriptor.setter)}}))};}
export function observeCallSource(row){
 const checked=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics===true})),actualRegistry=registry(row,checked),lowered=lowerCheckedJavaScriptCallModule(checked,actualRegistry);
 const heap={$:'ordinaryHeap',objects:[...(row.objects??[]).map(decodeObject),...actualRegistry.functions.map(fn=>({$:'ordinaryObject',identity:fn.identity,prototype:{$:'null'},extensible:true,callable:true,properties:[]}))].sort((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0),symbols:[actualRegistry.exoticKey.identity]};
 const arguments_=row.arguments.map(decodeJavaScriptSemanticValue),entry=actualRegistry.functions.find(fn=>fn.name===row.entry).identity;
 let current=beginJavaScriptCallMachine(lowered.program,heap,entry,{$:'undefined'},arguments_);const trace=[encodeJavaScriptCallMachineTransition(current)];
 for(let steps=0;current.$==='machineContinue';steps+=1){assert.ok(steps<4096,'every actual frontend invocation must reach its full terminal observation');current=stepJavaScriptCallMachine(lowered.program,current.machine);trace.push(encodeJavaScriptCallMachineTransition(current));}
 if(row.maximumFrames!==undefined)assert.ok(Math.max(...trace.filter(entry=>entry.$==='machineContinue').map(entry=>entry.machine.frames.length))<=row.maximumFrames,'lifted loops preserve their complete bounded active frame count');
 const expectedHeap=row.afterObjects?{...heap,objects:heap.objects.map(object=>{const replacement=row.afterObjects.find(entry=>BigInt(entry.identity)===object.identity);return replacement?decodeObject(replacement):object;})}:heap;
 const finalHeap=current.$==='machineAllocateError'||current.$==='machineAllocateSourceError'?current.machine.heap:current.heap;
 if(row.expectedOutcome==='allocation'){
  assert.equal(current.$,'machineAllocateError');assert.equal(String(current.realm),row.errorRealm);assert.equal(current.errorClass,row.errorClass);assert.equal(current.reason,row.errorReason);
 }else if(row.expectedOutcome==='sourceAllocation'){
  assert.equal(current.$,'machineAllocateSourceError');assert.equal(String(current.realm),row.errorRealm);assert.equal(current.errorClass,row.errorClass);
  assert.deepEqual(encodeJavaScriptSemanticValue(current.message),row.errorMessage);assert.equal(current.messagePresent,row.messagePresent);
 }else{
  assert.equal(current.$,'machineCompleted');
  if(row.expectedEncoding==='implementation-selected NaN'){
   assert.equal(current.value.$,'number');assert.equal(Number.isNaN(current.value.value),true);
   assert.equal(Number.isNaN(decodeJavaScriptSemanticValue(row.expected).value),true,'the retained expected Number is still NaN');
  }else assert.deepEqual(encodeJavaScriptSemanticValue(current.value),row.expected);
 }
 assert.deepEqual(encodeJavaScriptOrdinaryHeap(finalHeap),encodeJavaScriptOrdinaryHeap(expectedHeap));
 const originalRealm=createContext(),originals=runInContext('(function(){"use strict";'+row.source.replaceAll('export function ','function ')+';return {'+row.functions.join(',')+'};})()',originalRealm);
 let context;context=createOrdinaryHostContext(heap,call=>{
  const owner=actualRegistry.functions.find(fn=>fn.identity===call.callee.identity);assert.ok(owner,'actual source owns every registered property call');
  return Reflect.apply(originals[owner.name],context.materialize(call.receiver),call.arguments.map(context.materialize));
 },new Map([[String(actualRegistry.exoticKey.identity),runInContext('Symbol.toPrimitive',originalRealm)]]));
 let result,failure,threw=false;try{result=Reflect.apply(originals[row.entry],undefined,arguments_.map(context.materialize));}catch(error){failure=error;threw=true;}
 if(row.expectedOutcome==='allocation'||row.expectedOutcome==='sourceAllocation'){
  assert.equal(threw,true);assert.equal(failure.name,row.errorClass);
  assert.equal(Object.getPrototypeOf(failure),runInContext(row.errorClass,originalRealm).prototype,'the actual source realm owns the intrinsic Error instance');
  if(row.expectedOutcome==='sourceAllocation'){
   assert.equal(Object.hasOwn(failure,'message'),row.messagePresent,'source own-message presence is independently observable');
   assert.deepEqual(encodeJavaScriptSemanticValue(context.semantic(failure.message)),row.messagePresent?row.errorMessage:{$:'text',units:[]},'source messages retain every original UTF16 unit');
  }
 }else{
  assert.equal(threw,false);
  const observed=encodeJavaScriptSemanticValue(context.semantic(result));
  assert.deepEqual(observed,row.expectedEncoding==='implementation-selected NaN'?encodeJavaScriptSemanticValue(current.value):row.expected,'independent original source behavior retains every observed Number bit');
 }
 assertOrdinaryHostSnapshot(expectedHeap,context);
 const registeredCalls=context.calls.map(call=>({callee:encodeJavaScriptSemanticValue(call.callee),receiver:encodeJavaScriptSemanticValue(call.receiver),arguments:call.arguments.map(encodeJavaScriptSemanticValue)}));
 assert.deepEqual(registeredCalls,row.registeredCalls??[],'all independently observed registered property calls');
 const machineRegisteredCalls=[];
 for(let index=1;index<trace.length;index+=1){const before=trace[index-1],after=trace[index];if(before.$!=='machineContinue'||after.$!=='machineContinue'||after.machine.frames.length<=before.machine.frames.length)continue;
  const child=after.machine.frames.at(-1);if(!(row.registeredCallIdentities??[]).includes(child.identity))continue;
  machineRegisteredCalls.push({callee:{$:'object',identity:child.identity,callable:true},receiver:child.registers[0],arguments:child.arguments});
 }
 assert.deepEqual(machineRegisteredCalls,registeredCalls,'compiled source retains every actual property call receiver/argument/effect');
 return {...row,entry:String(entry),program:encodeJavaScriptCallProgram(lowered.program),heap:encodeJavaScriptOrdinaryHeap(heap),trace,
  sourceUnits:lowered.sourceUnits.map(unit=>({...unit,source:encodeJavaScriptCallSource(unit.source),compilation:encodeJavaScriptCallSourceCompilation(unit.compilation)}))};
}
export function observeCallSourceRefusal(row){const checked=checkProgram(parseJavaScript(row.source));assert.throws(()=>lowerCheckedJavaScriptCallModule(checked,registry(row,checked)),/registered call source lowering/);}
