// Whole actual-field selection; hosts transport selected types and values.
import {acceptRecordFieldName,readRecordFieldPosition} from './record-value-rules.js';
import {acceptJavaScriptRecordOrderNames} from './javascript-record-order-rules.js';
import {readRequiredNumberFact} from './fact-column-rules.js';
/** @returns {number[]} */
function refuseJavaScriptObjectRestName(){throw new TypeError('object rest needs every exact original field name');}
/** @param {number[][]} names @param {number} index @returns {number[]} */
function readJavaScriptObjectRestName(names,index){return names[index]??refuseJavaScriptObjectRestName();}
/** @returns {string} */
export function readJavaScriptObjectRestOperation(){return 'copyJavaScriptObjectRest';}
/** @param {string} kind @param {boolean} open @returns {string} */
export function readJavaScriptObjectRestOwnerForm(kind,open){
 if(kind==='var'||kind==='record'&&open)return 'wait';
 return kind==='record'?'copy':'unsupported';
}
/** @param {number[][]} names @param {number[][]} excluded @returns {number[]} */
export function selectJavaScriptObjectRestFields(names,excluded){
 if(!acceptJavaScriptRecordOrderNames(names))throw new TypeError('object rest needs every canonical actual own-field name');
 for(let index=0;index<excluded.length;index+=1)if(!acceptRecordFieldName(readJavaScriptObjectRestName(excluded,index)))throw new TypeError('object rest exclusions need exact original property names');
 let result=[];
 for(let index=0;index<names.length;index+=1)if(readRecordFieldPosition(excluded,readJavaScriptObjectRestName(names,index))<0)result=result.concat([index]);
 return result;
}
/** @param {number[]} plan @param {number} ordinal @returns {number} */
export function readJavaScriptObjectRestFieldPosition(plan,ordinal){return readRequiredNumberFact(plan,ordinal);}
