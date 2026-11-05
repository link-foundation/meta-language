// Compile independently supplied STANDARD operation owners into portable code.
// Function aliases share their real identity only when their operations agree.
import {findJavaScriptCallCodeFunction} from './javascript-call-code.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'collectionIteratorMethodOwner',identity:bigint,realm:bigint,operation:string,iteratorPrototype:JavaScriptSemanticValue,arrayPrototype:JavaScriptSemanticValue,objectPrototype:JavaScriptSemanticValue}} JavaScriptCollectionIteratorMethodOwner */
/** @typedef {{$:'collectionIteratorCallableDirectory',realms:bigint[],exoticKey:JavaScriptSemanticValue,methods:JavaScriptCollectionIteratorMethodOwner[]}} JavaScriptCollectionIteratorCallableDirectory */
/** @typedef {{$:'collectionIteratorCallCompilation',program:JavaScriptCallProgram,error:string}} JavaScriptCollectionIteratorCallCompilation */
/** @param {string} mode @returns {boolean} */
export function testJavaScriptCollectionIteratorIntrinsicMode(mode){return mode==='collection iterator intrinsic';}
/** @param {string} operation @returns {string} */
export function normalizeJavaScriptCollectionIteratorOperation(operation){return operation==='String.iterator'?'String.values':operation==='Set.keys'?'Set.values':operation;}
/** @param {string} operation @returns {string} */
export function readJavaScriptCollectionIteratorOperationKind(operation){if(['String.iterator','String.values','StringIterator.next'].includes(operation))return 'String';if(['Array.keys','Array.values','Array.entries','ArrayIterator.next'].includes(operation))return 'Array';if(['Map.keys','Map.values','Map.entries','MapIterator.next'].includes(operation))return 'Map';if(['Set.keys','Set.values','Set.entries','SetIterator.next'].includes(operation))return 'Set';return operation==='Iterator.self'?'self':'';}
/** @param {string} operation @returns {string} */
export function readJavaScriptCollectionIteratorOperationStrategy(operation){const actual=normalizeJavaScriptCollectionIteratorOperation(operation);return actual.endsWith('.keys')?'keys':actual.endsWith('.values')?'values':actual.endsWith('.entries')?'entries':'';}
/** @param {JavaScriptCollectionIteratorMethodOwner} left @param {JavaScriptCollectionIteratorMethodOwner} right @returns {boolean} */
export function testJavaScriptCollectionIteratorMethodAlias(left,right){
 const operation=normalizeJavaScriptCollectionIteratorOperation(left.operation);
 if(left.identity!==right.identity||left.realm!==right.realm||operation!==normalizeJavaScriptCollectionIteratorOperation(right.operation))return false;
 if(operation==='Iterator.self')return true;
 if(readJavaScriptCollectionIteratorOperationStrategy(operation)!=='')return testJavaScriptSemanticSameValue(left.iteratorPrototype,right.iteratorPrototype);
 return testJavaScriptSemanticSameValue(left.arrayPrototype,right.arrayPrototype)&&testJavaScriptSemanticSameValue(left.objectPrototype,right.objectPrototype);
}
/** @param {JavaScriptCollectionIteratorCallableDirectory} directory @returns {JavaScriptCollectionIteratorCallCompilation} */
export function compileJavaScriptCollectionIteratorCallDirectory(directory){
 const empty={$:'callProgram',functions:[],realms:directory.realms,exoticKey:directory.exoticKey};
 if(directory.methods.length>64||directory.realms.length>4096)return {$:'collectionIteratorCallCompilation',program:empty,error:'iterator operation directories exceed their explicit owner budget'};
 /** @type {JavaScriptCallFunction[]} */const functions=[];
 /** @type {JavaScriptCollectionIteratorMethodOwner[]} */const owners=[];
 for(const owner of directory.methods){
  const kind=readJavaScriptCollectionIteratorOperationKind(owner.operation),strategy=readJavaScriptCollectionIteratorOperationStrategy(owner.operation);
  if(kind===''||typeof owner.identity!=='bigint'||owner.identity<=0n||owner.identity>18446744073709551615n||!directory.realms.includes(owner.realm))return {$:'collectionIteratorCallCompilation',program:empty,error:'iterator methods require their actual global identity, realm and operation role'};
  const prior=owners.find(value=>value.identity===owner.identity);
  if(prior!==undefined){if(!testJavaScriptCollectionIteratorMethodAlias(prior,owner))return {$:'collectionIteratorCallCompilation',program:empty,error:'one iterator function identity cannot acquire conflicting operation semantics'};continue;}
  owners.push(owner);
  if(kind==='self'){
   functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'copy',destination:1,source:0},{$:'return',source:1}]});continue;
  }
  if(kind==='String'&&strategy==='values'){
   if(owner.iteratorPrototype.$!=='object'||owner.iteratorPrototype.callable)return {$:'collectionIteratorCallCompilation',program:empty,error:'String iterator creation retains its actual standard prototype'};
   functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'collection iterator intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'primitive',destination:1,operation:'require object coercible',arguments:[0]},{$:'primitive',destination:1,operation:'convert primitive text',arguments:[0]},{$:'constant',destination:2,value:owner.iteratorPrototype},{$:'collectionIteratorAllocate',destination:3,prototype:2,kind:'String',iterationKind:'values',input:1},{$:'return',source:3}]});continue;
  }
  if(strategy!==''){
   if(owner.iteratorPrototype.$!=='object'||owner.iteratorPrototype.callable)return {$:'collectionIteratorCallCompilation',program:empty,error:'iterator creation retains its actual standard prototype'};
   functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'collection iterator intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:3,captures:[],instructions:[{$:'constant',destination:1,value:owner.iteratorPrototype},{$:'collectionIteratorAllocate',destination:2,prototype:1,kind,iterationKind:strategy},{$:'return',source:2}]});continue;
  }
  if(owner.arrayPrototype.$!=='object'||owner.arrayPrototype.callable||owner.objectPrototype.$!=='object'||owner.objectPrototype.callable)return {$:'collectionIteratorCallCompilation',program:empty,error:'iterator next retains its actual realm Array and Object prototypes'};
  functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'collection iterator intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'constant',destination:1,value:owner.arrayPrototype},{$:'constant',destination:2,value:owner.objectPrototype},{$:'collectionIteratorResult',destination:3,kind,arrayPrototype:1,objectPrototype:2},{$:'return',source:3}]});
 }
 const ordered=functions.toSorted((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0),program={...empty,functions:ordered};
 for(const owner of owners)if(findJavaScriptCallCodeFunction(program,owner.identity)<0)return {$:'collectionIteratorCallCompilation',program:empty,error:'every actual iterator owner needs its registered executable code'};
 return {$:'collectionIteratorCallCompilation',program,error:''};
}
