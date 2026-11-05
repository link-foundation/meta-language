import {readJavaScriptCollectionConstructionReceiver} from './javascript-collection-construction.js';
import {readJavaScriptPendingConstruction} from './javascript-construction-parents.js';
import {readJavaScriptDescriptorOperands} from './javascript-descriptor-operands.js';
import {readJavaScriptFunctionText,readJavaScriptFunctionSourceDirectoryError} from './javascript-function-text.js';
import {beginJavaScriptFunctionBind,stepJavaScriptFunctionBind,resumeJavaScriptFunctionBindAllocation,resumeJavaScriptFunctionBindCall,readJavaScriptFunctionBindStateError} from './javascript-function-binding.js';
import {executeJavaScriptSymbolRegistryOperation,allocateJavaScriptRegisteredSymbol,readJavaScriptSymbolRegistrationFact} from './javascript-symbol-registry.js';
import {allocateJavaScriptWeakCollection,executeJavaScriptWeakCollectionOperation} from './javascript-weak-collections.js';
import {readJavaScriptWeakCollectionOperationKind} from './javascript-weak-collection-rules.js';
import {beginJavaScriptArrayConcatenation,stepJavaScriptArrayConcatenation,resumeJavaScriptArrayConcatenationCall,resumeJavaScriptArrayConcatenationConstruction,resumeJavaScriptArrayConcatenationAllocation,readJavaScriptArrayConcatenationStateError} from './javascript-array-concatenation.js';
import {beginJavaScriptArraySlice,stepJavaScriptArraySlice,resumeJavaScriptArraySliceCall,resumeJavaScriptArraySliceConstruction,resumeJavaScriptArraySliceAllocation,readJavaScriptArraySliceStateError} from './javascript-array-slice.js';
import {testJavaScriptArrayCreationOperation,beginJavaScriptArrayCreation,stepJavaScriptArrayCreation,resumeJavaScriptArrayCreationCall,resumeJavaScriptArrayCreationConstruction,resumeJavaScriptArrayCreationAllocation,provideJavaScriptArrayCreationPrimitivePrototype,resumeJavaScriptArrayCreationError,throwJavaScriptArrayCreation,readJavaScriptArrayCreationStateError} from './javascript-array-creation.js';
import {beginJavaScriptConstruction,stepJavaScriptConstruction,resumeJavaScriptConstructionCall,resumeJavaScriptConstructionAllocation,readJavaScriptConstructionStateError} from './javascript-construction.js';
import {beginJavaScriptObjectFromEntries,stepJavaScriptObjectFromEntries,resumeJavaScriptObjectFromEntriesCall,resumeJavaScriptObjectFromEntriesAllocation,provideJavaScriptObjectFromEntriesPrimitivePrototype,resumeJavaScriptObjectFromEntriesError,throwJavaScriptObjectFromEntries,readJavaScriptObjectFromEntriesStateError} from './javascript-object-from-entries.js';
import {beginJavaScriptPrimitiveWrapper,resumeJavaScriptPrimitiveWrapper,readJavaScriptPrimitiveWrapperStateError} from './javascript-primitive-wrappers.js';
import {testJavaScriptCollectionForEachOperation,beginJavaScriptCollectionForEach,stepJavaScriptCollectionForEach,resumeJavaScriptCollectionForEach,readJavaScriptCollectionForEachStateError} from './javascript-collection-for-each.js';
import {testJavaScriptPropertyDefinitionsOperation,beginJavaScriptPropertyDefinitions,stepJavaScriptPropertyDefinitions,resumeJavaScriptPropertyDefinitionsCall,resumeJavaScriptPropertyDefinitionsAllocation,readJavaScriptPropertyDefinitionsStateError} from './javascript-property-definitions.js';
import {testJavaScriptPropertyReflectionOperation,beginJavaScriptPropertyReflection,stepJavaScriptPropertyReflection,resumeJavaScriptPropertyReflectionAllocation,readJavaScriptPropertyReflectionStateError} from './javascript-property-reflection.js';
import {beginJavaScriptInstanceOf,stepJavaScriptInstanceOf,resumeJavaScriptInstanceOf,readJavaScriptInstanceOfStateError} from './javascript-instance-of.js';
import {testJavaScriptObjectObservationOperation,beginJavaScriptObjectObservation,stepJavaScriptObjectIntegrity,readJavaScriptObjectIntegrityStateError} from './javascript-object-integrity.js';
import {testJavaScriptObjectEnumerationOperation,beginJavaScriptObjectEnumeration,stepJavaScriptObjectEnumeration,resumeJavaScriptObjectEnumerationAllocation,resumeJavaScriptObjectEnumerationGetter,readJavaScriptObjectEnumerationStateError} from './javascript-own-enumeration.js';
import {constructJavaScriptPropertyFunctionName,constructJavaScriptSymbolFunctionName} from './javascript-function-names.js';
import {findJavaScriptCallArgumentList,readJavaScriptCallArgumentListsError,openJavaScriptCallArgumentList,appendJavaScriptCallArgument,retainJavaScriptCallArgumentLists} from './javascript-call-argument-lists.js';
import {createJavaScriptPropertyDescriptor,testJavaScriptPropertyDescriptorsSameValue} from './javascript-property-descriptors.js';
import {beginJavaScriptArgumentRest,stepJavaScriptArgumentRest,resumeJavaScriptArgumentRestAllocation,readJavaScriptArgumentRestStateError} from './javascript-argument-rest.js';
import {findJavaScriptIteratorRegion,readJavaScriptIteratorRegionsError,insertJavaScriptIteratorRegion,updateJavaScriptIteratorRegion,removeJavaScriptIteratorRegion,testJavaScriptIteratorRegionSlot} from './javascript-iterator-regions.js';
import {beginJavaScriptSynchronousIterator,beginJavaScriptSynchronousIteratorStep,beginJavaScriptSynchronousIteratorClose,stepJavaScriptIteratorProtocol,resumeJavaScriptIteratorProtocolCall,resumeJavaScriptIteratorPrimitiveGet,provideJavaScriptIteratorPrimitivePrototype,readJavaScriptIteratorProtocolStateError} from './javascript-iterator-protocol.js';
import {allocateJavaScriptStringIterator,beginJavaScriptStringIteratorResult} from './javascript-string-iteration.js';
import {provideJavaScriptArraySpreadPrimitivePrototype,beginJavaScriptArraySpread,stepJavaScriptArraySpread,resumeJavaScriptArraySpread,readJavaScriptArraySpreadStateError} from './javascript-array-spread.js';
import {allocateJavaScriptArrayIterator,beginJavaScriptArrayIteratorNext,stepJavaScriptArrayIteratorNext,resumeJavaScriptArrayIteratorCall,readJavaScriptArrayIteratorStateError,finishJavaScriptArrayIteratorCursor} from './javascript-array-iteration.js';
import {beginJavaScriptSpeciesConstruction,stepJavaScriptSpeciesConstruction,resumeJavaScriptSpeciesConstructionAllocation,resumeJavaScriptSpeciesConstructionCall,readJavaScriptSpeciesConstructionStateError} from './javascript-species-construction.js';
import {classifyJavaScriptArraySpeciesConstructor,readJavaScriptArraySpeciesDirectoryError,findJavaScriptArrayConstructorFact} from './javascript-array-species.js';
import {testJavaScriptArrayTransformOperation,beginJavaScriptArrayTransform,stepJavaScriptArrayTransform,resumeJavaScriptArrayTransformCall,resumeJavaScriptArrayTransformAllocation,resumeJavaScriptArrayTransformConstruction,readJavaScriptArrayTransformStateError} from './javascript-array-transform.js';
import {testJavaScriptArraySearchOperation,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {beginJavaScriptArraySearch,stepJavaScriptArraySearch,resumeJavaScriptArraySearch,readJavaScriptArraySearchStateError} from './javascript-array-search.js';
import {testJavaScriptArrayCallbackOperation,beginJavaScriptArrayCallback,stepJavaScriptArrayCallback,resumeJavaScriptArrayCallback,readJavaScriptArrayCallbackStateError} from './javascript-array-callbacks.js';
import {beginJavaScriptObjectAssignment,beginJavaScriptObjectSpread,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread,readJavaScriptObjectSpreadStateError} from './javascript-object-spread.js';
import {allocateJavaScriptCallClosure,findJavaScriptCallTemplate,projectJavaScriptCallTemplateFunction,readJavaScriptExecutableCallFunction,readJavaScriptCallTemplateDirectoryError} from './javascript-call-templates.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {allocateJavaScriptCollectionIterator,readJavaScriptCollectionIteratorReceiverError} from './javascript-collection-iteration.js';
import {readJavaScriptCollectionIteratorResultPrototypesError,beginJavaScriptArrayIteratorResult,beginJavaScriptCollectionIteratorResult,stepJavaScriptCollectionIteratorResult,resumeJavaScriptCollectionIteratorResultAllocation,readJavaScriptCollectionIteratorResultStateError} from './javascript-collection-iterator-results.js';
import {testJavaScriptCollectionIteratorIntrinsicMode} from './javascript-collection-iterator-call-source.js';
// Numeric-hint continuations retain legacy Number state tags for compatibility.
// Their source instruction distinguishes Number and BigInt conversion semantics.
import {readJavaScriptIntegerProtocolOperationError} from './javascript-call-integer-conversion.js';
import {readJavaScriptTextProtocolOperationError} from './javascript-text-conversion-protocol.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptCallPrimitiveArityError,executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';
import {testJavaScriptCallCodeCoordinate,findJavaScriptCallCodeFunction,readJavaScriptCallCodeRegisters,readJavaScriptCallCodeDestination,testJavaScriptCallStrictEqual} from './javascript-call-code.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition,insertJavaScriptOrdinaryObject,createJavaScriptOrdinarySetDescriptor,setJavaScriptOrdinaryPrototype,deleteJavaScriptOrdinaryProperty,readJavaScriptOrdinaryKeyError} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {beginJavaScriptPropertyDefinition,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition,completeJavaScriptPropertyDefinitionWithSymbol,readJavaScriptPropertyDefinitionStateError} from './javascript-property-definition.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {readJavaScriptNumberProtocolOperationError} from './javascript-number-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {testJavaScriptErrorConstructorMode,testJavaScriptErrorIntrinsicMode,readJavaScriptErrorConstructionKindError,readJavaScriptErrorConstructionOperandsError,readJavaScriptErrorAllocationDirectory,readJavaScriptErrorPriorDirectoryError,readJavaScriptErrorAllocationResultError} from './javascript-error-construction-rules.js';
import {testJavaScriptHostCapabilityMode} from './javascript-host-capability-rules.js';
import {testJavaScriptOrdinaryClassConstructorMode,testJavaScriptOrdinaryClassReceiverResult,readJavaScriptOrdinaryClassCallReason,readJavaScriptOrdinaryClassReceiverError} from './javascript-ordinary-class-rules.js';
import {testJavaScriptCollectionConstructorMode,testJavaScriptCollectionIntrinsicMode,testJavaScriptPrimitiveIntrinsicMode,readJavaScriptCollectionConstructorKind} from './javascript-collection-call-source.js';
import {executeJavaScriptPrimitiveMethod,testJavaScriptPrimitiveMethodOperation} from './javascript-primitive-methods.js';
import {allocateJavaScriptCollection,executeJavaScriptCollectionOperation,copyJavaScriptCollection} from './javascript-collections.js';
import {readJavaScriptCollectionOperationKind} from './javascript-collection-rules.js';
import {allocateJavaScriptArray,testJavaScriptArrayValue,testJavaScriptArrayIntrinsicOperation} from './javascript-arrays.js';
import {testJavaScriptArrayLength} from './javascript-array-objects.js';

/** @typedef {{$:'callCompletion',kind:string,value:JavaScriptSemanticValue,target:number,depth:number}} JavaScriptCallCompletion */
/** @typedef {{$:'iteratorOperation',destination:number,done:number,slot:number,operation:string,realm:bigint}} JavaScriptCallIteratorOperation */
/** @typedef {{$:'callHandler',target:number,destination:number} | {$:'callFinallyHandler',target:number} | {$:'callDeferredCompletion',completion:JavaScriptCallCompletion}} JavaScriptCallHandler */
/** @typedef {JavaScriptCallMachineTransition | {$:'callReturn',machine:JavaScriptCallMachine,value:JavaScriptSemanticValue}} JavaScriptCallCompletionTransition */
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
 /** @type {JavaScriptCallMachineTransition} */
 let current={$:'machineContinue',machine};
 let steps=0,used=0;
 while(steps<limit){
  if(current.$==='machineContinue')current=stepJavaScriptCallMachine(program,current.machine);
  else if(current.$==='machineAllocateObject'){
   const identity=objectIdentities[used];
   if(identity===undefined)return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
   const next=resumeJavaScriptCallObjectAllocation(program,current.machine,identity);
   if(next.$==='machineContinue')used+=1;
   current=next;
  }else return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
  steps+=1;
 }
 if(current.$==='machineContinue')current={$:'machineResourceLimit',machine:current.machine,reason:'execution exhausted its explicit transition budget'};
 return {$:'callExecution',transition:current,steps,objectIdentitiesUsed:used};
}

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
        case 'propertyDefinition':if(!program.realms.includes(instruction.realm)||instruction.operation!==undefined&&!['Object.defineProperty','Reflect.defineProperty'].includes(instruction.operation))return 'a property definition retains its actual intrinsic realm and Object or Reflect policy';break;
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
        case 'argumentListOpen':case 'argumentListAppend':case 'argumentListCall':if(!testJavaScriptIteratorRegionSlot(instruction.slot)||instruction.$==='argumentListCall'&&!['call','collection','error class','ordinary class','construct'].includes(instruction.mode))return 'Call argument Lists retain actual source slots and operation kinds';break;
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
    if(selectedParent?.pending.$==='waitDefinition'&&selectedParent.pending.state.call!==undefined){const call=selectedParent.pending.state.call,error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;}
    if(selectedParent?.pending.$==='waitPropertyDefinitions'){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'bulk definitions retain their actual selected child';const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitInstanceOf'){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'instanceof child retains its actual selected call';const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitObjectEnumeration'){
      const state=selectedParent.pending.state;if(state.phase!=='getter')return 'Object enumeration child retains its actual selected getter';const error=readJavaScriptSelectedCallFrameError(fn,frame,state.callee,state.input,[]);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitIteratorProtocol'){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'iterator region child retains its actual selected callback';const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitArraySpread'){
      const call=selectedParent.pending.state.protocol?.call;if(call===undefined)return 'Array spread child retains its actual selected iterator callback';const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitArrayCreation'){
      const pending=selectedParent.pending,call=pending.construction?.call??pending.state.call;if(call===undefined)return 'an Array creation child retains its actual selected callback or constructor body';const invalid=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(invalid!=='')return invalid;
      if(pending.construction?.call?.constructing&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},pending.construction.call.newTarget))return 'an Array creation constructor retains its actual newTarget';
    }
    if(selectedParent?.pending.$==='waitConstruction'){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'a construction child retains its actual selected call';const invalid=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(invalid!=='')return invalid;
      if(call.constructing&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},call.newTarget))return 'a construction body retains its actual newTarget';
    }
    if(selectedParent?.pending.$==='waitFunctionBind'){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'a binding child retains its actual metadata getter';const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    if(selectedParent?.pending.$==='waitCall'){
      const owner=readJavaScriptExecutableCallFunction(program,heap,selectedParent.identity),instruction=owner?.instructions[selectedParent.position-1];if(instruction?.$==='boundFunctionCall'){
        const bound=heap.objects[findJavaScriptOrdinaryObject(heap.objects,selectedParent.identity)];if(bound?.$!=='boundFunctionObject'||bound.boundArguments.length+selectedParent.arguments.length>1048576)return 'a bound call child retains its actual complete stored and supplied argument Lists';const error=readJavaScriptSelectedCallFrameError(fn,frame,bound.boundTarget,bound.boundThis,bound.boundArguments.concat(selectedParent.arguments));if(error!=='')return error;
      }
    }
    if(selectedParent!==undefined&&(selectedParent.pending.$==='waitObjectFromEntries'||selectedParent.pending.$==='waitCollectionForEach'||selectedParent.pending.$==='waitArrayCallback'||selectedParent.pending.$==='waitArraySearch'||selectedParent.pending.$==='waitArrayTransform'||selectedParent.pending.$==='waitArraySlice'||selectedParent.pending.$==='waitArrayConcatenation'||selectedParent.pending.$==='waitArrayIteratorNext')){
      const call=(selectedParent.pending.$==='waitArrayTransform'||selectedParent.pending.$==='waitArraySlice'||selectedParent.pending.$==='waitArrayConcatenation')&&selectedParent.pending.construction!==undefined?selectedParent.pending.construction.call:selectedParent.pending.state.call;if(call===undefined)return 'an Array operation child retains its actual selected call';
      const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    const spreadParent=machine.frames[index-1];
    if(spreadParent!==undefined&&spreadParent.pending.$==='waitObjectSpread'){
      const state=spreadParent.pending.state;
      if(state.call!==undefined){const error=readJavaScriptSelectedCallFrameError(fn,frame,state.call.callee,state.call.receiver,state.call.arguments);if(error!=='')return error;}
      else {const callee=state.callee,receiver=fn.receiverMode==='lexical'?fn.lexicalReceiver:state.input;if(state.phase!=='getter'||callee.$!=='object'||callee.identity!==frame.identity||frame.arguments.length!==0||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},receiver))return 'an object spread child retains its actual getter, receiver and empty arguments';}
    }
    if(frame.newTarget!==undefined&&fn.receiverMode!=='lexical'){
      const parent=machine.frames[index-1],caller=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity),instruction=caller===undefined||parent===undefined||parent.position===0?undefined:caller.instructions[parent.position-1];
      const generic=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending);
      if(generic!==undefined){if(generic.call?.constructing!==true||!testJavaScriptSemanticSameValue(generic.newTarget,frame.newTarget)||generic.target.$!=='object'||generic.target.identity!==frame.identity)return 'new.target retains its actual selected base construction';}
      else if((parent?.pending.$==='waitArrayTransform'||parent?.pending.$==='waitArraySlice'||parent?.pending.$==='waitArrayConcatenation')&&parent.pending.construction?.phase==='body'){if(!testJavaScriptSemanticSameValue(parent.pending.construction.constructor,frame.newTarget))return 'species new.target retains its actual constructor';}
      else{const legacy=instruction?.$==='callClassConstructor'||instruction?.$==='constructCollection'||instruction?.$==='constructErrorClass'||instruction?.$==='argumentListCall'&&['ordinary class','collection','error class'].includes(instruction.mode);if(!legacy||parent===undefined||!testJavaScriptSemanticSameValue(parent.registers[instruction.callee]??{$:'undefined'},frame.newTarget))return 'an ordinary Call cannot invent a construction new.target';}
    }
    if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||testJavaScriptCollectionConstructorMode(fn.receiverMode)){
      const parent=machine.frames[index-1],caller=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity),instruction=parent===undefined||caller===undefined||parent.position===0?undefined:caller.instructions[parent.position-1];
      const generic=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending);
      if(generic!==undefined){if(!(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||readJavaScriptCollectionConstructorKind(fn)===generic.mode)||generic.phase!=='body'||generic.call?.constructing!==true||generic.target.$!=='object'||generic.target.identity!==frame.identity||!testJavaScriptSemanticSameValue(generic.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(generic.newTarget,frame.newTarget??{$:'undefined'}))return 'a base class or collection retains its actual generic Construct call';}
      else if(parent!==undefined&&(parent.pending.$==='waitArrayTransform'||parent.pending.$==='waitArraySlice'||parent.pending.$==='waitArrayConcatenation')&&parent.pending.construction!==undefined){
        const state=parent.pending.construction;if(!testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||state.phase!=='body'||state.constructor.$!=='object'||state.constructor.identity!==frame.identity||!testJavaScriptSemanticSameValue(state.receiver,frame.registers[0]??{$:'undefined'})||frame.arguments.length!==1||!testJavaScriptSemanticSameValue(frame.arguments[0]??{$:'undefined'},{$:'number',value:state.length}))return 'species source class construction retains its actual pending new operation';
      }else{
      if(parent===undefined||parent.pending.$!=='waitCall'||instruction===undefined||(instruction.$!=='callClassConstructor'&&instruction.$!=='constructCollection'&&!(instruction.$==='argumentListCall'&&['ordinary class','collection'].includes(instruction.mode)))||instruction.destination!==parent.pending.destination)return 'a constructor frame retains its actual suspended source new operation';
      const ordinary=instruction.$==='callClassConstructor'||instruction.$==='argumentListCall'&&instruction.mode==='ordinary class';if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)!==ordinary)return 'a constructor frame retains its actual class or standard collection operation';
      const callee=parent.registers[instruction.callee]??{$:'undefined'},receiver=ordinary?(parent.registers[instruction.receiver]??{$:'undefined'}):{$:'undefined'};
      if(callee.$!=='object'||callee.identity!==frame.identity||!testJavaScriptSemanticSameValue(receiver,frame.registers[0]??{$:'undefined'}))return 'an ordinary class frame retains its actual new callee and receiver';
      if(instruction.$!=='argumentListCall'){if(instruction.arguments.length!==frame.arguments.length)return 'an ordinary class frame retains its actual argument list';for(let at=0;at<instruction.arguments.length;at++)if(!testJavaScriptSemanticSameValue(parent.registers[instruction.arguments[at]??-1]??{$:'undefined'},frame.arguments[at]??{$:'undefined'}))return 'ordinary class frames cannot substitute their actual ordered constructor arguments';}
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
    const joinEntries=fn.instructions.filter(instruction=>instruction.$==='arrayJoinEnter'),joinEntry=joinEntries[0],joinLeave=fn.instructions.findIndex(instruction=>instruction.$==='arrayJoinLeave');
    const entered=joinEntry?.$==='arrayJoinEnter'?frame.registers[joinEntry.destination]:undefined,ownsJoin=joinEntry!==undefined&&frame.position>fn.instructions.indexOf(joinEntry)&&frame.position<=joinLeave&&entered?.$==='boolean'&&entered.value;
    if(ownsJoin&&frame.activeJoin===undefined)return 'a successful unfinished join cannot lose its owned receiver guard';
    if(frame.activeJoin!==undefined){
      const joined=frame.activeJoin;if(joined.$!=='object'||readJavaScriptOrdinaryValueError(heap,joined)!=='')return 'an active join guard retains its actual receiver Object';
      if(joinEntries.length!==1||joinEntry?.$!=='arrayJoinEnter'||!ownsJoin||!testJavaScriptSemanticSameValue(frame.registers[joinEntry.receiver]??{$:'undefined'},joined))return 'an active join guard retains its selected receiver and successful unfinished enter instruction';
    }
    if(frame.newTarget!==undefined&&(frame.newTarget.$!=='object'||!frame.newTarget.callable||readJavaScriptOrdinaryValueError(heap,frame.newTarget)!==''))return 'new.target retains its actual constructor identity';
    if(frame.newTarget!==undefined&&program.constructorDirectory!==undefined){const actual=classifyJavaScriptArraySpeciesConstructor(heap,program.constructorDirectory,frame.newTarget);if(actual.$!=='arrayConstructorKnown'||!actual.fact.constructable)return 'new.target retains its independently registered constructor classification';}
    if(fn.receiverMode==='lexical'&&!testJavaScriptSemanticSameValue(frame.newTarget??{$:'undefined'},fn.lexicalNewTarget??{$:'undefined'}))return 'an arrow frame retains its captured lexical new.target';
    const pending=frame.pending;
    if(!testJavaScriptCallPendingDestination(pending,fn.registers))return 'pending calls retain their actual destination register';
    switch(pending.$){
      case 'waitError':if(!program.realms.includes(pending.realm))return 'pending Error allocation retains its actual intrinsic realm';break;
      case 'waitObjectPrototype':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='objectObservation'||instruction.operation!=='Object.getPrototypeOf'||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.input)||pending.input.$==='object'||pending.input.$==='null'||pending.input.$==='undefined'||suspended)return 'primitive prototype metadata retains its actual source input operation and realm';
        const invalid=readJavaScriptOrdinaryValueError(heap,pending.input);if(invalid!=='')return invalid;break;
      }
      case 'waitPropertyDefinitions':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,state=pending.state,selected=state.operation==='Object.create'?state.prototype:state.target;
        if(instruction?.$!=='propertyDefinitions'||instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.target]??{$:'undefined'},selected)||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},state.input)||!testJavaScriptSemanticSameValue(program.exoticKey,state.exoticKey)||suspended&&state.call===undefined)return 'bulk definitions retain actual source target attributes Symbol and selected child';
        const error=readJavaScriptPropertyDefinitionsStateError(heap,state);if(error!=='')return error;break;
      }
      case 'waitPropertyReflection':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,state=pending.state;
        if(instruction?.$!=='propertyReflection'||instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},state.key)||!testJavaScriptSemanticSameValue(frame.registers[instruction.objectPrototype]??{$:'undefined'},state.objectPrototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.arrayPrototype]??{$:'undefined'},state.arrayPrototype)||suspended)return 'property reflection retains actual source input key realm prototypes and no invented child call';
        const invalid=readJavaScriptPropertyReflectionStateError(heap,state);if(invalid!=='')return invalid;break;
      }
      case 'waitInstanceOf':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,state=pending.state;
        if(instruction?.$!=='instanceOf'||instruction.destination!==pending.destination||instruction.operation!==state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.left]??{$:'undefined'},state.left)||!testJavaScriptSemanticSameValue(frame.registers[instruction.right]??{$:'undefined'},state.right)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},state.key)||suspended&&state.call===undefined)return 'instanceof retains its actual selected source operands Symbol operation and realm';
        const invalid=readJavaScriptInstanceOfStateError(heap,state);if(invalid!=='')return invalid;break;
      }
      case 'waitObjectIntegrity':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='objectObservation'||instruction.destination!==pending.destination||instruction.operation!==pending.state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||suspended)return 'integrity changes retain actual source operation input and no invented child call';
        const invalid=readJavaScriptObjectIntegrityStateError(heap,pending.state);if(invalid!=='')return invalid;break;
      }
      case 'waitObjectEnumeration':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='objectEnumeration'||instruction.destination!==pending.destination||instruction.operation!==pending.state.operation||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.arguments[0]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype))return 'Object enumeration retains its actual source operation input and realm Array prototype';
        const invalid=readJavaScriptObjectEnumerationStateError(heap,pending.state);if(invalid!=='')return invalid;if(suspended&&pending.state.phase!=='getter')return 'Object enumeration child retains its actual getter phase';break;
      }
      case 'waitArrayCreation':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='arrayCreation'||instruction.destination!==pending.destination||pending.realm!==fn.realm||instruction.operation!==pending.state.operation||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},pending.state.receiver)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.arrayPrototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.iteratorKey]??{$:'undefined'},pending.state.iteratorKey)||!testJavaScriptSemanticSameValue(program.exoticKey,pending.state.exoticKey)||frame.arguments.length!==pending.state.arguments.length||frame.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,pending.state.arguments[index]??{$:'undefined'})))return 'Array creation retains its actual receiver arguments roots Symbols and source operation';
        const invalid=readJavaScriptArrayCreationStateError(heap,pending.state);if(invalid!=='')return invalid;
        if(pending.construction!==undefined||pending.fallback!==undefined){const request=stepJavaScriptArrayCreation(heap,pending.state);if(request.$!=='arrayCreationConstruct'||pending.error!==undefined||pending.construction!==undefined&&pending.fallback!==undefined)return 'Array creation retains only its actual selected construction phase';
         if(pending.construction!==undefined){const directory=program.constructorDirectory;if(directory===undefined)return 'Array creation construction retains actual constructor authority';const invalid=readJavaScriptConstructionStateError(program,heap,directory,pending.construction);if(invalid!=='')return invalid;if(!testJavaScriptSemanticSameValue(pending.construction.target,request.constructor)||!testJavaScriptSemanticSameValue(pending.construction.newTarget,request.constructor)||pending.construction.arguments.length!==request.arguments.length||pending.construction.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,request.arguments[index]??{$:'undefined'})))return 'Array creation retains its actual constructor and complete arguments';}
         else{const fallback=pending.fallback;if(fallback?.$!=='arrayCreationArrayFallback'||fallback.length!==request.fallback.length||!testJavaScriptSemanticSameValue(fallback.prototype,request.fallback.prototype)||suspended)return 'Array creation fallback retains its actual ArrayCreate length and prototype';const directory=program.constructorDirectory;if(directory===undefined)return 'Array creation fallback requires actual constructor authority';const classified=classifyJavaScriptArraySpeciesConstructor(heap,directory,request.constructor);if(classified.$==='arrayConstructorUnknown'||classified.$==='arrayConstructorKnown'&&classified.fact.constructable)return 'Array creation fallback cannot replace an actual constructor';}
        }
        const call=pending.construction?.call??pending.state.call;if(suspended&&(call===undefined||pending.error!==undefined))return 'an Array creation child retains only its actual selected call';
        if(pending.error!==undefined){const callee=pending.state.call?.callee,owner=callee?.$==='object'?readJavaScriptExecutableCallFunction(program,heap,callee.identity):undefined,actual=owner===undefined?undefined:readJavaScriptCallConstructorFailure(heap,owner);if(pending.construction!==undefined||pending.fallback!==undefined||actual===undefined||pending.error.$!=='objectFromEntriesErrorRequest'||pending.error.realm!==actual.realm||pending.error.errorClass!==actual.errorClass||pending.error.reason!==actual.reason)return 'Array creation setup failure retains its actual selected callee Error realm and reason';}break;
      }
      case 'waitConstructorCheck':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='constructorCheck'||instruction.destination!==pending.destination||instruction.required!==pending.required||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.input)||suspended)return 'constructor checks retain actual selected input predicate mode and instruction';break;
      }
      case 'waitConstruction':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,directory=program.constructorDirectory;
        if(directory===undefined||instruction===undefined||instruction.$!=='construct'&&!(instruction.$==='argumentListCall'&&instruction.mode==='construct')||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.callee]??{$:'undefined'},pending.state.target)||!testJavaScriptSemanticSameValue(frame.registers[instruction.$==='construct'?instruction.newTarget:instruction.receiver]??{$:'undefined'},pending.state.newTarget))return 'construction retains its actual source target newTarget destination and authority';
        if(instruction.$==='argumentListCall'&&(pending.heldArguments===undefined||pending.heldArguments.length!==pending.state.arguments.length||pending.heldArguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,pending.state.arguments[index]??{$:'undefined'}))))return 'variadic construction retains its consumed immutable argument List proof';
        if(instruction.$==='construct'&&(instruction.arguments.length!==pending.state.arguments.length||instruction.arguments.some((coordinate,index)=>!testJavaScriptSemanticSameValue(frame.registers[coordinate]??{$:'undefined'},pending.state.arguments[index]??{$:'undefined'}))))return 'construction retains every actual held ordered argument';
        const invalid=readJavaScriptConstructionStateError(program,heap,directory,pending.state);if(invalid!=='')return invalid;
        if(suspended&&pending.state.call===undefined)return 'a construction child retains its actual selected getter conversion or body call';break;
      }
      case 'waitFunctionBind':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||instruction?.$!=='functionBind'||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(pending.state.target,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.boundThis,frame.arguments[0]??{$:'undefined'})||pending.state.arguments.length!==Math.max(0,frame.arguments.length-1)||pending.state.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index+1]??{$:'undefined'})))return 'binding retains its actual intrinsic target receiver and complete argument prefix';
        const error=readJavaScriptFunctionBindStateError(heap,pending.state);if(error!=='')return error;if(suspended&&pending.state.call===undefined)return 'binding child frames retain their actual selected metadata getter';break;
      }
      case 'waitFunctionText':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,request=readJavaScriptFunctionText(program,heap,pending.input);if(instruction?.$!=='functionToString'||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptSemanticSameValue(pending.input,frame.registers[0]??{$:'undefined'})||request.$!=='functionTextNeedAuthority'||request.kind!==pending.kind||suspended)return 'function text retains its actual intrinsic receiver realm source category and external metadata request';break;
      }
      case 'waitObjectFromEntries':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='objectFromEntries'||instruction.destination!==pending.destination||pending.realm!==fn.realm||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype)||!testJavaScriptSemanticSameValue(frame.registers[instruction.iteratorKey]??{$:'undefined'},pending.state.iteratorKey)||!testJavaScriptSemanticSameValue(program.exoticKey,pending.state.exoticKey))return 'Object.fromEntries retains its actual source input realm prototype and well-known Symbols';
        const invalid=readJavaScriptObjectFromEntriesStateError(heap,pending.state);if(invalid!=='')return invalid;
        if(suspended&&(pending.state.call===undefined||pending.error!==undefined))return 'Object.fromEntries child retains its actual selected call rather than an allocation request';
        if(pending.error!==undefined){const callee=pending.state.call?.callee,owner=callee?.$==='object'?readJavaScriptExecutableCallFunction(program,heap,callee.identity):undefined,actual=owner===undefined?undefined:readJavaScriptCallConstructorFailure(heap,owner);if(actual===undefined||pending.error.$!=='objectFromEntriesErrorRequest'||pending.error.realm!==actual.realm||pending.error.errorClass!==actual.errorClass||pending.error.reason!==actual.reason)return 'Object.fromEntries selected-call failure retains its actual callee constructor kind Error realm and reason';}break;
      }
      case 'waitPrimitiveWrapper':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='primitiveWrapper'||instruction.destination!==pending.destination||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.state.input)||!testJavaScriptSemanticSameValue(frame.registers[instruction.prototype]??{$:'undefined'},pending.state.prototype)||suspended)return 'primitive wrappers retain their actual selected input prototype and allocation instruction';
        const invalid=readJavaScriptPrimitiveWrapperStateError(heap,pending.state);if(invalid!=='')return invalid;break;
      }
      case 'waitSymbolAllocation':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        const registry=instruction?.$==='symbolRegistry'&&instruction.operation==='for';
        if(instruction?.$!=='symbolAllocate'&&!registry||registry&&(pending.registryKey===undefined||!['text','sourceText'].includes(pending.description.$)||!testJavaScriptSemanticSameValue(pending.registryKey,pending.description))||!registry&&pending.registryKey!==undefined||instruction.destination!==pending.destination||!['undefined','text','sourceText'].includes(pending.description.$)||readJavaScriptOrdinaryValueError(heap,pending.description)!==''||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.description)||suspended)return 'Symbol allocation retains its actual converted optional description and selected instruction';break;
      }
      case 'waitSymbolDescription':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='symbolDescription'||instruction.destination!==pending.destination||pending.symbol.$!=='symbol'||readJavaScriptOrdinaryValueError(heap,pending.symbol)!==''||!testJavaScriptSemanticSameValue(frame.registers[instruction.input]??{$:'undefined'},pending.symbol)||suspended)return 'Symbol description metadata retains its actual selected Symbol input and instruction';break;
      }
      case 'waitFunctionNameSymbol':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='functionName'||instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.prefix!==pending.prefix||pending.key.$!=='symbol'||pending.object.$!=='object'||!pending.object.callable||!testJavaScriptSemanticSameValue(frame.registers[instruction.object]??{$:'undefined'},pending.object)||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},pending.key))return 'Symbol name metadata retains its actual selected function property key prefix and instruction';
        const invalid=readJavaScriptOrdinaryValueError(heap,pending.key)||readJavaScriptOrdinaryValueError(heap,pending.object);if(invalid!=='')return invalid;break;
      }
      case 'waitSourceError':{
        if(!program.realms.includes(pending.realm)||fn.realm!==pending.realm)return 'pending source Error allocation retains its actual source realm';
        if((pending.message.$!=='text'&&pending.message.$!=='sourceText')||readJavaScriptOrdinaryValueError(heap,pending.message)!==''||typeof pending.messagePresent!=='boolean')return 'pending source Error retains exact UTF16 message and Boolean presence';
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined)return 'pending source Error retains its actual source instruction and arguments';
        switch(instruction.$){
          case 'primitive':{
            if(readJavaScriptSourceExceptionClass(instruction.operation)!==pending.errorClass||pending.errorClass===''||instruction.arguments.length!==2)return 'pending source Error retains its actual source instruction and arguments';
            const message=frame.registers[instruction.arguments[0]?? -1]??{$:'undefined'},present=frame.registers[instruction.arguments[1]?? -1]??{$:'undefined'};
            if(!testJavaScriptSemanticSameValue(message,pending.message))return 'pending source Error retains its actual source instruction and arguments';
            if(present.$!=='boolean')return 'pending source Error retains its actual source instruction and arguments';
            if(present.value!==pending.messagePresent)return 'pending source Error retains its actual source instruction and arguments';
            break;
          }
          default:return 'pending source Error retains its actual source instruction and arguments';
        }
        break;
      }
      case 'waitErrorValue':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='constructErrorValue')return 'Error value resumption retains its actual constructor instruction';
        if(instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.errorClass!==pending.errorClass||instruction.bindReceiver!==pending.bindReceiver)return 'Error value resumption retains its actual destination, realm and constructor role';
        const constructor=frame.registers[instruction.constructor]??{$:'undefined'},newTarget=frame.registers[instruction.newTarget]??{$:'undefined'},prototype=frame.registers[instruction.prototype]??{$:'undefined'},message=frame.registers[instruction.message]??{$:'undefined'},present=frame.registers[instruction.present]??{$:'undefined'};
        const error=readJavaScriptErrorConstructionOperandsError(heap,constructor,newTarget,prototype,message,present);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(constructor,pending.constructor)||!testJavaScriptSemanticSameValue(newTarget,pending.newTarget)||!testJavaScriptSemanticSameValue(prototype,pending.prototype)||!testJavaScriptSemanticSameValue(message,pending.message))return 'an Error allocation cannot substitute its actual constructor operands';
        if(present.$!=='boolean'||present.value!==pending.messagePresent)return 'an Error allocation retains actual source presence';
        const prior=readJavaScriptErrorPriorDirectoryError(heap,pending.beforeDirectory);if(prior!=='')return prior;
        break;
      }
      case 'waitErrorIntrinsic':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(!testJavaScriptErrorIntrinsicMode(fn.receiverMode)||instruction===undefined||instruction.$!=='invokeErrorIntrinsic'||instruction.destination!==pending.destination)return 'an external Error callback retains its actual registered source instruction';
        break;
      }
      case 'waitHostCapability':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(!testJavaScriptHostCapabilityMode(fn.receiverMode)||instruction===undefined||instruction.$!=='invokeHostCapability'||instruction.destination!==pending.destination)return 'an external capability retains its actual qualified source instruction';
        const prior=readJavaScriptErrorPriorDirectoryError(heap,pending.beforeDirectory);if(prior!=='')return prior;
        break;
      }
      case 'waitArrayConcatenation':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='arrayConcatenation'||instruction.destination!==pending.destination||pending.realm!==fn.realm)return 'Array concatenation retains actual source instruction destination and realm';
        const invalid=readJavaScriptArrayConcatenationStateError(heap,instruction.species,pending.state);if(invalid!=='')return invalid;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.spreadableKey,frame.registers[instruction.spreadableKey]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey)||pending.state.arguments.length!==frame.arguments.length||pending.state.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index]??{$:'undefined'})))return 'Array concatenation retains original receiver arguments and Symbols';
        if(pending.construction!==undefined){if(pending.state.phase!=='construct'||pending.state.constructor===undefined||!testJavaScriptSemanticSameValue(pending.construction.constructor,pending.state.constructor)||pending.construction.length!==0)return 'Array concatenation retains its actual zero-length species construction';const invalid=readJavaScriptSpeciesConstructionStateError(program,heap,instruction.species,pending.construction);if(invalid!=='')return invalid;}
        if(suspended&&(pending.construction!==undefined?pending.construction.call===undefined:pending.state.call===undefined))return 'Array concatenation child retains its actual selected callback';break;
      }
      case 'waitArraySlice':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='arraySlice'||instruction.destination!==pending.destination||pending.realm!==fn.realm)return 'Array slice retains actual source instruction destination and realm';
        const invalid=readJavaScriptArraySliceStateError(heap,instruction.species,pending.state);if(invalid!=='')return invalid;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.start,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.end,frame.arguments[1]??{$:'undefined'})||pending.state.endPresent!==(frame.arguments.length>1)||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array slice retains original receiver bounds argument presence and Symbol';
        if(pending.construction!==undefined){if(pending.state.phase!=='construct'||pending.state.constructor===undefined||!testJavaScriptSemanticSameValue(pending.construction.constructor,pending.state.constructor)||pending.construction.length!==pending.state.count)return 'Array slice retains its actual selected species construction';const invalid=readJavaScriptSpeciesConstructionStateError(program,heap,instruction.species,pending.construction);if(invalid!=='')return invalid;}
        if(suspended&&(pending.construction!==undefined?pending.construction.call===undefined:pending.state.call===undefined))return 'Array slice child retains its actual selected callback';break;
      }
      case 'waitArrayTransform':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='arrayTransform'||instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return 'Array transform continuation retains its actual source operation, destination and realm';
        const error=readJavaScriptArrayTransformStateError(heap,instruction.species,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array transforms cannot substitute actual source, callback, thisArg or conversion Symbol';
        if(pending.construction!==undefined){
          if(pending.state.phase!=='construct'||pending.state.constructor===undefined||pending.state.constructorFact===undefined||!testJavaScriptSemanticSameValue(pending.construction.constructor,pending.state.constructor)||pending.construction.length!==(pending.state.operation==='Array.map'?pending.state.length:0))return 'nested species construction retains its actual transform selection';
          const error=readJavaScriptSpeciesConstructionStateError(program,heap,instruction.species,pending.construction);if(error!=='')return error;
        }
        if(suspended&&(pending.construction!==undefined?pending.construction.call===undefined:pending.state.call===undefined))return 'Array transform child frames retain their actual selected call';break;
      }
      case 'waitArraySearch':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='arraySearch'||instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return 'Array search continuation retains its actual source operation, destination and realm';
        const error=readJavaScriptArraySearchStateError(heap,pending.state);if(error!=='')return error;
        const at=instruction.operation==='Array.at',search=at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},from=frame.arguments[at?0:1]??{$:'undefined'},present=frame.arguments.length>=(at?1:2);
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.searchElement,search)||!testJavaScriptSemanticSameValue(pending.state.fromIndex,from)||pending.state.fromPresent!==present||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array search cannot substitute its actual receiver, search value, index argument, presence or Symbol';
        if(suspended&&pending.state.call===undefined)return 'only a selected Array search getter or conversion can retain a child frame';
        break;
      }
      case 'waitCollectionForEach':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction?.$!=='collectionForEach'||instruction.destination!==pending.destination||pending.realm!==fn.realm||instruction.operation!==pending.state.operation)return 'collection callback continuation retains its actual operation destination and realm';
        const error=readJavaScriptCollectionForEachStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'}))return 'collection callbacks retain their actual receiver callback and thisArgument';
        if(suspended&&pending.state.call===undefined)return 'a collection callback child retains its actual selected call';break;
      }
      case 'waitArrayCallback':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='arrayCallback'||instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return 'Array callback continuation retains its actual source operation, destination and realm';
        const error=readJavaScriptArrayCallbackStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[instruction.input??0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey)||pending.state.initialPresent!==(frame.arguments.length>=2))return 'Array callbacks cannot substitute their receiver, callback, thisArg or well-known Symbol';
        if(suspended&&pending.state.call===undefined)return 'only an actual Array getter, conversion or callback can retain a child frame';
        break;
      }
      case 'waitObjectSpread':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='objectSpread'||instruction.realm!==pending.realm||instruction.destination!==pending.destination||!program.realms.includes(pending.realm))return 'object spread retains its actual source instruction, destination and realm';
        if(pending.state.operation!==instruction.operation||instruction.operation==='assignment'&&!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'object copy retains its actual CreateDataProperty or Set operation and Symbol';
        const error=readJavaScriptObjectSpreadStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.target,frame.registers[instruction.target]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.input,frame.registers[instruction.input]??{$:'undefined'}))return 'object spread cannot substitute its actual target or source input';
        const excluded=instruction.excluded??[];if(excluded.length!==pending.state.excluded.length)return 'object copy retains every actual excluded-key operand';for(let at=0;at<excluded.length;at++)if(!testJavaScriptSemanticSameValue(pending.state.excluded[at]??{$:'undefined'},frame.registers[excluded[at]??-1]??{$:'undefined'}))return 'object copy cannot substitute an excluded key';
        if(suspended&&pending.state.phase!=='getter'&&pending.state.call===undefined)return 'object copy retains only actual selected getters setters and numeric callbacks';
        break;
      }
      case 'waitArrayAssignment':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(fn.realm!==pending.realm||instruction===undefined||instruction.$!=='ordinarySet'||instruction.destination!==pending.destination)return 'Array assignment retains its actual source Set instruction, destination and realm';
        const error=readJavaScriptArrayAssignmentStateError(heap,pending.state);if(error!=='')return error;
        const receiver=frame.registers[instruction.receiver]??{$:'undefined'},definition=pending.state.definition;
        if(receiver.$!=='object'||receiver.identity!==definition.identity||!testJavaScriptSemanticSameValue(definition.key,frame.registers[instruction.key]??{$:'undefined'})||!testJavaScriptSemanticSameValue(definition.descriptor.value,frame.registers[instruction.value]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array assignment cannot substitute its receiver, key, value or well-known Symbol';
        const descriptor=definition.descriptor;
        if(!descriptor.valuePresent||descriptor.writablePresent||descriptor.enumerablePresent||descriptor.configurablePresent||descriptor.getterPresent||descriptor.setterPresent)return 'Array assignment retains its actual value-only ordinary Set descriptor';
        if(suspended&&pending.state.$!=='arrayAssignmentNumber')return 'only a pending Array numeric callback can have a suspended child call';
        break;
      }
      case 'waitArgumentRest':{
        const instruction=frame.position===0?undefined:fn.instructions[frame.position-1];if(instruction?.$!=='argumentRest'||instruction.destination!==pending.destination||instruction.realm!==pending.realm||instruction.from!==pending.state.from||!testJavaScriptSemanticSameValue(pending.state.prototype,frame.registers[instruction.prototype]??{$:'undefined'}))return 'argument rest retains its actual source instruction and prototype';
        const error=readJavaScriptArgumentRestStateError(heap,frame.arguments,pending.state);if(error!=='')return error;if(suspended)return 'argument copying cannot hide a selected source callback';break;
      }
      case 'waitIteratorProtocol':{
        const instruction=frame.position===0?undefined:fn.instructions[frame.position-1];
        if(instruction?.$!=='iteratorProtocol'||instruction.destination!==pending.destination||instruction.done!==pending.done||instruction.slot!==pending.slot||instruction.realm!==pending.realm||instruction.operation!==pending.operation||pending.state.operation!==pending.operation)return 'iterator work retains its actual source instruction and region';
        const error=readJavaScriptIteratorProtocolStateError(heap,pending.state);if(error!=='')return error;if(pending.operation==='step'&&(pending.state.readValue??true)!==(instruction.readValue??true))return 'iterator advancement retains its actual value-versus-elision choice';
        if(pending.operation==='acquire'){
          if(!testJavaScriptSemanticSameValue(pending.state.input,frame.registers[instruction.input]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.key,frame.registers[instruction.iteratorKey]??{$:'undefined'}))return 'iterator acquisition retains its held source operand and key';
        }else{
          const regions=frame.iterators??[],region=regions[findJavaScriptIteratorRegion(regions,pending.slot)];if(region===undefined||!testJavaScriptSemanticSameValue(region.record.iterator,pending.state.record.iterator)||!testJavaScriptSemanticSameValue(region.record.nextMethod,pending.state.record.nextMethod)||region.record.done!==pending.state.record.done)return 'iterator continuation retains its actual cached region record';
          if(pending.operation==='close'){const completion=readJavaScriptCallIteratorCloseCompletion(frame);if(completion.kind!==pending.state.completion.kind||!testJavaScriptSemanticSameValue(completion.value,pending.state.completion.value))return 'iterator cleanup retains the actual deferred completion';}
        }
        if(suspended&&pending.state.call===undefined)return 'iterator child retains its actual selected getter or method';break;
      }
      case 'waitArraySpread':{
        const instruction=frame.position===0?undefined:fn.instructions[frame.position-1];
        if(instruction?.$!=='arraySpread'||instruction.destination!==pending.destination||instruction.realm!==pending.realm||pending.realm!==fn.realm)return 'Array spread retains its actual source instruction, destination and realm';
        const error=readJavaScriptArraySpreadStateError(heap,pending.state);if(error!=='')return error;
        for(const [operand,value]of [[instruction.target,pending.state.target],[instruction.input,pending.state.input],[instruction.iteratorKey,pending.state.iteratorKey]])if(!testJavaScriptSemanticSameValue(frame.registers[operand]??{$:'undefined'},value))return 'Array spread retains its actual target, source and iterator key';
        const index=frame.registers[instruction.index]??{$:'undefined'};if(index.$!=='number'||!Number.isSafeInteger(index.value)||index.value<0||pending.state.index<index.value)return 'Array spread retains its original exact accumulation coordinate';
        if(suspended&&pending.state.protocol?.call===undefined)return 'Array spread child retains its actual selected iterator call';break;
      }
      case 'waitArrayIteratorNext':{
        const instruction=fn.instructions[frame.position];
        if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||fn.realm!==pending.realm||instruction?.$!=='collectionIteratorResult'||instruction.kind!=='Array'||instruction.destination!==pending.destination)return 'Array next continuation retains its actual instruction, operation and realm';
        const error=readJavaScriptArrayIteratorStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array next retains its actual receiver and conversion Symbol';
        if(suspended&&pending.state.call===undefined)return 'Array next child retains its actual selected getter or conversion';break;
      }
      case 'waitCollectionIteratorResult':{
        const instruction=fn.instructions[frame.position];
        if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||fn.realm!==pending.realm||instruction===undefined||instruction.$!=='collectionIteratorResult'||instruction.destination!==pending.destination)return 'iterator result continuation retains its actual source instruction, destination and realm';
        const error=readJavaScriptCollectionIteratorResultStateError(heap,pending.state);if(error!=='')return error;
        if(pending.state.collectionKind!==instruction.kind||!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'}))return 'iterator result continuation cannot replace its actual receiver or operation brand';
        if(!testJavaScriptSemanticSameValue(pending.state.arrayPrototype,frame.registers[instruction.arrayPrototype]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.objectPrototype,frame.registers[instruction.objectPrototype]??{$:'undefined'}))return 'iterator result continuation cannot substitute its actual realm prototypes';
        if(suspended)return 'iterator result allocation cannot hide a pending source child call';
        break;
      }
      case 'waitKey':if(!program.realms.includes(pending.realm))return 'pending key conversion retains its actual intrinsic realm';break;
      case 'waitDefinition':{
        if(!program.realms.includes(pending.realm))return 'pending property definition retains its actual intrinsic realm';const invalid=readJavaScriptPropertyDefinitionStateError(heap,pending.state);if(invalid!=='')return invalid;
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined,target=instruction?.$==='propertyDefinition'||instruction?.$==='ordinaryDefine'?frame.registers[instruction.target]:instruction?.$==='ordinaryDataProperty'||instruction?.$==='literalAccessor'?frame.registers[instruction.object]:undefined;
        if(target===undefined||!testJavaScriptSemanticSameValue(target,pending.state.target))return 'property definition retains its actual selected source target';
        const operation=instruction?.$==='ordinaryDefine'?'Reflect.defineProperty':instruction?.$==='propertyDefinition'?instruction.operation:undefined;if((operation??'Object.defineProperty')!==(pending.state.operation??'Object.defineProperty')||instruction?.destination!==pending.destination||instruction?.realm!==pending.realm)return 'property definition retains its actual source result policy destination and realm';
        if(instruction?.$==='ordinaryDefine'){const operands=readJavaScriptDescriptorOperands(instruction.fields.map(coordinate=>frame.registers[coordinate]??{$:'undefined'})),descriptor=pending.state.$==='propertyDefinitionArray'?pending.state.assignment.definition.descriptor:pending.state.$==='propertyDefinitionReady'?pending.state.descriptor:undefined,key=pending.state.$==='propertyDefinitionArray'||pending.state.$==='propertyDefinitionReady'?pending.state.key:undefined;if(operands.error!==''||descriptor===undefined||key===undefined||!testJavaScriptSemanticSameValue(frame.registers[instruction.key]??{$:'undefined'},key)||!testJavaScriptPropertyDescriptorsSameValue(descriptor,operands.descriptor))return 'internal definitions retain their actual key and complete preconverted descriptor operands';}
        if(suspended&&(pending.state.$==='propertyDefinitionReady'||pending.state.operation==='Reflect.defineProperty'&&pending.state.call===undefined))return 'a definition suspension retains its actual selected child rather than a completed descriptor';
        if(pending.state.$==='propertyDefinitionArray'&&!testJavaScriptSemanticSameValue(pending.state.assignment.exoticKey,program.exoticKey))return 'Array definition retains its actual primitive conversion Symbol';
        if(pending.state.$==='propertyDefinitionKey'&&!testJavaScriptSemanticSameValue(pending.state.conversion.exoticKey,program.exoticKey)||pending.state.$==='propertyDefinitionDescriptor'&&pending.state.exoticKey!==undefined&&!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'property definition retains its actual primitive conversion authority';
        if(suspended&&pending.state.$==='propertyDefinitionArray'&&pending.state.assignment.$!=='arrayAssignmentNumber')return 'Array definition child retains its actual numeric callback phase';break;
      }
      case 'waitNumber':{
        if(!program.realms.includes(pending.realm))return 'pending number conversion retains its actual intrinsic realm';
        const numberState=pending.state;
        const stateError=readJavaScriptNumberCallStateError(heap,numberState);if(stateError!=='')return stateError;
        if(readJavaScriptNumberCallSourceOperation(fn,frame,pending.destination,pending.realm,program.exoticKey,numberState)==='')return 'pending number conversion retains its actual source instruction, receiver, Symbol and realm';
        if(suspended){
          switch(numberState.$){
            case 'numberPrimitiveValue':return 'a completed numeric primitive cannot have a suspended child call';
            case 'numberProtocolState':break;
          }
        }
        break;
      }
      case 'waitText':{
        if(!program.realms.includes(pending.realm))return 'pending text conversion retains its actual intrinsic realm';
        const textState=pending.state;
        const stateError=readJavaScriptTextCallStateError(heap,textState);if(stateError!=='')return stateError;
        if(readJavaScriptTextCallSourceOperation(fn,frame,pending.destination,pending.realm,program.exoticKey,textState)==='')return 'pending text conversion retains its actual source instruction, receiver, Symbol and realm';
        if(suspended){
          switch(textState.$){
            case 'textPrimitiveValue':return 'a completed string primitive cannot have a suspended child call';
            case 'textProtocolState':break;
          }
        }
        break;
      }
      default:break;
    }
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
 const transition=unwindJavaScriptCallCompletion(machine,{$:'callCompletion',kind:'throw',value,target:-1,depth:0});return transition.$==='callReturn'?{$:'machineInvalid',heap:transition.machine.heap,reason:'throw unwinding cannot become a return'}:transition;
}
/** @param {JavaScriptCallMachine} machine @param {JavaScriptCallCompletion} completion @returns {JavaScriptCallCompletionTransition} */
export function unwindJavaScriptCallCompletion(machine,completion){
 if(!['return','throw','jump'].includes(completion.kind)||readJavaScriptOrdinaryValueError(machine.heap,completion.value)!==''||!Number.isSafeInteger(completion.depth)||Object.is(completion.depth,-0)||completion.depth<0||completion.depth>1024||completion.kind==='jump'&&(!Number.isSafeInteger(completion.target)||completion.target<0||completion.target>=65536||Object.is(completion.target,-0))||completion.kind!=='jump'&&(completion.target!==-1||completion.depth!==0))return {$:'machineInvalid',heap:machine.heap,reason:'completion unwinding requires actual values and exact bounded control coordinates'};
 let frames=machine.frames,heap=machine.heap;
 while(frames.length>0){
  let frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'completion unwinding retains its complete actual stack'};
  if(completion.kind==='throw'&&frame.pending.$==='waitArrayCreation'&&frame.pending.state.call!==undefined&&frame.pending.construction===undefined){
   const pending=frame.pending,resumed=resumeJavaScriptArrayCreationCall(heap,pending.state,{$:'throwCompletion',value:completion.value});
   if(resumed.$==='arrayCreationThrow'){heap=resumed.heap;frame={...frame,pending:{$:'callIdle'}};frames=frames.slice(0,-1).concat([frame]);completion={...completion,value:resumed.value};}
   else if(resumed.$==='arrayCreationInvalid')return {$:'machineInvalid',heap:resumed.heap,reason:resumed.reason};
   else if(resumed.$==='arrayCreationContinue'||resumed.$==='arrayCreationCall'||resumed.$==='arrayCreationNeedError'||resumed.$==='arrayCreationAllocate'||resumed.$==='arrayCreationPrimitiveGet'||resumed.$==='arrayCreationConstruct'||resumed.$==='arrayCreationResourceLimit'){const next=replaceJavaScriptCallFrame(resumed.heap,frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCreation',destination:pending.destination,realm:pending.realm,state:resumed.state}));return resumed.$==='arrayCreationResourceLimit'?{$:'machineResourceLimit',machine:next,reason:resumed.reason}:{$:'machineContinue',machine:next};}
   else return {$:'machineInvalid',heap:resumed.heap,reason:'Array creation throw retains actual cleanup or final completion'};
  }
  if(completion.kind==='throw'&&frame.pending.$==='waitObjectFromEntries'&&frame.pending.state.call!==undefined){
   const pending=frame.pending,resumed=resumeJavaScriptObjectFromEntriesCall(heap,pending.state,{$:'throwCompletion',value:completion.value});
   if(resumed.$==='objectFromEntriesThrow'){heap=resumed.heap;frame={...frame,pending:{$:'callIdle'}};frames=frames.slice(0,-1).concat([frame]);completion={...completion,value:resumed.value};}
   else if(resumed.$==='objectFromEntriesInvalid')return {$:'machineInvalid',heap:resumed.heap,reason:resumed.reason};
   else if(resumed.$==='objectFromEntriesContinue'||resumed.$==='objectFromEntriesCall'||resumed.$==='objectFromEntriesNeedError'||resumed.$==='objectFromEntriesAllocate'||resumed.$==='objectFromEntriesPrimitiveGet'||resumed.$==='objectFromEntriesResourceLimit'){
    const next=replaceJavaScriptCallFrame(resumed.heap,frames,replaceJavaScriptCallPending(frame,{$:'waitObjectFromEntries',destination:pending.destination,realm:pending.realm,state:resumed.state}));return resumed.$==='objectFromEntriesResourceLimit'?{$:'machineResourceLimit',machine:next,reason:resumed.reason}:{$:'machineContinue',machine:next};
   }else return {$:'machineInvalid',heap:resumed.heap,reason:'Object.fromEntries throw resumes its actual cleanup or final abrupt completion'};
  }
  if(completion.kind==='throw'&&frame.pending.$==='waitIteratorProtocol'&&frame.pending.state.call!==undefined){
   const resolved=resumeJavaScriptCallIteratorThrow({$:'callMachine',heap,frames},completion.value);if(resolved.$==='iteratorThrowTransition')return resolved.transition;frames=resolved.machine.frames;heap=resolved.machine.heap;completion={...completion,value:resolved.value};frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'iterator throw retains its actual caller frame'};
  }
  let handlers=frame.handlers;
  if(completion.kind==='jump'&&completion.depth>handlers.length)return {$:'machineInvalid',heap,reason:'completion jump cannot invent an enclosing handler depth'};
  while(handlers.length>(completion.kind==='jump'?completion.depth:0)){
   const handler=handlers[handlers.length-1];handlers=handlers.slice(0,-1);if(handler===undefined)return {$:'machineInvalid',heap,reason:'completion handler storage remains dense'};
   if(handler.$==='callDeferredCompletion')continue;
   if(handler.$==='callFinallyHandler')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...frame,argumentLists:retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),position:handler.target,pending:{$:'callIdle'},handlers:handlers.concat([{$:'callDeferredCompletion',completion:{...completion}}])})};
   if(completion.kind==='throw')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...writeJavaScriptCallRegister(frame,handler.destination,completion.value),argumentLists:retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),position:handler.target,handlers})};
  }
  const argumentLists=retainJavaScriptCallArgumentLists(frame.argumentLists??[],handlers.length),current=replaceJavaScriptCallFrame(heap,frames,{...frame,handlers,argumentLists});
  if(completion.kind==='jump')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(heap,frames,{...frame,position:completion.target,pending:{$:'callIdle'},handlers,argumentLists})};
  if(completion.kind==='return')return {$:'callReturn',machine:current,value:completion.value};
  if(frame.pending.$==='waitArrayIteratorNext')heap=finishJavaScriptArrayIteratorCursor(heap,frame.pending.state,true);
  frames=frames.slice(0,-1);
 }
 return completion.kind==='throw'?{$:'machineThrow',heap,value:completion.value}:{$:'machineInvalid',heap,reason:'return/control completion requires its actual active frame'};
}
/** @param {JavaScriptCallMachine} machine @param {bigint} realm @param {string} errorClass @param {string} reason @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallError(machine,realm,errorClass,reason){
  let frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error allocation requires its actual active frame'};
  if(frame.pending.$==='waitArrayCreation'&&frame.pending.construction===undefined&&frame.pending.state.call!==undefined){const pending={...frame.pending,error:{$:'objectFromEntriesErrorRequest',realm,errorClass,reason}},next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,pending));return {$:'machineAllocateError',machine:next,realm,errorClass,reason};}
  if(frame.pending.$==='waitObjectFromEntries'&&frame.pending.state.call!==undefined){
   const pending={...frame.pending,error:{$:'objectFromEntriesErrorRequest',realm,errorClass,reason}},next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,pending));return {$:'machineAllocateError',machine:next,realm,errorClass,reason};
  }
  if(frame.pending.$==='waitIteratorProtocol'){
    const pending=frame.pending,regions=frame.iterators??[],update=pending.operation==='close'?removeJavaScriptIteratorRegion(machine.heap,regions,pending.slot):pending.operation==='step'?updateJavaScriptIteratorRegion(machine.heap,regions,pending.slot,{...pending.state.record,done:true}):{$:'iteratorRegionsUpdated',regions};
    if(update.$!=='iteratorRegionsUpdated')return {$:'machineInvalid',heap:machine.heap,reason:'iterator intrinsic failure retains its actual source region'};
    frame={...frame,iterators:update.regions,pending:{$:'callIdle'}};machine=replaceJavaScriptCallFrame(machine.heap,machine.frames,frame);
    if(pending.operation==='close'&&pending.state.completion.kind==='throw')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallIteratorResult(frame,pending.destination,pending.done,{$:'undefined'},true))};
  }
  const heap=frame.pending.$==='waitArrayIteratorNext'?finishJavaScriptArrayIteratorCursor(machine.heap,frame.pending.state,true):machine.heap;
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
 const frame=machine.frames[machine.frames.length-1],pending=frame?.pending;if(frame===undefined||pending?.$!=='waitFunctionNameSymbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol function naming requires its actual suspended source operation'};
 const named=constructJavaScriptSymbolFunctionName(description,pending.prefix);if(named.$==='functionNameInvalid')return {$:'machineInvalid',heap:machine.heap,reason:named.reason};if(named.$==='functionNameResourceLimit')return {$:'machineResourceLimit',machine,reason:named.reason};if(named.$!=='functionNameValue')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol name metadata must complete actual text assembly'};
 return finishJavaScriptCallFunctionName(program,machine,pending.destination,pending.object,pending.realm,named.value);
}

/** Independently supplied native/source metadata responds only to its actual
 * suspended receiver/category. Text itself grants no execution authority.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallFunctionText(program,machine,value){
 const error=readJavaScriptCallMachineError(program,machine)||readJavaScriptOrdinaryValueError(machine.heap,value);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame?.pending?.$!=='waitFunctionText'||value.$!=='text'&&value.$!=='sourceText'||readJavaScriptSemanticTextLength(value)<=0)return {$:'machineInvalid',heap:machine.heap,reason:'function text metadata resumes only its actual suspended request with complete nonempty text'};
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
  const heap=transition.heap??machine.heap,current={$:'callMachine',heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap,reason:'property definition needs its actual active frame'};
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
 const transition=unwindJavaScriptCallCompletion(machine,{$:'callCompletion',kind:'return',value,target:-1,depth:0});return transition.$==='callReturn'?finishJavaScriptCallReturn(program,transition.machine,transition.value):transition;
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
    case 'waitIteratorProtocol':return acceptJavaScriptCallIteratorProtocol(program,current,pending,resumeJavaScriptIteratorProtocolCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArraySpread':return acceptJavaScriptCallArraySpread(program,current,pending.destination,pending.realm,resumeJavaScriptArraySpread(machine.heap,pending.state,{$:'normalCompletion',value},false));
    case 'waitArrayIteratorNext':return acceptJavaScriptCallArrayIteration(program,current,resumeJavaScriptArrayIteratorCall(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayConcatenation':{const directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array concatenation return retains actual species authority'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,pending.construction,{$:'normalCompletion',value})):acceptJavaScriptCallArrayConcatenation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArraySlice':{const directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array slice return retains actual species authority'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,pending.construction,{$:'normalCompletion',value})):acceptJavaScriptCallArraySlice(program,current,pending.destination,pending.realm,resumeJavaScriptArraySliceCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArrayTransform':{const directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform return retains its actual source directory'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,current,resumeJavaScriptSpeciesConstructionCall(program,machine.heap,directory,pending.construction,{$:'normalCompletion',value})):acceptJavaScriptCallArrayTransform(program,current,pending.destination,pending.realm,resumeJavaScriptArrayTransformCall(machine.heap,directory,pending.state,{$:'normalCompletion',value}));}
    case 'waitArraySearch':return acceptJavaScriptCallArraySearch(program,current,pending.destination,pending.realm,resumeJavaScriptArraySearch(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayCreation':{if(pending.construction!==undefined){const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'}:acceptJavaScriptCallArrayConstruction(program,current,resumeJavaScriptConstructionCall(program,machine.heap,directory,pending.construction,{$:'normalCompletion',value}));}return acceptJavaScriptCallArrayCreation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'normalCompletion',value}));}
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
      const held=readJavaScriptCollectionConstructionReceiver(program,machine,frame,instruction);if(held.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:held.error};if(held.receiver!==undefined)return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,held.receiver))};
      if(instruction.$==='collectionIteratorAllocate'&&instruction.kind==='Array'){
        const receiver=registers[0]??{$:'undefined'},owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
        if(receiver.$==='null'||receiver.$==='undefined')return owner===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array iterator creation retains its actual realm'}:requestJavaScriptCallError(machine,owner.realm,'TypeError','Cannot convert undefined or null to object');
        if(receiver.$!=='object')return {$:'machineNeedArrayIteratorBoxing',machine,receiver};
      }
      if(instruction.$==='collectionIteratorAllocate'&&instruction.kind!=='Array'&&instruction.kind!=='String'){
        const receiver=readJavaScriptCollectionIteratorReceiverError(machine.heap,registers[0]??{$:'undefined'},instruction.kind);
        if(receiver.error!==''){
          const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
          return receiver.typeError&&owner!==undefined?requestJavaScriptCallError(machine,owner.realm,'TypeError',receiver.error):{$:'machineInvalid',heap:machine.heap,reason:receiver.error};
        }
      }
      if(instruction.$==='collectionIteratorAllocate'&&instruction.kind==='String'){
        const value=registers[instruction.input??-1]??{$:'undefined'};if(value.$!=='text'&&value.$!=='sourceText')return {$:'machineInvalid',heap:machine.heap,reason:'String iterator allocation retains its actual completed text conversion'};
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
        if(owner===undefined||plan.receiver.$!=='object'||plan.definition===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'deferred Array assignment requires its actual owner, receiver and value descriptor'};
        const current=replaceJavaScriptCallFrame(plan.heap,machine.frames,next);
        return acceptJavaScriptCallArrayAssignment(program,current,instruction.destination,owner.realm,beginJavaScriptArrayAssignment(plan.heap,plan.receiver.identity,registers[instruction.key]??{$:'undefined'},plan.definition,program.exoticKey));
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
     if(pending.construction!==undefined){const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'}:acceptJavaScriptCallArrayConstruction(program,machine,stepJavaScriptConstruction(program,machine.heap,directory,pending.construction));}
     if(pending.fallback!==undefined)return requestJavaScriptCallArrayFallback(program,machine,pending.destination,pending.realm,pending.state,pending.fallback);
     return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,stepJavaScriptArrayCreation(machine.heap,pending.state));
    }
    case 'waitConstructorCheck':return acceptJavaScriptCallConstructorCheck(program,machine,pending.destination,pending.input,pending.required);
    case 'waitConstruction':{const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'construction requires actual constructor authority'}:acceptJavaScriptCallConstruction(program,machine,pending.destination,pending.realm,stepJavaScriptConstruction(program,machine.heap,directory,pending.state));}
    case 'waitFunctionBind':return acceptJavaScriptCallFunctionBind(program,machine,pending.destination,pending.realm,stepJavaScriptFunctionBind(machine.heap,pending.state));
    case 'waitFunctionText':return {$:'machineNeedFunctionText',machine,input:pending.input,kind:pending.kind,realm:pending.realm};
    case 'waitObjectFromEntries':return pending.error!==undefined?{$:'machineAllocateError',machine,realm:pending.error.realm,errorClass:pending.error.errorClass,reason:pending.error.reason}:acceptJavaScriptCallObjectFromEntries(program,machine,pending.destination,pending.realm,stepJavaScriptObjectFromEntries(machine.heap,pending.state));
    case 'waitPrimitiveWrapper':return acceptJavaScriptCallPrimitiveWrapper(machine,pending.destination,beginJavaScriptPrimitiveWrapper(machine.heap,pending.state.input,pending.state.prototype));
    case 'waitSymbolAllocation':return machine.heap.symbols.length>=16384?{$:'machineResourceLimit',machine,reason:'Symbol allocation exceeds the selected 16384 identity profile'}:{$:'machineAllocateSymbol',machine,description:pending.description,...(pending.registryKey===undefined?{}:{registryKey:pending.registryKey})};
    case 'waitSymbolDescription':{const fact=readJavaScriptSymbolRegistrationFact(machine.heap,pending.symbol.identity);if(fact?.registered&&fact.key!==undefined)return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,fact.key))};return {$:'machineNeedSymbolDescription',machine,symbol:pending.symbol};}
    case 'waitFunctionNameSymbol':{const fact=readJavaScriptSymbolRegistrationFact(machine.heap,pending.key.identity);if(fact?.registered&&fact.key!==undefined)return resumeJavaScriptCallFunctionNameSymbolDescription(program,machine,fact.key);return {$:'machineNeedFunctionNameSymbolDescription',machine,object:pending.object,key:pending.key,prefix:pending.prefix};}
    case 'waitArrayConcatenation':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array concatenation step retains actual species authority'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,pending.construction)):acceptJavaScriptCallArrayConcatenation(program,machine,pending.destination,pending.realm,stepJavaScriptArrayConcatenation(machine.heap,directory,pending.state));}
    case 'waitArraySlice':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array slice step retains actual species authority'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,pending.construction)):acceptJavaScriptCallArraySlice(program,machine,pending.destination,pending.realm,stepJavaScriptArraySlice(machine.heap,directory,pending.state));}
    case 'waitArrayTransform':{const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform step retains its actual source directory'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,machine,stepJavaScriptSpeciesConstruction(program,machine.heap,directory,pending.construction)):acceptJavaScriptCallArrayTransform(program,machine,pending.destination,pending.realm,stepJavaScriptArrayTransform(machine.heap,directory,pending.state));}
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
    case 'waitIteratorProtocol':return acceptJavaScriptCallIteratorProtocol(program,machine,pending,stepJavaScriptIteratorProtocol(machine.heap,pending.state));
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
  const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a call step needs its actual source function'};
  const instruction=fn.instructions[frame.position];
  if(instruction===undefined)return returnJavaScriptCallValue(program,machine,{$:'undefined'});
  const next=advanceJavaScriptCallFrame(frame),current=replaceJavaScriptCallFrame(machine.heap,machine.frames,next),registers=frame.registers;
  switch(instruction.$){
    case 'functionBind':return acceptJavaScriptCallFunctionBind(program,current,instruction.destination,fn.realm,beginJavaScriptFunctionBind(machine.heap,registers[0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments.slice(1)));
    case 'functionToString':{const input=registers[0]??{$:'undefined'},result=readJavaScriptFunctionText(program,machine.heap,input);if(result.$==='functionTextValue')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,result.value))};if(result.$==='functionTextError')return requestJavaScriptCallError(current,fn.realm,result.errorClass,result.reason);if(result.$==='functionTextInvalid')return {$:'machineInvalid',heap:machine.heap,reason:result.reason};const retained=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(next,{$:'waitFunctionText',destination:instruction.destination,realm:fn.realm,input,kind:result.kind}));return {$:'machineNeedFunctionText',machine:retained,input,kind:result.kind,realm:fn.realm};}
    case 'boundFunctionCall':{
      const bound=machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,frame.identity)];if(bound?.$!=='boundFunctionObject')return {$:'machineInvalid',heap:machine.heap,reason:'a bound call retains its actual internal-slot owner'};
      if(bound.boundArguments.length+frame.arguments.length>1048576)return {$:'machineResourceLimit',machine,reason:'bound and supplied arguments exceed the shared call List budget'};
      const nextMachine=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(next,{$:'waitCall',destination:instruction.destination})),called=pushJavaScriptCall(program,nextMachine,bound.boundTarget,bound.boundThis,bound.boundArguments.concat(frame.arguments));return called.$==='machineResourceLimit'?{...called,machine}:called;
    }
    case 'constant':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,instruction.value))};
    case 'copy':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,registers[instruction.source]??{$:'undefined'}))};
    case 'argument':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[instruction.index]??{$:'undefined'}))};
    case 'argumentCount':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'number',value:frame.arguments.length}))};
    case 'argumentAt':{
      const index=registers[instruction.index]??{$:'undefined'};if(index.$!=='number'||!Number.isSafeInteger(index.value)||Object.is(index.value,-0)||index.value<0||index.value>=1048576)return {$:'machineInvalid',heap:machine.heap,reason:'dynamic argument reads retain their actual bounded Call List coordinate'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[index.value]??{$:'undefined'}))};
    }
    case 'jump':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:instruction.target})};
    case 'branch':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:convertJavaScriptSemanticBoolean(registers[instruction.condition]??{$:'undefined'})?instruction.yes:instruction.no})};
    case 'return':return returnJavaScriptCallValue(program,current,registers[instruction.source]??{$:'undefined'});
    case 'throw':return throwJavaScriptCallValue(current,registers[instruction.source]??{$:'undefined'});
    case 'pushFinally':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'finalizer storage exceeds its actual handler budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callFinallyHandler',target:instruction.target}])})};
    }
    case 'completeJump':return acceptJavaScriptCallCompletion(program,unwindJavaScriptCallCompletion(current,{$:'callCompletion',kind:'jump',value:{$:'undefined'},target:instruction.target,depth:instruction.depth}));
    case 'finishFinally':{
      const handler=frame.handlers[frame.handlers.length-1];if(handler?.$!=='callDeferredCompletion')return {$:'machineInvalid',heap:machine.heap,reason:'finalizer exit retains its actual deferred completion'};
      const resumed=replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)});return acceptJavaScriptCallCompletion(program,unwindJavaScriptCallCompletion(resumed,handler.completion));
    }
    case 'pushHandler':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'handler storage exceeds the selected execution resource budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callHandler',target:instruction.target,destination:instruction.destination}])})};
    }
    case 'popHandler':{
      if(frame.handlers[frame.handlers.length-1]?.$!=='callHandler')return {$:'machineInvalid',heap:machine.heap,reason:'a handler pop requires its actual active catch handler'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)})};
    }
    case 'argumentListOpen':case 'argumentListAppend':{
      const lists=frame.argumentLists??[];
      const update=instruction.$==='argumentListOpen'?openJavaScriptCallArgumentList(machine.heap,lists,instruction.slot,frame.handlers.length):appendJavaScriptCallArgument(machine.heap,lists,instruction.slot,frame.handlers.length,registers[instruction.value]??{$:'undefined'});
      if(update.$==='argumentListsInvalid')return {$:'machineInvalid',heap:machine.heap,reason:update.reason};
      if(update.$==='argumentListsResourceLimit')return {$:'machineResourceLimit',machine,reason:update.reason};
      let count=0;for(const active of machine.frames){count+=active.arguments.length;for(const list of active===frame?update.lists:active.argumentLists??[])count+=list.values.length;}
      if(count>1048576)return {$:'machineResourceLimit',machine,reason:'argument Lists share the complete actual stack argument storage budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,argumentLists:update.lists})};
    }
    case 'argumentListCall':{
      const lists=frame.argumentLists??[],at=findJavaScriptCallArgumentList(lists,instruction.slot),list=lists[at];if(list===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'a variadic Call consumes its actual opened argument List exactly once'};
      const caller=replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,argumentLists:lists.filter((row,index)=>index!==at),pending:{$:'waitCall',destination:instruction.destination}}),callee=registers[instruction.callee]??{$:'undefined'},receiver=registers[instruction.receiver]??{$:'undefined'};
      if(instruction.mode==='construct'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'variadic construction requires actual constructor authority'};return acceptJavaScriptCallConstruction(program,caller,instruction.destination,fn.realm,beginJavaScriptConstruction(program,machine.heap,directory,callee,receiver,list.values),list.values);}
      const transition=instruction.mode==='collection'?pushJavaScriptCollectionConstructor(program,caller,callee,list.values):instruction.mode==='error class'?pushJavaScriptErrorConstructor(program,caller,callee,list.values):instruction.mode==='ordinary class'?pushJavaScriptOrdinaryClassConstructor(program,caller,callee,receiver,list.values):pushJavaScriptCall(program,caller,callee,receiver,list.values);
      return transition.$==='machineResourceLimit'?{...transition,machine}:transition;
    }
    case 'call':{
      const arguments_=instruction.arguments.map(index=>registers[index]??{$:'undefined'});
      return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},arguments_);
    }
    case 'constructErrorClass':return pushJavaScriptErrorConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'callClassConstructor':return pushJavaScriptOrdinaryClassConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'constructCollection':return pushJavaScriptCollectionConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'argumentRest':return acceptJavaScriptCallArgumentRest(current,instruction.destination,instruction.realm,beginJavaScriptArgumentRest(machine.heap,frame.arguments,instruction.from,registers[instruction.prototype]??{$:'undefined'}));
    case 'iteratorProtocol':{
      const regions=frame.iterators??[],region=regions[findJavaScriptIteratorRegion(regions,instruction.slot)],pending={$:'iteratorOperation',destination:instruction.destination,done:instruction.done,slot:instruction.slot,operation:instruction.operation,realm:instruction.realm};
      if(instruction.operation==='acquire'){
        if(region!==undefined)return {$:'machineInvalid',heap:machine.heap,reason:'iterator opening cannot overwrite an active source region'};if(regions.length>=128)return {$:'machineResourceLimit',machine,reason:'iterator opening exceeds its actual frame region budget'};
        return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIterator(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'}));
      }
      if(region===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'iterator operation requires its actual opened source region'};
      if(instruction.operation==='step')return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIteratorStep(machine.heap,region.record,instruction.readValue??true));
      if(region.record.done){const removed=removeJavaScriptIteratorRegion(machine.heap,regions,instruction.slot);if(removed.$!=='iteratorRegionsUpdated')return {$:'machineInvalid',heap:machine.heap,reason:'exhausted iterator release retains its actual region'};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallIteratorResult({...next,iterators:removed.regions},instruction.destination,instruction.done,{$:'undefined'},true))};}
      return acceptJavaScriptCallIteratorProtocol(program,current,pending,beginJavaScriptSynchronousIteratorClose(machine.heap,region.record,readJavaScriptCallIteratorCloseCompletion(frame)));
    }
    case 'collectionIteratorResult':{
      if(instruction.kind==='String')return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,instruction.destination,fn.realm,beginJavaScriptStringIteratorResult(machine.heap,registers[0]??{$:'undefined'},registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'}));
      if(instruction.kind==='Array'){
        const error=readJavaScriptCollectionIteratorResultPrototypesError(machine.heap,registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'});if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
        return acceptJavaScriptCallArrayIteration(program,machine,beginJavaScriptArrayIteratorNext(machine.heap,registers[0]??{$:'undefined'},program.exoticKey));
      }
      return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,instruction.destination,fn.realm,beginJavaScriptCollectionIteratorResult(machine.heap,registers[0]??{$:'undefined'},instruction.kind,registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'}));
    }
    case 'collectionOperation':return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,(readJavaScriptWeakCollectionOperationKind(instruction.operation)!==''?executeJavaScriptWeakCollectionOperation:executeJavaScriptCollectionOperation)(machine.heap,registers[0]??{$:'undefined'},instruction.operation,frame.arguments));
    case 'primitiveMethod':{
     if(testJavaScriptArrayIntrinsicOperation(instruction.operation)){
      const result=testJavaScriptArrayValue(machine.heap,frame.arguments[0]??{$:'undefined'});if(result.error!=='')return {$:'machineInvalid',heap:machine.heap,reason:result.error};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:result.array}))};
     }
     const completion=executeJavaScriptPrimitiveMethod(registers[0]??{$:'undefined'},instruction.operation,frame.arguments);
     switch(completion.$){
      case 'primitiveValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,completion.value))};
      case 'primitiveInvalid':return {$:'machineInvalid',heap:machine.heap,reason:completion.reason};
      case 'primitiveError':return requestJavaScriptCallError(machine,fn.realm,completion.errorClass,completion.reason);
      case 'primitiveResourceLimit':return {$:'machineResourceLimit',machine,reason:completion.reason};
     }
    }
    case 'collectionCopy':return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,copyJavaScriptCollection(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},registers[instruction.adder]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},registers[instruction.iteratorMethod]??{$:'undefined'},registers[instruction.iteratorPrototype]??{$:'undefined'},registers[instruction.nextMethod]??{$:'undefined'}));
    case 'constructErrorValue':return requestJavaScriptErrorValue(program,machine,frame,instruction);
    case 'readDerivedReceiver':{const receiver=registers[0]??{$:'undefined'};if(receiver.$==='undefined')return requestJavaScriptCallError(current,fn.realm,'ReferenceError','derived this requires its actual completed super construction');return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,receiver))};}
    case 'invokeErrorIntrinsic':return {$:'machineInvokeErrorIntrinsic',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitErrorIntrinsic',destination:instruction.destination}}),realm:fn.realm,callee:{$:'object',identity:fn.identity,callable:true},receiver:registers[0]??{$:'undefined'},arguments:frame.arguments};
    case 'invokeHostCapability':return {$:'machineInvokeHostCapability',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitHostCapability',destination:instruction.destination,beforeDirectory:readJavaScriptErrorAllocationDirectory(machine.heap)}}),realm:fn.realm,callee:{$:'object',identity:fn.identity,callable:true},receiver:registers[0]??{$:'undefined'},arguments:frame.arguments};
    case 'strictEqual':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptCallStrictEqual(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'sameValue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:testJavaScriptSemanticSameValue(registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'})}))};
    case 'numberBinary':{
      const left=registers[instruction.left]??{$:'undefined'},right=registers[instruction.right]??{$:'undefined'};
      if(left.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      if(right.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'typed number instructions require two actual Number operands'};
      const op=instruction.operator,first=left.value,second=right.value;
      if(op==='lessThan'||op==='equal')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'boolean',value:op==='lessThan'?first<second:first===second}))};
      const value=op==='add'?first+second:op==='subtract'?first-second:op==='multiply'?first*second:op==='divide'?first/second:op==='remainder'?first%second:first**second;
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'number',value}))};
    }
    case 'primitive':return executeJavaScriptCallPrimitiveInstruction(program,machine,frame,instruction);
    case 'closureAllocate':case 'ordinaryAllocate':case 'arrayAllocate':case 'collectionAllocate':case 'collectionIteratorAllocate':case 'ordinaryGet':case 'ordinarySet':case 'ordinaryDelete':case 'ordinaryHas':case 'requireWrite':return executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction);
    case 'propertyKey':return acceptJavaScriptCallKey(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyKeyConversion(machine.heap,registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyDefinition':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyDefinition(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.attributes]??{$:'undefined'},program.exoticKey,instruction.operation));
    case 'ordinaryDefine':{const decoded=readJavaScriptDescriptorOperands(instruction.fields.map(coordinate=>registers[coordinate]??{$:'undefined'}));return decoded.error!==''?{$:'machineInvalid',heap:machine.heap,reason:decoded.error}:acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},decoded.descriptor,program.exoticKey,'Reflect.defineProperty'));}
    case 'arrayConcatenation':return acceptJavaScriptCallArrayConcatenation(program,current,instruction.destination,fn.realm,beginJavaScriptArrayConcatenation(machine.heap,instruction.species,registers[instruction.input??0]??{$:'undefined'},frame.arguments,registers[instruction.spreadableKey]??{$:'undefined'},program.exoticKey));
    case 'arraySlice':return acceptJavaScriptCallArraySlice(program,current,instruction.destination,fn.realm,beginJavaScriptArraySlice(machine.heap,instruction.species,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},frame.arguments.length>1,program.exoticKey));
    case 'arrayTransform':return acceptJavaScriptCallArrayTransform(program,current,instruction.destination,fn.realm,beginJavaScriptArrayTransform(machine.heap,instruction.species,instruction.operation,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},program.exoticKey));
    case 'arraySearch':{const at=instruction.operation==='Array.at';return acceptJavaScriptCallArraySearch(program,current,instruction.destination,fn.realm,beginJavaScriptArraySearch(machine.heap,instruction.operation,registers[instruction.input??0]??{$:'undefined'},at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},frame.arguments[at?0:1]??{$:'undefined'},frame.arguments.length>=(at?1:2),program.exoticKey));}
    case 'collectionForEach':return acceptJavaScriptCallCollectionForEach(program,current,instruction.destination,fn.realm,beginJavaScriptCollectionForEach(machine.heap,instruction.operation,registers[0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'}));
    case 'arrayCallback':return acceptJavaScriptCallArrayCallback(program,current,instruction.destination,fn.realm,beginJavaScriptArrayCallback(machine.heap,instruction.operation,registers[instruction.input??0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},program.exoticKey,frame.arguments.length>=2));
    case 'arraySpread':{
      const index=registers[instruction.index]??{$:'undefined'};if(index.$!=='number')return {$:'machineInvalid',heap:machine.heap,reason:'Array spread index retains its actual Number coordinate'};
      return acceptJavaScriptCallArraySpread(program,current,instruction.destination,instruction.realm,beginJavaScriptArraySpread(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},index.value,registers[instruction.iteratorKey]??{$:'undefined'}));
    }
    case 'objectSpread':return acceptJavaScriptCallObjectSpread(program,current,instruction.destination,instruction.realm,instruction.operation==='assignment'?beginJavaScriptObjectAssignment(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},program.exoticKey):beginJavaScriptObjectSpread(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},(instruction.excluded??[]).map(index=>registers[index]??{$:'undefined'})));
    case 'objectEnumeration':return acceptJavaScriptCallObjectEnumeration(program,current,instruction.destination,fn.realm,beginJavaScriptObjectEnumeration(machine.heap,instruction.operation,frame.arguments[0]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'}));
    case 'propertyDefinitions':return acceptJavaScriptCallPropertyDefinitions(program,current,instruction.destination,fn.realm,beginJavaScriptPropertyDefinitions(machine.heap,instruction.operation,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyReflection':return acceptJavaScriptCallPropertyReflection(current,instruction.destination,fn.realm,beginJavaScriptPropertyReflection(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'},registers[instruction.arrayPrototype]??{$:'undefined'}));
    case 'instanceOf':return acceptJavaScriptCallInstanceOf(program,current,instruction.destination,fn.realm,beginJavaScriptInstanceOf(machine.heap,instruction.operation,registers[instruction.left]??{$:'undefined'},registers[instruction.right]??{$:'undefined'},registers[instruction.key]??{$:'undefined'}));
    case 'objectObservation':return acceptJavaScriptCallObjectIntegrity(current,instruction.destination,fn.realm,beginJavaScriptObjectObservation(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'},registers[instruction.other]??{$:'undefined'}));
    case 'arrayJoinEnter':{
     const receiver=registers[instruction.receiver]??{$:'undefined'};if(receiver.$!=='object'||frame.activeJoin!==undefined)return {$:'machineInvalid',heap:machine.heap,reason:'join entry retains an actual Object and fresh activation'};
     const recursive=machine.frames.some(parent=>parent.activeJoin!==undefined&&readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity)?.realm===fn.realm&&testJavaScriptSemanticSameValue(parent.activeJoin,receiver)),joinedFrame=recursive?next:{...next,activeJoin:receiver};
     return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,current.frames,writeJavaScriptCallRegister(joinedFrame,instruction.destination,{$:'boolean',value:!recursive}))};
    }
    case 'arrayJoinLeave':{
     const receiver=registers[instruction.receiver]??{$:'undefined'};if(frame.activeJoin===undefined||!testJavaScriptSemanticSameValue(frame.activeJoin,receiver))return {$:'machineInvalid',heap:machine.heap,reason:'join leave retains its actually owned receiver guard'};
     const {activeJoin:released,...releasedFrame}=next;return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,current.frames,releasedFrame)};
    }
    case 'arrayCreation':return acceptJavaScriptCallArrayCreation(program,current,instruction.destination,fn.realm,beginJavaScriptArrayCreation(machine.heap,instruction.operation,registers[0]??{$:'undefined'},frame.arguments,registers[instruction.prototype]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},program.exoticKey));
    case 'readNewTarget':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.newTarget??{$:'undefined'}))};
    case 'constructorCheck':return acceptJavaScriptCallConstructorCheck(program,current,instruction.destination,registers[instruction.input]??{$:'undefined'},instruction.required);
    case 'construct':{const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'construction requires actual constructor authority'}:acceptJavaScriptCallConstruction(program,current,instruction.destination,fn.realm,beginJavaScriptConstruction(program,machine.heap,directory,registers[instruction.callee]??{$:'undefined'},registers[instruction.newTarget]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'})));}
    case 'objectFromEntries':return acceptJavaScriptCallObjectFromEntries(program,current,instruction.destination,fn.realm,beginJavaScriptObjectFromEntries(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'},registers[instruction.iteratorKey]??{$:'undefined'},program.exoticKey));
    case 'primitiveWrapper':return acceptJavaScriptCallPrimitiveWrapper(current,instruction.destination,beginJavaScriptPrimitiveWrapper(machine.heap,registers[instruction.input]??{$:'undefined'},registers[instruction.prototype]??{$:'undefined'}));
    case 'symbolRegistry':{
      const result=executeJavaScriptSymbolRegistryOperation(machine.heap,instruction.operation,registers[instruction.input]??{$:'undefined'});
      if(result.$==='symbolRegistryValue')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(result.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,result.value))};
      if(result.$==='symbolRegistryAllocate')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(result.heap,machine.frames,{...next,pending:{$:'waitSymbolAllocation',destination:instruction.destination,description:result.key,registryKey:result.key}})};
      if(result.$==='symbolRegistryError')return requestJavaScriptCallError(machine,fn.realm,result.errorClass,result.reason);
      if(result.$==='symbolRegistryResourceLimit')return {$:'machineResourceLimit',machine,reason:result.reason};
      return {$:'machineInvalid',heap:machine.heap,reason:result.reason};
    }
    case 'symbolAllocate':{
      const description=registers[instruction.input]??{$:'undefined'};if(!['undefined','text','sourceText'].includes(description.$))return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires its actual completed optional text description'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSymbolAllocation',destination:instruction.destination,description}})};
    }
    case 'symbolDescription':{
      const symbol=registers[instruction.input]??{$:'undefined'};if(symbol.$!=='symbol')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol description instructions require their actual primitive Symbol input'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitSymbolDescription',destination:instruction.destination,symbol}})};
    }
    case 'functionName':{
      const object=registers[instruction.object]??{$:'undefined'},key=registers[instruction.key]??{$:'undefined'},named=constructJavaScriptPropertyFunctionName(key,instruction.prefix);
      const target=object.$==='object'?machine.heap.objects[findJavaScriptOrdinaryObject(machine.heap.objects,object.identity)]:undefined;if(object.$!=='object'||!object.callable||target?.$!=='closureObject')return {$:'machineInvalid',heap:machine.heap,reason:'function naming requires its actual allocated runtime callable'};
      if(named.$==='functionNameInvalid')return {$:'machineInvalid',heap:machine.heap,reason:named.reason};
      if(named.$==='functionNameResourceLimit')return {$:'machineResourceLimit',machine,reason:named.reason};
      if(named.$==='functionNameSymbolDescription')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitFunctionNameSymbol',destination:instruction.destination,object,key,prefix:instruction.prefix,realm:instruction.realm}})};
      return finishJavaScriptCallFunctionName(program,current,instruction.destination,object,instruction.realm,named.value);
    }
    case 'literalAccessor':{
      const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true),value=registers[instruction.value]??{$:'undefined'};
      // An absent opposite field preserves an existing accessor's counterpart;
      // conversion from a data property supplies undefined through the kernel.
      if(instruction.kind==='get'){descriptor.getter=value;descriptor.setterPresent=false;}else{descriptor.setter=value;descriptor.getterPresent=false;}
      return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.object]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},descriptor,program.exoticKey));
    }
    case 'literalPrototype':{
      const object=registers[instruction.object]??{$:'undefined'},value=registers[instruction.value]??{$:'undefined'};
      if(object.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'literal prototype installation requires its actual allocated Object'};
      if(value.$!=='object'&&value.$!=='null')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'undefined'}))};
      const installed=setJavaScriptOrdinaryPrototype(machine.heap,object.identity,value);
      if(installed.error!=='')return {$:'machineInvalid',heap:installed.heap,reason:installed.error};
      if(!installed.success)return {$:'machineInvalid',heap:installed.heap,reason:'a fresh literal Object must admit its acyclic actual prototype'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(installed.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,{$:'undefined'}))};
    }
    case 'ordinaryDataProperty':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinitionWithSymbol(machine.heap,registers[instruction.object]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},createJavaScriptOrdinarySetDescriptor(registers[instruction.value]??{$:'undefined'},true),program.exoticKey));
  }
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
 const error=readJavaScriptCallMachineError(program,machine);
 if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
 const frame=machine.frames[machine.frames.length-1];
 if(frame!==undefined&&(frame.pending.$==='waitArrayTransform'||frame.pending.$==='waitArraySlice'||frame.pending.$==='waitArrayConcatenation')){
  const pending=frame.pending,directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Array transform allocation retains its actual source directory'};return pending.construction!==undefined?acceptJavaScriptCallSpeciesConstruction(program,machine,resumeJavaScriptSpeciesConstructionAllocation(program,machine.heap,directory,pending.construction,identity)):pending.$==='waitArrayConcatenation'?acceptJavaScriptCallArrayConcatenation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationAllocation(machine.heap,directory,pending.state,identity)):pending.$==='waitArraySlice'?acceptJavaScriptCallArraySlice(program,machine,pending.destination,pending.realm,resumeJavaScriptArraySliceAllocation(machine.heap,directory,pending.state,identity)):acceptJavaScriptCallArrayTransform(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayTransformAllocation(machine.heap,directory,pending.state,identity));
 }
 if(frame!==undefined&&frame.pending.$==='waitArgumentRest')return acceptJavaScriptCallArgumentRest(machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptArgumentRestAllocation(machine.heap,frame.arguments,frame.pending.state,identity));
 if(frame?.pending.$==='waitArrayCreation'){
  const pending=frame.pending,request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return request.$==='machineResourceLimit'?request:{$:'machineInvalid',heap:machine.heap,reason:'Array creation identity response retains its actual allocation request'};
  if(pending.construction!==undefined){const directory=program.constructorDirectory;return directory===undefined?{$:'machineInvalid',heap:machine.heap,reason:'Array construction requires actual authority'}:acceptJavaScriptCallArrayConstruction(program,machine,resumeJavaScriptConstructionAllocation(program,machine.heap,directory,pending.construction,identity));}
  if(pending.fallback!==undefined){const made=allocateJavaScriptArray(machine.heap,identity,pending.fallback.prototype,pending.fallback.length);if(made.$==='arrayNormal')return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayCreationConstruction(made.heap,pending.state,{$:'normalCompletion',value:made.value}));if(made.$==='arrayResourceLimit')return {$:'machineResourceLimit',machine,reason:made.reason};return {$:'machineInvalid',heap:made.heap,reason:made.$==='arrayInvalid'||made.$==='arrayError'?made.reason:'ArrayCreate retains its actual allocation result'};}
  return acceptJavaScriptCallArrayCreation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayCreationAllocation(machine.heap,pending.state,identity));
 }
 if(frame!==undefined&&frame.pending.$==='waitConstruction'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'construction allocation requires actual constructor authority'};const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return {$:'machineInvalid',heap:machine.heap,reason:'construction allocation retains its actual fresh Object request'};return acceptJavaScriptCallConstruction(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptConstructionAllocation(program,machine.heap,directory,frame.pending.state,identity));}
 if(frame?.pending.$==='waitFunctionBind')return acceptJavaScriptCallFunctionBind(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptFunctionBindAllocation(machine.heap,frame.pending.state,identity));
 if(frame!==undefined&&frame.pending.$==='waitObjectFromEntries'){const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateObject')return request.$==='machineResourceLimit'?request:{$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries Object response retains its actual allocation request'};return acceptJavaScriptCallObjectFromEntries(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptObjectFromEntriesAllocation(machine.heap,frame.pending.state,identity));}
 if(frame!==undefined&&frame.pending.$==='waitPrimitiveWrapper')return acceptJavaScriptCallPrimitiveWrapper(machine,frame.pending.destination,resumeJavaScriptPrimitiveWrapper(machine.heap,frame.pending.state,identity));
 if(frame!==undefined&&frame.pending.$==='waitPropertyDefinitions')return acceptJavaScriptCallPropertyDefinitions(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptPropertyDefinitionsAllocation(machine.heap,frame.pending.state,identity));
 if(frame!==undefined&&frame.pending.$==='waitPropertyReflection')return acceptJavaScriptCallPropertyReflection(machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptPropertyReflectionAllocation(machine.heap,frame.pending.state,identity));
 if(frame!==undefined&&frame.pending.$==='waitObjectEnumeration')return acceptJavaScriptCallObjectEnumeration(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptObjectEnumerationAllocation(machine.heap,frame.pending.state,identity));
 if(frame!==undefined&&frame.pending.$==='waitCollectionIteratorResult')return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,frame.pending.destination,frame.pending.realm,resumeJavaScriptCollectionIteratorResultAllocation(machine.heap,frame.pending.state,identity));
 if(frame===undefined||frame.pending.$!=='callIdle')return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual active allocation instruction'};
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
 const prototype=frame.registers[instruction.prototype]??{$:'undefined'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'machineInvalid',heap:machine.heap,reason:'ordinary allocation requires its actual null or Object prototype'};
 if(machine.heap.objects.length>=4096)return {$:'machineResourceLimit',machine,reason:'ordinary heap identity directory exceeds its explicit resource budget'};
 if(instruction.$==='arrayAllocate'){
  const allocation=allocateJavaScriptArray(machine.heap,identity,prototype,instruction.length);
  if(allocation.$==='arrayResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$!=='arrayNormal')return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='arrayError'?'literal Array length must have been validated before allocation':allocation.$==='arrayInvalid'?allocation.reason:'Array allocation requires its actual normal completion'};
  const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value);return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
 }
 if(instruction.$==='collectionIteratorAllocate'&&instruction.kind==='String'){
  const allocation=allocateJavaScriptStringIterator(machine.heap,identity,prototype,frame.registers[instruction.input??-1]??{$:'undefined'});
  if(allocation.$==='stringIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
  if(allocation.$==='stringIteratorResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='stringIteratorInvalid'?allocation.reason:'String iterator creation retains its actual allocation result'};
 }

 if(instruction.$==='collectionIteratorAllocate'&&instruction.kind==='Array'){
  const allocation=allocateJavaScriptArrayIterator(machine.heap,identity,prototype,frame.registers[0]??{$:'undefined'},instruction.iterationKind);
  if(allocation.$==='arrayIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
  if(allocation.$==='arrayIteratorAllocationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$==='arrayIteratorNeedBoxing')return {$:'machineNeedArrayIteratorBoxing',machine,receiver:allocation.receiver};
  if(allocation.$==='arrayIteratorError')return requestJavaScriptCallError(machine,fn.realm,allocation.errorClass,allocation.reason);
  return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='arrayIteratorInvalid'?allocation.reason:'Array iterator creation retains its actual allocation result'};
 }
 if(instruction.$==='collectionIteratorAllocate'){
  const allocation=allocateJavaScriptCollectionIterator(machine.heap,identity,prototype,frame.registers[0]??{$:'undefined'},instruction.kind,instruction.iterationKind);
  if(allocation.$==='collectionIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
  if(allocation.$==='collectionIterationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$==='collectionIterationTypeError')return requestJavaScriptCallError(machine,fn.realm,'TypeError',allocation.reason);
  return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='collectionIterationInvalid'?allocation.reason:'iterator allocation needs its actual completed creation'};
 }
 if(instruction.$==='collectionAllocate'){
  const allocation=(['WeakMap','WeakSet'].includes(instruction.kind)?allocateJavaScriptWeakCollection:allocateJavaScriptCollection)(machine.heap,identity,prototype,instruction.kind);if(allocation.$!=='collectionNormal')return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,allocation);
  const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value);return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
 }
 const allocation=insertJavaScriptOrdinaryObject(machine.heap,identity,prototype);
 if(!allocation.success)return {$:'machineInvalid',heap:machine.heap,reason:allocation.error};
 const next=writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,{$:'object',identity,callable:false});
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,next)};
}

/** The allocator must create the actual Error in the requested realm and
 * extend the same global heap. Its authority is external, not inferred from
 * an Object name/prototype or membership in this ordinary directory.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {bigint} realm @param {JavaScriptSemanticValue} value @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallErrorAllocation(program,machine,realm,value){
  const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption needs its actual active frame'};
  if(frame.pending.$==='waitArrayCreation'){
   const pending=frame.pending,request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||value.$!=='object'||value.callable||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Array creation retains actual Error allocation before cleanup'};
   const resumed=pending.error!==undefined?resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'throwCompletion',value}):resumeJavaScriptArrayCreationError(machine.heap,pending.state,value),{error:discarded,...base}=pending;return acceptJavaScriptCallArrayCreation(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
  }
  if(frame.pending.$==='waitObjectFromEntries'){
   const pending=frame.pending,request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!==''||value.$!=='object'||value.callable)return {$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries retains its actual realm Error allocation before cleanup'};
   const resumed=pending.error!==undefined?resumeJavaScriptObjectFromEntriesCall(machine.heap,pending.state,{$:'throwCompletion',value}):resumeJavaScriptObjectFromEntriesError(machine.heap,pending.state,value);
   const {error:discarded,...base}=pending;return acceptJavaScriptCallObjectFromEntries(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
  }
  if(frame.pending.$!=='waitError')return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption requires its actual pending allocation'};
  if(frame.pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'Error resumption retains its actual requested realm'};
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
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitErrorValue')return {$:'machineInvalid',heap:machine.heap,reason:'Error value allocation requires its actual pending constructor continuation'};
 const pending=frame.pending;if(pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'Error value allocation retains its actual requested intrinsic realm'};
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
 if(frame.pending.$==='waitArrayCreation'){
  const pending=frame.pending,request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Array creation abrupt Error allocation retains its actual request'};
  const resumed=pending.error!==undefined?resumeJavaScriptArrayCreationCall(machine.heap,pending.state,{$:'throwCompletion',value}):throwJavaScriptArrayCreation(machine.heap,pending.state,value),{error:discarded,...base}=pending;return acceptJavaScriptCallArrayCreation(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
 }
 if(frame.pending.$==='waitObjectFromEntries'){
  const pending=frame.pending,request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineAllocateError'||request.realm!==realm||readJavaScriptOrdinaryValueError(machine.heap,value)!=='')return {$:'machineInvalid',heap:machine.heap,reason:'Object.fromEntries abrupt Error allocation retains its actual request and thrown value'};
  const resumed=pending.error!==undefined?resumeJavaScriptObjectFromEntriesCall(machine.heap,pending.state,{$:'throwCompletion',value}):throwJavaScriptObjectFromEntries(machine.heap,pending.state,value),{error:discarded,...base}=pending;
  return acceptJavaScriptCallObjectFromEntries(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,base)),pending.destination,pending.realm,resumed);
 }
 const pending=frame.pending;if(pending.$!=='waitErrorValue'&&pending.$!=='waitError'&&pending.$!=='waitSourceError')return {$:'machineInvalid',heap:machine.heap,reason:'an abrupt allocator result needs its actual Error allocation request'};
 if(pending.realm!==realm)return {$:'machineInvalid',heap:machine.heap,reason:'an abrupt allocator retains its requested realm'};
 const invalid=readJavaScriptOrdinaryValueError(machine.heap,value);if(invalid!=='')return {$:'machineInvalid',heap:machine.heap,reason:invalid};
 return throwJavaScriptCallValue(machine,value);
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
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayAssignmentTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayAssignment(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array assignment requires its actual active source frame'};
 switch(transition.$){
  case 'arrayAssignmentContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayAssignment',destination,realm,state:transition.state}))};
  case 'arrayAssignmentCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayAssignment',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arrayAssignmentCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,{$:'boolean',value:transition.success}))};
  case 'arrayAssignmentThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arrayAssignmentError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arrayAssignmentInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayAssignmentResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayAssignment',destination,realm,state:transition.state})),reason:transition.reason};
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
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayCallbackTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayCallback(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];
 if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array callbacks require their actual active intrinsic frame'};
 switch(transition.$){
  case 'arrayCallbackContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCallback',destination,realm,state:transition.state}))};
  case 'arrayCallbackCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCallback',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arrayCallbackCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arrayCallbackThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arrayCallbackError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arrayCallbackInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayCallbackResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCallback',destination,realm,state:transition.state})),reason:transition.reason};
 }
}

/** Validate a child against the exact selected call, including lexical this.
 * @param {JavaScriptCallFunction} fn @param {JavaScriptCallFrame} frame
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {string} */
export function readJavaScriptSelectedCallFrameError(fn,frame,callee,receiver,arguments_){
 if(callee.$!=='object'||callee.identity!==frame.identity||arguments_.length!==frame.arguments.length||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},fn.receiverMode==='lexical'?fn.lexicalReceiver:receiver))return 'a selected child retains its actual callable identity, receiver and arguments';
 for(let at=0;at<arguments_.length;at++)if(!testJavaScriptSemanticSameValue(arguments_[at]??{$:'undefined'},frame.arguments[at]??{$:'undefined'}))return 'a selected child retains every actual ordered argument';
 return '';
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArraySearchTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArraySearch(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array search requires its actual active intrinsic frame'};
 switch(transition.$){
  case 'arraySearchContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySearch',destination,realm,state:transition.state}))};
  case 'arraySearchCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySearch',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arraySearchCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arraySearchThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arraySearchError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arraySearchInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arraySearchResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySearch',destination,realm,state:transition.state})),reason:transition.reason};
 }
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @returns {JavaScriptArraySpeciesDirectory | undefined} */
export function readJavaScriptCallArrayTransformDirectory(program,machine,destination,realm){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return undefined;const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity),instruction=frame.position>0?fn?.instructions[frame.position-1]:undefined;
 return fn===undefined||fn.realm!==realm||instruction===undefined||instruction.$!=='arrayTransform'&&instruction.$!=='arraySlice'&&instruction.$!=='arrayConcatenation'||instruction.destination!==destination?undefined:instruction.species;
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayConcatenationTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayConcatenation(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array transformation requires its actual active intrinsic frame'};
 switch(transition.$){
  case 'arrayConcatenationContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state}))};
  case 'arrayConcatenationCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arrayConcatenationAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state})),prototype:transition.prototype};
  case 'arrayConcatenationConstruct':{
   const suspended=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state})),directory=readJavaScriptCallArrayTransformDirectory(program,suspended,destination,realm);
   return directory===undefined?{$:'machineInvalid',heap:transition.heap,reason:'custom species construction retains its actual transform directory'}:acceptJavaScriptCallSpeciesConstruction(program,suspended,beginJavaScriptSpeciesConstruction(program,transition.heap,directory,transition.constructor,transition.fact,0));
  }
  case 'arrayConcatenationNeedAuthority':return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state})),constructor:transition.constructor};
  case 'arrayConcatenationCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arrayConcatenationThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arrayConcatenationError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arrayConcatenationInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayConcatenationResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination,realm,state:transition.state})),reason:transition.reason};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArraySliceTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArraySlice(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array transformation requires its actual active intrinsic frame'};
 switch(transition.$){
  case 'arraySliceContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state}))};
  case 'arraySliceCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arraySliceAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state})),prototype:transition.prototype};
  case 'arraySliceConstruct':{
   const suspended=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state})),directory=readJavaScriptCallArrayTransformDirectory(program,suspended,destination,realm);
   return directory===undefined?{$:'machineInvalid',heap:transition.heap,reason:'custom species construction retains its actual transform directory'}:acceptJavaScriptCallSpeciesConstruction(program,suspended,beginJavaScriptSpeciesConstruction(program,transition.heap,directory,transition.constructor,transition.fact,transition.state.count));
  }
  case 'arraySliceNeedAuthority':return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state})),constructor:transition.constructor};
  case 'arraySliceCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arraySliceThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arraySliceError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arraySliceInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arraySliceResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination,realm,state:transition.state})),reason:transition.reason};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayTransformTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayTransform(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array transformation requires its actual active intrinsic frame'};
 switch(transition.$){
  case 'arrayTransformContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state}))};
  case 'arrayTransformCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arrayTransformAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state})),prototype:transition.prototype};
  case 'arrayTransformConstruct':{
   const suspended=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state})),directory=readJavaScriptCallArrayTransformDirectory(program,suspended,destination,realm);
   return directory===undefined?{$:'machineInvalid',heap:transition.heap,reason:'custom species construction retains its actual transform directory'}:acceptJavaScriptCallSpeciesConstruction(program,suspended,beginJavaScriptSpeciesConstruction(program,transition.heap,directory,transition.constructor,transition.fact,transition.state.operation==='Array.map'?transition.state.length:0));
  }
  case 'arrayTransformNeedAuthority':return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state})),constructor:transition.constructor};
  case 'arrayTransformCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arrayTransformThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arrayTransformError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arrayTransformInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayTransformResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination,realm,state:transition.state})),reason:transition.reason};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallArrayConstruction(program,machine,completion){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitArrayTransform'&&frame.pending.$!=='waitArraySlice'&&frame.pending.$!=='waitArrayConcatenation')return {$:'machineInvalid',heap:machine.heap,reason:'species construction resumption retains its actual pending transform'};
 const pending=frame.pending,directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'species construction retains its actual source directory'};
 if(pending.construction===undefined||stepJavaScriptSpeciesConstruction(program,machine.heap,directory,pending.construction).$!=='speciesConstructionNeedImplementation')return {$:'machineInvalid',heap:machine.heap,reason:'external species construction completion requires its actual specialization request'};
 return pending.$==='waitArrayConcatenation'?acceptJavaScriptCallArrayConcatenation(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationConstruction(machine.heap,directory,pending.state,completion)):pending.$==='waitArraySlice'?acceptJavaScriptCallArraySlice(program,machine,pending.destination,pending.realm,resumeJavaScriptArraySliceConstruction(machine.heap,directory,pending.state,completion)):acceptJavaScriptCallArrayTransform(program,machine,pending.destination,pending.realm,resumeJavaScriptArrayTransformConstruction(machine.heap,directory,pending.state,completion));
}

