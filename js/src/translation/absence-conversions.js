// Actual checked values and types into the shared complete nullish conversion.
import {BOOL} from './types.js';
import {readJavaScriptAbsenceOnlyConversion} from './absence-conversion-rules.js';
export function convertJavaScriptAbsenceOnlyValue(value,type,language){
 const form=readJavaScriptAbsenceOnlyConversion(language,value.type.kind,value.type.element?.kind??'',value.type.absence??'',type.kind,type.absence??'');if(!form)return null;
 const name='ml_absence_conversion_value',variable={k:'var',name,type:value.type},none=absence=>({k:'none',absence,type});
 const body=form==='undefined'?none('undefined'):{k:'if',cond:{k:'strictEqual',left:variable,right:{k:'none',absence:'null',type:value.type},negate:false,type:BOOL},then:none('null'),else:none('undefined'),type};
 return {k:'let',name,value,body,type};
}
