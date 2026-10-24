// Exact StringNumericLiteral conversion. Hosts supply actual UTF-16 units.
// Integer division chooses the final binary64 mantissa with ties to even;
// there is no host decimal parser, Number(string) or intermediate rounding.
import { acceptJavaScriptWhitespaceUnit } from './javascript-whitespace-rules.js';
import {readJavaScriptNumericDigitUnit} from './numeric-digit-rules.js';
import {readJavaScriptUtf8StringUnits} from './utf8-string-unit-rules.js';

/** @param {number} unit @returns {number} */
export function readStringNumberDigit(unit) {
  return readJavaScriptNumericDigitUnit(unit);
}
/** @param {number} exponent @returns {bigint} */
function readStringNumberDecimalPower(exponent) {
  let value=1n,base=10n;
  while(exponent>0){
    if(exponent%2===1)value*=base;
    exponent=Math.floor(exponent/2);
    if(exponent>0)base*=base;
  }
  return value;
}
/** @param {bigint} value @returns {number} */
function readStringNumberExactMantissa(value) {
  let result=0,scale=1;
  while(value>0n){
    if(value%2n===1n)result+=scale;
    value/=2n;scale*=2;
  }
  return result;
}
/** @param {number} exponent @returns {number} */
function readStringNumberBinaryPower(exponent) {
  let result=1;
  while(exponent>0){result*=2;exponent-=1;}
  while(exponent<0){result/=2;exponent+=1;}
  return result;
}
/** Positive exact rational to binary64, including overflow and subnormals.
 * @param {bigint} numerator @param {bigint} denominator @returns {number} */
export function convertStringNumberRational(numerator,denominator) {
  if(numerator<0n||denominator<=0n)return NaN;
  if(numerator===0n)return 0;
  let exponent=0;
  if(numerator>=denominator){
    let scale=denominator;
    while(numerator>=scale*2n){
      scale*=2n;exponent+=1;
      if(exponent>=1024)return Infinity;
    }
  }else{
    let scale=numerator;
    while(scale<denominator){
      scale*=2n;exponent-=1;
      if(exponent< -1075)return 0;
    }
  }
  const shift=exponent>= -1022?52-exponent:1074;
  if(shift>=0){for(let index=0;index<shift;index+=1)numerator*=2n;}
  else {for(let index=0;index< -shift;index+=1)denominator*=2n;}
  // Long division uses only nonzero literal divisors. The same total body
  // therefore emits to formal targets without an opaque division precondition.
  let quotient=0n,remainder=numerator,scale=denominator,weight=1n;
  while(scale<=remainder){scale*=2n;weight*=2n;}
  while(weight>1n){
    scale/=2n;weight/=2n;
    if(remainder>=scale){remainder-=scale;quotient+=weight;}
  }
  const twice=remainder*2n;
  if(twice>denominator||twice===denominator&&quotient%2n===1n)quotient+=1n;
  return readStringNumberExactMantissa(quotient)*readStringNumberBinaryPower(-shift);
}
/** @param {number[]} units @param {number} start @param {number} end @returns {boolean} */
function acceptStringNumberInfinity(units,start,end) {
  if(end-start!==8)return false;
  const expected=[73,110,102,105,110,105,116,121];
  for(let index=0;index<expected.length;index+=1)if((units[start+index]?? -1)!==(expected[index]?? -2))return false;
  return true;
}
/** Complete StringNumericLiteral grammar, not parseFloat's prefix form.
 * @param {number[]} units @returns {number} */
