// Project actual checked operands into the shared full-copy value selection.
import {acceptRustScalarVectorCopyValue} from './rust-scalar-vector-copy-rules.js';
export function readRustScalarVectorCopyValue(language,node){
 if(node?.k!=='method')return null;
 const object=node.object,start=node.args?.[0],end=node.args?.[1];
 const accepted=acceptRustScalarVectorCopyValue(language,node.k,node.op,object?.type?.kind??'',object?.type?.element?.kind??'',object?.type?.reference===true,node.args?.length??0,start?.k==='lit'?start.type.kind:'',start?.k==='lit'?start.value:'',end?.k==='lit'?end.type.kind:'',end?.k==='lit'?end.value:'');
 return accepted?object:null;
}
