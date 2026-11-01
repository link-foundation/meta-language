// Semantic consistency checks, not an intrinsic authority oracle. Only the
// external realm allocator can create genuine Error internal data/stack state.
import {readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
/** @param {string} mode @returns {boolean} */
export function testJavaScriptErrorConstructorMode(mode){return mode==='derived Error';}
/** @param {string} mode @returns {boolean} */
export function testJavaScriptErrorIntrinsicMode(mode){return mode==='Error intrinsic';}
/** @param {string} errorClass @param {boolean} bindReceiver @param {string} mode @returns {string} */
export function readJavaScriptErrorConstructionKindError(errorClass,bindReceiver,mode){if(readJavaScriptSourceExceptionOperation(errorClass)==='')return 'Error value construction needs its actual standard intrinsic class';return bindReceiver&&!testJavaScriptErrorConstructorMode(mode)?'only an actual derived constructor binds the initialized receiver':'';}
/** @typedef {{$:'errorAllocationIdentity',identity:bigint,kind:string,callable:boolean}} JavaScriptErrorAllocationIdentity */
/** @typedef {{$:'errorAllocationDirectory',objects:JavaScriptErrorAllocationIdentity[],symbols:bigint[]}} JavaScriptErrorAllocationDirectory */
/** Freshness is prior membership, not numeric ordering. A globally allocated
 * Symbol may already have MAX_U64 while unused Object identities remain.
 * @param {JavaScriptOrdinaryHeap} heap @returns {JavaScriptErrorAllocationDirectory} */
export function readJavaScriptErrorAllocationDirectory(heap){return {$:'errorAllocationDirectory',objects:heap.objects.map(object=>({$:'errorAllocationIdentity',identity:object.identity,kind:object.$,callable:object.callable})),symbols:[...heap.symbols]};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptErrorAllocationDirectory} before @returns {string} */
export function readJavaScriptErrorPriorDirectoryError(heap,before){
 if(before.objects.length>4096||before.symbols.length>16384)return 'the prior Error allocation directory exceeds its explicit storage budget';
 let previous=0n;for(const row of before.objects){if(row.identity<=previous||row.identity>18446744073709551615n)return 'prior Object identities must retain strict global directory order';previous=row.identity;const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,row.identity)];if(object===undefined||object.$!==row.kind||object.callable!==row.callable)return 'an Error allocation cannot remove or retype prior Object identities';}
 previous=0n;for(const symbol of before.symbols){if(symbol<=previous||symbol>18446744073709551615n||before.objects.some(row=>row.identity===symbol))return 'prior Symbols must retain their distinct ordered global identities';previous=symbol;if(!heap.symbols.includes(symbol))return 'an Error allocation cannot remove or retype prior Symbol identities';}
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} constructor
 * @param {JavaScriptSemanticValue} newTarget @param {JavaScriptSemanticValue} prototype
 * @param {JavaScriptSemanticValue} message @param {JavaScriptSemanticValue} present @returns {string} */
export function readJavaScriptErrorConstructionOperandsError(heap,constructor,newTarget,prototype,message,present){
 for(const value of [constructor,newTarget,prototype,message,present]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(constructor.$!=='object'||!constructor.callable||newTarget.$!=='object'||!newTarget.callable)return 'Error construction needs actual constructor and newTarget Object identities';
 if(prototype.$!=='object'||prototype.callable)return 'the selected Error subclass retains its actual noncallable prototype Object';
 if(message.$!=='text'||present.$!=='boolean')return 'Error allocation retains normalized UTF16 text and independent own-message presence';
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,newTarget.identity)];if(object===undefined)return 'newTarget must remain in the actual global heap';
 const property=object.properties[findJavaScriptOrdinaryProperty(object.properties,{$:'text',units:[112,114,111,116,111,116,121,112,101]})];
 if(property===undefined||!property.descriptor.valuePresent||!testJavaScriptSemanticSameValue(property.descriptor.value,prototype))return 'this profile needs the actual newTarget own data prototype rather than an invented descriptor or getter';
 return '';
}
/** Validate freshness/prototype consistency only. This never certifies Error
 * internal slots or host stack authority from ordinary property spellings.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} value
 * @param {JavaScriptSemanticValue} prototype @param {JavaScriptErrorAllocationDirectory} before @returns {string} */
export function readJavaScriptErrorAllocationResultError(heap,value,prototype,before){
 const prior=readJavaScriptErrorPriorDirectoryError(heap,before);if(prior!=='')return prior;
 const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;
 if(value.$!=='object'||value.callable||before.objects.some(row=>row.identity===value.identity)||before.symbols.includes(value.identity))return 'an intrinsic Error result requires its actual fresh noncallable global Object identity';
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)];return object!==undefined&&object.$==='hostErrorObject'&&testJavaScriptSemanticSameValue(object.prototype,prototype)?'':'the intrinsic Error result retains its opaque host protocol and selected actual derived prototype';
}