export function convertJavaScriptStringNumberUnits(units) {
  for(let index=0;index<units.length;index+=1){
    const unit=units[index]?? -1;
    if(!Number.isSafeInteger(unit)||unit<0||unit>65535)return NaN;
  }
  let start=0,end=units.length;
  while(start<end&&acceptJavaScriptWhitespaceUnit(units[start]?? -1))start+=1;
  while(end>start&&acceptJavaScriptWhitespaceUnit(units[end-1]?? -1))end-=1;
  if(start===end)return 0;
  let negative=false,signed=false;
  const sign=units[start]?? -1;
  if(sign===43||sign===45){negative=sign===45;signed=true;start+=1;}
  if(start===end)return NaN;
  if(acceptStringNumberInfinity(units,start,end))return negative?-Infinity:Infinity;
  const integerDigits=[0n,1n,2n,3n,4n,5n,6n,7n,8n,9n,10n,11n,12n,13n,14n,15n];
  if(!signed&&(units[start]?? -1)===48&&end-start>=2){
    const marker=units[start+1]?? -1;
    const radix=marker===120||marker===88?16:marker===98||marker===66?2:marker===111||marker===79?8:0;
    if(radix!==0){
      let value=0n,overflow=false;
      const limit=readStringNumberDecimalPower(310);
      const base=radix===16?16n:radix===8?8n:2n;
      start+=2;if(start===end)return NaN;
      for(let index=start;index<end;index+=1){
        const digit=readStringNumberDigit(units[index]?? -1);
        if(digit<0||digit>=radix)return NaN;
        if(!overflow){value=value*base+(integerDigits[digit]??0n);overflow=value>limit;}
      }
      return overflow?Infinity:convertStringNumberRational(value,1n);
    }
  }
  let at=start,fraction=0,digits=0,significant=0,dot=false;
  while(at<end){
    const unit=units[at]?? -1;
    if(unit>=48&&unit<=57){
      digits+=1;if(dot)fraction+=1;
      if(unit!==48||significant>0)significant+=1;
      at+=1;continue;
    }
    if(unit===46&&!dot){dot=true;at+=1;continue;}
    break;
  }
  if(digits===0)return NaN;
  const decimalEnd=at;
  let exponent=0;
  const exponentLimit=units.length+10000;
  if(at<end&&((units[at]?? -1)===101||(units[at]?? -1)===69)){
    at+=1;let exponentNegative=false;
    const exponentSign=units[at]?? -1;
    if(exponentSign===43||exponentSign===45){exponentNegative=exponentSign===45;at+=1;}
    const first=at;
    while(at<end){
      const digit=(units[at]?? -1)-48;
      if(digit<0||digit>9)break;
      if(exponent<exponentLimit)exponent=Math.min(exponentLimit,exponent*10+digit);
      at+=1;
    }
    if(at===first)return NaN;
    if(exponentNegative)exponent=-exponent;
  }
  if(at!==end)return NaN;
  if(significant===0)return negative?-0:0;
  exponent-=fraction;
  const order=significant+exponent;
  if(order>310)return negative?-Infinity:Infinity;
  if(order< -325)return negative?-0:0;
  let mantissaEnd=decimalEnd;
  while(mantissaEnd>start){
    const unit=units[mantissaEnd-1]?? -1;
    if(unit===46){mantissaEnd-=1;continue;}
    if(unit!==48)break;
    mantissaEnd-=1;exponent+=1;
  }
  let mantissa=0n;
  for(let index=start;index<mantissaEnd;index+=1){
    const unit=units[index]?? -1;
    if(unit!==46)mantissa=mantissa*10n+(integerDigits[unit-48]??0n);
  }
  const numerator=exponent>=0?mantissa*readStringNumberDecimalPower(exponent):mantissa;
  const denominator=exponent<0?readStringNumberDecimalPower(-exponent):1n;
  const value=convertStringNumberRational(numerator,denominator);
  return negative?-value:value;
}

/** Explicit Number(BigInt) rounds once, unlike implicit ToNumber(BigInt).
 * @param {bigint} value @returns {number} */
export function convertJavaScriptBigIntegerNumber(value) {
  return value<0n?-convertStringNumberRational(-value,1n):convertStringNumberRational(value,1n);
}
/** UTF-8 transport for targets whose string value is a byte sequence.
 * @param {number[]} bytes @returns {number} */
