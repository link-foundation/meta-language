import {testJavaScriptArraySearchOperation} from './javascript-array-index-rules.js';
import {beginJavaScriptArraySearch,stepJavaScriptArraySearch,resumeJavaScriptArraySearch,readJavaScriptArraySearchStateError} from './javascript-array-search.js';
import {testJavaScriptArrayCallbackOperation,beginJavaScriptArrayCallback,stepJavaScriptArrayCallback,resumeJavaScriptArrayCallback,readJavaScriptArrayCallbackStateError} from './javascript-array-callbacks.js';
import {beginJavaScriptObjectSpread,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread,readJavaScriptObjectSpreadStateError} from './javascript-object-spread.js';
import {allocateJavaScriptCallClosure,findJavaScriptCallTemplate,projectJavaScriptCallTemplateFunction,readJavaScriptExecutableCallFunction,readJavaScriptCallTemplateDirectoryError} from './javascript-call-templates.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {allocateJavaScriptCollectionIterator,readJavaScriptCollectionIteratorReceiverError} from './javascript-collection-iteration.js';
import {beginJavaScriptCollectionIteratorResult,stepJavaScriptCollectionIteratorResult,resumeJavaScriptCollectionIteratorResultAllocation,readJavaScriptCollectionIteratorResultStateError} from './javascript-collection-iterator-results.js';
import {testJavaScriptCollectionIteratorIntrinsicMode} from './javascript-collection-iterator-call-source.js';
// Numeric-hint continuations retain legacy Number state tags for compatibility.
// Their source instruction distinguishes Number and BigInt conversion semantics.
import {readJavaScriptIntegerProtocolOperationError} from './javascript-call-integer-conversion.js';
import {readJavaScriptTextProtocolOperationError} from './javascript-text-conversion-protocol.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptCallPrimitiveArityError,executeJavaScriptCallPrimitive} from './javascript-call-primitives.js';
import {testJavaScriptCallCodeCoordinate,findJavaScriptCallCodeFunction,readJavaScriptCallCodeRegisters,readJavaScriptCallCodeDestination,testJavaScriptCallStrictEqual} from './javascript-call-code.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition,insertJavaScriptOrdinaryObject,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {beginJavaScriptPropertyDefinition,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition,completeJavaScriptPropertyDefinition} from './javascript-property-definition.js';
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

/** @typedef {{$:'callHandler',target:number,destination:number}} JavaScriptCallHandler */
/** @typedef {{$:'callIdle'} | {$:'waitCall',destination:number} |
 * {$:'waitGet',destination:number} | {$:'waitSet',destination:number} | {$:'waitHostSet',destination:number} |
 * {$:'waitCollectionIteratorResult',destination:number,realm:bigint,state:JavaScriptCollectionIteratorResultState} |
 * {$:'waitArraySearch',destination:number,realm:bigint,state:JavaScriptArraySearchState} |
 * {$:'waitArrayCallback',destination:number,realm:bigint,state:JavaScriptArrayCallbackState} |
 * {$:'waitObjectSpread',destination:number,realm:bigint,state:JavaScriptObjectSpreadState} |
 * {$:'waitArrayAssignment',destination:number,realm:bigint,state:JavaScriptArrayAssignmentState} |
 * {$:'waitKey',destination:number,realm:bigint,state:JavaScriptPrimitiveConversionState} |
 * {$:'waitDefinition',destination:number,realm:bigint,state:JavaScriptPropertyDefinitionState} |
 * {$:'waitNumber',destination:number,realm:bigint,state:JavaScriptNumberCallState} |
 * {$:'waitText',destination:number,realm:bigint,state:JavaScriptTextCallState} |
 * {$:'waitSourceError',realm:bigint,errorClass:string,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'waitErrorValue',destination:number,realm:bigint,errorClass:string,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,message:JavaScriptSemanticValue,messagePresent:boolean,bindReceiver:boolean,beforeDirectory:JavaScriptErrorAllocationDirectory} |
 * {$:'waitErrorIntrinsic',destination:number} | {$:'waitHostCapability',destination:number,beforeDirectory:JavaScriptErrorAllocationDirectory} |
 * {$:'waitError',realm:bigint,errorClass:string,reason:string}} JavaScriptCallPending
 */
