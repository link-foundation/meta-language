// ArraySpeciesCreate selection. Allocation and Construct are explicit requests,
// never assumed Objects or original-host-body fallbacks.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {testJavaScriptArrayValue} from './javascript-arrays.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** Facts come from the actual protected builtin/source declaration registry.
 * Their validation establishes shape/context, not an execution certificate.
 * @typedef {{$:'arrayConstructorFact',identity:bigint,constructable:boolean,realm:bigint,intrinsicArray:boolean}} JavaScriptArrayConstructorFact */
/** @typedef {{$:'arraySpeciesDirectory',realm:bigint,arrayConstructor:JavaScriptSemanticValue,arrayPrototype:JavaScriptSemanticValue,speciesKey:JavaScriptSemanticValue,constructors:JavaScriptArrayConstructorFact[]}} JavaScriptArraySpeciesDirectory */
/** @typedef {{$:'arraySpeciesSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArraySpeciesCall */
/** @typedef {{$:'arraySpeciesState',original:JavaScriptSemanticValue,length:number,realm:bigint,arrayConstructor:JavaScriptSemanticValue,arrayPrototype:JavaScriptSemanticValue,speciesKey:JavaScriptSemanticValue,candidate:JavaScriptSemanticValue,phase:string,call?:JavaScriptArraySpeciesCall}} JavaScriptArraySpeciesState */
/** @typedef {{$:'arrayConstructorNotConstructable'} | {$:'arrayConstructorKnown',fact:JavaScriptArrayConstructorFact} | {$:'arrayConstructorUnknown'}} JavaScriptArrayConstructorClassification */
/** @typedef {{$:'arraySpeciesContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpeciesState} |
 * {$:'arraySpeciesCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpeciesState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arraySpeciesNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpeciesState,constructor:JavaScriptSemanticValue} |
 * {$:'arraySpeciesAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpeciesState,prototype:JavaScriptSemanticValue,length:number} |
 * {$:'arraySpeciesConstruct',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpeciesState,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fact:JavaScriptArrayConstructorFact} |
 * {$:'arraySpeciesThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySpeciesError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arraySpeciesInvalid',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptArraySpeciesTransition */
/** @param {JavaScriptArrayConstructorFact[]} facts @param {bigint} identity @returns {number} */
export function findJavaScriptArrayConstructorFact(facts,identity){let low=0,high=facts.length;while(low<high){const middle=Math.floor((low+high)/2),fact=facts[middle];if(fact===undefined)return -1;if(fact.identity===identity)return middle;if(fact.identity<identity)low=middle+1;else high=middle;}return -1;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @returns {string} */
export function readJavaScriptArraySpeciesDirectoryError(heap,directory){
 const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
 if(typeof directory.realm!=='bigint'||directory.realm<=0n||directory.realm>18446744073709551615n||directory.constructors.length>4096)return 'species authority retains its actual unsigned64 realm and bounded constructor directory';
 for(const value of [directory.arrayConstructor,directory.arrayPrototype,directory.speciesKey]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(directory.arrayConstructor.$!=='object'||!directory.arrayConstructor.callable||directory.arrayPrototype.$!=='object'||directory.arrayPrototype.callable||directory.speciesKey.$!=='symbol')return 'species authority retains actual Array constructor/prototype and well-known Symbol identities';
 let prior=0n;for(const fact of directory.constructors){
  if(typeof fact.identity!=='bigint'||fact.identity<=prior||fact.identity>18446744073709551615n||typeof fact.constructable!=='boolean'||typeof fact.intrinsicArray!=='boolean'||typeof fact.realm!=='bigint'||fact.realm<=0n||fact.realm>18446744073709551615n||fact.intrinsicArray&&!fact.constructable)return 'constructor facts retain ordered unique actual identities, realms and classifications';prior=fact.identity;
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,fact.identity)];if(object===undefined||!object.callable||object.$==='closureObject'&&fact.constructable)return 'constructor facts cannot invent callable identities or constructable arrow closures';
 }
 const intrinsicRealms=directory.constructors.filter(fact=>fact.intrinsicArray).map(fact=>fact.realm).toSorted((a,b)=>a<b?-1:a>b?1:0);for(let at=1;at<intrinsicRealms.length;at++)if(intrinsicRealms[at]===intrinsicRealms[at-1])return 'a realm has one actual intrinsic Array constructor identity';
 const root=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,directory.arrayConstructor.identity)];
 return root===undefined||!root.constructable||!root.intrinsicArray||root.realm!==directory.realm?'species authority retains its actual current-realm intrinsic Array constructor fact':'';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArraySpeciesState} state @returns {string} */
export function readJavaScriptArraySpeciesStateError(heap,directory,state){
 const error=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(error!=='')return error;
 if(state.realm!==directory.realm||!testJavaScriptSemanticSameValue(state.arrayConstructor,directory.arrayConstructor)||!testJavaScriptSemanticSameValue(state.arrayPrototype,directory.arrayPrototype)||!testJavaScriptSemanticSameValue(state.speciesKey,directory.speciesKey))return 'species continuation cannot replace its actual realm, roots or Symbol';
 if(state.original.$!=='object'||!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0)||!['brand','constructor','constructor classification','species','species classification'].includes(state.phase))return 'species selection retains its actual Object, nonnegative length and phase';
 for(const value of [state.original,state.candidate]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.call!==undefined){
  if(!['constructor','species'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length>3)return 'species selection retains its actual getter call';
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 }
 if(state.phase==='species'&&state.candidate.$!=='object')return 'species Get retains its actual constructor Object receiver';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptSemanticValue} candidate @returns {JavaScriptArrayConstructorClassification} */
