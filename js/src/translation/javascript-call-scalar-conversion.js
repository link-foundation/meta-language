import {executeJavaScriptCallIntegerConversion,readJavaScriptIntegerProtocolOperationError} from './javascript-call-integer-conversion.js';
import {executeJavaScriptCallNumberConversion} from './javascript-call-number-conversion.js';
import {executeJavaScriptCallTextConversion} from './javascript-call-text-conversion.js';

/** @param {string} operation @returns {string} */
export function readJavaScriptCallScalarConversionError(operation){
 return operation==='convert explicit number'||operation==='coerce primitive number'||operation==='coerce primitive numeric'||operation==='convert primitive value'||operation==='convert number primitive value'||operation==='convert primitive text'||readJavaScriptIntegerProtocolOperationError(operation)===''?'':'scalar conversion requires its actual registered Number, integer or text source operation';
}

/** The caller retains the complete validated operand and its ToPrimitive state.
 * @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallScalarConversion(operation,value){
 if(operation==='convert primitive value'||operation==='convert number primitive value')return value.$==='object'?{$:'primitiveInvalid',reason:'object ToPrimitive requires its actual selected-hint continuation'}:{$:'primitiveValue',value};
 if(operation==='convert explicit number'||operation==='coerce primitive number'||operation==='coerce primitive numeric')return executeJavaScriptCallNumberConversion(operation,value);
 if(operation==='convert explicit integer'||operation==='coerce primitive integer')return executeJavaScriptCallIntegerConversion(operation,value);
 if(operation==='convert primitive text')return executeJavaScriptCallTextConversion(value);
 return {$:'primitiveInvalid',reason:readJavaScriptCallScalarConversionError(operation)};
}
