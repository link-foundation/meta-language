import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @returns {boolean} */
export function acceptRustIntegerBitMethod(method){return ['count_ones','count_zeros','leading_ones','leading_zeros','trailing_ones','trailing_zeros','reverse_bits'].includes(method);}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustIntegerBitReceiver(kind,bits,signed,owner,path,children,proven,layers){return kind==='fixed'&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** @param {string} method @param {string} receiver @returns {string} */
export function readRustIntegerBitResultOwner(method,receiver){return !acceptRustIntegerBitMethod(method)?'':method==='reverse_bits'?receiver:'u32';}
/** Five numeric cells per existing source term. Coordinate values are source
 * metadata, never conversions of represented integer operands.
 * @param {number} code @returns {string} */
export function readRustIntegerBitPlanOperation(code){
 return code===0?'unsigned input':code===1?'fixed zero':code===2?'fixed one':code===3?'fixed two':code===4?'count zero':code===5?'count one':code===6?'false':code===7?'true':code===8?'fixed reference':code===9?'count reference':code===10?'boolean reference':code===11?'equal':code===12?'remainder':code===13?'division':code===14?'count addition':code===15?'fixed addition':code===16?'fixed multiplication':code===17?'conditional':code===18?'binding':code===19?'source cast':'';
}
/** A complete finite source recurrence over the actual unsigned bit pattern.
 * Fixed division/remainder use positive two. Counts fit u32. Reversal prefixes
 * never exceed the original width; no represented bit value enters binary64.
 * Leading counts are the final matching run in least-to-most scanning order;
 * trailing counts stop after the first mismatch. Every input/state value has
 * one actual source binding. Hosts only project these terms into existing ASTs.
 * @param {string} method @param {number} bits @returns {number[]} */
export function planRustIntegerBits(method,bits){
 if(!acceptRustIntegerBitMethod(method)||![8,16,32,64,128].includes(bits))return [];
 const reverse=method==='reverse_bits',trailing=method==='trailing_zeros'||method==='trailing_ones',leading=method==='leading_zeros'||method==='leading_ones',ones=method.endsWith('ones');
 let plan=[0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,3,0,0,0,0,4,0,0,0,0,5,0,0,0,0,6,0,0,0,0,7,0,0,0,0];
 let bindings=[0,0,1,reverse?1:4,2,7],current=0,count=1,active=2,index=8;
 for(let bit=0;bit<bits;bit+=1){
  const at=index,nextCurrent=3+bit*3,nextCount=4+bit*3,nextActive=5+bit*3;
  plan=plan.concat([8,current,0,0,0,reverse?8:9,count,0,0,0,10,active,0,0,0,12,at,3,0,0]);index+=4;
  let nextCountTerm=0,nextActiveTerm=7;
  if(reverse){plan=plan.concat([16,at+1,3,0,0,15,at+4,at+3,0,0]);nextCountTerm=at+5;index+=2;}
  else{
   const match=index;plan=plan.concat([11,at+3,ones?2:1,0,0,14,at+1,5,0,0]);index+=2;
   if(trailing){const nextFlag=index;plan=plan.concat([17,at+2,match,6,0,17,nextFlag,match+1,at+1,0]);nextActiveTerm=nextFlag;nextCountTerm=nextFlag+1;index+=2;}
   else{nextCountTerm=index;plan=plan.concat([17,match,match+1,leading?4:at+1,0]);index+=1;}
  }
  const quotient=index;plan=plan.concat([13,at,3,0,0]);index+=1;
  bindings=bindings.concat([nextCurrent,quotient,nextCount,nextCountTerm]);
  if(trailing)bindings=bindings.concat([nextActive,nextActiveTerm]);
  current=nextCurrent;count=nextCount;if(trailing)active=nextActive;
 }
 plan=plan.concat([reverse?8:9,count,0,0,0]);let body=index;index+=1;
 if(reverse){plan=plan.concat([19,body,0,0,0]);body=index;index+=1;}
 for(let at=bindings.length-2;at>=0;at-=2){plan=plan.concat([18,bindings[at]??-1,bindings[at+1]??-1,body,0]);body=index;index+=1;}
 return plan;
}
