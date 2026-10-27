// Deferred exact key/transition and original computed-property controls.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,decodeValue,encodeValue} from './controls.mjs';
import {decodeHeap,encodeHeap,createOrdinaryHostContext} from './heap-controls.mjs';
import {observePrimitiveConversion} from './primitive-conversion-controls.mjs';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from '../../js/src/translation/javascript-property-key-conversion.js';
export {assertVersions};
export const keyFixture=readProjectData(new URL('../../parity/fixtures/javascript-property-key-conversion.lino',import.meta.url));
const encodeState=state=>({...state,receiver:encodeValue(state.receiver),exoticKey:encodeValue(state.exoticKey)});
function encodeTransition(value){switch(value.$){
 case 'propertyKeyContinuation':return {...value,state:encodeState(value.state)};
 case 'propertyKeyCall':return {...value,state:encodeState(value.state),callee:encodeValue(value.callee),receiver:encodeValue(value.receiver),arguments:value.arguments.map(encodeValue)};
 case 'propertyKeyCompleted':return {...value,key:encodeValue(value.key)};
 case 'propertyKeyThrow':return {...value,value:encodeValue(value.value)};
 case 'propertyKeyIntrinsicError':case 'propertyKeyInvalid':return value;
 default:throw new TypeError('uncovered property key transition '+value.$);
}}
export function observePropertyKeyConversion(row){
 // This executes original computed-property syntax and all exact literal
 // getters/method effects; the independent primitive result remains explicit.
 observePrimitiveConversion({...row,expected:row.primitiveExpected});
 if(row.expected.$==='propertyKeyCompleted'){
  const context=createOrdinaryHostContext(decodeHeap(row.heap),undefined,new Map([[row.exoticKey.identity,Symbol.toPrimitive]]));
  const primitive=context.materialize(decodeValue(row.primitiveExpected.value)),key=context.materialize(decodeValue(row.expected.key));
  assert.ok(Object.is(Reflect.ownKeys({[primitive]:true})[0],key),'independent literal final property key');
 }
 let heap=decodeHeap(row.heap),current=beginJavaScriptPropertyKeyConversion(heap,decodeValue(row.input),decodeValue(row.exoticKey)),at=0;
 const trace=[encodeTransition(current)],steps=[];
 for(let count=0;current.$==='propertyKeyContinuation'||current.$==='propertyKeyCall';count+=1){
  assert.ok(count<32,'every key continuation must finish without omitting calls');
  if(current.$==='propertyKeyContinuation'){
   steps.push({operation:'step',heap:encodeHeap(heap)});current=stepJavaScriptPropertyKeyConversion(heap,current.state);
  }else{
   const call=row.calls[at++];assert.ok(call);assert.deepEqual(encodeValue(current.callee),call.callee);assert.deepEqual(encodeValue(current.receiver),call.receiver);assert.deepEqual(current.arguments.map(encodeValue),call.arguments);
   heap=call.heap?decodeHeap(call.heap):heap;steps.push({operation:'resume',heap:encodeHeap(heap),completion:call.completion});
   current=resumeJavaScriptPropertyKeyConversion(heap,current.state,{...call.completion,value:decodeValue(call.completion.value)});
  }
  trace.push(encodeTransition(current));
 }
 assert.equal(at,row.calls.length);assert.deepEqual(encodeTransition(current),row.expected,row.name);return {...row,trace,steps};
}
export function observeInvalidPropertyKeyState(row){
 const heap=decodeHeap(keyFixture.invalidHeap),state={...row.state,receiver:decodeValue(row.state.receiver),exoticKey:decodeValue(row.state.exoticKey)};
 const result=row.operation==='resume'?resumeJavaScriptPropertyKeyConversion(heap,state,{...row.completion,value:decodeValue(row.completion.value)}):stepJavaScriptPropertyKeyConversion(heap,state);
 assert.deepEqual(result,{$:'propertyKeyInvalid',reason:row.reason});return {...row,result};
}
