// Complete recursive syntax and owner edges. Every position names an actual
// token; row identities name intermediate values, never invented source tokens.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
import {acceptJavaScriptSourceBindingName} from './source-identifier-rules.js';
import {findJavaScriptPatternDefaultExpressionEnd} from './array-formal-parameter-rules.js';
/** @returns {number} */
export function readJavaScriptObjectBindingDepthLimit(){return 256;}
/** Each row has eight cells: role, owner row, property token, binding token,
 * initializer first/end, nested opening/closing. Role zero is the root,
 * one selects a property, two copies the final rest. Nested rows retain -1
 * as their binding; their selected row owns all child rows.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {number} depth @param {number} maximumDepth
 * @returns {number[]} */
function readJavaScriptObjectBindingTree(kinds,values,start,reserved,depth,maximumDepth){
 if(depth>=maximumDepth)return [-9,start];
 if(start>=kinds.length||readRequiredTextFact(kinds,start)!=='punct'||readRequiredTextFact(values,start)!=='{')return [-1,start];
 let at=start+1,rows=[0,-1,start,-1,-1,-1,start,-1],properties=0;
 while(at<kinds.length&&!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='}')){
  const property=at;
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='...'){
   const binding=at+1;if(binding>=kinds.length)return [-1,binding];
   if(!acceptJavaScriptSourceBindingName(readRequiredTextFact(kinds,binding),readRequiredTextFact(values,binding),false,reserved))return [-8,binding];
   at=binding+1;if(at>=kinds.length||readRequiredTextFact(kinds,at)!=='punct'||readRequiredTextFact(values,at)!=='}')return [-6,at];
   rows=rows.concat([2,0,property,binding,-1,-1,-1,-1]);properties+=1;break;
  }
  if(readRequiredTextFact(kinds,at)!=='identifier')return [-7,at];
  at+=1;if(at>=kinds.length)return [-1,at];
  let binding=property,open=-1,close=-1,children=[];
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===':'){
   at+=1;if(at>=kinds.length)return [-1,at];
   if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='{'){
    open=at;children=readJavaScriptObjectBindingTree(kinds,values,at,reserved,depth+1,maximumDepth);
    if(readRequiredNumberFact(children,0)<0)return children;
    at=readRequiredNumberFact(children,0);close=at-1;binding=-1;
   }else{
    if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='[')return [-5,at];
    binding=at;at+=1;
   }
  }
  if(binding>=0&&!acceptJavaScriptSourceBindingName(readRequiredTextFact(kinds,binding),readRequiredTextFact(values,binding),false,reserved))return [-8,binding];
  if(at>=kinds.length)return [-1,at];let first=-1,last=-1;
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='='){
   first=at+1;last=findJavaScriptPatternDefaultExpressionEnd(kinds,values,first,'}');
   if(last<0)return [-1,first];at=last;
  }
  const owner=rows.length/8;
  rows=rows.concat([1,0,property,binding,first,last,open,close]);properties+=1;
  if(children.length){
   const count=readRequiredNumberFact(children,1),base=rows.length/8;
   for(let index=1;index<count;index+=1){
    const offset=2+index*8,old=readRequiredNumberFact(children,offset+1),parent=old===0?owner:base+old-1;
    rows=rows.concat([readRequiredNumberFact(children,offset),parent],children.slice(offset+2,offset+8));
   }
  }
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===','){at+=1;continue;}
  if(!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='}'))return [-1,at];
 }
 if(!properties)return [-4,start];
 if(at>=kinds.length||readRequiredTextFact(kinds,at)!=='punct'||readRequiredTextFact(values,at)!=='}')return [-1,at];
 rows=rows.slice(0,7).concat([at],rows.slice(8));
 return [at+1,rows.length/8].concat(rows);
}
/** @param {number[]} plan @returns {number[]} */
export function readJavaScriptRecursiveObjectBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return [];
 let result=[];
 for(let index=0;index<readRequiredNumberFact(plan,1);index+=1){const binding=readRequiredNumberFact(plan,2+index*8+3);if(binding>=0)result=result.concat([binding]);}
 return result;
}
/** @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {string[]} previous @param {number} maximumDepth
 * @returns {number[]} */
