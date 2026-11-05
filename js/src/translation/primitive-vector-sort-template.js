// Complete stable insertion source. Keys are compared in their actual domain;
// payload wrappers remain opaque throughout every algorithm.
/** @typedef {{$:'opaqueValue',value:number}} OpaqueValue */
/** @typedef {{$:'integerSortEntry',key:number,value:OpaqueValue}} IntegerSortEntry */
/** @typedef {{$:'booleanSortEntry',key:boolean,value:OpaqueValue}} BooleanSortEntry */
/** @typedef {{$:'stringSortEntry',key:string,value:OpaqueValue}} StringSortEntry */
/** @param {number} first @param {number} second @returns {boolean} */
export function integerVectorKeyPrecedes(first,second){return first<second;}
/** @param {boolean} first @param {boolean} second @returns {boolean} */
export function booleanVectorKeyPrecedes(first,second){return !first&&second;}
/** Rust String Ord compares valid scalar strings in UTF8 byte order. Unicode
 * scalar order is equivalent on that domain. Array.from visits one scalar;
 * codePointAt reads its actual scalar, independently of UTF16 unit ordering.
 * @param {string} first @param {string} second @returns {boolean} */
export function stringVectorKeyPrecedes(first,second){
 const left=Array.from(first).map(character=>character.codePointAt(0)??-1);
 const right=Array.from(second).map(character=>character.codePointAt(0)??-1);
 for(let index=0;index<left.length;index+=1){
  const a=left[index]??-1,b=right[index]??-1;
  if(a<b)return true;
  if(a>b)return false;
 }
 return left.length<right.length;
}
/** @param {IntegerSortEntry[]} values @returns {IntegerSortEntry[]} */
export function sortIntegerVectorEntries(values){
 let result=[];
 for(const value of values){
  let next=[];let inserted=false;
  for(const previous of result){
   if(!inserted&&integerVectorKeyPrecedes(value.key,previous.key)){next=next.concat([value]);inserted=true;}
   next=next.concat([previous]);
  }
  if(!inserted)next=next.concat([value]);
  result=next;
 }
 return result;
}
/** @param {BooleanSortEntry[]} values @returns {BooleanSortEntry[]} */
export function sortBooleanVectorEntries(values){
 let result=[];
 for(const value of values){
  let next=[];let inserted=false;
  for(const previous of result){
   if(!inserted&&booleanVectorKeyPrecedes(value.key,previous.key)){next=next.concat([value]);inserted=true;}
   next=next.concat([previous]);
  }
  if(!inserted)next=next.concat([value]);
  result=next;
 }
 return result;
}
/** @param {StringSortEntry[]} values @returns {StringSortEntry[]} */
export function sortStringVectorEntries(values){
 let result=[];
 for(const value of values){
  let next=[];let inserted=false;
  for(const previous of result){
   if(!inserted&&stringVectorKeyPrecedes(value.key,previous.key)){next=next.concat([value]);inserted=true;}
   next=next.concat([previous]);
  }
  if(!inserted)next=next.concat([value]);
  result=next;
 }
 return result;
}
