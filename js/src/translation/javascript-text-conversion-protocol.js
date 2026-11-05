import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
/** @typedef {{$:'textProtocolState',state:JavaScriptPrimitiveConversionState} |
 * {$:'textPrimitiveValue',value:JavaScriptSemanticValue}} JavaScriptTextCallState */
/** @param {string} operation @returns {string} */
export function readJavaScriptTextProtocolOperationError(operation){return operation==='convert primitive text'?'':'text conversion requires its actual registered source operation';}
/** @param {string} operation @param {JavaScriptPrimitiveConversionTransition} transition @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptTextProtocol(operation,transition){const error=readJavaScriptTextProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:finishJavaScriptScalarProtocol(operation,transition);}
/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptScalarConversionTransition} */
export function finishJavaScriptTextPrimitive(operation,value){const error=readJavaScriptTextProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:finishJavaScriptScalarPrimitive(operation,value);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} exoticKey @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function beginJavaScriptTextProtocol(heap,input,exoticKey,operation){const error=readJavaScriptTextProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:beginJavaScriptScalarProtocol(heap,input,exoticKey,operation);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function stepJavaScriptTextProtocol(heap,state,operation){const error=readJavaScriptTextProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:stepJavaScriptScalarProtocol(heap,state,operation);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveConversionState} state @param {JavaScriptPrimitiveConversionCompletion} completion @param {string} operation @returns {JavaScriptScalarConversionTransition} */
export function resumeJavaScriptTextProtocol(heap,state,completion,operation){const error=readJavaScriptTextProtocolOperationError(operation);return error!==''?{$:'scalarInvalid',reason:error}:resumeJavaScriptScalarProtocol(heap,state,completion,operation);}
