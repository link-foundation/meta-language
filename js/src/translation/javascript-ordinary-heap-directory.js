import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
import {findJavaScriptPropertyDescriptorEntry} from './javascript-property-descriptors.js';

/** Tombstones retain stable insertion ordinals after releasing deleted values.
 * @typedef {{$:'collectionEntry',key:JavaScriptSemanticValue,value:JavaScriptSemanticValue,present:boolean}} JavaScriptCollectionEntry */
/** Weak keys are not strong roots. A WeakMap value is an ephemeron edge,
 * reachable only while its key is independently live. No iteration is exposed.
 * @typedef {{$:'weakCollectionEntry',key:JavaScriptSemanticValue,value:JavaScriptSemanticValue}} JavaScriptWeakCollectionEntry */
/** Callable closure identity belongs to the ordinary global heap. Code sites
 * remain in the separate program template directory; captures are real values.
 * @typedef {{$:'closureObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,callable:boolean,properties:JavaScriptOrdinaryProperty[],owner:bigint,ordinal:number,captures:JavaScriptSemanticValue[],lexicalReceiver:JavaScriptSemanticValue,lexicalNewTarget?:JavaScriptSemanticValue}} JavaScriptClosureObject */
/** @typedef {{$:'boundFunctionObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,callable:boolean,properties:JavaScriptOrdinaryProperty[],boundTarget:JavaScriptSemanticValue,boundThis:JavaScriptSemanticValue,boundArguments:JavaScriptSemanticValue[]}} JavaScriptBoundFunctionObject */
/** @typedef {{$:'arrayObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,callable:boolean,properties:JavaScriptOrdinaryProperty[]}} JavaScriptArrayObject */
/** @typedef {JavaScriptClosureObject | JavaScriptBoundFunctionObject | JavaScriptArrayObject | {$:'collectionIteratorObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],collection:bigint,cursor:number,kind:string,collectionKind:string,executing?:boolean,iteratedText?:JavaScriptSemanticValue} |
 * {$:'mapObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],entries:JavaScriptCollectionEntry[]} |
 * {$:'setObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],entries:JavaScriptCollectionEntry[]} |
 * {$:'weakMapObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,callable:boolean,properties:JavaScriptOrdinaryProperty[],weakEntries:JavaScriptWeakCollectionEntry[]} |
 * {$:'weakSetObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,callable:boolean,properties:JavaScriptOrdinaryProperty[],weakEntries:JavaScriptWeakCollectionEntry[]} |
 * {$:'ordinaryObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,immutablePrototype?:boolean,regularExpression?:boolean,stringData?:JavaScriptSemanticValue,primitiveData?:JavaScriptSemanticValue,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[]} |
 * {$:'hostErrorObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],hostGet:JavaScriptSemanticValue,
 * hostSet:JavaScriptSemanticValue} |
 * {$:'hostURLObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],hostGet:JavaScriptSemanticValue,
 * hostSet:JavaScriptSemanticValue} |
 * {$:'hostArrayObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],hostGet:JavaScriptSemanticValue,
 * hostSet:JavaScriptSemanticValue} |
 * {$:'hostByteBufferObject',identity:bigint,prototype:JavaScriptSemanticValue,extensible:boolean,
 * callable:boolean,properties:JavaScriptOrdinaryProperty[],hostGet:JavaScriptSemanticValue,
 * hostSet:JavaScriptSemanticValue}} JavaScriptOrdinaryObject
 */
/** Additive authority for edition-16 CanBeHeldWeakly. Absence preserves old
 * heap profiles; unknown Symbol facts cannot authorize a new weak operation.
 * @typedef {{$:'symbolRegistrationFact',identity:bigint,registered:boolean,key?:JavaScriptSemanticValue}} JavaScriptSymbolRegistrationFact */
/** @typedef {{$:'symbolRegistryEntry',identity:bigint,key:JavaScriptSemanticValue}} JavaScriptSymbolRegistryEntry */
/** Objects and Symbols share one strictly ordered unsigned64 identity space.
 * Functional snapshots are semantic states. A frontend must keep the selected
 * state in one shared cell so all aliases observe the same transition.
 * Nominal definitions keep recursive source types shared rather than repeatedly
 * expanding structural records along every typed expression occurrence.
 * @typedef {{$:'ordinaryHeap',objects:JavaScriptOrdinaryObject[],symbols:bigint[],symbolRegistrationModel?:string,symbolRegistrations?:JavaScriptSymbolRegistrationFact[],symbolRegistryModel?:string,symbolRegistry?:JavaScriptSymbolRegistryEntry[]}} JavaScriptOrdinaryHeap
 */

/** @param {JavaScriptOrdinaryObject[]} objects @param {bigint} identity @returns {number} */
export function findJavaScriptHeapDirectoryObject(objects, identity) {
  let low = 0;
  let high = objects.length;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const object = objects[middle];
    if (object === undefined) return -1;
    if (object.identity === identity) return middle;
    if (object.identity < identity) low = middle + 1;
    else high = middle;
  }
  return -1;
}

/** @param {bigint[]} symbols @param {bigint} identity @returns {boolean} */
export function testJavaScriptHeapDirectorySymbol(symbols, identity) {
  let low = 0;
  let high = symbols.length;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const symbol = symbols[middle];
    if (symbol === undefined) return false;
    if (symbol === identity) return true;
    if (symbol < identity) low = middle + 1;
    else high = middle;
  }
  return false;
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptHeapDirectoryValueError(heap, value) {
  const error = readJavaScriptSemanticValueError(value);
  if (error !== '') return error;
  if (value.$ === 'symbol') return testJavaScriptHeapDirectorySymbol(heap.symbols, value.identity) ? '' : 'a semantic Symbol must belong to the complete global identity directory';
  if (value.$ !== 'object') return '';
  const at = findJavaScriptHeapDirectoryObject(heap.objects, value.identity);
  if (at < 0) return 'a semantic Object must belong to the complete global identity directory';
  const object = heap.objects[at];
  return object !== undefined && object.callable === value.callable ? '' : 'Object callability must agree with its intrinsic global identity';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} key @returns {string} */
export function readJavaScriptHeapDirectoryKeyError(heap, key) {
  if (key.$ !== 'text' && key.$ !== 'sourceText' && key.$ !== 'symbol') return 'an ordinary property key must be text or Symbol without implicit coercion';
  return readJavaScriptHeapDirectoryValueError(heap, key);
}

/** @param {JavaScriptOrdinaryProperty[]} properties @param {JavaScriptSemanticValue} key @returns {number} */
export function findJavaScriptHeapDirectoryProperty(properties, key) {
  return findJavaScriptPropertyDescriptorEntry(properties, key);
}
