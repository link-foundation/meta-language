import {readRequiredTextFact,readRequiredNumberFact} from './fact-column-rules.js';
/** @param {string[]} kinds @param {string[]} values @param {number} position @returns {number} */
function groupAfter(kinds,values,position){
 const open=readRequiredTextFact(values,position);let stack=[open==='{'?'}':open==='['?']':')'];
 for(let at=position+1;at<values.length;at+=1){
  if(readRequiredTextFact(kinds,at)!=='punct')continue;
  const value=readRequiredTextFact(values,at);
  if(value==='{'||value==='['||value==='(')stack=[...stack,value==='{'?'}':value==='['?']':')'];
  else if(value==='}'||value===']'||value===')'){
   if(value!==readRequiredTextFact(stack,stack.length-1))return -1;
   stack=stack.slice(0,-1);if(stack.length===0)return at+1;
  }
 }
 return -1;
}
/** Actual alias declarations, independent of declaration order. Five text
 * fields per row: qualified name, keyword, type start, type end, after item.
 * Only module scopes participate; bodies and macro token trees stay opaque.
 * The first field distinguishes a successful declaration table from an error.
 * @param {string[]} kinds @param {string[]} values @returns {string[]} */
export function readRustTypeAliasDeclarationPlan(kinds,values){
 if(kinds.length!==values.length)return ['error','unaligned source type alias columns'];
 const result=['aliases'];let names=[],owners=[],closes=[],position=0;
 while(position<values.length){
  const kind=readRequiredTextFact(kinds,position),value=readRequiredTextFact(values,position);
  if(closes.length>0&&position===readRequiredNumberFact(closes,closes.length-1)){closes=closes.slice(0,-1);owners=owners.slice(0,-1);position+=1;continue;}
  if(kind==='keyword'&&value==='mod'&&position+2<values.length&&readRequiredTextFact(kinds,position+1)==='identifier'&&readRequiredTextFact(values,position+2)==='{'){
   const after=groupAfter(kinds,values,position+2);if(after<0)return result;
   owners=[...owners,readRequiredTextFact(values,position+1)];closes=[...closes,after-1];position+=3;continue;
  }
  if(kind==='keyword'&&value==='type'){
   if(position+3>=values.length||readRequiredTextFact(kinds,position+1)!=='identifier'||readRequiredTextFact(values,position+2)!=='=')return ['error','generic, constrained or malformed type aliases require their complete parameter interpretation'];
   const name=[...owners,readRequiredTextFact(values,position+1)].join('.');if(names.includes(name))return ['error','duplicate source type alias'];names=[...names,name];
   let end=position+3;
   while(end<values.length){
    const boundary=readRequiredTextFact(values,end);
    if(readRequiredTextFact(kinds,end)==='punct'&&boundary===';')break;
    if(readRequiredTextFact(kinds,end)==='punct'&&(boundary==='{'||boundary==='['||boundary==='(')){const after=groupAfter(kinds,values,end);if(after<0)return ['error','unclosed source type alias group'];end=after;}
    else if(readRequiredTextFact(kinds,end)==='eof'||readRequiredTextFact(kinds,end)==='punct'&&boundary==='}')return ['error','unclosed source type alias declaration'];
    else end+=1;
   }
   if(end>=values.length||end===position+3)return ['error','empty or unclosed source type alias declaration'];
   result.push(name,String(position),String(position+3),String(end),String(end+1));
   if(names.length>10000)return ['error','source type alias declaration limit exceeded'];
   position=end+1;continue;
  }
  if(kind==='punct'&&(value==='{'||value==='['||value==='(')){const after=groupAfter(kinds,values,position);if(after<0)return result;position=after;continue;}
  position+=1;
 }
 return result;
}
/** The same ordinary lexical lookup order for complete source alias owners.
 * Imports have already been resolved by the actual frontend namespace.
 * @param {string[]} segments @param {string[]} context @returns {string[]} */
export function readRustTypeAliasLookupKeys(segments,context){
 let names=segments,start=context,absolute=false;
 if((names[0]??'')==='crate'){names=names.slice(1);start=[];absolute=true;}
 else if((names[0]??'')==='self'){names=names.slice(1);absolute=true;}
 else if((names[0]??'')==='super'){
  let count=0;while((names[count]??'')==='super')count+=1;
  if(count>start.length)return [];
  names=names.slice(count);start=start.slice(0,start.length-count);absolute=true;
 }
 if(names.length===0)return [];
 const result=[];
 for(let depth=start.length;depth>=0;depth-=1){result.push([...start.slice(0,depth),...names].join('.'));if(absolute)break;}
 return result;
}
/** @param {number} depth @param {number} expansions @param {boolean} recursive @returns {string} */
export function readRustTypeAliasExpansionError(depth,expansions,recursive){
 return recursive?'recursive source type alias':depth>=128?'source type alias depth limit exceeded':expansions>=4096?'source type alias expansion limit exceeded':'';
}
/** Expanded type and source-origin nodes, counted before copying. Sharing in
 * the alias source is not permission to allocate an exponential type tree.
 * @param {number} current @param {number} needed @returns {string} */
export function readRustTypeAliasCloneError(current,needed){return needed>262144-current?'source type alias materialization limit exceeded':'';}
