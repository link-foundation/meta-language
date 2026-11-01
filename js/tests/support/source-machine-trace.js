// Deferred native replay input. This helper executes only when a future control
// explicitly calls it; imports do not execute a represented source program.
import {runJavaScriptCallMachine,stepJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation,readJavaScriptCallMachineError,readJavaScriptCallProgramError} from '../../src/translation/javascript-call-machine.js';
import {resumeJavaScriptErrorModule} from '../../src/translation/javascript-error-module.js';
import {resumeJavaScriptSourceHostCapability} from '../../src/translation/javascript-host-capabilities.js';
import {testJavaScriptSemanticSameValue} from '../../src/translation/javascript-semantic-values.js';
import * as codecs from '../../../experiments/javascript-properties/call-machine-codecs.mjs';

export function readSourceMachineTrace(embedding,initial,{steps=65536}={}){
 if(!Number.isSafeInteger(steps)||Object.is(steps,-0)||steps<0||steps>1048576)throw new RangeError('source replay requires its actual bounded execution budget');
 const events=[],first=codecs.encodeJavaScriptCallMachineTransition(initial);
 let program=embedding.module.program,current=initial,used=0;
 function append(event){if(events.length>=4096)throw new RangeError('source replay exceeds its explicit event budget');events.push(event);}
 function adopt(){
  if(current.$==='machineInvalid')return;
  const heap=current.machine?.heap??current.heap;if(!heap)return;
  const error=current.machine?readJavaScriptCallMachineError(program,current.machine):readJavaScriptCallProgramError(program,heap);
  if(error)throw new TypeError(error);embedding.adoptHeap(heap);
 }
 adopt();
 while(used<steps){
  if(events.length>=4096)throw new RangeError('source replay exceeds its explicit event budget');
  if(current.$==='machineContinue'){
   const limit=Math.min(4096,steps-used),execution=runJavaScriptCallMachine(program,current.machine,limit,[]);
   append({operation:'run',steps:limit,expected:codecs.encodeJavaScriptCallExecution(execution)});
   current=execution.transition;used+=execution.steps;adopt();
   if(current.$==='machineResourceLimit'&&current.reason==='execution exhausted its explicit transition budget'&&execution.steps===limit&&used<steps){
    current={$:'machineContinue',machine:current.machine};
    append({operation:'resumeBatch',expected:codecs.encodeJavaScriptCallMachineTransition(current)});adopt();
   }
   if(execution.steps===0&&current.$==='machineContinue')throw new TypeError('source replay cannot invent progress for a suspended machine');
   continue;
  }
  if(current.$==='machineAllocateObject'){
   const actual=stepJavaScriptCallMachine(program,current.machine);
   if(actual.$!=='machineAllocateObject'||!testJavaScriptSemanticSameValue(actual.prototype,current.prototype))throw new TypeError('source replay allocation must retain its actual pending prototype');
   const identity=embedding.allocateIdentity();current=resumeJavaScriptCallObjectAllocation(program,current.machine,identity);
   append({operation:'allocateObject',identity:String(identity),expected:codecs.encodeJavaScriptCallMachineTransition(current)});
  }else if(current.$==='machineNeedConstructorAuthority'){
   if(typeof embedding.resumeArrayConstructorAuthority!=='function')break;
   const request=current,response=embedding.resumeArrayConstructorAuthority(request);if(!response.updated)break;
   program=embedding.module.program;current=response.transition;
   const frame=request.machine.frames[request.machine.frames.length-1],owner=program.functions.find(fn=>fn.identity===frame.identity),instruction=owner?.instructions[frame.position-1];
   if(instruction?.$!=='arrayTransform')throw new TypeError('constructor replay retains its actual selected Array transform instruction');
   append({operation:'extendConstructorAuthority',constructor:codecs.encodeJavaScriptSemanticValue(request.constructor),directory:codecs.encodeJavaScriptArraySpeciesDirectory(instruction.species),program:codecs.encodeJavaScriptCallProgram(program),expected:codecs.encodeJavaScriptCallMachineTransition(current)});
  }else if(['machineInvokeHostCapability','machineAllocateErrorValue','machineInvokeErrorIntrinsic','machineAllocateError','machineAllocateSourceError'].includes(current.$)){
   const request=current;let response;
   const authority={perform(actual){if(response!==undefined)throw new TypeError('source replay cannot repeat an external effect');response=embedding.authority.perform(actual);return response;}};
   current=request.$==='machineInvokeHostCapability'?resumeJavaScriptSourceHostCapability(embedding.module,request,authority):resumeJavaScriptErrorModule(embedding.module,request,authority);
   if(response===undefined)throw new TypeError('source replay requires its actual completed external response');
   append({operation:'external',requestKind:request.$,realm:String(request.realm),callee:request.callee?.identity===undefined?null:String(request.callee.identity),response:{heap:codecs.encodeJavaScriptOrdinaryHeap(response.heap),value:codecs.encodeJavaScriptSemanticValue(response.value),abrupt:response.abrupt},expected:codecs.encodeJavaScriptCallMachineTransition(current)});
  }else break;
  used++;adopt();
 }
 const budgetExhausted=used>=steps&&(current.$==='machineContinue'||current.$==='machineResourceLimit'&&current.reason==='execution exhausted its explicit transition budget');
 if(budgetExhausted&&current.$==='machineContinue')current={$:'machineResourceLimit',machine:current.machine,reason:'source execution exhausted its explicit transition budget'};
 return {initial:first,events,budget:steps,used,budgetExhausted,final:codecs.encodeJavaScriptCallMachineTransition(current),transition:current};
}
