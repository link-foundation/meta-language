// Transport actual source arguments through the shared complete capture plan.
import {planJavaScriptExcessArguments} from './excess-argument-rules.js';
import {bindLocal,coerce} from './check.js';
import {INT} from './types.js';
function wrap(bindings,body){for(const binding of bindings.toReversed())body={k:'let',name:binding.name,value:binding.value,body,type:body.type};return body;}
export function checkJavaScriptExcessCallableArguments(checker,fn,args,env,path,span){
 if(checker.checkedProjection||fn.type.kind!=='function')return null;
 const plan=planJavaScriptExcessArguments(checker.language,fn.type.params.length,args.length);if(!plan.length)return null;
 const callee={name:checker.freshName('ml_excess_argument_callee'),value:fn},bindings=[callee];
 for(const index of plan.slice(1)){const value=checker.expr(args[index],env,path,fn.type.params[index]);bindings.push({name:checker.freshName('ml_excess_argument_value'),value});}
 const values=bindings.slice(1,plan[0]+1).map((binding,index)=>coerce({k:'var',name:binding.name,type:binding.value.type},fn.type.params[index],checker.language,args[index].span??span));
 return wrap(bindings,{k:'apply',fn:{k:'var',name:callee.name,type:fn.type},args:values,type:fn.type.ret});
}
export function checkJavaScriptExcessNamedArguments(checker,entry,args,env,path,span){
 if(checker.checkedProjection)return null;
 const plan=planJavaScriptExcessArguments(checker.language,entry.params.length,args.length);if(!plan.length)return null;
 const bindings=[],scope=new Map(env),selected=[];
 for(const index of plan.slice(1)){
  const arg=args[index],param=entry.params[index],expected=param?.guard&&arg.k!=='num'?INT:param?.type,value=checker.expr(arg,env,path,expected),name=checker.freshName('ml_excess_argument_value');
  bindings.push({name,value});bindLocal(scope,name,value.type);
  if(index<plan[0])selected.push(arg.k==='none'&&arg.absence==='undefined'?arg:{k:'name',path:[name],span:arg.span??span});
 }
 const body=checker.application({k:'name',path:['crate',...entry.fullName.split('.')],span},selected,scope,path,entry.ret,span);
 return wrap(bindings,body);
}
