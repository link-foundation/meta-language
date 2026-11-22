import {resumeJavaScriptCallAllocation} from './javascript-call-allocation.js';
import {readJavaScriptSelectedParentCallError,readJavaScriptSuspendedConstructorFrameError} from './javascript-call-suspended-parents.js';
import {executeJavaScriptCallInstruction} from './javascript-call-instructions.js';
import {readJavaScriptCallPendingError} from './javascript-call-pending-state.js';
import {readJavaScriptCollectionConstructionReceiver} from './javascript-collection-construction.js';
import {readJavaScriptPendingConstruction} from './javascript-construction-parents.js';
import {readJavaScriptFunctionSourceDirectoryError} from './javascript-function-text.js';
import {stepJavaScriptFunctionBind,resumeJavaScriptFunctionBindCall} from './javascript-function-binding.js';
import {allocateJavaScriptRegisteredSymbol,readJavaScriptSymbolRegistrationFact,retainJavaScriptSymbolRegistration} from './javascript-symbol-registry.js';
import {stepJavaScriptArrayConcatenation,resumeJavaScriptArrayConcatenationCall} from './javascript-array-concatenation.js';
import {stepJavaScriptArraySlice,resumeJavaScriptArraySliceCall} from './javascript-array-slice.js';
import {testJavaScriptArrayCreationOperation,stepJavaScriptArrayCreation,resumeJavaScriptArrayCreationCall,provideJavaScriptArrayCreationPrimitivePrototype,resumeJavaScriptArrayCreationError,throwJavaScriptArrayCreation} from './javascript-array-creation.js';
import {stepJavaScriptConstruction,resumeJavaScriptConstructionCall} from './javascript-construction.js';
import {stepJavaScriptObjectFromEntries,resumeJavaScriptObjectFromEntriesCall,provideJavaScriptObjectFromEntriesPrimitivePrototype,resumeJavaScriptObjectFromEntriesError,throwJavaScriptObjectFromEntries} from './javascript-object-from-entries.js';
import {beginJavaScriptPrimitiveWrapper} from './javascript-primitive-wrappers.js';
import {testJavaScriptCollectionForEachOperation,stepJavaScriptCollectionForEach,resumeJavaScriptCollectionForEach} from './javascript-collection-for-each.js';
import {testJavaScriptPropertyDefinitionsOperation,stepJavaScriptPropertyDefinitions,resumeJavaScriptPropertyDefinitionsCall} from './javascript-property-definitions.js';
import {testJavaScriptPropertyReflectionOperation,stepJavaScriptPropertyReflection} from './javascript-property-reflection.js';
import {stepJavaScriptInstanceOf,resumeJavaScriptInstanceOf} from './javascript-instance-of.js';
import {testJavaScriptObjectObservationOperation,stepJavaScriptObjectIntegrity} from './javascript-object-integrity.js';
import {testJavaScriptObjectEnumerationOperation,stepJavaScriptObjectEnumeration,resumeJavaScriptObjectEnumerationGetter} from './javascript-own-enumeration.js';
import {constructJavaScriptSymbolFunctionName} from './javascript-function-names.js';
import {readJavaScriptCallArgumentListsError,retainJavaScriptCallArgumentLists} from './javascript-call-argument-lists.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {stepJavaScriptArgumentRest} from './javascript-argument-rest.js';
import {readJavaScriptIteratorRegionsError,insertJavaScriptIteratorRegion,updateJavaScriptIteratorRegion,removeJavaScriptIteratorRegion,testJavaScriptIteratorRegionSlot} from './javascript-iterator-regions.js';
import {stepJavaScriptIteratorProtocol,resumeJavaScriptIteratorProtocolCall,provideJavaScriptIteratorPrimitivePrototype} from './javascript-iterator-protocol.js';
import {provideJavaScriptArraySpreadPrimitivePrototype,stepJavaScriptArraySpread,resumeJavaScriptArraySpread} from './javascript-array-spread.js';
import {stepJavaScriptArrayIteratorNext,resumeJavaScriptArrayIteratorCall,finishJavaScriptArrayIteratorCursor} from './javascript-array-iteration.js';
import {stepJavaScriptSpeciesConstruction,resumeJavaScriptSpeciesConstructionCall} from './javascript-species-construction.js';
import {readJavaScriptKnownArrayConstructorFact,classifyJavaScriptArraySpeciesConstructor,readJavaScriptArraySpeciesDirectoryError} from './javascript-array-species.js';
import {testJavaScriptArrayTransformOperation,stepJavaScriptArrayTransform,resumeJavaScriptArrayTransformCall} from './javascript-array-transform.js';
import {testJavaScriptArraySearchOperation,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {stepJavaScriptArraySearch,resumeJavaScriptArraySearch} from './javascript-array-search.js';
import {testJavaScriptArrayCallbackOperation,stepJavaScriptArrayCallback,resumeJavaScriptArrayCallback} from './javascript-array-callbacks.js';
import {stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread} from './javascript-object-spread.js';
import {findJavaScriptCallTemplate,projectJavaScriptCallTemplateFunction,readJavaScriptExecutableCallFunction,readJavaScriptCallTemplateDirectoryError} from './javascript-call-templates.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment} from './javascript-array-assignment.js';
import {readJavaScriptCollectionIteratorReceiverError} from './javascript-collection-iteration.js';
import {stepJavaScriptCollectionIteratorResult} from './javascript-collection-iterator-results.js';
import {testJavaScriptCollectionIteratorIntrinsicMode} from './javascript-collection-iterator-call-source.js';
import {readJavaScriptIntegerProtocolOperationError} from './javascript-call-integer-conversion.js';
import {readJavaScriptTextProtocolOperationError} from './javascript-text-conversion-protocol.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptCallPrimitiveArityError} from './javascript-call-primitive-arity-rules.js';
import {executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';
import {testJavaScriptCallCodeCoordinate,findJavaScriptCallCodeFunction,readJavaScriptCallCodeRegisters,readJavaScriptCallCodeDestination} from './javascript-call-code.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition,deleteJavaScriptOrdinaryProperty,readJavaScriptOrdinaryKeyError,registerJavaScriptOrdinarySymbol} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength} from './javascript-semantic-values.js';
import {readJavaScriptPropertyDefinitionHeap,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition,completeJavaScriptPropertyDefinitionWithSymbol} from './javascript-property-definition.js';
import {stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {readJavaScriptNumberProtocolOperationError} from './javascript-number-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {testJavaScriptErrorConstructorMode,testJavaScriptErrorIntrinsicMode,readJavaScriptErrorConstructionKindError,readJavaScriptErrorConstructionOperandsError,readJavaScriptErrorAllocationDirectory,readJavaScriptErrorAllocationResultError} from './javascript-error-construction-rules.js';
import {testJavaScriptHostCapabilityMode} from './javascript-host-capability-rules.js';
import {testJavaScriptOrdinaryClassConstructorMode,testJavaScriptOrdinaryClassReceiverResult,readJavaScriptOrdinaryClassCallReason,readJavaScriptOrdinaryClassReceiverError} from './javascript-ordinary-class-rules.js';
import {testJavaScriptCollectionConstructorMode,testJavaScriptCollectionIntrinsicMode,testJavaScriptPrimitiveIntrinsicMode,readJavaScriptCollectionConstructorKind} from './javascript-collection-call-source.js';
import {testJavaScriptPrimitiveMethodOperation} from './javascript-primitive-methods.js';
import {readJavaScriptCollectionOperationKind} from './javascript-collection-rules.js';
import {testJavaScriptArrayIntrinsicOperation} from './javascript-arrays.js';
import {testJavaScriptArrayLength} from './javascript-array-objects.js';
import {acceptJavaScriptCallArrayAssignment,acceptJavaScriptCallArrayCallback,acceptJavaScriptCallArraySearch,readJavaScriptCallArrayTransformDirectory,acceptJavaScriptCallArrayConcatenation,acceptJavaScriptCallArraySlice,acceptJavaScriptCallArrayTransform,readJavaScriptPendingSpeciesConstruction,acceptJavaScriptCallSpeciesConstruction,acceptJavaScriptCallArrayIteration,acceptJavaScriptCallArraySpread,requestJavaScriptCallArrayFallback,acceptJavaScriptCallArrayCreation,acceptJavaScriptCallArrayConstruction} from './javascript-call-array-transitions.js';
export {acceptJavaScriptCallArrayAssignment,acceptJavaScriptCallArrayCallback,readJavaScriptSelectedCallFrameError,acceptJavaScriptCallArraySearch,readJavaScriptCallArrayTransformDirectory,acceptJavaScriptCallArrayConcatenation,acceptJavaScriptCallArraySlice,acceptJavaScriptCallArrayTransform,resumeJavaScriptCallArrayConstruction,extendJavaScriptCallArrayAuthority,acceptJavaScriptCallSpeciesConstruction,pushJavaScriptSpeciesConstructorBody,acceptJavaScriptCallArrayIteration,acceptJavaScriptCallArraySpread,requestJavaScriptCallArrayFallback,acceptJavaScriptCallArrayCreation,acceptJavaScriptCallArrayConstruction} from './javascript-call-array-transitions.js';

/** @typedef {{$:'callCompletion',kind:string,value:JavaScriptSemanticValue,target:number,depth:number}} JavaScriptCallCompletion */
/** @typedef {{$:'iteratorOperation',destination:number,done:number,slot:number,operation:string,realm:bigint}} JavaScriptCallIteratorOperation */
/** @typedef {{$:'callHandler',target:number,destination:number} | {$:'callFinallyHandler',target:number} | {$:'callDeferredCompletion',completion:JavaScriptCallCompletion}} JavaScriptCallHandler */
/** @typedef {{$:'callIdle'} | {$:'waitCall',destination:number} |
 * {$:'waitGet',destination:number} | {$:'waitSet',destination:number} | {$:'waitHostSet',destination:number} |
 * {$:'waitArgumentRest',destination:number,realm:bigint,state:JavaScriptArgumentRestState} |
 * {$:'waitIteratorProtocol',destination:number,done:number,slot:number,operation:string,realm:bigint,state:JavaScriptIteratorProtocolState} |
 * {$:'waitArraySpread',destination:number,realm:bigint,state:JavaScriptArraySpreadState} |
 * {$:'waitArrayIteratorNext',destination:number,realm:bigint,state:JavaScriptArrayIteratorState} |
 * {$:'waitCollectionIteratorResult',destination:number,realm:bigint,state:JavaScriptCollectionIteratorResultState} |
 * {$:'waitArrayConcatenation',destination:number,realm:bigint,state:JavaScriptArrayConcatenationState,construction?:JavaScriptSpeciesConstructionState} |
 * {$:'waitArraySlice',destination:number,realm:bigint,state:JavaScriptArraySliceState,construction?:JavaScriptSpeciesConstructionState} |
 * {$:'waitArrayTransform',destination:number,realm:bigint,state:JavaScriptArrayTransformState,construction?:JavaScriptSpeciesConstructionState} |
 * {$:'waitArraySearch',destination:number,realm:bigint,state:JavaScriptArraySearchState} |
 * {$:'waitCollectionForEach',destination:number,realm:bigint,state:JavaScriptCollectionForEachState} |
 * {$:'waitArrayCallback',destination:number,realm:bigint,state:JavaScriptArrayCallbackState} |
 * {$:'waitObjectSpread',destination:number,realm:bigint,state:JavaScriptObjectSpreadState} |
 * {$:'waitObjectEnumeration',destination:number,realm:bigint,state:JavaScriptObjectEnumerationState} |
 * {$:'waitPropertyDefinitions',destination:number,realm:bigint,state:JavaScriptPropertyDefinitionsState} |
 * {$:'waitPropertyReflection',destination:number,realm:bigint,state:JavaScriptPropertyReflectionState} |
 * {$:'waitInstanceOf',destination:number,realm:bigint,state:JavaScriptInstanceOfState} |
 * {$:'waitObjectIntegrity',destination:number,realm:bigint,state:JavaScriptObjectIntegrityState} |
 * {$:'waitObjectPrototype',destination:number,realm:bigint,input:JavaScriptSemanticValue} |
 * {$:'waitArrayAssignment',destination:number,realm:bigint,state:JavaScriptArrayAssignmentState} |
 * {$:'waitKey',destination:number,realm:bigint,state:JavaScriptPrimitiveConversionState} |
 * {$:'waitDefinition',destination:number,realm:bigint,state:JavaScriptPropertyDefinitionState} |
 * {$:'waitNumber',destination:number,realm:bigint,state:JavaScriptNumberCallState} |
 * {$:'waitText',destination:number,realm:bigint,state:JavaScriptTextCallState} |
 * {$:'waitArrayCreation',destination:number,realm:bigint,state:JavaScriptArrayCreationState,construction?:JavaScriptConstructionState,fallback?:JavaScriptArrayCreationFallback,error?:JavaScriptObjectFromEntriesErrorRequest} |
 * {$:'waitConstruction',destination:number,realm:bigint,state:JavaScriptConstructionState,heldArguments?:JavaScriptSemanticValue[]} |
 * {$:'waitFunctionBind',destination:number,realm:bigint,state:JavaScriptFunctionBindState} |
 * {$:'waitFunctionText',destination:number,realm:bigint,input:JavaScriptSemanticValue,kind:string} |
 * {$:'waitConstructorCheck',destination:number,input:JavaScriptSemanticValue,required:boolean} |
 * {$:'waitObjectFromEntries',destination:number,realm:bigint,state:JavaScriptObjectFromEntriesState,error?:JavaScriptObjectFromEntriesErrorRequest} |
 * {$:'waitPrimitiveWrapper',destination:number,state:JavaScriptPrimitiveWrapperState} |
 * {$:'waitSymbolAllocation',destination:number,description:JavaScriptSemanticValue,registryKey?:JavaScriptSemanticValue} |
 * {$:'waitSymbolDescription',destination:number,symbol:JavaScriptSemanticValue} |
 * {$:'waitFunctionNameSymbol',destination:number,object:JavaScriptSemanticValue,key:JavaScriptSemanticValue,prefix:string,realm:bigint} |
 * {$:'waitSourceError',realm:bigint,errorClass:string,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'waitErrorValue',destination:number,realm:bigint,errorClass:string,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,message:JavaScriptSemanticValue,messagePresent:boolean,bindReceiver:boolean,beforeDirectory:JavaScriptErrorAllocationDirectory} |
 * {$:'waitErrorIntrinsic',destination:number} | {$:'waitHostCapability',destination:number,beforeDirectory:JavaScriptErrorAllocationDirectory} |
 * {$:'waitError',realm:bigint,errorClass:string,reason:string}} JavaScriptCallPending
 */
/** @typedef {{$:'objectFromEntriesErrorRequest',realm:bigint,errorClass:string,reason:string}} JavaScriptObjectFromEntriesErrorRequest */
/** @typedef {{$:'callFrame',identity:bigint,position:number,registers:JavaScriptSemanticValue[],
 * arguments:JavaScriptSemanticValue[],handlers:JavaScriptCallHandler[],pending:JavaScriptCallPending,iterators?:JavaScriptCallIteratorRegion[],argumentLists?:JavaScriptCallArgumentList[],activeJoin?:JavaScriptSemanticValue,newTarget?:JavaScriptSemanticValue}} JavaScriptCallFrame
 */
/** @typedef {{$:'callMachine',heap:JavaScriptOrdinaryHeap,frames:JavaScriptCallFrame[]}} JavaScriptCallMachine */
/** @typedef {{$:'machineContinue',machine:JavaScriptCallMachine} |
 * {$:'machineNeedObjectPrototype',machine:JavaScriptCallMachine,input:JavaScriptSemanticValue,realm:bigint} |
 * {$:'machineAllocateSymbol',machine:JavaScriptCallMachine,description:JavaScriptSemanticValue,registryKey?:JavaScriptSemanticValue} |
 * {$:'machineNeedSymbolDescription',machine:JavaScriptCallMachine,symbol:JavaScriptSemanticValue} |
 * {$:'machineNeedFunctionNameSymbolDescription',machine:JavaScriptCallMachine,object:JavaScriptSemanticValue,key:JavaScriptSemanticValue,prefix:string} |
 * {$:'machineNeedFunctionText',machine:JavaScriptCallMachine,input:JavaScriptSemanticValue,kind:string,realm:bigint} |
 * {$:'machineCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineAllocateObject',machine:JavaScriptCallMachine,prototype:JavaScriptSemanticValue} |
 * {$:'machineAllocateError',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,reason:string} |
 * {$:'machineAllocateSourceError',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'machineAllocateErrorValue',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'machineInvokeErrorIntrinsic',machine:JavaScriptCallMachine,realm:bigint,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'machineInvokeHostCapability',machine:JavaScriptCallMachine,realm:bigint,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'machineConstruct',machine:JavaScriptCallMachine,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],operation:string,reason:string} |
 * {$:'machineConstructArraySpecies',machine:JavaScriptCallMachine,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fact:JavaScriptArrayConstructorFact,newTarget?:JavaScriptSemanticValue,operation?:string,reason?:string} |
 * {$:'machineNeedPrimitiveIteratorGet',machine:JavaScriptCallMachine,receiver:JavaScriptSemanticValue,key:JavaScriptSemanticValue} |
 * {$:'machineNeedArrayIteratorBoxing',machine:JavaScriptCallMachine,receiver:JavaScriptSemanticValue} |
 * {$:'machineNeedConstructorAuthority',machine:JavaScriptCallMachine,constructor:JavaScriptSemanticValue} |
 * {$:'machineNeedCollectionIterable',machine:JavaScriptCallMachine,target:JavaScriptSemanticValue,input:JavaScriptSemanticValue} |
 * {$:'machineInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'machineResourceLimit',machine:JavaScriptCallMachine,reason:string}} JavaScriptCallMachineTransition
 */
/**
 * @typedef {{$:'machineContinue',machine:JavaScriptCallMachine} |
 * {$:'machineThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'machineResourceLimit',machine:JavaScriptCallMachine,reason:string} |
 * {$:'callReturn',machine:JavaScriptCallMachine,value:JavaScriptSemanticValue}} JavaScriptCallCompletionTransition
 */
/** @typedef {{$:'callExecution',transition:JavaScriptCallMachineTransition,steps:number,objectIdentitiesUsed:number}} JavaScriptCallExecution */

/** Bounded, pure execution over the same machine. Identities are supplied by
 * the external allocator and consumed only by successful object resumptions.
 * An unavailable identity suspends at the allocation request; intrinsic Error
 * allocations retain their existing authority-specific resumption protocol.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} limit @param {bigint[]} objectIdentities
 * @returns {JavaScriptCallExecution} */
export function runJavaScriptCallMachine(program,machine,limit,objectIdentities){
 if(!testJavaScriptCallCoordinate(limit,1048577)||objectIdentities.length>4096)return {$:'callExecution',transition:{$:'machineInvalid',heap:machine.heap,reason:'execution requires bounded exact step and allocation budgets'},steps:0,objectIdentitiesUsed:0};
 const error=readJavaScriptCallMachineError(program,machine);
 if(error!=='')return {$:'callExecution',transition:{$:'machineInvalid',heap:machine.heap,reason:error},steps:0,objectIdentitiesUsed:0};
 let current=readJavaScriptCallExecutionTransition({$:'machineContinue',machine});
 let steps=0,used=0;
 while(steps<limit){
  const active=readJavaScriptCallExecutionTransition(current);
  if(active.$==='machineContinue')current=stepJavaScriptCallMachine(program,active.machine);
  else if(active.$==='machineAllocateObject'){
   const identity=objectIdentities[used];
   if(identity===undefined)return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
   const next=resumeJavaScriptCallObjectAllocation(program,active.machine,identity);
   if(next.$==='machineContinue')used+=1;
   current=next;
  }else return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
  steps+=1;
 }
 const final=readJavaScriptCallExecutionTransition(current);
 if(final.$==='machineContinue')return {$:'callExecution',transition:{$:'machineResourceLimit',machine:final.machine,reason:'execution exhausted its explicit transition budget'},steps,objectIdentitiesUsed:used};
 return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
}

/** @param {JavaScriptCallMachineTransition} transition @returns {JavaScriptCallMachineTransition} */
function readJavaScriptCallExecutionTransition(transition){return transition;}

/** @param {number} value @param {number} limit @returns {boolean} */
export function testJavaScriptCallCoordinate(value,limit){return testJavaScriptCallCodeCoordinate(value,limit);}
/** @param {JavaScriptCallProgram} program @param {bigint} identity @returns {number} */
export function findJavaScriptCallFunction(program,identity){return findJavaScriptCallCodeFunction(program,identity);}
/** @param {JavaScriptCallInstruction} instruction @returns {number[]} */
export function readJavaScriptCallRegisters(instruction){return readJavaScriptCallCodeRegisters(instruction);}

/** Validate actual code/identity facts, without executing any instruction.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @returns {string} */
export function readJavaScriptCallProgramError(program,heap){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
  const sourceError=readJavaScriptFunctionSourceDirectoryError(program);if(sourceError!=='')return sourceError;
  if(program.constructorDirectory!==undefined){const invalid=readJavaScriptArraySpeciesDirectoryError(heap,program.constructorDirectory);if(invalid!=='')return invalid;}
  if(program.functions.length>4096||program.realms.length>4096)return 'call code directories exceed their explicit resource budget';
  let previous=0n;
  for(const realm of program.realms){if(realm<=previous||realm>18446744073709551615n)return 'call realms require ordered unique positive unsigned64 identities';previous=realm;}
  if(program.exoticKey.$!=='symbol')return 'call code requires the authoritative well-known primitive conversion Symbol';
  const keyError=readJavaScriptOrdinaryValueError(heap,program.exoticKey);if(keyError!=='')return keyError;
  previous=0n;let count=0;
  for(const fn of program.functions){
    if(fn.identity<=previous||fn.identity>18446744073709551615n)return 'call functions require ordered unique positive unsigned64 identities';previous=fn.identity;
    count+=fn.instructions.length;if(count>65536)return 'call code exceeds its explicit total instruction budget';
    const bodyError=readJavaScriptCallFunctionBodyError(program,heap,fn);if(bodyError!=='')return bodyError;
  }
  const templateError=readJavaScriptCallTemplateDirectoryError(program,heap);if(templateError!=='')return templateError;
  for(const template of program.templates??[]){
    count+=template.instructions.length;if(count>65536)return 'static functions and templates share one total instruction budget';
    const body=projectJavaScriptCallTemplateFunction(template,template.owner,[],{$:'undefined'});
    const bodyError=readJavaScriptCallFunctionBodyError(program,heap,body);if(bodyError!=='')return bodyError;
  }
  for(const object of heap.objects)if(object.$==='boundFunctionObject'){const fn=readJavaScriptExecutableCallFunction(program,heap,object.identity);if(fn!==undefined){const error=readJavaScriptCallFunctionBodyError(program,heap,fn);if(error!=='')return error;}}
  return '';
}

/** Validate one body without inventing a callable identity for static templates.
 * Source ownership and actual captures are checked by their outer directories.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFunction} fn @returns {string} */
export function readJavaScriptCallFunctionBodyError(program,heap,fn){
    if(!program.realms.includes(fn.realm))return 'call functions require their actual declared source realm';
    const callable=readJavaScriptOrdinaryValueError(heap,{$:'object',identity:fn.identity,callable:true});if(callable!=='')return callable;
    if(fn.receiverMode!=='strict'&&fn.receiverMode!=='lexical'&&!testJavaScriptErrorConstructorMode(fn.receiverMode)&&!testJavaScriptErrorIntrinsicMode(fn.receiverMode)&&!testJavaScriptHostCapabilityMode(fn.receiverMode)&&!testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)&&!testJavaScriptCollectionConstructorMode(fn.receiverMode)&&!testJavaScriptCollectionIntrinsicMode(fn.receiverMode)&&!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)&&!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode))return 'call code requires explicit strict, lexical or registered Error receiver binding';
    if(fn.lexicalNewTarget!==undefined&&(fn.receiverMode!=='lexical'||fn.lexicalNewTarget.$!=='object'||!fn.lexicalNewTarget.callable||readJavaScriptOrdinaryValueError(heap,fn.lexicalNewTarget)!==''))return 'lexical functions retain their actual captured newTarget';
    if(readJavaScriptOrdinaryValueError(heap,fn.lexicalReceiver)!=='')return 'lexical receivers belong to the latest global context';
    if(testJavaScriptCollectionConstructorMode(fn.receiverMode)&&readJavaScriptCollectionConstructorKind(fn)==='')return 'collection constructors retain their actual single standard allocation kind';
    if(!testJavaScriptCallCoordinate(fn.registers,4097)||fn.registers===0||!testJavaScriptCallCoordinate(fn.parameters,fn.registers)||fn.parameters+fn.captures.length+1>fn.registers)return 'call function register and parameter coordinates must be complete';
    for(const capture of fn.captures)if(readJavaScriptOrdinaryValueError(heap,capture)!=='')return 'call captures belong to the latest global context';
    if(fn.instructions.length>65536||fn.instructions.length===0)return 'call code exceeds its explicit instruction budget or has no entry';
    for(const instruction of fn.instructions){
      if(testJavaScriptErrorConstructorMode(fn.receiverMode)&&readJavaScriptCallCodeDestination(instruction)===0)return 'derived this is initialized only by its actual super construction';
      if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)&&readJavaScriptCallCodeDestination(instruction)===0)return 'ordinary class this retains its actual allocated receiver';
      if((testJavaScriptCollectionConstructorMode(fn.receiverMode)||testJavaScriptCollectionIntrinsicMode(fn.receiverMode)||testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode))&&readJavaScriptCallCodeDestination(instruction)===0)return 'standard intrinsic frames retain their actual receiver register';
      if(instruction.$==='objectSpread'&&(instruction.excluded??[]).length>4096)return 'object copy exclusions retain their bounded actual register operands';
      for(const register of readJavaScriptCallRegisters(instruction))if(!testJavaScriptCallCoordinate(register,fn.registers))return 'call instructions require actual in-range register coordinates';
      switch(instruction.$){
        case 'functionBind':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode))return 'binding retains its actual protected intrinsic source owner';break;
        case 'functionToString':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode))return 'function text retains its actual protected intrinsic source owner';break;
        case 'boundFunctionCall':{const actual=heap.objects[findJavaScriptOrdinaryObject(heap.objects,fn.identity)];if(actual?.$!=='boundFunctionObject'||fn.instructions.length!==2||fn.instructions[0]?.$!=='boundFunctionCall'||fn.instructions[1]?.$!=='return'||instruction.destination!==1||fn.instructions[1].source!==1||fn.registers!==2||fn.parameters!==0||fn.captures.length!==0)return 'bound Function forwarding retains its actual runtime slots and exact shared code';break;}
        case 'arrayJoinEnter':case 'arrayJoinLeave':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode))return 'join recursion instructions retain their protected intrinsic owner';break;
        case 'constructorCheck':if(typeof instruction.required!=='boolean')return 'constructor checks retain their actual required or predicate operation';break;
        case 'completeJump':if(!Number.isSafeInteger(instruction.depth)||instruction.depth<0||instruction.depth>1024||Object.is(instruction.depth,-0)||!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'completion jumps retain their actual bounded handler depth and target';break;
        case 'jump':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call jumps require actual instruction targets';break;
        case 'branch':if(!testJavaScriptCallCoordinate(instruction.yes,fn.instructions.length)||!testJavaScriptCallCoordinate(instruction.no,fn.instructions.length))return 'call branches require both actual instruction targets';break;
        case 'pushHandler':case 'pushFinally':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call handlers require actual instruction targets';break;
        case 'argument':if(!testJavaScriptCallCoordinate(instruction.index,1048576))return 'call argument reads require exact bounded coordinates';break;
        case 'constant':if(readJavaScriptOrdinaryValueError(heap,instruction.value)!=='')return 'call constants belong to the latest global context';break;
        case 'primitive':{const error=readJavaScriptCallPrimitiveArityError(instruction.operation,instruction.arguments.length);if(error!=='')return error;break;}
        case 'numberBinary':if(!['add','subtract','multiply','divide','remainder','power','lessThan','equal'].includes(instruction.operator))return 'call number instructions require an actual registered operation';break;
        case 'propertyKey':if(!program.realms.includes(instruction.realm))return 'a property key operation retains its actual intrinsic realm';break;
        case 'propertyDefinition':if(!program.realms.includes(instruction.realm)||instruction.operation!==undefined&&instruction.operation!=='Object.defineProperty'&&instruction.operation!=='Reflect.defineProperty')return 'a property definition retains its actual intrinsic realm and Object or Reflect policy';break;
        case 'ordinaryDefine':if(!program.realms.includes(instruction.realm)||instruction.fields.length!==12)return 'internal property definitions retain their actual realm and twelve descriptor operands';break;
        case 'arrayConcatenation':{if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||instruction.species.realm!==fn.realm)return 'Array concatenation retains actual protected realm species authority';const invalid=readJavaScriptArraySpeciesDirectoryError(heap,instruction.species);if(invalid!=='')return invalid;break;}
        case 'arraySlice':{if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||instruction.species.realm!==fn.realm)return 'Array slice retains actual protected realm species authority';const invalid=readJavaScriptArraySpeciesDirectoryError(heap,instruction.species);if(invalid!=='')return invalid;break;}
        case 'arrayTransform':{if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArrayTransformOperation(instruction.operation)||instruction.species.realm!==fn.realm)return 'Array transformations retain their actual standard owner, operation and species realm';const error=readJavaScriptArraySpeciesDirectoryError(heap,instruction.species);if(error!=='')return error;break;}
        case 'arraySearch':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArraySearchOperation(instruction.operation))return 'Array search instructions retain their actual standard callable owner and operation';break;
        case 'objectFromEntries':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode))return 'Object.fromEntries retains its actual protected intrinsic frame';break;
        case 'collectionForEach':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptCollectionForEachOperation(instruction.operation))return 'collection callbacks retain their actual protected operation owner';break;
        case 'arrayCreation':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArrayCreationOperation(instruction.operation))return 'Array creation retains its actual protected intrinsic operation';break;
        case 'arrayCallback':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArrayCallbackOperation(instruction.operation))return 'Array callback instructions retain their actual standard owner and operation';break;
        case 'arraySpread':if(instruction.realm!==fn.realm)return 'Array spread retains its actual active source realm';break;
        case 'ordinaryDataProperty':if(!program.realms.includes(instruction.realm))return 'a data property definition retains its actual intrinsic realm';break;
        case 'objectEnumeration':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptObjectEnumerationOperation(instruction.operation))return 'Object enumeration retains its actual protected operation owner';break;
        case 'objectSpread':if(!program.realms.includes(instruction.realm)||instruction.operation!==undefined&&(instruction.operation!=='assignment'||!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||(instruction.excluded??[]).length!==0))return 'Object.assign retains its protected Set body and complete source keys';break;
        case 'propertyDefinitions':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptPropertyDefinitionsOperation(instruction.operation))return 'bulk property definitions retain their actual protected operation owner';break;
        case 'propertyReflection':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptPropertyReflectionOperation(instruction.operation))return 'property reflection retains its actual protected operation owner';break;
        case 'instanceOf':if(!['instanceof','ordinary has instance','prototype contains'].includes(instruction.operation)||instruction.operation!=='instanceof'&&!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode))return 'instanceof retains its actual operator or protected intrinsic body';break;
        case 'objectObservation':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptObjectObservationOperation(instruction.operation))return 'Object observations retain their actual protected operation owner';break;
        case 'literalAccessor':if(!program.realms.includes(instruction.realm)||!['get','set'].includes(instruction.kind))return 'literal accessors retain actual getter/setter kind and realm';break;
        case 'functionName':if(instruction.realm!==fn.realm||!['','get','set'].includes(instruction.prefix))return 'function naming retains its actual source realm and prefix';break;
        case 'constructErrorValue':{if(!program.realms.includes(instruction.realm))return 'Error value construction retains its actual intrinsic realm';const error=readJavaScriptErrorConstructionKindError(instruction.errorClass,instruction.bindReceiver,fn.receiverMode);if(error!=='')return error;break;}
        case 'readDerivedReceiver':if(!testJavaScriptErrorConstructorMode(fn.receiverMode))return 'derived receiver reads retain their actual constructor source owner';break;
        case 'invokeErrorIntrinsic':if(!testJavaScriptErrorIntrinsicMode(fn.receiverMode))return 'host Error intrinsic calls require an externally registered intrinsic owner';break;
        case 'invokeHostCapability':if(!testJavaScriptHostCapabilityMode(fn.receiverMode))return 'host capability calls require their explicitly qualified external source owner';break;
        case 'collectionIteratorAllocate':if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||!['Map','Set','Array','String'].includes(instruction.kind)||!['keys','values','entries'].includes(instruction.iterationKind)||instruction.kind==='String'&&(instruction.iterationKind!=='values'||instruction.input===undefined)||instruction.kind!=='String'&&instruction.input!==undefined)return 'iterator allocation retains its actual standard operation kind and owner';break;
        case 'argumentRest':if(instruction.realm!==fn.realm||!testJavaScriptCallCoordinate(instruction.from,1048577)||instruction.destination!==instruction.from+1||instruction.destination>fn.parameters)return 'argument rest retains its actual formal parameter coordinate and source realm';break;
        case 'iteratorProtocol':if(!['acquire','step','close'].includes(instruction.operation)||instruction.realm!==fn.realm||!testJavaScriptIteratorRegionSlot(instruction.slot)||instruction.destination===0||instruction.done===0||instruction.destination===instruction.done||instruction.readValue!==undefined&&(instruction.operation!=='step'||typeof instruction.readValue!=='boolean'))return 'iterator protocols retain actual source regions, realms and distinct private result coordinates';break;
        case 'argumentListOpen':case 'argumentListAppend':if(!testJavaScriptIteratorRegionSlot(instruction.slot))return 'Call argument Lists retain actual source slots and operation kinds';break;
        case 'argumentListCall':if(!testJavaScriptIteratorRegionSlot(instruction.slot)||!['call','collection','error class','ordinary class','construct'].includes(instruction.mode))return 'Call argument Lists retain actual source slots and operation kinds';break;
        case 'collectionIteratorResult':if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||!['Map','Set','Array','String'].includes(instruction.kind))return 'iterator next retains its actual standard operation kind and owner';break;
        case 'collectionAllocate':if(!testJavaScriptCollectionConstructorMode(fn.receiverMode)||!['Map','Set','Array','String','WeakMap','WeakSet'].includes(instruction.kind))return 'collection allocation retains its actual standard constructor kind and frame';break;
        case 'collectionCopy':if(!testJavaScriptCollectionConstructorMode(fn.receiverMode))return 'collection copying retains its actual standard constructor frame';break;
        case 'symbolRegistry':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!['for','keyFor'].includes(instruction.operation))return 'Symbol registry work retains its actual protected standard method owner';break;
        case 'collectionOperation':if(!testJavaScriptCollectionIntrinsicMode(fn.receiverMode)||readJavaScriptCollectionOperationKind(instruction.operation)==='')return 'collection operations retain their actual registered intrinsic frame';break;
        case 'primitiveMethod':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptPrimitiveMethodOperation(instruction.operation)&&!testJavaScriptArrayIntrinsicOperation(instruction.operation))return 'primitive methods retain their actual registered intrinsic frame';break;
        case 'closureAllocate':{
          if(testJavaScriptErrorConstructorMode(fn.receiverMode))return 'derived this closures require their shared lexical initialization cell';
          const template=(program.templates??[])[findJavaScriptCallTemplate(program,instruction.owner,instruction.ordinal)];
          if(template===undefined||template.realm!==fn.realm||template.captureCount!==instruction.captures.length)return 'closure allocation retains its actual lexical template, realm and capture vector';
          break;
        }
        case 'arrayAllocate':if(!testJavaScriptArrayLength(instruction.length))return 'Array literals retain their actual unsigned32 sparse length';break;
        default:break;
      }
    }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallFrame[]} frames @param {JavaScriptCallFrame} frame @returns {JavaScriptCallMachine} */
