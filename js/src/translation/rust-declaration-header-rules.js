import {readRequiredTextFact} from './fact-column-rules.js';
/** Find one complete lexical token group. Strings/comments are atomic tokens.
 * Returns the first token after its group, -2 for a longer window, -1 on error.
 * @param {string[]} kinds @param {string[]} values @returns {number} */
export function findRustDeclarationBodyBoundary(kinds,values){
 if(kinds.length!==values.length||values.length===0)return -1;
 if(readRequiredTextFact(kinds,0)!=='punct'||readRequiredTextFact(values,0)!=='{')return -1;
 const stack=['}'];
 for(let index=1;index<values.length;index+=1){
  const kind=readRequiredTextFact(kinds,index),value=readRequiredTextFact(values,index);
  if(kind==='eof')return -1;
  if(kind!=='punct')continue;
  if(value==='(')stack.push(')');else if(value==='[')stack.push(']');else if(value==='{')stack.push('}');
  else if(value===')'||value===']'||value==='}'){
   if(stack.length===0||readRequiredTextFact(stack,stack.length-1)!==value)return -1;
   stack.pop();if(stack.length===0)return index+1;
  }
 }
 return -2;
}
