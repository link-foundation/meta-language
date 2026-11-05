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
/** Receiver brand is tested before key eligibility. Primitive query keys do
 * not coerce or throw. Only insertion rejects an ineligible weak key.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {string} operation @param {JavaScriptSemanticValue[]} args @returns {JavaScriptCollectionCompletion} */
export function executeJavaScriptWeakCollectionOperation(heap,receiver,operation,args){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 const kind=readJavaScriptWeakCollectionOperationKind(operation);if(kind==='')return {$:'collectionInvalid',heap,reason:'weak collection operation retains its actual registered method owner'};
 if(args.length>4096)return {$:'collectionResourceLimit',heap,reason:'weak collection arguments exceed their explicit execution profile'};
 for(const value of [receiver,...args]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};}
 const object=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(object===undefined||!(kind==='WeakMap'&&object.$==='weakMapObject'||kind==='WeakSet'&&object.$==='weakSetObject'))return {$:'collectionTypeError',heap,reason:operation+' requires its actual weak internal slot receiver'};
 const key=args[0]??{$:'undefined'},eligibility=readJavaScriptWeakKeyEligibility(heap,key),inserting=operation==='WeakMap.set'||operation==='WeakSet.add';
 if(eligibility==='missing symbol registration authority')return {$:'collectionInvalid',heap,reason:'weak Symbol keys require their actual registered or unregistered global Symbol fact'};
 if(eligibility==='ineligible')return inserting?{$:'collectionTypeError',heap,reason:'weak insertion requires an Object or nonregistered Symbol key'}:{$:'collectionNormal',heap,value:operation==='WeakMap.get'?{$:'undefined'}:{$:'boolean',value:false}};
 const at=findJavaScriptWeakCollectionKey(object.weakEntries,key),entry=object.weakEntries[at];
 if(operation==='WeakMap.get')return {$:'collectionNormal',heap,value:entry?.value??{$:'undefined'}};
 if(operation==='WeakMap.has'||operation==='WeakSet.has')return {$:'collectionNormal',heap,value:{$:'boolean',value:at>=0}};
 if(operation==='WeakMap.delete'||operation==='WeakSet.delete')return {$:'collectionNormal',heap:at<0?heap:replaceJavaScriptOrdinaryObject(heap,{...object,weakEntries:object.weakEntries.filter((_,index)=>index!==at)}),value:{$:'boolean',value:at>=0}};
 if(at<0&&object.weakEntries.length>=65536)return {$:'collectionResourceLimit',heap,reason:'weak collection live slots exceed their explicit storage budget'};
 const value=kind==='WeakMap'?args[1]??{$:'undefined'}:{$:'undefined'},next={$:'weakCollectionEntry',key,value},weakEntries=at<0?object.weakEntries.slice(0,-at-1).concat([next],object.weakEntries.slice(-at-1)):object.weakEntries.map((current,index)=>index===at?next:current),updated=replaceJavaScriptOrdinaryObject(heap,{...object,weakEntries}),failure=readJavaScriptOrdinaryHeapError(updated);
 return failure===''?{$:'collectionNormal',heap:updated,value:receiver}:{$:'collectionResourceLimit',heap,reason:failure};
}
