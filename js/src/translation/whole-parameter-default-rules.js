// Distinct formal plan. Earlier exported required/recursive plans stay exact.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
import {acceptJavaScriptSourceBindingName} from './source-identifier-rules.js';
import {planJavaScriptRecursiveBindings,readJavaScriptRecursiveObjectBindingPositions} from './recursive-binding-rules.js';
import {findJavaScriptPatternDefaultExpressionEnd} from './array-formal-parameter-rules.js';
/** Records: ordinary [0,slot,token], defaulted ordinary [2,slot,token,first,last],
 * pattern [1,slot,length,...tree], defaulted pattern [3,slot,length,...tree,first,last].
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {number} depth @returns {number[]} */
export function planJavaScriptWholeParameterDefaults(kinds,values,start,reserved,depth){
 if(kinds.length!==values.length||!Number.isSafeInteger(start)||start<0||start>=kinds.length)return [-1,start];
 let at=start,slot=0,names=[],rows=[];
 while(at<kinds.length&&!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===')')){
  const token=at;let tree=[],pattern=false;
  if(readRequiredTextFact(kinds,at)==='punct'&&(readRequiredTextFact(values,at)==='{'||readRequiredTextFact(values,at)==='[')){
   pattern=true;tree=planJavaScriptRecursiveBindings(kinds,values,at,reserved,names,depth);if(readRequiredNumberFact(tree,0)<0)return tree;
   const positions=readJavaScriptRecursiveObjectBindingPositions(tree);for(let index=0;index<positions.length;index+=1)names=names.concat([readRequiredTextFact(values,readRequiredNumberFact(positions,index))]);at=readRequiredNumberFact(tree,0);
  }else{
   const name=readRequiredTextFact(values,at);if(!acceptJavaScriptSourceBindingName(readRequiredTextFact(kinds,at),name,names.includes(name),reserved))return [-8,at];names=names.concat([name]);at+=1;
  }
  if(at>=kinds.length)return [-1,at];let first=-1,last=-1;
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='='){first=at+1;last=findJavaScriptPatternDefaultExpressionEnd(kinds,values,first,')');if(last<0)return [-1,first];at=last;}
  if(pattern)rows=rows.concat([first<0?1:3,slot,tree.length],tree,first<0?[]:[first,last]);
  else rows=rows.concat(first<0?[0,slot,token]:[2,slot,token,first,last]);
  slot+=1;if(at>=kinds.length)return [-1,at];
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===','){at+=1;continue;}
  if(!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===')'))return [-1,at];
 }
 if(at>=kinds.length)return [-1,at];return [at+1,slot].concat(rows);
}
/** @param {number[]} plan @returns {boolean} */
export function containsJavaScriptWholeParameterDefault(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return false;let at=2,pattern=false,defaulted=false;
 while(at<plan.length){const form=readRequiredNumberFact(plan,at);if(form===0)at+=3;else if(form===2){defaulted=true;at+=5;}else{pattern=true;if(form===3)defaulted=true;at+=3+readRequiredNumberFact(plan,at+2)+(form===3?2:0);}}
 return pattern&&defaulted;
}
/** @param {number[]} plan @returns {number[]} */
export function readJavaScriptWholeParameterBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return [];let at=2,result=[];
 while(at<plan.length){const form=readRequiredNumberFact(plan,at);if(form===0||form===2){result=result.concat([readRequiredNumberFact(plan,at+2)]);at+=form===0?3:5;}else{const length=readRequiredNumberFact(plan,at+2);result=result.concat(readJavaScriptRecursiveObjectBindingPositions(plan.slice(at+3,at+3+length)));at+=3+length+(form===3?2:0);}}
 return result;
}
/** @param {string} context @returns {boolean} */
export function acceptJavaScriptWholeParameterDefaultContext(context){return context==='ordinary named function declaration'||context==='ordinary constant arrow function'||context==='ordinary constant function expression'||context==='ordinary namespace object method';}
/** Retained call-site commutation predicate. The current frontend instead uses
 * ordered body entries; this predicate never licenses moving a later default.
 * @param {number[]} plan @returns {boolean} */
export function acceptJavaScriptWholeParameterInitializationOrder(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return false;let at=2,pattern=false;
 while(at<plan.length){const form=readRequiredNumberFact(plan,at);
  if(form===0)at+=3;
  else if(form===2){if(pattern)return false;at+=5;}
  else{if(form===3&&pattern)return false;pattern=true;at+=3+readRequiredNumberFact(plan,at+2)+(form===3?2:0);}
 }
 return true;
}
