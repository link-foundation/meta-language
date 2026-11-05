// Real consuming state transitions; shared predicate policy chooses all rows.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {captureRustOwnedStandardSumMethodValue,createRustOwnedStandardSumMethodResult} from './rust-owned-standard-sum-methods.js';
import {readRustOwnedStandardSumCallbackValue} from './rust-owned-standard-sum-callbacks.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumPredicateArity,readRustStandardSumPredicateCaptureRoles,readRustStandardSumPredicateCallbackArgument,readRustStandardSumPredicateBranch,readRustStandardSumPredicateVersionError} from './rust-standard-sum-predicate-rules.js';
import {readRustOwnedStandardSumPredicateResultError} from './rust-owned-standard-sum-predicate-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust owned standard sum predicate',reason,span);};
export function readRustOwnedStandardSumPredicate(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const table=parser.origins,actual=table.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);if(!source)return null;
 const sum=source.standardSumSource,arity=readRustStandardSumPredicateArity(sum.owner,method.value);if(arity<0)return null;
 if(readRustStandardSumOwnershipFacts(table,{...actual,wrappers:[]}).profile!=='owned')return null;
 const version=readRustStandardSumPredicateVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 const c=parser.cursor,start=receiver.span?.start??method.start;c.expect('(','owned standard sum predicate');
 const movedReceiver=captureRustOwnedStandardSumMethodValue(parser,receiver,path),captures=[{role:'receiver',name:parser.freshProductBinding(),value:receiver,origin:movedReceiver}],boolean={...table.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}},roles=readRustStandardSumPredicateCaptureRoles(method.value);let resultType=arity===1?boolean:null,ownedDefault=null;
 for(let at=1;at<roles.length;at+=1){
  if(at>1)c.expect(',','owned standard sum predicate arguments');const role=roles[at];
  if(role==='default'){
   const value=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));let origin=table.origin(value,path);
   const profile=readRustStandardSumOwnershipFacts(table,origin).profile,error=readRustOwnedStandardSumPredicateResultError(profile,Boolean(readRustStandardSumSourceIdentity(parser,origin)));if(error)fail(error,value.span);
   if(profile==='owned'){origin=captureRustOwnedStandardSumMethodValue(parser,value,path);ownedDefault=origin;}
   resultType=origin;captures.push({role,name:parser.freshProductBinding(),value,origin});
  }else{
   const field=readRustStandardSumPredicateCallbackArgument(sum.owner,method.value,role);if(field===-2)fail('the source callback role is absent',method);
   const checked=readRustOwnedStandardSumCallbackValue(parser,path,field<0?[]:[sum.arguments[field]],role==='predicate'?boolean:role==='mapper'?resultType:null);
   if(role==='fallback'){
    const error=readRustOwnedStandardSumPredicateResultError(readRustStandardSumOwnershipFacts(table,checked.result).profile,Boolean(readRustStandardSumSourceIdentity(parser,checked.result)));if(error)fail(error,checked.value.span);resultType=checked.result;
   }
   captures.push({role,name:parser.freshProductBinding(),value:checked.value,origin:checked.origin});
  }
 }
 c.eat(',');const close=c.expect(')','owned standard sum predicate'),span={start,end:close.end};if(!resultType)fail('the source operands must establish a complete actual result',span);
 const receiverMoved=table.consumeOwnedBinding(movedReceiver,receiver.span),defaultMoved=ownedDefault?table.consumeOwnedBinding(ownedDefault,span):true,result=createRustOwnedStandardSumMethodResult(parser,resultType,receiverMoved,defaultMoved),capture=role=>captures.find(entry=>entry.role===role),local=role=>{const entry=capture(role);if(!entry)fail('the represented source capture is absent',span);return table.mark({k:'name',path:[entry.name],span},entry.origin);},rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const branch=readRustStandardSumPredicateBranch(sum.owner,variant,method.value),field=readRustStandardSumSourceFieldArgument(sum.owner,variant),count=readRustStandardSumSourceConstructorArity(sum.owner,variant),callbackField=readRustStandardSumPredicateCallbackArgument(sum.owner,method.value,branch),binding=count===1&&callbackField>=0?parser.freshProductBinding():null,payload=binding?table.mark({k:'name',path:[binding],span},sum.arguments[field]):null;
  let body;if(branch==='true'||branch==='false')body={k:'bool',value:branch==='true',span};else if(branch==='default')body=local('default');else if(['mapper','fallback','predicate'].includes(branch))body={k:'app',fn:local(branch),args:callbackField<0?[]:[payload],span};else fail('the actual source constructor has no method branch',span);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:count===1?[binding?{k:'bind',name:binding,span}:{k:'wild',span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local('receiver')],rows,span};for(let at=captures.length-1;at>=0;at-=1){const entry=captures[at];body={k:'let',name:entry.name,type:entry.origin.type,value:entry.value,body,span};}
 return table.mark(body,result);
}
