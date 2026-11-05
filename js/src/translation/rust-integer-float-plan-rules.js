/** Closed source AST vocabulary; each term occupies five numeric cells.
 * @param {number} code @returns {string} */
export function readRustIntegerFloatPlanOperation(code){
 if(code===0)return 'input';
 if(code===1)return 'fixed zero';
 if(code===2)return 'fixed two';
 if(code===3)return 'floating zero';
 if(code===4)return 'floating one';
 if(code===5)return 'floating two';
 if(code===6)return 'fixed reference';
 if(code===7)return 'floating reference';
 if(code===8)return 'equal';
 if(code===9)return 'less';
 if(code===10)return 'remainder';
 if(code===11)return 'division';
 if(code===12)return 'negation';
 if(code===13)return 'addition';
 if(code===14)return 'multiplication';
 if(code===15)return 'conditional';
 return code===16?'binding':'';
}
/** Exact decomposition of the six standard integer-to-f64 From domains.
 * Signed remainders retain their sign; no signed minimum is negated. Every
 * quotient divides only by positive two. The floating sum is an exact integer
 * of magnitude below 2^32, and every weight is an exact power of two.
 * Bindings give each actual input and every intermediate one source evaluation.
 * No host integer-to-floating cast is assumed by any instruction.
 * @param {number} bits @returns {number[]} */
export function planRustIntegerFloatConversion(bits){
 if(bits!==8&&bits!==16&&bits!==32)return [];
 let plan=[0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,3,0,0,0,0,4,0,0,0,0,5,0,0,0,0];
 let bindings=[0,0,1,3,2,4],current=0,sum=1,weight=2,index=6;
 for(let bit=0;bit<bits;bit+=1){
  const at=index,nextCurrent=3+bit*3,nextSum=4+bit*3,nextWeight=5+bit*3;
  plan=plan.concat([
   6,current,0,0,0,7,sum,0,0,0,7,weight,0,0,0,
   10,at,2,0,0,8,at+3,1,0,0,9,at,1,0,0,
   12,at+2,0,0,0,15,at+5,at+6,at+2,0,
   15,at+4,3,at+7,0,13,at+1,at+8,0,0,
   11,at,2,0,0,14,at+2,5,0,0,
  ]);
  bindings=bindings.concat([nextSum,at+9,nextCurrent,at+10,nextWeight,at+11]);
  current=nextCurrent;sum=nextSum;weight=nextWeight;index+=12;
 }
 plan=plan.concat([7,sum,0,0,0]);let body=index;index+=1;
 for(let at=bindings.length-2;at>=0;at-=2){
  plan=plan.concat([16,bindings[at]??-1,bindings[at+1]??-1,body,0]);body=index;index+=1;
 }
 return plan;
}
