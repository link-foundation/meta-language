// Source-owned arity completion and explicit parameter-lifting bridge decisions.
import {readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {boolean} optional @param {string} kind @param {string} absence @returns {boolean} */
export function acceptBodyEntryOptionalParameter(optional,kind,absence){return optional&&kind==='none'&&absence==='undefined';}
/** Slots name actual arguments; -1 constructs the implicit undefined value.
 * @param {string} language @param {string[]} kinds @param {string[]} absences
 * @param {number} provided @returns {number[]} */
export function planOptionalCallableArguments(language,kinds,absences,provided){
 if(language!=='JavaScript'||kinds.length!==absences.length||kinds.length>1024||!Number.isSafeInteger(provided)||provided<0||provided>kinds.length)return [];
 let result=[kinds.length];
 for(let at=0;at<kinds.length;at+=1){if(at<provided)result=result.concat([at]);else{const absence=readRequiredTextFact(absences,at);if(readRequiredTextFact(kinds,at)!=='option'||absence!=='undefined'&&absence!=='both')return [];result=result.concat([-1]);}}
 return result;
}
/** Actual callback parameters may accept the supplied type directly or its
 * explicit present wrapper. Every other representation retains its obligation.
 * @param {string} language @param {number} supplied @param {boolean[]} exact
 * @param {boolean[]} lifted @returns {number[]} */
export function planOptionalCallableParameterBridge(language,supplied,exact,lifted){
 if(language!=='JavaScript'||exact.length!==lifted.length||!Number.isSafeInteger(supplied)||supplied<exact.length||exact.length>1024)return [];
 let result=[exact.length],needed=false;
 for(let at=0;at<exact.length;at+=1){if(readRequiredBooleanFact(exact,at))result=result.concat([0]);else if(readRequiredBooleanFact(lifted,at)){needed=true;result=result.concat([1]);}else return [];}
 return needed?result:[];
}