/** @typedef {{$:'callFrame',identity:bigint,position:number,registers:JavaScriptSemanticValue[],
 * arguments:JavaScriptSemanticValue[],handlers:JavaScriptCallHandler[],pending:JavaScriptCallPending}} JavaScriptCallFrame
 */
/** @typedef {{$:'callMachine',heap:JavaScriptOrdinaryHeap,frames:JavaScriptCallFrame[]}} JavaScriptCallMachine */
/** @typedef {{$:'machineContinue',machine:JavaScriptCallMachine} |
 * {$:'machineCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'machineAllocateObject',machine:JavaScriptCallMachine,prototype:JavaScriptSemanticValue} |
 * {$:'machineAllocateError',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,reason:string} |
 * {$:'machineAllocateSourceError',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'machineAllocateErrorValue',machine:JavaScriptCallMachine,realm:bigint,errorClass:string,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,message:JavaScriptSemanticValue,messagePresent:boolean} |
 * {$:'machineInvokeErrorIntrinsic',machine:JavaScriptCallMachine,realm:bigint,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'machineInvokeHostCapability',machine:JavaScriptCallMachine,realm:bigint,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
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
    if(readJavaScriptOrdinaryValueError(heap,fn.lexicalReceiver)!=='')return 'lexical receivers belong to the latest global context';
    if(testJavaScriptCollectionConstructorMode(fn.receiverMode)&&readJavaScriptCollectionConstructorKind(fn)==='')return 'collection constructors retain their actual single standard allocation kind';
    if(!testJavaScriptCallCoordinate(fn.registers,4097)||fn.registers===0||!testJavaScriptCallCoordinate(fn.parameters,fn.registers)||fn.parameters+fn.captures.length+1>fn.registers)return 'call function register and parameter coordinates must be complete';
    for(const capture of fn.captures)if(readJavaScriptOrdinaryValueError(heap,capture)!=='')return 'call captures belong to the latest global context';
    if(fn.instructions.length>65536||fn.instructions.length===0)return 'call code exceeds its explicit instruction budget or has no entry';
    for(const instruction of fn.instructions){
      if(testJavaScriptErrorConstructorMode(fn.receiverMode)&&readJavaScriptCallCodeDestination(instruction)===0)return 'derived this is initialized only by its actual super construction';
      if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)&&readJavaScriptCallCodeDestination(instruction)===0)return 'ordinary class this retains its actual allocated receiver';
      if((testJavaScriptCollectionConstructorMode(fn.receiverMode)||testJavaScriptCollectionIntrinsicMode(fn.receiverMode)||testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode))&&readJavaScriptCallCodeDestination(instruction)===0)return 'standard intrinsic frames retain their actual receiver register';
      for(const register of readJavaScriptCallRegisters(instruction))if(!testJavaScriptCallCoordinate(register,fn.registers))return 'call instructions require actual in-range register coordinates';
      switch(instruction.$){
        case 'jump':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call jumps require actual instruction targets';break;
        case 'branch':if(!testJavaScriptCallCoordinate(instruction.yes,fn.instructions.length)||!testJavaScriptCallCoordinate(instruction.no,fn.instructions.length))return 'call branches require both actual instruction targets';break;
        case 'pushHandler':if(!testJavaScriptCallCoordinate(instruction.target,fn.instructions.length))return 'call handlers require actual instruction targets';break;
        case 'argument':if(!testJavaScriptCallCoordinate(instruction.index,1048576))return 'call argument reads require exact bounded coordinates';break;
        case 'constant':if(readJavaScriptOrdinaryValueError(heap,instruction.value)!=='')return 'call constants belong to the latest global context';break;
        case 'primitive':{const error=readJavaScriptCallPrimitiveArityError(instruction.operation,instruction.arguments.length);if(error!=='')return error;break;}
        case 'numberBinary':if(!['add','subtract','multiply','divide','remainder','power','lessThan','equal'].includes(instruction.operator))return 'call number instructions require an actual registered operation';break;
        case 'propertyKey':if(!program.realms.includes(instruction.realm))return 'a property key operation retains its actual intrinsic realm';break;
        case 'propertyDefinition':if(!program.realms.includes(instruction.realm))return 'a property definition retains its actual intrinsic realm';break;
        case 'arraySearch':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArraySearchOperation(instruction.operation))return 'Array search instructions retain their actual standard callable owner and operation';break;
        case 'arrayCallback':if(!testJavaScriptPrimitiveIntrinsicMode(fn.receiverMode)||!testJavaScriptArrayCallbackOperation(instruction.operation))return 'Array callback instructions retain their actual standard owner and operation';break;
        case 'objectSpread':if(!program.realms.includes(instruction.realm))return 'object spread retains its actual intrinsic realm';break;
        case 'ordinaryDataProperty':if(!program.realms.includes(instruction.realm))return 'a data property definition retains its actual intrinsic realm';break;
        case 'constructErrorValue':{if(!program.realms.includes(instruction.realm))return 'Error value construction retains its actual intrinsic realm';const error=readJavaScriptErrorConstructionKindError(instruction.errorClass,instruction.bindReceiver,fn.receiverMode);if(error!=='')return error;break;}
        case 'readDerivedReceiver':if(!testJavaScriptErrorConstructorMode(fn.receiverMode))return 'derived receiver reads retain their actual constructor source owner';break;
        case 'invokeErrorIntrinsic':if(!testJavaScriptErrorIntrinsicMode(fn.receiverMode))return 'host Error intrinsic calls require an externally registered intrinsic owner';break;
        case 'invokeHostCapability':if(!testJavaScriptHostCapabilityMode(fn.receiverMode))return 'host capability calls require their explicitly qualified external source owner';break;
        case 'collectionIteratorAllocate':if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||!['Map','Set'].includes(instruction.kind)||!['keys','values','entries'].includes(instruction.iterationKind))return 'iterator allocation retains its actual standard operation kind and owner';break;
        case 'collectionIteratorResult':if(!testJavaScriptCollectionIteratorIntrinsicMode(fn.receiverMode)||!['Map','Set'].includes(instruction.kind))return 'iterator next retains its actual standard operation kind and owner';break;
        case 'collectionAllocate':if(!testJavaScriptCollectionConstructorMode(fn.receiverMode)||!['Map','Set'].includes(instruction.kind))return 'collection allocation retains its actual standard constructor kind and frame';break;
        case 'collectionCopy':if(!testJavaScriptCollectionConstructorMode(fn.receiverMode))return 'collection copying retains its actual standard constructor frame';break;
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
  return {$:'callFrame',identity:fn.identity,position:0,registers,arguments:arguments_,handlers:[],pending:{$:'callIdle'}};
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
    slots+=frame.registers.length;argumentSlots+=frame.arguments.length;if(slots>1048576||argumentSlots>1048576||frame.handlers.length>1024)return 'call frame storage exceeds its explicit resource budget';
    const frameError=readJavaScriptCallFrameError(program,machine.heap,frame,fn,index<machine.frames.length-1);
    if(frameError!=='')return frameError;
    const selectedParent=machine.frames[index-1];
    if(selectedParent!==undefined&&(selectedParent.pending.$==='waitArrayCallback'||selectedParent.pending.$==='waitArraySearch')){
      const call=selectedParent.pending.state.call;if(call===undefined)return 'an Array operation child retains its actual selected call';
      const error=readJavaScriptSelectedCallFrameError(fn,frame,call.callee,call.receiver,call.arguments);if(error!=='')return error;
    }
    const spreadParent=machine.frames[index-1];
    if(spreadParent!==undefined&&spreadParent.pending.$==='waitObjectSpread'){
      const state=spreadParent.pending.state,callee=state.callee,receiver=fn.receiverMode==='lexical'?fn.lexicalReceiver:state.input;
      if(state.phase!=='getter'||callee.$!=='object'||callee.identity!==frame.identity||frame.arguments.length!==0||!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},receiver))return 'an object spread child retains its actual getter, receiver and empty arguments';
    }
    if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)||testJavaScriptCollectionConstructorMode(fn.receiverMode)){
      const parent=machine.frames[index-1],caller=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity),instruction=parent===undefined||caller===undefined||parent.position===0?undefined:caller.instructions[parent.position-1];
      if(parent===undefined||parent.pending.$!=='waitCall'||instruction===undefined||(instruction.$!=='callClassConstructor'&&instruction.$!=='constructCollection')||instruction.destination!==parent.pending.destination)return 'a constructor frame retains its actual suspended source new operation';
      if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)!==(instruction.$==='callClassConstructor'))return 'a constructor frame retains its actual class or standard collection operation';
      const callee=parent.registers[instruction.callee]??{$:'undefined'},receiver=instruction.$==='callClassConstructor'?(parent.registers[instruction.receiver]??{$:'undefined'}):{$:'undefined'};
      if(callee.$!=='object'||callee.identity!==frame.identity||!testJavaScriptSemanticSameValue(receiver,frame.registers[0]??{$:'undefined'})||instruction.arguments.length!==frame.arguments.length)return 'an ordinary class frame retains its actual new callee, receiver and argument list';
      for(let at=0;at<instruction.arguments.length;at++)if(!testJavaScriptSemanticSameValue(parent.registers[instruction.arguments[at]??-1]??{$:'undefined'},frame.arguments[at]??{$:'undefined'}))return 'ordinary class frames cannot substitute their actual ordered constructor arguments';
    }
  }
  return '';
}

/** Validate all destination-bearing variants before their independent state checks.
 * Keeping this decision separate avoids copying later continuations per variant.
 * @param {JavaScriptCallPending} pending @param {number} registers @returns {boolean} */
