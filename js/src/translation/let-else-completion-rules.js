// Read actual top-level let-else punctuation without entering opaque token data.
import {readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** >=0 selects the actual else token, -1 finishes without it, -2 needs more
 * source tokens. Else flags retain the lexer's raw-identifier distinction.
 * @param {string[]} kinds @param {string[]} values @param {boolean[]} keywords
 * @returns {number} */
export function findRustLetElseCompletionClause(kinds,values,keywords){
 if(kinds.length!==values.length||kinds.length!==keywords.length)return -1;
 let depth=0,initializer=false;
 for(let at=0;at<kinds.length;at+=1){
  const kind=readRequiredTextFact(kinds,at),value=readRequiredTextFact(values,at);
  if(kind==='eof')return -1;
  if(depth===0&&kind==='punct'&&value===';')return -1;
  if(depth===0&&kind==='punct'&&value==='=')initializer=true;
  if(initializer&&depth===0&&readRequiredBooleanFact(keywords,at)&&at>0&&readRequiredTextFact(values,at-1)!=='}')return at;
  if(kind==='punct'){
   if(value==='('||value==='['||value==='{')depth+=1;
   else if(value===')'||value===']'||value==='}'){if(depth===0)return -1;depth-=1;}
  }
 }
 return -2;
}
/** Preserve the existing abort-only binding route; select this completion
 * transport only for an actual return keyword in the fallback block.
 * @param {string[]} kinds @param {string[]} values @param {boolean[]} keywords
 * @param {boolean[]} returns @returns {number} */
export function findRustReturningLetElseCompletion(kinds,values,keywords,returns){
 if(kinds.length!==returns.length)return -1;
 const selected=findRustLetElseCompletionClause(kinds,values,keywords);if(selected<0)return selected;
 let depth=0,opened=false;
 for(let at=selected+1;at<kinds.length;at+=1){
  const kind=readRequiredTextFact(kinds,at),value=readRequiredTextFact(values,at);
  if(kind==='eof')return -1;
  if(opened&&readRequiredBooleanFact(returns,at))return selected;
  if(kind==='punct'){
   if(value==='{'){depth+=1;opened=true;}
   else if(value==='}'){if(depth<1)return -1;depth-=1;if(depth===0)return -1;}
  }
  if(!opened)return -1;
 }
 return -2;
}
