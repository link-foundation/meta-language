// Actual standard method identities select these shared primitive semantics.
// Object conversion/RegExp protocols remain explicit continuation obligations.
import {executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
import {readUtf16TextInteger} from './utf16-text-rules.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptPrimitiveMethodOperation(operation){return operation==='String.startsWith';}
/** @param {JavaScriptSemanticValue} receiver @param {string} operation
 * @param {JavaScriptSemanticValue[]} args @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptPrimitiveMethod(receiver,operation,args){
 if(!testJavaScriptPrimitiveMethodOperation(operation))return {$:'primitiveInvalid',reason:'primitive methods require their actual registered operation owner'};
 if(args.length>4096)return {$:'primitiveResourceLimit',reason:'primitive method arguments exceed their explicit call budget'};
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'primitiveError',errorClass:'TypeError',reason:'String.prototype.startsWith called on null or undefined'};
 if(receiver.$==='object')return {$:'primitiveInvalid',reason:'this String method requires its actual receiver ToPrimitive continuation'};
 const text=executeJavaScriptCallPrimitive('convert primitive text',[receiver]);if(text.$!=='primitiveValue')return text;
 const search=args[0]??{$:'undefined'};if(search.$==='object')return {$:'primitiveInvalid',reason:'String search requires its actual IsRegExp and ToPrimitive protocols'};
 const converted=executeJavaScriptCallPrimitive('convert primitive text',[search]);if(converted.$!=='primitiveValue')return converted;
 const position=args[1]??{$:'undefined'};if(position.$==='object')return {$:'primitiveInvalid',reason:'String search position requires its actual ToPrimitive continuation'};
 const number=executeJavaScriptCallPrimitive('coerce primitive number',[position]);if(number.$!=='primitiveValue')return number;if(number.value.$!=='number')return {$:'primitiveInvalid',reason:'String search position retains its actual Number conversion'};
 const length=readJavaScriptSemanticTextLength(text.value),searchLength=readJavaScriptSemanticTextLength(converted.value),start=Math.min(length,Math.max(0,readUtf16TextInteger(number.value.value)));
 if(searchLength>1048576)return {$:'primitiveResourceLimit',reason:'String prefix comparison exceeds its selected 1048576-unit work profile'};
 let matches=start+searchLength<=length;
 for(let at=0;matches&&at<searchLength;at++)if(readJavaScriptSemanticTextUnit(text.value,start+at)!==readJavaScriptSemanticTextUnit(converted.value,at))matches=false;
 return {$:'primitiveValue',value:{$:'boolean',value:matches}};
}
