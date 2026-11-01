// A next() result and an entries() pair are genuine allocated objects. The
// cursor advances once; identity suspension never repeats that advancement.
import {advanceJavaScriptBrandedCollectionIterator} from './javascript-collection-iteration.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
/** @typedef {{$:'collectionIteratorResultState',phase:string,receiver:JavaScriptSemanticValue,collectionKind:string,arrayPrototype:JavaScriptSemanticValue,objectPrototype:JavaScriptSemanticValue,key:JavaScriptSemanticValue,value:JavaScriptSemanticValue,done:boolean}} JavaScriptCollectionIteratorResultState */
/** @typedef {{$:'iteratorResultContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptCollectionIteratorResultState} |
 * {$:'iteratorResultAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptCollectionIteratorResultState,prototype:JavaScriptSemanticValue,kind:string} |
 * {$:'iteratorResultNormal',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'iteratorResultInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'iteratorResultTypeError',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'iteratorResultResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptCollectionIteratorResultState,reason:string}} JavaScriptCollectionIteratorResultCompletion */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} arrayPrototype @param {JavaScriptSemanticValue} objectPrototype @returns {string} */
export function readJavaScriptCollectionIteratorResultPrototypesError(heap,arrayPrototype,objectPrototype){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 for(const value of [arrayPrototype,objectPrototype]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(arrayPrototype.$!=='object'||arrayPrototype.callable||objectPrototype.$!=='object'||objectPrototype.callable)return 'iterator results retain their actual noncallable realm Array and Object prototypes';
 const array=heap.objects[findJavaScriptOrdinaryObject(heap.objects,arrayPrototype.identity)],ordinary=heap.objects[findJavaScriptOrdinaryObject(heap.objects,objectPrototype.identity)];
 return array===undefined||array.$!=='arrayObject'||ordinary===undefined||ordinary.$!=='ordinaryObject'?'iterator result prototypes require their actual realm Array and ordinary Object kinds':'';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionIteratorResultState} state @returns {string} */
export function readJavaScriptCollectionIteratorResultStateError(heap,state){
 const error=readJavaScriptCollectionIteratorResultPrototypesError(heap,state.arrayPrototype,state.objectPrototype);if(error!=='')return error;
 if(state.$!=='collectionIteratorResultState'||!['entry','result'].includes(state.phase)||typeof state.done!=='boolean'||state.phase==='entry'&&state.done||!['Map','Set','Array'].includes(state.collectionKind))return 'iterator result allocation requires its actual pending phase, operation brand and completion';
 for(const value of [state.receiver,state.key,state.value]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 const iterator=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
 if(iterator===undefined||iterator.$!=='collectionIteratorObject'||iterator.collectionKind!==state.collectionKind)return 'an iterator result retains its actual original receiver and operation brand';
 if(state.done&&(state.key.$!=='undefined'||state.value.$!=='undefined'))return 'an exhausted iterator yields its actual undefined value';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {string} collectionKind
 * @param {JavaScriptSemanticValue} arrayPrototype @param {JavaScriptSemanticValue} objectPrototype @returns {JavaScriptCollectionIteratorResultCompletion} */
export function beginJavaScriptCollectionIteratorResult(heap,receiver,collectionKind,arrayPrototype,objectPrototype){
 // Validate every independent operand before performing the cursor transition.
 const pending={$:'collectionIteratorResultState',phase:'result',receiver,collectionKind,arrayPrototype,objectPrototype,key:{$:'undefined'},value:{$:'undefined'},done:false};
 const error=readJavaScriptCollectionIteratorResultPrototypesError(heap,arrayPrototype,objectPrototype);if(error!=='')return {$:'iteratorResultInvalid',heap,reason:error};
 const next=advanceJavaScriptBrandedCollectionIterator(heap,receiver,collectionKind);
 if(next.$==='collectionIterationTypeError')return {$:'iteratorResultTypeError',heap,reason:next.reason};
 if(next.$!=='collectionIteration')return {$:'iteratorResultInvalid',heap,reason:'iterator result creation needs its actual completed cursor transition'};
 const state={...pending,phase:next.entry?'entry':'result',key:next.key,value:next.value,done:next.done};
 return {$:'iteratorResultContinue',heap:next.heap,state};
}
/** Reuse the actual result/entry allocator after the Array iterator's
 * getter/conversion continuation has completed and advanced its cursor once.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver
 * @param {boolean} done @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value
 * @param {boolean} entry @param {JavaScriptSemanticValue} arrayPrototype
 * @param {JavaScriptSemanticValue} objectPrototype @returns {JavaScriptCollectionIteratorResultCompletion} */
export function beginJavaScriptArrayIteratorResult(heap,receiver,done,key,value,entry,arrayPrototype,objectPrototype){
 const state={$:'collectionIteratorResultState',phase:entry?'entry':'result',receiver,collectionKind:'Array',arrayPrototype,objectPrototype,key,value,done},error=readJavaScriptCollectionIteratorResultStateError(heap,state);
 return error===''?{$:'iteratorResultContinue',heap,state}:{$:'iteratorResultInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionIteratorResultState} state @returns {JavaScriptCollectionIteratorResultCompletion} */
export function stepJavaScriptCollectionIteratorResult(heap,state){
 const error=readJavaScriptCollectionIteratorResultStateError(heap,state);if(error!=='')return {$:'iteratorResultInvalid',heap,reason:error};
 let properties=0;for(const object of heap.objects)properties+=object.properties.length;
 if(heap.objects.length>=4096||properties+(state.phase==='entry'?3:2)>16384)return {$:'iteratorResultResourceLimit',heap,state,reason:'iterator result allocation exceeds its actual shared object/property budget'};
 return {$:'iteratorResultAllocate',heap,state,prototype:state.phase==='entry'?state.arrayPrototype:state.objectPrototype,kind:state.phase==='entry'?'array':'ordinary'};
}
/** @param {JavaScriptSemanticValue} value @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptIteratorResultDescriptor(value){const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=value;descriptor.writable=true;return descriptor;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionIteratorResultState} state @param {bigint} identity @returns {JavaScriptCollectionIteratorResultCompletion} */
export function resumeJavaScriptCollectionIteratorResultAllocation(heap,state,identity){
 const request=stepJavaScriptCollectionIteratorResult(heap,state);if(request.$!=='iteratorResultAllocate')return request;
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n)return {$:'iteratorResultInvalid',heap,reason:'iterator result allocation retains its exact external unsigned64 identity'};
 if(state.phase==='entry'){
  const allocated=allocateJavaScriptArray(heap,identity,state.arrayPrototype,2);
  if(allocated.$==='arrayResourceLimit')return {$:'iteratorResultResourceLimit',heap,state,reason:allocated.reason};
  if(allocated.$!=='arrayNormal')return {$:'iteratorResultInvalid',heap,reason:'an entries pair needs its actual fresh global Array allocation'};
  const first=defineJavaScriptOrdinaryProperty(allocated.heap,identity,{$:'text',units:[48]},createJavaScriptIteratorResultDescriptor(state.key));
  const second=first.success?defineJavaScriptOrdinaryProperty(first.heap,identity,{$:'text',units:[49]},createJavaScriptIteratorResultDescriptor(state.value)):first;
  if(!first.success||!second.success||first.error!==''||second.error!=='')return {$:'iteratorResultInvalid',heap,reason:'fresh entry Array fields must retain their actual standard creation attributes'};
  return {$:'iteratorResultContinue',heap:second.heap,state:{...state,phase:'result',value:allocated.value}};
 }
 const allocated=insertJavaScriptOrdinaryObject(heap,identity,state.objectPrototype);
 if(!allocated.success)return {$:'iteratorResultInvalid',heap,reason:allocated.error};
 const value=defineJavaScriptOrdinaryProperty(allocated.heap,identity,{$:'text',units:[118,97,108,117,101]},createJavaScriptIteratorResultDescriptor(state.value));
 const done=value.success?defineJavaScriptOrdinaryProperty(value.heap,identity,{$:'text',units:[100,111,110,101]},createJavaScriptIteratorResultDescriptor({$:'boolean',value:state.done})):value;
 if(!value.success||!done.success||value.error!==''||done.error!=='')return {$:'iteratorResultInvalid',heap,reason:'a fresh iterator result must retain its actual ordered value/done data fields'};
 return {$:'iteratorResultNormal',heap:done.heap,value:{$:'object',identity,callable:false}};
}
