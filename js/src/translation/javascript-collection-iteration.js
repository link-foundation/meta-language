// Live insertion cursors belong to inspectable global heap objects. Deletions
// retain ordinals, additions remain visible, and exhaustion releases the source.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,replaceJavaScriptOrdinaryObject,testJavaScriptOrdinarySymbol} from './javascript-ordinary-heap.js';
/** Validate the receiver BEFORE asking an external allocator for an identity.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver
 * @param {string} collectionKind @returns {{error:string,typeError:boolean}} */
export function readJavaScriptCollectionIteratorReceiverError(heap,receiver,collectionKind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {error,typeError:false};
 if(!['Map','Set'].includes(collectionKind))return {error:'iterator creation needs its actual registered collection kind',typeError:false};
 const invalid=readJavaScriptOrdinaryValueError(heap,receiver);if(invalid!=='')return {error:invalid,typeError:false};
 const collection=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(collection===undefined||!(collectionKind==='Map'&&collection.$==='mapObject'||collectionKind==='Set'&&collection.$==='setObject'))return {error:'iterator creation needs its actual '+collectionKind+' internal-slot receiver',typeError:true};
 return {error:'',typeError:false};
}
/** The caller must allocate genuine iterator-result and entry-array objects
 * from these components before exposing an ECMAScript next() result.
 * @typedef {{$:'collectionIteration',heap:JavaScriptOrdinaryHeap,done:boolean,key:JavaScriptSemanticValue,value:JavaScriptSemanticValue,entry:boolean} |
 * {$:'collectionIterationInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionIterationTypeError',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionIterationResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionIteratorAllocated',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue}} JavaScriptCollectionIteration */
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} receiver
 * @param {string} collectionKind @param {string} kind @returns {JavaScriptCollectionIteration} */
export function allocateJavaScriptCollectionIterator(heap,identity,prototype,receiver,collectionKind,kind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionIterationInvalid',heap,reason:error};
 if(!['keys','values','entries'].includes(kind)||!['Map','Set'].includes(collectionKind))return {$:'collectionIterationInvalid',heap,reason:'collection iteration needs its actual registered operation kind'};
 for(const value of [prototype,receiver]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionIterationInvalid',heap,reason:invalid};}
 const collection=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(collection===undefined||!(collectionKind==='Map'&&collection.$==='mapObject'||collectionKind==='Set'&&collection.$==='setObject'))return {$:'collectionIterationTypeError',heap,reason:'collection iteration needs its actual branded collection receiver'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'collectionIterationInvalid',heap,reason:'collection iterators retain their actual registered prototype'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||testJavaScriptOrdinarySymbol(heap.symbols,identity))return {$:'collectionIterationInvalid',heap,reason:'iterator allocation needs its fresh external global identity'};
 if(heap.objects.length>=4096)return {$:'collectionIterationResourceLimit',heap,reason:'iterator allocation exceeds the global object directory budget'};
 const iterator={$:'collectionIteratorObject',identity,prototype,extensible:true,callable:false,properties:[],collection:collection.identity,cursor:0,kind,collectionKind};
 let at=0;while(at<heap.objects.length&&(heap.objects[at]?.identity??identity)<identity)at++;
 const updated={...heap,objects:heap.objects.slice(0,at).concat([iterator],heap.objects.slice(at))},invalidPrototype=readJavaScriptOrdinaryHeapError(updated);
 if(invalidPrototype!=='')return {$:'collectionIterationInvalid',heap,reason:invalidPrototype};
 return {$:'collectionIteratorAllocated',heap:updated,value:{$:'object',identity,callable:false}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver
 * @returns {JavaScriptCollectionIteration} */
export function advanceJavaScriptCollectionIterator(heap,receiver){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionIterationInvalid',heap,reason:error};
 const invalid=readJavaScriptOrdinaryValueError(heap,receiver);if(invalid!=='')return {$:'collectionIterationInvalid',heap,reason:invalid};
 const iterator=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(iterator===undefined||iterator.$!=='collectionIteratorObject'||!['Map','Set'].includes(iterator.collectionKind))return {$:'collectionIterationTypeError',heap,reason:'collection iterator next needs its actual Map or Set internal-slot receiver'};
 if(iterator.collection===0n)return {$:'collectionIteration',heap,done:true,key:{$:'undefined'},value:{$:'undefined'},entry:false};
 const collection=heap.objects[findJavaScriptOrdinaryObject(heap.objects,iterator.collection)];
 if(collection===undefined||collection.$!=='mapObject'&&collection.$!=='setObject')return {$:'collectionIterationInvalid',heap,reason:'the iterator source must remain in the actual shared heap'};
 for(let cursor=iterator.cursor;cursor<collection.entries.length;cursor++){
  const entry=collection.entries[cursor];if(entry===undefined||!entry.present)continue;
  const updated=replaceJavaScriptOrdinaryObject(heap,{...iterator,cursor:cursor+1});
  return {$:'collectionIteration',heap:updated,done:false,key:entry.key,value:iterator.kind==='keys'?entry.key:entry.value,entry:iterator.kind==='entries'};
 }
 return {$:'collectionIteration',heap:replaceJavaScriptOrdinaryObject(heap,{...iterator,collection:0n,cursor:0}),done:true,key:{$:'undefined'},value:{$:'undefined'},entry:false};
}

/** The brand survives exhaustion; one prototype's next cannot resume another
 * iterator family even after its source collection has been released.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver
 * @param {string} collectionKind @returns {JavaScriptCollectionIteration} */
export function advanceJavaScriptBrandedCollectionIterator(heap,receiver,collectionKind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionIterationInvalid',heap,reason:error};
 if(!['Map','Set'].includes(collectionKind))return {$:'collectionIterationInvalid',heap,reason:'iterator next needs its actual registered operation brand'};
 const invalid=readJavaScriptOrdinaryValueError(heap,receiver);if(invalid!=='')return {$:'collectionIterationInvalid',heap,reason:invalid};
 const iterator=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(iterator===undefined||iterator.$!=='collectionIteratorObject'||iterator.collectionKind!==collectionKind)return {$:'collectionIterationTypeError',heap,reason:'iterator next requires its actual '+collectionKind+' iterator internal brand'};
 return advanceJavaScriptCollectionIterator(heap,receiver);
}
