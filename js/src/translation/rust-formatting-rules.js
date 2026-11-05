// Plain Rust Display formatting over actual UTF16 literal units. Hosts only
// transport real text/value/name terms; grammar and ordering are source owned.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
/** @param {number} unit @returns {boolean} */
function acceptRustFormattingWhitespace(unit){return unit===133||unit!==65279&&acceptJavaScriptWhitespaceUnit(unit);}

/** Topologically ordered raw-operand captures, before any Display operation.
 * @param {number} count @returns {number[]} */
export function planRustFormattingOperandCapture(count){
 if(!Number.isSafeInteger(count)||count<0)return [];
 const result=[];for(let index=0;index<count;index+=1)result.push(index);return result;
}

/** Merge actual used positions without dropping any formatting occurrence.
 * This set projection is used only to diagnose unused arguments.
 * @param {number[]} values @returns {number[]} */
function sortRustFormattingPositions(values){
 if(values.length<=1)return values;
 const middle=Math.floor(values.length/2);
 const first=sortRustFormattingPositions(values.slice(0,middle));
 const second=sortRustFormattingPositions(values.slice(middle));
 const result=[];
 let left=0,right=0;
 while(left<first.length||right<second.length){
  const one=first[left]?? -1,two=second[right]?? -1;
  if(left>=first.length){result.push(two);right+=1;}
  else if(right>=second.length){result.push(one);left+=1;}
  else if(one<two){result.push(one);left+=1;}
  else if(one>two){result.push(two);right+=1;}
  else{result.push(one);left+=1;right+=1;}
 }
 return result;
}

/** Success [0,...triples]: 0 literal[start,end],1 actual positional[index,0],
 * 2 actual captured source identifier[start,end]. Named admission/normalization
 * uses the same actual source identifier grammar as ordinary bindings.
 * Errors [code,start,end]: -1 unmatched close,-2 unclosed open,-3 format spec,
 * -5 missing implicit,-6 missing explicit,-7 unused operand,-8 malformed facts.
 * Explicit positions never advance the independent implicit cursor.
 * @param {number[]} units @param {number} count @returns {number[]} */
export function planRustFormattingFieldTerms(units,count){
 if(!Number.isSafeInteger(count)||count<0)return [-8,0,0];
 for(let index=0;index<units.length;index+=1){const unit=units[index]?? -1;if(!Number.isSafeInteger(unit)||unit<0||unit>65535)return [-8,index,index+1];}
 const plan=[0];
 let at=0,textStart=0,next=0;
 while(at<units.length){
  const unit=units[at]??0,following=units[at+1]?? -1;
  if((unit===123&&following===123)||(unit===125&&following===125)){
   if(textStart<at){plan.push(0);plan.push(textStart);plan.push(at);}
   plan.push(0);plan.push(at);plan.push(at+1);at+=2;textStart=at;continue;
  }
  if(unit===125)return [-1,at,at+1];
  if(unit!==123){at+=1;continue;}
  if(textStart<at){plan.push(0);plan.push(textStart);plan.push(at);}
  let end=at+1;while(end<units.length&&units[end]!==125)end+=1;
  if(end>=units.length)return [-2,at,units.length];
  for(let index=at+1;index<end;index+=1)if(units[index]===58)return [-3,at+1,end];
  let last=end;while(last>at+1&&acceptRustFormattingWhitespace(units[last-1]??0))last-=1;
  if(last===at+1){
   if(next>=count)return [-5,at+1,last];
   plan.push(1);plan.push(next);plan.push(0);next+=1;
  }else{
   let numeric=true,position=0;
   for(let index=at+1;index<last;index+=1){const digit=(units[index]??0)-48;
    if(digit<0||digit>9){numeric=false;break;}position=Math.min(count+1,position*10+digit);
   }
   if(numeric){
    if(position>=count)return [-6,at+1,last];
    plan.push(1);plan.push(position);plan.push(0);
   }else{plan.push(2);plan.push(at+1);plan.push(last);}
  }
  at=end+1;textStart=at;
 }
 if(textStart<units.length||plan.length===1){plan.push(0);plan.push(textStart);plan.push(units.length);}
 return plan;
}

/** Preserve the original complete positional/captured-variable interpretation.
 * @param {number[]} units @param {number} count @returns {number[]} */
export function planRustFormattingValue(units,count){
 const plan=planRustFormattingFieldTerms(units,count);
 if(plan[0]!==0)return plan;
 const used=[];
 for(let at=1;at<plan.length;at+=3)if(plan[at]===1)used.push(plan[at+1]?? -1);
 return readRustFormattingUnusedPosition(used,count,plan);
}

/** Shared occurrence-set diagnosis, retaining every actual repeated operand.
 * @param {number[]} used @param {number} count @param {number[]} plan
 * @returns {number[]} */
export function readRustFormattingUnusedPosition(used,count,plan){
 const ordered=sortRustFormattingPositions(used);
 let expected=0;
 for(let index=0;index<ordered.length;index+=1){
  if(ordered[index]!==expected)return [-7,expected,expected+1];
  expected+=1;
 }
 if(expected<count)return [-7,expected,expected+1];
 return plan;
}
