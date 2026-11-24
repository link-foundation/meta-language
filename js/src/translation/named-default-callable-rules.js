import {readRequiredBooleanFact} from './fact-column-rules.js';
import {acceptBodyEntryOptionalParameter} from './optional-callable-rules.js';
/** Actual initializer metadata, excluding defaults already implemented in
 * ordered callee body entries. @param {boolean} present @param {boolean} optional
 * @param {string} kind @param {string} absence @returns {boolean} */
export function requireNamedCallableDefaultInitialization(present,optional,kind,absence){return present&&!acceptBodyEntryOptionalParameter(optional,kind,absence);}
/** Mode 0 original domain; 1 optional original; 2 guarded integer;
 * 3 optional guarded integer. [] retains the original reference.
 * @param {string} language @param {boolean[]} defaults @param {boolean[]} guards
 * @returns {number[]} */
export function planNamedCallableDefaultReference(language,defaults,guards){
 if(language!=='JavaScript')return [];
 if(defaults.length!==guards.length)return [-1];
 let needed=false;for(let at=0;at<defaults.length;at+=1)if(readRequiredBooleanFact(defaults,at))needed=true;
 if(!needed)return [];if(defaults.length>1024)return [-1];
 let result=[defaults.length];
 for(let at=0;at<defaults.length;at+=1){const initialized=readRequiredBooleanFact(defaults,at),guarded=readRequiredBooleanFact(guards,at);result=result.concat([(initialized?1:0)+(guarded?2:0)]);}
 return result;
}
/** @param {string} kind @param {string} absence @returns {string} */
export function readNamedCallableDefaultReferenceAbsence(kind,absence){
 if(kind!=='option'||absence==='undefined')return 'undefined';
 return absence==='null'||absence==='both'?'both':'';
}
