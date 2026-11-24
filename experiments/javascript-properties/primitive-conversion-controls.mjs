// Deferred complete protocol and independent JavaScript coercion controls.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,decodeValue,encodeValue,decodeDescriptor} from './controls.mjs';
import {decodeHeap,encodeHeap,createOrdinaryHostContext,assertOrdinaryHostSnapshot} from './heap-controls.mjs';
import {beginJavaScriptPrimitiveConversion,stepJavaScriptPrimitiveConversion,resumeJavaScriptPrimitiveConversion} from '../../js/src/translation/javascript-primitive-conversion.js';
export {assertVersions};
export const primitiveFixture=readProjectData(new URL('../../parity/fixtures/javascript-primitive-conversion.lino',import.meta.url));
function decodeState(state){return {...state,receiver:decodeValue(state.receiver),exoticKey:decodeValue(state.exoticKey)};}
function encodeState(state){return {...state,receiver:encodeValue(state.receiver),exoticKey:encodeValue(state.exoticKey)};}
function encodeTransition(value){switch(value.$){
 case 'primitiveContinuation':return {...value,state:encodeState(value.state)};
 case 'primitiveCall':return {...value,state:encodeState(value.state),callee:encodeValue(value.callee),receiver:encodeValue(value.receiver),arguments:value.arguments.map(encodeValue)};
 case 'primitiveCompleted':case 'primitiveThrow':return {...value,value:encodeValue(value.value)};
 case 'primitiveIntrinsicError':case 'primitiveInvalid':return value;
 default:throw new TypeError('uncovered primitive conversion transition '+value.$);
}}
export function observePrimitiveConversion(row){
 let heap=decodeHeap(row.heap),current=beginJavaScriptPrimitiveConversion(heap,decodeValue(row.input),decodeValue(row.exoticKey),row.hint),callIndex=0;
 const trace=[encodeTransition(current)],steps=[];
 for(let count=0;current.$==='primitiveContinuation'||current.$==='primitiveCall';count+=1){
  assert.ok(count<32,'bounded control must terminate without omitting any call');
  if(current.$==='primitiveContinuation'){
   steps.push({operation:'step',heap:encodeHeap(heap)});current=stepJavaScriptPrimitiveConversion(heap,current.state);
  }else{
   const call=row.calls[callIndex++];assert.ok(call,'every modeled call has its independent literal expectation');
   assert.deepEqual(encodeValue(current.callee),call.callee);assert.deepEqual(encodeValue(current.receiver),call.receiver);assert.deepEqual(current.arguments.map(encodeValue),call.arguments);
   heap=call.heap?decodeHeap(call.heap):heap;
   steps.push({operation:'resume',heap:encodeHeap(heap),completion:call.completion});
   current=resumeJavaScriptPrimitiveConversion(heap,current.state,{...call.completion,value:decodeValue(call.completion.value)});
  }
  trace.push(encodeTransition(current));
 }
 assert.equal(callIndex,row.calls.length);assert.deepEqual(encodeTransition(current),row.expected,row.name);
 let at=0,context,hostHeap=decodeHeap(row.heap);
 context=createOrdinaryHostContext(hostHeap,call=>{
  const expected=row.calls[at++];assert.ok(expected,'the original coercion cannot make an unrecorded call');
  assert.deepEqual(encodeValue(call.callee),expected.callee);assert.deepEqual(encodeValue(call.receiver),expected.receiver);assert.deepEqual(call.arguments.map(encodeValue),expected.arguments);
  for(const mutation of expected.mutations){const target=context.objects.get(mutation.identity);assert.ok(target);
   if(mutation.kind==='define')assert.equal(Reflect.defineProperty(target,context.materialize(decodeValue(mutation.key)),context.descriptor(decodeDescriptor(mutation.descriptor))),true);
   else if(mutation.kind==='prototype')assert.equal(Reflect.setPrototypeOf(target,context.materialize(decodeValue(mutation.prototype))),true);
   else throw new TypeError('uncovered literal coercion effect');
  }
  hostHeap=expected.heap?decodeHeap(expected.heap):hostHeap;assertOrdinaryHostSnapshot(hostHeap,context);
  const result=context.materialize(decodeValue(expected.completion.value));if(expected.completion.$==='throwCompletion')throw result;return result;
 },new Map([[row.exoticKey.identity,Symbol.toPrimitive]]));
 const coerce=value=>row.hint==='number'?Number(value):row.hint==='string'?Reflect.ownKeys({[value]:true})[0]:value+'';
 let actual,failure,threw=false;try{actual=coerce(context.materialize(decodeValue(row.input)));}catch(error){failure=error;threw=true;}
 assert.equal(at,row.calls.length,'all original getters and methods execute exactly once in order');assertOrdinaryHostSnapshot(hostHeap,context);
 if(row.expected.$==='primitiveCompleted'){
  let expected,expectedFailure,expectedThrow=false;try{expected=coerce(context.materialize(decodeValue(row.expected.value)));}catch(error){expectedFailure=error;expectedThrow=true;}
  assert.equal(threw,expectedThrow);if(expectedThrow){assert.equal(failure.name,expectedFailure.name);assert.equal(failure.name,'TypeError');}else assert.ok(Object.is(actual,expected),'complete independent coercion result');
 }else if(row.expected.$==='primitiveIntrinsicError'){assert.equal(threw,true);assert.equal(failure.name,row.expected.errorClass);}
 else if(row.expected.$==='primitiveThrow'){assert.equal(threw,true);assert.deepEqual(encodeValue(context.semantic(failure)),row.expected.value);}
 else assert.fail('an invalid model is not an independently executed source operation');
 return {...row,steps,trace};
}
export function observeInvalidPrimitiveState(row){
 const heap=decodeHeap(primitiveFixture.invalidHeap),state=decodeState(row.state);
 const actual=row.operation==='resume'?resumeJavaScriptPrimitiveConversion(heap,state,{...row.completion,value:decodeValue(row.completion.value)}):stepJavaScriptPrimitiveConversion(heap,state);
 assert.deepEqual(actual,{$:'primitiveInvalid',reason:row.reason});return {...row,result:actual};
}
