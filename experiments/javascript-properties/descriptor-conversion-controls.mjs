// Deferred independent ordinary-object oracle and complete continuation traces.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,decodeValue,encodeValue,decodeDescriptor,encodeDescriptor,numberFromBits} from './controls.mjs';
import {decodeHeap,createOrdinaryHostContext,assertOrdinaryHostSnapshot} from './heap-controls.mjs';
import {beginJavaScriptDescriptorConversion,resumeJavaScriptDescriptorConversion,stepJavaScriptDescriptorConversion,convertJavaScriptSemanticBoolean} from '../../js/src/translation/javascript-descriptor-conversion.js';
export {assertVersions};
export const conversionFixture=readProjectData(new URL('../../parity/fixtures/javascript-descriptor-conversion.lino',import.meta.url));
export function decodeState(state,nextBits){return {...state,receiver:decodeValue(state.receiver),descriptor:decodeDescriptor(state.descriptor),next:nextBits===undefined?state.next:numberFromBits(nextBits)};}
function encodeState(state){return {...state,receiver:encodeValue(state.receiver),descriptor:encodeDescriptor(state.descriptor)};}
export function encodeTransition(transition){switch(transition.$){
 case 'descriptorContinuation':return {...transition,state:encodeState(transition.state)};
 case 'descriptorCall':return {...transition,state:encodeState(transition.state),callee:encodeValue(transition.callee),receiver:encodeValue(transition.receiver),arguments:transition.arguments.map(encodeValue)};
 case 'descriptorCompleted':return {...transition,descriptor:encodeDescriptor(transition.descriptor)};
 case 'descriptorThrow':return {...transition,value:encodeValue(transition.value)};
 case 'descriptorIntrinsicError':case 'descriptorInvalid':return transition;
 default:throw new TypeError('uncovered descriptor transition '+transition.$);
}}
export function observeDescriptorConversion(row){
 let current=beginJavaScriptDescriptorConversion(decodeHeap(row.heap),decodeValue(row.receiver));const trace=[encodeTransition(current)];
 for(const step of row.steps){assert.equal(current.$,'descriptorCall',row.name);assert.equal(current.state.pending,true);assert.deepEqual(encodeValue(current.callee),step.callee);assert.deepEqual(encodeValue(current.receiver),step.receiver);assert.deepEqual(current.arguments.map(encodeValue),step.arguments);
  const completion={...step.completion,value:decodeValue(step.completion.value)};
  current=resumeJavaScriptDescriptorConversion(decodeHeap(step.heap),current.state,completion);trace.push(encodeTransition(current));
 }
 assert.deepEqual(encodeTransition(current),row.expected,row.name);
 let at=0,context;
 context=createOrdinaryHostContext(decodeHeap(row.heap),call=>{
  const step=row.steps[at++];assert.ok(step,'the original operation must not make an unrecorded call');assert.deepEqual(encodeValue(call.callee),step.callee);assert.deepEqual(encodeValue(call.receiver),step.receiver);assert.deepEqual(call.arguments.map(encodeValue),step.arguments);
  for(const mutation of step.mutations){const receiver=context.objects.get(mutation.identity);assert.ok(receiver);
   if(mutation.kind==='define')assert.equal(Reflect.defineProperty(receiver,context.materialize(decodeValue(mutation.key)),context.descriptor(decodeDescriptor(mutation.descriptor))),true);
   else if(mutation.kind==='prototype')assert.equal(Reflect.setPrototypeOf(receiver,context.materialize(decodeValue(mutation.prototype))),true);
   else throw new TypeError('uncovered literal getter effect');
  }
  assertOrdinaryHostSnapshot(decodeHeap(step.heap),context);
  const result=context.materialize(decodeValue(step.completion.value));if(step.completion.$==='throwCompletion')throw result;return result;
 });
 const target=Object.create(null);let failure,threw=false;
 try{Object.defineProperty(target,'property',context.materialize(decodeValue(row.receiver)));}catch(error){failure=error;threw=true;}
 assert.equal(at,row.steps.length,'the independent original operation must execute exactly all retained calls');
 if(row.expected.$==='descriptorCompleted'){
  assert.equal(threw,false);const actual=Object.getOwnPropertyDescriptor(target,'property'),partial=decodeDescriptor(row.expected.descriptor),expected=context.descriptor(partial);
  if('get'in expected||'set'in expected){if(!('get'in expected))expected.get=undefined;if(!('set'in expected))expected.set=undefined;}
  else {if(!('value'in expected))expected.value=undefined;if(!('writable'in expected))expected.writable=false;}
  if(!('enumerable'in expected))expected.enumerable=false;if(!('configurable'in expected))expected.configurable=false;
  assert.deepEqual(Reflect.ownKeys(actual).sort(),Reflect.ownKeys(expected).sort());for(const key of Reflect.ownKeys(expected))assert.ok(Object.is(actual[key],expected[key]),'complete independent descriptor attribute '+key);
 }else if(row.expected.$==='descriptorIntrinsicError'){assert.equal(threw,true);assert.equal(failure.name,row.expected.errorClass);assert.equal(Object.hasOwn(target,'property'),false);}
 else if(row.expected.$==='descriptorThrow'){assert.equal(threw,true);assert.deepEqual(encodeValue(context.semantic(failure)),row.expected.value);assert.equal(Object.hasOwn(target,'property'),false);}
 else assert.fail('an invalid model cannot stand in for an independent descriptor operation');
 return {...row,trace};
}
export function observeSemanticBoolean(row){const value=decodeValue(row.value),heap=decodeHeap(conversionFixture.invalidHeap),context=createOrdinaryHostContext(heap),actual=convertJavaScriptSemanticBoolean(value);assert.equal(actual,row.expected,row.name);assert.equal(actual,Boolean(context.materialize(value)),row.name);return {...row,actual};}
export function observeInvalidDescriptorState(row){const heap=decodeHeap(conversionFixture.invalidHeap),state=decodeState(row.state,row.nextBits);
 const result=row.operation==='resume'?resumeJavaScriptDescriptorConversion(heap,state,{...row.completion,value:decodeValue(row.completion.value)}):stepJavaScriptDescriptorConversion(heap,state);
 assert.deepEqual(result,{$:'descriptorInvalid',reason:row.reason});return {...row,result};}
