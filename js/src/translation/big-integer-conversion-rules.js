// Exact BigInt conversion algorithms. A zero/one element column distinguishes
// a failed conversion from every integer, including zero. Hosts select the
// language's syntax/range/type exception and retain its actual source message.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
import {readJavaScriptNumericDigitUnit} from './numeric-digit-rules.js';

/** Complete StringIntegerLiteral, without a Number intermediate or size cap.
 * @param {number[]} units @returns {bigint[]} */
export function convertJavaScriptStringBigIntegerUnits(units) {
  for(let index=0;index<units.length;index+=1){
    const unit=units[index]?? -1;
    if(!Number.isSafeInteger(unit)||unit<0||unit>65535)return [];
  }
  let start=0,end=units.length;
  while(start<end&&acceptJavaScriptWhitespaceUnit(units[start]?? -1))start+=1;
  while(end>start&&acceptJavaScriptWhitespaceUnit(units[end-1]?? -1))end-=1;
  if(start===end)return [0n];
  let negative=false,signed=false;
  const sign=units[start]?? -1;
  if(sign===43||sign===45){negative=sign===45;signed=true;start+=1;}
  if(start===end)return [];
  let radix=10,base=10n;
  if(!signed&&(units[start]?? -1)===48&&end-start>=2){
    const marker=units[start+1]?? -1;
    const selected=marker===120||marker===88?16:marker===98||marker===66?2:marker===111||marker===79?8:0;
    if(selected!==0){radix=selected;base=selected===16?16n:selected===8?8n:2n;start+=2;}
  }
  if(start===end)return [];
  const digits=[0n,1n,2n,3n,4n,5n,6n,7n,8n,9n,10n,11n,12n,13n,14n,15n];
  let value=0n;
  for(let index=start;index<end;index+=1){
    const digit=readJavaScriptNumericDigitUnit(units[index]?? -1);
    if(digit<0||digit>=radix)return [];
    value=value*base+(digits[digit]??0n);
  }
  return [negative?-value:value];
}

/** Every finite integral binary64 value becomes its exact integer.
 * @param {number} value @returns {bigint[]} */
export function convertJavaScriptNumberBigInteger(value) {
  if(!Number.isInteger(value))return [];
  const negative=value<0;
  value=Math.abs(value);
  let integer=0n,weight=1n;
  while(value>=1){
    if(value%2===1)integer+=weight;
    value=Math.floor(value/2);weight*=2n;
  }
  return [negative?-integer:integer];
}

/** @param {boolean} value @returns {bigint} */
export function convertJavaScriptBooleanBigInteger(value) {
  return value?1n:0n;
}

/** @param {string} kind @returns {string} */
export function readJavaScriptBigIntegerConversionForm(kind) {
  if(kind==='int'||kind==='nat')return 'identity';
  if(kind==='float')return 'number';
  if(kind==='string')return 'string';
  if(kind==='bool')return 'boolean';
  if(kind==='unit')return 'undefined';
  return kind==='option'?'optional':'unsupported';
}

/** @param {string} form @returns {string} */
export function readJavaScriptBigIntegerConversionErrorClass(form) {
  if(form==='string')return 'SyntaxError';
  if(form==='number')return 'RangeError';
  return form==='undefined'||form==='null'?'TypeError':'';
}

/** V8's version-bounded message profile is distinct from the conversion rule.
 * @param {string} form @param {string} value @returns {string} */
export function renderJavaScriptBigIntegerConversionMessage(form,value) {
  const pieces=readJavaScriptBigIntegerConversionMessagePieces(form);
  return pieces.length===2?(pieces[0]??'')+value+(pieces[1]??''):'';
}
/** Preserve actual UTF-16 payloads in runtimes whose host string is UTF-8.
 * @param {string} form @returns {string[]} */
export function readJavaScriptBigIntegerConversionMessagePieces(form) {
  if(form==='number')return ['The number ',' cannot be converted to a BigInt because it is not an integer'];
  return form==='string'||form==='undefined'||form==='null'?['Cannot convert ',' to a BigInt']:[];
}
