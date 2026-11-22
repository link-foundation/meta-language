import {readJavaScriptPendingConstruction} from './javascript-construction-parents.js';
import {resumeJavaScriptArrayConcatenationConstruction} from './javascript-array-concatenation.js';
import {resumeJavaScriptArraySliceConstruction} from './javascript-array-slice.js';
import {resumeJavaScriptArrayCreationConstruction} from './javascript-array-creation.js';
import {beginJavaScriptConstruction} from './javascript-construction.js';
import {beginJavaScriptSpeciesConstruction,stepJavaScriptSpeciesConstruction} from './javascript-species-construction.js';
import {readJavaScriptKnownArrayConstructorFact,classifyJavaScriptArraySpeciesConstructor,readJavaScriptArraySpeciesDirectoryError,findJavaScriptArrayConstructorFact,readJavaScriptConstructorRealmFallback} from './javascript-array-species.js';
import {resumeJavaScriptArrayTransformConstruction} from './javascript-array-transform.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {beginJavaScriptArrayIteratorResult} from './javascript-collection-iterator-results.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {testJavaScriptOrdinaryClassConstructorMode,readJavaScriptOrdinaryClassReceiverError} from './javascript-ordinary-class-rules.js';
import {testJavaScriptArrayLength} from './javascript-array-objects.js';
import {replaceJavaScriptCallFrame,writeJavaScriptCallRegister,replaceJavaScriptCallPending,createJavaScriptCallFrame,readJavaScriptCallMachineError,throwJavaScriptCallValue,requestJavaScriptCallError,pushJavaScriptCall,stepJavaScriptCallMachine,acceptJavaScriptCollectionIteratorResultCompletion,pushJavaScriptConstructionBody} from './javascript-call-machine.js';

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
 const error=readJavaScriptCallMachineError(program,machine);if(error!=='')return {$:'machineInvalid',heap:machine.heap,reason:error};const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'species construction needs its actual active source frame'};
 const pending=frame.pending;if(pending.$!=='waitArrayTransform'&&pending.$!=='waitArraySlice'&&pending.$!=='waitArrayConcatenation')return {$:'machineInvalid',heap:machine.heap,reason:'species construction resumption retains its actual pending transform'};
 const directory=readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:machine.heap,reason:'species construction retains its actual source directory'};
 const construction=pending.construction;
 if(construction===undefined||stepJavaScriptSpeciesConstruction(program,machine.heap,directory,construction).$!=='speciesConstructionNeedImplementation')return {$:'machineInvalid',heap:machine.heap,reason:'external species construction completion requires its actual specialization request'};
 return resumeJavaScriptCallArraySpeciesOperation(program,machine,frame,directory,completion,false);
}

/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptCallFrame} frame @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptPrimitiveConversionCompletion} completion @param {boolean} clearConstruction
 * @returns {JavaScriptCallMachineTransition} */
