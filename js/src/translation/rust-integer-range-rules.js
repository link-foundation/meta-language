import {readRequiredTextFact} from './fact-column-rules.js';
/** @param {string} value @returns {boolean} */
export function testRustIntegerRangeCanonicalDecimal(value){
 let digits=value;if(digits.startsWith('-'))digits=digits.slice(1);
 if(digits===''||digits.length>1&&digits.startsWith('0')||value==='-0')return false;
 for(let at=0;at<digits.length;at+=1)if('0123456789'.indexOf(digits.charAt(at))<0)return false;
 return true;
}
/** Exact canonical decimal order; no host Number conversion.
 * @param {string} left @param {string} right @returns {number} */
export function compareRustIntegerRangeDecimals(left,right){
 if(!testRustIntegerRangeCanonicalDecimal(left)||!testRustIntegerRangeCanonicalDecimal(right))return 2;
 const negativeLeft=left.startsWith('-'),negativeRight=right.startsWith('-');
 if(negativeLeft!==negativeRight)return negativeLeft?-1:1;
 const a=negativeLeft?left.slice(1):left,b=negativeRight?right.slice(1):right;
 let order=a.length<b.length?-1:a.length>b.length?1:0;
 if(order===0){for(let at=0;at<a.length;at+=1){const x='0123456789'.indexOf(a.charAt(at)),y='0123456789'.indexOf(b.charAt(at));if(x<y){order=-1;break;}if(x>y){order=1;break;}}}
 if(order===0)return 0;
 return negativeLeft?-order:order;
}
/** Exact successor, including the boundary after u128::MAX.
 * @param {string} value @returns {string} */
export function readRustIntegerRangeSuccessor(value){
 if(!testRustIntegerRangeCanonicalDecimal(value))return '';
 const negative=value.startsWith('-'),digits=negative?value.slice(1):value;
 let result='',carry=true;
 for(let at=digits.length-1;at>=0;at-=1){
  let digit='0123456789'.indexOf(digits.charAt(at));
  if(carry){if(negative){if(digit===0)digit=9;else{digit-=1;carry=false;}}else if(digit===9)digit=0;else{digit+=1;carry=false;}}
  result='0123456789'.charAt(digit)+result;
 }
 if(negative){if(result.startsWith('0')&&result.length>1)result=result.slice(1);return result==='0'?'0':'-'+result;}
 return carry?'1'+result:result;
}
/** @param {boolean} ordinary @param {string} operation @param {boolean} lower
 * @param {boolean} upper @returns {string} */
export function readRustIntegerRangeSourceError(ordinary,operation,lower,upper){
 if(!ordinary)return 'ranges are outside the portable pattern language';
 if(operation!=='..'&&operation!=='..=')return 'a range pattern requires its actual range operator';
 if(!lower&&!upper)return 'a range pattern needs at least one endpoint';
 if(operation==='..='&&!upper)return 'an inclusive range pattern needs its upper endpoint';
 return '';
}
/** @param {string} lower @param {string} upper @returns {string} */
export function readRustIntegerRangeBoundsError(lower,upper){
 const order=compareRustIntegerRangeDecimals(lower,upper);
 return order===2?'range endpoints require exact canonical integers':order>=0?'a range pattern must contain at least one integer':'';
}
/** Complete ordered half-open integer cells, independent of domain size.
 * @param {string} minimum @param {string} maximum @param {string[]} lowers
 * @param {string[]} uppers @returns {string[]} */
export function readRustIntegerRangeCuts(minimum,maximum,lowers,uppers){
 if(lowers.length!==uppers.length||readRustIntegerRangeBoundsError(minimum,maximum)!=='')return [];
 for(let row=0;row<lowers.length;row+=1){const lower=readRequiredTextFact(lowers,row),upper=readRequiredTextFact(uppers,row);if(lower===''&&upper==='')continue;if(readRustIntegerRangeBoundsError(lower,upper)!==''||compareRustIntegerRangeDecimals(lower,minimum)<0||compareRustIntegerRangeDecimals(upper,maximum)>0)return [];}
 const result=[minimum];let current=minimum;
 while(current!==maximum){
  let next=maximum;
  for(let row=0;row<lowers.length;row+=1){const lower=readRequiredTextFact(lowers,row),upper=readRequiredTextFact(uppers,row);if(lower!==''&&compareRustIntegerRangeDecimals(lower,current)>0&&compareRustIntegerRangeDecimals(lower,next)<0)next=lower;if(upper!==''&&compareRustIntegerRangeDecimals(upper,current)>0&&compareRustIntegerRangeDecimals(upper,next)<0)next=upper;}
  result.push(next);current=next;
 }
 return result;
}
/** Status, cell count, then each cell's count and source row indices.
 * @param {string[]} kinds @param {string[]} lowers @param {string[]} uppers
 * @param {string[]} cuts @returns {number[]} */
export function planRustIntegerRangeRows(kinds,lowers,uppers,cuts){
 if(kinds.length!==lowers.length||kinds.length!==uppers.length||cuts.length<2)return [-1];
 for(let cell=0;cell+1<cuts.length;cell+=1)if(readRustIntegerRangeBoundsError(readRequiredTextFact(cuts,cell),readRequiredTextFact(cuts,cell+1))!=='')return [-1];
 for(let row=0;row<kinds.length;row+=1){
  const kind=readRequiredTextFact(kinds,row),lower=readRequiredTextFact(lowers,row),upper=readRequiredTextFact(uppers,row);
  if(kind==='wild'||kind==='bind'){if(lower!==''||upper!=='')return [-1];}
  else if(kind==='lit'||kind==='interval'){if(readRustIntegerRangeBoundsError(lower,upper)!==''||!cuts.includes(lower)||!cuts.includes(upper)||kind==='lit'&&readRustIntegerRangeSuccessor(lower)!==upper)return [-1];}
  else return [-1];
 }
 const result=[0,cuts.length-1];
 for(let cell=0;cell+1<cuts.length;cell+=1){
  const lower=readRequiredTextFact(cuts,cell),upper=readRequiredTextFact(cuts,cell+1),rows=[];
  for(let row=0;row<kinds.length;row+=1){const kind=readRequiredTextFact(kinds,row);if(kind==='wild'||kind==='bind')rows.push(row);else if(kind==='lit'||kind==='interval'){if(compareRustIntegerRangeDecimals(readRequiredTextFact(lowers,row),lower)<=0&&compareRustIntegerRangeDecimals(readRequiredTextFact(uppers,row),upper)>=0)rows.push(row);}else return [-1];}
  if(rows.length===0)return [-2,cell];
  result.push(rows.length);for(const row of rows)result.push(row);
 }
 return result;
}

/** A rest token is complete only before its product delimiter.
 * @param {string} next @returns {boolean} */
export function testRustIntegerRangeRestFollower(next){return next===','||next===')';}
