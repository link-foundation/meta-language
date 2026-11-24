// A complete continuous suffix belongs inside the nullish receiver branch.
// Parentheses end that suffix; returned nullish values are never mistaken for
// a skipped outer chain. The source-only constructor becomes ordinary core IR.
import {readJavaScriptOptionalCallResultForm} from './javascript-optional-call-rules.js';
/** @returns {string} */
export function readJavaScriptOptionalChainOperation(){return 'continueJavaScriptOptionalChain';}
/** @returns {string} */
export function readJavaScriptOptionalChainField(){return 'receiver';}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOptionalChainForm(kind,element){
 if(kind==='unit'||kind==='option'&&element==='unit')return 'absent';
 return kind==='option'?'guarded':'direct';
}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOptionalChainInferenceForm(kind,element){return kind==='var'?'unknown':readJavaScriptOptionalChainForm(kind,element);}
/** Every represented object/function is truthy, including an empty container.
 * Primitive false/zero/NaN/empty text require a tag test instead. In particular
 * typeof null is object and must never pass an object receiver's guard.
 * @param {string} kind @returns {string} */
export function readJavaScriptOptionalChainPresenceForm(kind){
 return kind==='record'||kind==='array'||kind==='tuple'||kind==='map'||kind==='set'||kind==='function'||kind==='data'?'truthy':'tag';
}
/** @param {string} kind @returns {string} */
export function readJavaScriptOptionalChainResultForm(kind){return readJavaScriptOptionalCallResultForm(kind);}
/** @param {number} argumentsCount @param {boolean} spread @param {string} name @returns {boolean} */
export function acceptJavaScriptOptionalChainOperands(argumentsCount,spread,name){return argumentsCount===1&&!spread&&name.length>0;}