export function replaceJavaScriptCallFrame(heap,frames,frame){return {$:'callMachine',heap,frames:frames.map((entry,index)=>index===frames.length-1?frame:entry)};}
/** @param {JavaScriptCallFrame} frame @param {number} destination @param {JavaScriptSemanticValue} value @returns {JavaScriptCallFrame} */
export function writeJavaScriptCallRegister(frame,destination,value){return {...frame,registers:frame.registers.map((entry,index)=>index===destination?value:entry),pending:{$:'callIdle'}};}
/** Reuse one complete frame update for all suspended operation variants.
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallPending} pending
 * @returns {JavaScriptCallFrame} */
export function replaceJavaScriptCallPending(frame,pending){return {...frame,pending};}
/** @param {JavaScriptCallFrame} frame @returns {JavaScriptCallFrame} */
export function advanceJavaScriptCallFrame(frame){return {...frame,position:frame.position+1};}
/** @param {JavaScriptCallFunction} fn @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallFrame} */
export function createJavaScriptCallFrame(fn,receiver,arguments_){
  let registers=[];
  for(let index=0;index<fn.registers;index+=1)registers.push({$:'undefined'});
  const bound=testJavaScriptErrorConstructorMode(fn.receiverMode)?{$:'undefined'}:fn.receiverMode==='lexical'?fn.lexicalReceiver:receiver;
  registers=registers.map((entry,index)=>index===0?bound:entry);
  for(let index=0;index<fn.parameters;index+=1){const value=arguments_[index]??{$:'undefined'},slot=index+1;registers=registers.map((entry,position)=>position===slot?value:entry);}
  for(let index=0;index<fn.captures.length;index+=1){const value=fn.captures[index]??{$:'undefined'},slot=fn.parameters+1+index;registers=registers.map((entry,position)=>position===slot?value:entry);}
  return {$:'callFrame',identity:fn.identity,position:0,registers,arguments:arguments_,handlers:[],pending:{$:'callIdle'},...(fn.receiverMode==='lexical'&&fn.lexicalNewTarget!==undefined?{newTarget:fn.lexicalNewTarget}:{})};
}
/** Recover conversion policy from the actual suspended instruction; a state
 * cannot substitute a different registered operation, destination or realm.
 * @param {JavaScriptCallFunction} fn @param {JavaScriptCallFrame} frame
 * @param {number} destination @param {bigint} realm @param {JavaScriptSemanticValue} exoticKey
 * @param {JavaScriptNumberCallState} state @returns {string} */
