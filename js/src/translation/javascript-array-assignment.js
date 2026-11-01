// Drive BOTH ArraySetLength numeric conversions through actual shared-heap
// callbacks. Preserve partial deletion and never replay a completed conversion.
import {beginJavaScriptArrayPropertyDefinition,stepJavaScriptArrayPropertyDefinition,resumeJavaScriptArrayPropertyNumber,readJavaScriptArrayDefinitionStateError} from './javascript-arrays.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'arrayAssignmentDefinition',definition:JavaScriptArrayDefinitionState,exoticKey:JavaScriptSemanticValue} |
 * {$:'arrayAssignmentNumber',definition:JavaScriptArrayDefinitionState,exoticKey:JavaScriptSemanticValue,conversion:JavaScriptPrimitiveConversionState} |
 * {$:'arrayAssignmentPrimitive',definition:JavaScriptArrayDefinitionState,exoticKey:JavaScriptSemanticValue,value:JavaScriptSemanticValue}} JavaScriptArrayAssignmentState */
/** @typedef {{$:'arrayAssignmentContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayAssignmentState} |
 * {$:'arrayAssignmentCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayAssignmentState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayAssignmentCompleted',heap:JavaScriptOrdinaryHeap,success:boolean,writes?:string[]} |
 * {$:'arrayAssignmentThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayAssignmentError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayAssignmentInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayAssignmentResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayAssignmentState,reason:string}} JavaScriptArrayAssignmentTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayAssignmentState} state @returns {string} */
export function readJavaScriptArrayAssignmentStateError(heap,state){
 const error=readJavaScriptArrayDefinitionStateError(heap,state.definition);if(error!=='')return error;
 const keyError=readJavaScriptOrdinaryValueError(heap,state.exoticKey);if(keyError!=='')return keyError;
 if(state.exoticKey.$!=='symbol')return 'Array assignment retains its actual well-known primitive-conversion Symbol';
 switch(state.$){
  case 'arrayAssignmentDefinition':return '';
  case 'arrayAssignmentNumber':{
   if(state.definition.phase==='apply'||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.definition.descriptor.value)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Array numeric conversion retains its actual input, phase, Symbol and number hint';
   return readJavaScriptPrimitiveConversionStateError(heap,state.conversion);
  }
  case 'arrayAssignmentPrimitive':{
   if(state.definition.phase==='apply'||state.value.$==='object')return 'Array conversion resource suspension retains its completed primitive';
   return readJavaScriptOrdinaryValueError(heap,state.value);
  }
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayDefinitionState} definition
 * @param {JavaScriptSemanticValue} exoticKey @param {JavaScriptScalarConversionTransition} transition
 * @returns {JavaScriptArrayAssignmentTransition} */
export function continueJavaScriptArrayAssignmentNumber(heap,definition,exoticKey,transition){
 switch(transition.$){
  case 'scalarContinuation':return {$:'arrayAssignmentContinue',heap,state:{$:'arrayAssignmentNumber',definition,exoticKey,conversion:transition.state}};
  case 'scalarCall':return {$:'arrayAssignmentCall',heap,state:{$:'arrayAssignmentNumber',definition,exoticKey,conversion:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'scalarCompleted':return continueJavaScriptArrayAssignmentDefinition(exoticKey,resumeJavaScriptArrayPropertyNumber(heap,definition,transition.value));
  case 'scalarThrow':return {$:'arrayAssignmentThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arrayAssignmentError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arrayAssignmentInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayAssignmentResourceLimit',heap,state:{$:'arrayAssignmentPrimitive',definition,exoticKey,value:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptSemanticValue} exoticKey @param {JavaScriptArrayCompletion} transition @returns {JavaScriptArrayAssignmentTransition} */
export function continueJavaScriptArrayAssignmentDefinition(exoticKey,transition){
 switch(transition.$){
  case 'arrayDefinitionContinue':return {$:'arrayAssignmentContinue',heap:transition.heap,state:{$:'arrayAssignmentDefinition',definition:transition.state,exoticKey}};
  case 'arrayNumberRequest':return continueJavaScriptArrayAssignmentNumber(transition.heap,transition.state,exoticKey,beginJavaScriptScalarProtocol(transition.heap,transition.input,exoticKey,'coerce primitive number'));
  case 'arrayPropertyComplete':return {$:'arrayAssignmentCompleted',heap:transition.heap,success:transition.success,writes:transition.writes};
  case 'arrayError':return {$:'arrayAssignmentError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arrayInvalid':return {$:'arrayAssignmentInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayResourceLimit':return transition.state===undefined?{$:'arrayAssignmentInvalid',heap:transition.heap,reason:'an allocation resource outcome cannot substitute an Array definition continuation'}:{$:'arrayAssignmentResourceLimit',heap:transition.heap,state:{$:'arrayAssignmentDefinition',definition:transition.state,exoticKey},reason:transition.reason};
  case 'arrayNormal':return {$:'arrayAssignmentInvalid',heap:transition.heap,reason:'Array assignment cannot substitute an allocation completion'};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key
 * @param {JavaScriptPropertyDescriptor} descriptor @param {JavaScriptSemanticValue} exoticKey
 * @returns {JavaScriptArrayAssignmentTransition} */
export function beginJavaScriptArrayAssignment(heap,identity,key,descriptor,exoticKey){
 const transition=beginJavaScriptArrayPropertyDefinition(heap,identity,key,descriptor);
 // Beginning retains the first conversion phase; later steps drive callbacks.
 return continueJavaScriptArrayAssignmentDefinition(exoticKey,transition);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayAssignmentState} state @returns {JavaScriptArrayAssignmentTransition} */
export function stepJavaScriptArrayAssignment(heap,state){
 const error=readJavaScriptArrayAssignmentStateError(heap,state);if(error!=='')return {$:'arrayAssignmentInvalid',heap,reason:error};
 switch(state.$){
  case 'arrayAssignmentDefinition':{
   const transition=stepJavaScriptArrayPropertyDefinition(heap,state.definition);
   if(transition.$==='arrayResourceLimit')return {$:'arrayAssignmentResourceLimit',heap:transition.heap,state,reason:transition.reason};
   return continueJavaScriptArrayAssignmentDefinition(state.exoticKey,transition);
  }
  case 'arrayAssignmentNumber':return continueJavaScriptArrayAssignmentNumber(heap,state.definition,state.exoticKey,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
  case 'arrayAssignmentPrimitive':return continueJavaScriptArrayAssignmentNumber(heap,state.definition,state.exoticKey,finishJavaScriptScalarPrimitive('coerce primitive number',state.value));
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayAssignmentState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayAssignmentTransition} */
export function resumeJavaScriptArrayAssignment(heap,state,completion){
 const error=readJavaScriptArrayAssignmentStateError(heap,state);if(error!=='')return {$:'arrayAssignmentInvalid',heap,reason:error};
 if(state.$!=='arrayAssignmentNumber')return {$:'arrayAssignmentInvalid',heap,reason:'only an actual Array numeric callback can resume with a child completion'};
 return continueJavaScriptArrayAssignmentNumber(heap,state.definition,state.exoticKey,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
}
