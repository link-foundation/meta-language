import {readRustPrimitiveAssociatedArity} from './rust-primitive-associated-call-rules.js';
import {readRustPrimitiveDefaultTerm} from './rust-primitive-default-rules.js';
import {acceptRustWrappingShiftMethod} from './rust-wrapping-shift-rules.js';
import {acceptRustIntegerBitMethod,readRustIntegerBitResultOwner} from './rust-integer-bit-rules.js';
import {readRustIntegerMagnitudeResultOwner} from './rust-integer-magnitude-rules.js';
/** Exact scalar signatures only; escaping Option/Result and generic From keep
 * their independent obligations. No arithmetic implementation is repeated.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustPrimitiveFunctionValueArity(owner,method){
 if(method==='default')return readRustPrimitiveDefaultTerm(owner)===''?-1:0;
 if(method.startsWith('checked_'))return -1;
 return readRustPrimitiveAssociatedArity(owner,method);
}
/** @param {string} owner @param {string} method @param {number} index
 * @returns {string} */
export function readRustPrimitiveFunctionValueParameterOwner(owner,method,index){
 const arity=readRustPrimitiveFunctionValueArity(owner,method);
 if(index<0||index>=arity||index%1!==0)return '';
 return index===1&&acceptRustWrappingShiftMethod(method)?'u32':owner;
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustPrimitiveFunctionValueResultOwner(owner,method){
 if(readRustPrimitiveFunctionValueArity(owner,method)<0)return '';
 if(method==='is_negative'||method==='is_positive')return 'bool';
 if(acceptRustIntegerBitMethod(method))return readRustIntegerBitResultOwner(method,owner);
 if(method==='unsigned_abs'||method==='abs_diff')return readRustIntegerMagnitudeResultOwner(owner);
 return owner;
}
