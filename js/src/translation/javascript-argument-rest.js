// Rest formal parameters copy the actual immutable Call arguments list.
// Parameter binding writes never replace values in that original argument list.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
/** @typedef {{$:'argumentRestState',prototype:JavaScriptSemanticValue,from:number,count:number,index:number,target:JavaScriptSemanticValue}} JavaScriptArgumentRestState */
/** @typedef {{$:'argumentRestContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArgumentRestState} |
 * {$:'argumentRestAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArgumentRestState,prototype:JavaScriptSemanticValue} |
 * {$:'argumentRestCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'argumentRestInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'argumentRestResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArgumentRestState,reason:string}} JavaScriptArgumentRestTransition */
/** The arguments input is the current frame's original Call list, not mutable
 * parameter bindings or a language-level arguments Object.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue[]} arguments_
 * @param {JavaScriptArgumentRestState} state @returns {string} */
export function readJavaScriptArgumentRestStateError(heap,arguments_,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(state.$!=='argumentRestState'||arguments_.length>1048576||!Number.isSafeInteger(state.from)||state.from<0||Object.is(state.from,-0)||state.from>1048576||state.count!==Math.max(0,arguments_.length-state.from)||!Number.isSafeInteger(state.index)||state.index<0||Object.is(state.index,-0)||state.index>state.count)return 'argument rest retains its actual frame argument range and copy coordinate';
 for(const value of [state.prototype,state.target,...arguments_]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.prototype.$!=='object'||state.prototype.callable)return 'argument rest retains its actual realm Array prototype';
 if(state.target.$==='undefined')return state.index===0?'':'argument rest cannot advance before its actual Array allocation';
 const target=state.target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)]:undefined;return target?.$==='arrayObject'?'':'argument rest retains its actual newly allocated Array';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue[]} arguments_
 * @param {number} from @param {JavaScriptSemanticValue} prototype @returns {JavaScriptArgumentRestTransition} */
export function beginJavaScriptArgumentRest(heap,arguments_,from,prototype){
 const state={$:'argumentRestState',prototype,from,count:Math.max(0,arguments_.length-from),index:0,target:{$:'undefined'}},error=readJavaScriptArgumentRestStateError(heap,arguments_,state);return error===''?{$:'argumentRestContinue',heap,state}:{$:'argumentRestInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue[]} arguments_
 * @param {JavaScriptArgumentRestState} state @returns {JavaScriptArgumentRestTransition} */
export function stepJavaScriptArgumentRest(heap,arguments_,state){
 const error=readJavaScriptArgumentRestStateError(heap,arguments_,state);if(error!=='')return {$:'argumentRestInvalid',heap,reason:error};
 if(state.target.$==='undefined'){let properties=0;for(const row of heap.objects)properties+=row.properties.length;if(heap.objects.length>=4096||properties>=16384)return {$:'argumentRestResourceLimit',heap,state,reason:'argument rest Array allocation exceeds its actual shared storage budget'};return {$:'argumentRestAllocate',heap,state,prototype:state.prototype};}
 if(state.index===state.count)return {$:'argumentRestCompleted',heap,value:state.target};
 const copied=createJavaScriptIndexedDataProperty(heap,state.target,state.index,arguments_[state.from+state.index]??{$:'undefined'});
 if(copied.$==='indexedPropertyResourceLimit')return {$:'argumentRestResourceLimit',heap:copied.heap,state,reason:copied.reason};
 if(copied.$!=='indexedPropertyCreated')return {$:'argumentRestInvalid',heap:copied.heap,reason:copied.reason};
 return {$:'argumentRestContinue',heap:copied.heap,state:{...state,index:state.index+1}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue[]} arguments_
 * @param {JavaScriptArgumentRestState} state @param {bigint} identity @returns {JavaScriptArgumentRestTransition} */
export function resumeJavaScriptArgumentRestAllocation(heap,arguments_,state,identity){
 const request=stepJavaScriptArgumentRest(heap,arguments_,state);if(request.$!=='argumentRestAllocate')return request;
 const created=allocateJavaScriptArray(heap,identity,state.prototype,state.count);
 if(created.$==='arrayNormal')return {$:'argumentRestContinue',heap:created.heap,state:{...state,target:created.value}};
 if(created.$==='arrayResourceLimit')return {$:'argumentRestResourceLimit',heap:created.heap,state,reason:created.reason};
 return {$:'argumentRestInvalid',heap:created.heap,reason:created.$==='arrayInvalid'?created.reason:'rest arguments require their actual fresh Array creation'};
}