export function classifyJavaScriptArraySpeciesConstructor(heap,directory,candidate){
 if(candidate.$!=='object'||!candidate.callable)return {$:'arrayConstructorNotConstructable'};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,candidate.identity)];if(object!==undefined&&object.$==='closureObject')return {$:'arrayConstructorNotConstructable'};
 const fact=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,candidate.identity)];return fact===undefined?{$:'arrayConstructorUnknown'}:{$:'arrayConstructorKnown',fact};
}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArraySpeciesGetterOperation(operation){return operation==='Array.species getter';}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptSemanticValue} original @param {number} length @returns {JavaScriptArraySpeciesTransition} */
export function beginJavaScriptArraySpeciesSelection(heap,directory,original,length){
 const state={$:'arraySpeciesState',original,length,realm:directory.realm,arrayConstructor:directory.arrayConstructor,arrayPrototype:directory.arrayPrototype,speciesKey:directory.speciesKey,candidate:{$:'undefined'},phase:'brand'};
 const error=readJavaScriptArraySpeciesStateError(heap,directory,state);return error===''?{$:'arraySpeciesContinue',heap,state}:{$:'arraySpeciesInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesState} state @returns {JavaScriptArraySpeciesTransition} */
export function requestJavaScriptArraySpeciesDefault(heap,state){return {$:'arraySpeciesAllocate',heap,state,prototype:state.arrayPrototype,length:state.length};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesState} state
 * @param {JavaScriptSemanticValue} value @returns {JavaScriptArraySpeciesTransition} */
export function continueJavaScriptArraySpeciesGet(heap,state,value){const {call:priorCall,...base}=state;return {$:'arraySpeciesContinue',heap,state:{...base,candidate:state.phase==='species'&&value.$==='null'?{$:'undefined'}:value,phase:state.phase==='constructor'?'constructor classification':'species classification'}};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArraySpeciesState} state @returns {JavaScriptArraySpeciesTransition} */
export function stepJavaScriptArraySpeciesSelection(heap,directory,state){
 const error=readJavaScriptArraySpeciesStateError(heap,directory,state);if(error!=='')return {$:'arraySpeciesInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arraySpeciesCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='brand'){
  const brand=testJavaScriptArrayValue(heap,state.original);if(brand.error!=='')return {$:'arraySpeciesInvalid',heap,reason:brand.error};if(!brand.array)return requestJavaScriptArraySpeciesDefault(heap,state);
  return {$:'arraySpeciesContinue',heap,state:{...state,phase:'constructor'}};
 }
 if(state.phase==='constructor'||state.phase==='species'){
  const receiver=state.phase==='constructor'?state.original:state.candidate,key=state.phase==='constructor'?{$:'text',units:[99,111,110,115,116,114,117,99,116,111,114]}:state.speciesKey;
  if(receiver.$!=='object')return {$:'arraySpeciesInvalid',heap,reason:'species property access retains its actual receiver'};
  const get=planJavaScriptOrdinaryGet(heap,receiver.identity,key,receiver);if(get.error!=='')return {$:'arraySpeciesInvalid',heap,reason:get.error};
  if(get.action==='call'){const call={$:'arraySpeciesSelectedCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'arraySpeciesCall',heap,state:{...state,call},callee:get.callee,receiver:get.receiver,arguments:get.arguments};}
  if(get.action!=='value')return {$:'arraySpeciesInvalid',heap,reason:'species Get retains an actual value or getter continuation'};
  return continueJavaScriptArraySpeciesGet(heap,state,get.value);
 }
 if(state.candidate.$==='undefined')return requestJavaScriptArraySpeciesDefault(heap,state);
 const classification=classifyJavaScriptArraySpeciesConstructor(heap,directory,state.candidate);
 if(classification.$==='arrayConstructorUnknown')return {$:'arraySpeciesNeedAuthority',heap,state,constructor:state.candidate};
 if(state.phase==='constructor classification'){
  if(classification.$==='arrayConstructorKnown'&&classification.fact.constructable&&classification.fact.intrinsicArray&&classification.fact.realm!==state.realm)return requestJavaScriptArraySpeciesDefault(heap,state);
  // A nonconstructor Object can supply a valid constructor through @@species.
  if(state.candidate.$==='object')return {$:'arraySpeciesContinue',heap,state:{...state,phase:'species'}};
  return {$:'arraySpeciesError',heap,errorClass:'TypeError',reason:'Array species constructor is not a constructor'};
 }
 if(classification.$!=='arrayConstructorKnown'||!classification.fact.constructable)return {$:'arraySpeciesError',heap,errorClass:'TypeError',reason:'Array species constructor is not a constructor'};
 return {$:'arraySpeciesConstruct',heap,state,constructor:state.candidate,arguments:[{$:'number',value:state.length}],fact:{...classification.fact}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArraySpeciesState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArraySpeciesTransition} */
export function resumeJavaScriptArraySpeciesGetter(heap,directory,state,completion){
 const error=readJavaScriptArraySpeciesStateError(heap,directory,state);if(error!=='')return {$:'arraySpeciesInvalid',heap,reason:error};
 if(state.call===undefined)return {$:'arraySpeciesInvalid',heap,reason:'species resumption retains its actual selected getter'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arraySpeciesInvalid',heap,reason:invalid};
 return completion.$==='throwCompletion'?{$:'arraySpeciesThrow',heap,value:completion.value}:continueJavaScriptArraySpeciesGet(heap,state,completion.value);
}
