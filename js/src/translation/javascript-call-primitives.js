import {readJavaScriptCallIntegerArity,readJavaScriptNumericParsingArity,readJavaScriptStringMethodPrimitiveArity,readJavaScriptCallPrimitiveArityError,testJavaScriptCallUnaryNumberOperation} from './javascript-call-primitive-arity-rules.js';
import {constructJavaScriptSymbolDescriptiveText} from './javascript-symbol-descriptive-text.js';
import {executeJavaScriptNumericParsing} from './javascript-numeric-parsing-rules.js';
import {multiplyJavaScriptSignedIntegerBits,countJavaScriptLeadingZeroBits,roundJavaScriptBinary32} from './number-bitwise-rules.js';
import {executeJavaScriptStringMethodPrimitive} from './javascript-string-method-rules.js';
// Reuse the existing complete UTF16 definitions; adapters never substitute
// host scalar strings for code units or reimplement native operation bodies.
import {readJavaScriptArrayLikeLength} from './javascript-array-index-rules.js';
import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextString,constructJavaScriptSemanticTextUnits,constructJavaScriptSourceText} from './javascript-semantic-values.js';
import {executeJavaScriptCallIntegerPrimitive} from './javascript-call-integers.js';
import {executeJavaScriptCallTextConversion as executeJavaScriptScalarTextConversion} from './javascript-call-text-conversion.js';
import {executeJavaScriptCallNumberConversion} from './javascript-call-number-conversion.js';
import {executeJavaScriptCallScalarConversion,readJavaScriptCallScalarConversionError} from './javascript-call-scalar-conversion.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {readUtf16TextRepetitionLength,readUtf16TextPaddingLength,constructUtf16TextRepetition,constructUtf16TextPadding} from './utf16-text-expansion-rules.js';
import {replaceUtf16Text,planUtf16TextSubstitution,sliceUtf16Text,codePointUtf16Text,characterUtf16Text,relativeCharacterUtf16Text,indexUtf16Text,lastIndexUtf16Text,compareUtf16Text,startsUtf16Text,endsUtf16Text,includesUtf16Text,trimUtf16Text,characterUnitsUtf16Text} from './utf16-text-rules.js';

