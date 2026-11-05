// Complete ordered own-field algorithms. Names are exact UTF16 unit vectors;
// layouts are canonical structural fields, orders are separate value data.
import {acceptRecordFieldName,compareRecordFieldNames,readRecordFieldPosition} from './record-value-rules.js';
import {readJavaScriptLiteralPropertyIndex} from './javascript-literal-property-rules.js';

/** @returns {number[]} */
function refuseJavaScriptRecordOrderName(){throw new TypeError('an ordered record needs every actual own field name');}
/** @returns {number} */
function refuseJavaScriptRecordOrderPosition(){throw new TypeError('an ordered record needs every actual field position');}
/** @param {number[][]} names @param {number} index @returns {number[]} */
function readJavaScriptRecordOrderName(names,index){return names[index]??refuseJavaScriptRecordOrderName();}
/** @param {number[]} positions @param {number} index @returns {number} */
function readJavaScriptRecordOrderPosition(positions,index){return positions[index]??refuseJavaScriptRecordOrderPosition();}

/** @param {number[][]} names @returns {boolean} */
export function acceptJavaScriptRecordOrderNames(names){
 for(let index=0;index<names.length;index+=1){
  const name=readJavaScriptRecordOrderName(names,index);
  if(!acceptRecordFieldName(name))return false;
  if(index>0&&compareRecordFieldNames(readJavaScriptRecordOrderName(names,index-1),name)>=0)return false;
 }
 return true;
}
/** @param {number} count @param {number[]} order @returns {boolean} */
export function acceptJavaScriptRecordOrderPermutation(count,order){
 if(!Number.isSafeInteger(count)||count<0||count===0&&1/count<0||order.length!==count)return false;
 for(let index=0;index<order.length;index+=1){
  const value=readJavaScriptRecordOrderPosition(order,index);
  if(!Number.isInteger(value)||value<0||value>=count||value===0&&1/value<0)return false;
  for(let previous=0;previous<index;previous+=1)if(readJavaScriptRecordOrderPosition(order,previous)===value)return false;
 }
 return true;
}
/** @param {number[][]} names @param {number[]} insertion @returns {string} */
export function readJavaScriptRecordOrderCreationError(names,insertion){
 return !acceptJavaScriptRecordOrderNames(names)||!acceptJavaScriptRecordOrderPermutation(names.length,insertion)?'invalid ordered record field sequence':'';
}
/** Numeric keys precede other Strings, in ascending numeric order. Other
 * String keys retain first insertion, including after a value overwrite.
 * @param {number[][]} names @param {number[]} insertion @returns {number[]} */
export function createJavaScriptRecordPropertyOrder(names,insertion){
 const error=readJavaScriptRecordOrderCreationError(names,insertion);
 if(error)throw new TypeError(error);
 let numeric=[];
 for(let field=0;field<names.length;field+=1){
  const key=readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names,field));
  if(key<0)continue;
  let position=0;
  while(position<numeric.length&&readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names,readJavaScriptRecordOrderPosition(numeric,position)))<key)position+=1;
  numeric=numeric.slice(0,position).concat([field],numeric.slice(position));
 }
 let result=numeric;
 for(let index=0;index<insertion.length;index+=1){
  const field=readJavaScriptRecordOrderPosition(insertion,index);
  if(readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names,field))<0)result=result.concat([field]);
 }
 return result;
}
/** @param {number[][]} names @param {number[]} order @returns {boolean} */
export function acceptJavaScriptRecordPropertyOrder(names,order){
 if(!acceptJavaScriptRecordOrderNames(names)||!acceptJavaScriptRecordOrderPermutation(names.length,order))return false;
 const expected=createJavaScriptRecordPropertyOrder(names,order);
 for(let index=0;index<order.length;index+=1)if(readJavaScriptRecordOrderPosition(order,index)!==readJavaScriptRecordOrderPosition(expected,index))return false;
 return true;
}
/** @param {number[][]} names @param {number[][]} chunks @returns {string} */
export function readJavaScriptRecordOrderMergeError(names,chunks){
 if(!acceptJavaScriptRecordOrderNames(names))return 'invalid ordered record field names';
 let insertion=[];
 for(let chunk=0;chunk<chunks.length;chunk+=1){
  const values=readJavaScriptRecordOrderName(chunks,chunk);
  for(let index=0;index<values.length;index+=1){
   const field=readJavaScriptRecordOrderPosition(values,index);
   if(!Number.isInteger(field)||field<0||field>=names.length||field===0&&1/field<0)return 'invalid ordered record source field';
   if(!insertion.includes(field))insertion=insertion.concat([field]);
  }
 }
 return readJavaScriptRecordOrderCreationError(names,insertion);
}
/** Chunks come from actual field definitions and actual spread value orders.
 * A repeated field keeps its first position while its separately stored value
 * can be replaced. No absent field or placeholder name is manufactured.
 * @param {number[][]} names @param {number[][]} chunks @returns {number[]} */
