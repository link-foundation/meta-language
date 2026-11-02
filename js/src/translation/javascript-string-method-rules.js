import {readJavaScriptArrayIndex,testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
// Converted String operands share exact UTF16 access and explicit work bounds.
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit,readJavaScriptSemanticTextString} from './javascript-semantic-values.js';
import {readUtf16TextInteger} from './utf16-text-rules.js';
/** @param {string} operation @returns {number} */
export function readJavaScriptStringMethodPrimitiveArity(operation){return ['require String receiver','read String code unit','test String own key','read String own key'].includes(operation)?2:['substring String units','substr String units','test String prefix at','test String suffix at','test String inclusion at'].includes(operation)?3:-1;}
/** @param {string} operation @param {JavaScriptSemanticValue[]} values @returns {JavaScriptCallPrimitiveResult} */
export function executeJavaScriptStringMethodPrimitive(operation,values){
 const receiver=values[0]??{$:'undefined'},second=values[1]??{$:'undefined'},third=values[2]??{$:'undefined'};
 if(operation==='require String receiver'){
  if(second.$!=='text'&&second.$!=='sourceText')return {$:'primitiveInvalid',reason:'String receiver checks retain their actual protected method name'};
  return receiver.$==='null'||receiver.$==='undefined'?{$:'primitiveError',errorClass:'TypeError',reason:'String.prototype.'+readJavaScriptSemanticTextString(second)+' called on null or undefined'}:{$:'primitiveValue',value:receiver};
 }
 if(receiver.$!=='text'&&receiver.$!=='sourceText')return {$:'primitiveInvalid',reason:'String methods consume their actual converted UTF16 receiver'};
 const length=readJavaScriptSemanticTextLength(receiver);
 if(operation==='test String own key'||operation==='read String own key'){if(second.$!=='text'&&second.$!=='sourceText'&&second.$!=='symbol')return {$:'primitiveInvalid',reason:'String own-property reads retain actual converted property keys'};const index=readJavaScriptArrayIndex(second),isLength=testJavaScriptArrayLengthKey(second),present=isLength||index>=0&&index<length;if(operation==='test String own key')return {$:'primitiveValue',value:{$:'boolean',value:present}};return present?{$:'primitiveValue',value:isLength?{$:'number',value:length}:{$:'text',units:[readJavaScriptSemanticTextUnit(receiver,index)]}}:{$:'primitiveInvalid',reason:'String own-property value requires its actual present own key'};}
 if(operation==='read String code unit'){
  if(second.$!=='number')return {$:'primitiveInvalid',reason:'String code units retain their actual completed numeric position'};const position=readUtf16TextInteger(second.value);return {$:'primitiveValue',value:{$:'number',value:position<0||position>=length?NaN:readJavaScriptSemanticTextUnit(receiver,position)}};
 }
 if(operation==='substring String units'||operation==='substr String units'){
  if(second.$!=='number'||third.$!=='number')return {$:'primitiveInvalid',reason:'String slices retain their actual completed numeric bounds'};
  const first=readUtf16TextInteger(second.value),last=readUtf16TextInteger(third.value),start=operation==='substr String units'?first<0?Math.max(length+first,0):Math.min(first,length):Math.min(Math.max(first,0),length),end=operation==='substr String units'?start+Math.min(Math.max(last,0),length-start):Math.min(Math.max(last,0),length),low=Math.min(start,end),high=Math.max(start,end);
  if(high-low>1048576)return {$:'primitiveResourceLimit',reason:'String slicing exceeds its explicit UTF16 output budget'};const units=[];for(let at=low;at<high;at++)units.push(readJavaScriptSemanticTextUnit(receiver,at));return {$:'primitiveValue',value:{$:'text',units}};
 }
 if(second.$!=='text'&&second.$!=='sourceText'||third.$!=='number')return {$:'primitiveInvalid',reason:'String searches retain actual converted search text and numeric position'};
 const needleLength=readJavaScriptSemanticTextLength(second),position=Math.min(Math.max(readUtf16TextInteger(third.value),0),length),start=operation==='test String suffix at'?position-needleLength:position;
 if(operation==='test String prefix at'||operation==='test String suffix at'){
  if(needleLength>1048576)return {$:'primitiveResourceLimit',reason:'String boundary comparisons exceed the explicit UTF16 work budget'};let matches=start>=0&&start+needleLength<=length;for(let at=0;matches&&at<needleLength;at++)if(readJavaScriptSemanticTextUnit(receiver,start+at)!==readJavaScriptSemanticTextUnit(second,at))matches=false;return {$:'primitiveValue',value:{$:'boolean',value:matches}};
 }
 if(operation!=='test String inclusion at')return {$:'primitiveInvalid',reason:'String helpers require their actual registered operation'};
 if(Math.max(0,length-position-needleLength+1)*needleLength>67108864)return {$:'primitiveResourceLimit',reason:'String inclusion exceeds its explicit comparison work budget'};
 for(let start=position;start<=length-needleLength;start++){let matches=true;for(let at=0;matches&&at<needleLength;at++)if(readJavaScriptSemanticTextUnit(receiver,start+at)!==readJavaScriptSemanticTextUnit(second,at))matches=false;if(matches)return {$:'primitiveValue',value:{$:'boolean',value:true}};}return {$:'primitiveValue',value:{$:'boolean',value:false}};
}