export function readJavaScriptNumberCallSourceOperation(fn,frame,destination,realm,exoticKey,state){
 if(frame.position===0||fn.realm!==realm)return '';
 const instruction=fn.instructions[frame.position-1];if(instruction===undefined)return '';
 switch(instruction.$){
  case 'primitive':{
   if(instruction.destination!==destination||instruction.arguments.length!==1)return '';
   if(readJavaScriptNumberProtocolOperationError(instruction.operation)!=='')return '';
   const input=frame.registers[instruction.arguments[0]?? -1]??{$:'undefined'};
   if(input.$!=='object')return '';
   switch(state.$){
    case 'numberProtocolState':{
     if(state.state.hint!==(instruction.operation==='convert primitive value'?'default':'number'))return '';
     if(!testJavaScriptSemanticSameValue(state.state.receiver,input)||!testJavaScriptSemanticSameValue(state.state.exoticKey,exoticKey))return '';
     break;
    }
    case 'numberPrimitiveValue':break;
   }
   return instruction.operation;
  }
  default:return '';
 }
}
/** Recover conversion policy from the actual suspended instruction; a state
 * cannot substitute a different registered operation, destination or realm.
 * @param {JavaScriptCallFunction} fn @param {JavaScriptCallFrame} frame
 * @param {number} destination @param {bigint} realm @param {JavaScriptSemanticValue} exoticKey
 * @param {JavaScriptTextCallState} state @returns {string} */
export function readJavaScriptTextCallSourceOperation(fn,frame,destination,realm,exoticKey,state){
 if(frame.position===0||fn.realm!==realm)return '';
 const instruction=fn.instructions[frame.position-1];if(instruction===undefined)return '';
 switch(instruction.$){
  case 'primitive':{
   if(instruction.destination!==destination||instruction.arguments.length!==1)return '';
   if(readJavaScriptTextProtocolOperationError(instruction.operation)!=='')return '';
   const input=frame.registers[instruction.arguments[0]?? -1]??{$:'undefined'};
   if(input.$!=='object')return '';
   switch(state.$){
    case 'textProtocolState':{
     if(!testJavaScriptSemanticSameValue(state.state.receiver,input)||!testJavaScriptSemanticSameValue(state.state.exoticKey,exoticKey))return '';
     break;
    }
    case 'textPrimitiveValue':break;
   }
   return instruction.operation;
  }
  default:return '';
 }
}
/** Validate resumable machine state, never treat it as an execution certificate.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @returns {string} */
export function readJavaScriptCallMachineError(program,machine){
  const error=readJavaScriptCallProgramError(program,machine.heap);if(error!=='')return error;
  if(machine.frames.length===0||machine.frames.length>1024)return 'a call machine requires its complete bounded active stack';
  let slots=0,argumentSlots=0;
  for(let index=0;index<machine.frames.length;index+=1){
    const frame=machine.frames[index];if(frame===undefined)return 'call stacks must be dense';
    const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return 'every call frame retains its actual registered function';
    if(frame.registers.length!==fn.registers||!testJavaScriptCallCoordinate(frame.position,fn.instructions.length+1))return 'call frames retain their complete register and instruction coordinates';
    slots+=frame.registers.length;argumentSlots+=frame.arguments.length;for(const list of frame.argumentLists??[])argumentSlots+=list.values.length;if(slots>1048576||argumentSlots>1048576||frame.handlers.length>1024)return 'call frame storage exceeds its explicit resource budget';
    const frameError=readJavaScriptCallFrameError(program,machine.heap,frame,fn,index<machine.frames.length-1);
    if(frameError!=='')return frameError;
    if(frame.activeJoin!==undefined&&machine.frames.slice(0,index).some(parent=>parent.activeJoin!==undefined&&readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity)?.realm===fn.realm&&testJavaScriptSemanticSameValue(parent.activeJoin,frame.activeJoin)))return 'join activations in the same realm cannot own the same receiver guard';
    const selectedParent=machine.frames[index-1];
    if(selectedParent!==undefined){
      const selectedError=readJavaScriptSelectedParentCallError(program,machine.heap,selectedParent,fn,frame);
      if(selectedError!=='')return selectedError;
    }
    if(frame.newTarget!==undefined&&fn.receiverMode!=='lexical'){
      const parent=machine.frames[index-1],caller=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity),instruction=caller===undefined||parent===undefined||parent.position===0?undefined:caller.instructions[parent.position-1];
      const generic=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending),species=parent===undefined?undefined:readJavaScriptPendingSpeciesConstruction(parent.pending);
      if(generic!==undefined){if(generic.call?.constructing!==true||!testJavaScriptSemanticSameValue(generic.newTarget,frame.newTarget)||generic.target.$!=='object'||generic.target.identity!==frame.identity)return 'new.target retains its actual selected base construction';}
      else if(species!==undefined&&species.phase==='body'){if(!testJavaScriptSemanticSameValue(species.constructor,frame.newTarget))return 'species new.target retains its actual constructor';}
      else{if(instruction===undefined)return 'an ordinary Call cannot invent a construction new.target';const legacy=instruction.$==='callClassConstructor'||instruction.$==='constructCollection'||instruction.$==='constructErrorClass'||instruction.$==='argumentListCall'&&['ordinary class','collection','error class'].includes(instruction.mode??'');if(!legacy||parent===undefined||!testJavaScriptSemanticSameValue(parent.registers[instruction.callee]??{$:'undefined'},frame.newTarget))return 'an ordinary Call cannot invent a construction new.target';}
    }
    if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||testJavaScriptCollectionConstructorMode(fn.receiverMode)){
      const parent=machine.frames[index-1],caller=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity),instruction=parent===undefined||caller===undefined||parent.position===0?undefined:caller.instructions[parent.position-1];
      const generic=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending);
      if(generic!==undefined){if(!(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||readJavaScriptCollectionConstructorKind(fn)===generic.mode)||generic.phase!=='body'||generic.call?.constructing!==true||generic.target.$!=='object'||generic.target.identity!==frame.identity||!testJavaScriptSemanticSameValue(generic.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(generic.newTarget,frame.newTarget??{$:'undefined'}))return 'a base class or collection retains its actual generic Construct call';}
      else if(parent!==undefined&&(parent.pending.$==='waitArrayTransform'||parent.pending.$==='waitArraySlice'||parent.pending.$==='waitArrayConcatenation')&&parent.pending.construction!==undefined){
        const state=parent.pending.construction;if(!testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||state.phase!=='body'||state.constructor.$!=='object'||state.constructor.identity!==frame.identity||!testJavaScriptSemanticSameValue(state.receiver,frame.registers[0]??{$:'undefined'})||frame.arguments.length!==1||!testJavaScriptSemanticSameValue(frame.arguments[0]??{$:'undefined'},{$:'number',value:state.length}))return 'species source class construction retains its actual pending new operation';
      }else{
        if(parent===undefined||parent.pending.$!=='waitCall'||instruction===undefined)return 'a constructor frame retains its actual suspended source new operation';
        const error=readJavaScriptSuspendedConstructorFrameError(parent,frame,fn,instruction,parent.pending.destination);
        if(error!=='')return error;
      }
    }
  }
  return '';
}

/** Validate all destination-bearing variants before their independent state checks.
 * Keeping this decision separate avoids copying later continuations per variant.
 * @param {JavaScriptCallPending} pending @param {number} registers @returns {boolean} */
export function testJavaScriptCallPendingDestination(pending,registers){
 switch(pending.$){
  case 'waitPropertyDefinitions':case 'waitPropertyReflection':case 'waitInstanceOf':case 'waitObjectPrototype':case 'waitObjectIntegrity':case 'waitObjectEnumeration':case 'waitArrayCreation':case 'waitConstruction':case 'waitFunctionBind':case 'waitFunctionText':case 'waitConstructorCheck':case 'waitObjectFromEntries':case 'waitPrimitiveWrapper':case 'waitSymbolAllocation':case 'waitSymbolDescription':case 'waitFunctionNameSymbol':case 'waitCall':case 'waitGet':case 'waitSet':case 'waitHostSet':case 'waitKey':case 'waitDefinition':case 'waitNumber':case 'waitText':case 'waitErrorValue':case 'waitErrorIntrinsic':case 'waitHostCapability':case 'waitCollectionIteratorResult':case 'waitArrayIteratorNext':case 'waitArraySpread':case 'waitIteratorProtocol':case 'waitArgumentRest':case 'waitArrayAssignment':case 'waitObjectSpread':case 'waitCollectionForEach':case 'waitArrayCallback':case 'waitArraySearch':case 'waitArraySlice':case 'waitArrayConcatenation':case 'waitArrayTransform':
   return testJavaScriptCallCoordinate(pending.destination,registers);
  case 'callIdle':case 'waitError':case 'waitSourceError':return true;
 }
}
/** Complete per-frame validation, shared outside the stack-accounting loop.
 * Caller retains coordinate/storage checks before this actual frame traversal.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallFunction} fn
 * @param {boolean} suspended @returns {string} */
