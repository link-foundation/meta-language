// Array iterator internal slots live on genuine common-heap iterator Objects.
// Each next call reads live length and values; keys never read element getters.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,insertJavaScriptOrdinaryObject,replaceJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptArrayLikeLength} from './javascript-array-index-rules.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'arrayIteratorCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArrayIteratorCall */
/** @typedef {{$:'arrayIteratorState',receiver:JavaScriptSemanticValue,source:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,index:number,kind:string,phase:string,input:JavaScriptSemanticValue,conversion?:JavaScriptPrimitiveConversionState,call?:JavaScriptArrayIteratorCall}} JavaScriptArrayIteratorState */
/** @typedef {{$:'arrayIteratorAllocated',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayIteratorContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayIteratorState} |
 * {$:'arrayIteratorCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayIteratorState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayIteratorComponents',heap:JavaScriptOrdinaryHeap,receiver:JavaScriptSemanticValue,done:boolean,key:JavaScriptSemanticValue,value:JavaScriptSemanticValue,entry:boolean} |
 * {$:'arrayIteratorAllocationResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayIteratorNeedBoxing',heap:JavaScriptOrdinaryHeap,receiver:JavaScriptSemanticValue} |
 * {$:'arrayIteratorThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayIteratorError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayIteratorInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayIteratorResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayIteratorState,reason:string}} JavaScriptArrayIterationTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} receiver @param {string} kind @returns {JavaScriptArrayIterationTransition} */
export function allocateJavaScriptArrayIterator(heap,identity,prototype,receiver,kind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'arrayIteratorInvalid',heap,reason:error};const invalid=readJavaScriptOrdinaryValueError(heap,receiver);if(invalid!=='')return {$:'arrayIteratorInvalid',heap,reason:invalid};
 if(!['keys','values','entries'].includes(kind))return {$:'arrayIteratorInvalid',heap,reason:'Array iteration retains its actual registered iteration kind'};
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'arrayIteratorError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};
 if(receiver.$!=='object')return {$:'arrayIteratorNeedBoxing',heap,receiver};
 if(heap.objects.length>=4096)return {$:'arrayIteratorAllocationResourceLimit',heap,reason:'Array iterator allocation exceeds the actual shared Object budget'};
 const allocated=insertJavaScriptOrdinaryObject(heap,identity,prototype);if(!allocated.success)return {$:'arrayIteratorInvalid',heap,reason:allocated.error};
 const index=findJavaScriptOrdinaryObject(allocated.heap.objects,identity),row=allocated.heap.objects[index];if(row===undefined||row.$!=='ordinaryObject')return {$:'arrayIteratorInvalid',heap,reason:'Array iterator allocation retains its actual fresh Object'};
 const iterator={...row,$:'collectionIteratorObject',collection:receiver.identity,cursor:0,kind,collectionKind:'Array',executing:false},updated=replaceJavaScriptOrdinaryObject(allocated.heap,iterator),validation=readJavaScriptOrdinaryHeapError(updated);
 return validation===''?{$:'arrayIteratorAllocated',heap:updated,value:{$:'object',identity,callable:false}}:{$:'arrayIteratorInvalid',heap,reason:validation};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @returns {string} */
export function readJavaScriptArrayIteratorStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 for(const value of [state.receiver,state.source,state.exoticKey,state.input]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 const iterator=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
 if(state.$!=='arrayIteratorState'||iterator?.$!=='collectionIteratorObject'||iterator.collectionKind!=='Array'||iterator.executing!==true||state.source.$!=='object'||iterator.collection!==state.source.identity||iterator.cursor!==state.index||iterator.kind!==state.kind||state.exoticKey.$!=='symbol'||!['length','length getter','conversion','primitive','value','value getter'].includes(state.phase))return 'Array next retains its actual executing iterator, source, index and phase';
 if(state.phase==='conversion'){
  if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.input)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Array iterator length conversion retains its actual input, hint and Symbol';
  const invalid=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(invalid!=='')return invalid;
 }else if(state.conversion!==undefined)return 'only length conversion retains its primitive continuation';
 if(state.call!==undefined){if(!['length getter','conversion','value getter'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable)return 'Array iterator callbacks retain their actual selected getter/conversion call';for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}}
 else if(state.phase.endsWith('getter'))return 'Array iterator getter phases retain their actual selected child';return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArrayIterationTransition} */
