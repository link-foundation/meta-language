import {resumeJavaScriptFunctionBindAllocation} from './javascript-function-binding.js';
import {allocateJavaScriptWeakCollection} from './javascript-weak-collections.js';
import {resumeJavaScriptArrayConcatenationAllocation} from './javascript-array-concatenation.js';
import {resumeJavaScriptArraySliceAllocation} from './javascript-array-slice.js';
import {resumeJavaScriptArrayCreationConstruction,resumeJavaScriptArrayCreationAllocation} from './javascript-array-creation.js';
import {resumeJavaScriptConstructionAllocation} from './javascript-construction.js';
import {resumeJavaScriptObjectFromEntriesAllocation} from './javascript-object-from-entries.js';
import {resumeJavaScriptPrimitiveWrapper} from './javascript-primitive-wrappers.js';
import {resumeJavaScriptPropertyDefinitionsAllocation} from './javascript-property-definitions.js';
import {resumeJavaScriptPropertyReflectionAllocation} from './javascript-property-reflection.js';
import {resumeJavaScriptObjectEnumerationAllocation} from './javascript-own-enumeration.js';
import {resumeJavaScriptArgumentRestAllocation} from './javascript-argument-rest.js';
import {allocateJavaScriptStringIterator} from './javascript-string-iteration.js';
import {allocateJavaScriptArrayIterator} from './javascript-array-iteration.js';
import {resumeJavaScriptSpeciesConstructionAllocation} from './javascript-species-construction.js';
import {resumeJavaScriptArrayTransformAllocation} from './javascript-array-transform.js';
import {allocateJavaScriptCallClosure,readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {allocateJavaScriptCollectionIterator} from './javascript-collection-iteration.js';
import {resumeJavaScriptCollectionIteratorResultAllocation} from './javascript-collection-iterator-results.js';
import {insertJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {allocateJavaScriptCollection} from './javascript-collections.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {replaceJavaScriptCallFrame,writeJavaScriptCallRegister,advanceJavaScriptCallFrame,readJavaScriptCallMachineError,requestJavaScriptCallError,acceptJavaScriptCallFunctionBind,stepJavaScriptCallMachine,acceptJavaScriptCollectionCompletion,acceptJavaScriptCollectionIteratorResultCompletion,readJavaScriptCallArrayTransformDirectory,acceptJavaScriptCallArrayConcatenation,acceptJavaScriptCallArraySlice,acceptJavaScriptCallArrayTransform,acceptJavaScriptCallSpeciesConstruction,acceptJavaScriptCallArgumentRest,acceptJavaScriptCallObjectEnumeration,acceptJavaScriptCallPropertyReflection,acceptJavaScriptCallPropertyDefinitions,acceptJavaScriptCallPrimitiveWrapper,acceptJavaScriptCallObjectFromEntries,acceptJavaScriptCallConstruction,acceptJavaScriptCallArrayCreation,acceptJavaScriptCallArrayConstruction} from './javascript-call-machine.js';

/** The global allocator supplies an unused identity; heap maximums do not
 * establish global identity authority. A request leaves its instruction and
 * prototype register unchanged until this explicit resumption advances once.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} identity @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallAllocation(program,machine,identity){
 const error=readJavaScriptCallMachineError(program,machine);
 if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual active allocation instruction'};
 const pending=frame.pending;
 if(pending.$==='waitArrayTransform'){
  const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);
  if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform allocation retains its actual source directory'};
  const construction=pending.construction;
  if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,resumeJavaScriptSpeciesConstructionAllocation(program,machine.heap,directory,construction,identity));
  return acceptJavaScriptCallArrayTransform(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayTransformAllocation(machine.heap,directory,pending.state,identity));
 }
 if(pending.$==='waitArraySlice'){
  const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);
  if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform allocation retains its actual source directory'};
  const construction=pending.construction;
  if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,resumeJavaScriptSpeciesConstructionAllocation(program,machine.heap,directory,construction,identity));
  return acceptJavaScriptCallArraySlice(program,machine,pending.destination,pending.realm,resumeJavaScriptArraySliceAllocation(machine.heap,directory,pending.state,identity));
 }
 if(pending.$==='waitArrayConcatenation'){
  const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);
  if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform allocation retains its actual source directory'};
  const construction=pending.construction;
  if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,resumeJavaScriptSpeciesConstructionAllocation(program,machine.heap,directory,construction,identity));
  return acceptJavaScriptCallArrayConcatenation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationAllocation(machine.heap,directory,pending.state,identity));
 }
 if(pending.$==='waitArgumentRest')return acceptJavaScriptCallArgumentRest(machine,pending.destination,pending.realm,resumeJavaScriptArgumentRestAllocation(machine.heap,frame.arguments,pending.state,identity));
 if(pending.$==='waitArrayCreation'){
  const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return request.$==='machineResourceLimit'?request:{$:'machineInvalid',heap:machine.heap,reason:'Array creation identity response retains its actual allocation request'};
  const construction=pending.construction;
  if(construction!==undefined){const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'}:acceptJavaScriptCallArrayConstruction(program,machine,resumeJavaScriptConstructionAllocation(program,machine.heap,directory,construction,identity));}
  if(pending.fallback!==undefined){const made=allocateJavaScriptArray(machine.heap,identity,pending.fallback.prototype,pending.fallback.length);if(made.$==='arrayNormal')return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayCreationConstruction(made.heap,pending.state,{$:'normalCompletion',value:made.value}));if(made.$==='arrayResourceLimit')return {$:'machineResourceLimit',machine,reason:made.reason};if(made.$==='arrayInvalid')return {$:'machineInvalid',heap:made.heap,reason:made.reason};if(made.$==='arrayError')return {$:'machineInvalid',heap:made.heap,reason:made.reason};return {$:'machineInvalid',heap:made.heap,reason:'ArrayCreate retains its actual allocation result'};}
  return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayCreationAllocation(machine.heap,pending.state,identity));
 }
 if(pending.$==='waitConstruction'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'construction allocation requires actual constructor authority'};const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return {$:'machineInvalid',heap:machine.heap,reason:'construction allocation retains its actual fresh Object request'};return acceptJavaScriptCallConstruction(program,machine,pending.destination,pending.realm,resumeJavaScriptConstructionAllocation(program,machine.heap,directory,pending.state,identity));}
 if(pending.$==='waitFunctionBind')return acceptJavaScriptCallFunctionBind(program,machine,pending.destination,pending.realm,resumeJavaScriptFunctionBindAllocation(machine.heap,pending.state,identity));
 if(pending.$==='waitObjectFromEntries'){const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return request.$==='machineResourceLimit'?request:{$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries Object response retains its actual allocation request'};return acceptJavaScriptCallObjectFromEntries(program,machine,pending.destination,pending.realm,resumeJavaScriptObjectFromEntriesAllocation(machine.heap,pending.state,identity));}
 if(pending.$==='waitPrimitiveWrapper')return acceptJavaScriptCallPrimitiveWrapper(machine,pending.destination,resumeJavaScriptPrimitiveWrapper(machine.heap,pending.state,identity));
 if(pending.$==='waitPropertyDefinitions')return acceptJavaScriptCallPropertyDefinitions(program,machine,pending.destination,pending.realm,resumeJavaScriptPropertyDefinitionsAllocation(machine.heap,pending.state,identity));
 if(pending.$==='waitPropertyReflection')return acceptJavaScriptCallPropertyReflection(machine,pending.destination,pending.realm,resumeJavaScriptPropertyReflectionAllocation(machine.heap,pending.state,identity));
 if(pending.$==='waitObjectEnumeration')return acceptJavaScriptCallObjectEnumeration(program,machine,pending.destination,pending.realm,resumeJavaScriptObjectEnumerationAllocation(machine.heap,pending.state,identity));
 if(pending.$==='waitCollectionIteratorResult')return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,pending.destination,pending.realm,resumeJavaScriptCollectionIteratorResultAllocation(machine.heap,pending.state,identity));
 if(pending.$!=='callIdle')return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual active allocation instruction'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
 if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual source function'};
 const instruction=fn.instructions[frame.position];
 if(instruction===undefined||instruction.$!=='ordinaryAllocate'&&instruction.$!=='arrayAllocate'&&instruction.$!=='collectionAllocate'&&instruction.$!=='collectionIteratorAllocate'&&instruction.$!=='closureAllocate')return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual allocation instruction'};
 if(instruction.$==='closureAllocate'){
  const allocation=allocateJavaScriptCallClosure(program,machine.heap,identity,instruction.owner,instruction.ordinal,instruction.captures.map(index=>frame.registers[index]??{$:'undefined'}),frame.registers[instruction.receiver]??{$:'undefined'},frame.newTarget);
  if(allocation.$==='closureAllocationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$==='closureAllocationInvalid')return {$:'machineInvalid',heap:machine.heap,reason:allocation.reason};
  return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
 }
 const prototype=frame.registers[readJavaScriptCallAllocationPrototype(instruction)]??{$:'undefined'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary allocation requires its actual null or Object prototype'};
 if(machine.heap.objects.length>=4096)return {$:'machineResourceLimit',machine,reason:'ordinary heap identity directory exceeds its explicit resource budget'};
 if(instruction.$==='arrayAllocate'){
  const allocation=allocateJavaScriptArray(machine.heap,identity,prototype,instruction.length);
  if(allocation.$==='arrayResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$!=='arrayNormal')return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='arrayError'?'literal Array length must have been validated before allocation':allocation.$==='arrayInvalid'?allocation.reason:'Array allocation requires its actual normal completion'};
  const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value);return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
 }
 if(instruction.$==='collectionIteratorAllocate'){
  if(instruction.kind==='String'){
   const allocation=allocateJavaScriptStringIterator(machine.heap,identity,prototype,frame.registers[instruction.input??-1]??{$:'undefined'});
   if(allocation.$==='stringIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
   if(allocation.$==='stringIteratorResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
   return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='stringIteratorInvalid'?allocation.reason:'String iterator creation retains its actual allocation result'};
  }
  if(instruction.kind==='Array'){
   const allocation=allocateJavaScriptArrayIterator(machine.heap,identity,prototype,frame.registers[0]??{$:'undefined'},instruction.iterationKind);
   if(allocation.$==='arrayIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
   if(allocation.$==='arrayIteratorAllocationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
   if(allocation.$==='arrayIteratorNeedBoxing')return {$:'machineNeedArrayIteratorBoxing',machine,receiver:allocation.receiver};
   if(allocation.$==='arrayIteratorError')return requestJavaScriptCallError(machine,fn.realm,allocation.errorClass,allocation.reason);
   return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='arrayIteratorInvalid'?allocation.reason:'Array iterator creation retains its actual allocation result'};
  }
  const allocation=allocateJavaScriptCollectionIterator(machine.heap,identity,prototype,frame.registers[0]??{$:'undefined'},instruction.kind,instruction.iterationKind);
  if(allocation.$==='collectionIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
  if(allocation.$==='collectionIterationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$==='collectionIterationTypeError')return requestJavaScriptCallError(machine,fn.realm,'TypeError',allocation.reason);
  return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='collectionIterationInvalid'?allocation.reason:'iterator allocation needs its actual completed creation'};
 }
 if(instruction.$==='collectionAllocate'){
  const allocation=['WeakMap','WeakSet'].includes(instruction.kind)
   ?allocateJavaScriptWeakCollection(machine.heap,identity,prototype,instruction.kind)
   :allocateJavaScriptCollection(machine.heap,identity,prototype,instruction.kind);
  if(allocation.$!=='collectionNormal')return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,allocation);
  const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value);return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
 }
 if(instruction.$==='ordinaryAllocate'){
  const allocation=insertJavaScriptOrdinaryObject(machine.heap,identity,prototype);
  if(!allocation.success)return {$:'machineInvalid',heap:machine.heap,reason:allocation.error};
  const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,{$:'object',identity,callable:false});
  return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
 }
 return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual allocation instruction'};
}

/** @param {JavaScriptCallInstruction} instruction @returns {number} */
function readJavaScriptCallAllocationPrototype(instruction){
 switch(instruction.$){
  case 'ordinaryAllocate':
  case 'arrayAllocate':
  case 'collectionAllocate':
  case 'collectionIteratorAllocate':return instruction.prototype;
  default:return -1;
 }
}
