// Actual consuming receiver/default transitions into shared constructor plans.
// Copy-only methods keep their existing independent admission and lowering.
import {unsupported} from './diagnostics.js';
import {trackRustOwnedBindings,checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,createRustStandardSumSourceOrigin,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustOwnedStandardSumMethodArity,readRustOwnedStandardSumMethodReceiverMode,acceptRustOwnedStandardSumMethodCallProduction,readRustOwnedStandardSumMethodVersionError,readRustOwnedStandardSumMethodResult,planRustOwnedStandardSumMethodArguments,planRustOwnedStandardSumMethodBranch,readRustOwnedStandardSumMethodCaptureError,acceptRustOwnedStandardSumMethodArgument,acceptRustOwnedStandardSumMethodProduction} from './rust-owned-standard-sum-method-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust consuming owned standard sum method',reason,span);};
const clone=value=>structuredClone(value);
export function captureRustOwnedStandardSumMethodValue(parser,value,path){
 const table=parser.origins,before=table.origin(value,path),role=readRustStandardSumOwnershipFacts(table,before).profile;
 const entry=value.k==='app'&&value.fn.k==='name'?table.lookupValueOwner(value.fn.path,path):null,declared=entry?.k==='fn'?entry.result:null;
 const identity=readRustStandardSumSourceIdentity(parser,before),resultIdentity=declared?readRustStandardSumSourceIdentity(parser,declared):null;
 const recorded=Boolean(value.span&&table.freshCalls.has(`${value.span.start}:${value.span.end}`));
 if(before.sourceBindingIdentity==null&&acceptRustOwnedStandardSumMethodCallProduction(role,declared!==null&&readRustStandardSumOwnershipFacts(table,declared).profile==='owned',recorded,Boolean(identity&&identity===resultIdentity),before.wrappers.length,before.sourceBorrowedPlace===true))table.mark(value,{...before,sourceOwnedResultProof:true});
 const moved=table.transferOwnedResult(value,path);
 const error=readRustOwnedStandardSumMethodCaptureError(role,before.wrappers.length,before.sourceBorrowedPlace===true,trackRustOwnedBindings(table),before.sourceBindingIdentity??'',moved.sourceBindingIdentity??'',table.availableOwnedBinding(moved),moved.sourceOwnedResultProof===true);
 if(error)fail(error,value.span);return moved;
}
export function createRustOwnedStandardSumMethodResult(parser,origin,receiver,argument){
 const result=clone(origin),role=readRustStandardSumOwnershipFacts(parser.origins,result).profile;
 // All capability comes from the consumed real operands, never a type clone.
 for(const key of ['sourceBindingIdentity','sourcePlaceBindingIdentity'])result[key]=null;
 for(const key of ['sourceLoanBindingIdentities','sourcePlaceLoanBindingIdentities','sourceExtendingTemporaryBindings'])result[key]=[];
 result.sourceBorrowedPlace=false;result.sourceStandardSumBorrowedField=false;result.sourceStandardSumPatternMove=false;
 result.sourceOwnedResultProof=acceptRustOwnedStandardSumMethodProduction(role,receiver,argument);return result;
}
export function readRustOwnedStandardSumMethod(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const table=parser.origins,actual=table.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);if(!source)return null;
 const sum=source.standardSumSource,arity=readRustOwnedStandardSumMethodArity(sum.owner,method.value);if(arity<0)return null;
 const receiverMode=readRustOwnedStandardSumMethodReceiverMode(sum.owner,method.value,readRustStandardSumOwnershipFacts(table,{...actual,wrappers:[]}).profile);if(!receiverMode)return null;
 const version=readRustOwnedStandardSumMethodVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 const c=parser.cursor,start=receiver.span?.start??method.start;c.expect('(','consuming owned standard sum method');
 // Rust evaluates/moves self before evaluating the eager method argument.
 let receiverOrigin;
 if(receiverMode==='owned')receiverOrigin=captureRustOwnedStandardSumMethodValue(parser,receiver,path);
 else{checkRustOwnedOrigin(table,actual,receiver.span);if(actual.wrappers.length||!table.immutableProduct(actual))fail('the Copy receiver needs its actual direct immutable source proof',receiver.span);receiverOrigin=clone(actual);}
 const captures=[{name:parser.freshProductBinding(),value:receiver,origin:receiverOrigin}];
 let argument=null,argumentOrigin=null;
 if(arity===1){
  argument=withRustStandardSumSourceContext(parser,method.value==='unwrap_or'?sum.arguments[0]:null,()=>parser.expr(path));
  argumentOrigin=table.origin(argument,path);
  if(method.value==='unwrap_or'){
   table.argument(argumentOrigin,sum.arguments[0],argument.span);
   const identity=readRustStandardSumSourceIdentity(parser,argumentOrigin);if(!identity||identity!==readRustStandardSumSourceIdentity(parser,sum.arguments[0]))fail('the eager fallback must preserve the complete actual payload source type',argument.span);
  }
  const role=readRustStandardSumOwnershipFacts(table,argumentOrigin).profile;
  if(!acceptRustOwnedStandardSumMethodArgument(role)||!readRustStandardSumSourceIdentity(parser,argumentOrigin))fail('the eager argument needs its actual complete Copy or supported owned source type',argument.span);
  if(role==='owned')argumentOrigin=captureRustOwnedStandardSumMethodValue(parser,argument,path);
  captures.push({name:parser.freshProductBinding(),value:argument,origin:argumentOrigin});c.eat(',');
 }
 const close=c.expect(')','consuming owned standard sum method'),span={start,end:close.end},resultPolicy=readRustOwnedStandardSumMethodResult(sum.owner,method.value);
 const resultType=resultPolicy==='payload'?sum.arguments[0]:createRustStandardSumSourceOrigin(parser,resultPolicy,planRustOwnedStandardSumMethodArguments(sum.owner,method.value).map(index=>index<0?argumentOrigin:sum.arguments[index]),span);
 // Both eager operands are consumed regardless of the selected constructor.
 const receiverMoved=receiverMode==='copy'||table.consumeOwnedBinding(receiverOrigin,receiver.span);let argumentMoved=true;
 if(argumentOrigin&&readRustStandardSumOwnershipFacts(table,argumentOrigin).profile==='owned')argumentMoved=table.consumeOwnedBinding(argumentOrigin,argument.span);
 const result=createRustOwnedStandardSumMethodResult(parser,resultType,receiverMoved,argumentMoved),local=capture=>table.mark({k:'name',path:[capture.name],span},capture.origin),rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const plan=planRustOwnedStandardSumMethodBranch(sum.owner,variant,method.value);if(plan.length!==2)fail('the actual constructor has no consuming branch',span);
  const count=readRustStandardSumSourceConstructorArity(sum.owner,variant),field=readRustStandardSumSourceFieldArgument(sum.owner,variant),binding=count===1&&plan[1]==='payload'?parser.freshProductBinding():null;
  const payload=binding?table.mark({k:'name',path:[binding],span},sum.arguments[field]):null;
  const value=plan[1]==='payload'?payload:plan[1]==='argument'?local(captures[1]):null;let body=value;
  if(plan[0]){const constructor=result.standardSumSource.constructorPaths[plan[0]];if(!constructor)fail('the consuming result constructor has no actual datatype owner',span);body=value?{k:'app',fn:{k:'name',path:constructor,span},args:[value],span}:{k:'name',path:constructor,span};}
  if(!body)fail('the consuming branch must retain its actual result value',span);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:count===1?[binding?{k:'bind',name:binding,span}:{k:'wild',span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local(captures[0])],rows,span};for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.origin.type,value:capture.value,body,span};}
 return table.mark(body,result);
}