export function convertJavaScriptUtf8StringNumberUnits(bytes) {
  return convertJavaScriptStringNumberUnits(readJavaScriptUtf8StringUnits(bytes));
}
/** @param {string} name @returns {string} */
export function readJavaScriptNumberBuiltinOperation(name) {
  if(name==='Number')return 'convertJavaScriptNumber';
  if(name==='isFinite')return 'testCoercedJavaScriptNumberFinite';
  return name==='isNaN'?'testCoercedJavaScriptNumberNaN':'';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptNumberConversionResult(operation) {
  if(operation==='convertJavaScriptNumber'||operation==='coerceJavaScriptNumber')return 'float';
  return operation==='testCoercedJavaScriptNumberFinite'||operation==='testCoercedJavaScriptNumberNaN'?'bool':'';
}
/** The optional host recurses over the actual element type, never guesses it.
 * @param {string} kind @returns {string} */
export function readJavaScriptNumberConversionForm(kind) {
  if(kind==='float'||kind==='bool'||kind==='string'||kind==='unit'||kind==='int'||kind==='nat'||kind==='option')return kind;
  return 'unsupported';
}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptNumberConversion(kind,operation) {
  return kind==='math'&&(operation==='convertJavaScriptNumber'||operation==='coerceJavaScriptNumber');
}
/** @param {string} operation @param {string} kind @returns {string} */
export function readJavaScriptNumberConversionFailure(operation,kind) {
  return operation==='coerceJavaScriptNumber'&&(kind==='int'||kind==='nat')?'Cannot convert a BigInt value to a number':'';
}
/** @param {string} kind @param {string} operation @param {string} argument @returns {boolean} */
export function requireJavaScriptNumberConversionUnits(kind,operation,argument) {
  return acceptLinkedJavaScriptNumberConversion(kind,operation)&&argument==='string';
}
/** Keep BigInt identity for ToNumeric; ToNumber has a different BigInt rule.
 * @param {string} language @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptPrimitiveNumberOperand(language,kind,element) {
  if(language!=='JavaScript')return '';
  if(kind==='bool'||kind==='string'||kind==='unit')return 'coerceJavaScriptNumber';
  return kind==='option'&&element==='string'?'coerceJavaScriptNumber':'';
}
/** @param {string} kind @returns {string} */
export function readJavaScriptPrimitiveNumericInferenceKind(kind) {
  return kind==='bool'||kind==='string'||kind==='unit'?'float':'';
}
/** @param {string} symbol @returns {string} */
export function readJavaScriptPrimitiveUnaryOperation(symbol) {
  return symbol==='+'?'coerceJavaScriptNumber':'';
}
/** @param {string} language @param {string} operation @returns {string} */
export function readJavaScriptMathematicalConversionForm(language,operation) {
  if(language!=='JavaScript')return '';
  if(operation==='max'||operation==='min')return 'variadic';
  return operation==='abs'||operation==='floor'||operation==='ceil'||operation==='trunc'||operation==='round'||operation==='sign'||operation==='sqrt'?'unary':'';
}
/** Only scalar emission; actual Option structure is handled by its owner.
 * @param {string} format @param {string} operation @param {string} form
 * @param {string} value @param {boolean} utf16 @returns {string} */
export function renderJavaScriptNumberScalarConversion(format,operation,form,value,utf16) {
  if(!acceptLinkedJavaScriptNumberConversion('math',operation)||readJavaScriptNumberConversionForm(form)==='unsupported'||form==='option')return '';
  if(format==='JavaScript')return operation==='convertJavaScriptNumber'?'Number('+value+')':'(+('+value+'))';
  if(format==='Lean'||format==='Rocq'){
    if(readJavaScriptNumberConversionFailure(operation,form))return '';
    if(form==='float')return value;
    if(form==='unit')return format==='Lean'?'(let _ := '+value+'; ((0 : Float) / 0))':'(let _ := '+value+' in PrimFloat.nan)';
    if(form==='bool')return format==='Lean'?'(if '+value+' then (1 : Float) else (0 : Float))':'(if '+value+' then 1%float else 0%float)';
    if(form==='int'||form==='nat')return '(MLStringNumber.convertInteger '+(form==='nat'?(format==='Lean'?'(Int.ofNat '+value+')':'(Z.of_N '+value+')'):value)+')';
    if(format==='Lean')return '(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] '+value+'))';
    return '(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) '+value+'))';
  }
  if(format!=='Rust')return '';
  if(form==='float')return value;
  if(form==='bool')return '(if '+value+' { 1.0f64 } else { 0.0f64 })';
  if(form==='unit')return '{ let _ = '+value+'; f64::NAN }';
  if(form==='int'||form==='nat')return operation==='coerceJavaScriptNumber'
    ? '{ let _ = '+value+'; crate::ml_source_exception::SourceException::from_text("TypeError", "Cannot convert a BigInt value to a number").raise() }'
    : 'crate::ml_string_number::convert_java_script_big_integer_number('+value+')';
  const units=utf16?'('+value+').0.iter().copied()':'('+value+').encode_utf16()';
  return 'crate::ml_string_number::convert_java_script_string_number_units(&'+units+'.map(f64::from).collect::<Vec<f64>>())';
}
/** @returns {string} */
export function readJavaScriptNumberConversionStatement() {
  return 'ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws';
}
