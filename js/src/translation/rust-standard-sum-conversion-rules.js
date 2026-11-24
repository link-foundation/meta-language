import {readRustStandardSumSourceFieldArgument} from './rust-standard-sum-source-rules.js';

/** @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumConversionArity(owner,method){
 if(owner==='std::option::Option')return method==='ok_or'||method==='ok_or_else'?1:-1;
 return owner==='std::result::Result'?(method==='ok'||method==='err'?0:method==='map_err'?1:-1):-1;
}

/** @param {string} owner @param {string} method @param {number} minor
 * @returns {string} */
export function readRustStandardSumConversionSourceVersionError(owner,method,minor){
 if(readRustStandardSumConversionArity(owner,method)<0)return '';
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'standard sum conversion source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
}

/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumConversionSourceVersionRange(owner,method){
 return readRustStandardSumConversionArity(owner,method)<0?'':'>=1.90.0 <1.100.0';
}

/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumConversionOwner(owner,method){
 if(readRustStandardSumConversionArity(owner,method)<0)return '';
 return method==='ok'||method==='err'?'std::option::Option':'std::result::Result';
}

/** Actual child indices; -1 is the independent error argument/callback result.
 * @param {string} owner @param {string} method @returns {number[]} */
export function planRustStandardSumConversionArguments(owner,method){
 if(readRustStandardSumConversionArity(owner,method)<0)return [];
 return method==='ok'?[0]:method==='err'?[1]:[0,-1];
}

/** -2: no callback; -1: zero arguments; otherwise actual source child index.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumConversionCallbackArgument(owner,method){
 if(readRustStandardSumConversionArity(owner,method)<0)return -2;
 return method==='ok_or_else'?-1:method==='map_err'?1:-2;
}

/** @param {string} owner @param {string} constructor @param {string} method
 * @returns {string} */
export function readRustStandardSumConversionConstructor(owner,constructor,method){
 if(readRustStandardSumConversionArity(owner,method)<0)return '';
 const field=readRustStandardSumSourceFieldArgument(owner,constructor);
 if(field===-2)return '';
 if(method==='ok')return field===0?'Some':'None';
 if(method==='err')return field===1?'Some':'None';
 return field===0?'Ok':'Err';
}

/** Captures evaluate before the match; invocation occurs only in its branch.
 * @param {string} owner @param {string} constructor @param {string} method
 * @returns {string} */
export function readRustStandardSumConversionValue(owner,constructor,method){
 if(readRustStandardSumConversionArity(owner,method)<0)return '';
 const field=readRustStandardSumSourceFieldArgument(owner,constructor);
 if(field===-2)return '';
 if(method==='ok')return field===0?'payload':'empty';
 if(method==='err')return field===1?'payload':'empty';
 if(field===0)return 'payload';
 return method==='ok_or'?'argument':'invoke callback';
}

/** Real callable shape and independently checked result origins are premises.
 * @param {string} kind @param {number} parameters @param {number} expected
 * @param {number} layers @param {boolean} copy @param {boolean} result
 * @returns {string} */
export function readRustStandardSumConversionCallbackError(kind,parameters,expected,layers,copy,result){
 return kind==='function'&&parameters===expected&&layers===0&&copy&&result?'':'a sum conversion callback needs its actual direct Copy callable and complete parameter/result source origins';
}
