// Ordered conditional continuations. Failure contains no names from an inner
// scope. A one-field product distinguishes success with any payload from absence.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
/** @param {string} role @returns {string} */
export function readConditionalChainOperation(role){
 if(role==='result')return 'readConditionalChainResult';
 if(role==='success')return 'constructConditionalChainSuccess';
 if(role==='failure')return 'constructConditionalChainFailure';
 return '';
}
/** @param {string} operation @returns {boolean} */
export function acceptConditionalChainOperation(operation){return operation===readConditionalChainOperation('result')||operation===readConditionalChainOperation('success')||operation===readConditionalChainOperation('failure');}
/** @returns {number} */
export function readConditionalChainStageLimit(){return 256;}
/** Return actual let-token position, or -1 when the existing Boolean reader owns
 * the condition. Opaque tokens and balanced operand groups are never inspected.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @returns {number} */
export function findRustConditionalChainPattern(kinds,values,start){
 if(kinds.length!==values.length||!Number.isSafeInteger(start)||start<0)return -1;
 let closes=[];
 for(let at=start;at<kinds.length;at+=1){
  const kind=readRequiredTextFact(kinds,at),value=readRequiredTextFact(values,at);
  if(!closes.length&&kind==='identifier'&&value==='let')return at;
  if(kind!=='punct')continue;
  if(!closes.length&&value==='{')return -1;
  if(value==='('||value==='['||value==='{')closes=closes.concat([value==='('?')':value==='['?']':'}']);
  else if(value===')'||value===']'||value==='}'){
   if(!closes.length||readRequiredTextFact(closes,closes.length-1)!==value)return -1;
   closes=closes.slice(0,closes.length-1);
  }
 }
 return -1;
}
/** Postorder layers over the actual stages: zero Boolean, one pattern.
 * @param {number[]} forms @returns {number[]} */
export function planConditionalChainLayers(forms){
 if(forms.length<1||forms.length>readConditionalChainStageLimit())return [];
 let result=[],at=forms.length;
 while(at>0){at-=1;const form=readRequiredNumberFact(forms,at);if(form!==0&&form!==1)return [];result=result.concat([form,at]);}
 return result;
}
/** Unguarded coverage arm indices, then all actual arms in reverse order.
 * A guarded arm never discharges exhaustiveness by itself.
 * @param {number[]} guards @returns {number[]} */
export function planConditionalMatchArms(guards){
 if(guards.length<1||guards.length>readConditionalChainStageLimit())return [];
 let coverage=[],layers=[];
 for(let at=0;at<guards.length;at+=1){const guard=readRequiredNumberFact(guards,at);if(guard!==0&&guard!==1)return [];if(guard===0)coverage=coverage.concat([at]);}
 let at=guards.length;while(at>0){at-=1;layers=layers.concat([at]);}
 return [coverage.length].concat(coverage,layers);
}
