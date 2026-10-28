// Reuse the existing complete UTF16 definitions; adapters never substitute
// host scalar strings for code units or reimplement native operation bodies.
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
import {readJavaScriptCallIntegerArity,executeJavaScriptCallIntegerPrimitive} from './javascript-call-integers.js';
import {sliceUtf16Text,codePointUtf16Text,characterUtf16Text,relativeCharacterUtf16Text,indexUtf16Text,lastIndexUtf16Text,compareUtf16Text,startsUtf16Text,endsUtf16Text,includesUtf16Text,trimUtf16Text,characterUnitsUtf16Text} from './utf16-text-rules.js';

/** @typedef {{$:'primitiveValue',value:JavaScriptSemanticValue} |
 * {$:'primitiveInvalid',reason:string} | {$:'primitiveError',errorClass:string,reason:string} |
 * {$:'primitiveResourceLimit',reason:string}} JavaScriptCallPrimitiveResult */

/** Operations consume already checked primitive values. Unmodeled coercions,
 * wrappers and replacement intrinsics need their own source lowering.
 * @param {string} operation @param {number} count @returns {string} */
export function readJavaScriptCallPrimitiveArityError(operation,count){
 if(!Number.isSafeInteger(count)||count<0||count>4096)return 'primitive operands exceed their explicit call register budget';
 const integerArity=readJavaScriptCallIntegerArity(operation);
 if(integerArity!==0)return count===integerArity?'':'primitive instructions require every actual operand coordinate';
 if(operation==='read minimum number'||operation==='read maximum number'||operation==='construct text units')return '';
 const unary=['truncate number','read text length','trim text','trim text start','trim text end'];
 const binary=['read text code point','read text character','read relative text character','concatenate text','compare text','test text prefix','test text suffix','test text inclusion'];
 const ternary=['slice text','find text','find last text'];
 const arity=unary.includes(operation)?1:binary.includes(operation)?2:ternary.includes(operation)?3:-1;
 return arity<0?'primitive instructions require their actual registered operation':count!==arity?'primitive instructions require every actual operand coordinate':'';
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumericPrimitive(operation,values){
 let numbers=[];
 for(const value of values){if(value.$!=='number')return {$:'primitiveInvalid',reason:'numeric primitives require actual Number operands after source conversion'};numbers.push(value.value);}
 if(operation==='truncate number')return {$:'primitiveValue',value:{$:'number',value:Math.trunc(numbers[0]??NaN)}};
 if(operation==='read minimum number')return {$:'primitiveValue',value:{$:'number',value:Math.min(...numbers)}};
 if(operation==='read maximum number')return {$:'primitiveValue',value:{$:'number',value:Math.max(...numbers)}};
 if(operation==='construct text units')return {$:'primitiveValue',value:{$:'text',units:characterUnitsUtf16Text(numbers)}};
 return {$:'primitiveInvalid',reason:'the numeric primitive executor needs its actual registered operation'};
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
 if(operation==='concatenate text'){
  if(text.length+second.units.length>1048576)return {$:'primitiveResourceLimit',reason:'primitive text output exceeds its explicit UTF16 unit budget'};
  return {$:'primitiveValue',value:{$:'text',units:text.concat(second.units)}};
 }
 if(operation==='compare text')return {$:'primitiveValue',value:{$:'number',value:compareUtf16Text(text,second.units)}};
 if(operation==='test text prefix')return {$:'primitiveValue',value:{$:'boolean',value:startsUtf16Text(text,second.units)}};
 if(operation==='test text suffix')return {$:'primitiveValue',value:{$:'boolean',value:endsUtf16Text(text,second.units)}};
 // This explicit preflight bounds the existing search algorithm's maximum
 // unit comparisons, separately from input/output and language exceptions.
 if((Math.max(0,text.length-second.units.length)+1)*second.units.length>67108864)return {$:'primitiveResourceLimit',reason:'primitive text search exceeds its explicit comparison work quota'};
 if(operation==='test text inclusion')return {$:'primitiveValue',value:{$:'boolean',value:includesUtf16Text(text,second.units)}};
 if(operation==='find text'||operation==='find last text'){
  if(third.$!=='number')return {$:'primitiveInvalid',reason:'text search positions require their actual converted Number operand'};
  return {$:'primitiveValue',value:{$:'number',value:operation==='find text'?indexUtf16Text(text,second.units,third.value):lastIndexUtf16Text(text,second.units,third.value)}};
 }
 return {$:'primitiveInvalid',reason:'the text primitive executor needs its actual registered operation'};
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallPrimitive(operation,values){
 const arity=readJavaScriptCallPrimitiveArityError(operation,values.length);if(arity!=='')return {$:'primitiveInvalid',reason:arity};
 for(const value of values){const error=readJavaScriptSemanticValueError(value);if(error!=='')return {$:'primitiveInvalid',reason:error};}
 if(readJavaScriptCallIntegerArity(operation)!==0)return executeJavaScriptCallIntegerPrimitive(operation,values);
 return operation==='truncate number'||operation==='read minimum number'||operation==='read maximum number'||operation==='construct text units'?executeJavaScriptCallNumericPrimitive(operation,values):executeJavaScriptCallTextPrimitive(operation,values);
}
