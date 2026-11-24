// Distinct inspectable weak slots. No host WeakMap/WeakSet stores represented
// contents, and strong Map entries never stand in for ephemeron edges.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,replaceJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptWeakCollectionOperationKind,readJavaScriptWeakKeyEligibility} from './javascript-weak-collection-rules.js';
/** Ordered global identities provide a logarithmic key lookup; insertion order
 * is unobservable for weak collections. Negative result encodes insertion slot.
 * @param {JavaScriptWeakCollectionEntry[]} entries @param {JavaScriptSemanticValue} key @returns {number} */
export function findJavaScriptWeakCollectionKey(entries,key){
 if(key.$!=='object'&&key.$!=='symbol')return -1;let low=0,high=entries.length;
 while(low<high){const middle=Math.floor((low+high)/2),entry=entries[middle];if(entry===undefined)return -1;if(testJavaScriptSemanticSameValue(entry.key,key))return middle;if((entry.key.$==='object'||entry.key.$==='symbol')&&entry.key.identity<key.identity)low=middle+1;else high=middle;}return -low-1;
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {string} kind @returns {JavaScriptCollectionCompletion} */
export function allocateJavaScriptWeakCollection(heap,identity,prototype,kind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 if(!['WeakMap','WeakSet'].includes(kind))return {$:'collectionInvalid',heap,reason:'weak allocation needs its actual distinct standard collection kind'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||heap.symbols.includes(identity))return {$:'collectionInvalid',heap,reason:'weak collection allocation needs a fresh actual global identity'};
 if(heap.objects.length>=4096)return {$:'collectionResourceLimit',heap,reason:'weak collection allocation exceeds the shared Object directory budget'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'collectionInvalid',heap,reason:'weak collection allocation keeps its actual ordinary prototype'};
 const invalid=readJavaScriptOrdinaryValueError(heap,prototype);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};
 const object={$:kind==='WeakMap'?'weakMapObject':'weakSetObject',identity,prototype,extensible:true,callable:false,properties:[],weakEntries:[]};let index=0;while(index<heap.objects.length&&(heap.objects[index]?.identity??identity)<identity)index++;
 const updated={...heap,objects:heap.objects.slice(0,index).concat([object],heap.objects.slice(index))},failure=readJavaScriptOrdinaryHeapError(updated);return failure===''?{$:'collectionNormal',heap:updated,value:{$:'object',identity,callable:false}}:{$:'collectionInvalid',heap,reason:failure};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object
 * @param {JavaScriptWeakCollectionEntry[]} weakEntries @returns {JavaScriptOrdinaryHeap} */
function replaceJavaScriptWeakCollectionEntries(heap, object, weakEntries) {
 if (object.$!=='weakMapObject' && object.$!=='weakSetObject') return heap;
 const identity=object.identity,prototype=object.prototype,extensible=object.extensible,callable=object.callable,properties=object.properties;
 const replacement=object.$==='weakMapObject'
   ? {$:'weakMapObject',identity,prototype,extensible,callable,properties,weakEntries}
   : {$:'weakSetObject',identity,prototype,extensible,callable,properties,weakEntries};
 return replaceJavaScriptOrdinaryObject(heap,replacement);
}
/** @param {JavaScriptOrdinaryObject} object @param {string} kind
 * @returns {JavaScriptWeakCollectionEntry[]|undefined} */
function readJavaScriptWeakCollectionEntries(object, kind) {
 if (object.$==='weakMapObject') return kind==='WeakMap'?object.weakEntries:undefined;
 if (object.$==='weakSetObject') return kind==='WeakSet'?object.weakEntries:undefined;
 return undefined;
}
/** Receiver brand is tested before key eligibility. Primitive query keys do
 * not coerce or throw. Only insertion rejects an ineligible weak key.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {string} operation @param {JavaScriptSemanticValue[]} args @returns {JavaScriptCollectionCompletion} */
export function executeJavaScriptWeakCollectionOperation(heap,receiver,operation,args){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 const kind=readJavaScriptWeakCollectionOperationKind(operation);if(kind==='')return {$:'collectionInvalid',heap,reason:'weak collection operation retains its actual registered method owner'};
 if(args.length>4096)return {$:'collectionResourceLimit',heap,reason:'weak collection arguments exceed their explicit execution profile'};
 for(const value of [receiver,...args]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};}
 const object=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(object===undefined)return {$:'collectionTypeError',heap,reason:operation+' requires its actual weak internal slot receiver'};
 const stored=readJavaScriptWeakCollectionEntries(object,kind);
 if(stored===undefined)return {$:'collectionTypeError',heap,reason:operation+' requires its actual weak internal slot receiver'};
 const key=args[0]??{$:'undefined'},eligibility=readJavaScriptWeakKeyEligibility(heap,key),inserting=operation==='WeakMap.set'||operation==='WeakSet.add';
 if(eligibility==='missing symbol registration authority')return {$:'collectionInvalid',heap,reason:'weak Symbol keys require their actual registered or unregistered global Symbol fact'};
 if(eligibility==='ineligible')return inserting?{$:'collectionTypeError',heap,reason:'weak insertion requires an Object or nonregistered Symbol key'}:{$:'collectionNormal',heap,value:operation==='WeakMap.get'?{$:'undefined'}:{$:'boolean',value:false}};
 const at=findJavaScriptWeakCollectionKey(stored,key),entry=stored[at];
 if(operation==='WeakMap.get')return {$:'collectionNormal',heap,value:entry?.value??{$:'undefined'}};
 if(operation==='WeakMap.has'||operation==='WeakSet.has')return {$:'collectionNormal',heap,value:{$:'boolean',value:at>=0}};
 if(operation==='WeakMap.delete'||operation==='WeakSet.delete')return {$:'collectionNormal',heap:at<0?heap:replaceJavaScriptWeakCollectionEntries(heap,object,stored.filter((_,index)=>index!==at)),value:{$:'boolean',value:at>=0}};
 if(at<0&&stored.length>=65536)return {$:'collectionResourceLimit',heap,reason:'weak collection live slots exceed their explicit storage budget'};
 const value=kind==='WeakMap'?args[1]??{$:'undefined'}:{$:'undefined'},next={$:'weakCollectionEntry',key,value},weakEntries=at<0?stored.slice(0,-at-1).concat([next],stored.slice(-at-1)):stored.map((current,index)=>index===at?next:current),updated=replaceJavaScriptWeakCollectionEntries(heap,object,weakEntries),failure=readJavaScriptOrdinaryHeapError(updated);
 return failure===''?{$:'collectionNormal',heap:updated,value:receiver}:{$:'collectionResourceLimit',heap,reason:failure};
}
