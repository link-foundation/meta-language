// Generic Construct may already have allocated the actual weak receiver with
// newTarget's prototype. Only its real selected parent/child continuation can
// reuse that receiver; no public flag or strong collection brand grants this.
import {readJavaScriptCollectionConstructionReceiver} from './javascript-collection-construction.js';
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction @returns {{receiver:JavaScriptSemanticValue|undefined,error:string}} */
export function readJavaScriptWeakCollectionConstructionReceiver(program,machine,frame,instruction){
 const receiver=frame.registers[0]??{$:'undefined'};if(instruction.$!=='collectionAllocate'||!['WeakMap','WeakSet'].includes(instruction.kind)||receiver.$!=='object')return {receiver:undefined,error:''};
 return readJavaScriptCollectionConstructionReceiver(program,machine,frame,instruction);
}
