// Generic Construct may already have allocated the actual weak receiver with
// newTarget's prototype. Only its real selected parent/child continuation can
// reuse that receiver; no public flag or strong collection brand grants this.
import {findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {readJavaScriptCollectionConstructorKind} from './javascript-collection-call-source.js';
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction @returns {{receiver:JavaScriptSemanticValue|undefined,error:string}} */
export function readJavaScriptWeakCollectionConstructionReceiver(program,machine,frame,instruction){
 const receiver=frame.registers[0]??{$:'undefined'};if(instruction.$!=='collectionAllocate'||!['WeakMap','WeakSet'].includes(instruction.kind)||receiver.$!=='object')return {receiver:undefined,error:''};
 const parent=machine.frames[machine.frames.length-2],pending=parent?.pending,state=pending?.$==='waitConstruction'?pending.state:undefined,fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity),object=machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,receiver.identity)];
 if(!state||state.phase!=='body'||state.mode!==instruction.kind||state.allocationKind!=='weak collection'||state.target.$!=='object'||state.target.identity!==frame.identity||!testJavaScriptSemanticSameValue(state.receiver,receiver)||!testJavaScriptSemanticSameValue(state.newTarget,frame.newTarget??{$:'undefined'})||fn===undefined||readJavaScriptCollectionConstructorKind(fn)!==instruction.kind)return {receiver:undefined,error:'weak initializer reuse requires its actual generic Construct parent, target, receiver and newTarget'};
 if(object===undefined||!(instruction.kind==='WeakMap'&&object.$==='weakMapObject'||instruction.kind==='WeakSet'&&object.$==='weakSetObject')||object.weakEntries.length!==0||!testJavaScriptSemanticSameValue(object.prototype,state.prototype))return {receiver:undefined,error:'weak initializer reuse requires its actual fresh weak internal slots and selected newTarget prototype'};
 return {receiver,error:''};
}
