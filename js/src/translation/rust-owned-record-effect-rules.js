import {readRustExpressionCompletionForm,readRustExpressionCompletionChildRoles,readRustExpressionCompletionLimit} from './rust-expression-completion-rules.js';
/** @param {string} kind @param {string} operation @returns {string} */
export function readRustOwnedRecordEffectForm(kind,operation){if(kind==='math'&&operation==='rustOwnedRecordStateRead')return 'read state';if(kind==='math'&&operation==='rustOwnedRecordStateWrite')return 'write state';return readRustExpressionCompletionForm(kind,operation);}
/** All added roles are ordinary source operand order. Lazy branches remain
 * separate forms and callable bodies remain separate callable boundaries.
 * @param {string} kind @returns {string[]} */
export function readRustOwnedRecordEffectChildren(kind){
 const shared=readRustExpressionCompletionChildRoles(kind);if(shared.length)return shared;
 if(kind==='method')return ['object','args'];if(kind==='math')return ['args'];if(kind==='index')return ['object','index'];
 if(['toString','show','booleanValue','numberValue','typeOf','arrayIsArray'].includes(kind))return ['arg'];
 if(['stringPoints','copySlice','length','forOfIterable','optionalField'].includes(kind))return ['object'];
 if(kind==='some'||kind==='flattenOption')return ['value'];if(kind==='arrayFrom')return ['args'];
 if(kind==='stringTest')return ['object','search'];if(kind==='stringMap')return ['object'];
 return [];
}
/** @param {string} operation @returns {string} */
export function readRustOwnedRecordEffectOperation(operation){return operation==='read'?'rustOwnedRecordStateRead':operation==='write'?'rustOwnedRecordStateWrite':'';}
/** @param {string} kind @returns {number} */
export function readRustOwnedRecordEffectLimit(kind){return readRustExpressionCompletionLimit(kind);}
/** @param {boolean} sameCallable @param {boolean} declared @returns {boolean} */
export function acceptRustOwnedRecordEffectScope(sameCallable,declared){return sameCallable&&declared;}
/** Structural datatype/array equality remains in the proposition algebra.
 * These actual scalar domains already have the same expression comparison.
 * @param {string} kind @returns {boolean} */
export function acceptRustOwnedRecordAssertionComparison(kind){return kind==='bool'||kind==='fixed'||kind==='nat'||kind==='int'||kind==='string'||kind==='float';}
