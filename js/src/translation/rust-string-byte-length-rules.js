import {readRequiredNumberFact} from './fact-column-rules.js';
/** Exact UTF-8 measure over a complete well-formed UTF-16 scalar encoding.
 * @param {number[]} units @returns {bigint} */
export function measureRustUtf8ByteLength(units){
 let count=0n;
 for(let index=0;index<units.length;index+=1){const unit=readRequiredNumberFact(units,index);if(!Number.isInteger(unit)||unit<0||unit>65535)throw new TypeError('a UTF-8 byte count needs valid UTF-16 units');
  if(unit<128)count+=1n;
  else if(unit<2048)count+=2n;
  else if(unit>=55296&&unit<=56319){if(index+1>=units.length)throw new TypeError('a UTF-8 byte count needs Unicode scalar values');const next=readRequiredNumberFact(units,index+1);if(!Number.isInteger(next)||next<56320||next>57343)throw new TypeError('a UTF-8 byte count needs Unicode scalar values');index+=1;count+=4n;}
  else if(unit>=56320&&unit<=57343)throw new TypeError('a UTF-8 byte count needs Unicode scalar values');
  else count+=3n;
 }
 return count;
}
/** @returns {string} */
export function readRustStringByteLengthOperation(){return 'measureRustStringByteLength';}
/** @param {string} expression @param {string} operation @returns {boolean} */
export function acceptRustStringByteLengthOperation(expression,operation){return expression==='math'&&operation===readRustStringByteLengthOperation();}
/** @param {string} language @param {boolean} units @param {number} count
 * @param {boolean} spread @param {string} kind @returns {string} */
export function readRustStringByteLengthArgumentError(language,units,count,spread,kind){
 if(language!=='Rust'||units)return 'the byte count needs the Rust Unicode scalar String representation';
 return count===1&&!spread&&kind==='string'?'':'a byte count needs one actual String operand';
}
/** Bindings select standard UTF-8 measures on the declared scalar domain.
 * @param {string} target @param {string} value @returns {string} */
export function renderRustStringByteLength(target,value){
 if(target==='JavaScript')return 'BigInt(encodeURIComponent('+value+').replace(/%[0-9A-F]{2}/g,"x").length)';
 if(target==='Rust')return '(('+value+').len() as u64)';
 if(target==='Lean')return '(String.utf8ByteSize '+value+')';
 return target==='Rocq'?'(N.of_nat (String.length '+value+'))':'';
}
