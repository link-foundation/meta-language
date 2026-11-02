// Actual type/owner/loan transport into shared owned standard sum decisions.
// String/Vec leaves retain the existing ordinary ownership model. No Copy
// evidence is invented, and runtime allocation identities never define a type.
import {unsupported} from './diagnostics.js';
import {readRustNominalOwnershipFacts} from './rust-nominal-ownership.js';
import {readRustNominalConstructorError} from './rust-nominal-ownership-rules.js';
import {readRustOwnedContainerFacts,markRustOwnedBoxCallResult} from './rust-owned-containers.js';
import {markRustOwnedVectorCallResult} from './rust-owned-vectors.js';
import {acceptRustBorrowedVectorOwnedInterpretation} from './rust-borrowed-vector-rules.js';
import {acceptRustOwnedBoxTypeNode,acceptRustOwnedStandardSumBoxPayload} from './rust-owned-recursive-datatype-rules.js';
import {readRustCopyClosureValueBorrows,projectRustCopyClosureValueField} from './rust-copy-closure-lifetimes.js';
import {acceptRustOwnedBindingTracking,acceptRustOwnedBindingValue,projectRustOwnedLoanBindings,mergeRustOwnedLoanBindings,readRustOwnedLoanReadError} from './rust-owned-binding-rules.js';
import {readRustStandardSumOwnershipProfile,readRustStandardSumOwnedConstructorError,readRustStandardSumOwnedFieldProductionError,readRustStandardSumOwnedFieldDisposition,acceptRustStandardSumOwnedCallResult,readRustStandardSumOwnedPatternTransferError,readRustStandardSumOwnershipNodeLimit,readRustStandardSumOwnershipDepthLimit} from './rust-standard-sum-ownership-rules.js';
const empty=()=>({profile:'',owner:'',roles:[],definitionName:''});
const clone=value=>structuredClone(value);

export function readRustStandardSumOwnershipFacts(table,origin,active=new Set(),depth=0,context={visits:0,memo:new Map()}){
 if(table.standardCollectionSourceModel==='Rust closed standard collections 1'&&typeof table.standardCollectionSourceStorage==='function'&&table.standardCollectionSourceStorage(origin)===true)return {profile:'owned',owner:origin.sourceOwner.path.join('::'),roles:[],definitionName:origin.type.path.filter(part=>part!=='crate').join('.')};
 if(!acceptRustBorrowedVectorOwnedInterpretation(Boolean(origin?.arrayElementPosition)))return empty();
 if(++context.visits>readRustStandardSumOwnershipNodeLimit()||!origin?.proven||!origin.type||depth>readRustStandardSumOwnershipDepthLimit())return empty();
 if(origin.wrappers.length){const owner=origin.expandedSourceOwner??origin.sourceOwner;let child=owner,boxes=0;while(child&&boxes<256&&acceptRustOwnedBoxTypeNode(['box'],child.kind,child.path.join('::'),child.arguments.length)){boxes+=1;child=child.arguments[0];}if(!acceptRustOwnedStandardSumBoxPayload(origin.wrappers,origin.sourceLifetimes?.length??0,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,boxes))return empty();const facts=readRustOwnedContainerFacts(table,origin);return facts.node==='box'&&facts.profile==='owned'&&facts.closed?{profile:'owned',owner:owner.path.join('::'),roles:[],definitionName:''}:empty();}
 const declared=origin.type.kind==='named'?table.lookupTypeOwner(origin.type.path,origin.path):null;
 const stored=declared?.k==='data'?declared.standardSumSourceOrigin:null,sum=stored?.standardSumSource;
 if(sum){
  if(!stored.proven||sum.provisional===true||stored.wrappers.length)return empty();
  if(context.memo.has(declared.fullName))return context.memo.get(declared.fullName);
  if(active.has(declared.fullName))return empty();active.add(declared.fullName);
  const roles=sum.arguments.map(argument=>readRustStandardSumOwnershipFacts(table,argument,active,depth+1,context).profile);active.delete(declared.fullName);
  const result={profile:readRustStandardSumOwnershipProfile(sum.owner,roles),owner:sum.owner,roles,definitionName:declared.fullName};context.memo.set(declared.fullName,result);return result;
 }
 if(acceptRustOwnedBindingValue(origin.type.kind,origin.arraySourceModel??origin.textSourceModel??'',origin.wrappers.length,origin.proven===true))return {profile:'owned',owner:(origin.expandedSourceOwner??origin.sourceOwner)?.path.join('::')??'',roles:[],definitionName:''};
 if(declared?.k==='data'){const nominal=readRustNominalOwnershipFacts(table,origin);return {profile:table.nominalCopyOwners.has(declared.fullName)?'copy':nominal.profile,owner:'',roles:[],definitionName:declared.fullName};}
 if(origin.type.kind==='tuple'&&origin.fields){const roles=origin.fields.map(field=>readRustStandardSumOwnershipFacts(table,field,active,depth+1,context).profile);return {profile:roles.every(role=>role==='copy')&&table.immutableProduct(origin)?'copy':'',owner:'',roles,definitionName:''};}
 return {profile:table.immutableProduct(origin)?'copy':'',owner:'',roles:[],definitionName:''};
}

