// Actual standard method identities select these shared primitive semantics.
// Object conversion/RegExp protocols remain explicit continuation obligations.
import {testJavaScriptStandardPrimitiveMethodOperation} from './javascript-standard-operation-rules.js';
import {executeJavaScriptCallTextConversion} from './javascript-call-text-conversion.js';
import {executeJavaScriptCallNumberConversion} from './javascript-call-number-conversion.js';
import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
import {readUtf16TextInteger} from './utf16-text-rules.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptPrimitiveMethodOperation(operation){return testJavaScriptStandardPrimitiveMethodOperation(operation);}
/** @param {JavaScriptSemanticValue} receiver @param {string} operation
 * @param {JavaScriptSemanticValue[]} args @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptPrimitiveMethod(receiver,operation,args){
 if(!testJavaScriptPrimitiveMethodOperation(operation))return {$:'primitiveInvalid',reason:'primitive methods require their actual registered operation owner'};
 if(args.length>4096)return {$:'primitiveResourceLimit',reason:'primitive method arguments exceed their explicit call budget'};
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'primitiveError',errorClass:'TypeError',reason:'String.prototype.startsWith called on null or undefined'};
 if(receiver.$==='object')return {$:'primitiveInvalid',reason:'this String method requires its actual receiver ToPrimitive continuation'};
 const receiverError=readJavaScriptSemanticValueError(receiver);if(receiverError!=='')return {$:'primitiveInvalid',reason:receiverError};
 const text=executeJavaScriptCallTextConversion(receiver);if(text.$!=='primitiveValue')return text;
 return executeJavaScriptPrimitivePrefixSearch(text.value,args);
}

/** @param {JavaScriptSemanticValue} receiverText @param {JavaScriptSemanticValue[]} args
 * @returns {JavaScriptCallPrimitiveResult} */
function executeJavaScriptPrimitivePrefixSearch(receiverText,args){
 const search=args[0]??{$:'undefined'};
 if(search.$==='object')return {$:'primitiveInvalid',reason:'String search requires its actual IsRegExp and ToPrimitive protocols'};
 const searchError=readJavaScriptSemanticValueError(search);if(searchError!=='')return {$:'primitiveInvalid',reason:searchError};
 const converted=executeJavaScriptCallTextConversion(search);if(converted.$!=='primitiveValue')return converted;
 return executeJavaScriptPrimitivePrefixPosition(receiverText,converted.value,args[1]??{$:'undefined'});
}

/** @param {JavaScriptSemanticValue} receiverText @param {JavaScriptSemanticValue} searchText
 * @param {JavaScriptSemanticValue} position @returns {JavaScriptCallPrimitiveResult} */
function executeJavaScriptPrimitivePrefixPosition(receiverText,searchText,position){
 if(position.$==='object')return {$:'primitiveInvalid',reason:'String search position requires its actual ToPrimitive continuation'};
 const positionError=readJavaScriptSemanticValueError(position);if(positionError!=='')return {$:'primitiveInvalid',reason:positionError};
 const number=executeJavaScriptCallNumberConversion('coerce primitive number',position);if(number.$!=='primitiveValue')return number;
 const positionValue=number.value;if(positionValue.$!=='number')return {$:'primitiveInvalid',reason:'String search position retains its actual Number conversion'};
 const length=readJavaScriptSemanticTextLength(receiverText),searchLength=readJavaScriptSemanticTextLength(searchText),start=Math.min(length,Math.max(0,readUtf16TextInteger(positionValue.value)));
 if(searchLength>1048576)return {$:'primitiveResourceLimit',reason:'String prefix comparison exceeds its selected 1048576-unit work profile'};
 let matches=start+searchLength<=length;
 for(let at=0;matches&&at<searchLength;at++)if(readJavaScriptSemanticTextUnit(receiverText,start+at)!==readJavaScriptSemanticTextUnit(searchText,at))matches=false;
 return {$:'primitiveValue',value:{$:'boolean',value:matches}};
}
