import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCopyOperation(operation){return operation==='Array.toReversed'||operation==='Array.with';}
/** Copying methods deliberately use realm ArrayCreate, never source species or
 * Array.prototype setters. Existing bytecode owns conversion, Get, allocation
 * and CreateDataPropertyOrThrow, including selected getter resumptions.
 * @param {bigint} identity @param {bigint} realm @param {string} operation
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptSemanticValue|undefined} arrayPrototype
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArrayCopyFunction(identity,realm,operation,roots,arrayPrototype){
 if(!testJavaScriptArrayCopyOperation(operation)||arrayPrototype?.$!=='object'||arrayPrototype.callable)return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];
 if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 code.push({$:'constant',destination:2,value:{$:'text',units:[108,101,110,103,116,104]}},{$:'ordinaryGet',destination:3,object:1,key:2,receiver:1},{$:'primitive',destination:3,operation:'coerce primitive number',arguments:[3]},{$:'primitive',destination:3,operation:'read array like length',arguments:[3]});
 if(operation==='Array.with')code.push({$:'argument',destination:4,index:0},{$:'primitive',destination:4,operation:'coerce primitive number',arguments:[4]},{$:'primitive',destination:4,operation:'read checked array replacement index',arguments:[3,4]},{$:'argument',destination:5,index:1});
 // Establish RangeError before resource/allocation requests. A fresh zero-length
 // Array then receives the same validated length without observable user hooks.
 code.push({$:'primitive',destination:3,operation:'require Array creation length',arguments:[3]},{$:'constant',destination:6,value:arrayPrototype},{$:'arrayAllocate',destination:7,prototype:6,length:0},{$:'ordinarySet',destination:12,object:7,key:2,value:3,receiver:7},{$:'requireWrite',success:12},{$:'constant',destination:8,value:{$:'number',value:0}},{$:'constant',destination:9,value:{$:'number',value:1}});
 const start=code.length;code.push({$:'numberBinary',destination:10,left:8,right:3,operator:'lessThan'});const exhausted=code.length;code.push({$:'branch',condition:10,yes:exhausted+1,no:0});
 let replacement=-1,skip=-1;
 if(operation==='Array.with'){
  code.push({$:'numberBinary',destination:10,left:8,right:4,operator:'equal'});replacement=code.length;code.push({$:'branch',condition:10,yes:0,no:replacement+1},{$:'propertyKey',destination:11,input:8,realm},{$:'ordinaryGet',destination:13,object:1,key:11,receiver:1});skip=code.length;code.push({$:'jump',target:0});const replace=code.length;code.push({$:'copy',destination:13,source:5});code[replacement]={$:'branch',condition:10,yes:replace,no:replacement+1};
 }else code.push({$:'numberBinary',destination:10,left:3,right:9,operator:'subtract'},{$:'numberBinary',destination:10,left:10,right:8,operator:'subtract'},{$:'propertyKey',destination:11,input:10,realm},{$:'ordinaryGet',destination:13,object:1,key:11,receiver:1});
 const define=code.length;if(skip>=0)code[skip]={$:'jump',target:define};
 code.push({$:'propertyKey',destination:11,input:8,realm},{$:'ordinaryDataProperty',destination:12,object:7,key:11,value:13,realm},{$:'numberBinary',destination:8,left:8,right:9,operator:'add'},{$:'jump',target:start});const finish=code.length;code.push({$:'return',source:7});code[exhausted]={$:'branch',condition:10,yes:exhausted+1,no:finish};
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:14,captures:[],instructions:code};
}
