// Reuse the existing complete UTF16 definitions; adapters never substitute
// host scalar strings for code units or reimplement native operation bodies.
import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextString,constructJavaScriptSemanticTextUnits,constructJavaScriptSourceText} from './javascript-semantic-values.js';
import {readJavaScriptCallIntegerArity,executeJavaScriptCallIntegerPrimitive,readJavaScriptCallIntegerWidth} from './javascript-call-integers.js';
import {executeJavaScriptCallNumberConversion} from './javascript-call-number-conversion.js';
import {executeJavaScriptCallIntegerConversion} from './javascript-call-integer-conversion.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {readUtf16TextRepetitionLength,readUtf16TextPaddingLength,constructUtf16TextRepetition,constructUtf16TextPadding} from './utf16-text-expansion-rules.js';
import {replaceUtf16Text,planUtf16TextSubstitution,sliceUtf16Text,codePointUtf16Text,characterUtf16Text,relativeCharacterUtf16Text,indexUtf16Text,lastIndexUtf16Text,compareUtf16Text,startsUtf16Text,endsUtf16Text,includesUtf16Text,trimUtf16Text,characterUnitsUtf16Text} from './utf16-text-rules.js';

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
 const unary=['test object value','read semantic type','read text length','trim text','trim text start','trim text end','convert explicit number','coerce primitive number','coerce primitive numeric','convert primitive value','convert number primitive value','increment numeric value','decrement numeric value','negate numeric value','convert primitive text','convert explicit integer','coerce primitive integer'];
 const binary=['compare relational values','subtract numeric values','multiply numeric values','divide numeric values','read numeric remainder','raise numeric power','repeat text','read text code point','read text character','read relative text character','concatenate text','compare text','test text prefix','test text suffix','test text inclusion','add primitive values'];
 const ternary=['pad text start','pad text end','slice text','find text','find last text','replace text','replace all text'];
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
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallPrimitive(operation,values){
 const arity=readJavaScriptCallPrimitiveArityError(operation,values.length);if(arity!=='')return {$:'primitiveInvalid',reason:arity};
 for(const value of values){const error=readJavaScriptSemanticValueError(value);if(error!=='')return {$:'primitiveInvalid',reason:error};}
 if(operation==='increment numeric value'||operation==='decrement numeric value'||operation==='negate numeric value')return executeJavaScriptCallNumericUpdate(operation,values[0]??{$:'undefined'});
 if(['subtract numeric values','multiply numeric values','divide numeric values','read numeric remainder','raise numeric power'].includes(operation))return executeJavaScriptCallNumericBinary(operation,values[0]??{$:'undefined'},values[1]??{$:'undefined'});
 if(operation==='compare relational values')return executeJavaScriptCallRelationalComparison(values[0]??{$:'undefined'},values[1]??{$:'undefined'});
 if(operation==='convert primitive value'||operation==='convert number primitive value'){const value=values[0]??{$:'undefined'};return value.$==='object'?{$:'primitiveInvalid',reason:'object ToPrimitive requires its actual selected-hint continuation'}:{$:'primitiveValue',value};}
 if(operation==='add primitive values')return executeJavaScriptCallPrimitiveAddition(values[0]??{$:'undefined'},values[1]??{$:'undefined'});
 if(operation==='test object value')return {$:'primitiveValue',value:{$:'boolean',value:(values[0]??{$:'undefined'}).$==='object'}};
 if(operation==='read semantic type'){
  const value=values[0]??{$:'undefined'},kind=value.$==='null'?'object':value.$==='object'?(value.callable?'function':'object'):value.$==='integer'?'bigint':value.$==='text'||value.$==='sourceText'?'string':value.$,units=[];
  for(let at=0;at<kind.length;at++)units.push(kind.charCodeAt(at));return {$:'primitiveValue',value:{$:'text',units}};
 }
 if(readJavaScriptSourceExceptionClass(operation)!=='')return {$:'primitiveInvalid',reason:'source Error allocation requires its actual registered realm'};
 if(operation==='convert explicit number'||operation==='coerce primitive number'||operation==='coerce primitive numeric')return executeJavaScriptCallNumberConversion(operation,values[0]??{$:'undefined'});
 if(operation==='convert explicit integer'||operation==='coerce primitive integer')return executeJavaScriptCallIntegerConversion(operation,values[0]??{$:'undefined'});
 if(operation==='convert primitive text')return executeJavaScriptCallTextConversion(values[0]??{$:'undefined'});
 if(values.some(value=>value.$==='sourceText')){
  const first=values[0]??{$:'undefined'},second=values[1]??{$:'undefined'};
  if(operation==='read text length'&&readJavaScriptSemanticTextLength(first)>=0)return {$:'primitiveValue',value:{$:'number',value:readJavaScriptSemanticTextLength(first)}};
  if(operation==='concatenate text'&&readJavaScriptSemanticTextLength(first)>=0&&readJavaScriptSemanticTextLength(second)>=0){
   if(readJavaScriptSemanticTextLength(first)+readJavaScriptSemanticTextLength(second)>16777216)return {$:'primitiveResourceLimit',reason:'source text concatenation exceeds the selected 16777216 UTF16 unit profile'};
   return {$:'primitiveValue',value:constructJavaScriptSourceText(readJavaScriptSemanticTextString(first)+readJavaScriptSemanticTextString(second))};
  }
  if(values.some(value=>value.$==='sourceText'&&value.text.length>1048576))return {$:'primitiveResourceLimit',reason:'this text operation requires its separate bounded unit-array algorithm; source text remains inspectable and resumable'};
  return executeJavaScriptCallPrimitive(operation,values.map(value=>value.$==='sourceText'?{$:'text',units:constructJavaScriptSemanticTextUnits(value,1048576)}:value));
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
 if(first.value.$!=='number'||second.value.$!=='number')return {$:'primitiveInvalid',reason:'numeric addition retains its actual Number conversion results'};
 return {$:'primitiveValue',value:{$:'number',value:first.value.value+second.value.value}};
}

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
 if(left.$!=='number'||right.$!=='number')return {$:'primitiveInvalid',reason:'numeric arithmetic retains matching Number or BigInt operands'};
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
 if(a.$!=='number'||b.$!=='number')return {$:'primitiveInvalid',reason:'relational comparison retains its completed numeric operands'};
 return {$:'primitiveValue',value:{$:'number',value:Number.isNaN(a.value)||Number.isNaN(b.value)?2:a.value<b.value?-1:a.value>b.value?1:0}};
}
/** @param {JavaScriptCallPrimitiveResult} result @returns {JavaScriptCallPrimitiveResult} */
export function invertJavaScriptCallRelationalComparison(result){
 if(result.$!=='primitiveValue')return result;if(result.value.$!=='number')return {$:'primitiveInvalid',reason:'relational comparison inversion retains its actual ordering code'};
 const value=result.value.value;return {$:'primitiveValue',value:{$:'number',value:value===2?2:value===0?0:-value}};
}
