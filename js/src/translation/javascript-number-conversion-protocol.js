import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
/** @typedef {{$:'numberProtocolState',state:JavaScriptPrimitiveConversionState} |
 * {$:'numberPrimitiveValue',value:JavaScriptSemanticValue}} JavaScriptNumberCallState */
/** @param {string} operation @returns {string} */
export function readJavaScriptNumberProtocolOperationError(operation){return operation==='convert explicit number'||operation==='coerce primitive number'?'':'number conversion requires its actual registered source operation';}
/** @param {string} operation @param {JavaScriptPrimitiveConversionTransition} transition @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptNumberProtocol(operation,transition){const error=readJavaScriptNumberProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:finishJavaScriptScalarProtocol(operation,transition);}
/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptNumberPrimitive(operation,value){const error=readJavaScriptNumberProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:finishJavaScriptScalarPrimitive(operation,value);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} exoticKey @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function beginJavaScriptNumberProtocol(heap,input,exoticKey,operation){const error=readJavaScriptNumberProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:beginJavaScriptScalarProtocol(heap,input,exoticKey,operation);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function stepJavaScriptNumberProtocol(heap,state,operation){const error=readJavaScriptNumberProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:stepJavaScriptScalarProtocol(heap,state,operation);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state @param {JavaScriptPrimitiveConversionCompletion} completion @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function resumeJavaScriptNumberProtocol(heap,state,completion,operation){const error=readJavaScriptNumberProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:resumeJavaScriptScalarProtocol(heap,state,completion,operation);}
