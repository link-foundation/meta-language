import {readRustStandardSumSourceFieldArgument,readRustStandardSumSourceArgumentCount} from './rust-standard-sum-source-rules.js';
/** @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumPredicateArity(owner,method){
 if(readRustStandardSumSourceArgumentCount(owner)<0)return -1;
 if(owner==='std::option::Option'&&(method==='is_some_and'||method==='is_none_or'))return 1;
 if(owner==='std::result::Result'&&(method==='is_ok_and'||method==='is_err_and'))return 1;
 return method==='map_or'||method==='map_or_else'?2:-1;
}
/** @param {string} method @returns {string[]} */
export function readRustStandardSumPredicateCaptureRoles(method){
 if(method==='map_or')return ['receiver','default','mapper'];
 if(method==='map_or_else')return ['receiver','fallback','mapper'];
 return ['is_some_and','is_none_or','is_ok_and','is_err_and'].includes(method)?['receiver','predicate']:[];
}
/** -1 is a real zero-argument callback; -2 refuses the role.
 * @param {string} owner @param {string} method @param {string} role @returns {number} */
export function readRustStandardSumPredicateCallbackArgument(owner,method,role){
 if(readRustStandardSumPredicateArity(owner,method)<0)return -2;
 if(role==='mapper')return method==='map_or'||method==='map_or_else'?0:-2;
 if(role==='fallback')return method==='map_or_else'?(owner==='std::option::Option'?-1:1):-2;
 return role==='predicate'&&readRustStandardSumPredicateArity(owner,method)===1?method==='is_err_and'?1:0:-2;
}
/** @param {string} owner @param {string} variant @param {string} method @returns {string} */
export function readRustStandardSumPredicateBranch(owner,variant,method){
 if(readRustStandardSumPredicateArity(owner,method)<0)return '';
 const field=readRustStandardSumSourceFieldArgument(owner,variant);if(field===-2)return '';
 if(method==='map_or'||method==='map_or_else')return field===0?'mapper':method==='map_or'?'default':'fallback';
 return field===readRustStandardSumPredicateCallbackArgument(owner,method,'predicate')?'predicate':method==='is_none_or'?'true':'false';
}
/** @param {string} owner @param {string} method @param {number} minor @returns {string} */
export function readRustStandardSumPredicateVersionError(owner,method,minor){
 return readRustStandardSumPredicateArity(owner,method)<0||Number.isInteger(minor)&&minor>=90&&minor<100?'':'standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumPredicateBorrowError(owner,method){
 return owner==='std::option::Option'&&method==='filter'?'filter needs its actual borrowed predicate parameter and complete reference lifetime model':'';
}
/** Pending is actual retained lambda syntax, never an admitted result type.
 * @param {string} kind @param {boolean} pending @param {number} parameters
 * @param {number} expected @param {number} layers @param {boolean} identity @returns {string} */
export function readRustStandardSumPredicateDeferredError(kind,pending,parameters,expected,layers,identity){
 return kind==='lambda'&&pending&&parameters===expected&&layers===0&&identity?'':'an unfinished callback needs its actual parameter origins, closure source identity and independently established sibling result';
}
/** Result inference uses complete actual sibling facts; no fallback scalar type.
 * @param {boolean} first @param {boolean} mapper @returns {string} */
export function readRustStandardSumPredicateResultSource(first,mapper){return first?'first result':mapper?'mapper result':'';}
