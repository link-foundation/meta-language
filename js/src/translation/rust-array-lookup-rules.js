import {readRustPanicMessageSourceError,testRustOptionPanicConsumer,readRustOptionPanicMessage} from './rust-panic-value-rules.js';
import {readRustScalarArrayIndexReceiverError} from './rust-array-index-rules.js';
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustArrayLookupArity(method,ordinary){return !ordinary?-1:method==='get'?1:method==='first'||method==='last'?0:-1;}
/** @param {string} consumer @returns {boolean} */
export function acceptRustArrayLookupPresenceConsumer(consumer){return consumer==='is_some'||consumer==='is_none';}
/** @param {string} consumer @param {boolean} copied @returns {number} */
export function readRustArrayLookupConsumerArity(consumer,copied){return acceptRustArrayLookupPresenceConsumer(consumer)?0:copied&&(consumer==='map_or'||consumer==='map_or_else')?2:copied&&(consumer==='unwrap_or'||consumer==='unwrap_or_else'||testRustArrayLookupPredicateConsumer(consumer))?1:copied&&(consumer==='unwrap_or_default'||consumer==='unwrap')?0:copied&&consumer==='expect'?1:-1;}
/** @param {string} consumer @returns {boolean} */
export function testRustArrayLookupLazyFallback(consumer){return consumer==='unwrap_or_else'||consumer==='map_or_else';}
/** @param {string} kind @param {number} parameters @param {number} wrappers @param {boolean} knownResult @returns {string} */
export function readRustArrayLookupLazyFallbackError(kind,parameters,wrappers,knownResult){return kind==='function'&&parameters===0&&wrappers===0&&knownResult?'':'a lazy lookup fallback needs a known direct zero-argument callable result';}
/** @param {string} consumer @returns {boolean} */
export function testRustArrayLookupPredicateConsumer(consumer){return consumer==='is_some_and'||consumer==='is_none_or';}
/** @param {string} kind @param {number} parameters @param {number} wrappers @param {boolean} knownParameter @param {string} result @param {boolean} knownResult @returns {string} */
export function readRustArrayLookupPredicateError(kind,parameters,wrappers,knownParameter,result,knownResult){return kind==='function'&&parameters===1&&wrappers===0&&knownParameter&&result==='bool'&&knownResult?'':'a lookup predicate needs a known direct one-argument callable with a Boolean result';}
/** @param {string} kind @param {number} parameters @param {number} wrappers
 * @param {boolean} knownParameter @param {boolean} knownResult @returns {string} */
export function readRustArrayLookupMapperError(kind,parameters,wrappers,knownParameter,knownResult){return kind==='function'&&parameters===1&&wrappers===0&&knownParameter&&knownResult?'':'a lookup mapper needs a known direct one-argument callable with a complete result';}
/** Context comes from the actual standard scalar lookup, not closure text.
 * @param {string} consumer @param {boolean} receiver @param {boolean} element @returns {number} */
export function readRustArrayLookupCallableContextArity(consumer,receiver,element){
 if(!receiver||!element)return -1;
 return testRustArrayLookupLazyFallback(consumer)?0:testRustArrayLookupPredicateConsumer(consumer)||consumer==='map_or'?1:-1;
}
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
/** Standard String uses the existing immutable owned string value encoding;
 * arbitrary Clone implementations can carry observable user effects.
 * @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} ownerKind @param {string} owner @param {number} children
 * @returns {string} */
export function readRustArrayLookupClonedElementError(kind,bits,signed,ownerKind,owner,children){
 if(kind==='string'&&ownerKind==='library'&&owner==='std::string::String'&&children===0)return '';
 return kind==='unit'||ownerKind==='primitive'&&children===0&&readRustArrayLookupElementOwnerError(kind,bits,signed,owner)===''?'':'a cloned element needs its complete standard scalar or owned String source meaning';
}
/** @param {string} kind @param {boolean} standardString @returns {string} */
export function readRustArrayLookupDefaultTerm(kind,standardString){return kind==='unit'?'unit':kind==='string'&&standardString?'empty string':'primitive default';}
/** Lazy branch plans keep invalid reads and empty-last subtraction unreachable.
 * @param {string} method @returns {string[]} */
export function planRustArrayLookupCondition(method){return method==='get'?['index','length','lt']:method==='first'||method==='last'?['length','zero','gt']:[];}
/** @param {string} method @returns {string[]} */
export function planRustArrayLookupIndex(method){return method==='get'?['index']:method==='first'?['zero']:method==='last'?['length','one','sub']:[];}
/** @param {string} consumer @returns {string[]} */
export function planRustArrayLookupConsumer(consumer){return consumer==='is_some'?['condition']:consumer==='is_none'?['condition','not']:consumer==='is_some_and'?['condition','predicate','false','if']:consumer==='is_none_or'?['condition','predicate','true','if']:consumer==='unwrap'||consumer==='expect'||consumer==='unwrap_or'||consumer==='unwrap_or_else'||consumer==='unwrap_or_default'||consumer==='map_or'||consumer==='map_or_else'?['condition','selected','fallback','if']:[];}

/** @param {string} consumer @returns {boolean} */
export function testRustArrayLookupPanicConsumer(consumer){return testRustOptionPanicConsumer(consumer);}
/** @param {string} consumer @param {string} kind @returns {string} */
export function readRustArrayLookupPanicMessageError(consumer,kind){return consumer==='expect'&&kind!=='str'?'expect needs its complete literal str panic message; computed messages need a dynamic panic payload':'';}
/** Literal panic messages do not perform formatting; braces and NUL survive.
 * @param {string} consumer @param {string} message @returns {string} */
export function readRustArrayLookupPanicMessage(consumer,message){return readRustOptionPanicMessage(consumer,message);}

/** @param {string} consumer @param {string} kind @param {string} storage
 * @param {number} layers @param {boolean} known @param {boolean} references
 * @returns {string} */
export function readRustArrayLookupPanicValueError(consumer,kind,storage,layers,known,references){return readRustPanicMessageSourceError(consumer,kind,storage,layers,known,references);}
