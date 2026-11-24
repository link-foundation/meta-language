// The complete stable algorithm inspects only UTF16 keys. Opaque payloads keep
// their declared wrapper and are never compared, coerced or unwrapped.
/** @typedef {{$:'opaqueValue',value:number}} OpaqueValue */
/** @typedef {{$:'keyedValue',key:string,value:OpaqueValue}} KeyedValue */
/** Ordinary JavaScript lexicographic UTF16 order, including prefixes and lone
 * surrogate units. The ordinary checker retains its shared unicodeCodeUnit
 * string comparison operation for every target. On valid Rust owned String
 * inputs this matches encode_utf16().cmp().
 * @param {string} first @param {string} second @returns {number} */
export function compareStableVectorKeys(first,second){
 if(first<second)return -1;
 if(first>second)return 1;
 return 0;
}
/** Stable insertion of complete key/payload entries. Strictly smaller keys
 * insert before an existing entry; equal keys retain all prior entry order.
 * No represented comparator callback or host sorting implementation is used.
 * @param {KeyedValue[]} values @returns {KeyedValue[]} */
export function sortWrappedStableVectorValues(values){
 let result=[];
 for(const value of values){
  let next=[];let inserted=false;
  for(const previous of result){
   if(!inserted&&compareStableVectorKeys(value.key,previous.key)<0){next=next.concat([value]);inserted=true;}
   next=next.concat([previous]);
  }
  if(!inserted)next=next.concat([value]);
  result=next;
 }
 return result;
}
