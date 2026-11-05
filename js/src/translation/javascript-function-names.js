// SetFunctionName spelling consumes an already converted property key.
// Symbol descriptions are supplied by actual embedding metadata authority.
import {readJavaScriptSemanticTextLength,constructJavaScriptSemanticTextUnits,readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
/** @typedef {{$:'functionNameValue',value:JavaScriptSemanticValue} | {$:'functionNameInvalid',reason:string} | {$:'functionNameResourceLimit',reason:string} | {$:'functionNameSymbolDescription',key:JavaScriptSemanticValue,prefix:string}} JavaScriptFunctionNameResult */
/** @param {JavaScriptSemanticValue} key @param {string} prefix @returns {JavaScriptFunctionNameResult} */
export function constructJavaScriptPropertyFunctionName(key,prefix){
 const error=readJavaScriptSemanticValueError(key);if(error!==''||!['','get','set'].includes(prefix))return {$:'functionNameInvalid',reason:error||'function names retain actual empty/get/set prefixes'};
 if(key.$==='symbol')return {$:'functionNameSymbolDescription',key,prefix};
 if(key.$!=='text'&&key.$!=='sourceText')return {$:'functionNameInvalid',reason:'function names consume a completed text or Symbol property key'};
 const size=readJavaScriptSemanticTextLength(key),leading=prefix===''?[]:prefix==='get'?[103,101,116,32]:[115,101,116,32];
 if(size+leading.length>65536)return {$:'functionNameResourceLimit',reason:'function name exceeds its explicit UTF16 metadata budget'};
 return {$:'functionNameValue',value:{$:'text',units:leading.concat(constructJavaScriptSemanticTextUnits(key,65536))}};
}
/** Undefined description yields the empty name; an empty description yields [].
 * No Symbol-to-string conversion or user getter runs here.
 * @param {JavaScriptSemanticValue} description @param {string} prefix @returns {JavaScriptFunctionNameResult} */
export function constructJavaScriptSymbolFunctionName(description,prefix){
 const error=readJavaScriptSemanticValueError(description);if(error!==''||description.$!=='undefined'&&description.$!=='text'&&description.$!=='sourceText')return {$:'functionNameInvalid',reason:error||'Symbol function names require actual optional text descriptions'};
 const length=description.$==='undefined'?0:readJavaScriptSemanticTextLength(description);if(length>65534)return {$:'functionNameResourceLimit',reason:'Symbol function name exceeds its explicit UTF16 metadata budget'};
 const key={$:'text',units:description.$==='undefined'?[]:[91].concat(constructJavaScriptSemanticTextUnits(description,65536),[93])};return constructJavaScriptPropertyFunctionName(key,prefix);
}
