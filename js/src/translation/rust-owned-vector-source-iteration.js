// Actual owned std Vec IntoIterator source transport. One genuine container
// transfer precedes any item binding; the original owner remains moved even
// when traversal has zero items or exits at the first body.
import {testRustStandardCollectionProjectedGraphEquality} from './rust-standard-collection-call-adapter.js';
import {unsupported} from './diagnostics.js';
import {readRustOwnedContainerFacts} from './rust-owned-containers.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustExclusiveSourceLoanVersionError} from './rust-exclusive-source-loan-rules.js';
const views=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust owned vector source iteration',reason,span);};
function elementOrigin(parser,type,owner,path,depth=0){
 if(depth>128||!owner||!Array.isArray(owner.path)||!Array.isArray(owner.arguments))fail('an owned vector item retains its actual bounded complete element owner');
 const origin={...parser.origins.typeOrigin(type,path),sourceOwner:structuredClone(owner),wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceValueBorrowBindings:[],sourceValueFieldBorrowBindings:null,sourceBorrowedPlace:false};
 if(type.kind==='array'&&owner.kind==='library'&&['std::vec::Vec','alloc::vec::Vec'].includes(owner.path.join('::'))&&owner.arguments.length===1){origin.arraySourceModel='owned vector';origin.arrayElementCopyEvidence=['fixed','bool'].includes(type.element.kind);}
 if(type.kind==='string'){origin.textSourceModel='owned string';origin.copyEvidence=false;}
 if(type.kind==='tuple'){
  if(owner.kind!=='tuple'||owner.arguments.length!==type.elements.length)fail('an owned tuple item retains every actual element position');origin.fields=type.elements.map((type,index)=>elementOrigin(parser,type,owner.arguments[index],path,depth+1));
 }
 // Indirection cannot be inferred from an erased core element type. This
 // adapter's first closed domain is direct owned scalars/String/records/sums.
 if(owner.kind==='library'&&['std::boxed::Box','alloc::boxed::Box'].includes(owner.path.join('::')))fail('Box iterator items require their actual un-erased element position and ownership layers');
 return origin;
}
export function createRustOwnedVectorSourceIteration(parser,receiver,path,span=receiver.span){
 const table=parser.origins,actual=table.origin(receiver,path);if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'||actual.type?.kind!=='array')return null;
 const owner=actual.expandedSourceOwner??actual.sourceOwner;
 if(!table.collecting&&!actual.proven||actual.wrappers.length||actual.sourceBorrowedPlace===true||actual.arraySourceModel!=='owned vector'||owner?.kind!=='library'||!['std::vec::Vec','alloc::vec::Vec'].includes(owner.path.join('::'))||owner.arguments.length!==1||actual.arrayElementPosition&&(actual.arrayElementPosition.wrappers.length||actual.arrayElementPosition.arrayElementPosition||!testRustStandardCollectionProjectedGraphEquality(actual.arrayElementPosition.owner,owner.arguments[0]))||readRustCopyClosureValueBorrows(actual).length)fail('owned iteration needs its actual direct standard Vec owner without borrowed element positions or carried loans',span);
 const version=readRustExclusiveSourceLoanVersionError(1,parser.sourceVersionMinor);if(version)fail(version,span);
 checkRustOwnedOrigin(table,actual,span);const item=elementOrigin(parser,readRustRecordSourceType(parser,actual.type.element,actual.path),owner.arguments[0],path),facts=readRustOwnedContainerFacts(table,item),ownership=readRustStandardSumOwnershipFacts(table,item);
 if(!table.collecting&&(!facts.closed||!['copy','owned'].includes(facts.profile)||!['copy','owned'].includes(ownership.profile)))fail('owned iterator items require their actual complete closed destructor-free ownership graph',span);
 // Existing source transfer validates real binding availability or the actual
 // constructor/call producer; a declared type alone cannot create an owner.
 const transferred=table.transferOwnedResult(receiver,path);
 if(!table.collecting&&(!table.availableOwnedBinding(transferred)||transferred.sourceOwnedResultProof!==true))fail('IntoIterator consumes one actual available transferred Vec container',span);
 if(!table.collecting&&!table.consumeOwnedBinding(transferred,span))fail('IntoIterator consumes its real registered ownership root',span);
 item.copyEvidence=ownership.profile==='copy';item.sourceOwnedResultProof=ownership.profile==='owned';
 const origin={...structuredClone(transferred),arraySourceModel:null,sourceOwnedResultProof:false,sourceOwnedVectorIterator:true,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceOwner:{kind:'library',path:['std','vec','IntoIter'],arguments:structuredClone(owner.arguments)}};
 table.mark(receiver,origin);const view={node:receiver,item,actual,origin,loan:null,owned:true,operation:'into_iter',path:[...path],ownerTransfer:transferred.sourceBindingIdentity,headEnd:parser.cursor.index};views.set(receiver,view);return view;
}

// An iterator transport exists only at the active sole for-head. Neither the
// physical array nor an origin flag can authenticate a stored/forged iterator.
export function readRustOwnedVectorSourceIteration(parser,node){
 const view=views.get(node);if(!view)return null;const actual=parser.origins.origin(node,view.path);if(actual.sourceOwnedVectorIterator!==true||actual.arraySourceModel!==null||actual.wrappers.length||actual.sourceOwner?.kind!=='library'||actual.sourceOwner.path.join('::')!=='std::vec::IntoIter'||!testRustStandardCollectionProjectedGraphEquality(actual.type,view.origin.type)||!testRustStandardCollectionProjectedGraphEquality(actual.sourceOwner,view.origin.sourceOwner))return null;
 if(parser.cursor.tokens.slice(view.headEnd,parser.cursor.index).some(token=>token.value!==')'))return null;return view;
}
export function readRustOwnedVectorSourceIterationMethod(parser,receiver,method,path,span=receiver.span){
 if(method.value!=='into_iter')return null;const c=parser.cursor;c.expect('(','owned Vec IntoIterator');c.expect(')','zero-argument owned Vec IntoIterator');const view=createRustOwnedVectorSourceIteration(parser,receiver,path,span);if(!view)fail('into_iter requires its actual owned standard Vec in the active sole for-head',span);return view.node;
}
