// ArrayAccumulation spread delegates every iterator effect to the shared protocol.
// Completed values remain held across property storage suspension.
import {beginJavaScriptSynchronousIterator,beginJavaScriptSynchronousIteratorStep,stepJavaScriptIteratorProtocol,resumeJavaScriptIteratorProtocolCall,resumeJavaScriptIteratorPrimitiveGet,readJavaScriptIteratorProtocolStateError,readJavaScriptSynchronousIteratorRecordError} from './javascript-iterator-protocol.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'arraySpreadState',target:JavaScriptSemanticValue,input:JavaScriptSemanticValue,iteratorKey:JavaScriptSemanticValue,index:number,phase:string,value:JavaScriptSemanticValue,protocol?:JavaScriptIteratorProtocolState,record?:JavaScriptSynchronousIteratorRecord}} JavaScriptArraySpreadState */
/** @typedef {{$:'arraySpreadContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpreadState} |
 * {$:'arraySpreadCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpreadState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arraySpreadPrimitiveGet',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpreadState,receiver:JavaScriptSemanticValue,key:JavaScriptSemanticValue} |
 * {$:'arraySpreadCompleted',heap:JavaScriptOrdinaryHeap,index:number} |
 * {$:'arraySpreadThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySpreadError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arraySpreadInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arraySpreadResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySpreadState,reason:string}} JavaScriptArraySpreadTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpreadState} state @returns {string} */
export function readJavaScriptArraySpreadStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 for(const value of [state.target,state.input,state.iteratorKey,state.value]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 const target=state.target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)]:undefined;
 if(state.$!=='arraySpreadState'||target?.$!=='arrayObject'||state.iteratorKey.$!=='symbol'||!Number.isSafeInteger(state.index)||state.index<0||Object.is(state.index,-0)||!['iterator','step','write'].includes(state.phase))return 'Array spread retains its actual target Array, iterator key, index and phase';
 if(state.record!==undefined){const invalid=readJavaScriptSynchronousIteratorRecordError(heap,state.record);if(invalid!=='')return invalid;}
 if(state.phase==='iterator'){
  if(state.protocol===undefined)return 'Array spread iterator work retains its actual continuation';const invalid=readJavaScriptIteratorProtocolStateError(heap,state.protocol);if(invalid!=='')return invalid;
  if(state.protocol.operation==='acquire'&&(!testJavaScriptSemanticSameValue(state.protocol.input,state.input)||!testJavaScriptSemanticSameValue(state.protocol.key,state.iteratorKey)))return 'Array spread acquisition retains its original operand and well-known key';
  if(state.protocol.operation==='step'&&(state.record===undefined||!testJavaScriptSemanticSameValue(state.record.iterator,state.protocol.record.iterator)||!testJavaScriptSemanticSameValue(state.record.nextMethod,state.protocol.record.nextMethod)))return 'Array spread advancement retains its actual cached iterator record';
  if(!['acquire','step'].includes(state.protocol.operation))return 'Array accumulation cannot add an unrelated iterator cleanup operation';
 }else if(state.protocol!==undefined||state.record===undefined)return 'completed spread iterator work retains its actual record without a stale protocol';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} input @param {number} index @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptArraySpreadTransition} */
export function beginJavaScriptArraySpread(heap,target,input,index,iteratorKey){
 const state={$:'arraySpreadState',target,input,index,iteratorKey,phase:'iterator',value:{$:'undefined'}},acquired=beginJavaScriptSynchronousIterator(heap,input,iteratorKey);
 // Validate independent target/index operands before any iterator getter/call.
 const provisional={...state,protocol:acquired.$==='iteratorProtocolContinue'?acquired.state:undefined};
 if(acquired.$==='iteratorProtocolContinue'){const error=readJavaScriptArraySpreadStateError(heap,provisional);if(error!=='')return {$:'arraySpreadInvalid',heap,reason:error};}
 else{const invalid=readJavaScriptOrdinaryValueError(heap,target),object=target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)]:undefined;if(invalid!==''||object?.$!=='arrayObject'||!Number.isSafeInteger(index)||index<0||Object.is(index,-0))return {$:'arraySpreadInvalid',heap,reason:invalid||'Array accumulation retains its actual target and index'};}
 return continueJavaScriptArraySpreadIterator(heap,state,acquired);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpreadState} state @param {JavaScriptIteratorProtocolTransition} transition @returns {JavaScriptArraySpreadTransition} */
