// Direct MapData/SetData traversal uses the shared insertion history. No
// iterator allocation, Symbol.iterator lookup or user next() participates.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {readJavaScriptCollectionIteratorReceiverError} from './javascript-collection-iteration.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'collectionForEachSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptCollectionForEachCall */
/** @typedef {{$:'collectionForEachState',operation:string,receiver:JavaScriptSemanticValue,callback:JavaScriptSemanticValue,thisArgument:JavaScriptSemanticValue,cursor:number,phase:string,key:JavaScriptSemanticValue,value:JavaScriptSemanticValue,call?:JavaScriptCollectionForEachCall}} JavaScriptCollectionForEachState */
/** @typedef {{$:'collectionForEachContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptCollectionForEachState} |
 * {$:'collectionForEachCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptCollectionForEachState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'collectionForEachCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'collectionForEachThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'collectionForEachError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'collectionForEachInvalid',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptCollectionForEachTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptCollectionForEachOperation(operation){return operation==='Map.forEach'||operation==='Set.forEach';}
/** @param {string} operation @returns {string} */
export function readJavaScriptCollectionForEachKind(operation){return operation==='Map.forEach'?'Map':operation==='Set.forEach'?'Set':'';}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptCollectionForEachFunction(identity,realm,operation){
 if(!testJavaScriptCollectionForEachOperation(operation))return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'collectionForEach',destination:1,operation},{$:'return',source:1}]};
}
/** Pending values are captured before entering the callback. They deliberately
 * are NOT checked against the latest entry, which may have been changed,
 * deleted or cleared by that callback. The ordinal history itself is retained.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionForEachState} state @returns {string} */
export function readJavaScriptCollectionForEachStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(state.$!=='collectionForEachState'||!testJavaScriptCollectionForEachOperation(state.operation))return 'collection forEach retains its actual registered operation';
 const kind=readJavaScriptCollectionForEachKind(state.operation),receiver=readJavaScriptCollectionIteratorReceiverError(heap,state.receiver,kind);if(receiver.error!=='')return receiver.error;
 for(const value of [state.callback,state.thisArgument,state.key,state.value]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.callback.$!=='object'||!state.callback.callable)return 'collection forEach retains its actual callable callback';
 const collection=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
 if(collection===undefined||collection.$!=='mapObject'&&collection.$!=='setObject')return 'collection forEach retains its actual shared internal-slot object';
 if(!Number.isSafeInteger(state.cursor)||state.cursor<0||Object.is(state.cursor,-0)||state.cursor>collection.entries.length)return 'collection forEach cursor retains its exact insertion-history coordinate';
 if(!['ready','callback'].includes(state.phase))return 'collection forEach retains its actual traversal or callback phase';
 if(state.phase==='ready')return state.call!==undefined||state.key.$!=='undefined'||state.value.$!=='undefined'?'a ready collection forEach has no stale selected call or held entry':'';
 if(state.cursor===0||state.call===undefined)return 'a suspended collection forEach retains its already-advanced ordinal and selected call';
 const call=state.call,arguments_=call.arguments;
 if(call.$!=='collectionForEachSelectedCall'||!Array.isArray(arguments_)||arguments_.length!==3||!testJavaScriptSemanticSameValue(call.callee,state.callback)||!testJavaScriptSemanticSameValue(call.receiver,state.thisArgument))return 'collection forEach retains its actual callback, thisArgument and three operands';
 const expected=[state.value,kind==='Set'?state.value:state.key,state.receiver];
 for(let at=0;at<expected.length;at+=1)if(!testJavaScriptSemanticSameValue(arguments_[at]??{$:'undefined'},expected[at]??{$:'undefined'}))return 'collection forEach retains its actual ordered value key and collection operands';
 if(kind==='Set'&&!testJavaScriptSemanticSameValue(state.key,state.value))return 'Set forEach supplies its actual value in both key and value positions';
 return '';
}
/** Validate the internal-slot receiver before checking callback callability,
 * including for empty collections. thisArgument is never coerced here.
 * @param {JavaScriptOrdinaryHeap} heap @param {string} operation
 * @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} callback
 * @param {JavaScriptSemanticValue} thisArgument @returns {JavaScriptCollectionForEachTransition} */
