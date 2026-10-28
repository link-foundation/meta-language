// Actual primitive BigInt conversions share the existing integer-string and
// exact integral binary64 definitions. Objects need a represented continuation.
import {convertJavaScriptStringBigIntegerUnits,convertJavaScriptNumberBigInteger,convertJavaScriptBooleanBigInteger,readJavaScriptBigIntegerConversionErrorClass,readJavaScriptBigIntegerConversionMessagePieces} from './big-integer-conversion-rules.js';
import {readJavaScriptCallIntegerWidth} from './javascript-call-integers.js';

/** @param {string} form @param {string} value @returns {JavaScriptCallPrimitiveResult} */
export function refuseJavaScriptCallIntegerConversion(form,value){
 const pieces=readJavaScriptBigIntegerConversionMessagePieces(form);
 return {$:'primitiveError',errorClass:readJavaScriptBigIntegerConversionErrorClass(form),reason:(pieces[0]??'')+value+(pieces[1]??'')};
}
/** Preserve code units while assembling an implementation diagnostic.
 * @param {number[]} units @returns {string} */
export function readJavaScriptCallIntegerText(units){
 let text='';
 for(let index=0;index<units.length;index+=1)text+=String.fromCharCode(units[index]??0);
 return text;
}
/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallIntegerConversion(operation,value){
 if(operation!=='convert explicit integer'&&operation!=='coerce primitive integer')return {$:'primitiveInvalid',reason:'integer conversion requires its actual registered source operation'};
 switch(value.$){
  case 'integer':return {$:'primitiveValue',value};
  case 'boolean':return {$:'primitiveValue',value:{$:'integer',value:convertJavaScriptBooleanBigInteger(value.value)}};
  case 'undefined':return refuseJavaScriptCallIntegerConversion('undefined','undefined');
  case 'null':return refuseJavaScriptCallIntegerConversion('null','null');
  case 'symbol':return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Symbol value to a BigInt'};
  case 'object':return {$:'primitiveInvalid',reason:'object BigInt conversion requires its actual suspended ToPrimitive protocol'};
  case 'number':{
   if(operation==='coerce primitive integer')return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Number value to a BigInt'};
   const converted=convertJavaScriptNumberBigInteger(value.value);
   if(converted.length!==1)return refuseJavaScriptCallIntegerConversion('number',String(value.value));
   return {$:'primitiveValue',value:{$:'integer',value:converted[0]??0n}};
  }
  case 'text':{
   if(value.units.length>4096)return {$:'primitiveResourceLimit',reason:'BigInt text conversion exceeds the selected 4096-unit work profile'};
   const converted=convertJavaScriptStringBigIntegerUnits(value.units);
   if(converted.length!==1)return refuseJavaScriptCallIntegerConversion('string',readJavaScriptCallIntegerText(value.units));
   const integer=converted[0]??0n;
   if(readJavaScriptCallIntegerWidth(integer)<0)return {$:'primitiveResourceLimit',reason:'BigInt conversion output exceeds the selected 4096-bit work profile'};
   return {$:'primitiveValue',value:{$:'integer',value:integer}};
  }
 }
}