export function continueJavaScriptArraySpreadIterator(heap,state,transition){
 const {protocol:previous,...base}=state;
 switch(transition.$){
  case 'iteratorProtocolContinue':return {$:'arraySpreadContinue',heap:transition.heap,state:{...base,phase:'iterator',protocol:transition.state}};
  case 'iteratorProtocolCall':return {$:'arraySpreadCall',heap:transition.heap,state:{...base,phase:'iterator',protocol:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'iteratorProtocolPrimitiveGet':return {$:'arraySpreadPrimitiveGet',heap:transition.heap,state:{...base,phase:'iterator',protocol:transition.state},receiver:transition.receiver,key:transition.key};
  case 'iteratorProtocolAcquired':return {$:'arraySpreadContinue',heap:transition.heap,state:{...base,phase:'step',record:transition.record}};
  case 'iteratorProtocolStep':return transition.done?{$:'arraySpreadCompleted',heap:transition.heap,index:state.index}:{$:'arraySpreadContinue',heap:transition.heap,state:{...base,phase:'write',record:transition.record,value:transition.value}};
  case 'iteratorProtocolThrow':return {$:'arraySpreadThrow',heap:transition.heap,value:transition.value};
  case 'iteratorProtocolError':return {$:'arraySpreadError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'iteratorProtocolInvalid':return {$:'arraySpreadInvalid',heap:transition.heap,reason:transition.reason};
  case 'iteratorProtocolClosed':return {$:'arraySpreadInvalid',heap:transition.heap,reason:'Array accumulation cannot substitute a cleanup completion for iterator advancement'};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpreadState} state @returns {JavaScriptArraySpreadTransition} */
export function stepJavaScriptArraySpread(heap,state){
 const error=readJavaScriptArraySpreadStateError(heap,state);if(error!=='')return {$:'arraySpreadInvalid',heap,reason:error};
 if(state.phase==='iterator')return state.protocol===undefined?{$:'arraySpreadInvalid',heap,reason:'spread retains its actual iterator continuation'}:continueJavaScriptArraySpreadIterator(heap,state,stepJavaScriptIteratorProtocol(heap,state.protocol));
 if(state.phase==='step')return state.record===undefined?{$:'arraySpreadInvalid',heap,reason:'spread retains its cached iterator record'}:continueJavaScriptArraySpreadIterator(heap,state,beginJavaScriptSynchronousIteratorStep(heap,state.record));
 if(state.index===9007199254740991)return {$:'arraySpreadResourceLimit',heap,state,reason:'Array accumulation exceeds its exact safe index coordinate budget'};
 const created=createJavaScriptIndexedDataProperty(heap,state.target,state.index,state.value);
 if(created.$==='indexedPropertyResourceLimit')return {$:'arraySpreadResourceLimit',heap:created.heap,state,reason:created.reason};
 if(created.$==='indexedPropertyError')return {$:'arraySpreadError',heap:created.heap,errorClass:'TypeError',reason:created.reason};
 if(created.$==='indexedPropertyInvalid')return {$:'arraySpreadInvalid',heap:created.heap,reason:created.reason};
 return {$:'arraySpreadContinue',heap:created.heap,state:{...state,phase:'step',index:state.index+1,value:{$:'undefined'}}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpreadState} state @param {JavaScriptPrimitiveConversionCompletion} completion @param {boolean} primitiveGet @returns {JavaScriptArraySpreadTransition} */
export function resumeJavaScriptArraySpread(heap,state,completion,primitiveGet){
 const error=readJavaScriptArraySpreadStateError(heap,state);if(error!=='')return {$:'arraySpreadInvalid',heap,reason:error};if(state.phase!=='iterator'||state.protocol===undefined)return {$:'arraySpreadInvalid',heap,reason:'spread completion requires its actual selected iterator operation'};
 return continueJavaScriptArraySpreadIterator(heap,state,primitiveGet?resumeJavaScriptIteratorPrimitiveGet(heap,state.protocol,completion):resumeJavaScriptIteratorProtocolCall(heap,state.protocol,completion));
}
