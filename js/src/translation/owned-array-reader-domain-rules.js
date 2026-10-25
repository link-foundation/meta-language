// Complete typed domain selection supplements the unchanged scalar contract.
import {acceptOwnedArrayReaderResult} from './owned-array-append-rules.js';
/** Distinct primitive domains cannot contain a shared input array identity.
 * Integer refinements stay in the same domain. Optional/compound/unknown
 * elements retain their alias obligations. @param {string} kind @returns {string} */
export function readOwnedArrayReaderElementDomain(kind){
 if(kind==='float')return 'Number';
 if(kind==='int'||kind==='nat'||kind==='fixed')return 'integer';
 if(kind==='string'||kind==='bool')return kind;
 return '';
}
/** Source bodies must still pass every original occurrence/escape check and
 * their complete declared types must subsequently check. This selection does
 * not admit arbitrary readers by name, return shape or observed examples.
 * @param {string} result @param {string} resultElement @param {string} parameter
 * @param {string} parameterElement @returns {boolean} */
export function acceptOwnedArrayReaderDomain(result,resultElement,parameter,parameterElement){
 if(acceptOwnedArrayReaderResult(result))return true;
 if(result!=='array'||parameter!=='array')return false;
 const left=readOwnedArrayReaderElementDomain(resultElement),right=readOwnedArrayReaderElementDomain(parameterElement);
 return left!==''&&right!==''&&left!==right;
}
