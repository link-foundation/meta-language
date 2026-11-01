import {readRustStandardSumSourceArgumentCount,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructors} from './rust-standard-sum-source-rules.js';
/** @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumCombinatorArity(owner,method){
 if(readRustStandardSumSourceArgumentCount(owner)<0)return -1;
 return method==='and'||method==='or'||method==='or_else'||owner==='std::option::Option'&&method==='xor'?1:-1;
}
/** @param {string} method @returns {string[]} */
export function readRustStandardSumCombinatorCaptureRoles(method){return method==='or_else'?['receiver','callback']:method==='and'||method==='or'||method==='xor'?['receiver','argument']:[];}
/** The actual receiver fixes the entire Option or/xor/or_else argument, but
 * generic-changing operations must obtain their other child from real syntax.
 * @param {string} owner @param {string} method @returns {boolean} */
export function acceptRustStandardSumCombinatorReceiverContext(owner,method){return owner==='std::option::Option'&&(method==='or'||method==='xor'||method==='or_else');}
/** @param {string} owner @param {string} method @returns {number[]} */
export function readRustStandardSumCombinatorRetainedArguments(owner,method){
 if(readRustStandardSumCombinatorArity(owner,method)<0)return [];
 if(method==='and')return owner==='std::result::Result'?[1]:[];
 return [0];
}
/** -1 is the actual zero-argument Option fallback; -2 is not callable.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumCombinatorCallbackArgument(owner,method){return readRustStandardSumCombinatorArity(owner,method)<0||method!=='or_else'?-2:owner==='std::option::Option'?-1:1;}
/** Direct Result constructor constraints only: the new source argument fills
 * its real field, while the other field comes from actual receiver semantics.
 * @param {string} owner @param {string} method @param {string} variant @returns {string[]} */
export function planRustStandardSumCombinatorRightArguments(owner,method,variant){
 if(owner!=='std::result::Result')return [];
 return method==='and'&&variant==='Ok'?['argument','left error']:method==='or'&&variant==='Err'?['left value','argument']:[];
}
/** @param {string} owner @param {string} left @param {string} method
 * @param {string} right @returns {string} */
export function readRustStandardSumCombinatorBranch(owner,left,method,right){
 if(readRustStandardSumCombinatorArity(owner,method)<0)return '';
 const field=readRustStandardSumSourceFieldArgument(owner,left);if(field===-2)return '';
 if(method==='and')return field===0?'right':'construct failure';
 if(method==='or'||method==='or_else')return field===0?owner==='std::option::Option'?'left':'construct success':method==='or_else'?'callback':'right';
 const other=readRustStandardSumSourceFieldArgument(owner,right);if(other===-2)return '';
 return field===0&&other!==0?'left':field!==0&&other===0?'right':'construct failure';
}
/** @param {string} owner @param {boolean} success @returns {string} */
export function readRustStandardSumCombinatorConstructor(owner,success){
 const variants=readRustStandardSumSourceConstructors(owner);
 for(let at=0;at<variants.length;at+=1)if((readRustStandardSumSourceFieldArgument(owner,variants[at])===0)===success)return variants[at];
 return '';
}
/** @param {string} owner @param {string} method @param {number} minor @returns {string} */
export function readRustStandardSumCombinatorVersionError(owner,method,minor){return readRustStandardSumCombinatorArity(owner,method)<0||Number.isInteger(minor)&&minor>=90&&minor<100?'':'standard sum combinator source semantics are uncovered outside Rust >=1.90.0 <1.100.0';}
