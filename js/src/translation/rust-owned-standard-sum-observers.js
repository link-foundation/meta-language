// Nonbinding constructor inspection implements the actual &self observers.
// In particular, introducing a receiver let here would move an owned root.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin,trackRustOwnedBindings} from './rust-owned-bindings.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustOwnedStandardSumObserverArity,acceptRustOwnedStandardSumObserverProfile,readRustOwnedStandardSumObserverSourceError,readRustOwnedStandardSumObserverBranch,readRustOwnedStandardSumObserverVersionError} from './rust-owned-standard-sum-observer-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust owned standard sum observer',reason,span);};
export function readRustOwnedStandardSumObserver(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const actual=parser.origins.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);if(!source)return null;
 const sum=source.standardSumSource;if(readRustOwnedStandardSumObserverArity(sum.owner,method.value)<0)return null;
 const facts=readRustStandardSumOwnershipFacts(parser.origins,{...actual,wrappers:[]});if(!acceptRustOwnedStandardSumObserverProfile(facts.profile,actual.wrappers.length))return null;
 const version=readRustOwnedStandardSumObserverVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 checkRustOwnedOrigin(parser.origins,actual,receiver.span);
 const error=readRustOwnedStandardSumObserverSourceError(actual.proven===true,actual.wrappers.length,actual.wrappers.every(layer=>layer==='reference'),actual.sourceLifetimes?.length??0,trackRustOwnedBindings(parser.origins),parser.origins.availableOwnedBinding(actual),actual.sourceOwnedResultProof===true,actual.sourceBorrowedPlace===true,actual.sourceBindingIdentity!=null||(actual.sourceLoanBindingIdentities??[]).length>0);
 if(error)fail(error,receiver.span);
 const c=parser.cursor;c.expect('(','owned standard sum observer');const close=c.expect(')','owned standard sum observer'),span={start:receiver.span?.start??method.start,end:close.end};
 const rows=readRustStandardSumSourceConstructors(sum.owner).map(variant=>{
  const branch=readRustOwnedStandardSumObserverBranch(sum.owner,variant,method.value);if(branch!=='true'&&branch!=='false')fail('the actual constructor has no observer branch',span);
  const count=readRustStandardSumSourceConstructorArity(sum.owner,variant);
  return {patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:count===1?[{k:'wild',span}]:[],span}],body:{k:'bool',value:branch==='true',span},span};
 });
 const result={k:'match',scrutinees:[receiver],rows,span};
 return parser.origins.mark(result,{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}});
}