export function beginJavaScriptCollectionForEach(heap,operation,receiver,callback,thisArgument){
 if(!testJavaScriptCollectionForEachOperation(operation))return {$:'collectionForEachInvalid',heap,reason:'collection forEach requires its actual standard operation'};
 const kind=readJavaScriptCollectionForEachKind(operation),checked=readJavaScriptCollectionIteratorReceiverError(heap,receiver,kind);
 if(checked.error!=='')return checked.typeError?{$:'collectionForEachError',heap,errorClass:'TypeError',reason:operation+' requires its actual collection internal slot receiver'}:{$:'collectionForEachInvalid',heap,reason:checked.error};
 for(const value of [callback,thisArgument]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionForEachInvalid',heap,reason:invalid};}
 if(callback.$!=='object'||!callback.callable)return {$:'collectionForEachError',heap,errorClass:'TypeError',reason:'collection forEach callback must be callable'};
 return {$:'collectionForEachContinue',heap,state:{$:'collectionForEachState',operation,receiver,callback,thisArgument,cursor:0,phase:'ready',key:{$:'undefined'},value:{$:'undefined'}}};
}
/** One ordinal per transition bounds work and allows the VM's existing shared
 * step/resource suspension. Every transition reads the latest collection data,
 * so appended entries and future value replacements remain observable.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionForEachState} state @returns {JavaScriptCollectionForEachTransition} */
export function stepJavaScriptCollectionForEach(heap,state){
 const error=readJavaScriptCollectionForEachStateError(heap,state);if(error!=='')return {$:'collectionForEachInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'collectionForEachCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 const collection=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
 if(collection===undefined||collection.$!=='mapObject'&&collection.$!=='setObject')return {$:'collectionForEachInvalid',heap,reason:'collection forEach needs its actual retained internal-slot source'};
 if(state.cursor===collection.entries.length)return {$:'collectionForEachCompleted',heap,value:{$:'undefined'}};
 const entry=collection.entries[state.cursor];if(entry===undefined)return {$:'collectionForEachInvalid',heap,reason:'collection forEach requires its actual dense insertion history'};
 const cursor=state.cursor+1;if(!entry.present)return {$:'collectionForEachContinue',heap,state:{...state,cursor}};
 const key=entry.key,value=collection.$==='setObject'?entry.key:entry.value,arguments_=[value,collection.$==='setObject'?value:key,state.receiver],call={$:'collectionForEachSelectedCall',callee:state.callback,receiver:state.thisArgument,arguments:arguments_};
 return {$:'collectionForEachCall',heap,state:{...state,cursor,phase:'callback',key,value,call},callee:call.callee,receiver:call.receiver,arguments:arguments_};
}
/** The latest heap is retained on both completions. Callback return values are
 * ignored; an abrupt completion escapes immediately without IteratorClose.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptCollectionForEachState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptCollectionForEachTransition} */
export function resumeJavaScriptCollectionForEach(heap,state,completion){
 const error=readJavaScriptCollectionForEachStateError(heap,state);if(error!=='')return {$:'collectionForEachInvalid',heap,reason:error};
 if(state.phase!=='callback'||state.call===undefined)return {$:'collectionForEachInvalid',heap,reason:'only an actual selected collection callback can complete'};
 if(completion.$!=='normalCompletion'&&completion.$!=='throwCompletion')return {$:'collectionForEachInvalid',heap,reason:'collection callbacks retain their actual normal or throw completion'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'collectionForEachInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'collectionForEachThrow',heap,value:completion.value};
 const {call,...base}=state;return {$:'collectionForEachContinue',heap,state:{...base,phase:'ready',key:{$:'undefined'},value:{$:'undefined'}}};
}
