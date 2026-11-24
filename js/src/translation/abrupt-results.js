// Typed graph transport selected by the complete shared result-role rule.
import {readAbruptResultCoercionForm} from './abrupt-result-rules.js';
export function coerceAbruptResult(value,type){
 const form=readAbruptResultCoercionForm(value.k,value.op??'');
 if(form==='leaf')return {...value,type};
 if(form==='body'){const body=coerceAbruptResult(value.body,type);return body?{...value,body,type}:null;}
 if(form==='branches'){const then=coerceAbruptResult(value.then,type);if(!then)return null;const otherwise=coerceAbruptResult(value.else,type);return otherwise?{...value,then,else:otherwise,type}:null;}
 return null;
}
