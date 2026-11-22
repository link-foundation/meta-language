// Standard Ord on the admitted closed primitive domains. Rust str order is
// unsigned UTF8 byte order, which agrees with Unicode scalar order on valid
// scalar text. Comparing UTF16 code units would put astral values incorrectly.
import {readRequiredNumberFact} from './fact-column-rules.js';
/** Operands have already passed the same closed-domain scalar validator.
 * @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {number} */
export function compareRustStandardCollectionScalars(left,right){
 if(left.$==='rustCollectionBoolean'){
  if(right.$==='rustCollectionBoolean')return left.value===right.value?0:left.value?1:-1;
  return 0;
 }
 if(left.$==='rustCollectionInteger'){
  if(right.$==='rustCollectionInteger')return left.value===right.value?0:left.value<right.value?-1:1;
  return 0;
 }
 if(left.$!=='rustCollectionString')return 0;
 if(right.$!=='rustCollectionString')return 0;
 return compareRustStandardCollectionTextUnits(left.units,right.units);
}
/** @param {number[]} left @param {number[]} right @returns {number} */
function compareRustStandardCollectionTextUnits(left,right){
 let first=0,second=0;
 while(first<left.length&&second<right.length){
  const firstUnit=readRequiredNumberFact(left,first),secondUnit=readRequiredNumberFact(right,second);
  const firstPair=firstUnit>=55296&&firstUnit<=56319,secondPair=secondUnit>=55296&&secondUnit<=56319;
  const firstScalar=firstPair?65536+(firstUnit-55296)*1024+readRequiredNumberFact(left,first+1)-56320:firstUnit;
  const secondScalar=secondPair?65536+(secondUnit-55296)*1024+readRequiredNumberFact(right,second+1)-56320:secondUnit;
  if(firstScalar!==secondScalar)return firstScalar<secondScalar?-1:1;
  first+=firstPair?2:1;second+=secondPair?2:1;
 }
 return first===left.length?(second===right.length?0:-1):1;
}
