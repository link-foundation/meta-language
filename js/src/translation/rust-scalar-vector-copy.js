// Host syntax adaptation only. The existing full-slice operation retains one
// receiver evaluation and a fresh owned value in the target representation.
import {acceptRustScalarVectorCopyQuery,acceptRustScalarVectorCopyReceiver,acceptRustScalarVectorCopyOwner,readRustScalarVectorCopyOperation} from './rust-scalar-vector-copy-rules.js';
export function readRustScalarVectorCopy(parser,receiver,method,args,range,path){
 const origin=parser.origins.origin(receiver,path),type=origin.type,model=origin.arraySourceModel??'';
 if(!acceptRustScalarVectorCopyQuery(parser.ordinaryCompletionContext(),method.value,model,origin.wrappers.length))return null;
 if(!acceptRustScalarVectorCopyReceiver(type?.kind??'',type?.kind==='array'?type.element.kind:'',origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),origin.arrayElementCopyEvidence===true,args.length))return null;
 return createRustScalarVectorCopy(parser,receiver,origin,range,path);
}
/** Project an already admitted source-copy occurrence into the shared core. */
export function createRustScalarVectorCopy(parser,receiver,origin,range,path){
 const type=origin.type;
 const owner=origin.sourceOwner,known=owner&&acceptRustScalarVectorCopyOwner(owner.kind,owner.path.join('::'),owner.path.length,owner.arguments.length);
 const copied={...origin,type:structuredClone(type),path:[...path],wrappers:[],fields:null,textSourceModel:null,arraySourceModel:'owned vector',sourceLifetimes:[],sourceOwner:known?{kind:'library',path:['std','vec','Vec'],arguments:[structuredClone(owner.arguments[0])]}:null,expandedSourceOwner:null,freshArrayValue:true,freshVectorResultContract:false};
 return parser.origins.markVectorConstructor({k:'method',object:receiver,op:readRustScalarVectorCopyOperation(),args:[],span:range},copied);
}
