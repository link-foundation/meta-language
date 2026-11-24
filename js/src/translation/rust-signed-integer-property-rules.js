import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {boolean} */
export function acceptRustSignedIntegerPropertyMethod(method,ordinary){return ordinary&&(method==='is_negative'||method==='is_positive'||method==='signum');}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustSignedIntegerPropertyReceiver(kind,bits,signed,owner,path,children,proven,layers){return kind==='fixed'&&signed&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** Postfix AST terms. Every operand occurrence refers to the same captured
 * receiver. These decisions introduce no integer arithmetic beyond -1.
 * @param {string} method @returns {string[]} */
export function planRustSignedIntegerProperty(method){
 if(method==='is_negative')return ['operand','zero','lt'];
 if(method==='is_positive')return ['operand','zero','gt'];
 if(method==='signum')return ['operand','zero','lt','negative unit','operand','zero','gt','positive unit','zero','if','if'];
 return [];
}
/** @param {string} method @returns {string} */
export function readRustSignedIntegerPropertyResult(method){return method==='signum'?'receiver':method==='is_negative'||method==='is_positive'?'bool':'';}
