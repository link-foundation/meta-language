// Complete source operations on UTF-16 columns. Formal and native runtimes
// are emitted from these definitions; host scalar strings supply no meaning.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
import {readJavaScriptUtf8StringUnits} from './utf8-string-unit-rules.js';
import {normalizeJavaScriptUnsignedInteger} from './unsigned-integer-rules.js';

/** @param {number} value @returns {number} */
export function readUtf16TextInteger(value){return Number.isNaN(value)?0:Math.trunc(value);}
/** @param {number} value @param {number} length @returns {number} */
export function readUtf16TextSlicePosition(value,length){
  const integer=readUtf16TextInteger(value);
  return integer<0?Math.max(length+integer,0):Math.min(integer,length);
}
/** @param {number[]} text @param {number} start @param {number} end @returns {number[]} */
export function sliceUtf16Text(text,start,end){
  const first=readUtf16TextSlicePosition(start,text.length),last=readUtf16TextSlicePosition(end,text.length);
  const result=[];
  for(let at=first;at<last;at+=1)result.push(text[at]??0);
  return result;
}
/** @param {number[]} text @param {number} start @param {number} end @returns {number[]} */
export function substringUtf16Text(text,start,end){
  const first=Math.min(Math.max(readUtf16TextInteger(start),0),text.length);
  const last=Math.min(Math.max(readUtf16TextInteger(end),0),text.length);
  return sliceUtf16Text(text,Math.min(first,last),Math.max(first,last));
}
/** @param {number[]} text @param {number} position @returns {number[]} */
export function characterUtf16Text(text,position){
  const at=readUtf16TextInteger(position);
  return at<0||at>=text.length?[]:[text[at]??0];
}
/** @param {number[]} text @param {number} position @returns {number[][]} */
export function relativeCharacterUtf16Text(text,position){
  const integer=readUtf16TextInteger(position),at=integer<0?text.length+integer:integer;
  return at<0||at>=text.length?[]:[[text[at]??0]];
}
/** @param {number[]} text @param {number} position @returns {number[]} */
export function codePointUtf16Text(text,position){
  const at=readUtf16TextInteger(position);
  if(at<0||at>=text.length)return [];
  const first=text[at]??0,second=text[at+1]?? -1;
  return [first>=55296&&first<=56319&&second>=56320&&second<=57343?65536+(first-55296)*1024+second-56320:first];
}
/** @param {number[]} text @returns {number[][]} */
export function pointsUtf16Text(text){
  const result=[];
  for(let at=0;at<text.length;at+=1){
    const first=text[at]??0,second=text[at+1]?? -1;
    if(first>=55296&&first<=56319&&second>=56320&&second<=57343){result.push([first,second]);at+=1;}
    else result.push([first]);
  }
  return result;
}

/** Literal separator splitting; limit is ToUint32 before missing-separator
 * selection. An empty separator splits UTF16 units, including isolated halves.
 * @param {number[]} text @param {number[]} separator @param {number} limit
 * @param {boolean} missing @returns {number[][]} */
export function splitUtf16Text(text,separator,limit,missing){
 const count=normalizeJavaScriptUnsignedInteger(limit),result=[];
 if(count===0)return result;
 if(missing)return [sliceUtf16Text(text,0,text.length)];
 if(separator.length===0){
  for(let at=0;at<text.length&&result.length<count;at+=1)result.push([text[at]??0]);
  return result;
 }
 let start=0;
 for(;;){
  const found=indexUtf16Text(text,separator,start);
  if(found<0)break;
  result.push(sliceUtf16Text(text,start,found));
  if(result.length===count)return result;
  start=found+separator.length;
 }
 result.push(sliceUtf16Text(text,start,text.length));
 return result;
}
/** @param {number[]} text @param {number[]} search @param {number} start @returns {boolean} */
export function matchUtf16Text(text,search,start){
  if(start<0||start+search.length>text.length)return false;
  for(let at=0;at<search.length;at+=1)if(text[start+at]!==search[at])return false;
  return true;
}
/** @param {number[]} text @param {number[]} search @param {number} position @returns {number} */
export function indexUtf16Text(text,search,position){
  const start=Math.min(Math.max(readUtf16TextInteger(position),0),text.length);
  for(let at=start;at<=text.length-search.length;at+=1)if(matchUtf16Text(text,search,at))return at;
  return -1;
}
/** @param {number[]} text @param {number[]} search @param {number} position @returns {number} */
export function lastIndexUtf16Text(text,search,position){
  const integer=Number.isNaN(position)?Infinity:readUtf16TextInteger(position);
  const start=Math.min(Math.max(integer,0),text.length-search.length);
  for(let at=start;at>=0;at-=1)if(matchUtf16Text(text,search,at))return at;
  return -1;
}
/** @param {number[]} left @param {number[]} right @returns {number} */
export function compareUtf16Text(left,right){
  const count=Math.min(left.length,right.length);
  for(let at=0;at<count;at+=1){const a=left[at]??0,b=right[at]??0;if(a!==b)return a<b?-1:1;}
  return left.length===right.length?0:left.length<right.length?-1:1;
}
/** @param {number[]} text @param {number[]} search @returns {boolean} */
export function startsUtf16Text(text,search){return matchUtf16Text(text,search,0);}
/** @param {number[]} text @param {number[]} search @returns {boolean} */
export function endsUtf16Text(text,search){return matchUtf16Text(text,search,text.length-search.length);}
/** @param {number[]} text @param {number[]} search @returns {boolean} */
export function includesUtf16Text(text,search){return indexUtf16Text(text,search,0)>=0;}
/** @param {number[][]} values @param {number[]} separator @returns {number[]} */
export function joinUtf16Text(values,separator){
 const result=[];
 for(let at=0;at<values.length;at+=1){
  if(at!==0)for(let offset=0;offset<separator.length;offset+=1)result.push(separator[offset]??0);
  const value=values[at]??[];
  for(let offset=0;offset<value.length;offset+=1)result.push(value[offset]??0);
 }
 return result;
}
/** @param {number[]} text @param {boolean} start @param {boolean} end @returns {number[]} */
export function trimUtf16Text(text,start,end){
  let first=0,last=text.length;
  if(start)while(first<last&&acceptJavaScriptWhitespaceUnit(text[first]?? -1))first+=1;
  if(end)while(last>first&&acceptJavaScriptWhitespaceUnit(text[last-1]?? -1))last-=1;
  return sliceUtf16Text(text,first,last);
}
/** ToUint16 applied to already checked Number operands, with no array length
 * or argument count cutoff. @param {number[]} values @returns {number[]} */