function resumeJavaScriptCallArraySpeciesOperation(program,machine,frame,directory,completion,clearConstruction){
 const pending=frame.pending;
 switch(pending.$){
  case 'waitArrayConcatenation':{
   const next=clearConstruction?replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayConcatenation',destination:pending.destination,realm:pending.realm,state:pending.state})):machine;
   return acceptJavaScriptCallArrayConcatenation(program,next,pending.destination,pending.realm,resumeJavaScriptArrayConcatenationConstruction(machine.heap,directory,pending.state,completion));
  }
  case 'waitArraySlice':{
   const next=clearConstruction?replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArraySlice',destination:pending.destination,realm:pending.realm,state:pending.state})):machine;
   return acceptJavaScriptCallArraySlice(program,next,pending.destination,pending.realm,resumeJavaScriptArraySliceConstruction(machine.heap,directory,pending.state,completion));
  }
  case 'waitArrayTransform':{
   const next=clearConstruction?replaceJavaScriptCallFrame(machine.heap,machine.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayTransform',destination:pending.destination,realm:pending.realm,state:pending.state})):machine;
   return acceptJavaScriptCallArrayTransform(program,next,pending.destination,pending.realm,resumeJavaScriptArrayTransformConstruction(machine.heap,directory,pending.state,completion));
  }
  default:return {$:'machineInvalid',heap:machine.heap,reason:'species construction resumption retains its actual pending transform'};
 }
}

/** @param {JavaScriptCallInstruction} instruction @param {JavaScriptArraySpeciesDirectory} species @returns {JavaScriptCallInstruction} */
function replaceJavaScriptInstructionSpecies(instruction,species){
 switch(instruction.$){
  case 'arrayTransform':{
   const input=instruction.input;
   return {$:'arrayTransform',destination:instruction.destination,operation:instruction.operation,species,...(input===undefined?{}:{input})};
  }
  case 'arraySlice':{
   const input=instruction.input;
   return {$:'arraySlice',destination:instruction.destination,species,...(input===undefined?{}:{input})};
  }
  case 'arrayConcatenation':{
   const input=instruction.input;
   return {$:'arrayConcatenation',destination:instruction.destination,spreadableKey:instruction.spreadableKey,species,...(input===undefined?{}:{input})};
  }
  default:return instruction;
 }
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
 const prior=pending.$==='waitArrayTransform'||pending.$==='waitArraySlice'||pending.$==='waitArrayConcatenation'?readJavaScriptCallArrayTransformDirectory(program,machine,pending.destination,pending.realm):program.constructorDirectory;
 if(prior===undefined)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority retains its actual prior source directory'};
 const invalid=readJavaScriptArraySpeciesDirectoryError(machine.heap,directory);if(invalid!=='')return {$:'arrayAuthorityUpdate',program,machine,error:invalid};
 const rootDirectory=program.constructorDirectory;
 const snapshots=[prior];if(rootDirectory!==undefined&&!global)snapshots.push(rootDirectory);
 for(const retained of snapshots){
  if(directory.realm!==retained.realm||!testJavaScriptSemanticSameValue(directory.arrayConstructor,retained.arrayConstructor)||!testJavaScriptSemanticSameValue(directory.arrayPrototype,retained.arrayPrototype)||!testJavaScriptSemanticSameValue(directory.speciesKey,retained.speciesKey))return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority cannot replace existing intrinsic roots'};
  for(const fact of retained.constructors){const next=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,fact.identity)];if(next===undefined||next.constructable!==fact.constructable||next.realm!==fact.realm||next.intrinsicArray!==fact.intrinsicArray||next.intrinsicKind!==fact.intrinsicKind)return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established classification'};}
  for(const root of retained.constructorRealms??[]){
   const next=(directory.constructorRealms??[]).find(row=>row.realm===root.realm);
   if(next===undefined||!testJavaScriptSemanticSameValue(next.objectPrototype,root.objectPrototype)||!testJavaScriptSemanticSameValue(next.arrayPrototype,root.arrayPrototype)||!testJavaScriptSemanticSameValue(next.arrayConstructor,root.arrayConstructor))return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established realm root'};
   for(let index=0;index<9;index++){
    const value=readJavaScriptConstructorRealmFallback(root,index),updated=readJavaScriptConstructorRealmFallback(next,index);
    if(value!==undefined&&(updated===undefined||!testJavaScriptSemanticSameValue(updated,value)))return {$:'arrayAuthorityUpdate',program,machine,error:'constructor authority extension preserves every established realm root'};
   }
  }
 }

 const functions=global?program.functions:program.functions.map(fn=>fn.identity!==frame.identity?fn:{...fn,instructions:fn.instructions.map((instruction,index)=>index===frame.position-1?replaceJavaScriptInstructionSpecies(instruction,directory):instruction)});
 const next=program.constructorDirectory===undefined?{...program,functions}:{...program,functions,constructorDirectory:directory};
 const stateError=readJavaScriptCallMachineError(next,machine);
 return stateError===''?{$:'arrayAuthorityUpdate',program:next,machine,error:''}:{$:'arrayAuthorityUpdate',program,machine,error:stateError};
}

/** @param {JavaScriptCallPending} pending @returns {JavaScriptSpeciesConstructionState | undefined} */
function readJavaScriptPendingSpeciesConstruction(pending){
 switch(pending.$){
  case 'waitArrayTransform':return pending.construction;
  case 'waitArraySlice':return pending.construction;
  case 'waitArrayConcatenation':return pending.construction;
  default:return undefined;
 }
}

/** @param {JavaScriptCallPending} pending @param {JavaScriptSpeciesConstructionState} construction @returns {JavaScriptCallPending} */
function replaceJavaScriptPendingSpeciesConstruction(pending,construction){
 switch(pending.$){
  case 'waitArrayTransform':return {$:'waitArrayTransform',destination:pending.destination,realm:pending.realm,state:pending.state,construction};
  case 'waitArraySlice':return {$:'waitArraySlice',destination:pending.destination,realm:pending.realm,state:pending.state,construction};
  case 'waitArrayConcatenation':return {$:'waitArrayConcatenation',destination:pending.destination,realm:pending.realm,state:pending.state,construction};
  default:return pending;
 }
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptCallMachine} machine
 * @param {JavaScriptSpeciesConstructionTransition} transition @returns {JavaScriptCallMachineTransition} */
