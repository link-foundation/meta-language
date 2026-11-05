import {readRequiredNumberFact} from './fact-column-rules.js';
/** @param {string} context @returns {boolean} */
export function acceptJavaScriptScalarClosureDefaultContext(context){
 return context==='ordinary arrow function closure'||context==='ordinary function expression closure'||context==='ordinary object method closure';
}
/** A separate consumer of the existing complete whole-formal plan. Named and
 * constant callables keep their preceding routes; pattern plans keep theirs.
 * @param {number[]} plan @param {string} context @returns {boolean} */
export function containsJavaScriptScalarClosureDefault(plan,context){
 if(!acceptJavaScriptScalarClosureDefaultContext(context)||plan.length<2||readRequiredNumberFact(plan,0)<0)return false;
 let at=2,found=false;
 while(at<plan.length){const form=readRequiredNumberFact(plan,at);if(form===0)at+=3;else if(form===2){found=true;at+=5;}else return false;}
 return at===plan.length&&found;
}