export function characterUnitsUtf16Text(values){
  const result=[];
  for(let at=0;at<values.length;at+=1){
    const value=values[at]??0,integer=Number.isFinite(value)?Math.trunc(value):0;
    const residue=integer%65536;
    result.push(residue<0?residue+65536:residue===0?0:residue);
  }
  return result;
}
/** Success starts with -1; failure is the index of the first invalid Number.
 * The success suffix is the actual complete UTF-16 value.
 * @param {number[]} values @returns {number[]} */
export function codePointUnitsUtf16Text(values){
  const result=[-1];
  for(let at=0;at<values.length;at+=1){
    const value=values[at]?? -1;
    if(!Number.isInteger(value)||value<0||value>1114111)return [at];
    if(value<65536)result.push(value===0?0:value);
    else result.push(55296+Math.floor((value-65536)/1024),56320+(value-65536)%1024);
  }
  return result;
}
/** @param {number[]} bytes @returns {number[]} */
export function readUtf8TextUnits(bytes){return readJavaScriptUtf8StringUnits(bytes);}
/** @param {number[]} text @returns {number[]} */
export function displayPointsUtf16Text(text){
 const result=[];
 for(let at=0;at<text.length;at+=1){
  let point=text[at]??65533;
  if(point>=55296&&point<=56319){
   const low=text[at+1]?? -1;
   if(low>=56320&&low<=57343){point=65536+(point-55296)*1024+low-56320;at+=1;}
   else point=65533;
  }else if(point>=56320&&point<=57343)point=65533;
  result.push(point);
 }
 return result;
}
/** Scalar display boundary: replace isolated units, then encode actual UTF-8.
 * Internal strings remain full UTF-16 columns.
 * @param {number[]} text @returns {number[]} */
export function displayUtf8FromUtf16Text(text){
  const bytes=[];
  const points=displayPointsUtf16Text(text);
  for(let at=0;at<points.length;at+=1){
    const point=points[at]??65533;
    if(point<128)bytes.push(point);
    else if(point<2048)bytes.push(192+Math.floor(point/64),128+point%64);
    else if(point<65536)bytes.push(224+Math.floor(point/4096),128+Math.floor(point/64)%64,128+point%64);
    else bytes.push(240+Math.floor(point/262144),128+Math.floor(point/4096)%64,128+Math.floor(point/64)%64,128+point%64);
  }
  return bytes;
}

/** Complete GetSubstitution template plan for a literal search with no captures.
 * Triples select replacement ranges (0), matched text (1), original prefix (2)
 * or original suffix (3). Capture-looking forms remain literal in this domain.
 * @param {number[]} replacement @returns {number[]} */
export function planUtf16TextSubstitution(replacement){
 const result=[];
 let start=0,at=0;
 while(at<replacement.length){
  const next=replacement[at+1]?? -1;
  if((replacement[at]??0)===36&&(next===36||next===38||next===96||next===39)){
   if(start<at)result.push(0,start,at);
   if(next===36)result.push(0,at,at+1);
   else result.push(next===38?1:next===96?2:3,0,0);
   at+=2;start=at;
  }else at+=1;
 }
 if(start<replacement.length)result.push(0,start,replacement.length);
 return result;
}

/** Literal source replacement. Every substitution reads the unchanged original
 * text. Empty all-search advances one code unit, including the final boundary.
 * @param {number[]} text @param {number[]} search @param {number[]} replacement
 * @param {boolean} all @returns {number[]} */
export function replaceUtf16Text(text,search,replacement,all){
 const result=[],plan=planUtf16TextSubstitution(replacement);
 let start=0,from=0,searching=true;
 while(searching){
  const found=indexUtf16Text(text,search,from);
  if(found<0)searching=false;
  else{
   for(let at=start;at<found;at+=1)result.push(text[at]??0);
   for(let instruction=0;instruction<plan.length;instruction+=3){
    const kind=plan[instruction]??0;
    const first=kind===0?(plan[instruction+1]??0):kind===2?0:kind===1?found:found+search.length;
    const last=kind===0?(plan[instruction+2]??0):kind===2?found:kind===1?found+search.length:text.length;
    const source=kind===0?replacement:text;
    for(let at=first;at<last;at+=1)result.push(source[at]??0);
   }
   start=found+search.length;
   searching=all&&!(search.length===0&&found===text.length);
   from=start+(search.length===0?1:0);
  }
 }
 for(let at=start;at<text.length;at+=1)result.push(text[at]??0);
 return result;
}
