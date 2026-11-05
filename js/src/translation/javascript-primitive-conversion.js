import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptPrimitiveConversionSpelling} from './javascript-primitive-conversion-spellings.js';

/** The caller supplies the actual globally registered @@toPrimitive Symbol.
 * An arbitrary equal spelling never establishes well-known Symbol identity.
 * @typedef {{$:'primitiveConversion',receiver:JavaScriptSemanticValue,
 * exoticKey:JavaScriptSemanticValue,hint:string,phase:string}} JavaScriptPrimitiveConversionState
 */
/** @typedef {{$:'normalCompletion',value:JavaScriptSemanticValue} |
 * {$:'throwCompletion',value:JavaScriptSemanticValue}} JavaScriptPrimitiveConversionCompletion
 */
/** @typedef {{$:'primitiveContinuation',state:JavaScriptPrimitiveConversionState} |
 * {$:'primitiveCall',state:JavaScriptPrimitiveConversionState,callee:JavaScriptSemanticValue,
 * receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'primitiveCompleted',value:JavaScriptSemanticValue} |
 * {$:'primitiveThrow',value:JavaScriptSemanticValue} |
 * {$:'primitiveIntrinsicError',errorClass:string,reason:string} |
 * {$:'primitiveInvalid',reason:string}} JavaScriptPrimitiveConversionTransition
 */

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state @returns {string} */
export function readJavaScriptPrimitiveConversionStateError(heap,state){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
  if(state.receiver.$!=='object')return 'a primitive conversion continuation retains its actual Object receiver';
  const receiverError=readJavaScriptOrdinaryValueError(heap,state.receiver);if(receiverError!=='')return receiverError;
  if(state.exoticKey.$!=='symbol')return 'the exotic method key must be the actual registered well-known Symbol';
  const keyError=readJavaScriptOrdinaryValueError(heap,state.exoticKey);if(keyError!=='')return keyError;
  if(state.hint!=='default'&&state.hint!=='number'&&state.hint!=='string')return 'a primitive conversion requires an exact preferred-type hint';
  if(!['exoticGet','firstGet','secondGet','exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(state.phase))return 'a primitive conversion requires its exact method continuation';
  return '';
}

/** @param {JavaScriptPrimitiveConversionState} state @param {string} phase @returns {JavaScriptPrimitiveConversionTransition} */
export function continueJavaScriptPrimitiveConversion(state,phase){
  return {$:'primitiveContinuation',state:{$:'primitiveConversion',receiver:state.receiver,exoticKey:state.exoticKey,hint:state.hint,phase}};
}

/** Handle one actual Get result. Missing exotic methods fall back to ordinary
 * conversion; noncallable exotic methods throw. Ordinary noncallable methods
 * are skipped, without calling or coercing them.
 * @param {JavaScriptPrimitiveConversionState} state @param {JavaScriptSemanticValue} method
 * @returns {JavaScriptPrimitiveConversionTransition} */
export function applyJavaScriptPrimitiveConversionMethod(state,method){
  const exotic=state.phase==='exoticGet'||state.phase==='exoticGetter';
  const first=state.phase==='firstGet'||state.phase==='firstGetter';
  if(exotic&&(method.$==='undefined'||method.$==='null'))return continueJavaScriptPrimitiveConversion(state,'firstGet');
  if(method.$!=='object'){
    if(exotic)return {$:'primitiveIntrinsicError',errorClass:'TypeError',reason:'an exotic primitive conversion method must be callable or absent'};
    if(first)return continueJavaScriptPrimitiveConversion(state,'secondGet');
    return {$:'primitiveIntrinsicError',errorClass:'TypeError',reason:'ordinary conversion exhausted its callable primitive-producing methods'};
  }
  if(!method.callable){
    if(exotic)return {$:'primitiveIntrinsicError',errorClass:'TypeError',reason:'an exotic primitive conversion method must be callable or absent'};
    if(first)return continueJavaScriptPrimitiveConversion(state,'secondGet');
    return {$:'primitiveIntrinsicError',errorClass:'TypeError',reason:'ordinary conversion exhausted its callable primitive-producing methods'};
  }
  const phase=exotic?'exoticCall':first?'firstCall':'secondCall';
  const arguments_=exotic?[readJavaScriptPrimitiveConversionSpelling(state.hint)]:[];
  return {$:'primitiveCall',state:{$:'primitiveConversion',receiver:state.receiver,exoticKey:state.exoticKey,hint:state.hint,phase},callee:method,receiver:state.receiver,arguments:arguments_};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @returns {JavaScriptPrimitiveConversionTransition} */
export function stepJavaScriptPrimitiveConversion(heap,state){
  const error=readJavaScriptPrimitiveConversionStateError(heap,state);if(error!=='')return {$:'primitiveInvalid',reason:error};
  if(state.phase!=='exoticGet'&&state.phase!=='firstGet'&&state.phase!=='secondGet')return {$:'primitiveInvalid',reason:'a pending primitive conversion call needs its actual completion'};
  const receiver=state.receiver;
  if(receiver.$!=='object')return {$:'primitiveInvalid',reason:'a primitive conversion continuation retains its actual Object receiver'};
  const first=state.phase==='firstGet';
  const method=state.hint==='string'?(first?'toString':'valueOf'):(first?'valueOf':'toString');
  const key=state.phase==='exoticGet'?state.exoticKey:readJavaScriptPrimitiveConversionSpelling(method);
  const plan=planJavaScriptOrdinaryGet(heap,receiver.identity,key,receiver);
  if(plan.error!=='')return {$:'primitiveInvalid',reason:plan.error};
  if(plan.action==='call'){
    const phase=state.phase==='exoticGet'?'exoticGetter':first?'firstGetter':'secondGetter';
    return {$:'primitiveCall',state:{$:'primitiveConversion',receiver,exoticKey:state.exoticKey,hint:state.hint,phase},callee:plan.callee,receiver:plan.receiver,arguments:plan.arguments};
  }
  if(plan.action!=='value')return {$:'primitiveInvalid',reason:'ordinary Get needs its actual value or pending getter'};
  return applyJavaScriptPrimitiveConversionMethod(state,plan.value);
}

/** Begin ToPrimitive. Direct primitive inputs keep their exact payloads,
 * including signed zero, isolated surrogates, Symbols and arbitrary integers.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} exoticKey @param {string} hint
 * @returns {JavaScriptPrimitiveConversionTransition} */
export function beginJavaScriptPrimitiveConversion(heap,input,exoticKey,hint){
  const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return {$:'primitiveInvalid',reason:heapError};
  const error=readJavaScriptOrdinaryValueError(heap,input);if(error!=='')return {$:'primitiveInvalid',reason:error};
  if(hint!=='default'&&hint!=='number'&&hint!=='string')return {$:'primitiveInvalid',reason:'a primitive conversion requires an exact preferred-type hint'};
  if(input.$!=='object')return {$:'primitiveCompleted',value:input};
  return stepJavaScriptPrimitiveConversion(heap,{$:'primitiveConversion',receiver:input,exoticKey,hint,phase:'exoticGet'});
}

/** Actual getter/method effects happen in the executor. Resumption receives
 * that exact completion and latest global heap. Returning an Object from an
 * exotic method throws; ordinary methods continue in the required order.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion
 * @returns {JavaScriptPrimitiveConversionTransition} */
export function resumeJavaScriptPrimitiveConversion(heap,state,completion){
  const error=readJavaScriptPrimitiveConversionStateError(heap,state);if(error!=='')return {$:'primitiveInvalid',reason:error};
  const getter=state.phase==='exoticGetter'||state.phase==='firstGetter'||state.phase==='secondGetter';
  const call=state.phase==='exoticCall'||state.phase==='firstCall'||state.phase==='secondCall';
  if(!getter&&!call)return {$:'primitiveInvalid',reason:'primitive resumption needs its actual pending getter or method'};
  const valueError=readJavaScriptOrdinaryValueError(heap,completion.value);if(valueError!=='')return {$:'primitiveInvalid',reason:valueError};
  if(completion.$==='throwCompletion')return {$:'primitiveThrow',value:completion.value};
  if(getter)return applyJavaScriptPrimitiveConversionMethod(state,completion.value);
  if(completion.value.$!=='object')return {$:'primitiveCompleted',value:completion.value};
  if(state.phase==='firstCall')return continueJavaScriptPrimitiveConversion(state,'secondGet');
  return {$:'primitiveIntrinsicError',errorClass:'TypeError',reason:state.phase==='exoticCall'?'an exotic primitive conversion must return a primitive':'ordinary conversion exhausted its callable primitive-producing methods'};
}
