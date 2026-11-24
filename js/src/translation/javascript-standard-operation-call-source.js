// Pure bytecode builders for stateful standard operations.
import {testJavaScriptStandardArrayCreationOperation,testJavaScriptStandardCollectionForEachOperation,testJavaScriptStandardObjectObservationOperation} from './javascript-standard-operation-rules.js';
/** @param {bigint} identity @param {bigint} realm @param {string} operation @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptStandardArrayCreationFunction(identity,realm,operation,prototype,iteratorKey){
 if(!testJavaScriptStandardArrayCreationOperation(operation)||prototype.$!=='object'||prototype.callable||iteratorKey.$!=='symbol')return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'constant',destination:1,value:prototype},{$:'constant',destination:2,value:iteratorKey},{$:'arrayCreation',destination:3,operation,prototype:1,iteratorKey:2},{$:'return',source:3}]};
}
/** @param {bigint} identity @param {bigint} realm @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptStandardObjectFromEntriesFunction(identity,realm,prototype,iteratorKey){
 if(prototype.$!=='object'||prototype.callable||iteratorKey.$!=='symbol')return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:5,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'constant',destination:2,value:prototype},{$:'constant',destination:3,value:iteratorKey},{$:'objectFromEntries',destination:4,input:1,prototype:2,iteratorKey:3},{$:'return',source:4}]};
}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptStandardCollectionForEachFunction(identity,realm,operation){
 if(!testJavaScriptStandardCollectionForEachOperation(operation))return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'collectionForEach',destination:1,operation},{$:'return',source:1}]};
}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptStandardObjectObservationFunction(identity,realm,operation){
 if(!testJavaScriptStandardObjectObservationOperation(operation))return undefined;
 if(operation==='Boolean.toString')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'objectObservation',destination:1,operation:'Boolean.valueOf',input:0,other:0},{$:'primitive',destination:1,operation:'convert primitive text',arguments:[1]},{$:'return',source:1}]};
 if(['Symbol.valueOf','Number.valueOf','Boolean.valueOf','BigInt.valueOf'].includes(operation)||operation==='String.toString'||operation==='String.valueOf')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'objectObservation',destination:1,operation,input:0,other:0},{$:'return',source:1}]};
 if(operation==='Object.prototype.hasOwnProperty'||operation==='Object.prototype.propertyIsEnumerable')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'propertyKey',destination:1,input:1,realm},{$:'primitive',destination:2,operation:'require nonnull Object input',arguments:[0]},{$:'objectObservation',destination:3,operation,input:0,other:1},{$:'return',source:3}]};
 /** @type {JavaScriptCallInstruction[]} */const instructions=[{$:'argument',destination:1,index:0}];
 if(operation==='Object.hasOwn')instructions.push({$:'primitive',destination:1,operation:'require nonnull Object input',arguments:[1]});
 instructions.push({$:'argument',destination:2,index:1});if(operation==='Object.hasOwn')instructions.push({$:'propertyKey',destination:2,input:2,realm});
 instructions.push({$:'objectObservation',destination:3,operation,input:1,other:2},{$:'return',source:3});return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions};
}
