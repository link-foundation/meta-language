// Actual declarations and solver terms into one source-owned callable interface.
import {planNamedCallableDefaultReference,readNamedCallableDefaultReferenceAbsence,requireNamedCallableDefaultInitialization} from './named-default-callable-rules.js';
import {checkSequentialParameterCall} from './sequential-parameter-defaults.js';
import {INT,option} from './types.js';
import {unsupported} from './diagnostics.js';
export function namedCallableDefaultFlags(parameters){return parameters.map(param=>requireNamedCallableDefaultInitialization(Boolean(param.default),param.optional===true,param.default?.k??'',param.default?.absence??''));}
function parameterDomain(type,mode,resolve){
 if(mode>=2)type=INT;
 if(mode===0||mode===2)return type;
 const actual=resolve(type),absence=readNamedCallableDefaultReferenceAbsence(actual.kind,actual.absence??'');
 if(!absence)throw unsupported('named callable default domain','an optional interface needs its complete actual absence policy');
 return actual.kind==='option'?{...actual,absence}:{kind:'option',element:type,absence};
}
export function namedCallableReferenceTerms(signature,resolve){
 const plan=planNamedCallableDefaultReference('JavaScript',signature.invocationDefaults??signature.params.map(()=>false),signature.guardedInputs??signature.params.map(()=>false));
 if(!plan.length)return signature.params;
 if(plan[0]<0)throw unsupported('named callable default depth','the actual parameter list exceeds its complete source interface budget');
 return signature.params.map((type,index)=>parameterDomain(type,plan[index+1],resolve));
}
export function checkNamedCallableDefaultReference(checker,entry,span){
 if(checker.checkedProjection)return null;
 const plan=planNamedCallableDefaultReference(checker.language,namedCallableDefaultFlags(entry.params),entry.params.map(param=>Boolean(param.guard)));
 if(!plan.length)return null;
 if(plan[0]<0)throw unsupported('named callable default depth','the actual parameter list exceeds its complete source interface budget',span);
 const params=entry.params.map((param,index)=>({name:checker.freshName('ml_named_default_argument'),type:optionDomain(param.type,plan[index+1])})),env=new Map(params.map(param=>[param.name,param.type]));
 const args=params.map(param=>({k:'name',path:[param.name],span}));
 const body=checkSequentialParameterCall(checker,entry,args,env,entry.modulePath,span);
 return {k:'lambda',params,body,type:{kind:'function',params:params.map(param=>param.type),ret:body.type}};
}
function optionDomain(type,mode){const selected=parameterDomain(type,mode,value=>value);return selected.kind==='option'?option(selected.element,selected.absence):selected;}
