// Deferred complete ordinary Object.defineProperty/source transition controls.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,decodeValue,encodeValue,decodeDescriptor} from './controls.mjs';
import {decodeHeap,encodeHeap,createOrdinaryHostContext,assertOrdinaryHostSnapshot} from './heap-controls.mjs';
import {beginJavaScriptPropertyDefinition,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition} from '../../js/src/translation/javascript-property-definition.js';
export {assertVersions};
export const definitionFixture=readProjectData(new URL('../../parity/fixtures/javascript-property-definition.lino',import.meta.url));
const encodeConversion=state=>state.$==='propertyDefinitionKey'?{...state.conversion,receiver:encodeValue(state.conversion.receiver),exoticKey:encodeValue(state.conversion.exoticKey)}:
 {...state.conversion,receiver:encodeValue(state.conversion.receiver),descriptor:{...state.conversion.descriptor,value:encodeValue(state.conversion.descriptor.value),getter:encodeValue(state.conversion.descriptor.getter),setter:encodeValue(state.conversion.descriptor.setter)}};
function encodeState(state){return state.$==='propertyDefinitionKey'?{...state,target:encodeValue(state.target),attributes:encodeValue(state.attributes),conversion:encodeConversion(state)}:{...state,target:encodeValue(state.target),key:encodeValue(state.key),conversion:encodeConversion(state)};}
function encodeTransition(value){switch(value.$){
 case 'definitionContinuation':return {...value,state:encodeState(value.state)};
 case 'definitionCall':return {...value,state:encodeState(value.state),callee:encodeValue(value.callee),receiver:encodeValue(value.receiver),arguments:value.arguments.map(encodeValue)};
 case 'definitionCompleted':return {...value,heap:encodeHeap(value.heap),value:encodeValue(value.value)};
 case 'definitionThrow':return {...value,value:encodeValue(value.value)};
 case 'definitionIntrinsicError':case 'definitionInvalid':return value;
 default:throw new TypeError('uncovered definition transition '+value.$);
}}
export function observePropertyDefinition(row){
 let heap=decodeHeap(row.heap),current=beginJavaScriptPropertyDefinition(heap,decodeValue(row.target),decodeValue(row.keyInput),decodeValue(row.attributes),decodeValue(row.exoticKey)),at=0;
 const trace=[encodeTransition(current)],steps=[];
 for(let count=0;current.$==='definitionContinuation'||current.$==='definitionCall';count+=1){
  assert.ok(count<64,'every definition call/continuation remains complete');
  if(current.$==='definitionContinuation'){
   steps.push({operation:'step',heap:encodeHeap(heap)});current=stepJavaScriptPropertyDefinition(heap,current.state);
  }else{
   const call=row.calls[at++];assert.ok(call);assert.deepEqual(encodeValue(current.callee),call.callee);assert.deepEqual(encodeValue(current.receiver),call.receiver);assert.deepEqual(current.arguments.map(encodeValue),call.arguments);
   heap=call.heap?decodeHeap(call.heap):heap;steps.push({operation:'resume',heap:encodeHeap(heap),completion:call.completion});
   current=resumeJavaScriptPropertyDefinition(heap,current.state,{...call.completion,value:decodeValue(call.completion.value)});
  }
  trace.push(encodeTransition(current));
 }
 assert.equal(at,row.calls.length);assert.deepEqual(encodeTransition(current),row.expected,row.name);
 let hostAt=0,context,latest=decodeHeap(row.heap);
 context=createOrdinaryHostContext(latest,call=>{
  const expected=row.calls[hostAt++];assert.ok(expected);assert.deepEqual(encodeValue(call.callee),expected.callee);assert.deepEqual(encodeValue(call.receiver),expected.receiver);assert.deepEqual(call.arguments.map(encodeValue),expected.arguments);
  for(const mutation of expected.mutations){const target=context.objects.get(mutation.identity);assert.ok(target);
   if(mutation.kind==='define')assert.equal(Reflect.defineProperty(target,context.materialize(decodeValue(mutation.key)),context.descriptor(decodeDescriptor(mutation.descriptor))),true);
   else if(mutation.kind==='prevent')assert.equal(Reflect.preventExtensions(target),true);
   else if(mutation.kind==='prototype')assert.equal(Reflect.setPrototypeOf(target,context.materialize(decodeValue(mutation.prototype))),true);
   else throw new TypeError('uncovered definition callback effect');
  }
  latest=expected.heap?decodeHeap(expected.heap):latest;assertOrdinaryHostSnapshot(latest,context);
  const result=context.materialize(decodeValue(expected.completion.value));if(expected.completion.$==='throwCompletion')throw result;return result;
 },new Map([[row.exoticKey.identity,Symbol.toPrimitive]]));
 let result,failure,threw=false;try{result=Object.defineProperty(context.materialize(decodeValue(row.target)),context.materialize(decodeValue(row.keyInput)),context.materialize(decodeValue(row.attributes)));}catch(error){failure=error;threw=true;}
 assert.equal(hostAt,row.calls.length,'all original key/descriptor effects retain their order');
 if(row.expected.$==='definitionCompleted'){assert.equal(threw,false);assert.ok(Object.is(result,context.materialize(decodeValue(row.expected.value))));assertOrdinaryHostSnapshot(decodeHeap(row.expected.heap),context);}
 else if(row.expected.$==='definitionIntrinsicError'){assert.equal(threw,true);assert.equal(failure.name,row.expected.errorClass);assertOrdinaryHostSnapshot(latest,context);}
 else if(row.expected.$==='definitionThrow'){assert.equal(threw,true);assert.deepEqual(encodeValue(context.semantic(failure)),row.expected.value);assertOrdinaryHostSnapshot(latest,context);}
 else assert.fail('invalid model data cannot replace an independently executed source operation');
 return {...row,trace,steps};
}
export function observeInvalidPropertyDefinition(row){
 const heap=decodeHeap(definitionFixture.invalidHeap),conversion={...row.state.conversion,receiver:decodeValue(row.state.conversion.receiver),exoticKey:decodeValue(row.state.conversion.exoticKey)};
 const state={...row.state,target:decodeValue(row.state.target),attributes:decodeValue(row.state.attributes),conversion};
 const result=row.operation==='resume'?resumeJavaScriptPropertyDefinition(heap,state,{...row.completion,value:decodeValue(row.completion.value)}):stepJavaScriptPropertyDefinition(heap,state);
 assert.deepEqual(result,{$:'definitionInvalid',reason:row.reason});return {...row,result};
}
