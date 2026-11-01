// Actual standard owners/Copy values into shared eager/lazy combinator plans.
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustExpectedCopyClosure,readRustParameterOnlyCopyClosure} from './rust-copy-closures.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,resolveRustStandardSumSourceConstructor,createRustStandardSumSourceOrigin,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumMethodCallbackError} from './rust-standard-sum-method-rules.js';
import {readRustStandardSumCombinatorArity,readRustStandardSumCombinatorCaptureRoles,acceptRustStandardSumCombinatorReceiverContext,readRustStandardSumCombinatorRetainedArguments,readRustStandardSumCombinatorCallbackArgument,planRustStandardSumCombinatorRightArguments,readRustStandardSumCombinatorBranch,readRustStandardSumCombinatorConstructor,readRustStandardSumCombinatorVersionError} from './rust-standard-sum-combinator-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard sum combinator',reason,span);};
const clone=value=>structuredClone(value);
function checkRetainedArgument(parser,left,right,span){
 parser.origins.argument(left,right,span);const identity=readRustStandardSumSourceIdentity(parser,left);
 if(!identity||identity!==readRustStandardSumSourceIdentity(parser,right))fail('the retained source argument must preserve its complete actual type and owner',span);
}
function readConstrainedRightOperand(parser,left,method,path){
 const c=parser.cursor,start=c.peek();if(start.kind!=='identifier')return null;
 const segments=[start.value];let offset=1;
 while(c.is('::',offset)&&c.peek(offset+1).kind==='identifier'){
  if(segments.length>=128)fail('the direct constructor path exceeds its finite source budget',start);
  segments.push(c.peek(offset+1).value);offset+=2;
 }
 if(!c.is('(',offset))return null;
 const selected=resolveRustStandardSumSourceConstructor(parser,segments,path);
 if(!selected||selected.owner!==left.standardSumSource.owner)return null;
 const plan=planRustStandardSumCombinatorRightArguments(selected.owner,method,selected.constructor);if(!plan.length)return null;
 for(let at=0;at<offset;at+=1)c.next();c.expect('(','constrained standard constructor');
 if(c.is(')'))throw new TranslationError('type','the actual standard result constructor needs one field',start);
 const argument=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));
 if(c.eat(',')&&!c.is(')'))throw new TranslationError('type','the actual standard result constructor needs one field',start);
 const close=c.expect(')','constrained standard constructor'),span={start:start.start,end:close.end};
 if(!c.is(')')&&!c.is(','))fail('partial result constraints apply only to the complete direct right operand; a surrounding operation needs its own complete source type',span);
 const actual=parser.origins.origin(argument,path);
 if(!parser.origins.immutableProduct(actual)||!actual.type&&!actual.fields)fail('the constrained constructor field needs its actual complete immutable Copy source type',span);
 const values={'argument':actual,'left value':left.standardSumSource.arguments[0],'left error':left.standardSumSource.arguments[1]};
 const arguments_=plan.map(role=>{if(!values[role])fail('the actual receiver constructor constraint is incomplete',span);return values[role];});
 const result=createRustStandardSumSourceOrigin(parser,selected.owner,arguments_,span);
 return parser.origins.mark({k:'app',fn:{k:'name',path:result.standardSumSource.constructorPaths[selected.constructor],span},args:[argument],span},result);
}
function readFallbackCallback(parser,source,method,path,span){
 const field=readRustStandardSumCombinatorCallbackArgument(source.standardSumSource.owner,method),parameters=field<0?[]:[source.standardSumSource.arguments[field]];
 const expectedResult=acceptRustStandardSumCombinatorReceiverContext(source.standardSumSource.owner,method)?source:null;
 const expected=expectedResult?{...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:expectedResult.type},path),copyEvidence:true,callableParameters:parameters.map(clone),callableResult:clone(expectedResult)}:null;
 const value=expected?readRustExpectedCopyClosure(parser,path,expected):readRustParameterOnlyCopyClosure(parser,path,parameters);
 const origin=parser.origins.origin(value,path),actual=origin.callableParameters??[],result=origin.callableResult;
 const error=readRustStandardSumMethodCallbackError(origin.type?.kind??'',origin.type?.params?.length??-1,parameters.length,origin.wrappers.length,parser.origins.immutableProduct(origin),Boolean(result&&parser.origins.immutableProduct(result)&&readRustStandardSumSourceIdentity(parser,result)));
 if(error)fail(error,value.span??span);
 if(actual.length!==parameters.length)fail('the fallback must retain every actual source parameter',value.span??span);
 for(let at=0;at<parameters.length;at+=1)checkRetainedArgument(parser,actual[at],parameters[at],value.span??span);
 return {value,origin,result};
}

