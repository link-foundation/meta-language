// Per-value field moves share the existing owner-state snapshots and joins.
// No field availability or producer witness enters datatype TYPE contracts.
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordFieldIdentity,readRustOwnedRecordFieldRoot,readRustOwnedRecordWholeError,readRustOwnedRecordFieldError,readRustOwnedRecordUpdateError,readRustOwnedRecordAbsentFieldState,readRustOwnedRecordUltimateRoot,readRustOwnedRecordUpdateVersionError,testRustOwnedRecordFieldCopy,readRustOwnedRecordFieldMode} from './rust-owned-record-update-rules.js';
import {readRustNominalOwnershipFacts,createRustNominalConstructorOrigin,projectRustNominalPatternField} from './rust-nominal-ownership.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {readRustOwnedRecordPlaceBase} from './rust-owned-record-places.js';
import {readRustOwnedRecordPlaceDepthLimit} from './rust-owned-record-place-rules.js';
import {readRustOwnedBoxRecordOwner} from './rust-owned-containers.js';
import {readRustBorrowedVectorMutationError} from './rust-borrowed-vector-rules.js';
export function acceptRustOwnedRecordProjectionName(parser,node,path){
 if(node.k!=='name'||node.path.length!==1||!parser.cursor.is('.')||parser.cursor.is('(',2))return false;
 const origin=parser.origins.scope.get(node.path[0]),owner=origin?.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null,record=owner?parser.namedRecords.get(owner.fullName):null;
 return Boolean(record?.names.includes(parser.cursor.peek(1).value));
}
export function checkRustOwnedRecordWhole(table,origin,span){
 if(table.collecting||origin.sourceBindingIdentity==null)return;const boxed=readRustOwnedBoxRecordOwner(table,origin);if(!boxed&&readRustNominalOwnershipFacts(table,{...origin,wrappers:[]}).profile!=='owned')return;const error=readRustOwnedRecordWholeError(origin.sourceBindingIdentity, [...table.sourceBindingStates.keys()],[...table.sourceBindingStates.values()]);if(error)throw unsupported('Rust record field ownership',error,span);
}
export function rustOwnedRecordFieldState(snapshot,identity){if(snapshot.has(identity))return snapshot.get(identity);let root=readRustOwnedRecordFieldRoot(identity);for(let depth=0;depth<readRustOwnedRecordPlaceDepthLimit()&&root!=='';depth+=1){if(snapshot.has(root))return readRustOwnedRecordAbsentFieldState(snapshot.get(root));root=readRustOwnedRecordFieldRoot(root);}return 'absent';}
export function rustOwnedRecordFieldSurvives(table,identity,before,result){const root=readRustOwnedRecordUltimateRoot(identity);return root!==''&&table.sourceBindingStates.get(root)==='available'&&(before.has(root)||result.sourceBindingIdentity===root||(result.sourceExtendingTemporaryBindings??[]).includes(root));}
export function checkRustOwnedRecordRoot(table,origin,span){
 // Field selection checks root lifetime independently of whole-value moves.
 checkRustOwnedOrigin(table,{...origin,sourceBindingIdentity:null},span);
 const root=origin.sourceBindingIdentity??origin.sourcePlaceBindingIdentity;
 if(root!=null){const state=table.sourceBindingStates.get(root)??'',error=readRustOwnedRecordFieldError(state,'available');if(error)throw unsupported('Rust record field ownership',error,span);}
}
export function projectRustOwnedRecordField(table,parent,field,index,borrowed,span){
 checkRustOwnedRecordRoot(table,parent,span);
 const root=parent.sourceBindingIdentity??parent.sourcePlaceBindingIdentity,identity=root==null?'':readRustOwnedRecordFieldIdentity(root,index);
 if(identity&&!table.collecting){const error=readRustOwnedRecordFieldError(table.sourceBindingStates.get(root)??'',table.sourceBindingStates.get(identity)??'available');if(error)throw unsupported('Rust record field ownership',error,span);}
 // A field place differs from match ergonomics: retain only the field's own
 // reference layers. A shared parent forbids moving its owning child.
 const mode=readRustOwnedRecordFieldMode(testRustOwnedRecordFieldCopy(field.proven===true,field.copyEvidence===true,field.wrappers),borrowed||parent.sourceBorrowedPlace===true),loans=[...new Set([...(root==null?[]:[root]),...(parent.sourceLoanBindingIdentities??[]),...(parent.sourcePlaceLoanBindingIdentities??[])])];
 let result=projectRustNominalPatternField(table,parent,field,index,false);
 if(identity&&!table.collecting&&!table.sourceBindingStates.has(identity))table.sourceBindingStates.set(identity,'available');
 result={...result,sourceBindingIdentity:mode==='copied value'?null:identity||null,sourcePlaceBindingIdentity:identity||null,sourceNominalPatternMove:false,sourceOwnedResultProof:mode==='owned place'&&identity===''&&parent.sourceOwnedResultProof===true,sourceBorrowedPlace:mode==='borrowed place',sourceNominalBorrowedField:mode==='borrowed place',sourceLoanBindingIdentities:mode==='copied value'?[]:[...new Set([...loans,...(identity?[identity]:[])])],sourcePlaceLoanBindingIdentities:loans};
 return result;
}
export function prepareRustOwnedRecordEntry(parser,record,name,value,path){
 if(parser.headerDiscovery||parser.origins.collecting||!record)return;
 const index=record.names.indexOf(name),entry=parser.origins.lookupValueOwner(['crate',...record.constructor.split('.')],[]),field=index>=0?entry?.fields?.[index]:null;if(!field)return;
 parser.origins.argument(parser.origins.origin(value,path),field,value.span);parser.origins.prepareOwnedArgument(value,field,path);
}
export function createRustOwnedRecordUpdateOrigin(parser,record,entries,base,positions,node,path,span){
 const table=parser.origins,parent=table.origin(base,path),facts=readRustNominalOwnershipFacts(table,parent);
 if(parser.headerDiscovery||table.collecting||facts.profile!=='owned')return null;
 const version=readRustOwnedRecordUpdateVersionError(parser.sourceVersionMinor);if(version)throw unsupported('Rust owned record update',version,span);
 const entry=table.lookupValueOwner(['crate',...record.constructor.split('.')],[]),owner=parent.type?.kind==='named'?table.lookupTypeOwner(parent.type.path,parent.path):null;
 const error=readRustOwnedRecordUpdateError(facts.profile,entry?.k==='ctor'&&entry.owner===record.owner&&owner?.fullName===record.owner,parent.wrappers.length,entry?.fields.length??-1,positions.length);if(error)throw unsupported('Rust owned record update',error,span);
 checkRustOwnedRecordRoot(table,parent,span);
 if(parent.sourceBindingIdentity==null&&!parent.sourceOwnedResultProof)throw unsupported('Rust owned record update','an owned record base needs its actual available root or fresh constructor producer',span);
 // Explicit expressions have already transferred in source order. Validate
 // omitted fields before committing any selected field move.
 const actuals=positions.map((position,index)=>position<0?projectRustOwnedRecordField(table,parent,entry.fields[index],index,false,span):table.origin(entries[position].value,path));
 const previous=table.scope,scope=new Map(previous),args=[];table.scope=scope;
 try{
  for(let index=0;index<positions.length;index+=1){
   if(positions[index]>=0){args.push(entries[positions[index]].value);continue;}
   const name=parser.freshProductBinding(),value=actuals[index];scope.set(name,value);args.push({k:'name',path:[name],span});
  }
  return createRustNominalConstructorOrigin(parser,node,args,path,span);
 }finally{table.scope=previous;}
}
export function readRustOwnedRecordUpdateBase(parser,path){
 const mutation=readRustBorrowedVectorMutationError(parser.origins.borrowedVectorSourceEnabled===true);if(mutation){const token=parser.cursor.peek();throw unsupported('Rust borrowed vector source',mutation,{start:token.start,end:token.end});}
 const place=readRustOwnedRecordPlaceBase(parser,path);if(place)return place;
 const c=parser.cursor,token=c.peek(),origin=parser.origins.scope.get(token.value);
 // A simple place base is not a whole-value read: only omitted fields move.
 // Arbitrary expressions keep ordinary whole-value validation.
 if(origin&&c.is('}',1)&&readRustNominalOwnershipFacts(parser.origins,origin).profile==='owned'){
  c.next();checkRustOwnedRecordRoot(parser.origins,origin,{start:token.start,end:token.end});return {k:'name',path:[token.value],span:{start:token.start,end:token.end}};
 }
 return parser.expr(path);
}
