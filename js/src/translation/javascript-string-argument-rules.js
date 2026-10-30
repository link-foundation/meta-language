// A plan is actual source argument coordinates, not evaluated argument values.
// The empty call has its own sentinel because String() differs from String(undefined).
/** @param {number} count @returns {number[]} */
export function planJavaScriptStringArguments(count){
 if(!Number.isSafeInteger(count)||count<0||count>4096||count===0&&1/count<0)return [];
 if(count===0)return [-1];
 const plan=[];
 for(let index=0;index<count;index+=1)plan.push(index);
 return plan;
}
