// Actual declarations, environments and typed values into the shared order plan.
import {createSurfaceReferenceFacts} from './surface-reference-facts.js';
import {containsParameterDefaultNameReference,requireSequentialParameterInitialization,planSequentialParameterInitialization} from './sequential-parameter-default-rules.js';
import {acceptArgumentCount} from './frontend-rules.js';
import {INT} from './types.js';
import {coerce} from './check.js';
import {typeError,unsupported} from './diagnostics.js';
import {checkJavaScriptBindingDefaultValue} from './javascript-binding-defaults.js';
import {mayOmitTypedCallableParameter} from './optional-callables.js';
export function requiresSequentialParameterDefaults(parameters){
 const facts=createSurfaceReferenceFacts(parameters.map(param=>param.default??null));
 return requireSequentialParameterInitialization(parameters.map(param=>Boolean(param.default||param.optional)),containsParameterDefaultNameReference(facts.forms,facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,parameters.map(param=>param.name)));
}
export function checkSequentialParameterCall(checker,entry,args,env,path,span){
 const parameters=entry.params,provided=args.length;
 if(provided>parameters.length)throw typeError(entry.fullName+' expects '+parameters.length+' arguments but got '+provided,span);
 if(!acceptArgumentCount(parameters.map(param=>mayOmitTypedCallableParameter(checker.language,param)),provided))throw unsupported('partial application',entry.fullName+' expects '+parameters.length+' arguments',span);
 const plan=planSequentialParameterInitialization(parameters.length,provided);if(!plan.length)throw unsupported('parameter initialization depth','the actual parameter list exceeds its shared exact source budget',span);
 const inputs=new Map(),bindings=[],initialization=new Map(),values=[];
 for(let at=0;at<plan.length;at+=2){const slot=plan[at+1],param=parameters[slot];
  if(plan[at]===0){
   const value=checker.expr(args[slot],env,path,param.guard&&args[slot].k!=='num'?INT:param.type),name=checker.freshName('ml_parameter_argument');
   bindings.push({name,value});inputs.set(slot,{k:'var',name,type:value.type});continue;
  }
  const input=inputs.get(slot),fallback=param.default??(mayOmitTypedCallableParameter(checker.language,param)?{k:'none',absence:'undefined'}:null);
  let value;if(input&&fallback)value=checkJavaScriptBindingDefaultValue(checker,input,fallback,span,initialization,entry.modulePath);
  else if(input)value=input;
  else if(fallback)value=checker.expr(fallback,initialization,entry.modulePath,param.guard?INT:param.type);
  else throw unsupported('partial application',entry.fullName+' expects '+parameters.length+' arguments',span);
  if(!param.guard)value=coerce(value,param.type,checker.language,span);
  bindings.push({name:param.name,value});initialization.set(param.name,value.type);
  let selected=coerce({k:'var',name:param.name,type:value.type},param.type,checker.language,span,param.guard?'checked':undefined);
  if(selected.k==='cast'&&selected.flavor==='checked')selected={...selected,message:param.guard.message,order:param.guard.order};
  values.push(selected);
 }
 let result=checker.guardOrder({k:'call',fn:entry.fullName,args:values,type:entry.ret});
 for(const binding of bindings.reverse())result={k:'let',...binding,body:result,type:result.type};return result;
}
