import {readRequiredTextFact} from './fact-column-rules.js';
/** Source macro ownership is separate from the ordinary value namespace.
 * @param {string} owner @param {boolean} ordinary @returns {boolean} */
export function acceptRustPatternMatchMacroOwner(owner,ordinary){return ordinary&&(owner==='matches'||owner==='std::matches'||owner==='core::matches');}
/** Expansion codes retain every actual pattern/guard token in order. The
 * receiver is already parsed once and is supplied to the existing match reader.
 * First cell: original exclusive macro end; remaining cells: original token
 * indices or negative generated-token roles. Delimiter nesting remains exact.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string} close @returns {number[]} */
export function planRustPatternMatchRows(kinds,values,start,close){
 if(kinds.length!==values.length||!Number.isInteger(start)||start<0||start>=kinds.length||![')',']','}'].includes(close))return [];
 const stack=[];let end=-1;
 for(let at=start;at<kinds.length;at+=1){
  if(readRequiredTextFact(kinds,at)!=='punct')continue;
  const value=readRequiredTextFact(values,at);
  if(value==='('||value==='['||value==='{')stack.push(value==='('?')':value==='['?']':'}');
  else if(value===')'||value===']'||value==='}'){
   if(stack.length===0){if(value!==close)return [];end=at;break;}
   if(readRequiredTextFact(stack,stack.length-1)!==value)return [];stack.pop();
  }
 }
 if(end<0)return [];
 const last=end>start&&readRequiredTextFact(kinds,end-1)==='punct'&&readRequiredTextFact(values,end-1)===','?end-1:end;
 if(last===start)return [];
 const plan=[end+1,-1];for(let at=start;at<last;at+=1)plan.push(at);
 return plan.concat([-2,-3,-4,-5,-2,-6,-7]);
}
/** @param {number} code @returns {string} */
export function readRustPatternMatchTokenValue(code){return code===-1?'{':code===-2?'=>':code===-3?'true':code===-4?',':code===-5?'_':code===-6?'false':code===-7?'}':'';}
/** @param {number} code @returns {string} */
export function readRustPatternMatchTokenKind(code){return code===-3||code===-5||code===-6?'identifier':code===-1||code===-2||code===-4||code===-7?'punct':'';}
