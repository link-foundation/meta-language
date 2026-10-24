// Actual source type facts select existing Unit and ordered binding operations.
import {readRustConditionalCompletion,readRustCompletionKind} from './rust-completion-rules.js';

/** @param {string[]} profiles @returns {boolean} */
export function acceptRustOrdinaryCompletionProfiles(profiles){
 for(let index=0;index<profiles.length;index+=1)if(profiles[index]!=='')return false;
 return true;
}

/** Actual keyword/punctuation columns, starting at the owning function block.
 * 1 return present,0 complete block without return,-1 more tokens needed.
 * @param {string[]} values @returns {number} */
export function readRustFunctionReturnPresence(values){
 if(values[0]!=='{')return 0;
 let depth=0;
 for(let index=0;index<values.length;index+=1){
  const value=values[index]??'';
  if(value==='return')return 1;
  if(value==='{')depth+=1;
  if(value==='}'){depth-=1;if(depth===0)return 0;}
 }
 return -1;
}

/** Extend ordinary function-tail meanings without changing the closed profile.
 * @param {string} first @param {string} second @param {boolean} hasElse
 * @param {boolean} continuation @param {boolean} terminated @returns {string} */
export function readRustOrdinaryConditionalCompletion(first,second,hasElse,continuation,terminated){
 const original=readRustConditionalCompletion(first,second,hasElse,continuation);
 if(original!=='unsupported completion')return original;
 const firstKnown=first==='returned'||first==='aborted'||first==='value'||first==='mixed';
 const secondKnown=second==='returned'||second==='aborted'||second==='value'||second==='mixed';
 if(!continuation&&!hasElse&&firstKnown)return 'implicit Unit fallback';
 if(!continuation&&hasElse&&!terminated&&firstKnown&&secondKnown)return 'terminal mixed conditional';
 return original;
}

/** @param {string} action @param {string} first @param {string} second @returns {string} */
export function readRustOrdinaryCompletionKind(action,first,second){
 if(action==='terminal mixed conditional')return 'mixed';
 if(action==='implicit Unit fallback')return first==='value'?'value':'mixed';
 if(action==='guarded continuation'&&(second==='mixed'||first==='returned'&&second==='value'))return 'mixed';
 return readRustCompletionKind(action,first,second);
}

/** An else-if may consume the outer continuation only when the preceding
 * alternative cannot fall through. General joins need a completion carrier.
 * @param {string} first @returns {boolean} */
export function acceptRustOrdinaryElseIf(first){return first==='returned'||first==='aborted';}
/** A collecting pass defers only direct call-result ownership to the complete
 * declaration catalog. The final source pass requires an actual direct Unit.
 * @param {string} kind @param {boolean} collecting @param {boolean} proven
 * @param {string} type @param {boolean} indirected @returns {string} */
export function readRustUnitStatementDisposition(kind,collecting,proven,type,indirected){
 if(kind==='abort')return 'abort';
 if(proven&&type==='unit'&&!indirected)return 'bind';
 return collecting&&kind==='app'?'bind':'refuse';
}

/** @param {string} name @returns {string} */
export function readRustFunctionMacroDisposition(name){return name==='println'?'print':'';}

/** The host has identified an actual return keyword at start. Only punctuation
 * appears in values; literal text cannot supply a delimiter. A terminal return
 * has no continuation before the owning function block closes.
 * -1 refuses a continuation; -2 requests more actual source tokens.
 * @param {string[]} values @param {number} start @returns {number} */
export function findRustTerminalReturnBoundary(values,start){
 if(!Number.isSafeInteger(start)||start<0||start>=values.length)return -1;
 const stack=[];
 for(let at=start+1;at<values.length;at+=1){
  const value=values[at]??'';
  if(value==='('||value==='['||value==='{'){stack.push(value);continue;}
  if(stack.length===0&&(value===';'||value==='}')){
   let end=at;while(values[end]===';')end+=1;
   if(end>=values.length)return -2;
   if(values[end]==='}')return end;
   return -1;
  }
  if(value===')'||value===']'||value==='}'){
   const previous=stack[stack.length-1]??'';
   if(value===')'&&previous!=='('||value===']'&&previous!=='['||value==='}'&&previous!=='{')return -1;
   stack.pop();
  }
 }
 return -2;
}
