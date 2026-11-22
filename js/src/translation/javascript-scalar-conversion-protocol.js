import {executeJavaScriptCallScalarConversion,readJavaScriptCallScalarConversionError} from './javascript-call-scalar-conversion.js';
import {beginJavaScriptPrimitiveConversion,stepJavaScriptPrimitiveConversion,resumeJavaScriptPrimitiveConversion} from './javascript-primitive-conversion.js';
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';

/** Every callback is actual registered code. Resource refusal retains the
 * completed primitive so a caller never needs to replay conversion effects.
 * @typedef {{$:'scalarContinuation',state:JavaScriptPrimitiveConversionState} |
 * {$:'scalarCall',state:JavaScriptPrimitiveConversionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'scalarCompleted',value:JavaScriptSemanticValue} |
 * {$:'scalarThrow',value:JavaScriptSemanticValue} |
 * {$:'scalarIntrinsicError',errorClass:string,reason:string} |
 * {$:'scalarInvalid',reason:string} |
 * {$:'scalarResourceLimit',value:JavaScriptSemanticValue,reason:string}} JavaScriptScalarConversionTransition
 */
/** @param {string} operation @returns {string} */
export function readJavaScriptScalarProtocolOperationError(operation){return readJavaScriptCallScalarConversionError(operation);}
/** @param {string} operation @returns {string} */
export function readJavaScriptScalarProtocolHint(operation){return operation==='convert primitive text'?'string':operation==='convert primitive value'?'default':'number';}
/** @param {string} operation @param {JavaScriptPrimitiveConversionTransition} transition @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptScalarProtocol(operation,transition){
 const error=readJavaScriptScalarProtocolOperationError(operation);if(error!=='')return {$:'scalarInvalid',reason:error};
 switch(transition.$){
  case 'primitiveContinuation':return {$:'scalarContinuation',state:transition.state};
  case 'primitiveCall':return {$:'scalarCall',state:transition.state,callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'primitiveThrow':return {$:'scalarThrow',value:transition.value};
  case 'primitiveIntrinsicError':return {$:'scalarIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
  case 'primitiveInvalid':return {$:'scalarInvalid',reason:transition.reason};
  case 'primitiveCompleted':return finishJavaScriptScalarPrimitive(operation,transition.value);
 }
}
/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptScalarPrimitive(operation,value){
 const error=readJavaScriptScalarProtocolOperationError(operation);if(error!=='')return {$:'scalarInvalid',reason:error};
 const invalid=readJavaScriptSemanticValueError(value);if(invalid!=='')return {$:'scalarInvalid',reason:invalid};
 const result=executeJavaScriptCallScalarConversion(operation,value);
 switch(result.$){
  case 'primitiveValue':return {$:'scalarCompleted',value:result.value};
  case 'primitiveError':return {$:'scalarIntrinsicError',errorClass:result.errorClass,reason:result.reason};
  case 'primitiveInvalid':return {$:'scalarInvalid',reason:result.reason};
  case 'primitiveResourceLimit':return {$:'scalarResourceLimit',value,reason:result.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} exoticKey @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function beginJavaScriptScalarProtocol(heap,input,exoticKey,operation){
 const error=readJavaScriptScalarProtocolOperationError(operation);if(error!=='')return {$:'scalarInvalid',reason:error};
 return finishJavaScriptScalarProtocol(operation,beginJavaScriptPrimitiveConversion(heap,input,exoticKey,readJavaScriptScalarProtocolHint(operation)));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function stepJavaScriptScalarProtocol(heap,state,operation){
 const error=readJavaScriptScalarProtocolOperationError(operation);if(error!=='')return {$:'scalarInvalid',reason:error};
 if(state.hint!==(readJavaScriptScalarProtocolHint(operation)))return {$:'scalarInvalid',reason:'scalar conversion retains the hint of its actual source operation'};
 return finishJavaScriptScalarProtocol(operation,stepJavaScriptPrimitiveConversion(heap,state));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function resumeJavaScriptScalarProtocol(heap,state,completion,operation){
 const error=readJavaScriptScalarProtocolOperationError(operation);if(error!=='')return {$:'scalarInvalid',reason:error};
 if(state.hint!==(readJavaScriptScalarProtocolHint(operation)))return {$:'scalarInvalid',reason:'scalar conversion retains the hint of its actual source operation'};
 return finishJavaScriptScalarProtocol(operation,resumeJavaScriptPrimitiveConversion(heap,state,completion));
}
