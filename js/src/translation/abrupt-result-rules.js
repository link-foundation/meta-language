import {acceptLinkedRustPanicValue} from './rust-panic-value-rules.js';
import {acceptLinkedJavaScriptSourceException} from './source-exception-rules.js';
/** Retyping a nonreturning computation preserves its complete operands and
 * effects. Unknown calls and potentially failing computations are not proofs.
 * @param {string} kind @param {string} operation @returns {string} */
export function readAbruptResultCoercionForm(kind,operation){
 if(acceptLinkedRustPanicValue(kind,operation)||kind==='abort'||acceptLinkedJavaScriptSourceException(kind,operation))return 'leaf';
 if(kind==='let'||kind==='print')return 'body';
 return kind==='if'?'branches':'';
}
