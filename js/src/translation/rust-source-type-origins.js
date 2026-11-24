// Structural transport only. Callers retain their existing pre-copy budgets;
// datatype factories pass their already copied origins to the in-place walk.
import {readRustSourceTypeOriginRuntimeFieldPlan} from './rust-source-type-origin-rules.js';
export function sanitizeRustSourceTypeOrigin(origin){
 if(origin==null)return null;
 const pending=[origin],seen=new Set(),plan=readRustSourceTypeOriginRuntimeFieldPlan();
 while(pending.length){
  const value=pending.pop();if(seen.has(value))continue;seen.add(value);
  for(let at=0;at<plan.length;at+=2){
   const field=plan[at],operation=plan[at+1];
   if(operation==='delete')delete value[field];
   else if(operation==='empty list')value[field]=[];
   else if(operation==='false'||operation==='absent fresh array')value[field]=false;
   else throw new TypeError('the shared source type origin clearing plan has an unknown transport operation');
  }
  pending.push(...(value.fields??[]),...(value.callableParameters??[]),...(value.standardSumSource?.arguments??[]));
  if(value.callableResult)pending.push(value.callableResult);
 }
 return origin;
}
export function cloneRustSourceTypeOrigin(origin){return origin==null?null:sanitizeRustSourceTypeOrigin(structuredClone(origin));}
