// ECMAScript 2025, edition 16, CanBeHeldWeakly and WeakMap/WeakSet methods.
// Symbol classification comes from actual registry authority, never spelling.
import {readJavaScriptWeakCollectionOperationKind as classifyWeakCollectionOperation} from './javascript-weak-collection-operation-rules.js';
/** @param {string} operation @returns {string} */
export function readJavaScriptWeakCollectionOperationKind(operation){return classifyWeakCollectionOperation(operation);}
/** Missing facts remain an implementation boundary for OLD heap profiles;
 * they never certify an unregistered Symbol or invent a native TypeError.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptWeakKeyEligibility(heap,value){
 if(value.$==='object')return 'eligible';
 if(value.$!=='symbol')return 'ineligible';
 const facts=heap.symbolRegistrationModel==='actual global Symbol registry classification 1'?heap.symbolRegistrations??[]:[];let low=0,high=facts.length;while(low<high){const middle=Math.floor((low+high)/2),row=facts[middle];if(row!==undefined&&row.identity<value.identity)low=middle+1;else high=middle;}const candidate=facts[low],fact=candidate?.identity===value.identity?candidate:undefined;
 return fact===undefined?'missing symbol registration authority':fact.registered?'ineligible':'eligible';
}