export function isRustOwnedStandardSumValue(table,origin){
 if(origin?.type?.kind!=='named')return false;
 const facts=readRustStandardSumOwnershipFacts(table,origin);return facts.profile==='owned'&&facts.definitionName!==''&&readRustStandardSumOwnershipProfile(facts.owner,facts.roles)==='owned';
}

export function createRustStandardSumConstructorOrigin(parser,node,origin,args,path,span=node.span){
 const table=parser.origins;if(parser.headerDiscovery)return clone(origin);
 const facts=readRustStandardSumOwnershipFacts(table,origin);
 if(!['copy','owned'].includes(facts.profile)||!facts.definitionName)throw unsupported('Rust owned standard sum','the constructor needs its actual admitted standard datatype',span);
 const names=node.k==='app'&&node.fn.k==='name'?node.fn.path:node.k==='name'?node.path:[],entry=table.lookupValueOwner(names,path);
 const nominal=readRustNominalOwnershipFacts(table,origin),actual=entry?.k==='ctor'&&entry.owner===facts.definitionName;
 const error=nominal.profile&&nominal.owner===facts.definitionName?readRustNominalConstructorError(actual,entry?.fields?.length??-1,args.length):readRustStandardSumOwnedConstructorError(actual,entry?.fields?.length??-1,args.length);if(error)throw unsupported('Rust owned standard sum',error,span);
 const values=args.map(argument=>table.origin(argument,path));
 for(let at=0;at<args.length;at+=1){
  const argument=args[at],value=values[at],owned=readRustStandardSumOwnershipFacts(table,entry.fields[at]).profile==='owned',transferred=table.ownedArgumentTransfers.has(argument);
  table.argument(value,entry.fields[at],argument.span??span,transferred);
  const called=argument.k==='app'&&argument.fn.k==='name'?table.lookupValueOwner(argument.fn.path,path):null,recorded=Boolean(argument.span&&table.freshCalls.has(`${argument.span.start}:${argument.span.end}`));
  const callProducer=called?.k==='fn'&&readRustStandardSumOwnershipFacts(table,called.result).profile==='owned'&&recorded&&table.same(value,called.result);
  const producer=value.sourceOwnedResultProof===true||value.freshArrayValue===true||table.freshVectorConstructors.has(`${argument.span?.start}:${argument.span?.end}`)||callProducer;
  const error=table.collecting?'':readRustStandardSumOwnedFieldProductionError(owned,transferred,table.availableOwnedBinding(value),producer);if(error)throw unsupported('Rust owned standard sum',error,argument.span??span);
  if(!transferred)table.prepareOwnedArgument(argument,entry.fields[at],path);
 }
 const fieldBorrows=values.map(readRustCopyClosureValueBorrows),result={...clone(origin),sourceLexicalBindingIdentity:null,sourceValueBorrowBindings:fieldBorrows.reduce((all,values)=>mergeRustOwnedLoanBindings(all,values),[]),sourceValueFieldBorrowBindings:fieldBorrows,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceOwnedResultProof:facts.profile==='owned'};
 return result;
}
export function captureRustStandardSumConstructor(parser,node,origin,args,path,span=node.span){
 parser.origins.mark(node,createRustStandardSumConstructorOrigin(parser,node,origin,args,path,span));return node;
}

