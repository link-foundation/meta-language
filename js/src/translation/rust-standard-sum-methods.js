// Actual receiver/callable origins into the common constructor/match algebra.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {carryRustCopyClosureValueUnion} from './rust-copy-closure-lifetimes.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {captureRustOwnedStandardSumMethodValue,createRustOwnedStandardSumMethodResult} from './rust-owned-standard-sum-methods.js';
import {readRustExpectedCopyClosure,readRustParameterOnlyCopyClosure,readRustCopyClosureResultContextError} from './rust-copy-closures.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,createRustStandardSumSourceOrigin,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumMethodArity,acceptRustStandardSumOwnedPanicReceiver,readRustStandardSumMethodCallbackArgument,readRustStandardSumMethodResult,readRustStandardSumMethodBranch,readRustStandardSumMethodCallbackError,readRustStandardSumMethodSourceVersionError,readRustStandardSumMethodPanicMessageRole} from './rust-standard-sum-method-rules.js';
import {readRustOptionPanicMessage,readRustPanicMessageSourceError,readRustPanicValueOperation} from './rust-panic-value-rules.js';

const fail=(reason,span)=>{throw unsupported('Rust standard sum method',reason,span);};
const clone=value=>structuredClone(value);

function callback(parser,path,parameter,result,span){
 const parameters=parameter?[parameter]:[];
 const expected=result?{...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:result.type},path),copyEvidence:true,callableParameters:parameters.map(clone),callableResult:clone(result)}:null;
 const value=result?readRustExpectedCopyClosure(parser,path,expected):readRustParameterOnlyCopyClosure(parser,path,parameters);
 const origin=parser.origins.origin(value,path);
 const actual=origin.callableParameters??[],output=origin.callableResult;
 const error=readRustStandardSumMethodCallbackError(origin.type?.kind??'',origin.type?.params?.length??-1,parameters.length,origin.wrappers.length,parser.origins.immutableProduct(origin),Boolean(output&&parser.origins.immutableProduct(output)&&readRustStandardSumSourceIdentity(parser,output)));
 if(error)fail(error,value.span??span);
 if(actual.length!==parameters.length)fail('the callback must retain every actual source parameter origin',value.span??span);
 for(let at=0;at<parameters.length;at+=1){
  parser.origins.argument(actual[at],parameters[at],value.span??span);
  if(readRustStandardSumSourceIdentity(parser,actual[at])!==readRustStandardSumSourceIdentity(parser,parameters[at]))fail('the callback parameter must preserve its complete actual source type',value.span??span);
 }
 if(result){
  parser.origins.argument(output,result,value.span??span);
  if(readRustStandardSumSourceIdentity(parser,output)!==readRustStandardSumSourceIdentity(parser,result))fail('the callback result must preserve its complete actual source type',value.span??span);
 }
 return {value,origin,result:output};
}

function construct(parser,origin,variant,args,span){
 const sum=origin.standardSumSource,path=sum.constructorPaths[variant];
 if(!path)fail('the result constructor must belong to the actual source sum',span);
 const node=args.length?{k:'app',fn:{k:'name',path,span},args,span}:{k:'name',path,span};
 return parser.origins.mark(node,origin);
}