/** @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextConversion(value) {
 return executeJavaScriptScalarTextConversion(value);
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
 if(operation==='test NaN number')return {$:'primitiveValue',value:{$:'boolean',value:Number.isNaN(number)}};
 if(operation==='test finite number')return {$:'primitiveValue',value:{$:'boolean',value:Number.isFinite(number)}};
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
 if(values.some(value=>value.$==='sourceText'))return executeJavaScriptCallPrimitive(operation,values);
 if(operation==='repeat text'||operation==='pad text start'||operation==='pad text end')return executeJavaScriptCallTextExpansion(operation,values);
 if(operation==='replace text'||operation==='replace all text')return executeJavaScriptCallTextReplacementOperands(values,operation==='replace all text');
 const receiver=values[0]??{$:'undefined'};if(receiver.$!=='text')return {$:'primitiveInvalid',reason:'text primitives require their actual complete UTF16 receiver'};
 const text=receiver.units;
 if(operation==='read text length')return {$:'primitiveValue',value:{$:'number',value:text.length}};
 if(operation==='trim text'||operation==='trim text start'||operation==='trim text end')return {$:'primitiveValue',value:{$:'text',units:trimUtf16Text(text,operation!=='trim text end',operation!=='trim text start')}};
 if(operation==='read text code point'||operation==='read text character'||operation==='read relative text character'||operation==='slice text')return executeJavaScriptCallTextPosition(operation,text,values);
 return executeJavaScriptCallTextPair(operation,text,values);
}
/** Converted operands only. Resource limits remain distinct from exceptions.
 * @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextExpansion(operation,values){
 if(values.some(value=>value.$==='sourceText'))return executeJavaScriptCallPrimitive(operation,values);
 const receiver=values[0]??{$:'undefined'};
 if(receiver.$!=='text')return {$:'primitiveInvalid',reason:'text expansion requires its actual complete UTF16 receiver'};
 const count=values[1]??{$:'undefined'};
 if(count.$!=='number')return {$:'primitiveInvalid',reason:'text expansion requires its actual converted Number bound'};
 if(operation==='repeat text'){
  const length=readUtf16TextRepetitionLength(receiver.units.length,count.value);
  if(length===-1)return {$:'primitiveError',errorClass:'RangeError',reason:'Invalid count value: '+String(count.value)};
  if(length<0)return {$:'primitiveResourceLimit',reason:'text expansion exceeds its explicit UTF16 output budget'};
  return {$:'primitiveValue',value:{$:'text',units:constructUtf16TextRepetition(receiver.units,length)}};
 }
 if(operation!=='pad text start'&&operation!=='pad text end')return {$:'primitiveInvalid',reason:'text expansion requires its actual registered operation'};
 const filler=values[2]??{$:'undefined'};
 if(filler.$!=='text')return {$:'primitiveInvalid',reason:'text padding requires its actual converted UTF16 filler'};
 const length=readUtf16TextPaddingLength(receiver.units.length,count.value,filler.units.length);
 if(length<0)return {$:'primitiveResourceLimit',reason:'text expansion exceeds its explicit UTF16 output budget'};
 return {$:'primitiveValue',value:{$:'text',units:constructUtf16TextPadding(receiver.units,filler.units,length,operation==='pad text start')}};
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
/** Keep the new operand match independent of existing text-search continuations.
 * @param {JavaScriptSemanticValue[]} values @param {boolean} all @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextReplacementOperands(values,all){
 if(values.some(value=>value.$==='sourceText'))return executeJavaScriptCallPrimitive(all?'replace all text':'replace text',values);
 const text=values[0]??{$:'undefined'},search=values[1]??{$:'undefined'},replacement=values[2]??{$:'undefined'};
 if(text.$!=='text')return {$:'primitiveInvalid',reason:'text replacement requires its three actual converted UTF16 operands'};
 if(search.$!=='text')return {$:'primitiveInvalid',reason:'text replacement requires its three actual converted UTF16 operands'};
 if(replacement.$!=='text')return {$:'primitiveInvalid',reason:'text replacement requires its three actual converted UTF16 operands'};
 return executeJavaScriptCallTextReplacement(text.units,search.units,replacement.units,all);
}
/** Preflight the complete existing substitution algorithm without executing it.
 * @param {number} length @param {number} search @param {number[]} replacement
 * @param {boolean} all @returns {string} */
export function readJavaScriptCallTextReplacementWorkError(length,search,replacement,all){
 if(length>1048576||search>1048576||replacement.length>4096)return 'text replacement exceeds its explicit input or substitution-plan budget';
 let matches=search===0?length+1:Math.floor(length/search);
 if(!all)matches=Math.min(1,matches);
 if(matches===0)return '';
 if((length+1)*search>67108864)return 'text replacement exceeds its explicit search work quota';
 const plan=planUtf16TextSubstitution(replacement);
 if(matches*(plan.length/3)>67108864)return 'text replacement exceeds its explicit substitution work quota';
 let width=0;
 for(let index=0;index<plan.length;index+=3){
  const kind=plan[index]??0;
  width+=kind===0?(plan[index+2]??0)-(plan[index+1]??0):kind===1?search:length;
 }
 return length+matches*width>1048576?'text replacement exceeds its conservative UTF16 output work bound':'';
}
/** Reuse literal GetSubstitution after the independent work/output preflight.
 * @param {number[]} text @param {number[]} search @param {number[]} replacement
 * @param {boolean} all @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallTextReplacement(text,search,replacement,all){
 const error=readJavaScriptCallTextReplacementWorkError(text.length,search.length,replacement,all);
 return error!==''?{$:'primitiveResourceLimit',reason:error}:{$:'primitiveValue',value:{$:'text',units:replaceUtf16Text(text,search,replacement,all)}};
}
/** @param {string} reason @returns {JavaScriptCallPrimitiveResult} */
function constructJavaScriptCallPrimitiveTypeError(reason){return {$:'primitiveError',errorClass:'TypeError',reason};}

