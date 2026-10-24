// Source recognition and exact operand roles; actual order algorithms are
// complete ordinary definitions in javascript-record-order-rules.js.
import {readJavaScriptOwnPropertyReceiverForm} from './javascript-own-property-rules.js';
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOwnEnumerationReceiverForm(kind,element){
 return kind==='fixed'?'empty':readJavaScriptOwnPropertyReceiverForm(kind,element);
}
/** @param {string} name @returns {string} */
export function readJavaScriptEnumerationBuiltinOperation(name){
 if(name==='Object.keys')return 'enumerateJavaScriptObjectKeys';
 if(name==='Object.values')return 'enumerateJavaScriptObjectValues';
 return name==='Object.entries'?'enumerateJavaScriptObjectEntries':'';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptEnumerationForm(operation){
 if(operation==='enumerateJavaScriptObjectKeys')return 'keys';
 if(operation==='enumerateJavaScriptObjectValues')return 'values';
 return operation==='enumerateJavaScriptObjectEntries'?'entries':'';
}
/** @param {string} operation @returns {number} */
export function readJavaScriptRecordOrderKernelArity(operation){
 if(operation==='createJavaScriptRecordPropertyOrder'||operation==='mergeJavaScriptRecordPropertyOrder'||operation==='readJavaScriptRecordPropertyKeys'||operation==='acceptJavaScriptRecordPropertyOrder'||operation==='readJavaScriptRecordPropertyKeyTexts')return 2;
 if(operation==='rebaseJavaScriptRecordPropertyOrder')return 3;
 return operation==='createJavaScriptDensePropertyKeyTexts'?1:0;
}
/** @param {string} operation @param {number} index @returns {string} */
export function readJavaScriptRecordOrderKernelParameter(operation,index){
 const count=readJavaScriptRecordOrderKernelArity(operation);
 if(!Number.isInteger(index)||index<0||index>=count)return '';
 if(operation==='createJavaScriptDensePropertyKeyTexts')return 'number';
 if(index===0||operation==='mergeJavaScriptRecordPropertyOrder'||operation==='rebaseJavaScriptRecordPropertyOrder'&&index===1)return 'name rows';
 return 'positions';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptRecordOrderKernelResult(operation){
 if(!readJavaScriptRecordOrderKernelArity(operation))return '';
 if(operation==='readJavaScriptRecordPropertyKeys')return 'name rows';
 if(operation==='acceptJavaScriptRecordPropertyOrder')return 'boolean';
 if(operation==='readJavaScriptRecordPropertyKeyTexts'||operation==='createJavaScriptDensePropertyKeyTexts')return 'key texts';
 return 'positions';
}
/** @param {string} expression @param {string} operation @param {string} kind
 * @param {boolean} ordered @param {boolean} reference @returns {boolean} */
export function acceptLinkedOrderedRecordOrderRead(expression,operation,kind,ordered,reference){
 return expression==='method'&&operation==='readJavaScriptRecordPropertyOrder'&&kind==='record'&&ordered&&!reference;
}
