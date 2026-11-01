// Search algorithms retain real getters and scalar conversion continuations.
import {readJavaScriptArrayLikeLength,readJavaScriptArraySearchIndex,testJavaScriptArraySearchOperation,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {testJavaScriptCollectionKey} from './javascript-collection-rules.js';
import {testJavaScriptCallStrictEqual} from './javascript-call-code.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
/** @typedef {{$:'arraySearchSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArraySearchCall */
/** @typedef {{$:'arraySearchState',operation:string,receiver:JavaScriptSemanticValue,searchElement:JavaScriptSemanticValue,fromIndex:JavaScriptSemanticValue,fromPresent:boolean,exoticKey:JavaScriptSemanticValue,length:number,index:number,phase:string,input:JavaScriptSemanticValue,conversion?:JavaScriptPrimitiveConversionState,call?:JavaScriptArraySearchCall}} JavaScriptArraySearchState */
/** @typedef {{$:'arraySearchContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySearchState} |
 * {$:'arraySearchCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySearchState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arraySearchCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySearchThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySearchError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arraySearchInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arraySearchResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySearchState,reason:string}} JavaScriptArraySearchTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state @returns {string} */
export function readJavaScriptArraySearchStateError(heap,state){
 const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
 if(!testJavaScriptArraySearchOperation(state.operation)||state.receiver.$!=='object'||state.exoticKey.$!=='symbol'||typeof state.fromPresent!=='boolean')return 'Array search retains its actual operation, Object receiver, argument presence and conversion Symbol';
 for(const value of [state.receiver,state.searchElement,state.fromIndex,state.exoticKey,state.input]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0)||!Number.isSafeInteger(state.index)||state.index< -1||state.index>state.length||Object.is(state.index,-0))return 'Array search retains its captured ToLength and bounded index';
 if(!['length','length getter','length conversion','length primitive','index conversion','index primitive','ready','element getter'].includes(state.phase))return 'Array search retains its actual selected phase';
 if(state.phase==='length conversion'||state.phase==='index conversion'){
  if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.input)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Array search numeric conversion retains its actual operand, hint and Symbol';
  const error=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(error!=='')return error;
 }else if(state.conversion!==undefined)return 'only Array search numeric conversion retains a conversion continuation';
 if(state.phase==='element getter'&&(state.index<0||state.index>=state.length))return 'Array search getter retains its actual in-range coordinate';
 if(state.call!==undefined){
  if(!['length getter','length conversion','index conversion','element getter'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length>3)return 'Array search retains its actual selected getter or conversion call';
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 }else if(state.phase==='length getter'||state.phase==='element getter')return 'Array search getter phases retain their selected call';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArraySearchTransition} */
export function requestJavaScriptArraySearchCall(heap,state,callee,receiver,arguments_){return {$:'arraySearchCall',heap,state:{...state,call:{$:'arraySearchSelectedCall',callee,receiver,arguments:arguments_}},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptArraySearchState} state @returns {JavaScriptSemanticValue} */
export function readJavaScriptArraySearchMissingValue(state){return state.operation==='Array.includes'?{$:'boolean',value:false}:state.operation==='Array.at'?{$:'undefined'}:{$:'number',value:-1};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state
 * @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArraySearchTransition} */
export function continueJavaScriptArraySearchNumber(heap,state,transition){
 const {call:priorCall,conversion:priorConversion,...base}=state,lengthPhase=state.phase==='length'||state.phase==='length getter'||state.phase==='length conversion'||state.phase==='length primitive';
 switch(transition.$){
  case 'scalarContinuation':return {$:'arraySearchContinue',heap,state:{...base,phase:lengthPhase?'length conversion':'index conversion',conversion:transition.state}};
  case 'scalarCall':return requestJavaScriptArraySearchCall(heap,{...base,phase:lengthPhase?'length conversion':'index conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arraySearchInvalid',heap,reason:'Array search numeric conversion retains its actual ToNumber result'};
   if(lengthPhase){
    const length=readJavaScriptArrayLikeLength(transition.value.value),next={...base,length,input:state.fromIndex,phase:'index primitive'};
    if(length===0&&state.operation!=='Array.at')return {$:'arraySearchCompleted',heap,value:readJavaScriptArraySearchMissingValue(state)};
    if(!state.fromPresent&&state.operation!=='Array.at')return {$:'arraySearchContinue',heap,state:{...next,input:{$:'undefined'},phase:'ready',index:readJavaScriptArraySearchIndex(state.operation,length,false,0)}};
    return continueJavaScriptArraySearchNumber(heap,next,beginJavaScriptScalarProtocol(heap,state.fromIndex,state.exoticKey,'coerce primitive number'));
   }
   return {$:'arraySearchContinue',heap,state:{...base,phase:'ready',input:{$:'undefined'},index:readJavaScriptArraySearchIndex(state.operation,state.length,state.fromPresent,transition.value.value)}};
  }
  case 'scalarThrow':return {$:'arraySearchThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arraySearchError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arraySearchInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arraySearchResourceLimit',heap,state:{...base,phase:lengthPhase?'length primitive':'index primitive',input:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue} searchElement @param {JavaScriptSemanticValue} fromIndex
 * @param {boolean} fromPresent @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArraySearchTransition} */
export function beginJavaScriptArraySearch(heap,operation,receiver,searchElement,fromIndex,fromPresent,exoticKey){
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'arraySearchError',heap,errorClass:'TypeError',reason:'Array search called on null or undefined'};
 const state={$:'arraySearchState',operation,receiver,searchElement,fromIndex,fromPresent,exoticKey,length:0,index:0,phase:'length',input:{$:'undefined'}};
 const error=readJavaScriptArraySearchStateError(heap,state);return error===''?{$:'arraySearchContinue',heap,state}:{$:'arraySearchInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state
 * @param {JavaScriptSemanticValue} value @returns {JavaScriptArraySearchTransition} */
export function completeJavaScriptArraySearchElement(heap,state,value){
 if(state.operation==='Array.at')return {$:'arraySearchCompleted',heap,value};
 const found=state.operation==='Array.includes'?testJavaScriptCollectionKey(value,state.searchElement):testJavaScriptCallStrictEqual(value,state.searchElement);
 if(found)return {$:'arraySearchCompleted',heap,value:state.operation==='Array.includes'?{$:'boolean',value:true}:{$:'number',value:state.index}};
 return {$:'arraySearchContinue',heap,state:{...state,phase:'ready',index:state.index+(state.operation==='Array.lastIndexOf'?-1:1)}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state @returns {JavaScriptArraySearchTransition} */
export function stepJavaScriptArraySearch(heap,state){
 const error=readJavaScriptArraySearchStateError(heap,state);if(error!=='')return {$:'arraySearchInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arraySearchCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='length conversion'||state.phase==='index conversion')return state.conversion===undefined?{$:'arraySearchInvalid',heap,reason:'Array search conversion requires its actual continuation'}:continueJavaScriptArraySearchNumber(heap,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='length primitive'||state.phase==='index primitive')return continueJavaScriptArraySearchNumber(heap,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.input));
 if(state.receiver.$!=='object')return {$:'arraySearchInvalid',heap,reason:'Array search requires its actual boxed Object receiver'};
 if(state.phase==='length'){
  const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,{$:'text',units:[108,101,110,103,116,104]},state.receiver);if(get.error!=='')return {$:'arraySearchInvalid',heap,reason:get.error};
  if(get.action==='call')return requestJavaScriptArraySearchCall(heap,{...state,phase:'length getter'},get.callee,get.receiver,get.arguments);
  if(get.action!=='value')return {$:'arraySearchInvalid',heap,reason:'Array search length requires its actual Get result'};
  return continueJavaScriptArraySearchNumber(heap,{...state,input:get.value},beginJavaScriptScalarProtocol(heap,get.value,state.exoticKey,'coerce primitive number'));
 }
 if(state.phase!=='ready')return {$:'arraySearchInvalid',heap,reason:'Array search requires its actual getter completion'};
 if(state.index<0||state.index>=state.length)return {$:'arraySearchCompleted',heap,value:readJavaScriptArraySearchMissingValue(state)};
 const name=String(state.index),units=[];for(let at=0;at<name.length;at++)units.push(name.charCodeAt(at));const key={$:'text',units};
 if(state.operation==='Array.indexOf'||state.operation==='Array.lastIndexOf'){
  const presence=readJavaScriptIndexedPropertyPresence(heap,state.receiver.identity,key);if(presence.error!=='')return {$:'arraySearchInvalid',heap,reason:presence.error};
  if(!presence.present)return {$:'arraySearchContinue',heap,state:{...state,index:state.index+(state.operation==='Array.lastIndexOf'?-1:1)}};
 }
 const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,key,state.receiver);if(get.error!=='')return {$:'arraySearchInvalid',heap,reason:get.error};
 if(get.action==='call')return requestJavaScriptArraySearchCall(heap,{...state,phase:'element getter'},get.callee,get.receiver,get.arguments);
 if(get.action!=='value')return {$:'arraySearchInvalid',heap,reason:'Array search element requires its actual Get result'};
 return completeJavaScriptArraySearchElement(heap,state,get.value);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySearchState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArraySearchTransition} */
export function resumeJavaScriptArraySearch(heap,state,completion){
 const error=readJavaScriptArraySearchStateError(heap,state);if(error!=='')return {$:'arraySearchInvalid',heap,reason:error};
 if(state.call===undefined)return {$:'arraySearchInvalid',heap,reason:'Array search resumption retains its actual selected child'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arraySearchInvalid',heap,reason:invalid};if(completion.$==='throwCompletion')return {$:'arraySearchThrow',heap,value:completion.value};
 const {call:priorCall,...base}=state;
 if(state.phase==='length getter')return continueJavaScriptArraySearchNumber(heap,{...base,input:completion.value},beginJavaScriptScalarProtocol(heap,completion.value,state.exoticKey,'coerce primitive number'));
 if(state.phase==='length conversion'||state.phase==='index conversion')return state.conversion===undefined?{$:'arraySearchInvalid',heap,reason:'Array search numeric child retains its actual conversion state'}:continueJavaScriptArraySearchNumber(heap,base,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.phase==='element getter')return completeJavaScriptArraySearchElement(heap,base,completion.value);
 return {$:'arraySearchInvalid',heap,reason:'Array search resumption retains an actual getter or conversion phase'};
}
