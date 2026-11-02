// Opt-in immutable machine snapshots. The public machine validator still checks
// complete frames/programs. An unchanged certified heap skips only its repeated
// whole-directory validation, preserving original public untrusted boundaries.
import {copyJavaScriptImmutableSnapshotData,sealJavaScriptOrdinaryHeapSnapshot} from './javascript-heap-snapshot-authority.js';
import {readJavaScriptCallMachineError,stepJavaScriptCallMachine} from './javascript-call-machine.js';
const snapshots=new WeakSet();
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @returns {{snapshot:object|null,error:string}} */
export function createJavaScriptCallMachineSnapshot(program,machine){
 try{
  const machineData=copyJavaScriptImmutableSnapshotData(machine);if(!machineData||typeof machineData!=='object')return {snapshot:null,error:'immutable machine snapshots require actual complete machine data'};const heap=sealJavaScriptOrdinaryHeapSnapshot(machineData.heap);if(heap.error||heap.heap===null)return {snapshot:null,error:heap.error};
  const ownedProgram=copyJavaScriptImmutableSnapshotData(program),ownedMachine=copyJavaScriptImmutableSnapshotData({...machineData,heap:heap.heap}),error=readJavaScriptCallMachineError(ownedProgram,ownedMachine);if(error!=='')return {snapshot:null,error};
  const snapshot=Object.freeze({$:'immutableCallMachineSnapshot',model:'validated immutable ordinary call snapshots 1',program:ownedProgram,machine:ownedMachine});snapshots.add(snapshot);return {snapshot,error:''};
 }catch(error){return {snapshot:null,error:error instanceof Error?error.message:'immutable machine snapshot creation failed'};}
}
/** Each actual ordinary step remains the same shared execution operation.
 * Continue results publish another independently validated immutable snapshot.
 * Other transitions preserve the original suspended immutable machine for its
 * normal external resumption protocol; resumed states re-enter the full seal.
 * @param {object} snapshot @returns {{transition:JavaScriptCallMachineTransition|null,snapshot:object|null,error:string}} */
export function stepJavaScriptCallMachineSnapshot(snapshot){
 if(!snapshots.has(snapshot))return {transition:null,snapshot:null,error:'immutable stepping requires its actual privately published machine snapshot'};
 const transition=stepJavaScriptCallMachine(snapshot.program,snapshot.machine);if(transition.$!=='machineContinue')return {transition,snapshot:null,error:''};
 const next=createJavaScriptCallMachineSnapshot(snapshot.program,transition.machine);if(next.error||next.snapshot===null)return {transition:null,snapshot:null,error:next.error};return {transition:{$:'machineContinue',machine:next.snapshot.machine},snapshot:next.snapshot,error:''};
}
