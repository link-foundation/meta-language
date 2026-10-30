// Primitive Number conversion shares the exact StringNumericLiteral and
// integer rounding definitions. Object conversion belongs to its resumable
// ToPrimitive protocol, never an invented host conversion or property read.
import {convertJavaScriptStringNumberUnits,convertJavaScriptBigIntegerNumber} from './string-number-rules.js';
import {readJavaScriptCallIntegerWidth} from './javascript-call-integers.js';

/** @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumberConversion(operation,value){
 if(operation!=='convert explicit number'&&operation!=='coerce primitive number')return {$:'primitiveInvalid',reason:'number conversion needs its actual registered operation'};
 switch(value.$){
  case 'undefined':return {$:'primitiveValue',value:{$:'number',value:NaN}};
  case 'null':return {$:'primitiveValue',value:{$:'number',value:0}};
  case 'boolean':return {$:'primitiveValue',value:{$:'number',value:value.value?1:0}};
  case 'number':return {$:'primitiveValue',value};
  case 'integer':{
   if(operation==='coerce primitive number')return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a BigInt value to a number'};
   if(readJavaScriptCallIntegerWidth(value.value)<0)return {$:'primitiveResourceLimit',reason:'explicit Number conversion exceeds the selected integer work profile'};
   return {$:'primitiveValue',value:{$:'number',value:convertJavaScriptBigIntegerNumber(value.value)}};
  }
  case 'text':{
   if(value.units.length>4096)return {$:'primitiveResourceLimit',reason:'Number text conversion exceeds the selected 4096-unit work profile'};
   return {$:'primitiveValue',value:{$:'number',value:convertJavaScriptStringNumberUnits(value.units)}};
  }
  case 'symbol':return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Symbol value to a number'};
  case 'object':return {$:'primitiveInvalid',reason:'object Number conversion requires the actual suspended ToPrimitive protocol'};
 }
}