export function projectRustStandardSumPatternField(table,parent,field,borrowed){
 // A match scrutinee is not an extending initializer expression. Keep its
 // actual loans, but never turn its temporary candidates into field grants.
 // An outer syntactic borrow initializer has already moved its own drop scope.
 const result=projectRustCopyClosureValueField(parent,{...clone(field),sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceOwnedResultProof:false,sourceStandardSumPatternMove:false,sourceStandardSumBorrowedField:false},0);
 const parentBase={...parent,wrappers:[]},facts=readRustStandardSumOwnershipFacts(table,parentBase),ownedParent=facts.profile==='owned'&&facts.definitionName!=='';
 const lineage=parent.sourceBindingIdentity!=null||parent.sourcePlaceBindingIdentity!=null||(parent.sourceLoanBindingIdentities??[]).length>0||(parent.sourcePlaceLoanBindingIdentities??[]).length>0;
 const role=readRustStandardSumOwnedFieldDisposition(ownedParent||borrowed&&lineage,borrowed,readRustStandardSumOwnershipFacts(table,field).profile==='owned');
 if(borrowed){result.wrappers.unshift('reference');result.sourceLifetimes=[parent.sourceLifetimes?.[0]??null,...(result.sourceLifetimes??[])];result.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(result),readRustCopyClosureValueBorrows(parent));}
 if(role==='borrow'){
  result.sourceStandardSumBorrowedField=true;result.sourceNominalBorrowedField=parent.sourceNominalBorrowedField===true;
  result.sourceBindingIdentity=parent.sourceBindingIdentity??parent.sourcePlaceBindingIdentity??null;result.sourcePlaceBindingIdentity=parent.sourcePlaceBindingIdentity??parent.sourceBindingIdentity??null;
  result.sourceLoanBindingIdentities=mergeRustOwnedLoanBindings(projectRustOwnedLoanBindings(parent.sourceBindingIdentity??'',parent.sourceLoanBindingIdentities??[]),projectRustOwnedLoanBindings(parent.sourcePlaceBindingIdentity??'',parent.sourcePlaceLoanBindingIdentities??[]));result.sourceBorrowedPlace=true;
 }else if(role==='move'){
  result.sourceBindingIdentity=parent.sourceBindingIdentity??null;result.sourceBorrowedPlace=parent.sourceBorrowedPlace===true;result.sourceStandardSumPatternMove=true;result.sourceNominalPatternMove=parent.sourceNominalPatternMove===true;result.sourceOwnedResultProof=parent.sourceOwnedResultProof===true;
 }else result.sourceBorrowedPlace=parent.sourceBorrowedPlace===true;
 return result;
}

export function requireRustStandardSumPatternTransfer(table,origin,span){
 const error=readRustStandardSumOwnedPatternTransferError(origin.sourceStandardSumPatternMove===true||isRustOwnedStandardSumValue(table,origin),origin.sourceBindingIdentity!=null,origin.sourceOwnedResultProof===true);if(error)throw unsupported('Rust owned standard sum',error,span);
}

export function validateRustStandardSumBorrowedResult(table,origin,span){
 if(origin.sourceStandardSumBorrowedField!==true&&origin.sourceNominalBorrowedField!==true||!acceptRustOwnedBindingTracking(table.collecting,table.ownedBindingExcluded===true||Boolean(table.localValueStateModel)||Boolean(table.sharedArrayHandleSourceModel)||Boolean(table.arrayResultModel)))return;
 const identities=projectRustOwnedLoanBindings(origin.sourceBindingIdentity??'',origin.sourceLoanBindingIdentities??[]),error=readRustOwnedLoanReadError(identities.map(identity=>table.sourceBindingStates.get(identity)??''));if(error)throw unsupported('Rust owned binding',error,span);
}

export function finishRustStandardSumFieldDereference(table,origin,span){
 if(origin.sourceStandardSumBorrowedField!==true&&origin.sourceNominalBorrowedField!==true)return origin;
 validateRustStandardSumBorrowedResult(table,origin,span);
 if(origin.wrappers.length||readRustStandardSumOwnershipFacts(table,origin).profile!=='copy')return origin;
 return {...origin,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceStandardSumBorrowedField:false,sourceStandardSumPatternMove:false,sourceNominalBorrowedField:false,sourceNominalPatternMove:false,sourceOwnedResultProof:false};
}

export function markRustStandardSumOwnedCallResult(table,node,path,origin){
 const facts=readRustStandardSumOwnershipFacts(table,origin),owned=facts.profile==='owned'&&facts.definitionName!=='',names=node.k==='app'&&node.fn.k==='name'?node.fn.path:node.k==='name'?node.path:[];
 const actual=table.lookupValueOwner(names,path),output=actual?.k==='fn'?readRustStandardSumOwnershipFacts(table,actual.result):empty(),recorded=Boolean(node.span&&table.freshCalls.has(`${node.span.start}:${node.span.end}`));
 if(!acceptRustStandardSumOwnedCallResult(owned,actual?.k==='fn'&&output.profile==='owned'&&output.definitionName===facts.definitionName,recorded))return markRustOwnedVectorCallResult(table,node,path,markRustOwnedBoxCallResult(table,node,path,origin));
 return {...origin,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceOwnedResultProof:true};
}
