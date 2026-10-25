import {readRequiredBooleanFact,readRequiredTextFact} from './fact-column-rules.js';
/** A distinct ordinary immutable source profile, with no inferred ownership.
 * @param {boolean} ordinary @param {string[]} names @param {boolean[]} copies
 * @param {boolean} resultWritten @param {boolean} resultCopy @returns {string} */
export function readRustCopyClosureHeaderError(ordinary,names,copies,resultWritten,resultCopy){
 if(!ordinary)return 'closure ownership requires its separate active source profile';
 if(names.length!==copies.length||names.length>1024)return 'the complete typed closure header exceeds its source budget';
 for(let index=0;index<names.length;index+=1){if(!readRequiredBooleanFact(copies,index))return 'closure parameters require direct immutable Copy source types';for(let before=0;before<index;before+=1)if(readRequiredTextFact(names,before)===readRequiredTextFact(names,index))return 'closure parameters require distinct actual bindings';}
 if(!resultWritten)return 'the closure result requires its actual explicit type';
 return resultCopy?'':'the closure result requires a direct immutable Copy source type';
}
/** @param {boolean} collecting @param {boolean[]} copies @param {number} assigned @returns {string} */
export function readRustCopyClosureCaptureError(collecting,copies,assigned){
 if(assigned!==0)return 'mutable closure captures require their shared ownership interpretation';
 if(!collecting)for(let index=0;index<copies.length;index+=1)if(!readRequiredBooleanFact(copies,index))return 'closure captures require proven direct immutable Copy source values';
 return '';
}
/** @param {string} kind @returns {boolean} */
export function acceptRustCopyClosureInvocation(kind){return kind==='function';}
/** Anonymous closure identity never follows from equal callable signatures.
 * @param {boolean} first @param {boolean} second @param {boolean} same
 * @returns {string} */
export function readRustCopyClosureJoinError(first,second,same){return (first||second)&&!same?'distinct anonymous closure types require an actual source coercion':'';}
