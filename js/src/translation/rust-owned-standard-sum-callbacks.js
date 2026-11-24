// Actual owning receiver, safe callable contract, and lazy branch transport.
import {unsupported} from './diagnostics.js';
import {readRustOwnedCallbackClosure} from './rust-copy-closures.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {selectsRustOwnedStandardSumCallback} from './rust-owned-standard-sum-callback-selection.js';
import {captureRustOwnedStandardSumMethodValue,createRustOwnedStandardSumMethodResult} from './rust-owned-standard-sum-methods.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,createRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustOwnedStandardSumCallbackArity,readRustOwnedStandardSumCallbackVersionError,readRustOwnedStandardSumCallbackParameter,readRustOwnedStandardSumCallbackResult,readRustOwnedStandardSumCallbackRetainedArgument,readRustOwnedStandardSumCallbackBoundResultError,planRustOwnedStandardSumCallbackArguments,planRustOwnedStandardSumCallbackBranch,readRustOwnedStandardSumCallbackImmediateCaptureError,readRustOwnedStandardSumCallbackSourceError} from './rust-owned-standard-sum-callback-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust owned standard sum callback',reason,span);};
export function readRustOwnedStandardSumCallbackValue(parser,path,parameters,expected=null){
 const table=parser.origins,c=parser.cursor,callback=readRustOwnedCallbackClosure(parser,path,parameters,expected),callable=table.origin(callback,path),output=callable.callableResult;
 const error=readRustOwnedStandardSumCallbackSourceError(callable.type?.kind??'',callable.type?.params?.length??-1,parameters.length,callable.wrappers.length,table.immutableProduct(callable),readRustStandardSumOwnershipFacts(table,output).profile);if(error)fail(error,callback.span);
 if(callable.copyClosureSpan){const token=c.tokens.find(token=>token.start===callback.span?.start),identity=Boolean(callback.span&&callable.copyClosureSpan.start===callback.span.start&&callable.copyClosureSpan.end===callback.span.end),captureError=readRustOwnedStandardSumCallbackImmediateCaptureError(callable.copyClosureCaptureCount??-1,callback.k,identity,token?.value==='move',callback.k==='lambda'?callback.captures.length:0);if(captureError)fail(captureError,callback.span);}
 const inputs=callable.callableParameters??[];if(inputs.length!==parameters.length)fail('the callback must retain every actual parameter source contract',callback.span);
 for(let at=0;at<inputs.length;at+=1){table.argument(inputs[at],parameters[at],callback.span);const identity=readRustStandardSumSourceIdentity(parser,inputs[at]);if(!identity||identity!==readRustStandardSumSourceIdentity(parser,parameters[at]))fail('the callback parameter must preserve its complete actual source identity',callback.span);}
 if(expected){table.argument(output,expected,callback.span);const identity=readRustStandardSumSourceIdentity(parser,output);if(!identity||identity!==readRustStandardSumSourceIdentity(parser,expected))fail('the fallback callable must preserve the complete actual payload result contract',callback.span);}
 return {value:callback,origin:callable,result:output};
}
export function readRustOwnedStandardSumCallback(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const table=parser.origins,actual=table.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);if(!source)return null;
 const sum=source.standardSumSource;if(readRustOwnedStandardSumCallbackArity(sum.owner,method.value)<0)return null;
 const receiverRole=readRustStandardSumOwnershipFacts(table,{...actual,wrappers:[]}).profile;
 if(receiverRole!=='owned'&&(receiverRole!=='copy'||!selectsRustOwnedStandardSumCallback(parser,path)))return null;
 const version=readRustOwnedStandardSumCallbackVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 const c=parser.cursor,start=receiver.span?.start??method.start;c.expect('(','owned standard sum callback');
 // Self moves before the callback expression/factory is evaluated, even when
 // its invocation is absent on the selected constructor branch.
 let receiverOrigin;if(receiverRole==='owned')receiverOrigin=captureRustOwnedStandardSumMethodValue(parser,receiver,path);else{checkRustOwnedOrigin(table,actual,receiver.span);if(actual.wrappers.length||!table.immutableProduct(actual))fail('the copied receiver needs its actual direct immutable source proof',receiver.span);receiverOrigin=table.copy(actual);}
 const input=readRustOwnedStandardSumCallbackParameter(sum.owner,method.value),parameters=input<0?[]:[sum.arguments[input]],expected=method.value==='unwrap_or_else'?sum.arguments[0]:method.value==='or_else'&&sum.owner==='std::option::Option'?source:null;
 const {value:callback,origin:callable,result:output}=readRustOwnedStandardSumCallbackValue(parser,path,parameters,expected);
 c.eat(',');const close=c.expect(')','owned standard sum callback'),span={start,end:close.end},resultPolicy=readRustOwnedStandardSumCallbackResult(sum.owner,method.value);
 let resultType;
 if(resultPolicy==='payload')resultType=sum.arguments[0];
 else if(resultPolicy==='bound sum'){
  resultType=readRustStandardSumSourceOrigin(parser,output);const retained=readRustOwnedStandardSumCallbackRetainedArgument(sum.owner,method.value),bound=resultType?.standardSumSource;
  const retainedIdentity=retained<0?null:readRustStandardSumSourceIdentity(parser,sum.arguments[retained]);
  const identity=retained<0||Boolean(bound&&retainedIdentity&&retainedIdentity===readRustStandardSumSourceIdentity(parser,bound.arguments[retained]));
  const error=readRustOwnedStandardSumCallbackBoundResultError(sum.owner,bound?.owner??'',resultType?.wrappers.length??-1,identity);if(error)fail(error,callback.span);
 }else resultType=createRustStandardSumSourceOrigin(parser,resultPolicy,planRustOwnedStandardSumCallbackArguments(sum.owner,method.value).map(index=>index<0?output:sum.arguments[index]),span);
 const moved=receiverRole==='copy'||table.consumeOwnedBinding(receiverOrigin,receiver.span),result=createRustOwnedStandardSumMethodResult(parser,resultType,moved,true),captures=[{name:parser.freshProductBinding(),value:receiver,origin:receiverOrigin},{name:parser.freshProductBinding(),value:callback,origin:callable}],local=capture=>table.mark({k:'name',path:[capture.name],span},capture.origin),rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const plan=planRustOwnedStandardSumCallbackBranch(sum.owner,variant,method.value);if(plan.length!==2)fail('the constructor has no actual owned callback branch',span);
  const field=readRustStandardSumSourceFieldArgument(sum.owner,variant),count=readRustStandardSumSourceConstructorArity(sum.owner,variant),needsPayload=plan[1]==='payload'||plan[1]==='invoke'&&input>=0,binding=count===1&&needsPayload?parser.freshProductBinding():null,payload=binding?table.mark({k:'name',path:[binding],span},sum.arguments[field]):null;
  let body=plan[1]==='payload'?payload:plan[1]==='invoke'?{k:'app',fn:local(captures[1]),args:input<0?[]:[payload],span}:null;
  if(plan[0]){const constructor=result.standardSumSource.constructorPaths[plan[0]];if(!constructor)fail('the callback result constructor needs its actual datatype owner',span);body=body?{k:'app',fn:{k:'name',path:constructor,span},args:[body],span}:{k:'name',path:constructor,span};}
  if(!body)fail('the callback branch needs its actual result value',span);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:count===1?[binding?{k:'bind',name:binding,span}:{k:'wild',span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local(captures[0])],rows,span};for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.origin.type,value:capture.value,body,span};}
 return table.mark(body,result);
}
