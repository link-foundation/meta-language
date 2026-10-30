// Actual inference variable owners transported through shared source slots.
import {planJavaScriptMissingArgumentInference} from './missing-argument-rules.js';
export function completeMissingCallableInference(inference,parameters,args,defaults,where,queue){
 const plan=planJavaScriptMissingArgumentInference(parameters.map(param=>param.kind==='var'),defaults,args.length);
 if(!plan.length)return args;
 const result=plan.map((slot,index)=>slot>=0?args[slot]:slot===-2?parameters[index]:{kind:'option',element:inference.fresh(),absence:'undefined'});
 if(queue)inference.calls.push({signature:{params:parameters,defaults:parameters.map(()=>false),optionalParams:parameters.map(()=>false)},args:result,where});
 return result;
}
