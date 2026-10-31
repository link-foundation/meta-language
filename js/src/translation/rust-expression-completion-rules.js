// Ordered source expression roles for ordinary Rust callable completions.
// These decisions are shared source, awaiting automatic native generation.
import {readConditionalChainOperation} from './conditional-chain-rules.js';
/** @param {string} kind @param {string} operation @returns {string} */
export function readRustExpressionCompletionForm(kind,operation){
 if(kind==='rustStandardSumTry')return 'propagate';
 if(kind==='lambda')return 'callable boundary';
 if(kind==='if')return 'conditional';
 if(kind==='binary'&&(operation==='and'||operation==='or'))return 'short circuit';
 if(kind==='match'||kind==='match1')return 'match';
 if(kind==='let')return 'binding';
 if(kind==='print')return 'print';
 if(kind==='math'&&readRustExpressionCompletionPacketOperation(operation)!=='')return 'packet';
 return readRustExpressionCompletionChildRoles(kind).length?'ordered':'opaque';
}
/** Ordered actual expression properties; arrays retain their actual order.
 * @param {string} kind @returns {string[]} */
export function readRustExpressionCompletionChildRoles(kind){
 if(kind==='app')return ['fn','args'];
 if(kind==='binary')return ['left','right'];
 if(kind==='unary'||kind==='cast')return ['arg'];
 if(kind==='tuple'||kind==='array')return ['items'];
 if(kind==='tupleIndex'||kind==='field')return ['object'];
 return [];
}
/** These leaves have no evaluation effects, traps or mutable reads in the
 * admitted immutable Rust source profile. Their contextual types stay intact.
 * @param {string} kind @returns {boolean} */
export function acceptRustExpressionCompletionInertLeaf(kind){
 return kind==='name'||kind==='num'||kind==='bool'||kind==='str'||kind==='unit'||kind==='functionValue';
}
/** @param {string} operation @returns {boolean} */
export function readRustExpressionCompletionShortCircuitValue(operation){return operation==='or';}
/** @param {string} kind @returns {number} */
export function readRustExpressionCompletionLimit(kind){
 return kind==='depth'?256:kind==='work'?100000:kind==='inspection'?1000000:0;
}
/** @param {string} operation @returns {string} */
export function readRustExpressionCompletionPacketOperation(operation){
 return operation===readConditionalChainOperation('success')?'success':operation===readConditionalChainOperation('failure')?'failure':operation===readConditionalChainOperation('result')?'result':'';
}