export function readJavaScriptCallFrameError(program,heap,frame,fn,suspended){
    for(const value of frame.registers)if(readJavaScriptOrdinaryValueError(heap,value)!=='')return 'call registers belong to the latest global context';
    for(const value of frame.arguments)if(readJavaScriptOrdinaryValueError(heap,value)!=='')return 'call arguments belong to the latest global context';
    const closure=heap.objects[findJavaScriptOrdinaryObject(heap.objects,frame.identity)];
    if(closure!==undefined&&closure.$==='closureObject'){
      if(fn.receiverMode==='lexical'&&!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},closure.lexicalReceiver))return 'an arrow frame retains its actual lexical receiver';
      for(let index=0;index<closure.captures.length;index++)if(!testJavaScriptSemanticSameValue(frame.registers[fn.parameters+1+index]??{$:'undefined'},closure.captures[index]??{$:'undefined'}))return 'an arrow frame retains its actual immutable captured values';
    }
    if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)){const receiver=frame.registers[0]??{$:'undefined'},object=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;if(receiver.$!=='object'||receiver.callable||object?.$!=='ordinaryObject')return 'an active base constructor retains its actual allocated ordinary receiver';}
    const iteratorError=readJavaScriptIteratorRegionsError(heap,frame.iterators??[]);if(iteratorError!=='')return iteratorError;
    const argumentListError=readJavaScriptCallArgumentListsError(heap,frame.argumentLists??[],frame.handlers.length);if(argumentListError!=='')return argumentListError;
    for(const handler of frame.handlers){
      if(handler.$==='callHandler'){if(!testJavaScriptCallCoordinate(handler.destination,fn.registers)||!testJavaScriptCallCoordinate(handler.target,fn.instructions.length))return 'call handlers retain their complete register and instruction coordinates';}
      else if(handler.$==='callFinallyHandler'){if(!testJavaScriptCallCoordinate(handler.target,fn.instructions.length))return 'finalizers retain their actual source instruction entry';}
      else if(handler.$==='callDeferredCompletion'){const completion=handler.completion;if(!['return','throw','jump'].includes(completion.kind)||readJavaScriptOrdinaryValueError(heap,completion.value)!==''||!Number.isSafeInteger(completion.depth)||completion.depth<0||completion.depth>1024||Object.is(completion.depth,-0)||completion.kind==='jump'&&!testJavaScriptCallCoordinate(completion.target,fn.instructions.length)||completion.kind!=='jump'&&(completion.target!==-1||completion.depth!==0))return 'deferred completions retain actual values, kinds and bounded control coordinates';}
      else return 'handlers retain their actual registered catch, finalizer or deferred completion kind';
    }
    const joinEntries=fn.instructions.filter(instruction=>instruction.$==='arrayJoinEnter'),joinEntry=joinEntries[0],joinEnter=fn.instructions.findIndex(instruction=>instruction.$==='arrayJoinEnter'),joinLeave=fn.instructions.findIndex(instruction=>instruction.$==='arrayJoinLeave');
    const entered=joinEntry===undefined||joinEntry.$!=='arrayJoinEnter'?undefined:frame.registers[joinEntry.destination];
    const ownsJoin=frame.position>joinEnter&&frame.position<=joinLeave&&entered!==undefined&&entered.$==='boolean'&&entered.value;
    if(ownsJoin&&frame.activeJoin===undefined)return 'a successful unfinished join cannot lose its owned receiver guard';
    if(frame.activeJoin!==undefined){
      const joined=frame.activeJoin;if(joined.$!=='object'||readJavaScriptOrdinaryValueError(heap,joined)!=='')return 'an active join guard retains its actual receiver Object';
      if(joinEntries.length!==1||joinEntry?.$!=='arrayJoinEnter'||!ownsJoin||!testJavaScriptSemanticSameValue(frame.registers[joinEntry.receiver]??{$:'undefined'},joined))return 'an active join guard retains its selected receiver and successful unfinished enter instruction';
    }
    if(frame.newTarget!==undefined&&(frame.newTarget.$!=='object'||!frame.newTarget.callable||readJavaScriptOrdinaryValueError(heap,frame.newTarget)!==''))return 'new.target retains its actual constructor identity';
    if(frame.newTarget!==undefined&&program.constructorDirectory!==undefined){const actual=classifyJavaScriptArraySpeciesConstructor(heap,program.constructorDirectory,frame.newTarget);if(readJavaScriptKnownArrayConstructorFact(actual)?.constructable!==true)return 'new.target retains its independently registered constructor classification';}
    if(fn.receiverMode==='lexical'&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},fn.lexicalNewTarget??{$:'undefined'}))return 'an arrow frame retains its captured lexical new.target';
    const pending=frame.pending;
    if(!testJavaScriptCallPendingDestination(pending,fn.registers))return 'pending calls retain their actual destination register';
    const pendingError=readJavaScriptCallPendingError(program,heap,frame,fn,suspended,pending);
    if(pendingError!=='')return pendingError;
    if(suspended&&(pending.$==='waitConstructorCheck'||pending.$==='waitPrimitiveWrapper'||pending.$==='waitSymbolAllocation'||pending.$==='waitSymbolDescription'||pending.$==='waitFunctionNameSymbol'||pending.$==='callIdle'||pending.$==='waitError'||pending.$==='waitSourceError'||pending.$==='waitErrorValue'||pending.$==='waitErrorIntrinsic'||pending.$==='waitHostCapability'))return 'a suspended parent frame retains its actual pending call';
  return '';
}


/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptNumberCallState} state @returns {string} */
export function readJavaScriptNumberCallStateError(heap,state){
 switch(state.$){
  case 'numberProtocolState':{
   if(state.state.hint!=='number'&&state.state.hint!=='default')return 'Number conversion retains its actual numeric primitive hint';
   return readJavaScriptPrimitiveConversionStateError(heap,state.state);
  }
  case 'numberPrimitiveValue':{
   if(readJavaScriptOrdinaryValueError(heap,state.value)!=='')return 'a completed number conversion retains its actual primitive value';
   if(state.value.$==='object')return 'a completed number conversion retains its actual primitive value';
   return '';
  }
 }
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptTextCallState} state @returns {string} */
export function readJavaScriptTextCallStateError(heap,state){
 switch(state.$){
  case 'textProtocolState':{
   if(state.state.hint!=='string')return 'Text conversion retains its actual string primitive hint';
   return readJavaScriptPrimitiveConversionStateError(heap,state.state);
  }
  case 'textPrimitiveValue':{
   if(readJavaScriptOrdinaryValueError(heap,state.value)!=='')return 'a completed text conversion retains its actual primitive value';
   if(state.value.$==='object')return 'a completed text conversion retains its actual primitive value';
   return '';
  }
 }
}

/** @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function throwJavaScriptCallValue(machine,value){
 const transition=unwindJavaScriptCallCompletion(machine,{$:'callCompletion',kind:'throw',value,target:-1,depth:0});return projectJavaScriptCallCompletionTransition(transition);
}
/** @param {JavaScriptCallMachine} machine @param {JavaScriptCallCompletion} completion @returns {JavaScriptCallCompletionTransition} */
export function unwindJavaScriptCallCompletion(machine,completion){
 if(!['return','throw','jump'].includes(completion.kind)||readJavaScriptOrdinaryValueError(machine.heap,completion.value)!==''||!Number.isSafeInteger(completion.depth)||Object.is(completion.depth,-0)||completion.depth<0||completion.depth>1024||completion.kind==='jump'&&(!Number.isSafeInteger(completion.target)||completion.target<0||completion.target>=65536||Object.is(completion.target,-0))||completion.kind!=='jump'&&(completion.target!==-1||completion.depth!==0))return {$:'machineInvalid',heap:machine.heap,reason:'completion unwinding requires actual values and exact bounded control coordinates'};
 let frames=machine.frames,heap=machine.heap;
 while(frames.length>0){
  let frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'completion unwinding retains its complete actual stack'};
  if(completion.kind==='throw'){
   const pending=frame.pending;
   switch(pending.$){
    case 'waitArrayCreation':{
     if(pending.state.call===undefined||pending.construction!==undefined)break;
     const resumed=resumeJavaScriptArrayCreationCall(heap,pending.state,{$:'throwCompletion',value:completion.value});
     switch(resumed.$){
      case 'arrayCreationThrow':heap=resumed.heap;frame={...frame,pending:{$:'callIdle'}};frames=frames.slice(0,-1).concat([frame]);completion={...completion,value:resumed.value};break;
      case 'arrayCreationInvalid':return {$:'machineInvalid',heap:resumed.heap,reason:resumed.reason};
      case 'arrayCreationContinue':case 'arrayCreationCall':case 'arrayCreationNeedError':case 'arrayCreationAllocate':case 'arrayCreationPrimitiveGet':case 'arrayCreationConstruct':case 'arrayCreationResourceLimit':{
       const next=replaceJavaScriptCallFrame(resumed.heap,frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCreation',destination:pending.destination,realm:pending.realm,state:resumed.state}));
       return resumed.$==='arrayCreationResourceLimit'?{$:'machineResourceLimit',machine:next,reason:resumed.reason}:{$:'machineContinue',machine:next};
      }
      default:return {$:'machineInvalid',heap:resumed.heap,reason:'Array creation throw retains actual cleanup or final completion'};
     }
     break;
    }
    case 'waitObjectFromEntries':{
     if(pending.state.call===undefined)break;
     const resumed=resumeJavaScriptObjectFromEntriesCall(heap,pending.state,{$:'throwCompletion',value:completion.value});
     switch(resumed.$){
      case 'objectFromEntriesThrow':heap=resumed.heap;frame={...frame,pending:{$:'callIdle'}};frames=frames.slice(0,-1).concat([frame]);completion={...completion,value:resumed.value};break;
      case 'objectFromEntriesInvalid':return {$:'machineInvalid',heap:resumed.heap,reason:resumed.reason};
      case 'objectFromEntriesContinue':case 'objectFromEntriesCall':case 'objectFromEntriesNeedError':case 'objectFromEntriesAllocate':case 'objectFromEntriesPrimitiveGet':case 'objectFromEntriesResourceLimit':{
       const next=replaceJavaScriptCallFrame(resumed.heap,frames,replaceJavaScriptCallPending(frame,{$:'waitObjectFromEntries',destination:pending.destination,realm:pending.realm,state:resumed.state}));
       return resumed.$==='objectFromEntriesResourceLimit'?{$:'machineResourceLimit',machine:next,reason:resumed.reason}:{$:'machineContinue',machine:next};
      }
      default:return {$:'machineInvalid',heap:resumed.heap,reason:'Object.fromEntries throw resumes its actual cleanup or final abrupt completion'};
     }
     break;
    }
    case 'waitIteratorProtocol':{
     if(pending.state.call===undefined)break;
     const resolved=resumeJavaScriptCallIteratorThrow({$:'callMachine',heap,frames},completion.value);if(resolved.$==='iteratorThrowTransition'){
      const transition=resolved.transition;
      return transition.$==='machineInvalid'?{$:'machineInvalid',heap:transition.heap,reason:transition.reason}:{$:'machineContinue',machine:transition.machine};
     }frames=resolved.machine.frames;heap=resolved.machine.heap;completion={...completion,value:resolved.value};frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'iterator throw retains its actual caller frame'};
     break;
    }
   }
  }
  let handlers=frame.handlers;
  if(completion.kind==='jump'&&completion.depth>handlers.length)return {$:'machineInvalid',heap,reason:'completion jump cannot invent an enclosing handler depth'};
  while(handlers.length>(completion.kind==='jump'?completion.depth:0)){
   const handler=handlers[handlers.length-1];handlers=handlers.slice(0,-1);if(handler===undefined)return {$:'machineInvalid',heap,reason:'completion handler storage remains dense'};
   if(handler.$==='callFinallyHandler')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...frame,argumentLists:retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),position:handler.target,pending:{$:'callIdle'},handlers:handlers.concat([{$:'callDeferredCompletion',completion:{...completion}}])})};
   if(handler.$==='callHandler'){if(completion.kind==='throw')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...writeJavaScriptCallRegister(frame,handler.destination,completion.value),argumentLists:retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),position:handler.target,handlers})};}
  }
  const argumentLists=retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),current=replaceJavaScriptCallFrame(heap,frames,{...frame,handlers,argumentLists});
  if(completion.kind==='jump')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...frame,position:completion.target,pending:{$:'callIdle'},handlers,argumentLists})};
  if(completion.kind==='return')return {$:'callReturn',machine:current,value:completion.value};
  const finalPending=frame.pending;
  if(finalPending.$==='waitArrayIteratorNext')heap=finishJavaScriptArrayIteratorCursor(heap,finalPending.state,true);
  frames=frames.slice(0,-1);
 }
 return completion.kind==='throw'?{$:'machineThrow',heap,value:completion.value}:{$:'machineInvalid',heap,reason:'return/control completion requires its actual active frame'};
}
/** @param {JavaScriptCallMachine} machine @param {bigint} realm @param {string} errorClass @param {string} reason @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallError(machine,realm,errorClass,reason){
  let frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error allocation requires its actual active frame'};
  const initialPending=frame.pending;
  switch(initialPending.$){
   case 'waitArrayCreation':{
    if(initialPending.construction!==undefined||initialPending.state.call===undefined)break;
    const fallback=initialPending.fallback,error={$:'objectFromEntriesErrorRequest',realm,errorClass,reason};
    const pending=fallback===undefined
     ?{$:'waitArrayCreation',destination:initialPending.destination,realm:initialPending.realm,state:initialPending.state,error}
     :{$:'waitArrayCreation',destination:initialPending.destination,realm:initialPending.realm,state:initialPending.state,error,fallback};
    const next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,pending));return {$:'machineAllocateError',machine:next,realm,errorClass,reason};
   }
   case 'waitObjectFromEntries':{
    if(initialPending.state.call===undefined)break;
    const pending={$:'waitObjectFromEntries',destination:initialPending.destination,realm:initialPending.realm,state:initialPending.state,error:{$:'objectFromEntriesErrorRequest',realm,errorClass,reason}};
    const next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,pending));return {$:'machineAllocateError',machine:next,realm,errorClass,reason};
   }
   case 'waitIteratorProtocol':{
    const pending=initialPending,regions=frame.iterators??[],update=pending.operation==='close'?removeJavaScriptIteratorRegion(machine.heap,regions,pending.slot):pending.operation==='step'?updateJavaScriptIteratorRegion(machine.heap,regions,pending.slot,{...pending.state.record,done:true}):{$:'iteratorRegionsUpdated',regions};
    if(update.$!=='iteratorRegionsUpdated')return {$:'machineInvalid',heap:machine.heap,reason:'iterator intrinsic failure retains its actual source region'};
    frame={...frame,iterators:update.regions,pending:{$:'callIdle'}};machine=replaceJavaScriptCallFrame(machine.heap,machine.frames,frame);
    if(pending.operation==='close'&&pending.state.completion.kind==='throw')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallIteratorResult(frame,pending.destination,pending.done,{$:'undefined'},true))};
    break;
   }
  }
  const currentPending=frame.pending;
  const heap=currentPending.$==='waitArrayIteratorNext'?finishJavaScriptArrayIteratorCursor(machine.heap,currentPending.state,true):machine.heap;
  const next=replaceJavaScriptCallFrame(heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitError',realm,errorClass,reason}));
  return {$:'machineAllocateError',machine:next,realm,errorClass,reason};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptCall(program,machine,callee,receiver,arguments_){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a nested call needs its actual suspended parent'};
  const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call frame needs its registered source owner'};
  if(callee.$!=='object')return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected callee is not callable');
  if(!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected callee is not callable');
  const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'the actual callable needs its registered executable definition'};
  const setup=readJavaScriptCallConstructorFailure(machine.heap,fn);if(setup!==undefined)return requestJavaScriptCallError(machine,setup.realm,setup.errorClass,setup.reason);
  if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'an ordinary class call needs its actual named source declaration'};
  let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;for(const list of active.argumentLists??[])argumentSlots+=list.values.length;}
  if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'call stack storage exceeds the selected execution resource budget'};
  return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,receiver,arguments_)])}};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptFunctionBindTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallFunctionBind(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'binding retains its actual intrinsic frame'};
 switch(transition.$){
  case 'functionBindContinue':case 'functionBindCall':case 'functionBindAllocate':case 'functionBindResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitFunctionBind',destination,realm,state:transition.state}));
   if(transition.$==='functionBindCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   if(transition.$==='functionBindAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   return transition.$==='functionBindResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'functionBindCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'functionBindThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'functionBindError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'functionBindInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {JavaScriptSemanticValue} object @param {bigint} realm
 * @param {JavaScriptSemanticValue} name @returns {JavaScriptCallMachineTransition} */
export function finishJavaScriptCallFunctionName(program,machine,destination,object,realm,name){
 const entry=object.$==='object'?machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,object.identity)]:undefined;
 if(object.$!=='object'||!object.callable||entry?.$!=='closureObject')return {$:'machineInvalid',heap:machine.heap,reason:'source function naming retains its actual newly allocated runtime callable'};
 const descriptor=createJavaScriptPropertyDescriptor('data',true,false);descriptor.value=name;descriptor.writable=false;
 return acceptJavaScriptCallDefinition(program,machine,destination,realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,object,{$:'text',units:[110,97,109,101]},descriptor,program.exoticKey));
}
/** Actual embedding metadata supplies only the selected Symbol description.
 * No selected key conversion, method body or property Get is re-executed.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} description @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallFunctionNameSymbolDescription(program,machine,description){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Symbol function naming requires its actual suspended source operation'};
 const pending=frame.pending;if(pending.$!=='waitFunctionNameSymbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol function naming requires its actual suspended source operation'};
 const named=constructJavaScriptSymbolFunctionName(description,pending.prefix);if(named.$==='functionNameInvalid')return {$:'machineInvalid',heap:machine.heap,reason:named.reason};if(named.$==='functionNameResourceLimit')return {$:'machineResourceLimit',machine,reason:named.reason};if(named.$!=='functionNameValue')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol name metadata must complete actual text assembly'};
 return finishJavaScriptCallFunctionName(program,machine,pending.destination,pending.object,pending.realm,named.value);
}

/** Independently supplied native/source metadata responds only to its actual
 * suspended receiver/category. Text itself grants no execution authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallFunctionText(program,machine,value){
 const error=readJavaScriptCallMachineError(program,machine)||readJavaScriptOrdinaryValueError(machine.heap,value);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitFunctionText'||value.$!=='text'&&value.$!=='sourceText'||readJavaScriptSemanticTextLength(value)<=0)return {$:'machineInvalid',heap:machine.heap,reason:'function text metadata resumes only its actual suspended request with complete nonempty text'};
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,frame.pending.destination,value))};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptPropertyKeyConversionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallKey(program,machine,destination,realm,transition){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'key conversion needs its actual active frame'};
  switch(transition.$){
    case 'propertyKeyContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitKey',destination,realm,state:transition.state}))};
    case 'propertyKeyCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitKey',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
    case 'propertyKeyCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.key))};
    case 'propertyKeyThrow':return throwJavaScriptCallValue(machine,transition.value);
    case 'propertyKeyIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
    case 'propertyKeyInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
  }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptPropertyDefinitionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallDefinition(program,machine,destination,realm,transition){
  const heap=readJavaScriptPropertyDefinitionHeap(transition,machine.heap),current={$:'callMachine',heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'property definition needs its actual active frame'};
  switch(transition.$){
    case 'definitionContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitDefinition',destination,realm,state:transition.state}))};
    case 'definitionCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitDefinition',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
    case 'definitionCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
    case 'definitionThrow':return throwJavaScriptCallValue(current,transition.value);
    case 'definitionIntrinsicError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
    case 'definitionInvalid':return {$:'machineInvalid',heap,reason:transition.reason};
    case 'definitionResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitDefinition',destination,realm,state:transition.state})),reason:transition.reason};
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallNumber(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'number conversion needs its actual active frame'};
 switch(transition.$){
  case 'scalarContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitNumber',destination,realm,state:{$:'numberProtocolState',state:transition.state}}))};
  case 'scalarCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitNumber',destination,realm,state:{$:'numberProtocolState',state:transition.state}})),transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'scalarThrow':return throwJavaScriptCallValue(machine,transition.value);
  case 'scalarIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
  case 'scalarInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitNumber',destination,realm,state:{$:'numberPrimitiveValue',value:transition.value}})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptNumberCallState} state @returns {JavaScriptCallMachineTransition} */