export function planJavaScriptRecursiveObjectBindings(kinds,values,start,reserved,previous,maximumDepth){
 if(kinds.length!==values.length||!Number.isInteger(start)||start<0||start>=kinds.length||!Number.isSafeInteger(maximumDepth)||maximumDepth<1)return [-1,start];
 const plan=readJavaScriptObjectBindingTree(kinds,values,start,reserved,0,maximumDepth);
 if(readRequiredNumberFact(plan,0)<0)return plan;
 const positions=readJavaScriptRecursiveObjectBindingPositions(plan);let names=previous.slice();
 for(let index=0;index<positions.length;index+=1){const position=readRequiredNumberFact(positions,index),name=readRequiredTextFact(values,position);if(!acceptJavaScriptSourceBindingName(readRequiredTextFact(kinds,position),name,names.includes(name),reserved))return [-8,position];names=names.concat([name]);}
 return plan;
}
/** @param {number[]} plan @param {number} owner @returns {number[]} */
export function readJavaScriptRecursiveObjectExcludedPositions(plan,owner){
 let result=[];
 for(let index=0;index<readRequiredNumberFact(plan,1);index+=1){const at=2+index*8;if(readRequiredNumberFact(plan,at)===1&&readRequiredNumberFact(plan,at+1)===owner)result=result.concat([readRequiredNumberFact(plan,at+2)]);}
 return result;
}
/** A nested default precedes the first actual binding in its subtree.
 * @param {number[]} plan @param {number} row @returns {number} */
export function readJavaScriptRecursiveObjectInitializerBinding(plan,row){
 const at=2+row*8,binding=readRequiredNumberFact(plan,at+3);if(binding>=0)return binding;
 const open=readRequiredNumberFact(plan,at+6),close=readRequiredNumberFact(plan,at+7),positions=readJavaScriptRecursiveObjectBindingPositions(plan);
 for(let index=0;index<positions.length;index+=1){const position=readRequiredNumberFact(positions,index);if(position>open&&position<close)return position;}
 throw new TypeError('a recursive object initializer needs its actual first descendant binding');
}
/** @param {number[]} plan @returns {boolean} */
export function containsJavaScriptRecursiveObjectBinding(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return false;
 for(let index=1;index<readRequiredNumberFact(plan,1);index+=1)if(readRequiredNumberFact(plan,2+index*8+6)>=0)return true;
 return false;
}
/** Ordinary slots remain actual binders; pattern slots carry their complete
 * recursive plan as [1,slot,length,...tree]. No logical argument is invented.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {number} maximumDepth @returns {number[]} */
export function planJavaScriptRecursiveObjectFormals(kinds,values,start,reserved,maximumDepth){
 if(kinds.length!==values.length||!Number.isInteger(start)||start<0||start>=kinds.length)return [-1,start];
 let at=start,slot=0,names=[],instructions=[];
 while(at<kinds.length&&!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===')')){
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='{'){
   const tree=planJavaScriptRecursiveObjectBindings(kinds,values,at,reserved,names,maximumDepth);if(readRequiredNumberFact(tree,0)<0)return tree;
   const positions=readJavaScriptRecursiveObjectBindingPositions(tree);
   for(let index=0;index<positions.length;index+=1)names=names.concat([readRequiredTextFact(values,readRequiredNumberFact(positions,index))]);
   instructions=instructions.concat([1,slot,tree.length],tree);at=readRequiredNumberFact(tree,0);
  }else{
   if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='[')return [-3,at];
   if(!acceptJavaScriptSourceBindingName(readRequiredTextFact(kinds,at),readRequiredTextFact(values,at),names.includes(readRequiredTextFact(values,at)),reserved))return [-8,at];
   names=names.concat([readRequiredTextFact(values,at)]);instructions=instructions.concat([0,slot,at]);at+=1;
  }
  if(at>=kinds.length)return [-1,at];
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='=')return [-2,at];
  slot+=1;if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===','){at+=1;continue;}
  if(!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===')'))return [-1,at];
 }
 if(at>=kinds.length)return [-1,at];return [at+1,slot].concat(instructions);
}
/** @param {number[]} plan @returns {number[]} */
export function readJavaScriptRecursiveObjectFormalBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return [];
 let at=2,result=[];
 while(at<plan.length){const form=readRequiredNumberFact(plan,at);if(form===0){result=result.concat([readRequiredNumberFact(plan,at+2)]);at+=3;}else{const count=readRequiredNumberFact(plan,at+2),tree=plan.slice(at+3,at+3+count);result=result.concat(readJavaScriptRecursiveObjectBindingPositions(tree));at+=3+count;}}
 return result;
}
/** @param {number[]} plan @returns {boolean} */
export function containsJavaScriptRecursiveObjectFormals(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)return false;
 let at=2;
 while(at<plan.length){if(readRequiredNumberFact(plan,at)===0){at+=3;continue;}const count=readRequiredNumberFact(plan,at+2);if(containsJavaScriptRecursiveObjectBinding(plan.slice(at+3,at+3+count)))return true;at+=3+count;}
 return false;
}
