import {compileJavaScriptArrayMutationFunction} from './javascript-array-mutation-call-source.js';
import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';

// Compose existing mutation algorithms with shared ToObject. Register zero stays
// the original Call receiver; the held Object survives every callback/retry.
/** @param {bigint} identity @param {bigint} realm @param {string} operation
 * @param {import('./javascript-object-call-source.js').JavaScriptPrimitiveWrapperRoots|undefined} roots
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
  if('destination' in instruction&&instruction.destination===0)return undefined;
  const relocated=relocateJavaScriptArrayMutationInstruction(instruction,held,offset);
  if(relocated===undefined)return undefined;
  instructions.push(relocated);
 }
 return {...original,registers:held+5,instructions};
}

/** @param {JavaScriptCallInstruction} instruction @param {number} held
 * @param {number} offset @returns {JavaScriptCallInstruction|undefined} */
function relocateJavaScriptArrayMutationInstruction(instruction,held,offset){
 const register=value=>value===0?held:value;
 // Control coordinates and argument indices are not register operands. Keep an
 // explicit opcode inventory so a future instruction cannot silently bypass it.
 switch(instruction.$){
  case 'branch':return {...instruction,condition:register(instruction.condition),yes:instruction.yes+offset,no:instruction.no+offset};
  case 'jump':return {...instruction,target:instruction.target+offset};
  case 'ordinaryGet':return {...instruction,object:register(instruction.object),key:register(instruction.key),receiver:register(instruction.receiver)};
  case 'ordinarySet':return {...instruction,object:register(instruction.object),key:register(instruction.key),value:register(instruction.value),receiver:register(instruction.receiver)};
  case 'ordinaryHas':case 'ordinaryDelete':return {...instruction,object:register(instruction.object),key:register(instruction.key)};
  case 'primitive':return {...instruction,arguments:instruction.arguments.map(register)};
  case 'copy':case 'return':return {...instruction,source:register(instruction.source)};
  case 'numberBinary':case 'strictEqual':return {...instruction,left:register(instruction.left),right:register(instruction.right)};
  case 'propertyKey':return {...instruction,input:register(instruction.input)};
  case 'requireWrite':return {...instruction,success:register(instruction.success)};
  case 'argumentAt':return {...instruction,index:register(instruction.index)};
  case 'constant':case 'argument':case 'argumentCount':return {...instruction};
  default:return undefined;
 }
}
