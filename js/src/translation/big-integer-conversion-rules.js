// Exact BigInt conversion algorithms. A zero/one element column distinguishes
// a failed conversion from every integer, including zero. Hosts select the
// language's syntax/range/type exception and retain its actual source message.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
import {readJavaScriptNumericDigitUnit} from './numeric-digit-rules.js';
import {readJavaScriptUtf8StringUnits} from './utf8-string-unit-rules.js';
import {convertStringNumberRational} from './string-number-rules.js';

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
/** @param {number[]} bytes @returns {bigint[]} */
export function convertJavaScriptUtf8BigInteger(bytes) {
  return convertJavaScriptStringBigIntegerUnits(readJavaScriptUtf8StringUnits(bytes));
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

/** Truncating signed division and dividend-signed remainder. Empty means a
 * zero divisor. Only division by the literal two is a primitive here, so the
 * full definition self-translates without recursively calling itself.
 * @param {bigint} left @param {bigint} right @returns {bigint[]} */
export function divideJavaScriptBigInteger(left,right) {
  if(right===0n)return [];
  let remaining=left<0n?-left:left;
  const divisor=right<0n?-right:right;
  let shifted=divisor,weight=1n,quotient=0n;
  while(shifted<=remaining){shifted*=2n;weight*=2n;}
  while(weight>1n){
    shifted/=2n;weight/=2n;
    if(shifted<=remaining){remaining-=shifted;quotient+=weight;}
  }
  return [(left<0n)!==(right<0n)?-quotient:quotient,left<0n?-remaining:remaining];
}
/** Repeated squaring over exact integers. Empty is a negative exponent.
 * @param {bigint} base @param {bigint} exponent @returns {bigint[]} */
export function powerJavaScriptBigInteger(base,exponent){
 if(exponent<0n)return [];
 let result=1n,weight=base,remaining=exponent;
 while(remaining>0n){
  if(remaining%2n===1n)result*=weight;
  remaining/=2n;
  if(remaining>0n)weight*=weight;
 }
 return [result];
}

/** Directed fixed-point bounds use independent normalized exponents. This
 * keeps every significand bounded by precision even for enormous powers.
 * @param {bigint} value @param {bigint} exponent @param {bigint} scale @param {boolean} upward @returns {bigint[]} */
function normalizeNumberPowerBound(value,exponent,scale,upward){
 if(value<=0n)return [0n,0n];
 while(value>=scale*2n){value=(value+(upward?1n:0n))/2n;exponent+=1n;}
 while(value<scale){value*=2n;exponent-=1n;}
 return [value,exponent];
}
/** @param {bigint} numerator @param {bigint} denominator @param {boolean} upward @returns {bigint} */
function divideNumberPowerBound(numerator,denominator,upward){
 const result=divideJavaScriptBigInteger(numerator,denominator);
 const quotient=result[0]??0n,remainder=result[1]??0n;
 return quotient+(upward&&remainder!==0n?1n:0n);
}
/** @param {bigint} value @returns {bigint} */
function rootNumberPowerInteger(value){
 if(value<=0n)return 0n;
 let root=1n,remaining=value;
 while(remaining>1n){remaining/=4n;root*=2n;}
 root*=2n;
 while(true){
  const next=(root+divideNumberPowerBound(value,root,false))/2n;
  if(next>=root)return root;
  root=next;
 }
}
/** @param {bigint[]} left @param {bigint[]} right @param {bigint} scale @returns {bigint[]} */
function multiplyNumberPowerInterval(left,right,scale){
 const lower=normalizeNumberPowerBound(divideNumberPowerBound((left[0]??0n)*(right[0]??0n),scale,false),(left[1]??0n)+(right[1]??0n),scale,false);
 const upper=normalizeNumberPowerBound(divideNumberPowerBound((left[2]??0n)*(right[2]??0n),scale,true),(left[3]??0n)+(right[3]??0n),scale,true);
 return [lower[0]??0n,lower[1]??0n,upper[0]??0n,upper[1]??0n];
}
/** @param {bigint} value @param {bigint} exponent @param {bigint} scale @param {boolean} upward @returns {bigint[]} */
function rootNumberPowerBound(value,exponent,scale,upward){
 if(exponent%2n!==0n){value*=2n;exponent-=1n;}
 const square=value*scale;
 let root=rootNumberPowerInteger(square);
 if(upward&&root*root<square)root+=1n;
 return normalizeNumberPowerBound(root,exponent/2n,scale,upward);
}
/** @param {bigint[]} interval @param {bigint} scale @returns {bigint[]} */
function rootNumberPowerInterval(interval,scale){
 const lower=rootNumberPowerBound(interval[0]??0n,interval[1]??0n,scale,false);
 const upper=rootNumberPowerBound(interval[2]??0n,interval[3]??0n,scale,true);
 return [lower[0]??0n,lower[1]??0n,upper[0]??0n,upper[1]??0n];
}
/** @param {bigint[]} interval @param {bigint} scale @returns {bigint[]} */
function invertNumberPowerInterval(interval,scale){
 const lower=normalizeNumberPowerBound(divideNumberPowerBound(scale*scale,interval[2]??0n,false),-(interval[3]??0n),scale,false);
 const upper=normalizeNumberPowerBound(divideNumberPowerBound(scale*scale,interval[0]??0n,true),-(interval[1]??0n),scale,true);
 return [lower[0]??0n,lower[1]??0n,upper[0]??0n,upper[1]??0n];
}
/** Positive finite binary64 decomposition: exact integer significand and scale.
 * @param {number} value @returns {bigint[]} */
function decomposeNumberPowerMagnitude(value){
 let exponent=0n;
 while(value>=2){value/=2;exponent+=1n;}
 while(value<1){value*=2;exponent-=1n;}
 return [convertJavaScriptNumberBigInteger(value*4503599627370496)[0]??0n,exponent-52n];
}
/** @param {bigint} value @param {bigint} exponent @param {bigint} precision @returns {number} */
function roundNumberPowerBound(value,exponent,precision){
 if(value===0n||exponent< -1076n)return 0;
 if(exponent>1024n)return Infinity;
 const shift=exponent-precision;
 if(shift>=0n)return convertStringNumberRational(value*(powerJavaScriptBigInteger(2n,shift)[0]??0n),1n);
 return convertStringNumberRational(value,powerJavaScriptBigInteger(2n,-shift)[0]??0n);
}
/** A canonical correctly rounded dyadic-power implementation. Every positive
 * finite result is enclosed by directed integer intervals; precision increases
 * until both ends round identically. No host pow/log/exp or precision ceiling.
 * @param {number} base @param {number} exponent @returns {number} */
export function powerJavaScriptNumber(base,exponent){
 if(Number.isNaN(exponent))return NaN;
 if(exponent===0)return 1;
 if(Number.isNaN(base))return NaN;
 const magnitude=Math.abs(base);
 if(!Number.isFinite(exponent)){
  if(magnitude===1)return NaN;
  return magnitude>1?(exponent>0?Infinity:0):(exponent>0?0:Infinity);
 }
 const odd=Number.isInteger(exponent)&&Math.abs(exponent)%2===1;
 const negative=(base<0||base===0&&1/base<0)&&odd;
 if(base===0){const value=exponent>0?0:Infinity;return negative?-value:value;}
 if(!Number.isFinite(base)){const value=exponent>0?Infinity:0;return negative?-value:value;}
 if(base<0&&!Number.isInteger(exponent))return NaN;
 if(magnitude===1)return negative?-1:1;
 const input=decomposeNumberPowerMagnitude(magnitude),power=decomposeNumberPowerMagnitude(Math.abs(exponent));
 let precision=192n;
 while(true){
  const scale=powerJavaScriptBigInteger(2n,precision)[0]??0n;
  const value=(input[0]??0n)*(powerJavaScriptBigInteger(2n,precision-52n)[0]??0n);
  const initialExponent=(input[1]??0n)+52n;
  let interval=[value,initialExponent,value,initialExponent];
  if(exponent<0)interval=invertNumberPowerInterval(interval,scale);
  let shift=power[1]??0n;
  while(shift>0n){interval=multiplyNumberPowerInterval(interval,interval,scale);shift-=1n;}
  while(shift<0n){interval=rootNumberPowerInterval(interval,scale);shift+=1n;}
  let result=[scale,0n,scale,0n],remaining=power[0]??0n;
  while(remaining>0n){
   if(remaining%2n===1n)result=multiplyNumberPowerInterval(result,interval,scale);
   remaining/=2n;
   if(remaining>0n)interval=multiplyNumberPowerInterval(interval,interval,scale);
  }
  const lower=roundNumberPowerBound(result[0]??0n,result[1]??0n,precision);
  const upper=roundNumberPowerBound(result[2]??0n,result[3]??0n,precision);
  if(lower===upper)return negative?-lower:lower;
  precision*=2n;
 }
}
/** @param {string} prefix @param {boolean} parenthesized @returns {boolean} */
export function acceptJavaScriptExponentiationBase(prefix,parenthesized){
 return parenthesized||!['+','-','!','~','typeof','void','delete','await'].includes(prefix);
}
/** @returns {string} */
export function readJavaScriptBigIntegerExponentMessage(){return 'Exponent must be positive';}
/** @param {string} target @param {string} base @param {string} exponent @returns {string} */
export function renderJavaScriptBigIntegerPower(target,base,exponent){
 if(target==='JavaScript')return 'ml_big_integer_conversion.power('+base+','+exponent+')';
 if(target==='Rust')return 'crate::ml_big_integer_conversion::power_java_script_big_integer('+base+','+exponent+')';
 return target==='Lean'||target==='Rocq'?'(MLBigIntegerConversion.power '+base+' '+exponent+')':'';
}
/** Checked integer nodes already own canonical signed decimal text; empty
 * denotes a nonliteral. Zero spellings cannot prove a nonzero divisor.
 * @param {string} value @returns {boolean} */
export function acceptNonzeroJavaScriptIntegerLiteral(value){
  return value!==''&&value!=='0'&&value!=='-0';
}
/** @param {string} language @param {string} operation @param {string} kind @param {string} literal @returns {boolean} */
export function requireJavaScriptBigIntegerDivision(language,operation,kind,literal){
  return language==='JavaScript'&&kind==='int'&&(operation==='div'||operation==='rem')&&!acceptNonzeroJavaScriptIntegerLiteral(literal);
}
/** @returns {string} */
export function readJavaScriptBigIntegerDivisionMessage(){return 'Division by zero';}
/** @param {string} target @param {string} left @param {string} right @returns {string} */
export function renderJavaScriptBigIntegerDivision(target,left,right){
  if(target==='JavaScript')return 'ml_big_integer_conversion.divide('+left+','+right+')';
  if(target==='Rust')return 'crate::ml_big_integer_conversion::divide_java_script_big_integer('+left+','+right+')';
  return target==='Lean'||target==='Rocq'?'(MLBigIntegerConversion.divide '+left+' '+right+')':'';
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

/** @param {string} name @returns {string} */
export function readJavaScriptBigIntegerBuiltinOperation(name) {
  return name==='BigInt'?'convertJavaScriptBigInteger':'';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptBigIntegerResultOperation(operation) {
  if(operation==='powerJavaScriptNumberValue')return 'number-power';
  if(operation==='powerJavaScriptBigIntegerResult')return 'power';
  if(operation==='divideJavaScriptBigIntegerResult')return 'division';
  if(operation==='parseJavaScriptStringBigInteger')return 'string';
  if(operation==='convertJavaScriptBooleanBigIntegerValue')return 'boolean';
  return operation==='convertJavaScriptNumberBigIntegerResult'?'number':'';
}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptBigIntegerResult(kind,operation) {
  return kind==='math'&&readJavaScriptBigIntegerResultOperation(operation)!=='';
}
/** @param {string} target @param {string} base @param {string} exponent @returns {string} */
export function renderJavaScriptNumberPower(target,base,exponent){
 if(target==='JavaScript')return 'ml_big_integer_conversion.powerNumber('+base+','+exponent+')';
 if(target==='Rust')return 'crate::ml_big_integer_conversion::power_java_script_number('+base+','+exponent+')';
 return target==='Lean'||target==='Rocq'?'(MLBigIntegerConversion.powerNumber '+base+' '+exponent+')':'';
}
/** @param {string} target @param {string} form @param {string} value @param {boolean} utf16 @returns {string} */
export function renderJavaScriptBigIntegerResultExpression(target,form,value,utf16) {
  if(form==='boolean'){
    if(target==='JavaScript')return 'ml_big_integer_conversion.convertBoolean('+value+')';
    if(target==='Rust')return 'crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer('+value+')';
    if(target==='Lean'||target==='Rocq')return '(MLBigIntegerConversion.convertBoolean '+value+')';
    return '';
  }
  if(form!=='string'&&form!=='number')return '';
  if(target==='JavaScript')return form==='number'?'ml_big_integer_conversion.convertNumber('+value+')'
    :'((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({length:ml_text.length}, (_,index)=>ml_text.charCodeAt(index))))('+value+')';
  if(target==='Rust'){
    if(form==='number')return 'crate::ml_big_integer_conversion::convert_java_script_number_big_integer('+value+')';
    const units=utf16?'('+value+').0.iter().copied()':'('+value+').encode_utf16()';
    return 'crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&'+units+'.map(f64::from).collect::<Vec<f64>>())';
  }
  if(target==='Lean')return form==='number'?'(MLBigIntegerConversion.convertNumber '+value+')'
    :'(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] '+value+'))';
  if(target==='Rocq')return form==='number'?'(MLBigIntegerConversion.convertNumber '+value+')'
    :'(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) '+value+'))';
  return '';
}
