// Complete owned String method dispatch is separate from primitive compound
// assignment and generic AddAssign. No byte/UTF-16 index is accepted here.
import {acceptRustStringAdditionOwner,acceptRustStringAdditionOperand} from './rust-string-addition-rules.js';
/** @param {string} method @returns {number} */
export function readRustExclusiveSourceStringMethodArity(method){return method==='push_str'?1:method==='clear'?0:-1;}
/** @param {string} kind @param {string} model @param {number} layers
 * @param {boolean} proven @param {string} owner @param {string} path
 * @param {number} children @returns {boolean} */
export function acceptRustExclusiveSourceStringReceiver(kind,model,layers,proven,owner,path,children){return kind==='string'&&model==='owned string'&&layers===0&&proven&&acceptRustStringAdditionOwner('left',owner,path,children);}
/** @param {string} kind @param {string} model @param {number} layers
 * @param {boolean} proven @param {boolean} references @param {string} owner
 * @param {string} path @param {number} children @returns {boolean} */
export function acceptRustExclusiveSourceStringSuffix(kind,model,layers,proven,references,owner,path,children){return acceptRustStringAdditionOperand('right',kind,model,layers,proven,references,acceptRustStringAdditionOwner('right',owner,path,children));}
