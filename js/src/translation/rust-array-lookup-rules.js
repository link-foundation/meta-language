import {readRustScalarArrayIndexReceiverError} from './rust-array-index-rules.js';
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustArrayLookupArity(method,ordinary){return !ordinary?-1:method==='get'?1:method==='first'||method==='last'?0:-1;}
/** @param {string} consumer @returns {boolean} */
export function acceptRustArrayLookupPresenceConsumer(consumer){return consumer==='is_some'||consumer==='is_none';}
/** @param {string} consumer @param {boolean} copied @returns {number} */
export function readRustArrayLookupConsumerArity(consumer,copied){return acceptRustArrayLookupPresenceConsumer(consumer)?0:copied&&consumer==='unwrap_or'?1:copied&&consumer==='unwrap_or_default'?0:-1;}
/** @param {string} kind @param {string} element @param {string} model
 * @param {boolean} proven @param {boolean} immutable @param {boolean} copy
 * @param {boolean} copied @returns {string} */
export function readRustArrayLookupReceiverError(kind,element,model,proven,immutable,copy,copied){
 if(copied)return readRustScalarArrayIndexReceiverError(kind,element,model,proven,immutable,copy);
 return proven&&immutable&&(model==='owned vector'||model==='immutable slice')&&(kind==='array'||kind===''&&copy)?'':'the receiver needs a proven standard vector or immutable slice';
}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @returns {string} */
export function readRustArrayLookupElementOwnerError(kind,bits,signed,owner){return kind==='unit'||acceptRustPrimitiveFromSource(kind,bits,signed,'primitive',owner,0,true,0)?'':'a copied element needs its complete standard scalar source owner';}
/** Lazy branch plans keep invalid reads and empty-last subtraction unreachable.
 * @param {string} method @returns {string[]} */
export function planRustArrayLookupCondition(method){return method==='get'?['index','length','lt']:method==='first'||method==='last'?['length','zero','gt']:[];}
/** @param {string} method @returns {string[]} */
export function planRustArrayLookupIndex(method){return method==='get'?['index']:method==='first'?['zero']:method==='last'?['length','one','sub']:[];}
/** @param {string} consumer @returns {string[]} */
export function planRustArrayLookupConsumer(consumer){return consumer==='is_some'?['condition']:consumer==='is_none'?['condition','not']:consumer==='unwrap_or'||consumer==='unwrap_or_default'?['condition','selected','fallback','if']:[];}
