// Shared nominal Clone is a real trait operation over an authenticated source
// declaration. It is not ownership obtained by removing a reference marker.
import {unsupported} from './diagnostics.js';
import {readRustNominalOwnershipFacts,readRustSourceCloneImplementation} from './rust-nominal-ownership.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {checkRustOwnedRecordWhole} from './rust-owned-record-updates.js';
export function readRustFnMutSourceNominalClone(parser,receiver,method,path){
 if(parser.fnMutSourceModel!=='Rust closed source FnMut specializations 1'||method.value!=='clone')return null;
 const table=parser.origins,actual=table.origin(receiver,path);if(actual.type?.kind!=='named'||!(actual.wrappers.length===1&&actual.wrappers[0]==='reference'||actual.wrappers.length===0&&actual.sourceBorrowedPlace===true))return null;
 const base={...actual,wrappers:[],sourceLifetimes:[],sourceBorrowedPlace:false},facts=readRustNominalOwnershipFacts(table,base);if(!facts.profile)return null;
 const selected=parser.inherentMethods.get(facts.owner+'.clone');if(selected?.mode&&selected.owner===facts.owner)return null;
 if(!readRustSourceCloneImplementation(table,base))throw unsupported('Rust source FnMut nominal Clone','the actual borrowed declaration needs its complete Clone implementation',receiver.span);
 checkRustOwnedOrigin(table,actual,receiver.span);checkRustOwnedRecordWhole(table,base,receiver.span);
 const c=parser.cursor;c.expect('(','actual nominal Clone');const close=c.expect(')','actual nominal Clone'),span={start:receiver.span?.start??method.start,end:close.end},held=parser.freshProductBinding(),body={k:'let',name:held,type:base.type,value:receiver,body:{k:'name',path:[held],span},span};
 return table.mark(body,{...structuredClone(base),sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceValueBorrowBindings:[],sourceValueFieldBorrowBindings:null,sourceNominalPatternMove:false,sourceNominalBorrowedField:false,sourceStandardSumBorrowedField:false,sourceOwnedResultProof:facts.profile==='owned',copyEvidence:facts.profile==='copy'});
}
