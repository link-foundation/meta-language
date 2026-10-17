// Generated from the experimental exact original-link identity policy. Do not edit.
/** @param {number} value @returns {boolean} */
export function acceptOriginalLinkIdentityWord(value){return Number.isFinite(value)&&value>=0&&value<=4294967295&&value%1===0;}
/** @param {Array<number>} high @param {Array<number>} low @returns {boolean} */
function checkOriginalLinkIdentityIndex(high,low){
 if(high.length!==low.length||high.length>100000)return false;
 for(let index=0;index<high.length;index++){
  if(!acceptOriginalLinkIdentityWord(high[index]??-1)||!acceptOriginalLinkIdentityWord(low[index]??-1))return false;
  if(index>0&&!(high[index-1]<high[index]||(high[index-1]===high[index]&&low[index-1]<low[index])))return false;
 }
 return true;
}
/** The caller owns the checked immutable index. @param {Array<number>} high @param {Array<number>} low @param {number} queryHigh @param {number} queryLow @returns {number} */
function findOriginalLinkIdentityOrdinal(high,low,queryHigh,queryLow){
 let lower=0;let upper=high.length;
 while(lower<upper){
  const middle=Math.floor((lower+upper)/2);
  if(high[middle]<queryHigh||(high[middle]===queryHigh&&low[middle]<queryLow))lower=middle+1;else upper=middle;
 }
 if(lower>=high.length)return 0;
 return high[lower]===queryHigh&&low[lower]===queryLow?lower+1:0;
}
/** @param {Array<number>} high @param {Array<number>} low @param {Array<number>} queryHigh @param {Array<number>} queryLow @returns {Array<number>} */
export function readOriginalLinkIdentityOrdinals(high,low,queryHigh,queryLow){
 if(!checkOriginalLinkIdentityIndex(high,low))return [-1];
 if(queryHigh.length!==queryLow.length||queryHigh.length>100000)return [-2];
 for(let index=0;index<queryHigh.length;index++){
  if(!acceptOriginalLinkIdentityWord(queryHigh[index]??-1)||!acceptOriginalLinkIdentityWord(queryLow[index]??-1))return [-3,index];
 }
 const result=queryHigh.map((value,index)=>findOriginalLinkIdentityOrdinal(high,low,value,queryLow[index]??-1));
 for(let index=0;index<result.length;index++)if(result[index]===0)return [-4,index];
 return result;
}
