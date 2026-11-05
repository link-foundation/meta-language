// Bounded UTF16 expansion. Negative plans distinguish language errors from
// resource refusal; constructors consume the complete already admitted length.
/** @param {number} length @param {number} count @returns {number} */
export function readUtf16TextRepetitionLength(length,count){
 const integer=Number.isNaN(count)?0:Math.trunc(count);
 if(integer<0||integer===Infinity)return -1;
 if(!Number.isSafeInteger(length)||length<0||length>1048576)return -2;
 if(length===0||integer===0)return 0;
 if(integer>Math.floor(1048576/length))return -2;
 return length*integer;
}
/** @param {number} length @param {number} maximum @param {number} fillerLength @returns {number} */
export function readUtf16TextPaddingLength(length,maximum,fillerLength){
 if(!Number.isSafeInteger(length)||length<0||length>1048576||!Number.isSafeInteger(fillerLength)||fillerLength<0||fillerLength>1048576)return -2;
 const integer=Number.isNaN(maximum)?0:Math.trunc(maximum);
 const target=Math.min(9007199254740991,Math.max(0,integer));
 if(target<=length||fillerLength===0)return length;
 return target>1048576?-2:target;
}
/** @param {number[]} text @param {number} length @returns {number[]} */
export function constructUtf16TextRepetition(text,length){
 if(!Number.isSafeInteger(length)||length<0||length>1048576||length>0&&(text.length===0||length%text.length!==0))throw new Error('text repetition requires its complete admitted length');
 let result=[];
 // Registered semantic text is a complete dense unit vector. The admitted
 // nonzero length proves each remainder is inside that vector.
 for(let index=0;index<length;index+=1)result.push(text[index%text.length]??0);
 return result;
}
/** @param {number[]} text @param {number[]} filler @param {number} length @param {boolean} start @returns {number[]} */
export function constructUtf16TextPadding(text,filler,length,start){
 if(!Number.isSafeInteger(length)||length<text.length||length>1048576||length>text.length&&filler.length===0)throw new Error('text padding requires its complete admitted length');
 const added=length-text.length;
 let result=[];
 for(let index=0;index<length;index+=1){
  const original=start?index>=added:index<text.length;
  const position=original?(start?index-added:index):(start?index:index-text.length);
  // Dense registered units and the admitted added length bound both indices.
  result.push((original?text[position]:filler[position%filler.length])??0);
 }
 return result;
}
