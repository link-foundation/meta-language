import {testJavaScriptObjectObservationOperation,compileJavaScriptObjectObservationFunction} from './javascript-object-integrity.js';
import {testJavaScriptObjectEnumerationOperation} from './javascript-own-enumeration.js';
import {testJavaScriptArrayMutationOperation,compileJavaScriptArrayMutationFunction} from './javascript-array-mutation-call-source.js';
import {testJavaScriptArrayTransformOperation} from './javascript-array-transform.js';
import {testJavaScriptArraySpeciesGetterOperation} from './javascript-array-species.js';
import {testJavaScriptArraySearchOperation} from './javascript-array-index-rules.js';
import {testJavaScriptArrayCallbackOperation} from './javascript-array-callbacks.js';
import {compileJavaScriptCollectionIteratorCallDirectory} from './javascript-collection-iterator-call-source.js';
// Structural bytecode transport to the shared inspectable collection kernel.
// Actual standard function/prototype/iterator identities come from the realm.
import {readJavaScriptCollectionOperationKind} from './javascript-collection-rules.js';
import {findJavaScriptCallCodeFunction} from './javascript-call-code.js';
import {testJavaScriptPrimitiveMethodOperation} from './javascript-primitive-methods.js';
import {testJavaScriptArrayIntrinsicOperation} from './javascript-arrays.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptObjectDefinitionOperation(operation){return operation==='Object.defineProperty';}
/** @param {bigint} identity @param {bigint} realm @returns {JavaScriptCallFunction} */
export function compileJavaScriptObjectDefinitionFunction(identity,realm){return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:5,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'argument',destination:2,index:1},{$:'argument',destination:3,index:2},{$:'propertyDefinition',destination:4,target:1,key:2,attributes:3,realm},{$:'return',source:4}]};}

/** @typedef {{$:'collectionMethodOwner',identity:bigint,operation:string}} JavaScriptCollectionMethodOwner */
/** @typedef {{$:'collectionConstructorOwner',kind:string,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue,adder:JavaScriptSemanticValue,iteratorKey:JavaScriptSemanticValue,iteratorMethod:JavaScriptSemanticValue,iteratorPrototype:JavaScriptSemanticValue,nextMethod:JavaScriptSemanticValue,methods:JavaScriptCollectionMethodOwner[]}} JavaScriptCollectionConstructorOwner */
/** @typedef {{$:'primitiveMethodOwner',identity:bigint,realm:bigint,operation:string,species?:JavaScriptArraySpeciesDirectory,arrayPrototype?:JavaScriptSemanticValue,hasInstanceKey?:JavaScriptSemanticValue}} JavaScriptPrimitiveMethodOwner */
/** @typedef {{$:'collectionCallableDirectory',realms:bigint[],exoticKey:JavaScriptSemanticValue,constructors:JavaScriptCollectionConstructorOwner[],primitiveMethods:JavaScriptPrimitiveMethodOwner[],iteratorMethods?:JavaScriptCollectionIteratorMethodOwner[],ownerBudget?:number}} JavaScriptCollectionCallableDirectory */
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
/** General constructors share the actual iterator/ordinary Get/call/finally
 * instructions. The adder is selected once before GetIterator. Nullish input
 * returns before either protocol is observed. No native collection body runs.
 * @param {JavaScriptCollectionConstructorOwner} owner @returns {JavaScriptCallFunction} */
