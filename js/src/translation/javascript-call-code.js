import {readJavaScriptSemanticValueError,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';

/** Registers are coordinates in checked executable Links data, not language
 * property keys. Coercions belong to explicit operations before typed opcodes.
 * @typedef {{$:'constant',destination:number,value:JavaScriptSemanticValue} |
 * {$:'copy',destination:number,source:number} |
 * {$:'argument',destination:number,index:number} |
 * {$:'jump',target:number} | {$:'branch',condition:number,yes:number,no:number} |
 * {$:'call',destination:number,callee:number,receiver:number,arguments:number[]} |
 * {$:'return',source:number} | {$:'throw',source:number} |
 * {$:'pushHandler',destination:number,target:number} | {$:'popHandler'} |
 * {$:'numberBinary',destination:number,left:number,right:number,operator:string} |
 * {$:'sameValue',destination:number,left:number,right:number} |
 * {$:'strictEqual',destination:number,left:number,right:number} |
 * {$:'ordinaryGet',destination:number,object:number,key:number,receiver:number} |
 * {$:'ordinarySet',destination:number,object:number,key:number,value:number,receiver:number} |
 * {$:'propertyKey',destination:number,input:number,realm:bigint} |
 * {$:'propertyDefinition',destination:number,target:number,key:number,attributes:number,realm:bigint}} JavaScriptCallInstruction
 */
/** Strict functions bind the actual receiver unchanged. Lexical functions use
 * their captured receiver. Sloppy receiver boxing and arguments objects need
 * their own source lowering, rather than an invented frame convention.
 * @typedef {{$:'callFunction',identity:bigint,realm:bigint,receiverMode:string,
 * lexicalReceiver:JavaScriptSemanticValue,parameters:number,registers:number,
 * captures:JavaScriptSemanticValue[],instructions:JavaScriptCallInstruction[]}} JavaScriptCallFunction
 */
/** Realm identifiers are supplied by the actual intrinsic registry. Membership
 * checks do not prove authority or allocate a substitute realm/Error object.
 * @typedef {{$:'callProgram',functions:JavaScriptCallFunction[],realms:bigint[],exoticKey:JavaScriptSemanticValue}} JavaScriptCallProgram
 */
/** @param {number} value @param {number} limit @returns {boolean} */
export function testJavaScriptCallCodeCoordinate(value,limit){return Number.isSafeInteger(value)&&value>=0&&value<limit&&(value!==0||1/value>0);}
/** @param {JavaScriptCallProgram} program @param {bigint} identity @returns {number} */
export function findJavaScriptCallCodeFunction(program,identity){
  let low=0,high=program.functions.length;
  while(low<high){const middle=Math.floor((low+high)/2),entry=program.functions[middle];if(entry===undefined)return -1;if(entry.identity===identity)return middle;if(entry.identity<identity)low=middle+1;else high=middle;}
  return -1;
}
/** @param {JavaScriptCallInstruction} instruction @returns {number[]} */
export function readJavaScriptCallCodeRegisters(instruction){
  switch(instruction.$){
    case 'constant':case 'argument':case 'pushHandler':return [instruction.destination];
    case 'copy':return [instruction.destination,instruction.source];
    case 'jump':case 'popHandler':return [];
    case 'branch':return [instruction.condition];
    case 'call':return [instruction.destination,instruction.callee,instruction.receiver].concat(instruction.arguments);
    case 'return':case 'throw':return [instruction.source];
    case 'numberBinary':case 'sameValue':case 'strictEqual':return [instruction.destination,instruction.left,instruction.right];
    case 'ordinaryGet':return [instruction.destination,instruction.object,instruction.key,instruction.receiver];
    case 'ordinarySet':return [instruction.destination,instruction.object,instruction.key,instruction.value,instruction.receiver];
    case 'propertyKey':return [instruction.destination,instruction.input];
    case 'propertyDefinition':return [instruction.destination,instruction.target,instruction.key,instruction.attributes];
  }
}
/** Preserve a real typed dependency on the shared semantic operand model.
 * @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptCallConstantError(value){return readJavaScriptSemanticValueError(value);}

/** Strict equality has the Number equality relation rather than SameValue:
 * NaN is unequal to itself, and the signs of zero compare equal.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right
 * @returns {boolean} */
export function testJavaScriptCallStrictEqual(left,right){
 if(readJavaScriptSemanticValueError(left)!==''||readJavaScriptSemanticValueError(right)!=='')throw new TypeError('strict equality requires complete semantic operands');
 if(left.$==='number'){
  if(right.$==='number')return left.value===right.value;
  return false;
 }
 return testJavaScriptSemanticSameValue(left,right);
}
