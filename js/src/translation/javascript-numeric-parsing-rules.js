// Numeric prefix grammars operate on exact UTF16 and reuse exact rational
// rounding. Host parseInt/parseFloat are never represented execution fallbacks.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
import {normalizeJavaScriptSignedInteger} from './number-bitwise-rules.js';
import {convertJavaScriptBigIntegerNumber,convertJavaScriptStringNumberUnits} from './string-number-rules.js';
import {readJavaScriptSemanticTextLength,constructJavaScriptSemanticTextUnits} from './javascript-semantic-values.js';
/** @param {number} unit @returns {number} */
export function readJavaScriptRadixDigit(unit){return unit>=48&&unit<=57?unit-48:unit>=65&&unit<=90?unit-55:unit>=97&&unit<=122?unit-87:-1;}
/** ToString and radix ToNumber have already completed in their original order.
 * @param {number[]} units @param {number} radixNumber @returns {number} */
export function parseJavaScriptIntegerPrefix(units,radixNumber){
 let start=0;while(start<units.length&&acceptJavaScriptWhitespaceUnit(units[start]??-1))start+=1;
 let negative=false;const sign=units[start]??-1;if(sign===43||sign===45){negative=sign===45;start+=1;}
 let radix=normalizeJavaScriptSignedInteger(radixNumber);const strip=radix===0||radix===16;
 if(radix!==0&&(radix<2||radix>36))return NaN;if(radix===0)radix=10;
 if(strip&&(units[start]??-1)===48&&((units[start+1]??-1)===120||(units[start+1]??-1)===88)){radix=16;start+=2;}
 let value=0n,count=0;const base=BigInt(radix);
 for(let at=start;at<units.length;at+=1){const digit=readJavaScriptRadixDigit(units[at]??-1);if(digit<0||digit>=radix)break;value=value*base+BigInt(digit);count+=1;}
 if(count===0)return NaN;const result=convertJavaScriptBigIntegerNumber(value);return negative?-result:result;
}
/** Longest decimal prefix: a malformed exponent is excluded entirely.
 * @param {number[]} units @returns {number} */
export function parseJavaScriptFloatPrefix(units){
 let start=0;while(start<units.length&&acceptJavaScriptWhitespaceUnit(units[start]??-1))start+=1;
 let at=start;const sign=units[at]??-1;if(sign===43||sign===45)at+=1;
 const infinity=[73,110,102,105,110,105,116,121];let matches=true;
 for(let index=0;index<infinity.length;index+=1)if((units[at+index]??-1)!==(infinity[index]??-2))matches=false;
 if(matches)return sign===45?-Infinity:Infinity;
 let digits=0;while((units[at]??-1)>=48&&(units[at]??-1)<=57){at+=1;digits+=1;}
 if((units[at]??-1)===46){at+=1;while((units[at]??-1)>=48&&(units[at]??-1)<=57){at+=1;digits+=1;}}
 if(digits===0)return NaN;
 const mantissaEnd=at;if((units[at]??-1)===69||(units[at]??-1)===101){at+=1;if((units[at]??-1)===43||(units[at]??-1)===45)at+=1;const exponentStart=at;while((units[at]??-1)>=48&&(units[at]??-1)<=57)at+=1;if(at===exponentStart)at=mantissaEnd;}
 return convertJavaScriptStringNumberUnits(units.slice(start,at));
}
/** @param {string} operation @returns {number} */
export function readJavaScriptNumericParsingArity(operation){return operation==='parse integer text prefix'?2:operation==='parse float text prefix'?1:-1;}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptNumericParsing(operation,values){
 const arity=readJavaScriptNumericParsingArity(operation),input=values[0]??{$:'undefined'},radix=values[1]??{$:'undefined'},length=readJavaScriptSemanticTextLength(input);
 if(arity<0||values.length!==arity||length<0||arity===2&&radix.$!=='number')return {$:'primitiveInvalid',reason:'numeric prefix parsing requires its actual converted UTF16 text and numeric radix'};
 if(length>4096)return {$:'primitiveResourceLimit',reason:'numeric prefix parsing exceeds the selected 4096-unit work profile'};
 const units=constructJavaScriptSemanticTextUnits(input,4096);
 return {$:'primitiveValue',value:{$:'number',value:arity===2?parseJavaScriptIntegerPrefix(units,radix.$==='number'?radix.value:NaN):parseJavaScriptFloatPrefix(units)}};
}