export function acceptJavaScriptCallSpeciesConstruction(program,machine,transition){
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'species construction needs its actual active source frame'};
 const pending=frame.pending;if(pending.$!=='waitArrayTransform'&&pending.$!=='waitArraySlice'&&pending.$!=='waitArrayConcatenation')return {$:'machineInvalid',heap:transition.heap,reason:'species constructor continuation retains its actual pending transformation'};
 const directory=readJavaScriptCallArrayTransformDirectory(program,current,pending.destination,pending.realm);if(directory===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'species constructor retains its actual source directory'};
 switch(transition.$){
  case 'speciesConstructionContinue':return {$:'machineContinue',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state)))};
  case 'speciesConstructionAllocate':return {$:'machineAllocateObject',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state))),prototype:transition.prototype};
  case 'speciesConstructionCall':{const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state)));return transition.constructing?pushJavaScriptSpeciesConstructorBody(program,next,transition.callee,transition.receiver,transition.arguments):pushJavaScriptCall(program,next,transition.callee,transition.receiver,transition.arguments);}
  case 'speciesConstructionNeedAuthority':return {$:'machineNeedConstructorAuthority',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state))),constructor:transition.constructor};
  case 'speciesConstructionNeedImplementation':{
   const actual=readJavaScriptKnownArrayConstructorFact(classifyJavaScriptArraySpeciesConstructor(transition.heap,directory,transition.constructor));
   if(actual===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'species specialization retains its actual delegated target authority'};
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state)));
   const newTarget=transition.newTarget,operation=transition.operation,reason=transition.reason;
   return {$:'machineConstructArraySpecies',machine:next,constructor:transition.constructor,arguments:transition.arguments??[{$:'number',value:transition.state.length}],fact:{...actual},
    ...(newTarget===undefined?{}:{newTarget}),...(operation===undefined?{}:{operation}),...(reason===undefined?{}:{reason})};
  }
  case 'speciesConstructionCompleted':return resumeJavaScriptCallArraySpeciesOperation(program,current,frame,directory,{$:'normalCompletion',value:transition.value},true);
  case 'speciesConstructionThrow':return throwJavaScriptCallValue(current,transition.value);
  case 'speciesConstructionError':return requestJavaScriptCallError(current,transition.realm??pending.realm,transition.errorClass,transition.reason);
  case 'speciesConstructionInvalid':return {$:'machineInvalid',heap:transition.heap,reason:transition.reason};
  case 'speciesConstructionResourceLimit':return {$:'machineResourceLimit',machine:replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,replaceJavaScriptPendingSpeciesConstruction(pending,transition.state))),reason:transition.reason};
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
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames},frame=current.frames[current.frames.length-1],fn=frame===undefined?undefined:readJavaScriptExecutableCallFunction(program,transition.heap,frame.identity),instruction=frame===undefined||fn===undefined?undefined:fn.instructions[frame.position];
 if(frame===undefined||fn===undefined||instruction===undefined||instruction.$!=='collectionIteratorResult'||instruction.kind!=='Array')return {$:'machineInvalid',heap:transition.heap,reason:'Array next retains its actual active source instruction'};
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
   if(transition.$==='arrayCreationConstruct'){const directory=program.constructorDirectory;if(directory===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array creation requires actual constructor authority'};const selected=classifyJavaScriptArraySpeciesConstructor(transition.heap,directory,transition.constructor);if(selected.$==='arrayConstructorUnknown')return {$:'machineNeedConstructorAuthority',machine:next,constructor:selected.constructor??transition.constructor};if(readJavaScriptKnownArrayConstructorFact(selected)?.constructable===true)return acceptJavaScriptCallArrayConstruction(program,next,beginJavaScriptConstruction(program,transition.heap,directory,transition.constructor,transition.constructor,transition.arguments));return requestJavaScriptCallArrayFallback(program,next,destination,realm,transition.state,transition.fallback);}
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
 const frame=machine.frames[machine.frames.length-1];if(frame===undefined)return {$:'machineInvalid',heap:transition.heap,reason:'Array construction retains its actual outer continuation'};
 const pending=frame.pending;if(pending.$!=='waitArrayCreation')return {$:'machineInvalid',heap:transition.heap,reason:'Array construction retains its actual outer continuation'};
 const current={$:'callMachine',heap:transition.heap,frames:machine.frames};
 switch(transition.$){
  case 'constructionContinue':case 'constructionCall':case 'constructionAllocate':case 'constructionNeedAuthority':case 'constructionNeedImplementation':case 'constructionResourceLimit':{
   const next=replaceJavaScriptCallFrame(transition.heap,current.frames,replaceJavaScriptCallPending(frame,{$:'waitArrayCreation',destination:pending.destination,realm:pending.realm,state:pending.state,construction:transition.state}));
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
