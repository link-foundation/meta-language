// A generic Construct receiver is reused only by its actual selected iterable
// initializer frame. The existing Map/Set/WeakMap/WeakSet bodies own iteration.
import {findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {readJavaScriptCollectionConstructorKind} from './javascript-collection-call-source.js';
import {readJavaScriptConstructionStateError} from './javascript-construction.js';
import {testJavaScriptConstructedCollectionKind,readJavaScriptConstructedCollectionAllocationKind} from './javascript-collection-construction-rules.js';
import {readJavaScriptPendingConstruction} from './javascript-construction-parents.js';
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction
 * @returns {{receiver:JavaScriptSemanticValue|undefined,error:string}} */
export function readJavaScriptCollectionConstructionReceiver(program,machine,frame,instruction){
 const receiver=frame.registers[0]??{$:'undefined'};
 if(instruction.$!=='collectionAllocate'||!testJavaScriptConstructedCollectionKind(instruction.kind)||receiver.$!=='object')return {receiver:undefined,error:''};
 const parent=machine.frames[machine.frames.length-2],state=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending),fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
 if(state===undefined||state.phase!=='body'||state.mode!==instruction.kind||state.allocationKind!==readJavaScriptConstructedCollectionAllocationKind(instruction.kind)||state.target.$!=='object'||state.target.identity!==frame.identity||!testJavaScriptSemanticSameValue(state.receiver,receiver)||!testJavaScriptSemanticSameValue(state.newTarget,frame.newTarget??{$:'undefined'})||fn===undefined||readJavaScriptCollectionConstructorKind(fn)!==instruction.kind)return {receiver:undefined,error:'collection initializer reuse requires its actual generic Construct parent, target, receiver and newTarget'};
 const directory=program.constructorDirectory;if(directory===undefined)return {receiver:undefined,error:'collection initializer reuse retains its actual constructor authority'};
 const invalid=readJavaScriptConstructionStateError(program,machine.heap,directory,state);if(invalid!=='')return {receiver:undefined,error:invalid};
 const actual=fn.instructions[frame.position];if(actual?.$!=='collectionAllocate'||actual.kind!==instruction.kind||actual.destination!==instruction.destination||actual.prototype!==instruction.prototype||frame.arguments.length!==state.arguments.length||frame.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,state.arguments[index]??{$:'undefined'})))return {receiver:undefined,error:'collection initializer reuse retains its actual selected allocation instruction and complete ordered arguments'};
 const object=machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,receiver.identity)],brand=instruction.kind==='Map'?'mapObject':instruction.kind==='Set'?'setObject':instruction.kind==='WeakMap'?'weakMapObject':'weakSetObject';
 const empty=object!==undefined&&(object.$==='weakMapObject'||object.$==='weakSetObject')?object.weakEntries.length===0:object!==undefined&&(object.$==='mapObject'||object.$==='setObject')?object.entries.length===0:false;
 if(object===undefined||object.$!==brand||!empty||!testJavaScriptSemanticSameValue(object.prototype,state.prototype))return {receiver:undefined,error:'collection initializer reuse requires its actual fresh empty internal slots and selected newTarget prototype'};
 return {receiver,error:''};
}
