import {requireJavaScriptArraySpeciesDirectory} from './javascript-call-code.js';
// Shared ToObject composition exposes the actual boxed receiver to existing
// callback, search and species algorithms. No native Array body is invoked.
import {testJavaScriptStandardArrayCallbackOperation,testJavaScriptStandardArrayTransformOperation} from './javascript-standard-operation-rules.js';
import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
import {testJavaScriptArraySearchOperation} from './javascript-array-index-rules.js';
/** @param {bigint} identity @param {bigint} realm @param {string} operation
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptArraySpeciesDirectory|undefined} [species]
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptBoxedArrayReadFunction(identity,realm,operation,roots,species){
 const callback=testJavaScriptStandardArrayCallbackOperation(operation),search=testJavaScriptArraySearchOperation(operation),transform=testJavaScriptStandardArrayTransformOperation(operation);
 if(!callback&&!search&&!transform||transform&&(species?.realm!==realm))return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 if(transform&&species!==undefined)code.push({$:'arrayTransform',destination:6,input:1,operation,species:species??requireJavaScriptArraySpeciesDirectory()});
 else if(callback)code.push({$:'arrayCallback',destination:6,input:1,operation});
 else code.push({$:'arraySearch',destination:6,input:1,operation});
 code.push({$:'return',source:6});return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:7,captures:[],instructions:code};
}