export function testJavaScriptCallPendingDestination(pending,registers){
 switch(pending.$){
  case 'waitCall':case 'waitGet':case 'waitSet':case 'waitHostSet':case 'waitKey':case 'waitDefinition':case 'waitNumber':case 'waitText':case 'waitErrorValue':case 'waitErrorIntrinsic':case 'waitHostCapability':case 'waitCollectionIteratorResult':case 'waitArrayAssignment':case 'waitObjectSpread':case 'waitArrayCallback':case 'waitArraySearch':
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
      if(!testJavaScriptSemanticSameValue(frame.registers[0]??{$:'undefined'},closure.lexicalReceiver))return 'an arrow frame retains its actual lexical receiver';
      for(let index=0;index<closure.captures.length;index++)if(!testJavaScriptSemanticSameValue(frame.registers[fn.parameters+1+index]??{$:'undefined'},closure.captures[index]??{$:'undefined'}))return 'an arrow frame retains its actual immutable captured values';
    }
    if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)){const error=readJavaScriptOrdinaryClassReceiverError(heap,{$:'object',identity:fn.identity,callable:true},frame.registers[0]??{$:'undefined'});if(error!=='')return error;}
    for(const handler of frame.handlers)if(!testJavaScriptCallCoordinate(handler.destination,fn.registers)||!testJavaScriptCallCoordinate(handler.target,fn.instructions.length))return 'call handlers retain their complete register and instruction coordinates';
    const pending=frame.pending;
    if(!testJavaScriptCallPendingDestination(pending,fn.registers))return 'pending calls retain their actual destination register';
    switch(pending.$){
      case 'waitError':if(!program.realms.includes(pending.realm))return 'pending Error allocation retains its actual intrinsic realm';break;
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
      case 'waitArraySearch':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='arraySearch'||instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return 'Array search continuation retains its actual source operation, destination and realm';
        const error=readJavaScriptArraySearchStateError(heap,pending.state);if(error!=='')return error;
        const at=instruction.operation==='Array.at',search=at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},from=frame.arguments[at?0:1]??{$:'undefined'},present=frame.arguments.length>=(at?1:2);
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.searchElement,search)||!testJavaScriptSemanticSameValue(pending.state.fromIndex,from)||pending.state.fromPresent!==present||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey))return 'Array search cannot substitute its actual receiver, search value, index argument, presence or Symbol';
        if(suspended&&pending.state.call===undefined)return 'only a selected Array search getter or conversion can retain a child frame';
        break;
      }
      case 'waitArrayCallback':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='arrayCallback'||instruction.destination!==pending.destination||pending.realm!==fn.realm||pending.state.operation!==instruction.operation)return 'Array callback continuation retains its actual source operation, destination and realm';
        const error=readJavaScriptArrayCallbackStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.receiver,frame.registers[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.callback,frame.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.thisArgument,frame.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.exoticKey,program.exoticKey)||pending.state.initialPresent!==(frame.arguments.length>=2))return 'Array callbacks cannot substitute their receiver, callback, thisArg or well-known Symbol';
        if(suspended&&pending.state.call===undefined)return 'only an actual Array getter, conversion or callback can retain a child frame';
        break;
      }
      case 'waitObjectSpread':{
        const instruction=frame.position>0?fn.instructions[frame.position-1]:undefined;
        if(instruction===undefined||instruction.$!=='objectSpread'||instruction.realm!==pending.realm||instruction.destination!==pending.destination||!program.realms.includes(pending.realm))return 'object spread retains its actual source instruction, destination and realm';
        const error=readJavaScriptObjectSpreadStateError(heap,pending.state);if(error!=='')return error;
        if(!testJavaScriptSemanticSameValue(pending.state.target,frame.registers[instruction.target]??{$:'undefined'})||!testJavaScriptSemanticSameValue(pending.state.input,frame.registers[instruction.input]??{$:'undefined'}))return 'object spread cannot substitute its actual target or source input';
        if(suspended&&pending.state.phase!=='getter')return 'only an object spread getter can retain a suspended child call';
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
      case 'waitDefinition':if(!program.realms.includes(pending.realm))return 'pending property definition retains its actual intrinsic realm';break;
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
    if(suspended&&(pending.$==='callIdle'||pending.$==='waitError'||pending.$==='waitSourceError'||pending.$==='waitErrorValue'||pending.$==='waitErrorIntrinsic'||pending.$==='waitHostCapability'))return 'a suspended parent frame retains its actual pending call';
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
  let frames=machine.frames;
  while(frames.length>0){
    const frame=frames[frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'call stacks must be dense'};
    const handler=frame.handlers[frame.handlers.length-1];
    if(handler!==undefined){const restored={...writeJavaScriptCallRegister(frame,handler.destination,value),position:handler.target,handlers:frame.handlers.slice(0,-1)};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,restored)};}
    frames=frames.slice(0,-1);
  }
  return {$:'machineThrow',heap:machine.heap,value};
}
/** @param {JavaScriptCallMachine} machine @param {bigint} realm @param {string} errorClass @param {string} reason @returns {JavaScriptCallMachineTransition} */
export function requestJavaScriptCallError(machine,realm,errorClass,reason){
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Error allocation requires its actual active frame'};
  const next=replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitError',realm,errorClass,reason}));
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
  if(testJavaScriptErrorConstructorMode(fn.receiverMode))return requestJavaScriptCallError(machine,fn.realm,'TypeError','a derived Error constructor requires its actual new operation');
  if(testJavaScriptOrdinaryClassConstructorMode(fn.receiverMode)){const reason=readJavaScriptOrdinaryClassCallReason(machine.heap,fn.identity);return reason===''?{$:'machineInvalid',heap:machine.heap,reason:'an ordinary class call needs its actual named source declaration'}:requestJavaScriptCallError(machine,fn.realm,'TypeError',reason);}
  if(testJavaScriptCollectionConstructorMode(fn.receiverMode))return requestJavaScriptCallError(machine,fn.realm,'TypeError','Constructor '+readJavaScriptCollectionConstructorKind(fn)+" requires 'new'");
  let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;}
  if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'call stack storage exceeds the selected execution resource budget'};
  return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,receiver,arguments_)])}};
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
  const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'property definition needs its actual active frame'};
  switch(transition.$){
    case 'definitionContinuation':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitDefinition',destination,realm,state:transition.state}))};
    case 'definitionCall':return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitDefinition',destination,realm,state:transition.state})),transition.callee,transition.receiver,transition.arguments);
    case 'definitionCompleted':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,machine.frames,writeJavaScriptCallRegister(frame,destination,transition.value))};
    case 'definitionThrow':return throwJavaScriptCallValue(machine,transition.value);
    case 'definitionIntrinsicError':return requestJavaScriptCallError(machine,realm,transition.errorClass,transition.reason);
    case 'definitionInvalid':return {$:'machineInvalid',heap:machine.heap,reason:transition.reason};
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
    case 'waitCall':case 'waitGet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,value))};
    case 'waitSet':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,{$:'boolean',value:true}))};
    case 'waitHostSet':if(value.$!=='boolean')return {$:'machineInvalid',heap:machine.heap,reason:'an opaque Error Set completion retains its actual Boolean result'};return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,frames,writeJavaScriptCallRegister(parent,pending.destination,value))};
    case 'waitKey':return acceptJavaScriptCallKey(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyKeyConversion(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitDefinition':return acceptJavaScriptCallDefinition(program,current,pending.destination,pending.realm,resumeJavaScriptPropertyDefinition(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArraySearch':return acceptJavaScriptCallArraySearch(program,current,pending.destination,pending.realm,resumeJavaScriptArraySearch(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayCallback':return acceptJavaScriptCallArrayCallback(program,current,pending.destination,pending.realm,resumeJavaScriptArrayCallback(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitObjectSpread':return acceptJavaScriptCallObjectSpread(program,current,pending.destination,pending.realm,resumeJavaScriptObjectSpread(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitArrayAssignment':return acceptJavaScriptCallArrayAssignment(program,current,pending.destination,pending.realm,resumeJavaScriptArrayAssignment(machine.heap,pending.state,{$:'normalCompletion',value}));
    case 'waitNumber':return resumeJavaScriptCallNumber(program,current,pending.destination,pending.realm,pending.state,value);
    case 'waitText':return resumeJavaScriptCallText(program,current,pending.destination,pending.realm,pending.state,value);
    case 'callIdle':case 'waitError':case 'waitSourceError':case 'waitErrorValue':case 'waitErrorIntrinsic':case 'waitHostCapability':case 'waitCollectionIteratorResult':return {$:'machineInvalid',heap:machine.heap,reason:'a returning call needs its actual suspended parent continuation'};
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
      if(instruction.$==='collectionIteratorAllocate'){
        const receiver=readJavaScriptCollectionIteratorReceiverError(machine.heap,registers[0]??{$:'undefined'},instruction.kind);
        if(receiver.error!==''){
          const owner=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
          return receiver.typeError&&owner!==undefined?requestJavaScriptCallError(machine,owner.realm,'TypeError',receiver.error):{$:'machineInvalid',heap:machine.heap,reason:receiver.error};
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
    case 'waitArraySearch':return acceptJavaScriptCallArraySearch(program,machine,pending.destination,pending.realm,stepJavaScriptArraySearch(machine.heap,pending.state));
    case 'waitArrayCallback':return acceptJavaScriptCallArrayCallback(program,machine,pending.destination,pending.realm,stepJavaScriptArrayCallback(machine.heap,pending.state));
    case 'waitObjectSpread':return pending.state.phase==='getter'?pushJavaScriptCall(program,machine,pending.state.callee,pending.state.input,[]):acceptJavaScriptCallObjectSpread(program,machine,pending.destination,pending.realm,stepJavaScriptObjectSpread(machine.heap,pending.state));
    case 'waitArrayAssignment':return acceptJavaScriptCallArrayAssignment(program,machine,pending.destination,pending.realm,stepJavaScriptArrayAssignment(machine.heap,pending.state));
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
    case 'constant':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,instruction.value))};
    case 'copy':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,registers[instruction.source]??{$:'undefined'}))};
    case 'argument':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,writeJavaScriptCallRegister(next,instruction.destination,frame.arguments[instruction.index]??{$:'undefined'}))};
    case 'jump':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:instruction.target})};
    case 'branch':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,position:convertJavaScriptSemanticBoolean(registers[instruction.condition]??{$:'undefined'})?instruction.yes:instruction.no})};
    case 'return':return returnJavaScriptCallValue(program,current,registers[instruction.source]??{$:'undefined'});
    case 'throw':return throwJavaScriptCallValue(current,registers[instruction.source]??{$:'undefined'});
    case 'pushHandler':{
      if(frame.handlers.length>=1024)return {$:'machineResourceLimit',machine,reason:'handler storage exceeds the selected execution resource budget'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.concat([{$:'callHandler',target:instruction.target,destination:instruction.destination}])})};
    }
    case 'popHandler':{
      if(frame.handlers.length===0)return {$:'machineInvalid',heap:machine.heap,reason:'a handler pop requires its actual active handler'};
      return {$:'machineContinue',machine:replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,handlers:frame.handlers.slice(0,-1)})};
    }
    case 'call':{
      const arguments_=instruction.arguments.map(index=>registers[index]??{$:'undefined'});
      return pushJavaScriptCall(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},arguments_);
    }
    case 'constructErrorClass':return pushJavaScriptErrorConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'callClassConstructor':return pushJavaScriptOrdinaryClassConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},registers[instruction.receiver]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'constructCollection':return pushJavaScriptCollectionConstructor(program,replaceJavaScriptCallFrame(machine.heap,machine.frames,{...next,pending:{$:'waitCall',destination:instruction.destination}}),registers[instruction.callee]??{$:'undefined'},instruction.arguments.map(index=>registers[index]??{$:'undefined'}));
    case 'collectionIteratorResult':return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,instruction.destination,fn.realm,beginJavaScriptCollectionIteratorResult(machine.heap,registers[0]??{$:'undefined'},instruction.kind,registers[instruction.arrayPrototype]??{$:'undefined'},registers[instruction.objectPrototype]??{$:'undefined'}));
    case 'collectionOperation':return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,executeJavaScriptCollectionOperation(machine.heap,registers[0]??{$:'undefined'},instruction.operation,frame.arguments));
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
    case 'closureAllocate':case 'ordinaryAllocate':case 'arrayAllocate':case 'collectionAllocate':case 'collectionIteratorAllocate':case 'ordinaryGet':case 'ordinarySet':case 'requireWrite':return executeJavaScriptOrdinaryCallProperty(program,machine,frame,instruction);
    case 'propertyKey':return acceptJavaScriptCallKey(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyKeyConversion(machine.heap,registers[instruction.input]??{$:'undefined'},program.exoticKey));
    case 'propertyDefinition':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,beginJavaScriptPropertyDefinition(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},registers[instruction.attributes]??{$:'undefined'},program.exoticKey));
    case 'arraySearch':{const at=instruction.operation==='Array.at';return acceptJavaScriptCallArraySearch(program,current,instruction.destination,fn.realm,beginJavaScriptArraySearch(machine.heap,instruction.operation,registers[0]??{$:'undefined'},at?{$:'undefined'}:frame.arguments[0]??{$:'undefined'},frame.arguments[at?0:1]??{$:'undefined'},frame.arguments.length>=(at?1:2),program.exoticKey));}
    case 'arrayCallback':return acceptJavaScriptCallArrayCallback(program,current,instruction.destination,fn.realm,beginJavaScriptArrayCallback(machine.heap,instruction.operation,registers[0]??{$:'undefined'},frame.arguments[0]??{$:'undefined'},frame.arguments[1]??{$:'undefined'},program.exoticKey,frame.arguments.length>=2));
    case 'objectSpread':return acceptJavaScriptCallObjectSpread(program,current,instruction.destination,instruction.realm,beginJavaScriptObjectSpread(machine.heap,registers[instruction.target]??{$:'undefined'},registers[instruction.input]??{$:'undefined'}));
    case 'ordinaryDataProperty':return acceptJavaScriptCallDefinition(program,current,instruction.destination,instruction.realm,completeJavaScriptPropertyDefinition(machine.heap,registers[instruction.object]??{$:'undefined'},registers[instruction.key]??{$:'undefined'},createJavaScriptOrdinarySetDescriptor(registers[instruction.value]??{$:'undefined'},true)));
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
 if(frame!==undefined&&frame.pending.$==='waitCollectionIteratorResult')return acceptJavaScriptCollectionIteratorResultCompletion(program,machine,frame,frame.pending.destination,frame.pending.realm,resumeJavaScriptCollectionIteratorResultAllocation(machine.heap,frame.pending.state,identity));
 if(frame===undefined||frame.pending.$!=='callIdle')return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual active allocation instruction'};
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,frame.identity);
 if(fn===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual source function'};
 const instruction=fn.instructions[frame.position];
 if(instruction===undefined||instruction.$!=='ordinaryAllocate'&&instruction.$!=='arrayAllocate'&&instruction.$!=='collectionAllocate'&&instruction.$!=='collectionIteratorAllocate'&&instruction.$!=='closureAllocate')return {$:'machineInvalid',heap:machine.heap,reason:'Object resumption requires its actual allocation instruction'};
 if(instruction.$==='closureAllocate'){
  const allocation=allocateJavaScriptCallClosure(program,machine.heap,identity,instruction.owner,instruction.ordinal,instruction.captures.map(index=>frame.registers[index]??{$:'undefined'}),frame.registers[instruction.receiver]??{$:'undefined'});
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
 if(instruction.$==='collectionIteratorAllocate'){
  const allocation=allocateJavaScriptCollectionIterator(machine.heap,identity,prototype,frame.registers[0]??{$:'undefined'},instruction.kind,instruction.iterationKind);
  if(allocation.$==='collectionIteratorAllocated')return {$:'machineContinue',machine:replaceJavaScriptCallFrame(allocation.heap,machine.frames,writeJavaScriptCallRegister(advanceJavaScriptCallFrame(frame),instruction.destination,allocation.value))};
  if(allocation.$==='collectionIterationResourceLimit')return {$:'machineResourceLimit',machine,reason:allocation.reason};
  if(allocation.$==='collectionIterationTypeError')return requestJavaScriptCallError(machine,fn.realm,'TypeError',allocation.reason);
  return {$:'machineInvalid',heap:machine.heap,reason:allocation.$==='collectionIterationInvalid'?allocation.reason:'iterator allocation needs its actual completed creation'};
 }
 if(instruction.$==='collectionAllocate'){
  const allocation=allocateJavaScriptCollection(machine.heap,identity,prototype,instruction.kind);if(allocation.$!=='collectionNormal')return acceptJavaScriptCollectionCompletion(program,machine,frame,instruction.destination,allocation);
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
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'derived constructor frames exceed the explicit execution storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,{$:'undefined'},arguments_)])}};
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
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'ordinary class frames exceed the explicit execution storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,receiver,arguments_)])}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue[]} arguments_
 * @returns {JavaScriptCallMachineTransition} */
export function pushJavaScriptCollectionConstructor(program,machine,callee,arguments_){
 const parent=machine.frames[machine.frames.length-1],owner=parent===undefined?undefined:readJavaScriptExecutableCallFunction(program,machine.heap,parent.identity);if(owner===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'collection construction needs its actual suspended caller'};
 if(callee.$!=='object'||!callee.callable)return requestJavaScriptCallError(machine,owner.realm,'TypeError','the selected collection is not a constructor');
 const fn=readJavaScriptExecutableCallFunction(program,machine.heap,callee.identity);if(fn===undefined||!testJavaScriptCollectionConstructorMode(fn.receiverMode))return {$:'machineInvalid',heap:machine.heap,reason:'collection construction requires its actual standard registered constructor'};
 let slots=fn.registers,argumentSlots=arguments_.length;for(const active of machine.frames){slots+=active.registers.length;argumentSlots+=active.arguments.length;}
 if(machine.frames.length>=1024||slots>1048576||argumentSlots>1048576)return {$:'machineResourceLimit',machine,reason:'collection constructor frames exceed their actual storage budget'};
 return {$:'machineContinue',machine:{$:'callMachine',heap:machine.heap,frames:machine.frames.concat([createJavaScriptCallFrame(fn,{$:'undefined'},arguments_)])}};
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
  case 'objectSpreadError':return requestJavaScriptCallError(current,realm,'TypeError',transition.reason);
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
