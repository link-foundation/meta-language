import {readRustStandardSumSourceFieldArgument} from './rust-standard-sum-source-rules.js';

/** @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumNestedArity(owner,method){
 return owner==='std::option::Option'&&(method==='flatten'||method==='transpose')||owner==='std::result::Result'&&method==='transpose'?0:-1;
}
/** @param {string} owner @param {string} method @param {string} inner
 * @returns {string} */
export function readRustStandardSumNestedProfile(owner,method,inner){
 if(owner==='std::option::Option'&&method==='flatten'&&inner==='std::option::Option')return 'option flatten';
 if(owner==='std::option::Option'&&method==='transpose'&&inner==='std::result::Result')return 'option result transpose';
 return owner==='std::result::Result'&&method==='transpose'&&inner==='std::option::Option'?'result option transpose':'';
}
/** Source roles and actual standard owner construction instructions.
 * @param {string} profile @returns {string[]} */
export function planRustStandardSumNestedResult(profile){
 return profile==='option flatten'?['inner']:profile==='option result transpose'?['inner success','std::option::Option','inner error','std::result::Result']:profile==='result option transpose'?['inner success','outer error','std::result::Result','std::option::Option']:[];
}
/** Payload is the actual selected constructor field. Child instructions use
 * the result's actual first type argument; all terms build existing ctors.
 * @param {string} profile @param {string} owner @param {string} constructor
 * @param {boolean} inner @returns {string[]} */
export function planRustStandardSumNestedBranch(profile,owner,constructor,inner){
 const field=readRustStandardSumSourceFieldArgument(owner,constructor);if(field===-2)return [];
 if(profile==='option flatten'&&owner==='std::option::Option')return field===-1?['result None']:inner?['payload','result Some']:['match inner'];
 if(profile==='option result transpose'){
  if(!inner&&owner==='std::option::Option')return field===-1?['child None','result Ok']:['match inner'];
  if(inner&&owner==='std::result::Result')return field===0?['payload','child Some','result Ok']:['payload','result Err'];
 }
 if(profile==='result option transpose'){
  if(!inner&&owner==='std::result::Result')return field===0?['match inner']:['payload','child Err','result Some'];
  if(inner&&owner==='std::option::Option')return field===-1?['result None']:['payload','child Ok','result Some'];
 }
 return [];
}
/** -1 is result root, 0 its actual first argument; -2 refuses.
 * @param {string} term @returns {number} */
export function readRustStandardSumNestedConstructionPosition(term){
 return ['result None','result Some','result Ok','result Err'].includes(term)?-1:['child None','child Some','child Ok','child Err'].includes(term)?0:-2;
}
/** @param {string} term @returns {string} */
export function readRustStandardSumNestedConstructionVariant(term){
 return term==='result None'||term==='child None'?'None':term==='result Some'||term==='child Some'?'Some':term==='result Ok'||term==='child Ok'?'Ok':term==='result Err'||term==='child Err'?'Err':'';
}
/** @param {string} owner @param {string} method @param {number} minor
 * @returns {string} */
export function readRustStandardSumNestedSourceVersionError(owner,method,minor){
 if(readRustStandardSumNestedArity(owner,method)<0)return '';
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumNestedSourceVersionRange(owner,method){return readRustStandardSumNestedArity(owner,method)<0?'':'>=1.90.0 <1.100.0';}
