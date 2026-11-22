import {readJavaScriptCallIntegerWidth} from './javascript-call-integer-width.js';

/** Primitive ToString uses the existing scalar formatting definitions. Symbol
 * descriptions/explicit String(Symbol) and Object ToPrimitive are separate
 * operations, never inferred from an identity or a source spelling.
 * @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextConversion(value){
 let text='';
 switch(value.$){
  case 'text':case 'sourceText':return {$:'primitiveValue',value};
  case 'undefined':text='undefined';break;
  case 'null':text='null';break;
  case 'boolean':text=value.value?'true':'false';break;
  case 'number':text=String(value.value);break;
  case 'integer':{
   if(readJavaScriptCallIntegerWidth(value.value)<0)return {$:'primitiveResourceLimit',reason:'integer text conversion exceeds the selected 4096-bit work profile'};
   text=String(value.value);break;
  }
  case 'symbol':return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Symbol value to a string'};
  case 'object':return {$:'primitiveInvalid',reason:'object text conversion requires the actual suspended ToPrimitive protocol'};
 }
 /** @type {number[]} */ const units=[];
 for(let index=0;index<text.length;index+=1)units.push(text.charCodeAt(index));
 return {$:'primitiveValue',value:{$:'text',units}};
}

