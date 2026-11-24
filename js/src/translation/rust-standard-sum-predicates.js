// Actual direct Copy callbacks and complete source results into ordinary
// capture/match terms. Factories evaluate in argument order, calls stay lazy.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
import {readRustExpectedCopyClosure,readRustParameterOnlyCopyClosure,finishRustDeferredCopyClosure,readRustCopyClosureResultContextError} from './rust-copy-closures.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumMethodCallbackError} from './rust-standard-sum-method-rules.js';
import {readRustStandardSumPredicateArity,readRustStandardSumPredicateCaptureRoles,readRustStandardSumPredicateCallbackArgument,readRustStandardSumPredicateBranch,readRustStandardSumPredicateVersionError,readRustStandardSumPredicateBorrowError,readRustStandardSumPredicateDeferredError,readRustStandardSumPredicateResultSource} from './rust-standard-sum-predicate-rules.js';
const clone=value=>structuredClone(value);
const fail=(reason,span)=>{throw unsupported('Rust standard sum predicate',reason,span);};
const complete=(parser,origin)=>Boolean(origin&&parser.origins.immutableProduct(origin)&&readRustStandardSumSourceIdentity(parser,origin));
function same(parser,actual,expected,span){
 parser.origins.argument(actual,expected,span);
 const identity=readRustStandardSumSourceIdentity(parser,actual);
 if(!identity||identity!==readRustStandardSumSourceIdentity(parser,expected))fail('the callback or eager default must preserve its complete actual source result type',span);
}
function readCallback(parser,path,parameters,result,span){
 const expected=result?{...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:result.type},path),copyEvidence:true,callableParameters:parameters.map(clone),callableResult:clone(result)}:null;
 const value=expected?readRustExpectedCopyClosure(parser,path,expected):readRustParameterOnlyCopyClosure(parser,path,parameters,true);
 const origin=parser.origins.origin(value,path),pending=value.k==='lambda'&&value.ret==null&&origin.type===null;
 if(pending){
  const identity=Boolean(value.span&&origin.copyClosureSpan&&value.span.start===origin.copyClosureSpan.start&&value.span.end===origin.copyClosureSpan.end);
  const error=readRustStandardSumPredicateDeferredError(value.k,true,origin.callableParameters?.length??-1,parameters.length,origin.wrappers.length,identity);
  if(error)fail(error,value.span??span);
 }
 return {value,origin,pending,parameters,resultHint:result};
}
function finishCallback(parser,callback,result,path,span){
 if(callback.pending){callback.value=finishRustDeferredCopyClosure(parser,callback.value,result,path);callback.origin=parser.origins.origin(callback.value,path);callback.pending=false;}
 const{origin,parameters}=callback,actual=origin.callableParameters??[],output=origin.callableResult;
 const error=readRustStandardSumMethodCallbackError(origin.type?.kind??'',origin.type?.params?.length??-1,parameters.length,origin.wrappers.length,parser.origins.immutableProduct(origin),complete(parser,output));
 if(error)fail(error,callback.value.span??span);
 if(actual.length!==parameters.length)fail('the callback must retain every actual by-value source parameter',callback.value.span??span);
 for(let at=0;at<parameters.length;at+=1)same(parser,actual[at],parameters[at],callback.value.span??span);
 if(result)same(parser,output,result,callback.value.span??span);
 callback.result=output;return callback;
}

