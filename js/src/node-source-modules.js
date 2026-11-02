// Explicit Node embedding entry point. Represented bodies execute only in the
// portable machine; this module coordinates its qualified external operations.
export {readSuppliedSourceLibraryClosure} from './translation/source-library-closure.js';
export {captureSourceLibraryErrorClosure,checkSourceLibraryErrorClosure} from './translation/javascript-error-closure-source.js';
export {prepareJavaScriptErrorSourceClosure,beginJavaScriptErrorSourceClosure,advanceJavaScriptErrorSourceClosure,resumeJavaScriptErrorSourceClosure} from './translation/javascript-error-closure.js';
export {captureJavaScriptRuntimeDataModule,checkJavaScriptRuntimeDataModule} from './translation/runtime-data-module-source.js';
export {readActiveRuntimeDataVersions,readShippedRuntimeDataAdapterContract,createRuntimeDataModuleScope,prepareJavaScriptRuntimeDataModule} from './translation/runtime-data-module.js';
export {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from './translation/javascript-project-data-source.js';
export {readShippedJavaScriptProjectDataSource,prepareJavaScriptProjectDataModule} from './translation/javascript-project-data-module.js';
export {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from './translation/javascript-ordinary-class-source.js';
export {prepareJavaScriptOrdinaryClassModule} from './translation/javascript-ordinary-class-module.js';
export {captureJavaScriptOrdinaryClassSourceClosure,checkJavaScriptOrdinaryClassSourceClosure} from './translation/javascript-ordinary-class-closure-source.js';
export {prepareJavaScriptOrdinaryClassSourceClosure} from './translation/javascript-ordinary-class-closure.js';
import {runJavaScriptCallMachine,stepJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation,readJavaScriptCallMachineError,readJavaScriptCallProgramError} from './translation/javascript-call-machine.js';
import {resumeJavaScriptErrorModule} from './translation/javascript-error-module.js';
import {testJavaScriptSemanticSameValue} from './translation/javascript-semantic-values.js';
import {resumeJavaScriptSourceHostCapability} from './translation/javascript-host-capabilities.js';

/** Drive an actual prepared embedding to its next terminal/resource boundary.
 * No represented function, constructor, accessor or formatter is invoked as
 * a host fallback. Step exhaustion retains the exact resumable machine.
 * @param {object} embedding @param {object} transition
 * @param {{steps?:number}} options */
export function runJavaScriptSourceTransition(embedding,transition,{steps=65536}={}){
 if(!Number.isSafeInteger(steps)||Object.is(steps,-0)||steps<0||steps>1048576)throw new RangeError('source execution needs an explicit step budget from zero through 1048576');
 let module=embedding.module,program=module.program,current=transition,used=0,allocated=0;
 function adopt(value){
  if(value.$==='machineInvalid')return;
  const heap=value.machine?.heap??value.heap;if(!heap)return;
  const error=value.machine?readJavaScriptCallMachineError(program,value.machine):readJavaScriptCallProgramError(program,heap);
  if(error)throw new TypeError(error);
  embedding.adoptHeap(heap);
 }
 adopt(current);
 while(used<steps){
  if(current.$==='machineContinue'){
   const execution=runJavaScriptCallMachine(program,current.machine,steps-used,[]);
   current=execution.transition;used+=execution.steps;adopt(current);
   continue;
  }
  if(current.$==='machineAllocateObject'){
   // Re-derive the pending request before consuming an external global ID.
   const pending=stepJavaScriptCallMachine(program,current.machine);
   if(pending.$!=='machineAllocateObject'||!testJavaScriptSemanticSameValue(pending.prototype,current.prototype))throw new TypeError('source allocation must retain its actual pending prototype');
   current=resumeJavaScriptCallObjectAllocation(program,current.machine,embedding.allocateIdentity());
   if(current.$==='machineContinue')allocated+=1;
  }else if(current.$==='machineNeedObjectPrototype'){
   if(typeof embedding.resumeObjectPrimitivePrototype!=='function')break;const response=embedding.resumeObjectPrimitivePrototype(current);if(!response.resumed)break;current=response.transition;
  }else if(current.$==='machineAllocateSymbol'){
   if(typeof embedding.resumeSymbolAllocation!=='function')break;const response=embedding.resumeSymbolAllocation(current);if(!response.resumed)break;current=response.transition;if(current.$==='machineContinue')allocated+=1;
  }else if(current.$==='machineNeedSymbolDescription'){
   if(typeof embedding.resumeSymbolDescription!=='function')break;const response=embedding.resumeSymbolDescription(current);if(!response.resumed)break;current=response.transition;
  }else if(current.$==='machineNeedFunctionNameSymbolDescription'){
   if(typeof embedding.resumeFunctionNameSymbolDescription!=='function')break;const response=embedding.resumeFunctionNameSymbolDescription(current);if(!response.resumed)break;current=response.transition;
  }else if(current.$==='machineNeedPrimitiveIteratorGet'){
   if(typeof embedding.resumePrimitiveIteratorGet!=='function')break;const response=embedding.resumePrimitiveIteratorGet(current);if(!response.resumed)break;current=response.transition;
  }else if(current.$==='machineNeedConstructorAuthority'){
   if(typeof embedding.resumeArrayConstructorAuthority!=='function')break;
   const response=embedding.resumeArrayConstructorAuthority(current);
   if(!response||typeof response.updated!=='boolean'||!response.transition)throw new TypeError('constructor authority retains an explicit update and suspended transition');
   if(!response.updated)break;
   module=embedding.module;program=module.program;current=response.transition;
  }else if(current.$==='machineInvokeHostCapability'){
   current=resumeJavaScriptSourceHostCapability(module,current,embedding.authority);
  }else if(['machineAllocateErrorValue','machineInvokeErrorIntrinsic','machineAllocateError','machineAllocateSourceError'].includes(current.$)){
   current=resumeJavaScriptErrorModule(module,current,embedding.authority);
  }else break;
  used+=1;adopt(current);
 }
 if(current.$==='machineContinue')current={$:'machineResourceLimit',machine:current.machine,reason:'source execution exhausted its explicit transition budget'};
 adopt(current);
 return {$:'callExecution',transition:current,steps:used,objectIdentitiesUsed:allocated};
}

/** Explicitly retry a retained source machine with a new bounded work slice.
 * Storage/protocol exhaustion can return the same boundary until its actual
 * constraint is addressed. This never treats exhaustion as a normal result or
 * calls an original host body. The ordinary driver validates the retained
 * current program/heap/frames before adopting any state.
 * @param {object} embedding @param {object} transition
 * @param {{steps?:number}} options */
export function resumeJavaScriptSourceTransition(embedding,transition,options={}){
 if(transition?.$!=='machineResourceLimit'||!transition.machine)throw new TypeError('source retry requires its actual retained resource machine');
 return runJavaScriptSourceTransition(embedding,{$:'machineContinue',machine:transition.machine},options);
}
