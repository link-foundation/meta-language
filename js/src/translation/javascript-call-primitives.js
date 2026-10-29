// Reuse the existing complete UTF16 definitions; adapters never substitute
// host scalar strings for code units or reimplement native operation bodies.
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
import {readJavaScriptCallIntegerArity,executeJavaScriptCallIntegerPrimitive,readJavaScriptCallIntegerWidth} from './javascript-call-integers.js';
import {executeJavaScriptCallNumberConversion} from './javascript-call-number-conversion.js';
import {executeJavaScriptCallIntegerConversion} from './javascript-call-integer-conversion.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {sliceUtf16Text,codePointUtf16Text,characterUtf16Text,relativeCharacterUtf16Text,indexUtf16Text,lastIndexUtf16Text,compareUtf16Text,startsUtf16Text,endsUtf16Text,includesUtf16Text,trimUtf16Text,characterUnitsUtf16Text} from './utf16-text-rules.js';

/** @typedef {{$:'primitiveValue',value:JavaScriptSemanticValue} |
 * {$:'primitiveInvalid',reason:string} | {$:'primitiveError',errorClass:string,reason:string} |
 * {$:'primitiveResourceLimit',reason:string}} JavaScriptCallPrimitiveResult */

/** @param {string} operation @returns {boolean} */
export function testJavaScriptCallUnaryNumberOperation(operation){
 return ['negate number','truncate number','read absolute number','floor number','ceil number','round number','read number sign','read square root','test integer number','test safe integer number'].includes(operation);
}
/** Operations consume already checked primitive values. Unmodeled coercions,
 * wrappers and replacement intrinsics need their own source lowering.
 * @param {string} operation @param {number} count @returns {string} */
