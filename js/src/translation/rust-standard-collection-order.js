// Standard Ord on the admitted closed primitive domains. Rust str order is
// unsigned UTF8 byte order, which agrees with Unicode scalar order on valid
// scalar text. Comparing UTF16 code units would put astral values incorrectly.
/** Operands have already passed the same closed-domain scalar validator.
 * @param {RustStandardCollectionScalar} left @param {RustStandardCollectionScalar} right @returns {number} */
export function compareRustStandardCollectionScalars(left,right){
 if(left.$==='rustCollectionBoolean'&&right.$==='rustCollectionBoolean')return left.value===right.value?0:left.value?1:-1;
 if(left.$==='rustCollectionInteger'&&right.$==='rustCollectionInteger')return left.value===right.value?0:left.value<right.value?-1:1;
 if(left.$!=='rustCollectionString'||right.$!=='rustCollectionString')return 0;
 let first=0,second=0;
 while(first<left.units.length&&second<right.units.length){
  const firstUnit=left.units[first],secondUnit=right.units[second],firstPair=firstUnit>=55296&&firstUnit<=56319,secondPair=secondUnit>=55296&&secondUnit<=56319,firstScalar=firstPair?65536+(firstUnit-55296)*1024+left.units[first+1]-56320:firstUnit,secondScalar=secondPair?65536+(secondUnit-55296)*1024+right.units[second+1]-56320:secondUnit;
  if(firstScalar!==secondScalar)return firstScalar<secondScalar?-1:1;
  first+=firstPair?2:1;second+=secondPair?2:1;
 }
 return first===left.units.length?(second===right.units.length?0:-1):1;
}
