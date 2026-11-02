// Closed owned/state ABI views used beside the independent signature3 region
// interpreter. Shape/type normalization remains owned by the real checker.
import {sameType} from './types.js';
import {unsupported} from './diagnostics.js';
const fail=reason=>{throw unsupported('Rust closed source region call',reason);};
export function readRustBorrowedSourceClosedCallInterface(entry){
 if(entry?.k!=='fn')return null;if(!Array.isArray(entry.params)||!entry.ret)fail('a closed call needs its actual complete checked declaration');let parameters,result,modes,indices,callback=null,packet=false;
 const facet=entry.sourceFnMutSpecialization,exclusive=entry.sourceExclusiveSignature,signature=entry.sourceSignature;
 if(facet){if(!Array.isArray(facet.parameters)||!Array.isArray(facet.packet)||!facet.callback)fail('the FnMut call retains its normalized checked facet');parameters=facet.parameters.map(row=>row.position);modes=facet.parameters.map(row=>row.mode);indices=facet.packet;result=facet.result;callback=facet.callback;packet=true;}
 else if(exclusive){if(!Array.isArray(exclusive.parameters)||!Array.isArray(exclusive.packet))fail('the exclusive call retains its normalized checked facet');parameters=exclusive.parameters.map(row=>row.position);modes=exclusive.parameters.map(row=>row.mode);indices=exclusive.packet;result=exclusive.result;packet=indices.length>0;}
 else if(signature?.model==='Rust source function signature 2'){if(!Array.isArray(signature.parameters)||signature.parameters.some(position=>!position||!Array.isArray(position.wrappers)))fail('the owned call retains every normalized actual source parameter');parameters=signature.parameters;modes=parameters.map(position=>position.wrappers[0]==='reference'?'shared':'value');indices=[];result=signature.result;}
 else return null;
 if(parameters.length!==entry.params.length||parameters.length>512||modes.length!==parameters.length||parameters.some((position,index)=>!position?.type||!sameType(position.type,entry.params[index].type))||!Array.isArray(indices)||indices.some((index,at)=>!Number.isSafeInteger(index)||index<0||index>=parameters.length||modes[index]!=='exclusive'||at&&indices[at-1]>=index))fail('the closed call retains its complete actual checked formal and packet order');
 const outputs=[result,...indices.map(index=>parameters[index])],returns=packet&&entry.ret.kind==='tuple'?entry.ret.elements:packet?[]:[entry.ret];if(!result||packet&&entry.ret.kind!=='tuple'||returns.length!==outputs.length||outputs.some((position,index)=>!sameType(position.type,returns[index])))fail('the closed result/state ABI retains every actual checked output slot');
 return {parameters,modes,result,indices,callback,packet,outputs};
}
