// Ordered String value construction over the actual mutable source place.
// Concatenation and empty text already have shared checked core semantics.
import {unsupported,typeError} from './diagnostics.js';
import {STRING} from './types.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
import {projectRustOwnedRecordStatePlace} from './rust-owned-record-state.js';
import {readRustExclusiveSourceStringMethodArity,acceptRustExclusiveSourceStringReceiver,acceptRustExclusiveSourceStringSuffix} from './rust-exclusive-source-string-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive String source mutation',reason,span);};
export function readRustExclusiveSourceStringMethod(parser,origin,method,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'||origin.type?.kind!=='string')return null;
 const arity=readRustExclusiveSourceStringMethodArity(method.value);if(arity<0)return null;
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 if(!parser.origins.collecting&&!acceptRustExclusiveSourceStringReceiver(origin.type.kind,origin.textSourceModel??'',origin.wrappers.length,origin.proven===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1))fail('the actual receiver must be the admitted direct standard owned String place',method);
 const parameter={...parser.origins.typeOrigin(STRING,path),proven:true,copyEvidence:true,wrappers:['reference'],sourceLifetimes:[null],textSourceModel:'string slice',sourceOwner:{kind:'primitive',path:['str'],arguments:[]}};
 return {method:method.value,parameters:arity===1?[parameter]:[]};
}
export function createRustExclusiveSourceStringMutation(parser,place,selected,args,path,span){
 if(args.length!==selected.parameters.length)throw typeError('String mutation requires its actual complete argument list',span);
 if(selected.method==='push_str'){
  const suffix=parser.origins.origin(args[0],path),owner=suffix.expandedSourceOwner??suffix.sourceOwner??readRustSourceTypeOwner(parser,[suffix.textSourceModel==='string slice'?'str':'String'],[],STRING);
  if(!parser.origins.collecting&&!acceptRustExclusiveSourceStringSuffix(suffix.type?.kind??'',suffix.textSourceModel??'',suffix.wrappers.length,suffix.proven===true,suffix.wrappers.every(layer=>layer==='reference'),owner.kind,owner.path.join('::'),owner.arguments.length))fail('push_str needs one actual standard str/String shared reference with its real live source origin',span);
  checkRustOwnedOrigin(parser.origins,suffix,span);
 }
 const result={...place.field,wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceBorrowedPlace:false,sourceExclusiveParameter:false,sourceOwnedResultProof:true,copyEvidence:false,textSourceModel:'owned string'};
 if(selected.method==='clear')return parser.origins.markOwnedStringConstructor({k:'str',value:'',span},result);
 const transport=parser.ownedRecordState?.effectTransport;if(typeof transport?.read!=='function')fail('String mutation requires its actual ordered caller state continuation',span);
 const root=transport.read(place.slot,{k:'name',path:[place.slot.name],span}),left=parser.origins.mark(projectRustOwnedRecordStatePlace(parser,place.layouts,root,span),place.field);
 return parser.origins.markOwnedStringConstructor({k:'binary',op:'concat',left,right:args[0],span},result);
}
