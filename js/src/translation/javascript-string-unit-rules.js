// JavaScript strings are ordered UTF-16 code units, including isolated
// surrogates. Unicode scalar decoding is a different source operation.
import {readJavaScriptNumericDigitUnit} from './numeric-digit-rules.js';

/** @typedef {{ $:'units', units:number[], end:number } | { $:'malformed' }} JavaScriptStringEscape */
/** Read the actual complete escape; arbitrarily many leading hexadecimal
 * zeroes are valid in a braced escape. Offsets are relative to start.
 * @param {number[]} source @param {number} start @returns {JavaScriptStringEscape} */
export function decodeJavaScriptStringUnicodeEscape(source,start) {
  if(!Number.isSafeInteger(start)||start<0||source[start]!==92)return{$:'malformed'};
  const marker=source[start+1]?? -1;
  if(marker!==117&&marker!==120)return{$:'malformed'};
  const braced=marker===117&&source[start+2]===123;
  const count=marker===120?2:4;
  let at=start+(braced?3:2),digits=0,code=0;
  while(at<source.length&&(braced||digits<count)){
    const digit=readJavaScriptNumericDigitUnit(source[at]?? -1);
    if(digit<0||digit>=16)break;
    // Saturation preserves rejection without losing numeric precision.
    code=Math.min(1114112,code*16+digit);digits+=1;at+=1;
  }
  if(braced){if(digits===0||source[at]!==125)return{$:'malformed'};at+=1;}
  else if(digits!==count)return{$:'malformed'};
  if(code>1114111)return{$:'malformed'};
  const units=code<65536?[code]:[55296+Math.floor((code-65536)/1024),56320+(code-65536)%1024];
  return{$:'units',units,end:at-start};
}

/** @param {number[]} units @returns {boolean} */
export function acceptJavaScriptStringUnits(units) {
  for(let at=0;at<units.length;at+=1){
    const unit=units[at]?? -1;
    if(!Number.isSafeInteger(unit)||unit<0||unit>65535)return false;
  }
  return true;
}

/** @param {number[]} units @returns {boolean} */
export function requireJavaScriptStringUnitTransport(units) {
  for(let at=0;at<units.length;at+=1){
    const unit=units[at]?? -1;
    if(unit>=55296&&unit<=56319){
      const low=units[at+1]?? -1;
      if(low<56320||low>57343)return true;
      at+=1;
    }else if(unit>=56320&&unit<=57343)return true;
  }
  return false;
}

/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptStringUnitConstruction(kind,operation) {
  return kind==='math'&&operation==='createJavaScriptStringFromUnits';
}

/** A checked column contains integral UTF-16 units. No target may turn it
 * into a scalar string before a scalar boundary explicitly replaces errors.
 * @param {string} target @param {string} values @returns {string} */
export function renderJavaScriptStringUnitConstruction(target,values) {
  if(target==='JavaScript')return '(('+values+').map(ml_unit=>String.fromCharCode(ml_unit)).join(""))';
  if(target==='Rust')return 'crate::ml_string::Text::from_units(('+values+').into_iter().map(|ml_unit| ml_unit as u16).collect())';
  return '';
}
