import {beginJavaScriptPrimitiveConversion,stepJavaScriptPrimitiveConversion,resumeJavaScriptPrimitiveConversion} from './javascript-primitive-conversion.js';
import {executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';

/** @typedef {{$:'numberProtocolState',state:JavaScriptPrimitiveConversionState} |
 * {$:'numberPrimitiveValue',value:JavaScriptSemanticValue}} JavaScriptNumberCallState */

/** Every callback is actual registered code. Resource refusal retains the
 * completed primitive so a caller never needs to replay conversion effects.
 * @typedef {{$:'numberContinuation',state:JavaScriptPrimitiveConversionState} |
 * {$:'numberCall',state:JavaScriptPrimitiveConversionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'numberCompleted',value:JavaScriptSemanticValue} |
 * {$:'numberThrow',value:JavaScriptSemanticValue} |
 * {$:'numberIntrinsicError',errorClass:string,reason:string} |
 * {$:'numberInvalid',reason:string} |
 * {$:'numberResourceLimit',value:JavaScriptSemanticValue,reason:string}} JavaScriptNumberConversionTransition
 */
/** @param {string} operation @returns {string} */
export function readJavaScriptNumberProtocolOperationError(operation){
 return operation==='convert explicit number'||operation==='coerce primitive number'?'':'number conversion requires its actual explicit or implicit source operation';
}
/** @param {string} operation @param {JavaScriptPrimitiveConversionTransition} transition @returns {JavaScriptNumberConversionTransition} */
export function finishJavaScriptNumberProtocol(operation,transition){
 const error=readJavaScriptNumberProtocolOperationError(operation);if(error!=='')return {$:'numberInvalid',reason:error};
 switch(transition.$){
  case 'primitiveContinuation':return {$:'numberContinuation',state:transition.state};
  case 'primitiveCall':return {$:'numberCall',state:transition.state,callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'primitiveThrow':return {$:'numberThrow',value:transition.value};
  case 'primitiveIntrinsicError':return {$:'numberIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
  case 'primitiveInvalid':return {$:'numberInvalid',reason:transition.reason};
  case 'primitiveCompleted':return finishJavaScriptNumberPrimitive(operation,transition.value);
 }
}
/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptNumberConversionTransition} */
export function finishJavaScriptNumberPrimitive(operation,value){
 const error=readJavaScriptNumberProtocolOperationError(operation);if(error!=='')return {$:'numberInvalid',reason:error};
 const result=executeJavaScriptCallPrimitive(operation,[value]);
 switch(result.$){
  case 'primitiveValue':return {$:'numberCompleted',value:result.value};
  case 'primitiveError':return {$:'numberIntrinsicError',errorClass:result.errorClass,reason:result.reason};
  case 'primitiveInvalid':return {$:'numberInvalid',reason:result.reason};
  case 'primitiveResourceLimit':return {$:'numberResourceLimit',value,reason:result.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} exoticKey @param {string} operation @returns {JavaScriptNumberConversionTransition} */
export function beginJavaScriptNumberProtocol(heap,input,exoticKey,operation){
 const error=readJavaScriptNumberProtocolOperationError(operation);if(error!=='')return {$:'numberInvalid',reason:error};
 return finishJavaScriptNumberProtocol(operation,beginJavaScriptPrimitiveConversion(heap,input,exoticKey,'number'));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {string} operation @returns {JavaScriptNumberConversionTransition} */
export function stepJavaScriptNumberProtocol(heap,state,operation){
 const error=readJavaScriptNumberProtocolOperationError(operation);if(error!=='')return {$:'numberInvalid',reason:error};
 if(state.hint!=='number')return {$:'numberInvalid',reason:'Number conversion retains its actual numeric primitive hint'};
 return finishJavaScriptNumberProtocol(operation,stepJavaScriptPrimitiveConversion(heap,state));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @param {string} operation @returns {JavaScriptNumberConversionTransition} */
export function resumeJavaScriptNumberProtocol(heap,state,completion,operation){
 const error=readJavaScriptNumberProtocolOperationError(operation);if(error!=='')return {$:'numberInvalid',reason:error};
 if(state.hint!=='number')return {$:'numberInvalid',reason:'Number conversion retains its actual numeric primitive hint'};
 return finishJavaScriptNumberProtocol(operation,resumeJavaScriptPrimitiveConversion(heap,state,completion));
}
