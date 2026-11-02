// Built-in primitive compound assignment only. Custom AddAssign dispatch and
// nonprimitive evaluation order retain their separate source obligations.
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
/** @param {string} token @returns {string} */
export function readRustExclusiveSourceCompoundOperation(token){return token==='+='?'add':token==='-='?'sub':token==='*='?'mul':token==='/='?'div':token==='%='?'rem':'';}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustExclusiveSourceCompoundPrimitive(kind,bits,signed,owner,path,children,proven,layers){return (kind==='fixed'||kind==='float')&&acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,children,proven,layers);}
