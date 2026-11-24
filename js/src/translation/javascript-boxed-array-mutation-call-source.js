import {compileJavaScriptArrayMutationFunction} from './javascript-array-mutation-call-source.js';
import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
import {readJavaScriptCallCodeDestination} from './javascript-call-code.js';

// Compose existing mutation algorithms with shared ToObject. Register zero stays
// the original Call receiver; the held Object survives every callback/retry.
/** @param {bigint} identity @param {bigint} realm @param {string} operation
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptBoxedArrayMutationFunction(identity,realm,operation,roots){
 const original=compileJavaScriptArrayMutationFunction(identity,realm,operation);
 if(original===undefined)return undefined;
 const held=original.registers;
 /** @type {JavaScriptCallInstruction[]} */const instructions=[];
 if(!appendJavaScriptObjectCoercion(instructions,0,held,held+1,held+2,held+3,held+4,roots))return undefined;
 const offset=instructions.length;
 for(const instruction of original.instructions){
  // Refuse future algorithms that overwrite the immutable original receiver.
  if(readJavaScriptCallCodeDestination(instruction)===0)return undefined;
  const relocated=relocateJavaScriptArrayMutationInstruction(instruction,held,offset);
  if(relocated===undefined)return undefined;
  instructions.push(relocated??instruction);
 }
 return {$:'callFunction',identity:original.identity,realm:original.realm,receiverMode:original.receiverMode,lexicalReceiver:original.lexicalReceiver,parameters:original.parameters,registers:held+5,captures:original.captures,instructions};
}

/** @param {JavaScriptCallInstruction} instruction @param {number} held
 * @param {number} offset @returns {JavaScriptCallInstruction|undefined} */
function relocateJavaScriptArrayMutationInstruction(instruction,held,offset){
 // Control coordinates and argument indices are not register operands. Keep an
 // explicit opcode inventory so a future instruction cannot silently bypass it.
 switch(instruction.$){
  case 'branch':return {$:'branch',condition:replaceJavaScriptBoxedArrayReceiverRegister(instruction.condition,held),yes:instruction.yes+offset,no:instruction.no+offset};
  case 'jump':return {$:'jump',target:instruction.target+offset};
  case 'ordinaryGet':return {$:'ordinaryGet',destination:instruction.destination,object:replaceJavaScriptBoxedArrayReceiverRegister(instruction.object,held),key:replaceJavaScriptBoxedArrayReceiverRegister(instruction.key,held),receiver:replaceJavaScriptBoxedArrayReceiverRegister(instruction.receiver,held)};
  case 'ordinarySet':return {$:'ordinarySet',destination:instruction.destination,object:replaceJavaScriptBoxedArrayReceiverRegister(instruction.object,held),key:replaceJavaScriptBoxedArrayReceiverRegister(instruction.key,held),value:replaceJavaScriptBoxedArrayReceiverRegister(instruction.value,held),receiver:replaceJavaScriptBoxedArrayReceiverRegister(instruction.receiver,held)};
  case 'ordinaryHas':return {$:'ordinaryHas',destination:instruction.destination,object:replaceJavaScriptBoxedArrayReceiverRegister(instruction.object,held),key:replaceJavaScriptBoxedArrayReceiverRegister(instruction.key,held)};
  case 'ordinaryDelete':return {$:'ordinaryDelete',destination:instruction.destination,object:replaceJavaScriptBoxedArrayReceiverRegister(instruction.object,held),key:replaceJavaScriptBoxedArrayReceiverRegister(instruction.key,held)};
  case 'primitive':{
   const operands=[];
   for(const operand of instruction.arguments)operands.push(replaceJavaScriptBoxedArrayReceiverRegister(operand,held));
   return {$:'primitive',destination:instruction.destination,operation:instruction.operation,arguments:operands};
  }
  case 'copy':return {$:'copy',destination:instruction.destination,source:replaceJavaScriptBoxedArrayReceiverRegister(instruction.source,held)};
  case 'return':return {$:'return',source:replaceJavaScriptBoxedArrayReceiverRegister(instruction.source,held)};
  case 'numberBinary':return {$:'numberBinary',destination:instruction.destination,left:replaceJavaScriptBoxedArrayReceiverRegister(instruction.left,held),right:replaceJavaScriptBoxedArrayReceiverRegister(instruction.right,held),operator:instruction.operator};
  case 'strictEqual':return {$:'strictEqual',destination:instruction.destination,left:replaceJavaScriptBoxedArrayReceiverRegister(instruction.left,held),right:replaceJavaScriptBoxedArrayReceiverRegister(instruction.right,held)};
  case 'propertyKey':return {$:'propertyKey',destination:instruction.destination,input:replaceJavaScriptBoxedArrayReceiverRegister(instruction.input,held),realm:instruction.realm};
  case 'requireWrite':return {$:'requireWrite',success:replaceJavaScriptBoxedArrayReceiverRegister(instruction.success,held)};
  case 'argumentAt':return {$:'argumentAt',destination:instruction.destination,index:replaceJavaScriptBoxedArrayReceiverRegister(instruction.index,held)};
  case 'constant':return {$:'constant',destination:instruction.destination,value:instruction.value};
  case 'argument':return {$:'argument',destination:instruction.destination,index:instruction.index};
  case 'argumentCount':return {$:'argumentCount',destination:instruction.destination};
  default:return undefined;
 }
}

/** @param {number} register @param {number} held @returns {number} */
function replaceJavaScriptBoxedArrayReceiverRegister(register,held){
 return register===0?held:register;
}
