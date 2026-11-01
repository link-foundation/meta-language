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
/** @param {JavaScriptOrdinaryHeap} heap @returns {bigint} */
export function readJavaScriptErrorAllocationBoundary(heap){let highest=0n;for(const object of heap.objects)if(object.identity>highest)highest=object.identity;for(const symbol of heap.symbols)if(symbol>highest)highest=symbol;return highest;}
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
 * @param {JavaScriptSemanticValue} prototype @param {bigint} before @returns {string} */
export function readJavaScriptErrorAllocationResultError(heap,value,prototype,before){
 const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;
 if(value.$!=='object'||value.callable||value.identity<=before)return 'an intrinsic Error result requires its actual fresh noncallable global Object identity';
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)];return object!==undefined&&testJavaScriptSemanticSameValue(object.prototype,prototype)?'':'the intrinsic Error result retains the selected actual derived prototype';
}
