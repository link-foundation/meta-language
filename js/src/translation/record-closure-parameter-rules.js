// Closure provenance is explicit; it is never declared a named function.
import {planJavaScriptSourceRecordParameters} from './javascript-record-parameter-rules.js';
/** @param {boolean} method @param {boolean} expression @returns {string} */
export function readJavaScriptRecordClosureContext(method,expression){
 return method?'ordinary object method closure':expression?'ordinary function expression closure':'ordinary arrow function closure';
}
/** @param {string} context @param {boolean} asynchronous
 * @param {number} documentationParameters @returns {boolean} */
export function acceptJavaScriptRecordClosureParameterProfile(context,asynchronous,documentationParameters){
 return (context==='ordinary object method closure'||context==='ordinary function expression closure'||context==='ordinary arrow function closure')&&!asynchronous&&documentationParameters===0;
}
/** Actual pattern decisions reuse the complete original required-field source.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptRecordClosureParameterBindings(kinds,values,start,reserved){
 return planJavaScriptSourceRecordParameters(kinds,values,start,reserved);
}
