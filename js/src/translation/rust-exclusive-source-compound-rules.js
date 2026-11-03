// Built-in primitive compound assignment only. Custom AddAssign dispatch and
// nonprimitive evaluation order retain their separate source obligations.
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
/** @param {string} token @returns {string} */
export function readRustExclusiveSourceCompoundOperation(token){return token==='+='?'add':token==='-='?'sub':token==='*='?'mul':token==='/='?'div':token==='%='?'rem':'';}
/** Ordered syntactic places, before ownership admission. The first two cells
 * retain the end and root token indices; remaining cells are field token
 * indices or -1 for dereference. Postfix fields bind before unary stars.
 * @param {string[]} forms @returns {number[]} */
export function readRustExclusiveSourceCompoundPlace(forms){
 return readRustExclusiveSourcePlacePlan(forms);
}
/** @param {string[]} forms @returns {number[]} */
export function readRustExclusiveSourcePlacePlan(forms){
 if(forms.length>1025)return [];
 const plan=readRustExclusiveSourceCompoundPlaceAt(forms,0,0);
 return plan.length>=2&&forms[plan[0]]==='}'?plan:[];
}
/** @param {string[]} forms @param {number} start @param {number} depth
 * @returns {number[]} */
function readRustExclusiveSourceCompoundPlaceAt(forms,start,depth){
 if(depth>=256||start>=forms.length)return [];
 if(forms[start]==='*'){
  const inner=readRustExclusiveSourceCompoundPlaceAt(forms,start+1,depth+1);
  return inner.length>=2?[...inner,-1]:[];
 }
 let plan=[start+1,start];
 if(forms[start]==='('){
  plan=readRustExclusiveSourceCompoundPlaceAt(forms,start+1,depth+1);
  if(plan.length<2||forms[plan[0]]!==')')return [];
  plan[0]+=1;
 }else if(forms[start]!=='name')return [];
 while(forms[plan[0]]==='.'){
  const field=plan[0]+1;
  if(forms[field]!=='name'&&forms[field]!=='number')return [];
  plan.push(field);plan[0]=field+1;
 }
 return plan;
}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustExclusiveSourceCompoundPrimitive(kind,bits,signed,owner,path,children,proven,layers){return (kind==='fixed'||kind==='float')&&acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,children,proven,layers);}
