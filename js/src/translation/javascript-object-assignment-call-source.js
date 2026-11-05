import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';

// Object.assign holds its ToObject result before inspecting any source. Every
// source delegates to the shared strict-Set kernel; no host assignment runs.
/** @param {bigint} identity @param {bigint} realm
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptObjectAssignmentFunction(identity,realm,roots){
 /** @type {JavaScriptCallInstruction[]} */const code=[{$:'argument',destination:1,index:0}];
 if(!appendJavaScriptObjectCoercion(code,1,2,3,4,5,6,roots))return undefined;
 code.push({$:'argumentCount',destination:7},{$:'constant',destination:8,value:{$:'number',value:1}},{$:'constant',destination:9,value:{$:'number',value:1}});
 const loop=code.length;
 code.push({$:'numberBinary',destination:5,left:8,right:7,operator:'lessThan'},{$:'branch',condition:5,yes:loop+2,no:loop+6},{$:'argumentAt',destination:10,index:8},{$:'objectSpread',destination:11,target:2,input:10,realm,operation:'assignment'},{$:'numberBinary',destination:8,left:8,right:9,operator:'add'},{$:'jump',target:loop},{$:'return',source:2});
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:12,captures:[],instructions:code};
}
