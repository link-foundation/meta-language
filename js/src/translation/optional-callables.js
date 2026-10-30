// Materialize actual source slots and typed wrappers selected by shared rules.
import {planOptionalCallableArguments,planOptionalCallableParameterBridge} from './optional-callable-rules.js';
import {sameType} from './types.js';
import {coerce} from './check.js';
import {planJavaScriptExcessArguments} from './excess-argument-rules.js';
export function mayOmitTypedCallableParameter(language,parameter){
 return Boolean(parameter.default||parameter.optional)||planOptionalCallableArguments(language,[parameter.type?.kind??''],[parameter.type?.absence??''],0).length>0;
}
export function completeOptionalCallableArguments(language,parameters,args,span){
 const plan=planOptionalCallableArguments(language,parameters.map(param=>param.kind),parameters.map(param=>param.absence??''),args.length);
 return !plan.length||args.length===parameters.length?args:plan.slice(1).map(slot=>slot>=0?args[slot]:{k:'none',absence:'undefined',span});
}
export function completeOptionalCallableTerms(parameters,args){
 const excess=planJavaScriptExcessArguments('JavaScript',parameters.length,args.length);if(excess.length)args=excess.slice(1,excess[0]+1).map(index=>args[index]);
 const plan=planOptionalCallableArguments('JavaScript',parameters.map(param=>param.kind),parameters.map(param=>param.absence??''),args.length);
 return !plan.length||args.length===parameters.length?args:plan.slice(1).map((slot,index)=>slot>=0?args[slot]:parameters[index]);
}
export function constructOptionalCallbackBridge(checker,value,supplied,span){
 if(value.type.kind!=='function')return value;
 const actual=value.type.params,exact=actual.map((type,index)=>index<supplied.length&&sameType(type,supplied[index])),lifted=actual.map((type,index)=>index<supplied.length&&type.kind==='option'&&sameType(type.element,supplied[index]));
 const plan=planOptionalCallableParameterBridge(checker.language,supplied.length,exact,lifted);if(!plan.length)return value;
 const name=checker.freshName('ml_optional_callback'),bound={k:'var',name,type:value.type},params=actual.map((_,index)=>({name:checker.freshName('ml_optional_callback_argument'),type:supplied[index]}));
 const args=params.map((param,index)=>{const argument={k:'var',name:param.name,type:param.type};return plan[index+1]===0?argument:coerce(argument,actual[index],checker.language,span);});
 const type={kind:'function',params:params.map(param=>param.type),ret:value.type.ret};
 const body={k:'lambda',params,body:{k:'apply',fn:bound,args,type:value.type.ret},type};
 return {k:'let',name,value,body,type};
}