export function readRustStandardSumSourceCombinator(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const source=readRustStandardSumSourceOrigin(parser,parser.origins.origin(receiver,path));if(!source)return null;
 const left=source.standardSumSource;if(readRustStandardSumCombinatorArity(left.owner,method.value)<0)return null;
 const version=readRustStandardSumCombinatorVersionError(left.owner,method.value,parser.sourceVersionMinor);if(version)fail(version,method);
 if(source.wrappers.length||!parser.origins.immutableProduct(source))fail('the receiver needs its actual direct immutable Copy standard sum',receiver.span);
 const c=parser.cursor,start=receiver.span?.start??method.start,initial={start,end:method.end};c.expect('(','standard sum combinator');
 const roles=readRustStandardSumCombinatorCaptureRoles(method.value),captures=[{role:'receiver',name:parser.freshProductBinding(),value:receiver,origin:source}];let rightValue,rightOrigin,callable=null;
 if(roles[1]==='callback'){
  callable=readFallbackCallback(parser,source,method.value,path,initial);rightValue=callable.value;rightOrigin=callable.result;
  captures.push({role:'callback',name:parser.freshProductBinding(),value:rightValue,origin:callable.origin});
 }else{
  rightValue=readConstrainedRightOperand(parser,source,method.value,path)??withRustStandardSumSourceContext(parser,acceptRustStandardSumCombinatorReceiverContext(left.owner,method.value)?source:null,()=>parser.expr(path));
  rightOrigin=parser.origins.origin(rightValue,path);
  captures.push({role:'argument',name:parser.freshProductBinding(),value:rightValue,origin:rightOrigin});
 }
 c.eat(',');const close=c.expect(')','standard sum combinator'),span={start,end:close.end};
 const result=readRustStandardSumSourceOrigin(parser,rightOrigin);
 if(!result||result.wrappers.length||!parser.origins.immutableProduct(result)||result.standardSumSource.owner!==left.owner)fail('the right operand or fallback needs its actual direct Copy standard sum of the same family',span);
 for(const index of readRustStandardSumCombinatorRetainedArguments(left.owner,method.value))checkRetainedArgument(parser,result.standardSumSource.arguments[index],left.arguments[index],span);
 const local=role=>{const capture=captures.find(entry=>entry.role===role);if(!capture)fail('the actual combinator capture is absent',span);return parser.origins.mark({k:'name',path:[capture.name],span},capture.origin);};
 const pattern=(origin,variant)=>{
  const sum=origin.standardSumSource,count=readRustStandardSumSourceConstructorArity(sum.owner,variant),field=readRustStandardSumSourceFieldArgument(sum.owner,variant),binding=count===1?parser.freshProductBinding():null;
  return {pattern:{k:'ctor',path:sum.constructorPaths[variant],args:binding?[{k:'bind',name:binding,span}]:[],span},payload:binding?parser.origins.mark({k:'name',path:[binding],span},sum.arguments[field]):null};
 };
 const rows=[],variants=readRustStandardSumSourceConstructors(left.owner);
 for(const variant of variants)for(const other of method.value==='xor'?variants:['']){
  const first=pattern(source,variant),second=other?pattern(result,other):null,branch=readRustStandardSumCombinatorBranch(left.owner,variant,method.value,other);let body;
  if(branch==='left')body=local('receiver');
  else if(branch==='right')body=local('argument');
  else if(branch==='callback'){const field=readRustStandardSumCombinatorCallbackArgument(left.owner,method.value);body={k:'app',fn:local('callback'),args:field<0?[]:[first.payload],span};}
  else if(branch==='construct success'||branch==='construct failure'){
   const target=readRustStandardSumCombinatorConstructor(left.owner,branch==='construct success'),count=readRustStandardSumSourceConstructorArity(left.owner,target);
   body={k:'app',fn:{k:'name',path:result.standardSumSource.constructorPaths[target],span},args:count===1?[first.payload]:[],span};
  }else fail('the actual constructor pair has no represented combinator branch',span);
  rows.push({patterns:second?[first.pattern,second.pattern]:[first.pattern],body,span});
 }
 let body={k:'match',scrutinees:method.value==='xor'?[local('receiver'),local('argument')]:[local('receiver')],rows,span};
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.origin.type,value:capture.value,body,span};}
 return parser.origins.mark(body,result);
}
