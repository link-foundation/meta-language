// Object Call ToObject creates actual wrapper slots and String own descriptors.
// The VM supplies an authenticated realm prototype and a fresh global link ID;
// neither native represented constructors nor prototype resemblance give slots.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,testJavaScriptOrdinarySymbol,insertJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
/** @typedef {{$:'primitiveWrapperState',input:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue}} JavaScriptPrimitiveWrapperState */
/** @typedef {{$:'primitiveWrapperAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptPrimitiveWrapperState,prototype:JavaScriptSemanticValue} |
 * {$:'primitiveWrapperCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'primitiveWrapperInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'primitiveWrapperResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptPrimitiveWrapperState,reason:string}} JavaScriptPrimitiveWrapperTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveWrapperState} state @returns {string} */
export function readJavaScriptPrimitiveWrapperStateError(heap,state){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return invalid;
 if(state.$!=='primitiveWrapperState'||state.input.$==='object')return 'wrapper allocation retains an actual primitive or nullish Object Call input';
 for(const value of [state.input,state.prototype]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.prototype.$!=='object'&&state.prototype.$!=='null')return 'wrapper allocation retains its actual selected realm Object prototype';
 if(state.prototype.$==='object'){const parent=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.prototype.identity)];if(parent!==undefined&&!['ordinaryObject','closureObject','arrayObject','mapObject','setObject','weakMapObject','weakSetObject','collectionIteratorObject'].includes(parent.$))return 'wrapper allocation with an opaque prototype requires its actual traversal protocol';}
 return '';
}
/** Inputs and state are already validated. Budgets are resource requests,
 * never synthetic language TypeError/RangeError or partial allocations.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveWrapperState} state @returns {string} */
function readJavaScriptPrimitiveWrapperWorkError(heap,state){
 if(heap.objects.length>=4096)return 'primitive wrapper allocation exceeds the selected 4096 Object identity profile';
 const length=readJavaScriptSemanticTextLength(state.input),properties=length>=0?length+1:0;
 if(properties>4096)return 'String wrapper allocation exceeds the selected 4096 own-property profile';
 let total=properties;for(const object of heap.objects)total+=object.properties.length;
 return total>16384?'primitive wrapper allocation exceeds the selected 16384 shared own-property profile':'';
}
/** Object inputs pass through without consulting or allocating a prototype.
 * Null/undefined are Object Call's empty-Object special case, not ToObject's
 * nullish TypeError; root callers must retain that distinct operation contract.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} prototype @returns {JavaScriptPrimitiveWrapperTransition} */
export function beginJavaScriptPrimitiveWrapper(heap,input,prototype){
 const invalid=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,input);if(invalid!=='')return {$:'primitiveWrapperInvalid',heap,reason:invalid};
 if(input.$==='object')return {$:'primitiveWrapperCompleted',heap,value:input};
 const state={$:'primitiveWrapperState',input,prototype},error=readJavaScriptPrimitiveWrapperStateError(heap,state);if(error!=='')return {$:'primitiveWrapperInvalid',heap,reason:error};
 const work=readJavaScriptPrimitiveWrapperWorkError(heap,state);return work!==''?{$:'primitiveWrapperResourceLimit',heap,state,reason:work}:{$:'primitiveWrapperAllocate',heap,state,prototype};
}
/** @param {number} index @returns {JavaScriptSemanticValue} */
function constructJavaScriptWrapperIndexKey(index){
 const reversed=[],units=[];let remaining=index;
 do{const digit=remaining%10;reversed.push(48+digit);remaining=Math.floor(remaining/10);}while(remaining>0);
 for(let at=reversed.length-1;at>=0;at-=1)units.push(reversed[at]??48);return {$:'text',units};
}
/** @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value
 * @param {boolean} enumerable @returns {JavaScriptOrdinaryProperty} */
function constructJavaScriptWrapperStringProperty(key,value,enumerable){
 const descriptor=createJavaScriptPropertyDescriptor('data',false,enumerable);descriptor.value=value;descriptor.writable=false;return {$:'ordinaryProperty',key,descriptor};
}
/** Allocation is atomic at this transition. The returned Object retains only
 * slots supplied by the actual primitive input, regardless of its prototype.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPrimitiveWrapperState} state
 * @param {bigint} identity @returns {JavaScriptPrimitiveWrapperTransition} */
export function resumeJavaScriptPrimitiveWrapper(heap,state,identity){
 const invalid=readJavaScriptPrimitiveWrapperStateError(heap,state);if(invalid!=='')return {$:'primitiveWrapperInvalid',heap,reason:invalid};
 const work=readJavaScriptPrimitiveWrapperWorkError(heap,state);if(work!=='')return {$:'primitiveWrapperResourceLimit',heap,state,reason:work};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||testJavaScriptOrdinarySymbol(heap.symbols,identity))return {$:'primitiveWrapperInvalid',heap,reason:'primitive wrappers require a distinct externally allocated unsigned64 global identity'};
 const length=readJavaScriptSemanticTextLength(state.input),properties=[];
 if(length>=0){
  for(let index=0;index<length;index+=1)properties.push(constructJavaScriptWrapperStringProperty(constructJavaScriptWrapperIndexKey(index),{$:'text',units:[readJavaScriptSemanticTextUnit(state.input,index)]},true));
  properties.push(constructJavaScriptWrapperStringProperty({$:'text',units:[108,101,110,103,116,104]},{$:'number',value:length},false));
 }
 const allocated=insertJavaScriptOrdinaryObject(heap,identity,state.prototype);if(allocated.error!=='')return {$:'primitiveWrapperInvalid',heap,reason:allocated.error};
 const input=state.input,objects=allocated.heap.objects.map(object=>object.identity!==identity||object.$!=='ordinaryObject'?object:length>=0?{...object,properties,stringData:input}:input.$==='null'||input.$==='undefined'?object:{...object,primitiveData:input});
 const next={...allocated.heap,objects},error=readJavaScriptOrdinaryHeapError(next);if(error!=='')return {$:'primitiveWrapperInvalid',heap,reason:error};
 return {$:'primitiveWrapperCompleted',heap:next,value:{$:'object',identity,callable:false}};
}