/** Metadata extension is explicit and preserves every prior owner/root fact.
 * It updates no executable operation, register, callback or held machine value.
 * @typedef {{$:'arrayAuthorityUpdate',program:JavaScriptCallProgram,machine:JavaScriptCallMachine,error:string}} JavaScriptArrayAuthorityUpdate */
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptArraySpeciesDirectory} directory @returns {JavaScriptArrayAuthorityUpdate} */
export function extendJavaScriptCallArrayAuthority(program,machine,directory){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'arrayAuthorityUpdate',program,machine,error};
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority requires its actual suspended source frame'};
 const pending=frame.pending,global=pending.$==='waitConstruction'||pending.$==='waitConstructorCheck'||pending.$==='waitArrayCreation';
 if(!global&&pending.$!=='waitArrayTransform'&&pending.$!=='waitArraySlice'&&pending.$!=='waitArrayConcatenation')return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension requires its actual pending classification or realm lookup'};
 const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineNeedConstructorAuthority')return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority cannot replace a selected body getter or completed operation'};
 const prior=global?program.constructorDirectory:readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(prior===undefined)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority retains its actual prior source directory'};
 const invalid=readJavaScriptArraySpeciesDirectoryError(machine.heap,directory);if(invalid!=='')return {$:'arrayAuthorityUpdate',program,machine,error:invalid};
 const snapshots=program.constructorDirectory!==undefined&&program.constructorDirectory!==prior?[prior,program.constructorDirectory]:[prior];
 for(const retained of snapshots){
  if(directory.realm!==retained.realm||!testJavaScriptSemanticSameValue(directory.arrayConstructor,retained.arrayConstructor)||!testJavaScriptSemanticSameValue(directory.arrayPrototype,retained.arrayPrototype)||!testJavaScriptSemanticSameValue(directory.speciesKey,retained.speciesKey))return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority cannot replace existing intrinsic roots'};
  for(const fact of retained.constructors){const next=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,fact.identity)];if(next===undefined||next.constructable!==fact.constructable||next.realm!==fact.realm||next.intrinsicArray!==fact.intrinsicArray||next.intrinsicKind!==fact.intrinsicKind)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established classification'};}
  for(const root of retained.constructorRealms??[]){const next=(directory.constructorRealms??[]).find(row=>row.realm===root.realm);if(next===undefined)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established realm root'};const before=[root.objectPrototype,root.arrayPrototype,root.arrayConstructor,root.numberPrototype,root.booleanPrototype,root.stringPrototype,root.integerPrototype,root.symbolPrototype,root.mapPrototype,root.setPrototype,root.weakMapPrototype,root.weakSetPrototype],after=[next.objectPrototype,next.arrayPrototype,next.arrayConstructor,next.numberPrototype,next.booleanPrototype,next.stringPrototype,next.integerPrototype,next.symbolPrototype,next.mapPrototype,next.setPrototype,next.weakMapPrototype,next.weakSetPrototype];for(let at=0;at<before.length;at++){const value=before[at],updated=after[at];if(value!==undefined&&(updated===undefined||!testJavaScriptSemanticSameValue(updated,value)))return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established realm root'};}}
 }

 const functions=global?program.functions:program.functions.map(fn=>fn.identity!==frame.identity?fn:{...fn,instructions:fn.instructions.map((instruction,index)=>index===frame.position-1&&(instruction.$==='arrayTransform'||instruction.$==='arraySlice'||instruction.$==='arrayConcatenation')?{...instruction,species:directory}:instruction)}),next={...program,functions,...(program.constructorDirectory===undefined?{}:{constructorDirectory:directory})},stateError=readJavaScriptCallMachineError(next,machine);
 return stateError===''?{$:'arrayAuthorityUpdate',program:next,machine,error:''}:{$:'arrayAuthorityUpdate',program,machine,error:stateError};
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSpeciesConstructionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallSpeciesConstruction(program,machine,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined||frame.pending.$!=='waitArrayTransform'&&frame.pending.$!=='waitArraySlice'&&frame.pending.$!=='waitArrayConcatenation')return {$:'machineInvalid',heap:transition.heap,reason:'species constructor continuation retains its actual pending transformation'};
 const pending=frame.pending,directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'species constructor retains its actual source directory'};
 switch(transition.$){
  case 'speciesConstructionContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state}))};
  case 'speciesConstructionAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state})),prototype:transition.prototype};
  case 'speciesConstructionCall':{const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state}));return transition.constructing?pushJavaScriptSpeciesConstructorBody(program,next,transition.callee,transition.receiver,transition.arguments):pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);}
  case 'speciesConstructionNeedAuthority':return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state})),constructor:transition.constructor};
  case 'speciesConstructionNeedImplementation':{const actual=classifyJavaScriptArraySpeciesConstructor(transition.heap,directory,transition.constructor);if(actual.$!=='arrayConstructorKnown')return {$:'machineInvalid',heap:transition.heap,reason:'species specialization retains its actual delegated target authority'};return {$:'machineConstructArraySpecies',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state})),constructor:transition.constructor,arguments:transition.arguments??[{$:'number',value:transition.state.length}],fact:{...actual.fact},...(transition.newTarget===undefined?{}:{newTarget:transition.newTarget}),...(transition.operation===undefined?{}:{operation:transition.operation}),...(transition.reason===undefined?{}:{reason:transition.reason})};}
  case 'speciesConstructionCompleted':{const {construction:previous,...base}=pending,next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,base));return pending.$==='waitArrayConcatenation'?acceptJavaScriptCallArrayConcatenation(program,next,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationConstruction(transition.heap,directory,pending.state,{$:'normalCompletion',value:transition.value})):pending.$==='waitArraySlice'?acceptJavaScriptCallArraySlice(program,next,pending.destination,pending.realm,resumeJavaScriptArraySliceConstruction(transition.heap,directory,pending.state,{$:'normalCompletion',value:transition.value})):acceptJavaScriptCallArrayTransform(program,next,pending.destination,pending.realm,resumeJavaScriptArrayTransformConstruction(transition.heap,directory,pending.state,{$:'normalCompletion',value:transition.value}));}
  case 'speciesConstructionThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'speciesConstructionError':return requestJavaScriptCallError(current,transition.realm??pending.realm,transition.errorClass,transition.reason);
  case 'speciesConstructionInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'speciesConstructionResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state})),reason:transition.reason};
 }
}
/** Execute actual checked base class/ordinary function code under Construct.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} constructor @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptSpeciesConstructorBody(program,machine,constructor,receiver,arguments_){
 const parent=machine.frames[machine.frames.length-1],operation=parent===undefined?undefined:readJavaScriptPendingConstruction(parent.pending);if(operation!==undefined)return pushJavaScriptConstructionBody(program,machine,constructor,receiver,arguments_,operation.newTarget);
 if(constructor.$!=='object'||!constructor.callable||receiver.$!=='object'||receiver.callable)return {$:'machineInvalid',heap:machine.heap,reason:'species constructor body retains its actual callable and allocated receiver'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,constructor.identity);if(fn===undefined||fn.receiverMode!=='strict'&&!testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'species base construction uses actual checked ordinary function/class code'};
 if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)){const error=readJavaScriptOrdinaryClassReceiverError(machine.heap,constructor,receiver);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};}
 let slots=fn.registers,argumentsCount=arguments_.length;for(const frame of machine.frames){slots+=frame.registers.length;argumentsCount+=frame.arguments.length;}if(machine.frames.length>=1024||slots>1048576||argumentsCount>1048576)return {$:'machineResourceLimit',machine,reason:'species constructor call exceeds its actual shared stack budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([{...createJavaScriptCallFrame(fn,receiver,arguments_),newTarget:constructor}])}};
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptArrayIterationTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayIteration(program,machine,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1],fn=frame===undefined?undefined:readJavaScriptExecutableCallFunction(program,transition.heap,frame.identity),instruction=frame===undefined?undefined:fn?.instructions[frame.position];
 if(frame===undefined||fn===undefined||instruction?.$!=='collectionIteratorResult'||instruction.kind!=='Array')return {$:'machineInvalid',heap:transition.heap,reason:'Array next retains its actual active source instruction'};
 switch(transition.$){
  case 'arrayIteratorContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayIteratorNext',destination:instruction.destination,realm:fn.realm,state:transition.state}))};
  case 'arrayIteratorCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayIteratorNext',destination:instruction.destination,realm:fn.realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arrayIteratorComponents':return acceptJavaScriptCollectionIteratorResultCompletion(program,current,frame,instruction.destination,fn.realm,beginJavaScriptArrayIteratorResult(transition.heap,transition.receiver,transition.done,transition.key,transition.value,transition.entry,frame.registers[instruction.arrayPrototype]??{$:'undefined'},frame.registers[instruction.objectPrototype]??{$:'undefined'}));
  case 'arrayIteratorThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arrayIteratorError':return requestJavaScriptCallError(current,fn.realm,transition.errorClass,transition.reason);
  case 'arrayIteratorResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayIteratorNext',destination:instruction.destination,realm:fn.realm,state:transition.state})),reason:transition.reason};
  case 'arrayIteratorInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  default:return {$:'machineInvalid',heap:transition.heap,reason:'Array next requires its actual advancement completion'};
 }
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArraySpreadTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArraySpread(program,machine,destination,realm,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array spread retains its actual active source frame'};
 switch(transition.$){
  case 'arraySpreadContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySpread',destination,realm,state:transition.state}))};
  case 'arraySpreadCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySpread',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
  case 'arraySpreadPrimitiveGet':return {$:'machineNeedPrimitiveIteratorGet',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySpread',destination,realm,state:transition.state})),receiver:transition.receiver,key:transition.key};
  case 'arraySpreadCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,{$:'number',value:transition.index}))};
  case 'arraySpreadThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'arraySpreadError':return requestJavaScriptCallError(current,realm,transition.errorClass,transition.reason);
  case 'arraySpreadInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'arraySpreadResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySpread',destination,realm,state:transition.state})),reason:transition.reason};
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallPrimitiveIteratorGet(program,machine,completion){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame?.pending.$!=='waitArraySpread')return {$:'machineInvalid',heap:machine.heap,reason:'primitive iterator response retains its actual suspended source operation'};
 const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineNeedPrimitiveIteratorGet')return {$:'machineInvalid',heap:machine.heap,reason:'primitive iterator response cannot replace a live iterator call'};
 return acceptJavaScriptCallArraySpread(program,machine,frame.pending.destination,frame.pending.realm,resumeJavaScriptArraySpread(machine.heap,frame.pending.state,completion,true));
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} prototype @returns {JavaScriptCallMachineTransition} */
export function resumeJavaScriptCallPrimitiveIteratorPrototype(program,machine,prototype){
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame===undefined||frame.pending.$!=='waitArraySpread'&&frame.pending.$!=='waitIteratorProtocol'&&frame.pending.$!=='waitObjectFromEntries'&&frame.pending.$!=='waitArrayCreation')return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response retains its actual suspended iterator acquisition'};
 const request=stepJavaScriptCallMachine(program,machine);if(request.$!=='machineNeedPrimitiveIteratorGet')return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response cannot replace a live getter or iterator call'};
 if(frame.pending.$==='waitArrayCreation')return acceptJavaScriptCallArrayCreation(program,machine,frame.pending.destination,frame.pending.realm,provideJavaScriptArrayCreationPrimitivePrototype(machine.heap,frame.pending.state,prototype));
 if(frame.pending.$==='waitObjectFromEntries')return acceptJavaScriptCallObjectFromEntries(program,machine,frame.pending.destination,frame.pending.realm,provideJavaScriptObjectFromEntriesPrimitivePrototype(machine.heap,frame.pending.state,prototype));
 if(frame.pending.$==='waitIteratorProtocol')return acceptJavaScriptCallIteratorProtocol(program,machine,frame.pending,provideJavaScriptIteratorPrimitivePrototype(machine.heap,frame.pending.state,prototype));
 return acceptJavaScriptCallArraySpread(program,machine,frame.pending.destination,frame.pending.realm,provideJavaScriptArraySpreadPrimitivePrototype(machine.heap,frame.pending.state,prototype));
}


