// Exact BigInt conversion algorithms. A zero/one element column distinguishes
// a failed conversion from every integer, including zero. Hosts select the
// language's syntax/range/type exception and retain its actual source message.
import {acceptJavaScriptWhitespaceUnit} from './javascript-whitespace-rules.js';
import {readJavaScriptNumericDigitUnit} from './numeric-digit-rules.js';
import {readJavaScriptUtf8StringUnits} from './utf8-string-unit-rules.js';

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
  if(operation==='divideJavaScriptBigIntegerResult')return 'division';
  if(operation==='parseJavaScriptStringBigInteger')return 'string';
  if(operation==='convertJavaScriptBooleanBigIntegerValue')return 'boolean';
  return operation==='convertJavaScriptNumberBigIntegerResult'?'number':'';
}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptBigIntegerResult(kind,operation) {
  return kind==='math'&&readJavaScriptBigIntegerResultOperation(operation)!=='';
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
