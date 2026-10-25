// Distinct actual ordinary callable contexts; no named-declaration alias.
import {planJavaScriptSourceRecordParameters} from './javascript-record-parameter-rules.js';
/** @param {boolean} expression @returns {string} */
export function readJavaScriptRecordConstantContext(expression){return expression?'ordinary constant function expression':'ordinary constant arrow function';}
/** @param {string} context @param {boolean} asynchronous
 * @param {number} documentationParameters @returns {boolean} */
export function acceptJavaScriptRecordCallableParameterProfile(context,asynchronous,documentationParameters){
 return (context==='ordinary constant function expression'||context==='ordinary constant arrow function'||context==='ordinary namespace object method')&&!asynchronous&&documentationParameters===0;
}
/** @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptRecordCallableParameterBindings(kinds,values,start,reserved){return planJavaScriptSourceRecordParameters(kinds,values,start,reserved);}
