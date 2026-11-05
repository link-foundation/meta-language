// String iteration advances exact UTF16 code points, preserving lone surrogates.
// Callers complete RequireObjectCoercible/ToString before iterator allocation.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,insertJavaScriptOrdinaryObject,replaceJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
import {readJavaScriptCollectionIteratorResultPrototypesError,readJavaScriptCollectionIteratorResultStateError} from './javascript-collection-iterator-results.js';
/** @typedef {{$:'stringIteratorAllocated',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'stringIteratorComponents',heap:JavaScriptOrdinaryHeap,receiver:JavaScriptSemanticValue,done:boolean,value:JavaScriptSemanticValue} |
 * {$:'stringIteratorTypeError',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'stringIteratorInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'stringIteratorResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptStringIterationTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} value @returns {JavaScriptStringIterationTransition} */
export function allocateJavaScriptStringIterator(heap,identity,prototype,value){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'stringIteratorInvalid',heap,reason:error};const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!==''||readJavaScriptSemanticTextLength(value)<0)return {$:'stringIteratorInvalid',heap,reason:invalid||'String iterator allocation retains its actual completed ToString value'};
 if(heap.objects.length>=4096)return {$:'stringIteratorResourceLimit',heap,reason:'String iterator allocation exceeds its actual shared Object budget'};
 let units=readJavaScriptSemanticTextLength(value);for(const row of heap.objects){if(row.$==='collectionIteratorObject'&&row.collectionKind==='String'&&row.iteratedText!==undefined)units+=Math.max(0,readJavaScriptSemanticTextLength(row.iteratedText));if(row.$==='mapObject'||row.$==='setObject')for(const entry of row.entries)units+=Math.max(0,readJavaScriptSemanticTextLength(entry.key))+Math.max(0,readJavaScriptSemanticTextLength(entry.value));}if(units>67108864)return {$:'stringIteratorResourceLimit',heap,reason:'String iterator text exceeds its actual shared UTF16 storage budget'};
 const allocated=insertJavaScriptOrdinaryObject(heap,identity,prototype);if(!allocated.success)return {$:'stringIteratorInvalid',heap,reason:allocated.error};
 const row=allocated.heap.objects[findJavaScriptOrdinaryObject(allocated.heap.objects,identity)];if(row?.$!=='ordinaryObject')return {$:'stringIteratorInvalid',heap,reason:'String iteration retains its actual fresh Object'};
 const iterator={...row,$:'collectionIteratorObject',collection:0n,cursor:0,kind:'values',collectionKind:'String',iteratedText:value},updated=replaceJavaScriptOrdinaryObject(allocated.heap,iterator),validation=readJavaScriptOrdinaryHeapError(updated);
 return validation===''?{$:'stringIteratorAllocated',heap:updated,value:{$:'object',identity,callable:false}}:{$:'stringIteratorInvalid',heap,reason:validation};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @returns {JavaScriptStringIterationTransition} */
export function advanceJavaScriptStringIterator(heap,receiver){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'stringIteratorInvalid',heap,reason:error};const invalid=readJavaScriptOrdinaryValueError(heap,receiver);if(invalid!=='')return {$:'stringIteratorInvalid',heap,reason:invalid};
 const iterator=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;if(iterator?.$!=='collectionIteratorObject'||iterator.collectionKind!=='String')return {$:'stringIteratorTypeError',heap,reason:'String Iterator next requires its actual internal-slot receiver'};
 const value=iterator.iteratedText;if(value===undefined)return {$:'stringIteratorInvalid',heap,reason:'String iteration retains its actual internal text slot'};
 if(value.$==='undefined')return {$:'stringIteratorComponents',heap,receiver,done:true,value:{$:'undefined'}};
 const length=readJavaScriptSemanticTextLength(value),index=iterator.cursor;
 if(index>=length)return {$:'stringIteratorComponents',heap:replaceJavaScriptOrdinaryObject(heap,{...iterator,cursor:0,iteratedText:{$:'undefined'}}),receiver,done:true,value:{$:'undefined'}};
 const first=readJavaScriptSemanticTextUnit(value,index),units=[first];let count=1;
 if(first>=55296&&first<=56319&&index+1<length){const second=readJavaScriptSemanticTextUnit(value,index+1);if(second>=56320&&second<=57343){units.push(second);count=2;}}
 return {$:'stringIteratorComponents',heap:replaceJavaScriptOrdinaryObject(heap,{...iterator,cursor:index+count}),receiver,done:false,value:{$:'text',units}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} arrayPrototype @param {JavaScriptSemanticValue} objectPrototype @returns {JavaScriptCollectionIteratorResultCompletion} */
export function beginJavaScriptStringIteratorResult(heap,receiver,arrayPrototype,objectPrototype){
 const error=readJavaScriptCollectionIteratorResultPrototypesError(heap,arrayPrototype,objectPrototype);if(error!=='')return {$:'iteratorResultInvalid',heap,reason:error};
 const next=advanceJavaScriptStringIterator(heap,receiver);if(next.$==='stringIteratorTypeError')return {$:'iteratorResultTypeError',heap,reason:next.reason};if(next.$!=='stringIteratorComponents')return {$:'iteratorResultInvalid',heap,reason:next.$==='stringIteratorInvalid'?next.reason:'String next retains its actual completed cursor transition'};
 const state={$:'collectionIteratorResultState',phase:'result',receiver,collectionKind:'String',arrayPrototype,objectPrototype,key:{$:'undefined'},value:next.value,done:next.done},invalid=readJavaScriptCollectionIteratorResultStateError(next.heap,state);
 return invalid===''?{$:'iteratorResultContinue',heap:next.heap,state}:{$:'iteratorResultInvalid',heap:next.heap,reason:invalid};
}
