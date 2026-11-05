// Unicode 16 default casing, with context evaluated on original UTF-16 input.
import {readUnicodeCaseLowerRanges,readUnicodeCaseLowerIndexes,readUnicodeCaseLowerValues,readUnicodeCaseUpperRanges,readUnicodeCaseUpperIndexes,readUnicodeCaseUpperValues,readUnicodeCaseCasedRanges,readUnicodeCaseIgnorableRanges} from './unicode-case-table-declarations.js';

/** @param {number} point @param {number[]} ranges @returns {boolean} */
export function acceptUnicodeCaseProperty(point,ranges){
 let first=0,last=Math.floor(ranges.length/2);
 while(first<last){const middle=Math.floor((first+last)/2),start=ranges[middle*2]?? -1,end=ranges[middle*2+1]?? -1;
  if(point<start)last=middle;else if(point>end)first=middle+1;else return true;
 }
 return false;
}

/** Sorted range rows [first,last,stride,delta]; expansion rows [point,offset,count].
 * @param {number} point @param {number[]} ranges @param {number[]} indexes @param {number[]} values @returns {number[]} */
export function readUnicodeCaseMapping(point,ranges,indexes,values){
 let first=0,last=Math.floor(indexes.length/3);
 while(first<last){const middle=Math.floor((first+last)/2),key=indexes[middle*3]?? -1;
  if(point<key)last=middle;else if(point>key)first=middle+1;else {
   const offset=indexes[middle*3+1]??0,count=indexes[middle*3+2]??0,result=[];
   for(let at=0;at<count;at+=1)result.push(values[offset+at]??0);
   return result;
  }
 }
 first=0;last=Math.floor(ranges.length/4);
 while(first<last){const middle=Math.floor((first+last)/2),start=ranges[middle*4]?? -1,end=ranges[middle*4+1]?? -1;
  if(point<start)last=middle;else if(point>end)first=middle+1;else {
   const stride=ranges[middle*4+2]??1,delta=ranges[middle*4+3]??0;
   return [(point-start)%stride===0?point+delta:point];
  }
 }
 return [point];
}

/** Tables are data arguments, permitting explicit independent Unicode profiles.
 * Case_Ignorable is possessive and takes precedence over overlapping Cased.
 * @param {number[]} text @param {boolean} upper @param {number[]} ranges
 * @param {number[]} indexes @param {number[]} values @param {number[]} cased
 * @param {number[]} ignorable @returns {number[]} */
export function mapUnicodeCaseUtf16Columns(text,upper,ranges,indexes,values,cased,ignorable){
 const points=[];
 for(let at=0;at<text.length;at+=1){
  const first=text[at]?? -1;
  if(!Number.isSafeInteger(first)||first<0||first>65535)throw new Error('Unicode casing requires actual UTF16 units');
  const second=text[at+1]?? -1;
  if(first>=55296&&first<=56319&&Number.isSafeInteger(second)&&second>=56320&&second<=57343){points.push(65536+(first-55296)*1024+second-56320);at+=1;}
  else points.push(first);
 }
 const following=[];
 let next=false;
 for(let at=points.length-1;at>=0;at-=1){
  const point=points[at]??0;following.push(next);
  if(!acceptUnicodeCaseProperty(point,ignorable))next=acceptUnicodeCaseProperty(point,cased);
 }
 const result=[];
 let previous=false;
 for(let at=0;at<points.length;at+=1){
  const point=points[at]??0;
  const finalSigma=!upper&&point===931&&previous&&!(following[points.length-1-at]??false);
  const mapped=finalSigma?[962]:readUnicodeCaseMapping(point,ranges,indexes,values);
  for(let index=0;index<mapped.length;index+=1){
   const value=mapped[index]??0;
   if(value<65536)result.push(value);
   else {result.push(55296+Math.floor((value-65536)/1024));result.push(56320+(value-65536)%1024);}
  }
  if(!acceptUnicodeCaseProperty(point,ignorable))previous=acceptUnicodeCaseProperty(point,cased);
 }
 return result;
}

/** Complete pinned Unicode 16 default upper/lower mapping.
 * @param {number[]} text @param {boolean} upper @returns {number[]} */
export function mapUnicodeCaseUtf16Text(text,upper){
 const ranges=upper?readUnicodeCaseUpperRanges():readUnicodeCaseLowerRanges();
 const indexes=upper?readUnicodeCaseUpperIndexes():readUnicodeCaseLowerIndexes();
 const values=upper?readUnicodeCaseUpperValues():readUnicodeCaseLowerValues();
 return mapUnicodeCaseUtf16Columns(text,upper,ranges,indexes,values,readUnicodeCaseCasedRanges(),readUnicodeCaseIgnorableRanges());
}
