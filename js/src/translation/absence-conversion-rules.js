import {requireJavaScriptAbsenceOnlyValue} from './absence-only-rules.js';
/** Convert only an actual absence-only domain into an optional target that
 * can retain every source nullish alternative, including a present Unit.
 * @param {string} language @param {string} kind @param {string} element
 * @param {string} absence @param {string} target @param {string} policy
 * @returns {string} */
export function readJavaScriptAbsenceOnlyConversion(language,kind,element,absence,target,policy){
 if(!requireJavaScriptAbsenceOnlyValue(language,kind,element)||target!=='option')return '';
 if(kind==='unit'||absence==='undefined')return policy==='undefined'||policy==='both'?'undefined':'';
 return (absence==='null'||absence==='both')&&policy==='both'?'tagged':'';
}
