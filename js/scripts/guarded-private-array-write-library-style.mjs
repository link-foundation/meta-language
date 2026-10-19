// Source-only spelling corrections for the complete checked guard planner.
// Recorded decorators retain the original algorithms and lazy evaluation.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

function readGuardedArrayStyleSyntax(source){
 const {tokens,comments}=tokenize(source,'Rust'),matching=new Map(),stack=[];
 for(let at=0;at<tokens.length;at+=1){
  if(tokens[at].kind!=='punct')continue;
  if(['(','[','{'].includes(tokens[at].value))stack.push(at);
  else if([')',']','}'].includes(tokens[at].value)){
   const start=stack.pop();assert.notEqual(start,undefined);
   assert.equal(tokens[start].value,({')':'(',']':'[','}':'{'})[tokens[at].value]);matching.set(start,at);matching.set(at,start);
  }
 }
 assert.equal(stack.length,0);
 const opaque=new Set();
 for(let at=0;at<tokens.length;at+=1)if(tokens[at].kind==='macro'){
  const open=at+1,close=matching.get(open);assert.notEqual(close,undefined);
  for(let index=at;index<=close;index+=1)opaque.add(index);at=close;
 }
 return {tokens,comments,matching,opaque};
}

function simplifyGuardedArrayStringConstructors(source){
 const {tokens,comments,opaque}=readGuardedArrayStyleSyntax(source);
 const edits=[],pattern=['||','String','::','new','(',')'];
 for(let at=0;at+pattern.length<tokens.length;at+=1){
  if(opaque.has(at)||!pattern.every((value,index)=>tokens[at+index].value===value))continue;
  if(tokens[at-3]?.value!=='.'||tokens[at-2]?.value!=='unwrap_or_else'||tokens[at-1]?.value!=='('||tokens[at+6]?.value!==')')continue;
  const start=tokens[at-2].start,end=tokens[at+6].end;
  if(comments.some(comment=>comment.start>=start&&comment.end<=end))continue;
  edits.push({start,end});at+=5;
 }
 for(const {start,end}of edits.reverse())source=source.slice(0,start)+'unwrap_or_default()'+source.slice(end);
 return source;
}

function simplifyGuardedArrayStringComparisons(source,stringFunctions){
 const {tokens,comments,matching,opaque}=readGuardedArrayStyleSyntax(source),edits=[];
 for(let at=1;at+1<tokens.length;at+=1){
  if(opaque.has(at)||!['==','!='].includes(tokens[at].value)||tokens[at+1].kind!=='string'||tokens[at+1].value!=='')continue;
  const end=at-1;if(tokens[end].value!==')')continue;
  const start=matching.get(end);let first=start,last=end;
  while(tokens[first]?.value==='('&&matching.get(first)===last){first+=1;last-=1;}
  if(tokens[first]?.value!=='self'||tokens[first+1]?.value!=='::'||!stringFunctions.has(tokens[first+2]?.value)||tokens[first+3]?.value!=='('||matching.get(first+3)!==last)continue;
  if(comments.some(comment=>comment.start>=tokens[start].start&&comment.end<=tokens[at+1].end))continue;
  const receiver=source.slice(tokens[start].start,tokens[end].end);
  edits.push({start:tokens[start].start,end:tokens[at+1].end,to:(tokens[at].value==='!='?'!':'')+receiver+'.is_empty()'});at+=1;
 }
 for(const {start,end,to}of edits.reverse())source=source.slice(0,start)+to+source.slice(end);
 return source;
}

export function readGuardedPrivateArrayWriteLibraryStyles(source){
 assert.doesNotMatch(source,/^(?:pub\s+)?(?:struct|enum|type|trait)\s+String\b/mu,'the checked planner does not replace the standard String type');
 let adapted=source;const actions=[];
 for(const [,name]of [...source.matchAll(/^pub fn (\w+)\(/gmu)]){
  const from=readSourceTextRustBlock(adapted,'fn',name),to=simplifyGuardedArrayStringConstructors(from);
  if(from!==to){assert.equal(adapted.split(from).length,2);actions.push({op:'replace',field:'source',from,to});adapted=adapted.replace(from,()=>to);}
 }
 // Runtime spellings exposed these empty literals after the original lint
 // pass. Correct only the proved calls; retain its original annotations.
 const stringFunctions=new Set([...adapted.matchAll(/^pub fn (\w+)\([^)]*\)\s*->\s*String\s*\{/gmu)].map(match=>match[1]));
 for(const [,name]of [...adapted.matchAll(/^pub fn (\w+)\(/gmu)]){
  const from=readSourceTextRustBlock(adapted,'fn',name),to=simplifyGuardedArrayStringComparisons(from,stringFunctions);
  if(from!==to){assert.equal(adapted.split(from).length,2);actions.push({op:'replace',field:'source',from,to});adapted=adapted.replace(from,()=>to);}
 }
 return {source:adapted,actions};
}