/** @param {JavaScriptCallProgram} program @param {JavaScriptCallCompletionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallCompletion(program,transition){return transition.$==='callReturn'?finishJavaScriptCallReturn(program,transition.machine,transition.value):transition;}


/** @param {JavaScriptCallFrame} frame @returns {JavaScriptIteratorCompletion} */
export function readJavaScriptCallIteratorCloseCompletion(frame){
 for(let at=frame.handlers.length-1;at>=0;at--){const handler=frame.handlers[at];if(handler?.$==='callDeferredCompletion'){const original=handler.completion;return {$:'iteratorCompletion',kind:original.kind==='jump'?'normal':original.kind,value:original.value,target:-1};}}
 return {$:'iteratorCompletion',kind:'normal',value:{$:'undefined'},target:-1};
}
/** @param {JavaScriptCallFrame} frame @param {number} destination @param {number} done @param {JavaScriptSemanticValue} value @param {boolean} finished @returns {JavaScriptCallFrame} */
export function writeJavaScriptCallIteratorResult(frame,destination,done,value,finished){return writeJavaScriptCallRegister(writeJavaScriptCallRegister(frame,destination,value),done,{$:'boolean',value:finished});}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallIteratorOperation|JavaScriptCallPending} pending @param {JavaScriptIteratorProtocolTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallIteratorProtocol(program,machine,pending,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined||pending.$!=='waitIteratorProtocol'&&pending.$!=='iteratorOperation')return {$:'machineInvalid',heap:transition.heap,reason:'iterator protocol retains its actual suspended source operation'};
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},regions=frame.iterators??[];
 if(transition.$==='iteratorProtocolContinue'||transition.$==='iteratorProtocolCall'||transition.$==='iteratorProtocolPrimitiveGet'){
  const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitIteratorProtocol',destination:pending.destination,done:pending.done,slot:pending.slot,operation:pending.operation,realm:pending.realm,state:transition.state}));
  if(transition.$==='iteratorProtocolCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
  return transition.$==='iteratorProtocolPrimitiveGet'?{$:'machineNeedPrimitiveIteratorGet',machine:next,receiver:transition.receiver,key:transition.key}:{$:'machineContinue',machine:next};
 }
 if(transition.$==='iteratorProtocolInvalid')return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 const update=transition.$==='iteratorProtocolAcquired'?insertJavaScriptIteratorRegion(transition.heap,regions,pending.slot,transition.record):pending.operation==='close'?removeJavaScriptIteratorRegion(transition.heap,regions,pending.slot):transition.$==='iteratorProtocolStep'?updateJavaScriptIteratorRegion(transition.heap,regions,pending.slot,transition.record):pending.operation==='step'&&(transition.$==='iteratorProtocolError'||transition.$==='iteratorProtocolThrow')?updateJavaScriptIteratorRegion(transition.heap,regions,pending.slot,transition.state.record):{$:'iteratorRegionsUpdated',regions};
 if(update.$==='iteratorRegionsInvalid')return {$:'machineInvalid',heap:transition.heap,reason:update.reason};if(update.$==='iteratorRegionsResourceLimit')return {$:'machineResourceLimit',machine:current,reason:update.reason};
 const next=replaceJavaScriptCallFrame(transition.heap,current.frames,{...frame,iterators:update.regions,pending:{$:'callIdle'}});
 if(transition.$==='iteratorProtocolError')return requestJavaScriptCallError(next,pending.realm,transition.errorClass,transition.reason);
 if(transition.$==='iteratorProtocolThrow')return throwJavaScriptCallValue(next,transition.value);
 const result=transition.$==='iteratorProtocolStep'?transition.value:{$:'undefined'},done=transition.$==='iteratorProtocolStep'?transition.done:pending.operation==='close';return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,next.frames,writeJavaScriptCallIteratorResult(next.frames[next.frames.length-1]??frame,pending.destination,pending.done,result,done))};
}


