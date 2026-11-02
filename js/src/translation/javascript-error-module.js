// Executable source consumer. The portable machine owns every represented
// constructor/helper body. The embedding owns real realm intrinsics and live
// Error handles; neither side reconstructs Error.stack from ordinary fields.
import {compileJavaScriptErrorClassModule,compileJavaScriptInitializedErrorClassModule} from './javascript-error-classes.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine,readJavaScriptCallMachineError,stepJavaScriptCallMachine,resumeJavaScriptErrorValueAllocation,resumeJavaScriptErrorAllocationAbrupt,resumeJavaScriptErrorIntrinsic,resumeJavaScriptCallErrorAllocation,resumeJavaScriptCallSourceErrorAllocation} from './javascript-call-machine.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';

export function prepareJavaScriptErrorModule(capture,registry){
 const compiled=compileJavaScriptErrorClassModule(capture,registry);
 return {capture:compiled.capture,registry,program:compiled.program};
}
export function prepareJavaScriptInitializedErrorModule(capture,registry,state){const compiled=compileJavaScriptInitializedErrorClassModule(capture,registry,state);return {capture:compiled.capture,registry,program:compiled.program};}

/** An explicit entry operation distinguishes `new C(...)` from `f(...)`.
 * Supplying a constructor as an ordinary call remains outside this source
 * profile; it must not silently run as a successful derived construction. */
export function beginJavaScriptErrorModule(module,heap,sourceIdentity,arguments_,{construct=false}={}){
 const definition=module.capture.plan.functions.find(row=>row.identity===sourceIdentity);
 const owner=module.registry.owners.find(row=>row.source===sourceIdentity);
 if(!definition||!owner||!module.capture.plan.exports.includes(sourceIdentity))throw new TypeError('an Error module entry needs its actual captured exported declaration');
 if(construct!==(definition.$==='errorConstructorPlan'))throw new TypeError('the entry operation must retain the actual source call/new distinction');
 return beginJavaScriptCallMachine(module.program,heap,owner.identity,{$:'undefined'},arguments_);
}

/** Each call performs bounded portable work, stopping at the genuine host
 * boundary. A result is a transition, never an observation/evidence record. */
export function advanceJavaScriptErrorModule(module,transition,{steps=65536,objectIdentities=[]}={}){
 if(transition.$!=='machineContinue')return transition;
 return runJavaScriptCallMachine(module.program,transition.machine,steps,objectIdentities).transition;
}

/** Check the observed request against the actual suspended frame BEFORE any
 * engine effect. A transport may not substitute arguments/callee/realm while
 * retaining a separately valid machine snapshot. */
export function readJavaScriptErrorModuleRequestError(module,request){
 if(!['machineAllocateErrorValue','machineInvokeErrorIntrinsic','machineAllocateError','machineAllocateSourceError'].includes(request.$))return 'only an actual suspended Error host operation can be resumed';
 const invalid=readJavaScriptCallMachineError(module.program,request.machine);if(invalid)return invalid;
 const frame=request.machine.frames.at(-1),pending=frame?.pending;if(!frame||!pending)return 'an Error host request needs its actual suspended frame';
 if(request.$==='machineInvokeErrorIntrinsic'){
  const fn=module.program.functions.find(row=>row.identity===frame.identity);
  if(pending.$!=='waitErrorIntrinsic'||request.callee.$!=='object'||!request.callee.callable||request.callee.identity!==frame.identity||fn?.realm!==request.realm)return 'the host invocation must retain its actual pending callable and realm';
  if(!testJavaScriptSemanticSameValue(request.receiver,frame.registers[0]??{$:'undefined'})||request.arguments.length!==frame.arguments.length||request.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index]??{$:'undefined'})))return 'the host invocation must retain its actual receiver and ordered arguments';
  return '';
 }
 if(request.$==='machineAllocateErrorValue'){
  if(pending.$!=='waitErrorValue'||pending.realm!==request.realm||pending.errorClass!==request.errorClass||pending.messagePresent!==request.messagePresent)return 'the Error allocator must retain its actual pending kind, realm and presence';
  for(const key of ['constructor','newTarget','prototype','message'])if(!testJavaScriptSemanticSameValue(request[key],pending[key]))return 'the Error allocator cannot substitute actual pending operands';
  return '';
 }
 if(request.$==='machineAllocateSourceError')return pending.$==='waitSourceError'&&pending.realm===request.realm&&pending.errorClass===request.errorClass&&pending.messagePresent===request.messagePresent&&testJavaScriptSemanticSameValue(pending.message,request.message)?'':'source Error allocation must retain its actual pending operands';
 if(pending.$==='waitObjectFromEntries'){
  const actual=stepJavaScriptCallMachine(module.program,request.machine);return actual.$==='machineAllocateError'&&actual.realm===request.realm&&actual.errorClass===request.errorClass&&actual.reason===request.reason?'':'Object.fromEntries Error allocation retains its actual pending cleanup or selected-call failure';
 }
 return pending.$==='waitError'&&pending.realm===request.realm&&pending.errorClass===request.errorClass&&pending.reason===request.reason?'':'Error allocation must retain its actual pending reason and realm';
}

/** `authority` is supplied by the engine embedding, independently of the
 * captured source and archived heap. It must validate requested constructor,
 * newTarget, prototype and realm against its actual intrinsic registry. It
 * must retain live Error identities and lazy property access, dispatch owned
 * functions back to the machine, and return the latest shared heap. No source
 * body, stack formatter or getter is evaluated as a fallback here.
 *
 * The strict callback result shape prevents an unavailable host cell from
 * becoming an undefined successful semantic result. Exceptions from a host
 * operation belong in its `abrupt` result, not in a fabricated Error object. */
export function resumeJavaScriptErrorModule(module,request,authority){
 if(!authority||typeof authority.perform!=='function')throw new TypeError('actual Error realm authority is required');
 const invalid=readJavaScriptErrorModuleRequestError(module,request);if(invalid)throw new TypeError(invalid);
 const result=authority.perform(request);
 if(!result||!result.heap||!result.value||typeof result.abrupt!=='boolean')throw new TypeError('the host must retain every actual result/throw and the latest complete semantic heap');
 const machine={...request.machine,heap:result.heap};
 if(request.$==='machineInvokeErrorIntrinsic'){
  if(request.callee.$!=='object'||!request.callee.callable)throw new TypeError('the suspended intrinsic must retain its actual callable identity');
  return resumeJavaScriptErrorIntrinsic(module.program,machine,request.realm,request.callee.identity,result.value,result.abrupt);
 }
 if(result.abrupt)return resumeJavaScriptErrorAllocationAbrupt(module.program,machine,request.realm,result.value);
 if(request.$==='machineAllocateErrorValue')return resumeJavaScriptErrorValueAllocation(module.program,machine,request.realm,result.value);
 if(request.$==='machineAllocateSourceError')return resumeJavaScriptCallSourceErrorAllocation(module.program,machine,request.realm,result.value);
 return resumeJavaScriptCallErrorAllocation(module.program,machine,request.realm,result.value);
}
