// External capability provider seam. The portable machine retains every
// represented body; a source archive or bytecode mode cannot grant authority.
import {readJavaScriptCallMachineError,resumeJavaScriptHostCapability} from './javascript-call-machine.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {testJavaScriptHostCapabilityMode} from './javascript-host-capability-rules.js';

export function readJavaScriptHostCapabilityRequestError(program,request){
 if(request?.$!=='machineInvokeHostCapability')return 'only an actual suspended host capability can be resumed';
 const invalid=readJavaScriptCallMachineError(program,request.machine);if(invalid)return invalid;
 const frame=request.machine.frames.at(-1),owner=program.functions.find(row=>row.identity===frame?.identity);
 if(!frame||frame.pending.$!=='waitHostCapability'||!owner||!testJavaScriptHostCapabilityMode(owner.receiverMode)||owner.realm!==request.realm||request.callee?.$!=='object'||!request.callee.callable||request.callee.identity!==frame.identity)return 'a host operation must retain its actual pending qualified owner and realm';
 if(!testJavaScriptSemanticSameValue(request.receiver,frame.registers[0]??{$:'undefined'})||!Array.isArray(request.arguments)||request.arguments.length!==frame.arguments.length||request.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,frame.arguments[index])))return 'a host operation cannot substitute its actual receiver or ordered arguments';
 return '';
}

export function resumeJavaScriptSourceHostCapability(module,request,authority){
 const invalid=readJavaScriptHostCapabilityRequestError(module.program,request);if(invalid)throw new TypeError(invalid);
 if(!authority||typeof authority.perform!=='function')throw new TypeError('actual qualified host capability authority is required');
 // The provider additionally checks callee identity against independently
 // supplied operation/source/version owners, before any IO/decoder effect.
 const result=authority.perform(request);
 if(!result||!result.heap||!result.value||typeof result.abrupt!=='boolean')throw new TypeError('a capability provider retains its actual value/throw and complete shared heap');
 return resumeJavaScriptHostCapability(module.program,{...request.machine,heap:result.heap},request.realm,request.callee.identity,result.value,result.abrupt);
}