export function compileJavaScriptIterableCollectionConstructor(owner){
 /** @type {JavaScriptCallInstruction[]} */const instructions=[
  {$:'argument',destination:1,index:0},{$:'constant',destination:2,value:owner.prototype},{$:'collectionAllocate',destination:3,prototype:2,kind:owner.kind},
  {$:'constant',destination:7,value:{$:'undefined'}},{$:'strictEqual',destination:8,left:1,right:7},{$:'branch',condition:8,yes:0,no:6},
  {$:'constant',destination:7,value:{$:'null'}},{$:'strictEqual',destination:8,left:1,right:7},{$:'branch',condition:8,yes:0,no:9},
  {$:'constant',destination:4,value:{$:'text',units:owner.kind==='Map'?[115,101,116]:[97,100,100]}},
  {$:'ordinaryGet',destination:5,object:3,key:4,receiver:3},{$:'primitive',destination:5,operation:'require collection adder',arguments:[5]},
  {$:'constant',destination:6,value:owner.iteratorKey},{$:'iteratorProtocol',destination:7,done:8,input:1,iteratorKey:6,slot:0,operation:'acquire',realm:owner.realm},
  {$:'pushFinally',target:0}
 ];
 const start=instructions.length;
 instructions.push({$:'iteratorProtocol',destination:9,done:8,input:0,iteratorKey:0,slot:0,operation:'step',realm:owner.realm});
 const exhausted=instructions.length;instructions.push({$:'branch',condition:8,yes:0,no:exhausted+1});
 if(owner.kind==='Map')instructions.push(
  {$:'primitive',destination:9,operation:'require Map entry object',arguments:[9]},
  {$:'constant',destination:13,value:{$:'text',units:[48]}},{$:'ordinaryGet',destination:10,object:9,key:13,receiver:9},
  {$:'constant',destination:13,value:{$:'text',units:[49]}},{$:'ordinaryGet',destination:11,object:9,key:13,receiver:9}
 );
 instructions.push({$:'call',destination:12,callee:5,receiver:3,arguments:owner.kind==='Map'?[10,11]:[9]},{$:'jump',target:start});
 const leave=instructions.length;instructions.push({$:'completeJump',target:0,depth:0});
 const cleanup=instructions.length;instructions.push({$:'iteratorProtocol',destination:7,done:8,input:0,iteratorKey:0,slot:0,operation:'close',realm:owner.realm},{$:'finishFinally'});
 const end=instructions.length;instructions.push({$:'return',source:3});
 instructions[5]={$:'branch',condition:8,yes:end,no:6};instructions[8]={$:'branch',condition:8,yes:end,no:9};instructions[14]={$:'pushFinally',target:cleanup};instructions[exhausted]={$:'branch',condition:8,yes:leave,no:exhausted+1};instructions[leave]={$:'completeJump',target:end,depth:0};
 return {$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'collection constructor',lexicalReceiver:{$:'undefined'},parameters:0,registers:14,captures:[],instructions};
}
/** @param {JavaScriptCollectionCallableDirectory} directory @returns {JavaScriptCollectionCallCompilation} */
export function compileJavaScriptCollectionCallDirectory(directory){
 const empty={$:'callProgram',functions:[],realms:directory.realms,exoticKey:directory.exoticKey};
 const ownerBudget=directory.ownerBudget??16;if(ownerBudget!==16&&ownerBudget!==64)return {$:'collectionCallCompilation',program:empty,error:'standard callable owner budgets require their actual registered 16 or 64 profile'};
 if(directory.constructors.length>2||directory.realms.length>4096||directory.primitiveMethods.length>ownerBudget)return {$:'collectionCallCompilation',program:empty,error:'standard collection call directories exceed their actual owner budget'};
 const identities=[];/** @type {JavaScriptCallFunction[]} */let functions=[];
 for(const owner of directory.constructors){
  if(!['Set','Map'].includes(owner.kind)||owner.identity<=0n||owner.identity>18446744073709551615n||!directory.realms.includes(owner.realm)||identities.includes(owner.identity)||owner.prototype.$!=='object'||owner.prototype.callable||owner.methods.length>16)return {$:'collectionCallCompilation',program:empty,error:'collection constructors require actual unique callable, realm, kind and prototype owners'};
  identities.push(owner.identity);
  functions.push(compileJavaScriptIterableCollectionConstructor(owner));
  for(const method of owner.methods){
   if(method.identity<=0n||method.identity>18446744073709551615n||identities.includes(method.identity)||readJavaScriptCollectionOperationKind(method.operation)!==owner.kind)return {$:'collectionCallCompilation',program:empty,error:'collection methods require their actual distinct standard function identities and operation owners'};
   identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:owner.realm,receiverMode:'collection intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'collectionOperation',destination:1,operation:method.operation},{$:'return',source:1}]});
  }
 }
 for(const method of directory.primitiveMethods){
  if(method.identity<=0n||method.identity>18446744073709551615n||identities.includes(method.identity)||!directory.realms.includes(method.realm)||!testJavaScriptPrimitiveMethodOperation(method.operation)&&!testJavaScriptArrayIntrinsicOperation(method.operation)&&!testJavaScriptArrayCallbackOperation(method.operation)&&!testJavaScriptArraySearchOperation(method.operation)&&!testJavaScriptArraySpeciesGetterOperation(method.operation)&&!testJavaScriptArrayTransformOperation(method.operation)&&!testJavaScriptArrayMutationOperation(method.operation)&&!testJavaScriptObjectEnumerationOperation(method.operation)&&!testJavaScriptObjectObservationOperation(method.operation)&&!testJavaScriptObjectDefinitionOperation(method.operation)&&method.operation!=='Function.hasInstance')return {$:'collectionCallCompilation',program:empty,error:'primitive methods require their actual distinct standard identity, realm and operation'};
  if(testJavaScriptArrayTransformOperation(method.operation)&&(method.species===undefined||method.species.realm!==method.realm))return {$:'collectionCallCompilation',program:empty,error:'Array transformations require their actual intrinsic owner realm species directory'};
  if(testJavaScriptObjectEnumerationOperation(method.operation)){if(method.arrayPrototype?.$!=='object'||method.arrayPrototype.callable)return {$:'collectionCallCompilation',program:empty,error:'Object enumeration requires its actual realm Array prototype'};identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:method.realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:3,captures:[],instructions:[{$:'constant',destination:1,value:method.arrayPrototype},{$:'objectEnumeration',destination:2,operation:method.operation,prototype:1},{$:'return',source:2}]});continue;}
  if(method.operation==='Function.hasInstance'){
   if(method.hasInstanceKey?.$!=='symbol')return {$:'collectionCallCompilation',program:empty,error:'Function hasInstance retains its actual realm well-known Symbol'};
   identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:method.realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'constant',destination:2,value:method.hasInstanceKey},{$:'instanceOf',destination:3,left:1,right:0,key:2,operation:'ordinary has instance'},{$:'return',source:3}]});continue;
  }
  if(testJavaScriptObjectDefinitionOperation(method.operation)){identities.push(method.identity);functions.push(compileJavaScriptObjectDefinitionFunction(method.identity,method.realm));continue;}
  if(testJavaScriptObjectObservationOperation(method.operation)){const fn=compileJavaScriptObjectObservationFunction(method.identity,method.realm,method.operation);if(fn===undefined)return {$:'collectionCallCompilation',program:empty,error:'Object observation retains its actual registered operation'};identities.push(method.identity);functions.push(fn);continue;}
  if(testJavaScriptArrayMutationOperation(method.operation)){const fn=compileJavaScriptArrayMutationFunction(method.identity,method.realm,method.operation);if(fn===undefined)return {$:'collectionCallCompilation',program:empty,error:'Array mutation retains its actual registered operation'};identities.push(method.identity);functions.push(fn);continue;}
  identities.push(method.identity);functions.push({$:'callFunction',identity:method.identity,realm:method.realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:testJavaScriptArrayTransformOperation(method.operation)&&method.species!==undefined?[{$:'arrayTransform',destination:1,operation:method.operation,species:method.species},{$:'return',source:1}]:testJavaScriptArraySpeciesGetterOperation(method.operation)?[{$:'copy',destination:1,source:0},{$:'return',source:1}]:[{$:testJavaScriptArrayCallbackOperation(method.operation)?'arrayCallback':testJavaScriptArraySearchOperation(method.operation)?'arraySearch':'primitiveMethod',destination:1,operation:method.operation},{$:'return',source:1}]});
 }
 const iterators=compileJavaScriptCollectionIteratorCallDirectory({$:'collectionIteratorCallableDirectory',realms:directory.realms,exoticKey:directory.exoticKey,methods:directory.iteratorMethods??[]});
 if(iterators.error!=='')return {$:'collectionCallCompilation',program:empty,error:iterators.error};
 for(const fn of iterators.program.functions){if(identities.includes(fn.identity))return {$:'collectionCallCompilation',program:empty,error:'iterator code cannot replace an actual constructor, property or primitive method owner'};identities.push(fn.identity);functions.push(fn);}
 functions=functions.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0);
 const program={$:'callProgram',functions,realms:directory.realms,exoticKey:directory.exoticKey};
 for(let at=0;at<functions.length;at++){const fn=functions[at];if(fn===undefined||findJavaScriptCallCodeFunction(program,fn.identity)!==at)return {$:'collectionCallCompilation',program:empty,error:'portable standard operation directories retain their complete sorted callable identities'};}
 return {$:'collectionCallCompilation',program,error:''};
}
