import {readRustCopyClosureParameterHeaderError} from './rust-copy-closure-rules.js';
import {readRequiredTextFact} from './fact-column-rules.js';
/** Only a real direct function pointer context supplies missing annotations.
 * @param {string} kind @param {boolean} copy @param {string} token @returns {boolean} */
export function acceptRustContextualCopyClosure(kind,copy,token){return kind==='function'&&copy&&(token==='|'||token==='||'||token==='move');}
/** Unlike the written-result profile, a concrete actual result may be inferred.
 * @param {boolean} ordinary @param {string[]} names @param {boolean[]} copies
 * @param {boolean} known @param {boolean} resultCopy @returns {string} */
export function readRustContextualCopyClosureHeaderError(ordinary,names,copies,known,resultCopy){
 const error=readRustCopyClosureParameterHeaderError(ordinary,names,copies);if(error)return error;
 if(!known)return 'the closure result requires its actual explicit type';
 return resultCopy?'':'the closure result requires a direct immutable Copy source type';
}
/** Actual syntax roles only; caller preserves raw-identifier distinctions.
 * -2 asks for a larger token window, -1 leaves ordinary parsing in charge.
 * @param {string[]} syntax @returns {number} */
export function findRustContextualClosureParenthesisCount(syntax){
 for(let index=0;index<syntax.length;index+=1){const token=readRequiredTextFact(syntax,index);if(token==='(')continue;return index>0&&(token==='|'||token==='||'||token==='move')?index:-1;}
 return -2;
}
/** A direct terminal pointer-result block retains one actual source expression.
 * @param {string} kind @param {boolean} copy @param {boolean} block
 * @param {string} first @returns {boolean} */
export function acceptRustContextualCopyPointerResultBlock(kind,copy,block,first){return kind==='function'&&copy&&block&&(first==='|'||first==='||'||first==='move'||first==='('||first==='return');}