export function stepJavaScriptCallNumber(program,machine,destination,realm,state){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'number conversion needs its actual active frame'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'number conversion needs its actual source function'};
 const operation=readJavaScriptNumberCallSourceOperation(fn,frame,destination,realm,program.exoticKey,state);
 if(operation==='')return {$:'machineInvalid',heap:machine.heap,reason:'pending number conversion retains its actual source instruction, receiver, Symbol and realm'};
 switch(state.$){
  case 'numberProtocolState':return acceptJavaScriptCallNumber(program,machine,destination,realm,stepJavaScriptScalarProtocol(machine.heap,state.state,operation));
  case 'numberPrimitiveValue':return acceptJavaScriptCallNumber(program,machine,destination,realm,finishJavaScriptScalarPrimitive(operation,state.value));
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptNumberCallState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallNumber(program,machine,destination,realm,state,value){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'number conversion needs its actual active frame'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'number conversion needs its actual source function'};
 const operation=readJavaScriptNumberCallSourceOperation(fn,frame,destination,realm,program.exoticKey,state);
 if(operation==='')return {$:'machineInvalid',heap:machine.heap,reason:'pending number conversion retains its actual source instruction, receiver, Symbol and realm'};
 switch(state.$){
  case 'numberProtocolState':return acceptJavaScriptCallNumber(program,machine,destination,realm,resumeJavaScriptScalarProtocol(machine.heap,state.state,{$:'normalCompletion',value},operation));
  case 'numberPrimitiveValue':return {$:'machineInvalid',heap:machine.heap,reason:'a completed numeric primitive cannot have a suspended child call'};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallText(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'text conversion needs its actual active frame'};
 switch(transition.$){
  case 'scalarContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitText',destination,realm,state:{$:'textProtocolState',state:transition.state}}))};
  case 'scalarCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitText',destination,realm,state:{$:'textProtocolState',state:transition.state}})),transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'scalarThrow':return throwJavaScriptCallValue(machine,transition.value);
  case 'scalarIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
  case 'scalarInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitText',destination,realm,state:{$:'textPrimitiveValue',value:transition.value}})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptTextCallState} state @returns {JavaScriptCallMachineTransition} */
export function stepJavaScriptCallText(program,machine,destination,realm,state){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'text conversion needs its actual active frame'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'text conversion needs its actual source function'};
 const operation=readJavaScriptTextCallSourceOperation(fn,frame,destination,realm,program.exoticKey,state);
 if(operation==='')return {$:'machineInvalid',heap:machine.heap,reason:'pending text conversion retains its actual source instruction, receiver, Symbol and realm'};
 switch(state.$){
  case 'textProtocolState':return acceptJavaScriptCallText(program,machine,destination,realm,stepJavaScriptScalarProtocol(machine.heap,state.state,operation));
  case 'textPrimitiveValue':return acceptJavaScriptCallText(program,machine,destination,realm,finishJavaScriptScalarPrimitive(operation,state.value));
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {bigint} realm @param {JavaScriptTextCallState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallText(program,machine,destination,realm,state,value){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'text conversion needs its actual active frame'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'text conversion needs its actual source function'};
 const operation=readJavaScriptTextCallSourceOperation(fn,frame,destination,realm,program.exoticKey,state);
 if(operation==='')return {$:'machineInvalid',heap:machine.heap,reason:'pending text conversion retains its actual source instruction, receiver, Symbol and realm'};
 switch(state.$){
  case 'textProtocolState':return acceptJavaScriptCallText(program,machine,destination,realm,resumeJavaScriptScalarProtocol(machine.heap,state.state,{$:'normalCompletion',value},operation));
  case 'textPrimitiveValue':return {$:'machineInvalid',heap:machine.heap,reason:'a completed string primitive cannot have a suspended child call'};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function returnJavaScriptCallValue(program,machine,value){
 const transition=unwindJavaScriptCallCompletion(machine,{$:'callCompletion',kind:'return',value,target:-1,depth:0});return acceptJavaScriptCallCompletion(program,transition);
}
/** Normalize constructor returns only after all active finalizers complete.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function finishJavaScriptCallReturn(program,machine,value){
  const active=machine.frames[machine.frames.length-1],owner=active===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,active.identity);
  if(active!==undefined&&owner!==undefined&&testJavaScriptOrdinaryClassReceiverResult(owner.receiverMode,value.$)){const receiver=active.registers[0]??{$:'undefined'};if(receiver.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'an ordinary class completion needs its actual allocated receiver'};value=receiver;}
  if(active!==undefined&&owner!==undefined&&testJavaScriptErrorConstructorMode(owner.receiverMode)){
    if(value.$==='undefined'){const receiver=active.registers[0]??{$:'undefined'};if(receiver.$==='undefined')return requestJavaScriptCallError(machine,owner.realm,'ReferenceError','a derived constructor must initialize this through its actual super call');value=receiver;}
    if(value.$!=='object')return requestJavaScriptCallError(machine,owner.realm,'TypeError','a derived constructor cannot return a primitive value');
  }
  const frames=machine.frames.slice(0,-1),parent=frames[frames.length-1];
  if(parent===undefined)return {$:'machineCompleted',heap:machine.heap,value};
  const current={$:'callMachine',heap:machine.heap,frames},pending=parent.pending;
  switch(pending.$){
    case 'waitFunctionBind':return acceptJavaScriptCallFunctionBind(program,current,pending.destination,pending.realm,resumeJavaScriptFunctionBindCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitCall':case 'waitGet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,value))};
    case 'waitSet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,{$:'boolean',value:true}))};
    case 'waitHostSet':if(value.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'an opaque Error Set completion retains its actual Boolean result'};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,value))};
    case 'waitKey':return acceptJavaScriptCallKey(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyKeyConversion(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitDefinition':return acceptJavaScriptCallDefinition(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyDefinition(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitIteratorProtocol':return acceptJavaScriptCallIteratorProtocol(program,current,projectJavaScriptCallIteratorOperation(pending),resumeJavaScriptIteratorProtocolCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArraySpread':return acceptJavaScriptCallArraySpread(program,current,pending.destination,pending.realm,resumeJavaScriptArraySpread(machine.heap,pending.state,{$:'normalCompletion',value},false));
    case 'waitArrayIteratorNext':return acceptJavaScriptCallArrayIteration(program,current,resumeJavaScriptArrayIteratorCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayConcatenation':{const construction=pending.construction,directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array concatenation return retains actual species authority'};return construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,construction,{$:'normalCompletion',value})):acceptJavaScriptCallArrayConcatenation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArraySlice':{const construction=pending.construction,directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array slice return retains actual species authority'};return construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,construction,{$:'normalCompletion',value})):acceptJavaScriptCallArraySlice(program,current,pending.destination,pending.realm,resumeJavaScriptArraySliceCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArrayTransform':{const construction=pending.construction,directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform return retains its actual source directory'};return construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,construction,{$:'normalCompletion',value})):acceptJavaScriptCallArrayTransform(program,current,pending.destination,pending.realm,resumeJavaScriptArrayTransformCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArraySearch':return acceptJavaScriptCallArraySearch(program,current,pending.destination,pending.realm,resumeJavaScriptArraySearch(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayCreation':{const construction=pending.construction;if(construction!==undefined){const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'}:acceptJavaScriptCallArrayConstruction(program,current,resumeJavaScriptConstructionCall(program,machine.heap,directory,construction,{$:'normalCompletion',value}));}return acceptJavaScriptCallArrayCreation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'normalCompletion',value}));}
    case 'waitConstruction':{const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'construction requires actual constructor authority'}:acceptJavaScriptCallConstruction(program,current,pending.destination,pending.realm,resumeJavaScriptConstructionCall(program,machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitObjectFromEntries':return acceptJavaScriptCallObjectFromEntries(program,current,pending.destination,pending.realm,resumeJavaScriptObjectFromEntriesCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitCollectionForEach':return acceptJavaScriptCallCollectionForEach(program,current,pending.destination,pending.realm,resumeJavaScriptCollectionForEach(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayCallback':return acceptJavaScriptCallArrayCallback(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCallback(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitObjectSpread':return acceptJavaScriptCallObjectSpread(program,current,pending.destination,pending.realm,resumeJavaScriptObjectSpread(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitPropertyDefinitions':return acceptJavaScriptCallPropertyDefinitions(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyDefinitionsCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitInstanceOf':return acceptJavaScriptCallInstanceOf(program,current,pending.destination,pending.realm,resumeJavaScriptInstanceOf(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitObjectEnumeration':return acceptJavaScriptCallObjectEnumeration(program,current,pending.destination,pending.realm,resumeJavaScriptObjectEnumerationGetter(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayAssignment':return acceptJavaScriptCallArrayAssignment(program,current,pending.destination,pending.realm,resumeJavaScriptArrayAssignment(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitNumber':return resumeJavaScriptCallNumber(program,current,pending.destination,pending.realm,pending.state,value);
    case 'waitText':return resumeJavaScriptCallText(program,current,pending.destination,pending.realm,pending.state,value);
    case 'waitPropertyReflection':case 'waitObjectPrototype':case 'waitObjectIntegrity':case 'waitConstructorCheck':case 'waitPrimitiveWrapper':case 'waitSymbolAllocation':case 'waitSymbolDescription':case 'waitFunctionNameSymbol':case 'waitFunctionText':case 'callIdle':case 'waitError':case 'waitSourceError':case 'waitErrorValue':case 'waitErrorIntrinsic':case 'waitHostCapability':case 'waitCollectionIteratorResult':case 'waitArgumentRest':return {$:'machineInvalid',heap:machine.heap,reason:'a returning call needs its actual suspended parent continuation'};
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction
 * @returns {JavaScriptCallMachineTransition} */
export function executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction){
  const next=advanceJavaScriptCallFrame(frame),registers=frame.registers;
  switch(instruction.$){
    case 'closureAllocate':{
      const template=(program.templates??[])[findJavaScriptCallTemplate(program,instruction.owner,instruction.ordinal)];
      if(template===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'closure allocation needs its actual static lexical template'};
      if(machine.heap.objects.length>=4096)return {$:'machineResourceLimit',machine,reason:'closure allocation exceeds the shared heap identity budget'};
      return {$:'machineAllocateObject',machine,prototype:template.prototype};
    }
    case 'ordinaryAllocate':case 'arrayAllocate':case 'collectionAllocate':case 'collectionIteratorAllocate':{
      const held=readJavaScriptCollectionConstructionReceiver(program,machine,frame,instruction);if(held.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:held.error};const receiver=held.receiver;if(receiver!==undefined)return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,receiver))};
      if(instruction.$==='collectionIteratorAllocate'){
        const kind=instruction.kind;
        if(kind==='Array'){
          const receiver=registers[0]??{$:'undefined'},owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
          if(receiver.$==='null'||receiver.$==='undefined')return owner===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array iterator creation retains its actual realm'}:requestJavaScriptCallError(machine,owner.realm,'TypeError','Cannot convert undefined or null to object');
          if(receiver.$!=='object')return {$:'machineNeedArrayIteratorBoxing',machine,receiver};
        }else if(kind==='String'){
          const value=registers[instruction.input??-1]??{$:'undefined'};if(value.$!=='text'&&value.$!=='sourceText')return {$:'machineInvalid',heap:machine.heap,reason:'String iterator allocation retains its actual completed text conversion'};
        }else{
          const receiver=readJavaScriptCollectionIteratorReceiverError(machine.heap,registers[0]??{$:'undefined'},kind);
          if(receiver.error!==''){
            const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
            return receiver.typeError&&owner!==undefined?requestJavaScriptCallError(machine,owner.realm,'TypeError',receiver.error):{$:'machineInvalid',heap:machine.heap,reason:receiver.error};
          }
        }
      }
      const prototype=registers[instruction.prototype]??{$:'undefined'};
      if(prototype.$!=='null'&&prototype.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary allocation requires its actual null or Object prototype'};
      if(machine.heap.objects.length>=4096)return {$:'machineResourceLimit',machine,reason:'ordinary heap identity directory exceeds its explicit resource budget'};
      return {$:'machineAllocateObject',machine,prototype};
    }
    case 'ordinaryGet':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Get instructions require their actual Object operand'};
      const plan=planJavaScriptOrdinaryGet(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'});
      if(plan.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:plan.error};
      if(plan.action==='value')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,plan.value))};
      if(plan.action==='call')return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitGet',destination:instruction.destination}}),plan.callee,plan.receiver,plan.arguments);
      return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Get needs its actual value or pending getter'};
    }
    case 'ordinaryHas':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary HasProperty retains its actual Object reference'};
      const keyError=readJavaScriptOrdinaryKeyError(machine.heap,registers[instruction.key]??{$:'undefined'});if(keyError!=='')return {$:'machineInvalid',heap:machine.heap,reason:keyError};
      const presence=readJavaScriptIndexedPropertyPresence(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'});if(presence.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:presence.error};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:presence.present}))};
    }
    case 'ordinaryDelete':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary deletion retains its actual Object reference'};
      const deleted=deleteJavaScriptOrdinaryProperty(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'});if(deleted.error!=='')return {$:'machineInvalid',heap:deleted.heap,reason:deleted.error};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(deleted.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:deleted.success}))};
    }
    case 'ordinarySet':{
      const object=registers[instruction.object]??{$:'undefined'};if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary Set instructions require their actual Object operand'};
      const plan=planJavaScriptOrdinarySetWithArrayDefinition(machine.heap,object.identity,registers[instruction.key]??{$:'undefined'},registers[instruction.value]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'});
      if(plan.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:plan.error};
      if(plan.action==='array definition'){
        const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
        const receiver=plan.receiver,definition=plan.definition;
        if(owner===undefined||receiver.$!=='object'||definition===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'deferred Array assignment requires its actual owner, receiver and value descriptor'};
        const identity=receiver.identity;
        if(identity===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'deferred Array assignment requires its actual owner, receiver and value descriptor'};
        const current=replaceJavaScriptCallFrame(plan.heap,machine.frames,next);
        return acceptJavaScriptCallArrayAssignment(program,current,instruction.destination,owner.realm,beginJavaScriptArrayAssignment(plan.heap,identity,registers[instruction.key]??{$:'undefined'},definition,program.exoticKey));
      }
      if(plan.action==='call')return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSet',destination:instruction.destination}}),plan.callee,plan.receiver,plan.arguments);
      if(plan.action==='host call')return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitHostSet',destination:instruction.destination}}),plan.callee,plan.receiver,plan.arguments);
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(plan.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:plan.success}))};
    }
    case 'requireWrite':{
      const success=registers[instruction.success]??{$:'undefined'},owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
      if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a strict write guard needs its actual registered source owner'};
      if(success.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'a strict write guard requires its actual ordinary Set result'};
      const current=replaceJavaScriptCallFrame(machine.heap,machine.frames,next);
      return success.value?{$:'machineContinue',machine:current}:requestJavaScriptCallError(current,owner.realm,'TypeError','the ordinary assignment was rejected');
    }
    default:return {$:'machineInvalid',heap:machine.heap,reason:'an ordinary property executor needs its actual selected instruction'};
  }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction @returns {JavaScriptCallMachineTransition} */