/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallPrimitive(operation,values){
 const arity=readJavaScriptCallPrimitiveArityError(operation,values.length);if(arity!=='')return {$:'primitiveInvalid',reason:arity};
 if(operation==='throw restricted Function access')return {$:'primitiveError',errorClass:'TypeError',reason:"'caller', 'callee', and 'arguments' properties may not be accessed on strict mode functions or the arguments objects for calls to them"};
 for(const value of values){const error=readJavaScriptSemanticValueError(value);if(error!=='')return {$:'primitiveInvalid',reason:error};}
 if(operation==='require Array creation length'){
  const value=values[0]??{$:'undefined'};const invalid='ArrayCreate length follows actual ToLength';if(value.$!=='number')return {$:'primitiveInvalid',reason:invalid};if(!Number.isSafeInteger(value.value)||value.value<0||Object.is(value.value,-0))return {$:'primitiveInvalid',reason:invalid};
  return value.value>4294967295?{$:'primitiveError',errorClass:'RangeError',reason:'Invalid array length'}:{$:'primitiveValue',value};
 }
 if(operation==='read checked array replacement index'){
  const length=values[0]??{$:'undefined'},value=values[1]??{$:'undefined'};
  const invalid='replacement indices retain actual ToLength and completed ToNumber';
  if(length.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(!Number.isSafeInteger(length.value)||length.value<0||Object.is(length.value,-0))return {$:'primitiveInvalid',reason:invalid};
  if(value.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  const integer=Number.isNaN(value.value)||value.value===0?0:Math.trunc(value.value),index=integer<0?length.value+integer:integer;
  return index<0||index>=length.value?{$:'primitiveError',errorClass:'RangeError',reason:'Invalid index'}:{$:'primitiveValue',value:{$:'number',value:index===0?0:index}};
 }
 if(operation==='read array deletion count'){
  const maximum=values[0]??{$:'undefined'},value=values[1]??{$:'undefined'};
  const invalid='array deletion count retains captured remaining length and completed ToNumber';
  if(maximum.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(!Number.isSafeInteger(maximum.value)||maximum.value<0||Object.is(maximum.value,-0))return {$:'primitiveInvalid',reason:invalid};
  if(value.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  const integer=Number.isNaN(value.value)||value.value===0?0:Math.trunc(value.value);return {$:'primitiveValue',value:{$:'number',value:Math.min(Math.max(integer,0),maximum.value)}};
 }
 if(operation==='construct Symbol descriptive text')return constructJavaScriptSymbolDescriptiveText(values[0]??{$:'undefined'});
 if(readJavaScriptNumericParsingArity(operation)>=0)return executeJavaScriptNumericParsing(operation,values);
 if(operation==='round binary32 number'){const value=values[0]??{$:'undefined'};return value.$==='number'?{$:'primitiveValue',value:{$:'number',value:roundJavaScriptBinary32(value.value)}}:{$:'primitiveInvalid',reason:'binary32 rounding requires its completed Number operand'};}
 if(operation==='count leading zero bits'){const value=values[0]??{$:'undefined'};return value.$==='number'?{$:'primitiveValue',value:{$:'number',value:countJavaScriptLeadingZeroBits(value.value)}}:{$:'primitiveInvalid',reason:'leading zero count requires its converted Number operand'};}
 if(operation==='multiply signed integer bits'){
  const left=values[0]??{$:'undefined'},right=values[1]??{$:'undefined'};
  const invalid='integer multiplication requires its converted Number operands';
  if(left.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(right.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  return {$:'primitiveValue',value:{$:'number',value:multiplyJavaScriptSignedIntegerBits(left.value,right.value)}};
 }
 if(readJavaScriptStringMethodPrimitiveArity(operation)>=0)return executeJavaScriptStringMethodPrimitive(operation,values);
 if(operation==='increment numeric value'||operation==='decrement numeric value'||operation==='negate numeric value')return executeJavaScriptCallNumericUpdate(operation,values[0]??{$:'undefined'});
 if(['subtract numeric values','multiply numeric values','divide numeric values','read numeric remainder','raise numeric power'].includes(operation))return executeJavaScriptCallNumericBinary(operation,values[0]??{$:'undefined'},values[1]??{$:'undefined'});
 if(operation==='compare relational values')return executeJavaScriptCallRelationalComparison(values[0]??{$:'undefined'},values[1]??{$:'undefined'});

 if(operation==='add primitive values')return executeJavaScriptCallPrimitiveAddition(values[0]??{$:'undefined'},values[1]??{$:'undefined'});
 if(operation==='require array method object'||operation==='require ordinary property object'){
  const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:value.$==='null'||value.$==='undefined'?{$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert undefined or null to object'}:{$:'primitiveInvalid',reason:operation==='require array method object'?'Array method receiver requires its actual primitive boxing protocol':'ordinary property receiver requires its actual primitive boxing protocol'};
 }
 if(operation==='require V8 array join length'){const value=values[0]??{$:'undefined'};const invalid='V8 join length follows actual ToLength';if(value.$!=='number')return {$:'primitiveInvalid',reason:invalid};if(!Number.isSafeInteger(value.value)||value.value<0||Object.is(value.value,-0))return {$:'primitiveInvalid',reason:invalid};return value.value>4294967295?{$:'primitiveError',errorClass:'TypeError',reason:'Invalid array length'}:{$:'primitiveValue',value};}
 if(operation==='require callable target'){const value=values[0]??{$:'undefined'};const invalid=constructJavaScriptCallPrimitiveTypeError('Function invocation requires a callable target');if(value.$!=='object')return invalid;return value.callable?{$:'primitiveValue',value}:invalid;}
 if(operation==='require array like arguments Object'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:{$:'primitiveError',errorClass:'TypeError',reason:'CreateListFromArrayLike requires an Object argument list'};}
 if(operation==='require Object assign target'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:value.$==='null'||value.$==='undefined'?{$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert undefined or null to object'}:{$:'primitiveInvalid',reason:'Object.assign primitive targets require actual wrapper allocation and internal slots'};}
 if(operation==='require Object reflection target'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:{$:'primitiveError',errorClass:'TypeError',reason:'Reflect property operations require an Object target'};}
 if(operation==='require object for in'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:{$:'primitiveError',errorClass:'TypeError',reason:'The right operand of in must be an Object'};}
 if(operation==='require nonnull Object input'){const value=values[0]??{$:'undefined'};return value.$==='null'||value.$==='undefined'?{$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert undefined or null to object'}:{$:'primitiveValue',value};}
 if(operation==='read integer index'){
  const value=values[0]??{$:'undefined'};if(value.$!=='number')return {$:'primitiveInvalid',reason:'ToIndex consumes its completed ToNumber value'};
  const index=Number.isNaN(value.value)||value.value===0?0:Math.trunc(value.value);
  return index<0||index>9007199254740991?{$:'primitiveError',errorClass:'RangeError',reason:'Index must be between 0 and 9007199254740991'}:{$:'primitiveValue',value:{$:'number',value:index===0?0:index}};
 }
 if(operation==='read array like length'){const value=values[0]??{$:'undefined'};return value.$==='number'?{$:'primitiveValue',value:{$:'number',value:readJavaScriptArrayLikeLength(value.value)}}:{$:'primitiveInvalid',reason:'ToLength consumes its completed ToNumber value'};}
 if(operation==='read relative array index'){
  const length=values[0]??{$:'undefined'},number=values[1]??{$:'undefined'};
  const invalid='relative Array indices retain actual ToLength and completed ToNumber operands';
  if(length.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(!Number.isSafeInteger(length.value)||length.value<0)return {$:'primitiveInvalid',reason:invalid};
  if(number.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  const integer=Number.isNaN(number.value)||number.value===0?0:Math.trunc(number.value),index=integer===-Infinity?0:integer<0?Math.max(length.value+integer,0):Math.min(integer,length.value);return {$:'primitiveValue',value:{$:'number',value:index}};
 }
 if(operation==='read checked array append length'){
  const length=values[0]??{$:'undefined'},count=values[1]??{$:'undefined'};
  const invalid='Array append length retains actual ToLength and argument count coordinates';
  if(length.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(count.$!=='number')return {$:'primitiveInvalid',reason:invalid};
  if(!Number.isSafeInteger(length.value)||length.value<0||!Number.isSafeInteger(count.value)||count.value<0)return {$:'primitiveInvalid',reason:invalid};
  return length.value>9007199254740991-count.value?{$:'primitiveError',errorClass:'TypeError',reason:'Array-like length exceeds the maximum safe integer'}:{$:'primitiveValue',value:{$:'number',value:length.value+count.value}};
 }
 if(operation==='require delete success'){const value=values[0]??{$:'undefined'};return value.$!=='boolean'?{$:'primitiveInvalid',reason:'strict deletion consumes its actual Boolean result'}:value.value?{$:'primitiveValue',value:{$:'undefined'}}:{$:'primitiveError',errorClass:'TypeError',reason:'Cannot delete a non-configurable property'};}
 if(operation==='require collection adder'){const value=values[0]??{$:'undefined'};const invalid=constructJavaScriptCallPrimitiveTypeError('Collection adder is not callable');if(value.$!=='object')return invalid;return value.callable?{$:'primitiveValue',value}:invalid;}
 if(operation==='require Map entry object'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveValue',value}:{$:'primitiveError',errorClass:'TypeError',reason:'Iterator value is not an entry object'};}
 if(operation==='require object coercible'){const value=values[0]??{$:'undefined'};return value.$==='null'||value.$==='undefined'?{$:'primitiveError',errorClass:'TypeError',reason:'Cannot destructure null or undefined'}:{$:'primitiveValue',value:{$:'undefined'}};}
 if(operation==='test object value')return {$:'primitiveValue',value:{$:'boolean',value:(values[0]??{$:'undefined'}).$==='object'}};
 if(operation==='read semantic type'){
  const value=values[0]??{$:'undefined'},kind=value.$==='null'?'object':value.$==='object'?(value.callable?'function':'object'):value.$==='integer'?'bigint':value.$==='text'||value.$==='sourceText'?'string':value.$,units=[];
  for(let at=0;at<kind.length;at++)units.push(kind.charCodeAt(at));return {$:'primitiveValue',value:{$:'text',units}};
 }
 if(readJavaScriptSourceExceptionClass(operation)!=='')return {$:'primitiveInvalid',reason:'source Error allocation requires its actual registered realm'};
 if(readJavaScriptCallScalarConversionError(operation)==='')return executeJavaScriptCallScalarConversion(operation,values[0]??{$:'undefined'});
 if(values.some(value=>value.$==='sourceText')){
  const first=values[0]??{$:'undefined'},second=values[1]??{$:'undefined'};
  if(operation==='read text length'&&readJavaScriptSemanticTextLength(first)>=0)return {$:'primitiveValue',value:{$:'number',value:readJavaScriptSemanticTextLength(first)}};
  if(operation==='concatenate text'&&readJavaScriptSemanticTextLength(first)>=0&&readJavaScriptSemanticTextLength(second)>=0){
   if(readJavaScriptSemanticTextLength(first)+readJavaScriptSemanticTextLength(second)>16777216)return {$:'primitiveResourceLimit',reason:'source text concatenation exceeds the selected 16777216 UTF16 unit profile'};
   return {$:'primitiveValue',value:constructJavaScriptSourceText(readJavaScriptSemanticTextString(first)+readJavaScriptSemanticTextString(second))};
  }
  if(values.some(value=>value.$==='sourceText'&&value.text.length>1048576))return {$:'primitiveResourceLimit',reason:'this text operation requires its separate bounded unit-array algorithm; source text remains inspectable and resumable'};
  /** @type {JavaScriptSemanticValue[]} */
  let materialized=[];
  for(const value of values){
   /** @type {JavaScriptSemanticValue} */
   const operand=value.$==='sourceText'?{$:'text',units:constructJavaScriptSemanticTextUnits(value,1048576)}:value;
   materialized=materialized.concat([operand]);
  }
  return executeJavaScriptCallPrimitive(operation,materialized);
 }
 if(readJavaScriptCallIntegerArity(operation)!==0)return executeJavaScriptCallIntegerPrimitive(operation,values);
 return testJavaScriptCallUnaryNumberOperation(operation)||['read minimum number','read maximum number','construct text units'].includes(operation)?executeJavaScriptCallNumericPrimitive(operation,values):executeJavaScriptCallTextPrimitive(operation,values);
}
/** Primitive Add follows the actual string-vs-numeric branch after evaluation.
 * Objects require the source-ordered ToPrimitive continuation, never erasure.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right
 * @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallPrimitiveAddition(left,right){
 if(left.$==='object'||right.$==='object')return {$:'primitiveInvalid',reason:'addition requires its actual ordered object ToPrimitive continuations'};
 if(left.$==='text'||left.$==='sourceText'||right.$==='text'||right.$==='sourceText'){
  const first=executeJavaScriptCallTextConversion(left);if(first.$!=='primitiveValue')return first;const second=executeJavaScriptCallTextConversion(right);if(second.$!=='primitiveValue')return second;return executeJavaScriptCallPrimitive('concatenate text',[first.value,second.value]);
 }
 if(left.$==='symbol'||right.$==='symbol')return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Symbol value to a number'};
 if(left.$==='integer'&&right.$==='integer')return executeJavaScriptCallIntegerPrimitive('add integers',[left,right]);
 if(left.$==='integer'||right.$==='integer')return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot mix BigInt and other types, use explicit conversions'};
 const first=executeJavaScriptCallNumberConversion('coerce primitive number',left);if(first.$!=='primitiveValue')return first;const second=executeJavaScriptCallNumberConversion('coerce primitive number',right);if(second.$!=='primitiveValue')return second;
 const firstNumber=first.value,secondNumber=second.value;
 const invalid='numeric addition retains its actual Number conversion results';
 if(firstNumber.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 if(secondNumber.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 return {$:'primitiveValue',value:{$:'number',value:firstNumber.value+secondNumber.value}};
}

/** ToNumeric has already completed; preserve Number versus BigInt exactly.
 * @param {string} operation @param {JavaScriptSemanticValue} value @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumericUpdate(operation,value){
 if(operation!=='increment numeric value'&&operation!=='decrement numeric value'&&operation!=='negate numeric value')return {$:'primitiveInvalid',reason:'numeric updates require their actual increment or decrement operation'};
 if(value.$==='number')return {$:'primitiveValue',value:{$:'number',value:operation==='increment numeric value'?value.value+1:operation==='decrement numeric value'?value.value-1:-value.value}};
 if(value.$==='integer')return operation==='negate numeric value'?executeJavaScriptCallIntegerPrimitive('negate integer',[value]):executeJavaScriptCallIntegerPrimitive(operation==='increment numeric value'?'add integers':'subtract integers',[value,{$:'integer',value:1n}]);
 return {$:'primitiveInvalid',reason:'numeric updates retain their actual completed Number or BigInt conversion'};
}

/** Arithmetic after left-to-right ToNumeric, with no implicit Number/BigInt mix.
 * @param {string} operation @param {JavaScriptSemanticValue} left
 * @param {JavaScriptSemanticValue} right @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumericBinary(operation,left,right){
 if(!['subtract numeric values','multiply numeric values','divide numeric values','read numeric remainder','raise numeric power'].includes(operation))return {$:'primitiveInvalid',reason:'numeric binary instructions require their actual arithmetic operation'};
 if(!['number','integer'].includes(left.$)||!['number','integer'].includes(right.$))return {$:'primitiveInvalid',reason:'numeric arithmetic retains both actual completed ToNumeric operands'};
 if(left.$!==right.$)return {$:'primitiveError',errorClass:'TypeError',reason:'Cannot mix BigInt and other types, use explicit conversions'};
 if(left.$==='integer'&&right.$==='integer')return executeJavaScriptCallIntegerPrimitive(operation==='subtract numeric values'?'subtract integers':operation==='multiply numeric values'?'multiply integers':operation==='divide numeric values'?'divide integers':operation==='read numeric remainder'?'read integer remainder':'raise integer power',[left,right]);
 const invalid='numeric arithmetic retains matching Number or BigInt operands';
 if(left.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 if(right.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 const a=left.value,b=right.value;return {$:'primitiveValue',value:{$:'number',value:operation==='subtract numeric values'?a-b:operation==='multiply numeric values'?a*b:operation==='divide numeric values'?a/b:operation==='read numeric remainder'?a%b:a**b}};
}

/** Abstract relational comparison after ordered number-hint ToPrimitive.
 * Two denotes unordered; it must never be treated as a true >= relation.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallRelationalComparison(left,right){
 if(left.$==='object'||right.$==='object')return {$:'primitiveInvalid',reason:'relational comparison requires both actual ToPrimitive completions'};
 const textLeft=left.$==='text'||left.$==='sourceText',textRight=right.$==='text'||right.$==='sourceText';
 if(textLeft&&textRight)return executeJavaScriptCallPrimitive('compare text',[left,right]);
 if(textLeft&&right.$==='integer')return executeJavaScriptCallIntegerPrimitive('compare text integer',[left,right]);
 if(left.$==='integer'&&textRight)return invertJavaScriptCallRelationalComparison(executeJavaScriptCallIntegerPrimitive('compare text integer',[right,left]));
 const first=executeJavaScriptCallPrimitive('coerce primitive numeric',[left]);if(first.$!=='primitiveValue')return first;const second=executeJavaScriptCallPrimitive('coerce primitive numeric',[right]);if(second.$!=='primitiveValue')return second;
 const a=first.value,b=second.value;
 if(a.$==='integer'&&b.$==='integer')return executeJavaScriptCallIntegerPrimitive('compare integers',[a,b]);
 if(a.$==='number'&&b.$==='integer')return executeJavaScriptCallIntegerPrimitive('compare number integer',[a,b]);
 if(a.$==='integer'&&b.$==='number')return invertJavaScriptCallRelationalComparison(executeJavaScriptCallIntegerPrimitive('compare number integer',[b,a]));
 const invalid='relational comparison retains its completed numeric operands';
 if(a.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 if(b.$!=='number')return {$:'primitiveInvalid',reason:invalid};
 return {$:'primitiveValue',value:{$:'number',value:Number.isNaN(a.value)||Number.isNaN(b.value)?2:a.value<b.value?-1:a.value>b.value?1:0}};
}
/** @param {JavaScriptCallPrimitiveResult} result @returns {JavaScriptCallPrimitiveResult} */
export function invertJavaScriptCallRelationalComparison(result){
 if(result.$!=='primitiveValue')return result;
 const order=result.value;
 if(order.$!=='number')return {$:'primitiveInvalid',reason:'relational comparison inversion retains its actual ordering code'};
 const value=order.value;return {$:'primitiveValue',value:{$:'number',value:value===2?2:value===0?0:-value}};
}