export function beginJavaScriptArrayIteratorNext(heap,receiver,exoticKey){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'arrayIteratorInvalid',heap,reason:error};for(const value of [receiver,exoticKey]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'arrayIteratorInvalid',heap,reason:invalid};}if(exoticKey.$!=='symbol')return {$:'arrayIteratorInvalid',heap,reason:'Array next retains its actual primitive conversion Symbol'};
 const iterator=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(iterator?.$!=='collectionIteratorObject'||iterator.collectionKind!=='Array')return {$:'arrayIteratorError',heap,errorClass:'TypeError',reason:'Array Iterator next requires its actual internal-slot receiver'};
 if(iterator.executing)return {$:'arrayIteratorError',heap,errorClass:'TypeError',reason:'Generator is already running'};
 if(iterator.collection===0n)return {$:'arrayIteratorComponents',heap,receiver,done:true,key:{$:'undefined'},value:{$:'undefined'},entry:false};
 const source=heap.objects[findJavaScriptOrdinaryObject(heap.objects,iterator.collection)];if(source===undefined)return {$:'arrayIteratorInvalid',heap,reason:'Array iterator source remains in the common heap'};
 return {$:'arrayIteratorContinue',heap:replaceJavaScriptOrdinaryObject(heap,{...iterator,executing:true}),state:{$:'arrayIteratorState',receiver,source:{$:'object',identity:source.identity,callable:source.callable},exoticKey,index:iterator.cursor,kind:iterator.kind,phase:'length',input:{$:'undefined'}}};
}
/** Release or advance only after the selected next operation has completed.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @param {boolean} done @returns {JavaScriptOrdinaryHeap} */
export function finishJavaScriptArrayIteratorCursor(heap,state,done){
 const iterator=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;if(iterator?.$!=='collectionIteratorObject')return heap;
 return replaceJavaScriptOrdinaryObject(heap,{...iterator,collection:done?0n:iterator.collection,cursor:done?0:state.index+1,executing:false});
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArrayIterationTransition} */
export function continueJavaScriptArrayIteratorLength(heap,state,transition){
 const {call:previousCall,conversion:previousConversion,...base}=state;
 switch(transition.$){
  case 'scalarContinuation':return {$:'arrayIteratorContinue',heap,state:{...base,phase:'conversion',conversion:transition.state}};
  case 'scalarCall':{const call={$:'arrayIteratorCall',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'arrayIteratorCall',heap,state:{...base,phase:'conversion',conversion:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arrayIteratorInvalid',heap,reason:'Array iterator length retains its actual ToNumber result'};
   if(state.index>=readJavaScriptArrayLikeLength(transition.value.value))return {$:'arrayIteratorComponents',heap:finishJavaScriptArrayIteratorCursor(heap,state,true),receiver:state.receiver,done:true,key:{$:'undefined'},value:{$:'undefined'},entry:false};
   if(state.kind==='keys')return {$:'arrayIteratorComponents',heap:finishJavaScriptArrayIteratorCursor(heap,state,false),receiver:state.receiver,done:false,key:{$:'number',value:state.index},value:{$:'number',value:state.index},entry:false};
   return {$:'arrayIteratorContinue',heap,state:{...base,phase:'value',input:{$:'undefined'}}};
  }
  case 'scalarThrow':return {$:'arrayIteratorThrow',heap:finishJavaScriptArrayIteratorCursor(heap,state,true),value:transition.value};
  case 'scalarIntrinsicError':return {$:'arrayIteratorError',heap:finishJavaScriptArrayIteratorCursor(heap,state,true),errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arrayIteratorInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayIteratorResourceLimit',heap,state:{...base,phase:'primitive',input:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayIterationTransition} */
export function continueJavaScriptArrayIteratorValue(heap,state,value){
 const {call:previous,...base}=state;
 if(state.phase==='length'||state.phase==='length getter'){
  const next={...base,input:value};return continueJavaScriptArrayIteratorLength(heap,next,value.$==='object'?beginJavaScriptScalarProtocol(heap,value,state.exoticKey,'coerce primitive number'):finishJavaScriptScalarPrimitive('coerce primitive number',value));
 }
 return {$:'arrayIteratorComponents',heap:finishJavaScriptArrayIteratorCursor(heap,state,false),receiver:state.receiver,done:false,key:{$:'number',value:state.index},value,entry:state.kind==='entries'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @returns {JavaScriptArrayIterationTransition} */
export function stepJavaScriptArrayIteratorNext(heap,state){
 const error=readJavaScriptArrayIteratorStateError(heap,state);if(error!=='')return {$:'arrayIteratorInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arrayIteratorCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='conversion')return state.conversion===undefined?{$:'arrayIteratorInvalid',heap,reason:'Array iterator conversion retains its actual continuation'}:continueJavaScriptArrayIteratorLength(heap,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='primitive')return continueJavaScriptArrayIteratorLength(heap,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.input));
 const key=state.phase==='length'?{$:'text',units:[108,101,110,103,116,104]}:{$:'text',units:Array.from(String(state.index),ch=>ch.charCodeAt(0))};if(state.source.$!=='object')return {$:'arrayIteratorInvalid',heap,reason:'Array iteration retains its actual source Object'};
 const get=planJavaScriptOrdinaryGet(heap,state.source.identity,key,state.source);if(get.error!=='')return {$:'arrayIteratorInvalid',heap,reason:get.error};
 if(get.action==='call'){const call={$:'arrayIteratorCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'arrayIteratorCall',heap,state:{...state,phase:state.phase+' getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
 return get.action==='value'?continueJavaScriptArrayIteratorValue(heap,state,get.value):{$:'arrayIteratorInvalid',heap,reason:'Array iterator property access requires its actual getter/value protocol'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayIteratorState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayIterationTransition} */
export function resumeJavaScriptArrayIteratorCall(heap,state,completion){
 const error=readJavaScriptArrayIteratorStateError(heap,state);if(error!=='')return {$:'arrayIteratorInvalid',heap,reason:error};if(state.call===undefined)return {$:'arrayIteratorInvalid',heap,reason:'Array iterator resumption retains its actual selected child'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayIteratorInvalid',heap,reason:invalid};
 if(state.phase==='conversion')return state.conversion===undefined?{$:'arrayIteratorInvalid',heap,reason:'Array iterator conversion retains its actual selected continuation'}:continueJavaScriptArrayIteratorLength(heap,state,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(completion.$==='throwCompletion')return {$:'arrayIteratorThrow',heap:finishJavaScriptArrayIteratorCursor(heap,state,true),value:completion.value};
 return continueJavaScriptArrayIteratorValue(heap,state,completion.value);
}
