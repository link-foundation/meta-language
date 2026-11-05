// Omitted source arguments are real undefined inputs, not type placeholders.
import {readRequiredBooleanFact} from './fact-column-rules.js';
/** Existing arguments use their positions; -1 is a real implicit undefined;
 * -2 retains a declared/defaulted parameter's independent inference route.
 * @param {boolean[]} inferred @param {boolean[]} defaults @param {number} provided
 * @returns {number[]} */
export function planJavaScriptMissingArgumentInference(inferred,defaults,provided){
 if(inferred.length!==defaults.length||inferred.length>1024||!Number.isSafeInteger(provided)||provided<0||provided>=inferred.length)return [];
 let result=[],needed=false;
 for(let index=0;index<inferred.length;index+=1){if(index<provided)result=result.concat([index]);else if(readRequiredBooleanFact(inferred,index)&&!readRequiredBooleanFact(defaults,index)){result=result.concat([-1]);needed=true;}else result=result.concat([-2]);}
 return needed?result:[];
}