// Postfix calls this before consuming the opening parenthesis.
export function readRustStandardSumSourcePredicate(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const source=readRustStandardSumSourceOrigin(parser,parser.origins.origin(receiver,path));if(!source)return null;
 const sum=source.standardSumSource,borrow=readRustStandardSumPredicateBorrowError(sum.owner,method.value);if(borrow)fail(borrow,method);
 const arity=readRustStandardSumPredicateArity(sum.owner,method.value);if(arity<0)return null;
 const version=readRustStandardSumPredicateVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 if(source.wrappers.length||!parser.origins.immutableProduct(source))fail('the receiver needs its actual direct immutable Copy standard sum',receiver.span);
 const c=parser.cursor,start=receiver.span?.start??method.start,initial={start,end:method.end};c.expect('(','standard sum predicate');
 const boolean={...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 const roles=readRustStandardSumPredicateCaptureRoles(method.value),captures=[{role:'receiver',name:parser.freshProductBinding(),value:receiver,origin:source}],callbacks=new Map();let defaultValue=null,firstResult=null;
 const parameters=role=>{const field=readRustStandardSumPredicateCallbackArgument(sum.owner,method.value,role);if(field===-2)fail('the shared callback role is absent',initial);return field<0?[]:[sum.arguments[field]];};
 for(let at=1;at<roles.length;at+=1){
  const role=roles[at];if(at>1)c.expect(',','standard sum predicate arguments');
  if(role==='default'){
   defaultValue=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));const origin=parser.origins.origin(defaultValue,path);firstResult=complete(parser,origin)?origin:null;
   captures.push({role,name:parser.freshProductBinding(),value:defaultValue,origin});
  }else{
   const result=role==='predicate'?boolean:role==='mapper'?firstResult:null;
   const callable=readCallback(parser,path,parameters(role),result,initial);callbacks.set(role,callable);
   if(role==='fallback')firstResult=complete(parser,callable.origin.callableResult)?callable.origin.callableResult:null;
   captures.push({role,name:parser.freshProductBinding(),value:callable.value,origin:callable.origin});
  }
 }
 c.eat(',');const close=c.expect(')','standard sum predicate'),span={start,end:close.end};let resultOrigin;
 if(arity===1)resultOrigin=boolean;
 else{
  const mapped=callbacks.get('mapper').origin.callableResult;
  const selection=readRustStandardSumPredicateResultSource(complete(parser,firstResult),complete(parser,mapped));
  if(!selection)fail('the actual default or callback must establish a complete immutable Copy result; unresolved siblings cannot acquire a guessed Unit type',span);
  resultOrigin=clone(selection==='first result'?firstResult:mapped);
 }
 for(const[role,callback]of callbacks){
  finishCallback(parser,callback,role==='predicate'?boolean:resultOrigin,path,span);
  const capture=captures.find(entry=>entry.role===role);capture.value=callback.value;capture.origin=callback.origin;
 }
 if(defaultValue){
  const error=readRustCopyClosureResultContextError(defaultValue,resultOrigin);if(error)fail(error,defaultValue.span??span);
  const actual=parser.origins.origin(defaultValue,path);parser.origins.argument(actual,resultOrigin,defaultValue.span??span);
  if(complete(parser,actual))same(parser,actual,resultOrigin,defaultValue.span??span);
  captures.find(entry=>entry.role==='default').origin=clone(resultOrigin);
 }
 const capture=role=>{const found=captures.find(entry=>entry.role===role);if(!found)fail('an actual method capture is absent',span);return found;};
 const local=role=>{const found=capture(role);return parser.origins.mark({k:'name',path:[found.name],span},found.origin);};
 const rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const field=readRustStandardSumSourceFieldArgument(sum.owner,variant),count=readRustStandardSumSourceConstructorArity(sum.owner,variant),binding=count===1?parser.freshProductBinding():null;
  const payload=binding?parser.origins.mark({k:'name',path:[binding],span},sum.arguments[field]):null,branch=readRustStandardSumPredicateBranch(sum.owner,variant,method.value);let body;
  if(branch==='true'||branch==='false')body={k:'bool',value:branch==='true',span};
  else if(branch==='default')body=local('default');
  else if(branch==='mapper'||branch==='predicate'||branch==='fallback'){
   const args=readRustStandardSumPredicateCallbackArgument(sum.owner,method.value,branch)<0?[]:[payload];body={k:'app',fn:local(branch),args,span};
  }else fail('the actual standard constructor has no represented method branch',span);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:binding?[{k:'bind',name:binding,span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local('receiver')],rows,span};
 for(let at=captures.length-1;at>=0;at-=1){const found=captures[at];body={k:'let',name:found.name,type:found.origin.type,value:found.value,body,span};}
 return parser.origins.mark(body,resultOrigin);
}