/** @typedef {{$:'iteratorThrowContinue',machine:JavaScriptCallMachine,value:JavaScriptSemanticValue} | {$:'iteratorThrowTransition',transition:JavaScriptCallMachineTransition}} JavaScriptIteratorThrowResolution */
/** Resume the selected getter/method throw before crossing source finalizers.
 * @param {JavaScriptCallMachine} machine @param {JavaScriptSemanticValue} value @returns {JavaScriptIteratorThrowResolution} */
export function resumeJavaScriptCallIteratorThrow(machine,value){
 const frame=machine.frames[machine.frames.length-1];if(frame?.pending.$!=='waitIteratorProtocol')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:machine.heap,reason:'iterator throw retains its actual suspended source operation'}};
 const pending=frame.pending,transition=resumeJavaScriptIteratorProtocolCall(machine.heap,pending.state,{$:'throwCompletion',value}),regions=frame.iterators??[];
 if(transition.$!=='iteratorProtocolClosed'&&transition.$!=='iteratorProtocolThrow')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:transition.heap,reason:transition.$==='iteratorProtocolInvalid'?transition.reason:'selected iterator throw retains its actual abrupt or suppressed completion'}};
 const update=pending.operation==='close'?removeJavaScriptIteratorRegion(transition.heap,regions,pending.slot):pending.operation==='step'?updateJavaScriptIteratorRegion(transition.heap,regions,pending.slot,transition.$==='iteratorProtocolThrow'?transition.state.record:pending.state.record):{$:'iteratorRegionsUpdated',regions};
 if(update.$!=='iteratorRegionsUpdated')return {$:'iteratorThrowTransition',transition:{$:'machineInvalid',heap:transition.heap,reason:'iterator throw retains its actual region update'}};
 const next={...frame,iterators:update.regions,pending:{$:'callIdle'}},current=replaceJavaScriptCallFrame(transition.heap,machine.frames,next);
 return transition.$==='iteratorProtocolClosed'?{$:'iteratorThrowTransition',transition:{$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallIteratorResult(next,pending.destination,pending.done,{$:'undefined'},true))}}:{$:'iteratorThrowContinue',machine:current,value:transition.value};
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
 const frame=machine.frames[machine.frames.length-1];if(frame?.pending.$!=='waitObjectPrototype'||prototype.$!=='object'||prototype.callable)return {$:'machineInvalid',heap:machine.heap,reason:'primitive prototype response requires its actual pending source operation and intrinsic Object root'};
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
 const frame=machine.frames[machine.frames.length-1],pending=frame?.pending;if(frame===undefined||pending?.$!=='waitSymbolDescription')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol descriptions require their actual suspended source instruction'};
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
 const frame=machine.frames[machine.frames.length-1],pending=frame?.pending;if(frame===undefined||pending?.$!=='waitSymbolAllocation')return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires its actual suspended source instruction'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||machine.heap.symbols.includes(identity)||machine.heap.objects.some(object=>object.identity===identity))return {$:'machineInvalid',heap:machine.heap,reason:'Symbol allocation requires an unused actual positive unsigned64 global identity'};
 if(machine.heap.symbols.length>=16384)return {$:'machineResourceLimit',machine,reason:'Symbol allocation exceeds the selected 16384 identity profile'};
 if(pending.registryKey!==undefined){const registered=allocateJavaScriptRegisteredSymbol(machine.heap,identity,pending.registryKey);if(registered.$==='symbolRegistryValue')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(registered.heap,machine.frames,writeJavaScriptCallRegister({...frame,pending:{$:'callIdle'}},pending.destination,registered.value))};if(registered.$==='symbolRegistryResourceLimit')return {$:'machineResourceLimit',machine,reason:registered.reason};return {$:'machineInvalid',heap:machine.heap,reason:registered.reason};}
 const heap={...machine.heap,symbols:machine.heap.symbols.concat([identity]).toSorted((left,right)=>left<right?-1:left>right?1:0),symbolRegistrationModel:'actual global Symbol registry classification 1',symbolRegistrations:(machine.heap.symbolRegistrations??[]).concat([{$:'symbolRegistrationFact',identity,registered:false}]).toSorted((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0)};
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
 const constructable=checked.$==='arrayConstructorKnown'&&checked.fact.constructable;
 if(!constructable&&required){const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);return owner===undefined?{$:'machineInvalid',heap:machine.heap,reason:'constructor checks retain their actual source realm'}:requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected value is not a constructor');}
 return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,required?input:{$:'boolean',value:constructable}))};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptConstructionTransition} transition
 * @param {JavaScriptSemanticValue[]|undefined} [heldArguments] @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallConstruction(program,machine,destination,realm,transition,heldArguments){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'construction retains its actual source frame'};
 const held=heldArguments??(frame.pending.$==='waitConstruction'?frame.pending.heldArguments:undefined);
 switch(transition.$){
  case 'constructionContinue':case 'constructionCall':case 'constructionAllocate':case 'constructionNeedAuthority':case 'constructionNeedImplementation':case 'constructionResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitConstruction',destination,realm,state:transition.state,...(held===undefined?{}:{heldArguments:[...held]})}));
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

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayCreationState} state
 * @param {JavaScriptArrayCreationFallback} fallback @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallArrayFallback(program,machine,destination,realm,state,fallback){
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'ArrayCreate fallback retains its actual source frame'};
 if(!testJavaScriptArrayLength(fallback.length))return requestJavaScriptCallError(replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),realm,'RangeError','Invalid array length');
 const next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCreation',destination,realm,state,fallback}));let properties=1;for(const object of machine.heap.objects)properties+=object.properties.length;
 return machine.heap.objects.length>=4096||properties>16384?{$:'machineResourceLimit',machine:next,reason:'ArrayCreate exceeds shared Object or property storage bounds'}:{$:'machineAllocateObject',machine:next,prototype:fallback.prototype};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {number} destination @param {bigint} realm @param {JavaScriptArrayCreationTransition} transition
 * @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayCreation(program,machine,destination,realm,transition){
 const frame=machine.frames[machine.frames.length-1],current={$:'callMachine',heap:transition.heap,frames:machine.frames};if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array creation retains its actual source frame'};
 switch(transition.$){
  case 'arrayCreationContinue':case 'arrayCreationCall':case 'arrayCreationConstruct':case 'arrayCreationAllocate':case 'arrayCreationPrimitiveGet':case 'arrayCreationNeedError':case 'arrayCreationResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCreation',destination,realm,state:transition.state}));
   if(transition.$==='arrayCreationCall')return pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   if(transition.$==='arrayCreationConstruct'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array creation requires actual constructor authority'};const selected=classifyJavaScriptArraySpeciesConstructor(transition.heap,directory,transition.constructor);if(selected.$==='arrayConstructorUnknown')return {$:'machineNeedConstructorAuthority',machine:next,constructor:selected.constructor??transition.constructor};if(selected.$==='arrayConstructorKnown'&&selected.fact.constructable)return acceptJavaScriptCallArrayConstruction(program,next,beginJavaScriptConstruction(program,transition.heap,directory,transition.constructor,transition.constructor,transition.arguments));return requestJavaScriptCallArrayFallback(program,next,destination,realm,transition.state,transition.fallback);}
   if(transition.$==='arrayCreationAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   if(transition.$==='arrayCreationPrimitiveGet')return {$:'machineNeedPrimitiveIteratorGet',machine:next,receiver:transition.receiver,key:transition.key};
   if(transition.$==='arrayCreationNeedError')return {$:'machineAllocateError',machine:next,realm,errorClass:transition.errorClass,reason:transition.reason};
   return transition.$==='arrayCreationResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'arrayCreationCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
  case 'arrayCreationThrow':return throwJavaScriptCallValue(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),transition.value);
  case 'arrayCreationError':return requestJavaScriptCallError(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),realm,transition.errorClass,transition.reason);
  case 'arrayCreationInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** Construct occurs before GetIteratorFromMethod; no live outer iterator exists.
 * @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptConstructionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallArrayConstruction(program,machine,transition){
 const frame=machine.frames[machine.frames.length-1];if(frame?.pending.$!=='waitArrayCreation')return {$:'machineInvalid',heap:transition.heap,reason:'Array construction retains its actual outer continuation'};const pending=frame.pending,current={$:'callMachine',heap:transition.heap,frames:machine.frames};
 switch(transition.$){
  case 'constructionContinue':case 'constructionCall':case 'constructionAllocate':case 'constructionNeedAuthority':case 'constructionNeedImplementation':case 'constructionResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{...pending,construction:transition.state}));
   if(transition.$==='constructionCall')return transition.constructing?pushJavaScriptConstructionBody(program,next,transition.callee,transition.receiver,transition.arguments,transition.newTarget):pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);
   if(transition.$==='constructionAllocate')return {$:'machineAllocateObject',machine:next,prototype:transition.prototype};
   if(transition.$==='constructionNeedAuthority')return {$:'machineNeedConstructorAuthority',machine:next,constructor:transition.constructor};
   if(transition.$==='constructionNeedImplementation')return {$:'machineConstruct',machine:next,constructor:transition.constructor,newTarget:transition.newTarget,arguments:transition.arguments,operation:transition.operation,reason:transition.reason};
   return transition.$==='constructionResourceLimit'?{$:'machineResourceLimit',machine:next,reason:transition.reason}:{$:'machineContinue',machine:next};
  }
  case 'constructionCompleted':return acceptJavaScriptCallArrayCreation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCreationConstruction(transition.heap,pending.state,{$:'normalCompletion',value:transition.value}));
  case 'constructionThrow':return acceptJavaScriptCallArrayCreation(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCreationConstruction(transition.heap,pending.state,{$:'throwCompletion',value:transition.value}));
  case 'constructionError':return requestJavaScriptCallError(replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'callIdle'})),transition.realm??pending.realm,transition.errorClass,transition.reason);
  case 'constructionInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
 }
}
