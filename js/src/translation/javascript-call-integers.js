// Exact integer and Number bit operations reuse their normative definitions.
import {applyBigIntegerBitwiseOperation} from './big-integer-bitwise-rules.js';
import {applyJavaScriptNumberBitwiseOperation} from './number-bitwise-rules.js';
import {divideJavaScriptBigInteger,powerJavaScriptBigInteger,compareJavaScriptNumberBigInteger,compareJavaScriptStringBigIntegerUnits} from './big-integer-conversion-rules.js';

/** @param {string} operation @returns {number} */
export function readJavaScriptCallIntegerArity(operation){
 if(operation==='negate integer'||operation==='complement integer bits'||operation==='complement number bits')return 1;
 return ['add integers','subtract integers','multiply integers','divide integers','read integer remainder','raise integer power','compare integers','compare number integer','compare text integer',
 'conjoin integer bits','disjoin integer bits','exclude equal integer bits','shift integer bits left','shift integer bits right',
 'conjoin number bits','disjoin number bits','exclude equal number bits','shift number bits left','shift number bits right','shift number bits right unsigned'].includes(operation)?2:0;
}
/** The selected resource profile bounds intermediate work separately from
 * arbitrary-integer language semantics. -1 denotes an oversized input.
 * @param {bigint} value @returns {number} */
export function readJavaScriptCallIntegerWidth(value){
 let rest=value<0n?-value:value,width=0;
 while(rest>0n){if(width>=4096)return -1;rest/=2n;width+=1;}
 return width;
}
/** @param {string} operation @returns {number} */
export function readJavaScriptCallIntegerBitCode(operation){
 if(operation==='conjoin integer bits'||operation==='conjoin number bits')return 0;
 if(operation==='disjoin integer bits'||operation==='disjoin number bits')return 1;
 if(operation==='exclude equal integer bits'||operation==='exclude equal number bits')return 2;
 if(operation==='shift integer bits left'||operation==='shift number bits left')return 3;
 if(operation==='shift integer bits right'||operation==='shift number bits right')return 4;
 if(operation==='shift number bits right unsigned')return 5;
 if(operation==='complement integer bits'||operation==='complement number bits')return 6;
 return -1;
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallNumberBits(operation,values){
 const left=values[0]??{$:'undefined'},right=values[1]??{$:'number',value:0};
 if(left.$!=='number')return {$:'primitiveInvalid',reason:'Number bit operations require actual converted Number operands'};
 if(right.$!=='number')return {$:'primitiveInvalid',reason:'Number bit operations require actual converted Number operands'};
 const code=readJavaScriptCallIntegerBitCode(operation);
 if(code<0)return {$:'primitiveInvalid',reason:'Number bit operations require their actual registered operation'};
 return {$:'primitiveValue',value:{$:'number',value:applyJavaScriptNumberBitwiseOperation(code,left.value,right.value)}};
}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallIntegerPrimitive(operation,values){
 if(operation.includes('number bits'))return executeJavaScriptCallNumberBits(operation,values);
 if(operation==='compare number integer'||operation==='compare text integer')return executeJavaScriptCallMixedIntegerComparison(operation,values);
 const first=values[0]??{$:'undefined'},second=values[1]??{$:'integer',value:0n};
 if(first.$!=='integer')return {$:'primitiveInvalid',reason:'integer operations require actual converted BigInt operands'};
 if(second.$!=='integer')return {$:'primitiveInvalid',reason:'integer operations require actual converted BigInt operands'};
 const left=first.value,right=second.value;
 if((operation==='divide integers'||operation==='read integer remainder')&&right===0n)return {$:'primitiveError',errorClass:'RangeError',reason:'Division by zero'};
 if(operation==='raise integer power'&&right<0n)return {$:'primitiveError',errorClass:'RangeError',reason:'Exponent must be positive'};
 const leftWidth=readJavaScriptCallIntegerWidth(left),rightWidth=readJavaScriptCallIntegerWidth(right);
 if(leftWidth<0||rightWidth<0)return {$:'primitiveResourceLimit',reason:'integer input exceeds the selected 4096-bit work profile'};
 if(operation==='compare integers')return {$:'primitiveValue',value:{$:'number',value:left<right?-1:left>right?1:0}};
 if(operation==='multiply integers'&&leftWidth+rightWidth>4096)return {$:'primitiveResourceLimit',reason:'integer multiplication estimate exceeds the selected output work profile'};
 if(operation==='raise integer power'&&left!==0n&&left!==1n&&left!==-1n&&right*BigInt(leftWidth)>4096n)return {$:'primitiveResourceLimit',reason:'integer exponentiation estimate exceeds the selected output work profile'};
 if(operation==='shift integer bits left'||operation==='shift integer bits right'){
  const growing=operation==='shift integer bits left'?right>=0n:right<0n,count=right<0n?-right:right;
  if(left!==0n&&growing&&count>BigInt(4096-leftWidth))return {$:'primitiveResourceLimit',reason:'integer shift estimate exceeds the selected output work profile'};
 }
 const code=readJavaScriptCallIntegerBitCode(operation);
 let result=0n;
 if(operation==='add integers')result=left+right;
 else if(operation==='subtract integers')result=left-right;
 else if(operation==='multiply integers')result=left*right;
 else if(operation==='divide integers'||operation==='read integer remainder')result=divideJavaScriptBigInteger(left,right)[operation==='divide integers'?0:1]??0n;
 else if(operation==='raise integer power')result=powerJavaScriptBigInteger(left,right)[0]??0n;
 else if(operation==='negate integer')result=-left;
 else if(code>=0)result=applyBigIntegerBitwiseOperation(code,left,right);
 else return {$:'primitiveInvalid',reason:'integer primitives require their actual registered operation'};
 if(readJavaScriptCallIntegerWidth(result)<0)return {$:'primitiveResourceLimit',reason:'integer output exceeds the selected 4096-bit work profile'};
 return {$:'primitiveValue',value:{$:'integer',value:result}};
}

/** Two is the existing shared unordered result, not Number rounding of an
 * arbitrary integer. Text uses StringIntegerLiteral, preserving UTF16 units.
 * @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptCallMixedIntegerComparison(operation,values){
 const first=values[0]??{$:'undefined'},second=values[1]??{$:'undefined'};
 if(second.$!=='integer')return {$:'primitiveInvalid',reason:'mixed integer comparisons require their actual BigInt operand'};
 if(readJavaScriptCallIntegerWidth(second.value)<0)return {$:'primitiveResourceLimit',reason:'mixed comparison integer exceeds the selected 4096-bit work profile'};
 if(operation==='compare number integer'){
  if(first.$!=='number')return {$:'primitiveInvalid',reason:'mixed Number comparison requires its actual Number operand'};
  return {$:'primitiveValue',value:{$:'number',value:compareJavaScriptNumberBigInteger(first.value,second.value)}};
 }
 if(operation==='compare text integer'){
  if(first.$!=='text')return {$:'primitiveInvalid',reason:'mixed text comparison requires its actual UTF16 operand'};
  if(first.units.length>4096)return {$:'primitiveResourceLimit',reason:'mixed comparison text exceeds the selected 4096-unit work profile'};
  return {$:'primitiveValue',value:{$:'number',value:compareJavaScriptStringBigIntegerUnits(first.units,second.value)}};
 }
 return {$:'primitiveInvalid',reason:'mixed integer comparison requires its actual registered operation'};
}