// Called before the ordinary postfix reader consumes '(' or parses arguments.
export function readRustStandardSumSourceMethod(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const actual=parser.origins.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);
 if(!source)return null;
 const sum=source.standardSumSource,arity=readRustStandardSumMethodArity(sum.owner,method.value);
 if(arity<0)return null;
 const versionError=readRustStandardSumMethodSourceVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(versionError)fail(versionError,method);
 const owned=acceptRustStandardSumOwnedPanicReceiver(sum.owner,method.value,readRustStandardSumOwnershipFacts(parser.origins,source).profile);
 if(source.wrappers.length||!owned&&!parser.origins.immutableProduct(source))fail('the receiver needs its actual direct Copy or supported owned Option panic source',receiver.span);
 const c=parser.cursor,start=receiver.span?.start??method.start,resultPolicy=readRustStandardSumMethodResult(sum.owner,method.value);
 c.expect('(','standard sum method');
 checkRustOwnedOrigin(parser.origins,actual,receiver.span);
 const receiverOrigin=owned?captureRustOwnedStandardSumMethodValue(parser,receiver,path):actual;
 const captures=[{name:parser.freshProductBinding(),value:receiver,origin:receiverOrigin}];
 let argument=null,callable=null,resultOrigin=null;
 const callbackArgument=readRustStandardSumMethodCallbackArgument(sum.owner,method.value);
 const messageRole=readRustStandardSumMethodPanicMessageRole(sum.owner,method.value);
 if(messageRole==='supplied message'){
  argument=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));
  const message=parser.origins.origin(argument,path),error=readRustPanicMessageSourceError(method.value,message.type?.kind??'',message.textSourceModel??'',message.wrappers.length,message.proven===true,message.wrappers.every(layer=>layer==='reference'));
  if(error)fail(error,argument.span);
  captures.push({name:parser.freshProductBinding(),value:argument,origin:message});
 }else if(method.value==='unwrap_or'){
  argument=withRustStandardSumSourceContext(parser,sum.arguments[0],()=>parser.expr(path));
  const error=readRustCopyClosureResultContextError(argument,sum.arguments[0]);if(error)fail(error,argument.span);
  parser.origins.argument(parser.origins.origin(argument,path),sum.arguments[0],argument.span);
  captures.push({name:parser.freshProductBinding(),value:argument,origin:sum.arguments[0]});
 }else if(callbackArgument!==-2){
  // The enclosing return context can describe a later expression. A mapper's
  // result comes from its actual callable; fallback results use receiver T.
  const result=method.value==='unwrap_or_else'?sum.arguments[0]:null;
  callable=callback(parser,path,callbackArgument<0?null:sum.arguments[callbackArgument],result,{start,end:method.end});
  captures.push({name:parser.freshProductBinding(),value:callable.value,origin:callable.origin});
 }
 if(arity>0)c.eat(',');const close=c.expect(')','standard sum method'),span={start,end:close.end};
 const receiverMoved=!owned||parser.origins.consumeOwnedBinding(receiverOrigin,receiver.span);
 if(resultPolicy==='boolean')resultOrigin={...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 else if(resultPolicy==='payload')resultOrigin=clone(sum.arguments[0]);
 else if(resultPolicy==='mapped sum')resultOrigin=createRustStandardSumSourceOrigin(parser,sum.owner,[callable.result,...sum.arguments.slice(1)],span);
 else if(resultPolicy==='bound sum'){
  resultOrigin=readRustStandardSumSourceOrigin(parser,callable.result);
  if(!resultOrigin||resultOrigin.wrappers.length||resultOrigin.standardSumSource.owner!==sum.owner)fail('and_then must return an actual direct standard sum of the same family',span);
  if(sum.arguments.length>1&&readRustStandardSumSourceIdentity(parser,sum.arguments[1])!==readRustStandardSumSourceIdentity(parser,resultOrigin.standardSumSource.arguments[1]))fail('and_then must preserve the complete actual Result error type',span);
 }else fail('the shared method result policy is absent',span);
 const local=(capture,range=span)=>parser.origins.mark({k:'name',path:[capture.name],span:range},capture.origin);
 const rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const disposition=readRustStandardSumMethodBranch(sum.owner,variant,method.value),field=readRustStandardSumSourceFieldArgument(sum.owner,variant),arity=readRustStandardSumSourceConstructorArity(sum.owner,variant);
  const binding=arity===1?parser.freshProductBinding():null;
  const payload=binding?parser.origins.mark({k:'name',path:[binding],span},sum.arguments[field]):null;
  let body;
  if(disposition==='true'||disposition==='false')body={k:'bool',value:disposition==='true',span};
  else if(disposition==='payload')body=payload;
  else if(disposition==='default')body=local(captures[1]);
  else if(disposition==='panic'){
   const message=messageRole==='supplied message'?local(captures[1]):{k:'str',value:readRustOptionPanicMessage(method.value,''),span};
   body={k:'math',op:readRustPanicValueOperation(),name:'Rust standard Option panic message',args:[{value:message,spread:false}],span};
  }
  else if(disposition==='invoke fallback')body={k:'app',fn:local(captures[1]),args:callbackArgument<0?[]:[payload],span};
  else if(disposition==='construct mapped')body=construct(parser,resultOrigin,variant,[{k:'app',fn:local(captures[1]),args:[payload],span}],span);
  else if(disposition==='invoke mapper')body={k:'app',fn:local(captures[1]),args:[payload],span};
  else if(disposition==='forward failure')body=construct(parser,resultOrigin,variant,payload?[payload]:[],span);
  else fail('the shared constructor branch policy is absent',span);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:binding?[{k:'bind',name:binding,span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local(captures[0])],rows,span};
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.origin.type,value:capture.value,body,span};}
 const valueOrigins=[receiverOrigin,...captures.slice(1).map(capture=>parser.origins.origin(capture.value,path))];
 return parser.origins.mark(body,carryRustCopyClosureValueUnion(owned?createRustOwnedStandardSumMethodResult(parser,resultOrigin,receiverMoved,true):resultOrigin,valueOrigins));
}