export function readJavaScriptCallPrimitiveArityError(operation,count){
 if(!Number.isSafeInteger(count)||count<0||count>4096)return 'primitive operands exceed their explicit call register budget';
 if(readJavaScriptSourceExceptionClass(operation)!=='')return count===2?'':'a source Error requires its actual text message and own-message presence';
 const integerArity=readJavaScriptCallIntegerArity(operation);
 if(integerArity!==0)return count===integerArity?'':'primitive instructions require every actual operand coordinate';
 if(operation==='read minimum number'||operation==='read maximum number'||operation==='construct text units')return '';
 if(testJavaScriptCallUnaryNumberOperation(operation))return count===1?'':'primitive instructions require every actual operand coordinate';
 const unary=['read text length','trim text','trim text start','trim text end','convert explicit number','coerce primitive number','convert primitive text','convert explicit integer','coerce primitive integer'];
 const binary=['read text code point','read text character','read relative text character','concatenate text','compare text','test text prefix','test text suffix','test text inclusion'];
 const ternary=['slice text','find text','find last text'];
 const arity=unary.includes(operation)?1:binary.includes(operation)?2:ternary.includes(operation)?3:-1;
 return arity<0?'primitive instructions require their actual registered operation':count!==arity?'primitive instructions require every actual operand coordinate':'';
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumericPrimitive(operation,values){
 // A Number-only array can canonicalize NaNs in supported engines. Unary
 // operations consume the original scalar directly, preserving its input bits.
 if(testJavaScriptCallUnaryNumberOperation(operation)){
  if(values.length!==1)return {$:'primitiveInvalid',reason:'primitive instructions require every actual operand coordinate'};
  const first=values[0]??{$:'undefined'};
  if(first.$!=='number')return {$:'primitiveInvalid',reason:'numeric primitives require actual Number operands after source conversion'};
  return executeJavaScriptCallUnaryNumber(operation,first.value);
 }
 let numbers=[];
 for(const value of values){if(value.$!=='number')return {$:'primitiveInvalid',reason:'numeric primitives require actual Number operands after source conversion'};numbers.push(value.value);}
 if(operation==='read minimum number')return {$:'primitiveValue',value:{$:'number',value:Math.min(...numbers)}};
 if(operation==='read maximum number')return {$:'primitiveValue',value:{$:'number',value:Math.max(...numbers)}};
 if(operation==='construct text units')return {$:'primitiveValue',value:{$:'text',units:characterUnitsUtf16Text(numbers)}};
 return {$:'primitiveInvalid',reason:'the numeric primitive executor needs its actual registered operation'};
}
/** @param {string} operation @param {number} number @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallUnaryNumber(operation,number){
 if(operation==='negate number')return {$:'primitiveValue',value:{$:'number',value:-number}};
 if(operation==='truncate number')return {$:'primitiveValue',value:{$:'number',value:Math.trunc(number)}};
 if(operation==='test integer number')return {$:'primitiveValue',value:{$:'boolean',value:Number.isInteger(number)}};
 if(operation==='test safe integer number')return {$:'primitiveValue',value:{$:'boolean',value:Number.isSafeInteger(number)}};
 if(!testJavaScriptCallUnaryNumberOperation(operation))return {$:'primitiveInvalid',reason:'unary numeric primitives require their actual registered operation'};
 const result=operation==='read absolute number'?Math.abs(number):operation==='floor number'?Math.floor(number):operation==='ceil number'?Math.ceil(number):operation==='round number'?Math.round(number):operation==='read number sign'?Math.sign(number):Math.sqrt(number);
 return {$:'primitiveValue',value:{$:'number',value:result}};
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextPrimitive(operation,values){
 const receiver=values[0]??{$:'undefined'};if(receiver.$!=='text')return {$:'primitiveInvalid',reason:'text primitives require their actual complete UTF16 receiver'};
 const text=receiver.units;
 if(operation==='read text length')return {$:'primitiveValue',value:{$:'number',value:text.length}};
 if(operation==='trim text'||operation==='trim text start'||operation==='trim text end')return {$:'primitiveValue',value:{$:'text',units:trimUtf16Text(text,operation!=='trim text end',operation!=='trim text start')}};
 if(operation==='read text code point'||operation==='read text character'||operation==='read relative text character'||operation==='slice text')return executeJavaScriptCallTextPosition(operation,text,values);
 return executeJavaScriptCallTextPair(operation,text,values);
}
/** @param {string} operation @param {number[]} text @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextPosition(operation,text,values){
 const second=values[1]??{$:'undefined'},third=values[2]??{$:'undefined'};
 if(operation==='read text code point'||operation==='read text character'||operation==='read relative text character'){
  if(second.$!=='number')return {$:'primitiveInvalid',reason:'text positions require their actual converted Number operand'};
  if(operation==='read text character')return {$:'primitiveValue',value:{$:'text',units:characterUtf16Text(text,second.value)}};
  if(operation==='read relative text character'){const result=relativeCharacterUtf16Text(text,second.value),value=result[0];return {$:'primitiveValue',value:value===undefined?{$:'undefined'}:{$:'text',units:value}};}
  const result=codePointUtf16Text(text,second.value),value=result[0];return {$:'primitiveValue',value:value===undefined?{$:'undefined'}:{$:'number',value}};
 }
 if(operation==='slice text'){
  if(second.$!=='number')return {$:'primitiveInvalid',reason:'text slice bounds require their actual converted Number operands'};
  if(third.$!=='number')return {$:'primitiveInvalid',reason:'text slice bounds require their actual converted Number operands'};
  return {$:'primitiveValue',value:{$:'text',units:sliceUtf16Text(text,second.value,third.value)}};
 }
 return {$:'primitiveInvalid',reason:'a text position executor needs its actual registered operation'};
}
/** @param {string} operation @param {number[]} text @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextPair(operation,text,values){
 const second=values[1]??{$:'undefined'},third=values[2]??{$:'undefined'};
 if(second.$!=='text')return {$:'primitiveInvalid',reason:'text search and concatenation require the actual converted UTF16 operand'};
 const needle=second.units;
 if(operation==='concatenate text'){
  if(text.length+needle.length>1048576)return {$:'primitiveResourceLimit',reason:'primitive text output exceeds its explicit UTF16 unit budget'};
  return {$:'primitiveValue',value:{$:'text',units:text.concat(needle)}};
 }
 if(operation==='compare text')return {$:'primitiveValue',value:{$:'number',value:compareUtf16Text(text,needle)}};
 if(operation==='test text prefix')return {$:'primitiveValue',value:{$:'boolean',value:startsUtf16Text(text,needle)}};
 if(operation==='test text suffix')return {$:'primitiveValue',value:{$:'boolean',value:endsUtf16Text(text,needle)}};
 // This explicit preflight bounds the existing search algorithm's maximum
 // unit comparisons, separately from input/output and language exceptions.
 if((Math.max(0,text.length-needle.length)+1)*needle.length>67108864)return {$:'primitiveResourceLimit',reason:'primitive text search exceeds its explicit comparison work quota'};
 if(operation==='test text inclusion')return {$:'primitiveValue',value:{$:'boolean',value:includesUtf16Text(text,needle)}};
 if(operation==='find text'||operation==='find last text'){
  if(third.$!=='number')return {$:'primitiveInvalid',reason:'text search positions require their actual converted Number operand'};
  return {$:'primitiveValue',value:{$:'number',value:operation==='find text'?indexUtf16Text(text,needle,third.value):lastIndexUtf16Text(text,needle,third.value)}};
 }
 return {$:'primitiveInvalid',reason:'the text primitive executor needs its actual registered operation'};
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallPrimitive(operation,values){
 const arity=readJavaScriptCallPrimitiveArityError(operation,values.length);if(arity!=='')return {$:'primitiveInvalid',reason:arity};
 for(const value of values){const error=readJavaScriptSemanticValueError(value);if(error!=='')return {$:'primitiveInvalid',reason:error};}
 if(readJavaScriptSourceExceptionClass(operation)!=='')return {$:'primitiveInvalid',reason:'source Error allocation requires its actual registered realm'};
 if(operation==='convert explicit number'||operation==='coerce primitive number')return executeJavaScriptCallNumberConversion(operation,values[0]??{$:'undefined'});
 if(operation==='convert explicit integer'||operation==='coerce primitive integer')return executeJavaScriptCallIntegerConversion(operation,values[0]??{$:'undefined'});
 if(operation==='convert primitive text')return executeJavaScriptCallTextConversion(values[0]??{$:'undefined'});
 if(readJavaScriptCallIntegerArity(operation)!==0)return executeJavaScriptCallIntegerPrimitive(operation,values);
 return testJavaScriptCallUnaryNumberOperation(operation)||['read minimum number','read maximum number','construct text units'].includes(operation)?executeJavaScriptCallNumericPrimitive(operation,values):executeJavaScriptCallTextPrimitive(operation,values);
}

/** Primitive ToString uses the existing scalar formatting definitions. Symbol
 * descriptions/explicit String(Symbol) and Object ToPrimitive are separate
 * operations, never inferred from an identity or a source spelling.
 * @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextConversion(value){
 let text='';
 switch(value.$){
  case 'text':return {$:'primitiveValue',value};
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
