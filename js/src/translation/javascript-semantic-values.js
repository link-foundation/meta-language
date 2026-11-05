import {readJavaScriptSourceTextProfileError} from './javascript-source-text-rules.js';
/** Explicit semantic operands for ordinary object operations. Reference IDs
 * belong to the caller's checked global link space; callability is an intrinsic
 * fact of that object model, not a host callback or an inferred property name.
 * Text retains UTF-16 units. Integer values and reference identities remain
 * separate domains, even when their numerical encodings happen to agree.
 * @typedef {{ $:'undefined' } | { $:'null' } | { $:'boolean', value:boolean }
 * | { $:'number', value:number } | { $:'integer', value:bigint }
 * | { $:'text', units:number[] } | { $:'sourceText',profile:string,text:string}
 * | { $:'symbol', identity:bigint }
 * | { $:'object', identity:bigint, callable:boolean }} JavaScriptSemanticValue
 */

/** @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptSemanticValueError(value) {
  switch (value.$) {
    case 'undefined': return '';
    case 'null': return '';
    case 'boolean': return '';
    case 'number': return '';
    case 'integer': return '';
    case 'sourceText':return typeof value.text!=='string'?'source text requires its actual complete String value':readJavaScriptSourceTextProfileError(value.profile,value.text.length);
    case 'text': {
      const units = value.units;
      if (units.length > 1048576) return 'semantic text exceeds the explicit UTF16 unit budget';
      for (let at = 0; at < units.length; at += 1) {
        const unit = units[at] ?? -1;
        if (!Number.isInteger(unit) || unit < 0 || unit > 65535) return 'semantic text requires complete ordered UTF16 code units';
      }
      return '';
    }
    case 'symbol': return value.identity > 0n && value.identity <= 18446744073709551615n ? '' : 'a symbol requires a positive unsigned64 global link identity';
    case 'object': return value.identity > 0n && value.identity <= 18446744073709551615n ? '' : 'an object requires a positive unsigned64 global link identity';
  }
}

/** Semantic SameValue, including NaN, signed zero, exact integers, text units,
 * symbol identity and object identity. It does not equate independently named
 * concepts or establish correspondence between different global link spaces.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right
 * @returns {boolean} */
export function testJavaScriptSemanticSameValue(left, right) {
  if (readJavaScriptSemanticValueError(left) !== '' || readJavaScriptSemanticValueError(right) !== '')
    throw new TypeError('SameValue requires two complete values in the declared semantic model');
  if(left.$==='text'||left.$==='sourceText')return testJavaScriptSemanticTextEqual(left,right);
  switch (left.$) {
    case 'undefined': return right.$ === 'undefined';
    case 'null': return right.$ === 'null';
    case 'boolean': {
      if (right.$ !== 'boolean') return false;
      return left.value === right.value;
    }
    case 'number': {
      if (right.$ !== 'number') return false;
      if (left.value === right.value) return left.value !== 0 || 1 / left.value === 1 / right.value;
      return Number.isNaN(left.value) && Number.isNaN(right.value);
    }
    case 'integer': {
      if (right.$ !== 'integer') return false;
      return left.value === right.value;
    }
    case 'symbol': {
      if (right.$ !== 'symbol') return false;
      return left.identity === right.identity;
    }
    case 'object': {
      if (right.$ !== 'object') return false;
      return left.identity === right.identity;
    }
  }
}

/** No flattening or repeated full-content validation of compact source text.
 * The actual source String carries every UTF16 unit in inspectable wire data.
 * @param {JavaScriptSemanticValue} value @returns {number} */
export function readJavaScriptSemanticTextLength(value){return value.$==='sourceText'?value.text.length:value.$==='text'?value.units.length:-1;}
/** @param {JavaScriptSemanticValue} value @param {number} index @returns {number} */
export function readJavaScriptSemanticTextUnit(value,index){
 if(!Number.isSafeInteger(index)||index<0||index>=readJavaScriptSemanticTextLength(value))return -1;
 return value.$==='sourceText'?value.text.charCodeAt(index):value.$==='text'?(value.units[index]??-1):-1;
}
/** @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right @returns {boolean} */
export function testJavaScriptSemanticTextEqual(left,right){
 const length=readJavaScriptSemanticTextLength(left);if(length<0||length!==readJavaScriptSemanticTextLength(right))return false;
 if(left.$==='sourceText'&&right.$==='sourceText')return left.text===right.text;
 for(let at=0;at<length;at++){
  const a=left.$==='text'?(left.units[at]??-1):left.$==='sourceText'?left.text.charCodeAt(at):-1;
  const b=right.$==='text'?(right.units[at]??-1):right.$==='sourceText'?right.text.charCodeAt(at):-1;
  if(a!==b)return false;
 }return true;
}
/** @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptSemanticTextString(value){
 if(value.$==='sourceText')return value.text;
 if(value.$!=='text')throw new TypeError('semantic text transport requires its actual complete text value');
 let text='';for(const unit of value.units)text+=String.fromCharCode(unit);return text;
}
/** Callers prove the explicit operation budget before requesting unit arrays.
 * @param {JavaScriptSemanticValue} value @param {number} limit @returns {number[]} */
export function constructJavaScriptSemanticTextUnits(value,limit){
 const length=readJavaScriptSemanticTextLength(value);if(length<0||length>limit)throw new RangeError('semantic text conversion requires its actual operation unit budget');
 if(value.$==='text')return value.units;
 const units=[];for(let at=0;at<length;at++)units.push(readJavaScriptSemanticTextUnit(value,at));return units;
}
/** @param {string} text @returns {JavaScriptSemanticValue} */
export function constructJavaScriptSourceText(text){if(typeof text!=='string')throw new TypeError('source text requires its actual complete String value');const profile='inspectable UTF16 source text 1',error=readJavaScriptSourceTextProfileError(profile,text.length);if(error!=='')throw new RangeError(error);return {$:'sourceText',profile,text};}

/** @param {JavaScriptSemanticValue} value @returns {boolean} */
export function testJavaScriptSemanticAccessor(value) {
  if (readJavaScriptSemanticValueError(value) !== '') return false;
  if (value.$ === 'undefined') return true;
  if (value.$ !== 'object') return false;
  return value.callable;
}