export function mergeJavaScriptRecordPropertyOrder(names,chunks){
 const error=readJavaScriptRecordOrderMergeError(names,chunks);
 if(error)throw new TypeError(error);
 let insertion=[];
 for(let chunk=0;chunk<chunks.length;chunk+=1){
  const values=readJavaScriptRecordOrderName(chunks,chunk);
  for(let index=0;index<values.length;index+=1){
   const field=readJavaScriptRecordOrderPosition(values,index);
   if(!insertion.includes(field))insertion=insertion.concat([field]);
  }
 }
 return createJavaScriptRecordPropertyOrder(names,insertion);
}
/** @param {number[][]} target @param {number[][]} source @param {number[]} order @returns {string} */
export function readJavaScriptRecordOrderRebaseError(target,source,order){
 if(!acceptJavaScriptRecordOrderNames(target)||!acceptJavaScriptRecordPropertyOrder(source,order))return 'invalid ordered record spread source';
 for(let index=0;index<source.length;index+=1)if(readRecordFieldPosition(target,readJavaScriptRecordOrderName(source,index))<0)return 'an ordered record spread field is missing from its target';
 return '';
}
/** Rebase an actual spread's own-field order into the new complete layout.
 * @param {number[][]} target @param {number[][]} source @param {number[]} order @returns {number[]} */
export function rebaseJavaScriptRecordPropertyOrder(target,source,order){
 const error=readJavaScriptRecordOrderRebaseError(target,source,order);
 if(error)throw new TypeError(error);
 let result=[];
 for(let index=0;index<order.length;index+=1){
  const name=readJavaScriptRecordOrderName(source,readJavaScriptRecordOrderPosition(order,index));
  const field=readRecordFieldPosition(target,name);
  result=result.concat([field]);
 }
 return result;
}
/** A rest copy retains the original source order after excluding fields.
 * Target is the complete canonical subset, never an invented field layout.
 * @param {number[][]} target @param {number[][]} source @param {number[]} order
 * @returns {number[]} */
export function copyJavaScriptRecordPropertyOrder(target,source,order){
 if(!acceptJavaScriptRecordOrderNames(target)||!acceptJavaScriptRecordPropertyOrder(source,order))throw new TypeError('invalid object rest source field order');
 for(let index=0;index<target.length;index+=1)if(readRecordFieldPosition(source,readJavaScriptRecordOrderName(target,index))<0)throw new TypeError('an object rest target must be a complete source subset');
 let result=[];
 for(let index=0;index<order.length;index+=1){const field=readRecordFieldPosition(target,readJavaScriptRecordOrderName(source,readJavaScriptRecordOrderPosition(order,index)));if(field>=0)result=result.concat([field]);}
 return result;
}
/** @param {number[][]} names @param {number[]} order @returns {string} */
export function readJavaScriptRecordOrderKeyError(names,order){return acceptJavaScriptRecordPropertyOrder(names,order)?'':'invalid ordered record property order';}
/** @param {number[][]} names @param {number[]} order @returns {number[][]} */
export function readJavaScriptRecordPropertyKeys(names,order){
 const error=readJavaScriptRecordOrderKeyError(names,order);
 if(error)throw new TypeError(error);
 let result=[];
 for(let index=0;index<order.length;index+=1)result=result.concat([readJavaScriptRecordOrderName(names,readJavaScriptRecordOrderPosition(order,index))]);
 return result;
}

/** Exact source String keys, including isolated UTF16 units.
 * @param {number[][]} names @param {number[]} order @returns {string[]} */
export function readJavaScriptRecordPropertyKeyTexts(names,order){
 const rows=readJavaScriptRecordPropertyKeys(names,order);
 let result=[];
 for(let index=0;index<rows.length;index+=1){
  const units=readJavaScriptRecordOrderName(rows,index);let key='';
  for(let at=0;at<units.length;at+=1)key+=String.fromCharCode(readJavaScriptRecordOrderPosition(units,at));
  result=result.concat([key]);
 }
 return result;
}
/** The runtime key is an exact String, not a source-name alias or numeric
 * coercion. The existing canonical field algorithm owns the actual search.
 * @param {number[][]} names @param {string} key @returns {number} */
export function readJavaScriptRecordPropertyIndex(names,key){
 let units=[];for(let index=0;index<key.length;index+=1)units=units.concat([key.charCodeAt(index)]);
 return readRecordFieldPosition(names,units);
}
/** Dense Array/tuple indices and String virtual enumerable properties.
 * @param {number} length @returns {string[]} */
export function createJavaScriptDensePropertyKeyTexts(length){
 if(!Number.isSafeInteger(length)||length<0||length===0&&1/length<0)throw new TypeError('invalid dense own-field length');
 let result=[];
 for(let index=0;index<length;index+=1)result=result.concat([String(index)]);
 return result;
}
/** String wrappers enumerate individual code units, not code points.
 * @param {string} text @returns {string[]} */
export function createJavaScriptStringOwnPropertyValues(text){
 let result=[];
 for(let index=0;index<text.length;index+=1)result=result.concat([text.charAt(index)]);
 return result;
}
