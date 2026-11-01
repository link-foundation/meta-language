import {testJavaScriptArraySearchOperation} from './javascript-array-index-rules.js';
import {testJavaScriptArrayCallbackOperation} from './javascript-array-callbacks.js';
import {compileJavaScriptCollectionIteratorCallDirectory} from './javascript-collection-iterator-call-source.js';
// Structural bytecode transport to the shared inspectable collection kernel.
// Actual standard function/prototype/iterator identities come from the realm.
import {readJavaScriptCollectionOperationKind} from './javascript-collection-rules.js';
import {findJavaScriptCallCodeFunction} from './javascript-call-code.js';
import {testJavaScriptPrimitiveMethodOperation} from './javascript-primitive-methods.js';
import {testJavaScriptArrayIntrinsicOperation} from './javascript-arrays.js';
/** @typedef {{$:'collectionMethodOwner',identity:bigint,operation:string}} JavaScriptCollectionMethodOwner */
/** @typedef {{$:'collectionConstructorOwner',kind:string,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue,adder:JavaScriptSemanticValue,iteratorKey:JavaScriptSemanticValue,iteratorMethod:JavaScriptSemanticValue,iteratorPrototype:JavaScriptSemanticValue,nextMethod:JavaScriptSemanticValue,methods:JavaScriptCollectionMethodOwner[]}} JavaScriptCollectionConstructorOwner */
/** @typedef {{$:'primitiveMethodOwner',identity:bigint,realm:bigint,operation:string}} JavaScriptPrimitiveMethodOwner */
/** @typedef {{$:'collectionCallableDirectory',realms:bigint[],exoticKey:JavaScriptSemanticValue,constructors:JavaScriptCollectionConstructorOwner[],primitiveMethods:JavaScriptPrimitiveMethodOwner[],iteratorMethods?:JavaScriptCollectionIteratorMethodOwner[]}} JavaScriptCollectionCallableDirectory */
/** @typedef {{$:'collectionCallCompilation',program:JavaScriptCallProgram,error:string}} JavaScriptCollectionCallCompilation */
/** @param {string} mode @returns {boolean} */
export function testJavaScriptCollectionConstructorMode(mode){return mode==='collection constructor';}
/** @param {string} mode @returns {boolean} */
export function testJavaScriptCollectionIntrinsicMode(mode){return mode==='collection intrinsic';}
/** @param {string} mode @returns {boolean} */
export function testJavaScriptPrimitiveIntrinsicMode(mode){return mode==='primitive intrinsic';}
/** @param {JavaScriptCallFunction} fn @returns {string} */
export function readJavaScriptCollectionConstructorKind(fn){
 if(!testJavaScriptCollectionConstructorMode(fn.receiverMode))return '';
 let kind='';for(const instruction of fn.instructions)if(instruction.$==='collectionAllocate'){if(kind!==''||!['Map','Set'].includes(instruction.kind))return '';kind=instruction.kind;}return kind;
}
/** @param {JavaScriptCollectionCallableDirectory} directory @returns {JavaScriptCollectionCallCompilation} */
export function compileJavaScriptCollectionCallDirectory(directory){
 const empty={$:'callProgram',functions:[],realms:directory.realms,exoticKey:directory.exoticKey};
 if(directory.constructors.length>2||directory.realms.length>4096||directory.primitiveMethods.length>16)return {$:'collectionCallCompilation',program:empty,error:'standard collection call directories exceed their actual owner budget'};
 const identities=[];/** @type {JavaScriptCallFunction[]} */let functions=[];
 for(const owner of directory.constructors){
  if(!['Set','Map'].includes(owner.kind)||owner.identity<=0n||owner.identity>18446744073709551615n||!directory.realms.includes(owner.realm)||identities.includes(owner.identity)||owner.prototype.$!=='object'||owner.prototype.callable||owner.methods.length>16)return {$:'collectionCallCompilation',program:empty,error:'collection constructors require actual unique callable, realm, kind and prototype owners'};
  identities.push(owner.identity);
  functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'collection constructor',lexicalReceiver:{$:'undefined'},parameters:0,registers:10,captures:[],instructions:[
   {$:'argument',destination:1,index:0},{$:'constant',destination:2,value:owner.prototype},{$:'collectionAllocate',destination:3,prototype:2,kind:owner.kind},
   {$:'constant',destination:4,value:owner.adder},{$:'constant',destination:5,value:owner.iteratorKey},{$:'constant',destination:6,value:owner.iteratorMethod},{$:'constant',destination:7,value:owner.iteratorPrototype},{$:'constant',destination:8,value:owner.nextMethod},
   {$:'collectionCopy',destination:9,target:3,input:1,adder:4,iteratorKey:5,iteratorMethod:6,iteratorPrototype:7,nextMethod:8},{$:'return',source:9}
  ]});
  for(const method of owner.methods){
   if(method.identity<=0n||method.identity>18446744073709551615n||identities.includes(method.identity)||readJavaScriptCollectionOperationKind(method.operation)!==owner.kind)return {$:'collectionCallCompilation',program:empty,error:'collection methods require their actual distinct standard function identities and operation owners'};
   identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:owner.realm,receiverMode:'collection intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'collectionOperation',destination:1,operation:method.operation},{$:'return',source:1}]});
  }
 }
 for(const method of directory.primitiveMethods){
  if(method.identity<=0n||method.identity>18446744073709551615n||identities.includes(method.identity)||!directory.realms.includes(method.realm)||!testJavaScriptPrimitiveMethodOperation(method.operation)&&!testJavaScriptArrayIntrinsicOperation(method.operation)&&!testJavaScriptArrayCallbackOperation(method.operation)&&!testJavaScriptArraySearchOperation(method.operation))return {$:'collectionCallCompilation',program:empty,error:'primitive methods require their actual distinct standard identity, realm and operation'};
  identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:method.realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:testJavaScriptArrayCallbackOperation(method.operation)?'arrayCallback':testJavaScriptArraySearchOperation(method.operation)?'arraySearch':'primitiveMethod',destination:1,operation:method.operation},{$:'return',source:1}]});
 }
 const iterators=compileJavaScriptCollectionIteratorCallDirectory({$:'collectionIteratorCallableDirectory',realms:directory.realms,exoticKey:directory.exoticKey,methods:directory.iteratorMethods??[]});
 if(iterators.error!=='')return {$:'collectionCallCompilation',program:empty,error:iterators.error};
 for(const fn of iterators.program.functions){if(identities.includes(fn.identity))return {$:'collectionCallCompilation',program:empty,error:'iterator code cannot replace an actual constructor, property or primitive method owner'};identities.push(fn.identity);functions.push(fn);}
 functions=functions.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0);
 const program={$:'callProgram',functions,realms:directory.realms,exoticKey:directory.exoticKey};
 for(let at=0;at<functions.length;at++){const fn=functions[at];if(fn===undefined||findJavaScriptCallCodeFunction(program,fn.identity)!==at)return {$:'collectionCallCompilation',program:empty,error:'portable standard operation directories retain their complete sorted callable identities'};}
 return {$:'collectionCallCompilation',program,error:''};
}