export function executeJavaScriptCallPrimitiveInstruction(program,machine,frame,instruction){
 if(instruction.$!=='primitive')return {$:'machineInvalid',heap:machine.heap,reason:'a primitive executor requires its actual selected instruction'};
 const arity=readJavaScriptCallPrimitiveArityError(instruction.operation,instruction.arguments.length);if(arity!=='')return {$:'machineInvalid',heap:machine.heap,reason:arity};
 const arguments_=instruction.arguments.map(index=>frame.registers[index]??{$:'undefined'}),input=arguments_[0]??{$:'undefined'};
 if(readJavaScriptSourceExceptionClass(instruction.operation)!=='')return requestJavaScriptCallSourceError(program,machine,frame,instruction);
 if((instruction.operation==='convert explicit number'||instruction.operation==='coerce primitive number'||readJavaScriptIntegerProtocolOperationError(instruction.operation)==='')&&input.$==='object'){
  const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
  if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a number conversion needs its actual registered source owner'};
  const current=replaceJavaScriptCallFrame(machine.heap,machine.frames,advanceJavaScriptCallFrame(frame));
  return acceptJavaScriptCallNumber(program,current,instruction.destination,owner.realm,beginJavaScriptScalarProtocol(machine.heap,input,program.exoticKey,instruction.operation));
 }
 if(instruction.operation==='convert primitive text'&&input.$==='object'){
  const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
  if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a text conversion needs its actual registered source owner'};
  const current=replaceJavaScriptCallFrame(machine.heap,machine.frames,advanceJavaScriptCallFrame(frame));
  return acceptJavaScriptCallText(program,current,instruction.destination,owner.realm,beginJavaScriptScalarProtocol(machine.heap,input,program.exoticKey,instruction.operation));
 }
 const result=executeJavaScriptCallPrimitive(instruction.operation,arguments_);
 switch(result.$){
  case 'primitiveValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,result.value))};
  case 'primitiveInvalid':return {$:'machineInvalid',heap:machine.heap,reason:result.reason};
  case 'primitiveResourceLimit':return {$:'machineResourceLimit',machine,reason:result.reason};
  case 'primitiveError':{
   const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
   if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a primitive failure needs its actual registered source owner'};
   return requestJavaScriptCallError(replaceJavaScriptCallFrame(machine.heap,machine.frames,advanceJavaScriptCallFrame(frame)),owner.realm,result.errorClass,result.reason);
  }
 }
}

/** One instruction/continuation per step. Resource limits are explicit model
 * outcomes; no timeout is passed off as a language-level exception.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @returns {JavaScriptCallMachineTransition} */
export function stepJavaScriptCallMachine(program,machine){
  const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call step needs its actual active frame'};
  const pending=frame.pending;
  switch(pending.$){
    case 'waitArrayCreation':{
     if(pending.error!==undefined)return {$:'machineAllocateError',machine,realm:pending.error.realm,errorClass:pending.error.errorClass,reason:pending.error.reason};
     const construction=pending.construction;
     if(construction!==undefined){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'};return acceptJavaScriptCallArrayConstruction(program,machine,stepJavaScriptConstruction(program,machine.heap,directory,construction));}
     const fallback=pending.fallback;
     if(fallback!==undefined)return requestJavaScriptCallArrayFallback(program,machine,pending.destination,pending.realm,pending.state,fallback);
     return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,stepJavaScriptArrayCreation(machine.heap,pending.state));
    }
    case 'waitConstructorCheck':return acceptJavaScriptCallConstructorCheck(program,machine,pending.destination,pending.input,pending.required);
    case 'waitConstruction':{const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'construction requires actual constructor authority'}:acceptJavaScriptCallConstruction(program,machine,pending.destination,pending.realm,stepJavaScriptConstruction(program,machine.heap,directory,pending.state));}
    case 'waitFunctionBind':return acceptJavaScriptCallFunctionBind(program,machine,pending.destination,pending.realm,stepJavaScriptFunctionBind(machine.heap,pending.state));
    case 'waitFunctionText':return {$:'machineNeedFunctionText',machine,input:pending.input,kind:pending.kind,realm:pending.realm};
    case 'waitObjectFromEntries':return pending.error!==undefined?{$:'machineAllocateError',machine,realm:pending.error.realm,errorClass:pending.error.errorClass,reason:pending.error.reason}:acceptJavaScriptCallObjectFromEntries(program,machine,pending.destination,pending.realm,stepJavaScriptObjectFromEntries(machine.heap,pending.state));
    case 'waitPrimitiveWrapper':return acceptJavaScriptCallPrimitiveWrapper(machine,pending.destination,beginJavaScriptPrimitiveWrapper(machine.heap,pending.state.input,pending.state.prototype));
    case 'waitSymbolAllocation':return machine.heap.symbols.length>=16384?{$:'machineResourceLimit',machine,reason:'Symbol allocation exceeds the selected 16384 identity profile'}:{$:'machineAllocateSymbol',machine,description:pending.description,...(pending.registryKey===undefined?{}:{registryKey:pending.registryKey})};
    case 'waitSymbolDescription':{
     const symbol=pending.symbol;
     if(symbol.$!=='symbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol description needs its actual primitive Symbol'};
     const fact=readJavaScriptSymbolRegistrationFact(machine.heap,symbol.identity);
     const key=fact!==undefined&&fact.registered?fact.key:undefined;
     if(key!==undefined)return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,key))};
     return {$:'machineNeedSymbolDescription',machine,symbol};
    }
    case 'waitFunctionNameSymbol':{
     const symbol=pending.key;
     if(symbol.$!=='symbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol function naming needs its actual primitive Symbol'};
     const fact=readJavaScriptSymbolRegistrationFact(machine.heap,symbol.identity);
     const key=fact!==undefined&&fact.registered?fact.key:undefined;
     if(key!==undefined)return resumeJavaScriptCallFunctionNameSymbolDescription(program,machine,key);
     return {$:'machineNeedFunctionNameSymbolDescription',machine,object:pending.object,key:symbol,prefix:pending.prefix};
    }
    case 'waitArrayConcatenation':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array concatenation step retains actual species authority'};const construction=pending.construction;if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,construction));return acceptJavaScriptCallArrayConcatenation(program,machine,pending.destination,pending.realm,stepJavaScriptArrayConcatenation(machine.heap,directory,pending.state));}
    case 'waitArraySlice':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array slice step retains actual species authority'};const construction=pending.construction;if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,construction));return acceptJavaScriptCallArraySlice(program,machine,pending.destination,pending.realm,stepJavaScriptArraySlice(machine.heap,directory,pending.state));}
    case 'waitArrayTransform':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform step retains its actual source directory'};const construction=pending.construction;if(construction!==undefined)return acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,construction));return acceptJavaScriptCallArrayTransform(program,machine,pending.destination,pending.realm,stepJavaScriptArrayTransform(machine.heap,directory,pending.state));}
    case 'waitArraySearch':return acceptJavaScriptCallArraySearch(program,machine,pending.destination,pending.realm,stepJavaScriptArraySearch(machine.heap,pending.state));
    case 'waitCollectionForEach':return acceptJavaScriptCallCollectionForEach(program,machine,pending.destination,pending.realm,stepJavaScriptCollectionForEach(machine.heap,pending.state));
    case 'waitArrayCallback':return acceptJavaScriptCallArrayCallback(program,machine,pending.destination,pending.realm,stepJavaScriptArrayCallback(machine.heap,pending.state));
    case 'waitObjectSpread':return pending.state.phase==='getter'?pushJavaScriptCall(program,machine,pending.state.callee,pending.state.input,[]):acceptJavaScriptCallObjectSpread(program,machine,pending.destination,pending.realm,stepJavaScriptObjectSpread(machine.heap,pending.state));
    case 'waitObjectEnumeration':return acceptJavaScriptCallObjectEnumeration(program,machine,pending.destination,pending.realm,stepJavaScriptObjectEnumeration(machine.heap,pending.state));
    case 'waitPropertyDefinitions':return acceptJavaScriptCallPropertyDefinitions(program,machine,pending.destination,pending.realm,stepJavaScriptPropertyDefinitions(machine.heap,pending.state));
    case 'waitPropertyReflection':return acceptJavaScriptCallPropertyReflection(machine,pending.destination,pending.realm,stepJavaScriptPropertyReflection(machine.heap,pending.state));
    case 'waitInstanceOf':return acceptJavaScriptCallInstanceOf(program,machine,pending.destination,pending.realm,stepJavaScriptInstanceOf(machine.heap,pending.state));
    case 'waitObjectIntegrity':return acceptJavaScriptCallObjectIntegrity(machine,pending.destination,pending.realm,stepJavaScriptObjectIntegrity(machine.heap,pending.state));
    case 'waitObjectPrototype':return {$:'machineNeedObjectPrototype',machine,input:pending.input,realm:pending.realm};
    case 'waitArrayAssignment':return acceptJavaScriptCallArrayAssignment(program,machine,pending.destination,pending.realm,stepJavaScriptArrayAssignment(machine.heap,pending.state));
    case 'waitArgumentRest':return acceptJavaScriptCallArgumentRest(machine,pending.destination,pending.realm,stepJavaScriptArgumentRest(machine.heap,frame.arguments,pending.state));
    case 'waitIteratorProtocol':return acceptJavaScriptCallIteratorProtocol(program,machine,projectJavaScriptCallIteratorOperation(pending),stepJavaScriptIteratorProtocol(machine.heap,pending.state));
    case 'waitArraySpread':return acceptJavaScriptCallArraySpread(program,machine,pending.destination,pending.realm,stepJavaScriptArraySpread(machine.heap,pending.state));
    case 'waitArrayIteratorNext':return acceptJavaScriptCallArrayIteration(program,machine,stepJavaScriptArrayIteratorNext(machine.heap,pending.state));
    case 'waitCollectionIteratorResult':return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,pending.destination,pending.realm,stepJavaScriptCollectionIteratorResult(machine.heap,pending.state));
    case 'waitKey':return acceptJavaScriptCallKey(program,machine,pending.destination,pending.realm,stepJavaScriptPropertyKeyConversion(machine.heap,pending.state));
    case 'waitDefinition':return acceptJavaScriptCallDefinition(program,machine,pending.destination,pending.realm,stepJavaScriptPropertyDefinition(machine.heap,pending.state));
    case 'waitNumber':return stepJavaScriptCallNumber(program,machine,pending.destination,pending.realm,pending.state);
    case 'waitText':return stepJavaScriptCallText(program,machine,pending.destination,pending.realm,pending.state);
    case 'callIdle':break;
    default:return {$:'machineInvalid',heap:machine.heap,reason:'a pending call or allocation needs its actual completion'};
  }
  return executeJavaScriptCallInstruction(program,machine,frame);
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallMachineTransition} */
export function beginJavaScriptCallMachine(program,heap,identity,receiver,arguments_){
  const error=readJavaScriptCallProgramError(program,heap);if(error!=='')return {$:'machineInvalid',heap,reason:error};
  const fn=readJavaScriptExecutableCallFunction(program,heap,identity);if(fn===undefined)return {$:'machineInvalid',heap,reason:'the entry callable needs its actual executable definition'};
  if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap,reason:'an ordinary class entry requires its actual source construction operation'};
  if(testJavaScriptCollectionConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap,reason:'a collection entry requires its actual source construction operation'};
  if(arguments_.length>1048576)return {$:'machineInvalid',heap,reason:'entry arguments exceed the explicit call storage budget'};
  const receiverError=readJavaScriptOrdinaryValueError(heap,receiver);if(receiverError!=='')return {$:'machineInvalid',heap,reason:receiverError};
  for(const value of arguments_)if(readJavaScriptOrdinaryValueError(heap,value)!=='')return {$:'machineInvalid',heap,reason:'entry arguments belong to the actual global context'};
  return {$:'machineContinue',machine:{$:'callMachine',heap,frames:[createJavaScriptCallFrame(fn,receiver,arguments_)]}};
}
/** The global allocator supplies an unused identity; heap maximums do not
 * establish global identity authority. A request leaves its instruction and
 * prototype register unchanged until this explicit resumption advances once.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} identity @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallObjectAllocation(program,machine,identity){
  return resumeJavaScriptCallAllocation(program,machine,identity);
}
/** The allocator must create the actual Error in the requested realm and
 * extend the same global heap. Its authority is external, not inferred from
 * an Object name/prototype or membership in this ordinary directory.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallErrorAllocation(program,machine,realm,value){
  const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption needs its actual active frame'};
  const pending=frame.pending;
  if(pending.$==='waitArrayCreation'){
   const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||value.$!=='object'||value.callable||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Array creation retains actual Error allocation before cleanup'};
   const resumed=pending.error!==undefined?resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'throwCompletion',value}):resumeJavaScriptArrayCreationError(machine.heap,pending.state,value),{error:discarded,...base}=pending;return acceptJavaScriptCallArrayCreation(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
  }
  if(pending.$==='waitObjectFromEntries'){
   const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!==''||value.$!=='object'||value.callable)return {$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries retains its actual realm Error allocation before cleanup'};
   const resumed=pending.error!==undefined?resumeJavaScriptObjectFromEntriesCall(machine.heap,pending.state,{$:'throwCompletion',value}):resumeJavaScriptObjectFromEntriesError(machine.heap,pending.state,value);
   const {error:discarded,...base}=pending;return acceptJavaScriptCallObjectFromEntries(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
  }
  if(pending.$!=='waitError')return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption requires its actual pending allocation'};
  if(pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption retains its actual requested realm'};
  const valueError=readJavaScriptOrdinaryValueError(machine.heap,value);if(valueError!=='')return {$:'machineInvalid',heap:machine.heap,reason:valueError};
  if(value.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'an intrinsic Error allocation supplies its actual noncallable Object'};
  if(value.callable)return {$:'machineInvalid',heap:machine.heap,reason:'an intrinsic Error allocation supplies its actual noncallable Object'};
  return throwJavaScriptCallValue(machine,value);
}

/** A source throw constructs the actual requested standard Error externally.
 * Text remains UTF16 data; presence distinguishes Error() from Error('').
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction
 * @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallSourceError(program,machine,frame,instruction){
 if(instruction.$!=='primitive')return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction retains its actual primitive instruction'};
 const errorClass=readJavaScriptSourceExceptionClass(instruction.operation);
 if(errorClass===''||instruction.arguments.length!==2)return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction retains its actual class and arguments'};
 const message=frame.registers[instruction.arguments[0]?? -1]??{$:'undefined'},present=frame.registers[instruction.arguments[1]?? -1]??{$:'undefined'};
 if(message.$!=='text'&&message.$!=='sourceText')return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction requires a normalized UTF16 message'};
 const messageError=readJavaScriptOrdinaryValueError(machine.heap,message);if(messageError!=='')return {$:'machineInvalid',heap:machine.heap,reason:messageError};
 if(present.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction requires actual own-message presence'};
 if(typeof present.value!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction requires actual own-message presence'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'source Error construction retains its actual registered source owner'};
 const next={...frame,position:frame.position+1,pending:{$:'waitSourceError',realm:fn.realm,errorClass,message,messagePresent:present.value}};
 return {$:'machineAllocateSourceError',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,next),realm:fn.realm,errorClass,message,messagePresent:present.value};
}

/** The authoritative allocator creates the actual Error in the requested realm,
 * uses undefined when messagePresent is false, and extends the global heap.
 * Membership/prototype names alone cannot establish allocation authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallSourceErrorAllocation(program,machine,realm,value){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'source Error resumption requires its actual active frame'};
 if(frame.pending.$!=='waitSourceError')return {$:'machineInvalid',heap:machine.heap,reason:'source Error resumption requires its actual pending allocation'};
 if(frame.pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'source Error resumption retains its actual requested realm'};
 const valueError=readJavaScriptOrdinaryValueError(machine.heap,value);if(valueError!=='')return {$:'machineInvalid',heap:machine.heap,reason:valueError};
 if(value.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'source Error allocation supplies its actual noncallable Object'};
 if(value.callable)return {$:'machineInvalid',heap:machine.heap,reason:'source Error allocation supplies its actual noncallable Object'};
 return throwJavaScriptCallValue(machine,value);
}

/** Constructor calls retain a distinct operation: ordinary calls of a class
 * cannot execute its defaults/body. The first source profile fixes newTarget
 * to this actual registered derived class; Reflect.construct needs a wider
 * explicit source profile.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue[]} arguments_
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptErrorConstructor(program,machine,callee,arguments_){
 const parent=machine.frames[machine.frames.length-1];if(parent===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error construction needs its actual suspended caller'};
 const owner=readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity);if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error construction needs its actual caller realm'};
 if(callee.$!=='object'||!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected Error class is not a constructor');
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);if(fn===undefined||!testJavaScriptErrorConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'Error construction requires the actual registered derived constructor definition'};
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;for(const list of active.argumentLists??[])argumentSlots+=list.values.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'derived constructor frames exceed the explicit execution storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([{...createJavaScriptCallFrame(fn,{$:'undefined'},arguments_),newTarget:callee}])}};
}
/** The static base-class profile has already evaluated every argument before
 * requesting receiver allocation. This call checks the real callee own data
 * prototype and the allocated receiver; source spellings grant no authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptOrdinaryClassConstructor(program,machine,callee,receiver,arguments_){
 const parent=machine.frames[machine.frames.length-1],owner=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity);
 if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'ordinary class construction needs its actual suspended caller'};
 if(callee.$!=='object'||!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected class is not a constructor');
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);
 if(fn===undefined||!testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'ordinary class construction requires its actual registered constructor body'};
 const invalid=readJavaScriptOrdinaryClassReceiverError(machine.heap,callee,receiver);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;for(const list of active.argumentLists??[])argumentSlots+=list.values.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'ordinary class frames exceed the explicit execution storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([{...createJavaScriptCallFrame(fn,receiver,arguments_),newTarget:callee}])}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue[]} arguments_
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptCollectionConstructor(program,machine,callee,arguments_){
 const parent=machine.frames[machine.frames.length-1],owner=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity);if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'collection construction needs its actual suspended caller'};
 if(callee.$!=='object'||!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected collection is not a constructor');
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);if(fn===undefined||!testJavaScriptCollectionConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'collection construction requires its actual standard registered constructor'};
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;for(const list of active.argumentLists??[])argumentSlots+=list.values.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'collection constructor frames exceed their actual storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([{...createJavaScriptCallFrame(fn,{$:'undefined'},arguments_),newTarget:callee}])}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {number} destination
 * @param {JavaScriptCollectionCompletion} completion @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCollectionCompletion(program,machine,frame,destination,completion){
 const current={$:'callMachine',heap:completion.heap,frames:machine.frames};
 switch(completion.$){
  case 'collectionNormal':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(completion.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),destination,completion.value))};
  case 'collectionInvalid':return {$:'machineInvalid',heap:completion.heap,reason:completion.reason};
  case 'collectionTypeError':{const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);return owner===undefined?{$:'machineInvalid',heap:completion.heap,reason:'collection errors require their actual intrinsic realm'}:requestJavaScriptCallError(current,owner.realm,'TypeError',completion.reason);}
  case 'collectionResourceLimit':return {$:'machineResourceLimit',machine:current,reason:completion.reason};
  case 'collectionNeedIterable':return {$:'machineNeedCollectionIterable',machine:current,target:completion.target,input:completion.input};
 }
}
/** Source conversion and argument effects finish before the allocator runs.
 * Constructor/prototype membership is consistency, never intrinsic authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptCallInstruction} instruction
 * @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptErrorValue(program,machine,frame,instruction){
 if(instruction.$!=='constructErrorValue')return {$:'machineInvalid',heap:machine.heap,reason:'Error allocation needs its actual value construction instruction'};
 const constructor=frame.registers[instruction.constructor]??{$:'undefined'},newTarget=frame.registers[instruction.newTarget]??{$:'undefined'},prototype=frame.registers[instruction.prototype]??{$:'undefined'},message=frame.registers[instruction.message]??{$:'undefined'},present=frame.registers[instruction.present]??{$:'undefined'};
 const error=readJavaScriptErrorConstructionOperandsError(machine.heap,constructor,newTarget,prototype,message,present);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 if(present.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'Error presence requires its actual Boolean operand'};
 if(instruction.bindReceiver&&(newTarget.$!=='object'||newTarget.identity!==frame.identity))return {$:'machineInvalid',heap:machine.heap,reason:'the direct derived constructor retains its actual newTarget identity'};
 const pending={$:'waitErrorValue',destination:instruction.destination,realm:instruction.realm,errorClass:instruction.errorClass,constructor,newTarget,prototype,message,messagePresent:present.value,bindReceiver:instruction.bindReceiver,beforeDirectory:readJavaScriptErrorAllocationDirectory(machine.heap)};
 return {$:'machineAllocateErrorValue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...advanceJavaScriptCallFrame(frame),pending}),realm:instruction.realm,errorClass:instruction.errorClass,constructor,newTarget,prototype,message,messagePresent:present.value};
}
/** An authoritative realm allocator supplies the live Error and latest shared
 * heap. It must retain genuine internal Error data and lazy host stack hooks.
 * Ordinary attributes or a self-asserted flag cannot certify that authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value
 * @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptErrorValueAllocation(program,machine,realm,value){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error value allocation requires its actual pending constructor continuation'};
 const pending=frame.pending;if(pending.$!=='waitErrorValue')return {$:'machineInvalid',heap:machine.heap,reason:'Error value allocation requires its actual pending constructor continuation'};if(pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'Error value allocation retains its actual requested intrinsic realm'};
 const invalid=readJavaScriptErrorAllocationResultError(machine.heap,value,pending.prototype,pending.beforeDirectory);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
 let next=writeJavaScriptCallRegister(frame,pending.destination,value);
 if(pending.bindReceiver){const previous=frame.registers[0]??{$:'undefined'};if(previous.$!=='undefined')return requestJavaScriptCallError(machine,realm,'ReferenceError','the derived receiver was already initialized by a previous super call');next=writeJavaScriptCallRegister(next,0,value);}
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,next)};
}
/** An actual host allocator may throw before producing the requested Error.
 * Propagate that distinct value through the existing handler algebra rather
 * than treating it as a successful initialized receiver.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value
 * @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptErrorAllocationAbrupt(program,machine,realm,value){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'an abrupt allocator result needs its actual suspended frame'};
 const pending=frame.pending;
 if(pending.$==='waitArrayCreation'){
  const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Array creation abrupt Error allocation retains its actual request'};
  const resumed=pending.error!==undefined?resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'throwCompletion',value}):throwJavaScriptArrayCreation(machine.heap,pending.state,value),{error:discarded,...base}=pending;return acceptJavaScriptCallArrayCreation(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
 }
 if(pending.$==='waitObjectFromEntries'){
  const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries abrupt Error allocation retains its actual request and thrown value'};
  const resumed=pending.error!==undefined?resumeJavaScriptObjectFromEntriesCall(machine.heap,pending.state,{$:'throwCompletion',value}):throwJavaScriptObjectFromEntries(machine.heap,pending.state,value),{error:discarded,...base}=pending;
  return acceptJavaScriptCallObjectFromEntries(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
 }
 switch(pending.$){
  case 'waitErrorValue':case 'waitError':case 'waitSourceError':{
   if(pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'an abrupt allocator retains its requested realm'};
   const invalid=readJavaScriptOrdinaryValueError(machine.heap,value);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
   return throwJavaScriptCallValue(machine,value);
  }
  default:return {$:'machineInvalid',heap:machine.heap,reason:'an abrupt allocator result needs its actual Error allocation request'};
 }
}
/** Intrinsic accessors/methods (including engine-specific lazy stack access)
 * execute only through their actual externally registered callable identity.
 * This resumption transports an observed result/throw; it fabricates no stack.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {bigint} identity @param {JavaScriptSemanticValue} value
 * @param {boolean} abrupt @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptErrorIntrinsic(program,machine,realm,identity,value,abrupt){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitErrorIntrinsic'||frame.identity!==identity)return {$:'machineInvalid',heap:machine.heap,reason:'an Error intrinsic result retains its actual pending callable identity'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,identity);if(fn===undefined||fn.realm!==realm||!testJavaScriptErrorIntrinsicMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'an Error intrinsic result retains its actual external realm authority'};
 const invalid=readJavaScriptOrdinaryValueError(machine.heap,value);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
 if(abrupt)return throwJavaScriptCallValue(machine,value);
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,frame.pending.destination,value))};
}

/** Transport the actual qualified operation result without executing a source
 * body. The saved global directory forbids identity removal/retyping/reuse.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {bigint} identity @param {JavaScriptSemanticValue} value
 * @param {boolean} abrupt @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptHostCapability(program,machine,realm,identity,value,abrupt){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitHostCapability'||frame.identity!==identity)return {$:'machineInvalid',heap:machine.heap,reason:'a capability result retains its actual pending callable identity'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,identity);if(fn===undefined||fn.realm!==realm||!testJavaScriptHostCapabilityMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'a capability result retains its actual qualified realm authority'};
 const invalid=readJavaScriptOrdinaryValueError(machine.heap,value);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
 if(abrupt)return throwJavaScriptCallValue(machine,value);
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,frame.pending.destination,value))};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {number} destination @param {bigint} realm
 * @param {JavaScriptCollectionIteratorResultCompletion} completion @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,destination,realm,completion){
 const current={$:'callMachine',heap:completion.heap,frames:machine.frames};
 switch(completion.$){
  case 'iteratorResultContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(completion.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitCollectionIteratorResult',destination,realm,state:completion.state}))};
  case 'iteratorResultAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(completion.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitCollectionIteratorResult',destination,realm,state:completion.state})),prototype:completion.prototype};
  case 'iteratorResultNormal':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(completion.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(replaceJavaScriptCallPending(frame,{$:'callIdle'})),destination,completion.value))};
  case 'iteratorResultResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(completion.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitCollectionIteratorResult',destination,realm,state:completion.state})),reason:completion.reason};
  case 'iteratorResultTypeError':return requestJavaScriptCallError(current,realm,'TypeError',completion.reason);
  case 'iteratorResultInvalid':return {$:'machineInvalid',heap:completion.heap,reason:completion.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptObjectSpreadTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallObjectSpread(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'object spread requires its actual active source frame'};
 switch(transition.$){
  case 'objectSpreadContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectSpread',destination,realm,state:transition.state}))};
  case 'objectSpreadCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectSpread',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'objectSpreadCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'objectSpreadThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'objectSpreadError':return requestJavaScriptCallError(current,realm,transition.errorClass??'TypeError',transition.reason);
  case 'objectSpreadInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'objectSpreadResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectSpread',destination,realm,state:transition.state})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallPrimitiveIteratorGet(program,machine,completion){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'primitive iterator response retains its actual suspended source operation'};
 const pending=frame.pending;
 if(pending.$!=='waitArraySpread')return {$:'machineInvalid',heap:machine.heap,reason:'primitive iterator response retains its actual suspended source operation'};
 const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineNeedPrimitiveIteratorGet')return {$:'machineInvalid',heap:machine.heap,reason:'primitive iterator response cannot replace a live iterator call'};
 return acceptJavaScriptCallArraySpread(program,machine,pending.destination,pending.realm,resumeJavaScriptArraySpread(machine.heap,pending.state,completion,true));
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} prototype @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallPrimitiveIteratorPrototype(program,machine,prototype){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response retains its actual suspended iterator acquisition'};
 const pending=frame.pending;
 if(pending.$!=='waitArraySpread'&&pending.$!=='waitIteratorProtocol'&&pending.$!=='waitObjectFromEntries'&&pending.$!=='waitArrayCreation')return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response retains its actual suspended iterator acquisition'};
 const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineNeedPrimitiveIteratorGet')return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response cannot replace a live getter or iterator call'};
 switch(pending.$){
  case 'waitArrayCreation':return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,provideJavaScriptArrayCreationPrimitivePrototype(machine.heap,pending.state,prototype));
  case 'waitObjectFromEntries':return acceptJavaScriptCallObjectFromEntries(program,machine,pending.destination,pending.realm,provideJavaScriptObjectFromEntriesPrimitivePrototype(machine.heap,pending.state,prototype));
  case 'waitIteratorProtocol':return acceptJavaScriptCallIteratorProtocol(program,machine,projectJavaScriptCallIteratorOperation(pending),provideJavaScriptIteratorPrimitivePrototype(machine.heap,pending.state,prototype));
  case 'waitArraySpread':return acceptJavaScriptCallArraySpread(program,machine,pending.destination,pending.realm,provideJavaScriptArraySpreadPrimitivePrototype(machine.heap,pending.state,prototype));
  default:return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response retains its actual suspended iterator acquisition'};
 }
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallCompletionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallCompletion(program,transition){return transition.$==='callReturn'?finishJavaScriptCallReturn(program,transition.machine,transition.value):projectJavaScriptCallCompletionTransition(transition);}

/** @param {JavaScriptCallCompletionTransition} transition @returns {JavaScriptCallMachineTransition} */
function projectJavaScriptCallCompletionTransition(transition){
 switch(transition.$){
  case 'machineContinue':return {$:'machineContinue',machine:transition.machine};
  case 'machineThrow':return {$:'machineThrow',heap:transition.heap,value:transition.value};
  case 'machineInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'machineResourceLimit':return {$:'machineResourceLimit',machine:transition.machine,reason:transition.reason};
  case 'callReturn':return {$:'machineInvalid',heap:transition.machine.heap,reason:'throw unwinding cannot become a return'};
 }
}


