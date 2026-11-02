import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {bigint} identity @param {bigint} realm
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptSemanticValue|undefined} tagKey @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptObjectStringFunction(identity,realm,roots,tagKey){
 if(tagKey?.$!=='symbol')return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[{$:'constant',destination:2,value:{$:'undefined'}},{$:'strictEqual',destination:3,left:0,right:2},{$:'branch',condition:3,yes:0,no:3},{$:'constant',destination:2,value:{$:'null'}},{$:'strictEqual',destination:3,left:0,right:2},{$:'branch',condition:3,yes:0,no:6}];
 if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 code.push({$:'objectObservation',destination:6,operation:'Object.builtinTag',input:1,other:1},{$:'constant',destination:2,value:tagKey},{$:'ordinaryGet',destination:7,object:1,key:2,receiver:1},{$:'primitive',destination:2,operation:'read semantic type',arguments:[7]},{$:'constant',destination:3,value:{$:'text',units:[115,116,114,105,110,103]}},{$:'strictEqual',destination:4,left:2,right:3});
 const tagBranch=code.length;code.push({$:'branch',condition:4,yes:tagBranch+1,no:tagBranch+2},{$:'copy',destination:6,source:7},{$:'constant',destination:2,value:{$:'text',units:[91,111,98,106,101,99,116,32]}},{$:'primitive',destination:6,operation:'concatenate text',arguments:[2,6]},{$:'constant',destination:2,value:{$:'text',units:[93]}},{$:'primitive',destination:6,operation:'concatenate text',arguments:[6,2]},{$:'return',source:6});
 const absent=code.length;code.push({$:'constant',destination:6,value:{$:'text',units:[91,111,98,106,101,99,116,32,85,110,100,101,102,105,110,101,100,93]}},{$:'return',source:6});const nullish=code.length;code.push({$:'constant',destination:6,value:{$:'text',units:[91,111,98,106,101,99,116,32,78,117,108,108,93]}},{$:'return',source:6});code[2]={$:'branch',condition:3,yes:absent,no:3};code[5]={$:'branch',condition:3,yes:nullish,no:6};
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:8,captures:[],instructions:code};
}
/** Array toString Gets the current join once. A noncallable join uses the
 * actual realm Object toString owner, without looking it up on the receiver.
 * @param {bigint} identity @param {bigint} realm
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptSemanticValue|undefined} objectString @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArrayStringFunction(identity,realm,roots,objectString){
 if(objectString?.$!=='object'||!objectString.callable)return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 code.push({$:'constant',destination:2,value:{$:'text',units:[106,111,105,110]}},{$:'ordinaryGet',destination:6,object:1,key:2,receiver:1},{$:'primitive',destination:2,operation:'read semantic type',arguments:[6]},{$:'constant',destination:3,value:{$:'text',units:[102,117,110,99,116,105,111,110]}},{$:'strictEqual',destination:4,left:2,right:3});const branch=code.length;code.push({$:'branch',condition:4,yes:branch+2,no:branch+1},{$:'constant',destination:6,value:objectString},{$:'call',destination:7,callee:6,receiver:1,arguments:[]},{$:'return',source:7});
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:8,captures:[],instructions:code};
}
