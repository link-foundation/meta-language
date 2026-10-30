import {readJavaScriptOptionalPolicyWidening} from './optional-policy-rules.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {sameType} from './types.js';
export function widenJavaScriptOptionalPolicy(value,type,language,span){
 const absence=readJavaScriptOptionalPolicyWidening(language,value.type.kind,value.type.absence??'',type.kind,type.absence??'',value.type.kind==='option'&&type.kind==='option'&&sameType(value.type.element,type.element));if(!absence)return null;
 const name='ml_optional_policy_value',variable={k:'var',name,type:value.type};
 const body={k:'if',cond:constructJavaScriptOptionalPresence(variable,value.type.element,span),then:{k:'some',value:{k:'unwrapOption',arg:variable,type:value.type.element},type},else:{k:'none',absence,type},type};
 return {k:'let',name,value,body,type};
}