/** @param {JavaScriptCallFrame} frame @returns {JavaScriptIteratorCompletion} */
export function readJavaScriptCallIteratorCloseCompletion(frame){
 for(let at=frame.handlers.length-1;at>=0;at--){const handler=frame.handlers[at];if(handler?.$==='callDeferredCompletion'){const original=handler.completion;return {$:'iteratorCompletion',kind:original.kind==='jump'?'normal':original.kind,value:original.value,target:-1};}}
 return {$:'iteratorCompletion',kind:'normal',value:{$:'undefined'},target:-1};
}
/** @param {JavaScriptCallFrame} frame @param {number} destination @param {number} done @param {JavaScriptSemanticValue} value @param {boolean} finished @returns {JavaScriptCallFrame} */
export function writeJavaScriptCallIteratorResult(frame,destination,done,value,finished){return writeJavaScriptCallRegister(writeJavaScriptCallRegister(frame,destination,value),done,{$:'boolean',value:finished});}
/** @param {JavaScriptCallIteratorRegion[]} regions @param {number} slot
 * @param {string} operation @param {JavaScriptIteratorProtocolTransition} transition
 * @returns {JavaScriptIteratorRegionsUpdate} */
function updateJavaScriptCallIteratorProtocolRegions(regions,slot,operation,transition){
 if(transition.$==='iteratorProtocolAcquired')return insertJavaScriptIteratorRegion(transition.heap,regions,slot,transition.record);
 if(operation==='close')return removeJavaScriptIteratorRegion(transition.heap,regions,slot);
 switch(transition.$){
  case 'iteratorProtocolStep':return updateJavaScriptIteratorRegion(transition.heap,regions,slot,transition.record);
  case 'iteratorProtocolError':
  case 'iteratorProtocolThrow':return operation==='step'?updateJavaScriptIteratorRegion(transition.heap,regions,slot,transition.state.record):{$:'iteratorRegionsUpdated',regions};
  default:return {$:'iteratorRegionsUpdated',regions};
 }
}
/** @param {JavaScriptCallPending} pending @returns {JavaScriptCallIteratorOperation|undefined} */
function projectJavaScriptCallIteratorOperation(pending){
 if(pending.$!=='waitIteratorProtocol')return undefined;
 return {$:'iteratorOperation',destination:pending.destination,done:pending.done,slot:pending.slot,operation:pending.operation,realm:pending.realm};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallIteratorOperation|undefined} pending @param {JavaScriptIteratorProtocolTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallIteratorProtocol(program,machine,pending,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||pending===undefined||pending.$!=='iteratorOperation')return {$:'machineInvalid',heap:transition.heap,reason:'iterator protocol retains its actual suspended source operation'};
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},regions=frame.iterators??[];
 if(transition.$==='iteratorProtocolContinue'||transition.$==='iteratorProtocolCall'||transition.$==='iteratorProtocolPrimitiveGet'){
  const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitIteratorProtocol',destination:pending.destination,done:pending.done,slot:pending.slot,operation:pending.operation,realm:pending.realm,state:transition.state}));
  if(transition.$==='iteratorProtocolCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
  return transition.$==='iteratorProtocolPrimitiveGet'?{$:'machineNeedPrimitiveIteratorGet',machine:next,receiver:transition.receiver,key:transition.key}:{$:'machineContinue',machine:next};
 }
 if(transition.$==='iteratorProtocolInvalid')return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 const update=updateJavaScriptCallIteratorProtocolRegions(regions,pending.slot,pending.operation,transition);
 if(update.$==='iteratorRegionsInvalid')return {$:'machineInvalid',heap:transition.heap,reason:update.reason};if(update.$==='iteratorRegionsResourceLimit')return {$:'machineResourceLimit',machine:current,reason:update.reason};
 const next=replaceJavaScriptCallFrame(transition.heap,current.frames,{...frame,iterators:update.regions,pending:{$:'callIdle'}});
 if(transition.$==='iteratorProtocolError')return requestJavaScriptCallError(next,pending.realm,transition.errorClass,transition.reason);
 if(transition.$==='iteratorProtocolThrow')return throwJavaScriptCallValue(next,transition.value);
 const result=transition.$==='iteratorProtocolStep'?transition.value:{$:'undefined'},done=transition.$==='iteratorProtocolStep'?transition.done:pending.operation==='close';return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,next.frames,writeJavaScriptCallIteratorResult(next.frames[next.frames.length-1]??frame,pending.destination,pending.done,result,done))};
}


