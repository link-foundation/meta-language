import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArraySortOperation(operation){return operation==='Array.sort'||operation==='Array.toSorted';}
/** Stable sorting through existing ordinary operations, calls and scalar
 * continuations. The hidden dense working Array is never given to user code.
 * Comparator invocation order is an implementation choice; no V8 comparison
 * trace is claimed. Every loop remains resumable at existing VM coordinates.
 * @param {bigint} identity @param {bigint} realm @param {string} operation
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptSemanticValue|undefined} arrayPrototype
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArraySortFunction(identity,realm,operation,roots,arrayPrototype){
 if(!testJavaScriptArraySortOperation(operation)||arrayPrototype?.$!=='object'||arrayPrototype.callable)return undefined;
 const copied=operation==='Array.toSorted';
 /** @type {JavaScriptCallInstruction[]} */const code=[{$:'argument',destination:2,index:0},{$:'constant',destination:3,value:{$:'undefined'}},{$:'strictEqual',destination:4,left:2,right:3}];
 const comparator=code.length;code.push({$:'branch',condition:4,yes:comparator+2,no:comparator+1},{$:'primitive',destination:2,operation:'require callable target',arguments:[2]});
 if(!appendJavaScriptObjectCoercion(code,0,1,23,24,25,26,roots))return undefined;
 code.push({$:'constant',destination:5,value:{$:'text',units:[108,101,110,103,116,104]}},{$:'ordinaryGet',destination:6,object:1,key:5,receiver:1},{$:'primitive',destination:6,operation:'coerce primitive number',arguments:[6]},{$:'primitive',destination:6,operation:'read array like length',arguments:[6]});
 if(copied)code.push({$:'primitive',destination:6,operation:'require Array creation length',arguments:[6]});
 code.push({$:'constant',destination:7,value:arrayPrototype},{$:'arrayAllocate',destination:8,prototype:7,length:0},{$:'constant',destination:9,value:{$:'number',value:0}},{$:'constant',destination:10,value:{$:'number',value:0}},{$:'constant',destination:11,value:{$:'number',value:1}},{$:'constant',destination:12,value:{$:'number',value:0}});
 if(copied)code.push({$:'ordinarySet',destination:13,object:8,key:5,value:6,receiver:8},{$:'requireWrite',success:13});
 // Capture every selected source value before invoking a comparator or writing
 // back. Sort skips absent properties; toSorted reads through holes as undefined.
 const capture=code.length;code.push({$:'numberBinary',destination:13,left:9,right:6,operator:'lessThan'});const captured=code.length;code.push({$:'branch',condition:13,yes:captured+1,no:0},{$:'propertyKey',destination:14,input:9,realm});
 let absent=-1;if(!copied){code.push({$:'ordinaryHas',destination:13,object:1,key:14});absent=code.length;code.push({$:'branch',condition:13,yes:absent+1,no:0});}
 code.push({$:'ordinaryGet',destination:15,object:1,key:14,receiver:1},{$:'propertyKey',destination:14,input:10,realm},{$:'ordinaryDataProperty',destination:13,object:8,key:14,value:15,realm},{$:'numberBinary',destination:10,left:10,right:11,operator:'add'});
 const advance=code.length;code.push({$:'numberBinary',destination:9,left:9,right:11,operator:'add'},{$:'jump',target:capture});if(absent>=0)code[absent]={$:'branch',condition:13,yes:absent+1,no:advance};
 const ready=code.length;code[captured]={$:'branch',condition:13,yes:captured+1,no:ready};code.push({$:'copy',destination:9,source:11});
 const outer=code.length;code.push({$:'numberBinary',destination:13,left:9,right:10,operator:'lessThan'});const sorted=code.length;code.push({$:'branch',condition:13,yes:sorted+1,no:0},{$:'propertyKey',destination:14,input:9,realm},{$:'ordinaryGet',destination:17,object:8,key:14,receiver:8},{$:'numberBinary',destination:16,left:9,right:11,operator:'subtract'});
 const inner=code.length;code.push({$:'numberBinary',destination:13,left:16,right:12,operator:'lessThan'});const beginning=code.length;code.push({$:'branch',condition:13,yes:0,no:beginning+1},{$:'propertyKey',destination:14,input:16,realm},{$:'ordinaryGet',destination:18,object:8,key:14,receiver:8},{$:'strictEqual',destination:13,left:18,right:3});
 const leftUndefined=code.length;code.push({$:'branch',condition:13,yes:0,no:leftUndefined+1},{$:'strictEqual',destination:13,left:17,right:3});const rightUndefined=code.length;code.push({$:'branch',condition:13,yes:0,no:rightUndefined+1},{$:'strictEqual',destination:13,left:2,right:3});const defaultCompare=code.length;code.push({$:'branch',condition:13,yes:0,no:defaultCompare+1});
 code.push({$:'call',destination:19,callee:2,receiver:3,arguments:[18,17]},{$:'primitive',destination:19,operation:'coerce primitive number',arguments:[19]});const compared=code.length;code.push({$:'jump',target:0});
 const stringCompare=code.length;code.push({$:'primitive',destination:20,operation:'convert primitive text',arguments:[18]},{$:'primitive',destination:21,operation:'convert primitive text',arguments:[17]},{$:'primitive',destination:19,operation:'compare text',arguments:[20,21]});
 const compareResult=code.length;code.push({$:'numberBinary',destination:13,left:12,right:19,operator:'lessThan'});const move=code.length;code.push({$:'branch',condition:13,yes:0,no:0});
 const undefinedLeft=code.length;code.push({$:'strictEqual',destination:13,left:17,right:3});const bothUndefined=code.length;code.push({$:'branch',condition:13,yes:0,no:0});
 const shift=code.length;code.push({$:'numberBinary',destination:22,left:16,right:11,operator:'add'},{$:'propertyKey',destination:14,input:22,realm},{$:'ordinaryDataProperty',destination:13,object:8,key:14,value:18,realm},{$:'numberBinary',destination:16,left:16,right:11,operator:'subtract'},{$:'jump',target:inner});
 const insert=code.length;code.push({$:'numberBinary',destination:22,left:16,right:11,operator:'add'},{$:'propertyKey',destination:14,input:22,realm},{$:'ordinaryDataProperty',destination:13,object:8,key:14,value:17,realm},{$:'numberBinary',destination:9,left:9,right:11,operator:'add'},{$:'jump',target:outer});
 code[beginning]={$:'branch',condition:13,yes:insert,no:beginning+1};code[leftUndefined]={$:'branch',condition:13,yes:undefinedLeft,no:leftUndefined+1};code[rightUndefined]={$:'branch',condition:13,yes:insert,no:rightUndefined+1};code[defaultCompare]={$:'branch',condition:13,yes:stringCompare,no:defaultCompare+1};code[compared]={$:'jump',target:compareResult};code[move]={$:'branch',condition:13,yes:shift,no:insert};code[bothUndefined]={$:'branch',condition:13,yes:insert,no:shift};
 const publish=code.length;code[sorted]={$:'branch',condition:13,yes:sorted+1,no:publish};
 if(copied)code.push({$:'return',source:8});
 else{
  code.push({$:'copy',destination:9,source:12});const writing=code.length;code.push({$:'numberBinary',destination:13,left:9,right:10,operator:'lessThan'});const written=code.length;code.push({$:'branch',condition:13,yes:written+1,no:0},{$:'propertyKey',destination:14,input:9,realm},{$:'ordinaryGet',destination:15,object:8,key:14,receiver:8},{$:'ordinarySet',destination:13,object:1,key:14,value:15,receiver:1},{$:'requireWrite',success:13},{$:'numberBinary',destination:9,left:9,right:11,operator:'add'},{$:'jump',target:writing});
  const deleting=code.length;code[written]={$:'branch',condition:13,yes:written+1,no:deleting};code.push({$:'numberBinary',destination:13,left:9,right:6,operator:'lessThan'});const complete=code.length;code.push({$:'branch',condition:13,yes:complete+1,no:0},{$:'propertyKey',destination:14,input:9,realm},{$:'ordinaryDelete',destination:13,object:1,key:14},{$:'primitive',destination:13,operation:'require delete success',arguments:[13]},{$:'numberBinary',destination:9,left:9,right:11,operator:'add'},{$:'jump',target:deleting});const done=code.length;code.push({$:'return',source:1});code[complete]={$:'branch',condition:13,yes:complete+1,no:done};
 }
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:27,captures:[],instructions:code};
}
