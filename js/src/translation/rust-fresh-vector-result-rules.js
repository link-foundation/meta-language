import {acceptRustFreshOwnedVectorArgument} from './rust-fresh-vector-argument-rules.js';
/** Fresh owned results have no borrowed element or outlives obligation. The
 * existing explicit result profile retains its own independent interpretation.
 * @param {boolean} ordinary @param {string} model @param {number} wrappers
 * @param {string} element @param {boolean} fresh @returns {boolean} */
export function acceptRustOrdinaryFreshVectorResult(ordinary,model,wrappers,element,fresh){return acceptRustFreshOwnedVectorArgument(ordinary,model,wrappers,element,fresh);}
/** Only actual source-call evidence can propagate a checked result declaration.
 * Local bindings clear that evidence; a bare function value never records a call.
 * @param {string} kind @param {string} model @param {number} wrappers
 * @param {boolean} checked @returns {boolean} */
export function acceptRustKnownFreshVectorCall(kind,model,wrappers,checked){return checked&&(kind==='app'||kind==='name')&&model==='owned vector'&&wrappers===0;}
