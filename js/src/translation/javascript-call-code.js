import {readJavaScriptSemanticValueError,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';

/** Registers are coordinates in checked executable Links data, not language
 * property keys. Coercions belong to explicit operations before typed opcodes.
 * @typedef {{$:'constant',destination:number,value:JavaScriptSemanticValue} |
 * {$:'copy',destination:number,source:number} |
 * {$:'argument',destination:number,index:number} |
 * {$:'argumentCount',destination:number} | {$:'argumentAt',destination:number,index:number} |
 * {$:'pushFinally',target:number} | {$:'finishFinally'} | {$:'completeJump',target:number,depth:number} |
 * {$:'jump',target:number} | {$:'branch',condition:number,yes:number,no:number} |
 * {$:'call',destination:number,callee:number,receiver:number,arguments:number[]} |
 * {$:'argumentListOpen',slot:number} | {$:'argumentListAppend',slot:number,value:number} |
 * {$:'argumentListCall',destination:number,slot:number,callee:number,receiver:number,mode:string} |
 * {$:'constructErrorClass',destination:number,callee:number,arguments:number[]} |
 * {$:'callClassConstructor',destination:number,callee:number,receiver:number,arguments:number[]} |
 * {$:'constructCollection',destination:number,callee:number,arguments:number[]} |
 * {$:'collectionAllocate',destination:number,prototype:number,kind:string} |
 * {$:'collectionIteratorAllocate',destination:number,prototype:number,kind:string,iterationKind:string,input?:number} |
 * {$:'argumentRest',destination:number,prototype:number,from:number,realm:bigint} |
 * {$:'iteratorProtocol',destination:number,done:number,input:number,iteratorKey:number,slot:number,operation:string,realm:bigint,readValue?:boolean} |
 * {$:'collectionIteratorResult',destination:number,kind:string,arrayPrototype:number,objectPrototype:number} |
 * {$:'collectionOperation',destination:number,operation:string} |
 * {$:'primitiveMethod',destination:number,operation:string} |
 * {$:'arrayCallback',destination:number,operation:string} |
 * {$:'arraySearch',destination:number,operation:string} |
 * {$:'arrayTransform',destination:number,operation:string,species:JavaScriptArraySpeciesDirectory} |
 * {$:'objectEnumeration',destination:number,operation:string,prototype:number} |
 * {$:'instanceOf',destination:number,left:number,right:number,key:number,operation:string} |
 * {$:'objectObservation',destination:number,operation:string,input:number,other:number} |
 * {$:'collectionCopy',destination:number,target:number,input:number,adder:number,iteratorKey:number,iteratorMethod:number,iteratorPrototype:number,nextMethod:number} |
 * {$:'constructErrorValue',destination:number,constructor:number,newTarget:number,prototype:number,message:number,present:number,realm:bigint,errorClass:string,bindReceiver:boolean} |
 * {$:'readDerivedReceiver',destination:number} | {$:'invokeErrorIntrinsic',destination:number} | {$:'invokeHostCapability',destination:number} |
 * {$:'return',source:number} | {$:'throw',source:number} |
 * {$:'pushHandler',destination:number,target:number} | {$:'popHandler'} |
 * {$:'numberBinary',destination:number,left:number,right:number,operator:string} |
 * {$:'sameValue',destination:number,left:number,right:number} |
 * {$:'strictEqual',destination:number,left:number,right:number} |
 * {$:'requireWrite',success:number} |
 * {$:'primitive',destination:number,operation:string,arguments:number[]} |
 * {$:'ordinaryAllocate',destination:number,prototype:number} |
 * {$:'literalPrototype',destination:number,object:number,value:number} |
 * {$:'literalAccessor',destination:number,object:number,key:number,value:number,kind:string,realm:bigint} |
 * {$:'functionName',destination:number,object:number,key:number,prefix:string,realm:bigint} |
 * {$:'arrayAllocate',destination:number,prototype:number,length:number} |
 * {$:'closureAllocate',destination:number,owner:bigint,ordinal:number,captures:number[],receiver:number} |
 * {$:'arraySpread',destination:number,target:number,input:number,index:number,iteratorKey:number,realm:bigint} |
 * {$:'objectSpread',destination:number,target:number,input:number,realm:bigint,excluded?:number[]} |
 * {$:'ordinaryDataProperty',destination:number,object:number,key:number,value:number,realm:bigint} |
 * {$:'ordinaryGet',destination:number,object:number,key:number,receiver:number} |
 * {$:'ordinaryDelete',destination:number,object:number,key:number} |
 * {$:'ordinaryHas',destination:number,object:number,key:number} |
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
/** Code-only arrow templates have no heap identity. Their owner is the actual
 * registered source function; ordinals distinguish its nested lexical sites.
 * @typedef {{$:'callTemplate',owner:bigint,ordinal:number,realm:bigint,parameters:number,registers:number,captureCount:number,instructions:JavaScriptCallInstruction[],prototype:JavaScriptSemanticValue,name:JavaScriptSemanticValue,length:number,receiverMode?:string}} JavaScriptCallTemplate
 */
/** Realm identifiers are supplied by the actual intrinsic registry. Membership
 * checks do not prove authority or allocate a substitute realm/Error object.
 * @typedef {{$:'callProgram',functions:JavaScriptCallFunction[],realms:bigint[],exoticKey:JavaScriptSemanticValue,templates?:JavaScriptCallTemplate[]}} JavaScriptCallProgram
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
    case 'constant':case 'argument':case 'argumentCount':case 'pushHandler':case 'readDerivedReceiver':case 'invokeErrorIntrinsic':case 'invokeHostCapability':case 'collectionOperation':case 'primitiveMethod':case 'arrayCallback':case 'arraySearch':case 'arrayTransform':return [instruction.destination];
    case 'argumentAt':return [instruction.destination,instruction.index];
    case 'copy':return [instruction.destination,instruction.source];
    case 'jump':case 'popHandler':case 'pushFinally':case 'finishFinally':case 'completeJump':case 'argumentListOpen':return [];
    case 'argumentListAppend':return [instruction.value];
    case 'argumentListCall':return [instruction.destination,instruction.callee,instruction.receiver];
    case 'branch':return [instruction.condition];
    case 'call':case 'callClassConstructor':return [instruction.destination,instruction.callee,instruction.receiver].concat(instruction.arguments);
    case 'constructErrorClass':case 'constructCollection':return [instruction.destination,instruction.callee].concat(instruction.arguments);
    case 'constructErrorValue':return [instruction.destination,instruction.constructor,instruction.newTarget,instruction.prototype,instruction.message,instruction.present];
    case 'primitive':return [instruction.destination].concat(instruction.arguments);
    case 'return':case 'throw':return [instruction.source];
    case 'requireWrite':return [instruction.success];
    case 'numberBinary':case 'sameValue':case 'strictEqual':return [instruction.destination,instruction.left,instruction.right];
    case 'instanceOf':return [instruction.destination,instruction.left,instruction.right,instruction.key];
    case 'objectObservation':return [instruction.destination,instruction.input,instruction.other];
    case 'ordinaryGet':return [instruction.destination,instruction.object,instruction.key,instruction.receiver];
    case 'ordinaryDelete':case 'ordinaryHas':return [instruction.destination,instruction.object,instruction.key];
    case 'literalPrototype':return [instruction.destination,instruction.object,instruction.value];
    case 'functionName':return [instruction.destination,instruction.object,instruction.key];
    case 'ordinaryAllocate':case 'arrayAllocate':case 'collectionAllocate':case 'argumentRest':case 'objectEnumeration':return [instruction.destination,instruction.prototype];
    case 'collectionIteratorAllocate':return [instruction.destination,instruction.prototype].concat(instruction.input===undefined?[]:[instruction.input]);
    case 'iteratorProtocol':return [instruction.destination,instruction.done,instruction.input,instruction.iteratorKey];
    case 'collectionIteratorResult':return [instruction.destination,instruction.arrayPrototype,instruction.objectPrototype];
    case 'closureAllocate':return [instruction.destination,instruction.receiver].concat(instruction.captures);
    case 'collectionCopy':return [instruction.destination,instruction.target,instruction.input,instruction.adder,instruction.iteratorKey,instruction.iteratorMethod,instruction.iteratorPrototype,instruction.nextMethod];
    case 'ordinaryDataProperty':case 'literalAccessor':return [instruction.destination,instruction.object,instruction.key,instruction.value];
    case 'ordinarySet':return [instruction.destination,instruction.object,instruction.key,instruction.value,instruction.receiver];
    case 'arraySpread':return [instruction.destination,instruction.target,instruction.input,instruction.index,instruction.iteratorKey];
    case 'objectSpread':return [instruction.destination,instruction.target,instruction.input].concat(instruction.excluded??[]);
    case 'propertyKey':return [instruction.destination,instruction.input];
    case 'propertyDefinition':return [instruction.destination,instruction.target,instruction.key,instruction.attributes];
  }
}
/** @param {JavaScriptCallInstruction} instruction @returns {number} */
export function readJavaScriptCallCodeDestination(instruction){switch(instruction.$){case 'jump':case 'branch':case 'popHandler':case 'pushFinally':case 'finishFinally':case 'completeJump':case 'argumentListOpen':case 'argumentListAppend':case 'return':case 'throw':case 'requireWrite':return -1;default:return instruction.destination;}}
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
