// Actual source origins/callable syntax into source-generated conversion plans.
import {unsupported} from './diagnostics.js';
import {readRustParameterOnlyCopyClosure} from './rust-copy-closures.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity,createRustStandardSumSourceOrigin,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumConversionArity,readRustStandardSumConversionOwner,planRustStandardSumConversionArguments,readRustStandardSumConversionCallbackArgument,readRustStandardSumConversionConstructor,readRustStandardSumConversionValue,readRustStandardSumConversionCallbackError,readRustStandardSumConversionSourceVersionError} from './rust-standard-sum-conversion-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard sum conversion',reason,span);};
function readCallback(parser,path,parameter,span){
 const parameters=parameter?[parameter]:[];
 // An unknown result has no Type/function return slot at all. Only actual
 // parameter origins reach the parameter-only closure/function-item reader.
 const value=readRustParameterOnlyCopyClosure(parser,path,parameters);
 const origin=parser.origins.origin(value,path),actual=origin.callableParameters??[],output=origin.callableResult;
 const error=readRustStandardSumConversionCallbackError(origin.type?.kind??'',origin.type?.params?.length??-1,parameters.length,origin.wrappers.length,parser.origins.immutableProduct(origin),Boolean(output&&parser.origins.immutableProduct(output)&&readRustStandardSumSourceIdentity(parser,output)));
 if(error)fail(error,value.span??span);
 if(actual.length!==parameters.length)fail('the conversion callable must retain every actual parameter origin',value.span??span);
 for(let at=0;at<parameters.length;at+=1){parser.origins.argument(actual[at],parameters[at],value.span??span);if(readRustStandardSumSourceIdentity(parser,actual[at])!==readRustStandardSumSourceIdentity(parser,parameters[at]))fail('the conversion callback parameter must preserve its complete actual source type',value.span??span);}
 return {value,origin,result:output};
}

export function readRustStandardSumSourceConversion(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const source=readRustStandardSumSourceOrigin(parser,parser.origins.origin(receiver,path));if(!source)return null;
 const sum=source.standardSumSource,arity=readRustStandardSumConversionArity(sum.owner,method.value);if(arity<0)return null;
 const versionError=readRustStandardSumConversionSourceVersionError(sum.owner,method.value,parser.sourceVersionMinor);if(versionError)fail(versionError,method);
 if(source.wrappers.length||!parser.origins.immutableProduct(source))fail('the conversion receiver needs its actual direct immutable Copy standard sum',receiver.span);
 const c=parser.cursor,start=receiver.span?.start??method.start,owner=readRustStandardSumConversionOwner(sum.owner,method.value),argumentPlan=planRustStandardSumConversionArguments(sum.owner,method.value),callbackArgument=readRustStandardSumConversionCallbackArgument(sum.owner,method.value);
 c.expect('(','standard sum conversion');
 const captures=[{name:parser.freshProductBinding(),value:receiver,origin:source}];let argument=null,argumentOrigin=null,callback=null;
 if(arity>0&&callbackArgument===-2){
  argument=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));
  argumentOrigin=parser.origins.origin(argument,path);
 }else if(callbackArgument!==-2){
  callback=readCallback(parser,path,callbackArgument<0?null:sum.arguments[callbackArgument],{start,end:method.end});argumentOrigin=callback.result;
 }
 if(arity>0)c.eat(',');const close=c.expect(')','standard sum conversion'),span={start,end:close.end};
 const arguments_=argumentPlan.map(index=>index<0?argumentOrigin:sum.arguments[index]);
 if(arguments_.some(origin=>!origin))fail('the conversion result needs every actual source argument',span);
 const result=createRustStandardSumSourceOrigin(parser,owner,arguments_,span),output=result.standardSumSource;
 if(argument)captures.push({name:parser.freshProductBinding(),value:argument,origin:output.arguments[1]});
 if(callback)captures.push({name:parser.freshProductBinding(),value:callback.value,origin:callback.origin});
 const local=capture=>parser.origins.mark({k:'name',path:[capture.name],span},capture.origin),rows=[];
 for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
  const field=readRustStandardSumSourceFieldArgument(sum.owner,variant),count=readRustStandardSumSourceConstructorArity(sum.owner,variant),binding=count===1?parser.freshProductBinding():null;
  const payload=binding?parser.origins.mark({k:'name',path:[binding],span},sum.arguments[field]):null;
  const role=readRustStandardSumConversionValue(sum.owner,variant,method.value),target=readRustStandardSumConversionConstructor(sum.owner,variant,method.value);let values;
  if(role==='payload')values=[payload];else if(role==='empty')values=[];else if(role==='argument')values=[local(captures[1])];else if(role==='invoke callback')values=[{k:'app',fn:local(captures[1]),args:callbackArgument<0?[]:[payload],span}];else fail('the shared conversion branch value is absent',span);
  const constructor=output.constructorPaths[target];if(!constructor)fail('the conversion constructor must belong to its actual result datatype',span);
  const body=parser.origins.mark(values.length?{k:'app',fn:{k:'name',path:constructor,span},args:values,span}:{k:'name',path:constructor,span},result);
  rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:binding?[{k:'bind',name:binding,span}]:[],span}],body,span});
 }
 let body={k:'match',scrutinees:[local(captures[0])],rows,span};
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.origin.type,value:capture.value,body,span};}
 return parser.origins.mark(body,result);
}
