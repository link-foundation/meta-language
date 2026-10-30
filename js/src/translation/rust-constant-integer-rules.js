import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {testRustIntegerRangeCanonicalDecimal} from './rust-integer-range-rules.js';
/** Canonical, already checked decimal input, without binary64 conversion.
 * @param {string} text @returns {bigint} */
function readRustConstantIntegerDecimal(text){
 const negative=text.startsWith('-'),digits=negative?text.slice(1):text;
 let result=0n;
 for(let at=0;at<digits.length;at+=1){const digit='0123456789'.indexOf(digits.charAt(at));let value=0n;for(let count=0;count<digit;count+=1)value+=1n;result=result*10n+value;}
 return negative?-result:result;
}
/** @param {string} operation @param {string} left @param {string} right
 * @param {string} minimum @param {string} maximum @param {number} bits
 * @returns {string[]} */
export function evaluateRustConstantIntegerOperation(operation,left,right,minimum,maximum,bits){
 if(!testRustIntegerRangeCanonicalDecimal(left)||!testRustIntegerRangeCanonicalDecimal(right)||!testRustIntegerRangeCanonicalDecimal(minimum)||!testRustIntegerRangeCanonicalDecimal(maximum)||![8,16,32,64,128].includes(bits))return ['error','constant integer evaluation needs exact operands and an actual integer domain'];
 const a=readRustConstantIntegerDecimal(left),b=readRustConstantIntegerDecimal(right),low=readRustConstantIntegerDecimal(minimum),high=readRustConstantIntegerDecimal(maximum);
 let modulus=1n;for(let at=0;at<bits;at+=1)modulus*=2n;
 const expectedLow=low<0n?-(modulus/2n):0n,expectedHigh=low<0n?modulus/2n-1n:modulus-1n;
 if(low!==expectedLow||high!==expectedHigh)return ['error','constant integer evaluation needs an actual integer domain'];
 if(operation!=='cast'&&(a<low||a>high))return ['error','a constant integer operand is outside its actual source domain'];
 const shift=operation==='shl'||operation==='shr',unary=operation==='neg'||operation==='bitnot'||operation==='cast';
 if(!shift&&!unary&&(b<low||b>high))return ['error','a constant integer operand is outside its actual source domain'];
 if(operation==='eq')return ['boolean',a===b?'true':'false'];
 if(operation==='ne')return ['boolean',a!==b?'true':'false'];
 if(operation==='lt')return ['boolean',a<b?'true':'false'];
 if(operation==='le')return ['boolean',a<=b?'true':'false'];
 if(operation==='gt')return ['boolean',a>b?'true':'false'];
 if(operation==='ge')return ['boolean',a>=b?'true':'false'];
 let value=0n;
 if(operation==='add')value=a+b;
 else if(operation==='sub')value=a-b;
 else if(operation==='mul')value=a*b;
 else if(operation==='div'||operation==='rem'){
  if(b===0n)return ['error','division by zero in a constant integer expression'];
  if(low<0n&&a===low&&b===-1n)return ['error','overflow in a constant integer expression'];
  value=operation==='div'?a/b:a%b;
 }else if(operation==='neg'){
  if(low===0n)return ['error','an unsigned constant integer cannot use unary negation'];
  value=-a;
 }else if(operation==='bitnot')value=low===0n?(~a)&high:~a;
 else if(operation==='bitand')value=a&b;
 else if(operation==='bitor')value=a|b;
 else if(operation==='bitxor')value=a^b;
 else if(shift){
  let width=0n;for(let at=0;at<bits;at+=1)width+=1n;
  if(b<0n||b>=width)return ['error','overflow in a constant integer shift'];
  value=operation==='shl'?a<<b:a>>b;
 }else if(operation==='cast')value=a;
 else return ['error','the constant integer operation needs its source interpretation'];
 if(operation==='cast'||operation==='shl'){const modulus=high-low+1n;value=(value%modulus+modulus)%modulus;if(value>high)value-=modulus;}
 if(value<low||value>high)return ['error','overflow in a constant integer expression'];
 return ['integer',String(value)];
}
/** @param {string} operation @param {boolean} left @param {boolean} right
 * @returns {string[]} */
export function evaluateRustConstantBooleanOperation(operation,left,right){
 if(operation==='not')return ['boolean',left?'false':'true'];
 if(operation==='and')return ['boolean',left&&right?'true':'false'];
 if(operation==='or')return ['boolean',left||right?'true':'false'];
 if(operation==='eq')return ['boolean',left===right?'true':'false'];
 if(operation==='ne')return ['boolean',left!==right?'true':'false'];
 return ['error','the constant Boolean operation needs its source interpretation'];
}
/** @param {string} operation @param {boolean} left @returns {boolean} */
export function requireRustConstantBooleanRight(operation,left){return operation==='and'?left:operation==='or'?!left:true;}
/** @param {number} depth @param {number} steps @param {boolean} cycle
 * @returns {string} */
export function readRustConstantPatternEvaluationError(depth,steps,cycle){
 if(cycle)return 'a constant pattern dependency cannot evaluate itself recursively';
 return Number.isInteger(depth)&&depth>=0&&depth<=256&&Number.isInteger(steps)&&steps>=0&&steps<=65536?'':'constant pattern evaluation exceeds its source resource budget';
}

/** Only a direct actual integer alias supplies primitive associated constants.
 * @param {string} kind @param {string} ownerKind @param {string} owner
 * @param {number} argumentsCount @param {number} wrappers @param {string} bits
 * @param {boolean} signed @returns {string} */
export function readRustConstantPrimitiveAliasOwner(kind,ownerKind,owner,argumentsCount,wrappers,bits,signed){
 return kind==='fixed'&&ownerKind==='primitive'&&argumentsCount===0&&wrappers===0&&acceptRustSourceSignaturePrimitive(kind,owner,bits,signed)?owner:'';
}
