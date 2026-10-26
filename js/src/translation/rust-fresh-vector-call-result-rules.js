// Closed source declarations establish postconditions together. In a finite
// returning evaluation, forwarding ultimately reaches a constructor tail;
// a recursive cycle alone cannot manufacture a returning value. Every actual
// declaration body is still checked before a complete program is admitted.
import {acceptRustOrdinaryFreshVectorResult,acceptRustKnownFreshVectorCall} from './rust-fresh-vector-result-rules.js';
/** @param {boolean} ordinary @param {string} model @param {number} wrappers @param {string} element @param {boolean} elementCopy @returns {boolean} */
export function acceptRustFreshVectorResultDeclaration(ordinary,model,wrappers,element,elementCopy){
 return elementCopy&&acceptRustOrdinaryFreshVectorResult(ordinary,model,wrappers,element,true);
}
/** @param {string} kind @param {string} model @param {number} wrappers @param {boolean} constructed @param {boolean} declared @param {boolean} called @returns {boolean} */
export function acceptRustFreshVectorResultCall(kind,model,wrappers,constructed,declared,called){
 return called&&acceptRustKnownFreshVectorCall(kind,model,wrappers,constructed||declared);
}