/**
 * @typedef {{$:'machineInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'machineContinue',machine:JavaScriptCallMachine}} JavaScriptIteratorThrowTransition
 */
/** @typedef {{$:'iteratorThrowContinue',machine:JavaScriptCallMachine,value:JavaScriptSemanticValue} | {$:'iteratorThrowTransition',transition:JavaScriptIteratorThrowTransition}} JavaScriptIteratorThrowResolution */
/** Resume the selected getter/method throw before crossing source finalizers.
 * @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptIteratorThrowResolution} */
export function resumeJavaScriptCallIteratorThrow(machine,value){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:machine.heap,reason:'iterator throw retains its actual suspended source operation'}};
 const pending=frame.pending;if(pending.$!=='waitIteratorProtocol')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:machine.heap,reason:'iterator throw retains its actual suspended source operation'}};
 const transition=resumeJavaScriptIteratorProtocolCall(machine.heap,pending.state,{$:'throwCompletion',value}),regions=frame.iterators??[];
 if(transition.$!=='iteratorProtocolClosed'&&transition.$!=='iteratorProtocolThrow')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:transition.heap,reason:transition.$==='iteratorProtocolInvalid'?transition.reason:'selected iterator throw retains its actual abrupt or suppressed completion'}};
 const update=pending.operation==='close'?removeJavaScriptIteratorRegion(transition.heap,regions,pending.slot):pending.operation==='step'?updateJavaScriptIteratorRegion(transition.heap,regions,pending.slot,transition.$==='iteratorProtocolThrow'?transition.state.record:pending.state.record):{$:'iteratorRegionsUpdated',regions};
 if(update.$!=='iteratorRegionsUpdated')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:transition.heap,reason:'iterator throw retains its actual region update'}};
 const next={...frame,iterators:update.regions,pending:{$:'callIdle'}},current=replaceJavaScriptCallFrame(transition.heap,machine.frames,next);
 if(transition.$==='iteratorProtocolClosed')return {$:'iteratorThrowTransition',transition:{$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallIteratorResult(next,pending.destination,pending.done,{$:'undefined'},true))}};
 if(transition.$==='iteratorProtocolThrow')return {$:'iteratorThrowContinue',machine:current,value:transition.value};
 return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:transition.heap,reason:'selected iterator throw retains its actual abrupt or suppressed completion'}};
}


/** @param {JavaScriptCallMachine} machine @param {number} destination @param {bigint} realm
 * @param {JavaScriptArgumentRestTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArgumentRest(machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'argument rest retains its actual current source frame'};
 switch(transition.$){
  case 'argumentRestContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArgumentRest',destination,realm,state:transition.state}))};
  case 'argumentRestAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArgumentRest',destination,realm,state:transition.state})),prototype:transition.prototype};
  case 'argumentRestCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'argumentRestInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'argumentRestResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArgumentRest',destination,realm,state:transition.state})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptObjectEnumerationTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallObjectEnumeration(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Object enumeration retains its actual current source frame'};
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames};
 switch(transition.$){
  case 'objectEnumerationContinue':case 'objectEnumerationAllocate':case 'objectEnumerationCall':case 'objectEnumerationResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectEnumeration',destination,realm,state:transition.state}));
   if(transition.$==='objectEnumerationAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   if(transition.$==='objectEnumerationCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   return transition.$==='objectEnumerationResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'objectEnumerationCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'objectEnumerationThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'objectEnumerationError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'objectEnumerationInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** @param {JavaScriptCallMachine} machine @param {number} destination @param {bigint} realm
 * @param {JavaScriptObjectIntegrityTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallObjectIntegrity(machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Object observation retains its actual current source frame'};const current={$:'callMachine',heap:transition.heap,frames:machine.frames};
 switch(transition.$){
  case 'objectPrototypeMetadata':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectPrototype',destination,realm,input:transition.input}))};
  case 'objectIntegrityContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectIntegrity',destination,realm,state:transition.state}))};
  case 'objectIntegrityCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'objectIntegrityResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectIntegrity',destination,realm,state:transition.state})),reason:transition.reason};
  case 'objectIntegrityError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'objectIntegrityInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} prototype @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallObjectPrimitivePrototype(program,machine,prototype){
 const error=readJavaScriptCallMachineError(program,machine)||readJavaScriptOrdinaryValueError(machine.heap,prototype);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitObjectPrototype'||prototype.$!=='object'||prototype.callable)return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response requires its actual pending source operation and intrinsic Object root'};
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,frame.pending.destination,prototype))};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination @param {bigint} realm @param {JavaScriptInstanceOfTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallInstanceOf(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'instanceof retains its actual current source frame'};
 switch(transition.$){
  case 'instanceOfContinue':case 'instanceOfCall':case 'instanceOfResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitInstanceOf',destination,realm,state:transition.state}));
   if(transition.$==='instanceOfCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   return transition.$==='instanceOfResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'instanceOfCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'instanceOfThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'instanceOfError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'instanceOfInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** @param {JavaScriptCallMachine} machine @param {number} destination @param {bigint} realm @param {JavaScriptPropertyReflectionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallPropertyReflection(machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'property reflection retains its actual active frame'};
 switch(transition.$){
  case 'propertyReflectionContinue':case 'propertyReflectionAllocate':case 'propertyReflectionResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitPropertyReflection',destination,realm,state:transition.state}));
   return transition.$==='propertyReflectionAllocate'?{$:'machineAllocateObject',machine:next,prototype:transition.prototype}:transition.$==='propertyReflectionResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'propertyReflectionCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'propertyReflectionError':return requestJavaScriptCallError({$:'callMachine',heap:transition.heap,frames:machine.frames},realm,transition.errorClass,transition.reason);
  case 'propertyReflectionInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {number} destination @param {bigint} realm @param {JavaScriptPropertyDefinitionsTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallPropertyDefinitions(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'bulk definitions retain their actual active frame'};
 switch(transition.$){
  case 'propertyDefinitionsContinue':case 'propertyDefinitionsCall':case 'propertyDefinitionsAllocate':case 'propertyDefinitionsResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitPropertyDefinitions',destination,realm,state:transition.state}));
   if(transition.$==='propertyDefinitionsCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   return transition.$==='propertyDefinitionsAllocate'?{$:'machineAllocateObject',machine:next,prototype:transition.prototype}:transition.$==='propertyDefinitionsResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'propertyDefinitionsCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'propertyDefinitionsThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'propertyDefinitionsError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'propertyDefinitionsInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** Metadata resumption authenticates the suspended instruction before accepting
 * optional text; the embedding separately proves actual Symbol ownership.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} description @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallSymbolDescription(program,machine,description){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Symbol descriptions require their actual suspended source instruction'};
 const pending=frame.pending;if(pending.$!=='waitSymbolDescription')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol descriptions require their actual suspended source instruction'};
 if(description.$!=='undefined'&&description.$!=='text'&&description.$!=='sourceText'||readJavaScriptOrdinaryValueError(machine.heap,description)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol descriptions retain actual optional UTF16 text metadata'};
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,description))};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm
 * @param {JavaScriptCollectionForEachTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallCollectionForEach(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'collection callbacks retain their actual active intrinsic frame'};
 switch(transition.$){
  case 'collectionForEachContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitCollectionForEach',destination,realm,state:transition.state}))};
  case 'collectionForEachCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitCollectionForEach',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'collectionForEachCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'collectionForEachThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'collectionForEachError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'collectionForEachInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** Only actual global identity authority can supply the fresh Symbol identity.
 * Description coercion is already complete; allocation never re-runs it.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} identity @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallSymbolAllocation(program,machine,identity){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires its actual suspended source instruction'};
 const pending=frame.pending;if(pending.$!=='waitSymbolAllocation')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires its actual suspended source instruction'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||machine.heap.symbols.includes(identity)||machine.heap.objects.some(object=>object.identity===identity))return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires an unused actual positive unsigned64 global identity'};
 if(machine.heap.symbols.length>=16384)return {$:'machineResourceLimit',machine,reason:'Symbol allocation exceeds the selected 16384 identity profile'};
 const registryKey=pending.registryKey;
 if(registryKey!==undefined){
  const registered=allocateJavaScriptRegisteredSymbol(machine.heap,identity,registryKey);
  switch(registered.$){
   case 'symbolRegistryValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(registered.heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,registered.value))};
   case 'symbolRegistryResourceLimit':return {$:'machineResourceLimit',machine,reason:registered.reason};
   case 'symbolRegistryError':
   case 'symbolRegistryInvalid':return {$:'machineInvalid',heap:machine.heap,reason:registered.reason};
   case 'symbolRegistryAllocate':return {$:'machineInvalid',heap:machine.heap,reason:'Symbol registry allocation retains its actual completion'};
  }
 }
 const allocation=registerJavaScriptOrdinarySymbol(machine.heap,identity);
 if(!allocation.success)return {$:'machineInvalid',heap:machine.heap,reason:allocation.error};
 const heap=retainJavaScriptSymbolRegistration(allocation.heap,{$:'symbolRegistrationFact',identity,registered:false});
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,{$:'symbol',identity}))};
}

/** @param {JavaScriptCallMachine} machine @param {number} destination
 * @param {JavaScriptPrimitiveWrapperTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallPrimitiveWrapper(machine,destination,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'primitive wrapping retains its actual source frame'};
 switch(transition.$){
  case 'primitiveWrapperCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'primitiveWrapperAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitPrimitiveWrapper',destination,state:transition.state})),prototype:transition.prototype};
  case 'primitiveWrapperInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'primitiveWrapperResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitPrimitiveWrapper',destination,state:transition.state})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm
 * @param {JavaScriptObjectFromEntriesTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallObjectFromEntries(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Object.fromEntries retains its actual active source frame'};
 switch(transition.$){
  case 'objectFromEntriesContinue':case 'objectFromEntriesCall':case 'objectFromEntriesAllocate':case 'objectFromEntriesPrimitiveGet':case 'objectFromEntriesNeedError':case 'objectFromEntriesResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitObjectFromEntries',destination,realm,state:transition.state}));
   if(transition.$==='objectFromEntriesCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   if(transition.$==='objectFromEntriesAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   if(transition.$==='objectFromEntriesPrimitiveGet')return {$:'machineNeedPrimitiveIteratorGet',machine:next,receiver:transition.receiver,key:transition.key};
   if(transition.$==='objectFromEntriesNeedError')return {$:'machineAllocateError',machine:next,realm,errorClass:transition.errorClass,reason:transition.reason};
   return transition.$==='objectFromEntriesResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'objectFromEntriesCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'objectFromEntriesThrow':return throwJavaScriptCallValue(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),transition.value);
  case 'objectFromEntriesError':return requestJavaScriptCallError(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),realm,transition.errorClass,transition.reason);
  case 'objectFromEntriesInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}

/** Recompute Call setup failures from the actual executable callee. A retained
 * Error marker cannot manufacture a throw for an ordinary callable or change
 * the actual constructor's realm/reason before IteratorClose.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCallFunction} fn
 * @returns {JavaScriptObjectFromEntriesErrorRequest|undefined} */
export function readJavaScriptCallConstructorFailure(heap,fn){
 let reason='';
 if(testJavaScriptErrorConstructorMode(fn.receiverMode))reason='a derived Error constructor requires its actual new operation';
 else if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode))reason=readJavaScriptOrdinaryClassCallReason(heap,fn.identity);
 else if(testJavaScriptCollectionConstructorMode(fn.receiverMode))reason='Constructor '+readJavaScriptCollectionConstructorKind(fn)+" requires 'new'";
 return reason===''?undefined:{$:'objectFromEntriesErrorRequest',realm:fn.realm,errorClass:'TypeError',reason};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {JavaScriptSemanticValue} input @param {boolean} required
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallConstructorCheck(program,machine,destination,input,required){
 const frame=machine.frames[machine.frames.length-1],directory=program.constructorDirectory;if(frame===undefined||directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'constructor checks require an actual active owner and authority'};
 const checked=classifyJavaScriptArraySpeciesConstructor(machine.heap,directory,input);
 if(checked.$==='arrayConstructorUnknown')return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitConstructorCheck',destination,input,required})),constructor:checked.constructor??input};
 const constructable=readJavaScriptKnownArrayConstructorFact(checked)?.constructable===true;
 if(!constructable&&required){const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);return owner===undefined?{$:'machineInvalid',heap:machine.heap,reason:'constructor checks retain their actual source realm'}:requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected value is not a constructor');}
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,required?input:{$:'boolean',value:constructable}))};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptConstructionTransition} transition
 * @param {JavaScriptSemanticValue[]|undefined} [heldArguments] @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallConstruction(program,machine,destination,realm,transition,heldArguments){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'construction retains its actual source frame'};
 const pending=frame.pending;
 /** @type {JavaScriptSemanticValue[] | undefined} */let held=heldArguments;
 if(held===undefined&&pending.$==='waitConstruction')held=pending.heldArguments;
 switch(transition.$){
  case 'constructionContinue':case 'constructionCall':case 'constructionAllocate':case 'constructionNeedAuthority':case 'constructionNeedImplementation':case 'constructionResourceLimit':{
   const state=transition.state;
   const suspension=held===undefined?{$:'waitConstruction',destination,realm,state}:{$:'waitConstruction',destination,realm,state,heldArguments:held.slice()};
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,suspension));
   if(transition.$==='constructionCall')return transition.constructing?pushJavaScriptConstructionBody(program,next,transition.callee,transition.receiver,transition.arguments,transition.newTarget):pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   if(transition.$==='constructionAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   if(transition.$==='constructionNeedAuthority')return {$:'machineNeedConstructorAuthority',machine:next,constructor:transition.constructor};
   if(transition.$==='constructionNeedImplementation')return {$:'machineConstruct',machine:next,constructor:transition.constructor,newTarget:transition.newTarget,arguments:transition.arguments,operation:transition.operation,reason:transition.reason};
   return transition.$==='constructionResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'constructionCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'constructionThrow':return throwJavaScriptCallValue(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),transition.value);
  case 'constructionError':return requestJavaScriptCallError(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),transition.realm??realm,transition.errorClass,transition.reason);
  case 'constructionInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @param {JavaScriptSemanticValue} newTarget
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptConstructionBody(program,machine,callee,receiver,arguments_,newTarget){
 if(callee.$!=='object'||!callee.callable||receiver.$!=='object'||receiver.callable||newTarget.$!=='object'||!newTarget.callable)return {$:'machineInvalid',heap:machine.heap,reason:'construction bodies retain actual target receiver and newTarget identities'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);if(fn===undefined||fn.receiverMode!=='strict'&&fn.receiverMode!=='ordinary class'&&!['Map','Set','WeakMap','WeakSet'].includes(readJavaScriptCollectionConstructorKind(fn)))return {$:'machineInvalid',heap:machine.heap,reason:'construction bodies require actual registered ordinary function, base class or collection code'};
 let slots=fn.registers,count=arguments_.length;for(const frame of machine.frames){slots+=frame.registers.length;count+=frame.arguments.length;for(const list of frame.argumentLists??[])count+=list.values.length;}if(machine.frames.length>=1024||slots>1048576||count>1048576)return {$:'machineResourceLimit',machine,reason:'construction body exceeds shared stack and argument storage bounds'};
 const child={...createJavaScriptCallFrame(fn,receiver,arguments_),newTarget};return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([child])}};
}
