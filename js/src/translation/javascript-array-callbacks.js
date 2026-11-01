import {readJavaScriptArrayLikeLength,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
// Shared inspectable-object Array callback algorithms without species creation.
// Boxed primitive receivers and opaque HasProperty need explicit protocols.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
/** @typedef {{$:'arrayCallbackSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArrayCallbackCall */
/** @typedef {{$:'arrayCallbackState',operation:string,receiver:JavaScriptSemanticValue,callback:JavaScriptSemanticValue,thisArgument:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,length:number,index:number,phase:string,initialPresent:boolean,hasAccumulator:boolean,accumulator:JavaScriptSemanticValue,element:JavaScriptSemanticValue,conversion?:JavaScriptPrimitiveConversionState,call?:JavaScriptArrayCallbackCall}} JavaScriptArrayCallbackState */
/** @typedef {{$:'arrayCallbackContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCallbackState} |
 * {$:'arrayCallbackCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCallbackState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayCallbackCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayCallbackThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayCallbackError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayCallbackInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayCallbackResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCallbackState,reason:string}} JavaScriptArrayCallbackTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCallbackOperation(operation){return ['Array.forEach','Array.some','Array.every','Array.find','Array.findIndex','Array.findLast','Array.findLastIndex','Array.reduce','Array.reduceRight'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCallbackReverse(operation){return operation==='Array.findLast'||operation==='Array.findLastIndex'||operation==='Array.reduceRight';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCallbackVisitHoles(operation){return operation==='Array.find'||operation==='Array.findIndex'||operation==='Array.findLast'||operation==='Array.findLastIndex';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCallbackReduction(operation){return operation==='Array.reduce'||operation==='Array.reduceRight';}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state @returns {string} */
export function readJavaScriptArrayCallbackStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(!testJavaScriptArrayCallbackOperation(state.operation)||state.receiver.$!=='object'||state.exoticKey.$!=='symbol')return 'Array callbacks retain their actual operation, Object receiver and conversion Symbol';
 for(const value of [state.receiver,state.callback,state.thisArgument,state.exoticKey,state.element,state.accumulator]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(typeof state.initialPresent!=='boolean'||typeof state.hasAccumulator!=='boolean'||!testJavaScriptArrayCallbackReduction(state.operation)&&state.hasAccumulator)return 'Array reductions retain their actual initial-value presence and accumulator state';
 if(!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0)||!Number.isSafeInteger(state.index)||state.index< -1||state.index>state.length||Object.is(state.index,-0))return 'Array callback coordinates retain their captured ToLength and bounded exact index';
 if(!['length','length getter','length primitive','length conversion','ready','element getter','callback'].includes(state.phase))return 'Array callbacks retain their actual source continuation phase';
 if(state.phase==='length conversion'){if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.element)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Array length conversion retains its actual number-hint state';const invalid=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(invalid!=='')return invalid;}else if(state.conversion!==undefined)return 'only Array length conversion retains a primitive conversion continuation';
 if(['element getter','callback'].includes(state.phase)&&(state.index<0||state.index>=state.length))return 'Array element callbacks retain an actual in-range captured coordinate';
 if(['ready','element getter','callback'].includes(state.phase)&&(state.callback.$!=='object'||!state.callback.callable))return 'an initialized Array iteration retains its actual callable predicate';
 if(state.call!==undefined){
  if(!['length getter','length conversion','element getter','callback'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length>4)return 'Array callbacks retain their actual pending callable and bounded operands';
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
  if(state.phase==='callback'){
   const reduction=testJavaScriptArrayCallbackReduction(state.operation),expected=reduction?[state.accumulator,state.element,{$:'number',value:state.index},state.receiver]:[state.element,{$:'number',value:state.index},state.receiver],receiver=reduction?{$:'undefined'}:state.thisArgument,args=state.call.arguments;
   if(!testJavaScriptSemanticSameValue(state.call.callee,state.callback)||!testJavaScriptSemanticSameValue(state.call.receiver,receiver)||args.length!==expected.length||reduction&&!state.hasAccumulator)return 'Array predicate/reducer calls retain their actual callback and receiver convention';
   for(let at=0;at<expected.length;at++)if(!testJavaScriptSemanticSameValue(args[at]??{$:'undefined'},expected[at]??{$:'undefined'}))return 'Array predicate/reducer calls retain their actual ordered element and accumulator operands';
  }
 }else if(['length getter','element getter','callback'].includes(state.phase))return 'Array getter/callback phases require their actual selected call';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArrayCallbackTransition} */
export function requestJavaScriptArrayCallbackCall(heap,state,callee,receiver,arguments_){const call={$:'arrayCallbackSelectedCall',callee,receiver,arguments:arguments_};return {$:'arrayCallbackCall',heap,state:{...state,call},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state
 * @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArrayCallbackTransition} */
export function continueJavaScriptArrayCallbackLength(heap,state,transition){
 const {conversion:priorConversion,call:priorCall,...base}=state;
 switch(transition.$){
  case 'scalarContinuation':return {$:'arrayCallbackContinue',heap,state:{...base,phase:'length conversion',conversion:transition.state}};
  case 'scalarCall':return requestJavaScriptArrayCallbackCall(heap,{...base,phase:'length conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arrayCallbackInvalid',heap,reason:'Array LengthOfArrayLike retains its actual ToNumber result'};
   const length=readJavaScriptArrayLikeLength(transition.value.value);
   if(state.callback.$!=='object'||!state.callback.callable)return {$:'arrayCallbackError',heap,errorClass:'TypeError',reason:'Array callback must be callable'};
   return {$:'arrayCallbackContinue',heap,state:{...base,phase:'ready',length,index:testJavaScriptArrayCallbackReverse(state.operation)?length-1:0,element:{$:'undefined'}}};
  }
  case 'scalarThrow':return {$:'arrayCallbackThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arrayCallbackError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arrayCallbackInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayCallbackResourceLimit',heap,state:{...base,phase:'length primitive',element:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation
 * @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} callback
 * @param {JavaScriptSemanticValue} thisArgument @param {JavaScriptSemanticValue} exoticKey
 * @param {boolean} [initialPresent] @returns {JavaScriptArrayCallbackTransition} */
export function beginJavaScriptArrayCallback(heap,operation,receiver,callback,thisArgument,exoticKey,initialPresent=false){
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'arrayCallbackError',heap,errorClass:'TypeError',reason:'Array.prototype callback called on null or undefined'};
 const state={$:'arrayCallbackState',operation,receiver,callback,thisArgument,exoticKey,length:0,index:0,phase:'length',initialPresent,hasAccumulator:testJavaScriptArrayCallbackReduction(operation)&&initialPresent,accumulator:testJavaScriptArrayCallbackReduction(operation)&&initialPresent?thisArgument:{$:'undefined'},element:{$:'undefined'}};
 const error=readJavaScriptArrayCallbackStateError(heap,state);return error===''?{$:'arrayCallbackContinue',heap,state}:{$:'arrayCallbackInvalid',heap,reason:error};
}
/** Ordinary HasProperty refuses opaque prototype boundaries instead of
 * inferring absence from an incomplete descriptor snapshot.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key
 * @returns {{present:boolean,error:string}} */
export function readJavaScriptArrayCallbackPropertyPresence(heap,identity,key){return readJavaScriptIndexedPropertyPresence(heap,identity,key);}
/** @param {JavaScriptArrayCallbackState} state @returns {JavaScriptSemanticValue} */
export function readJavaScriptArrayCallbackEmptyResult(state){if(testJavaScriptArrayCallbackReduction(state.operation))return state.accumulator;return state.operation==='Array.some'?{$:'boolean',value:false}:state.operation==='Array.every'?{$:'boolean',value:true}:state.operation==='Array.findIndex'||state.operation==='Array.findLastIndex'?{$:'number',value:-1}:{$:'undefined'};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state
 * @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCallbackTransition} */
export function beginJavaScriptArrayCallbackElement(heap,state,value){
 if(testJavaScriptArrayCallbackReduction(state.operation)){
  if(!state.hasAccumulator)return {$:'arrayCallbackContinue',heap,state:{...state,phase:'ready',hasAccumulator:true,accumulator:value,index:state.index+(testJavaScriptArrayCallbackReverse(state.operation)?-1:1),element:{$:'undefined'}}};
  return requestJavaScriptArrayCallbackCall(heap,{...state,phase:'callback',element:value},state.callback,{$:'undefined'},[state.accumulator,value,{$:'number',value:state.index},state.receiver]);
 }
 return requestJavaScriptArrayCallbackCall(heap,{...state,phase:'callback',element:value},state.callback,state.thisArgument,[value,{$:'number',value:state.index},state.receiver]);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state @returns {JavaScriptArrayCallbackTransition} */
export function stepJavaScriptArrayCallback(heap,state){
 const error=readJavaScriptArrayCallbackStateError(heap,state);if(error!=='')return {$:'arrayCallbackInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arrayCallbackCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='length conversion')return state.conversion===undefined?{$:'arrayCallbackInvalid',heap,reason:'Array length conversion retains its actual pending state'}:continueJavaScriptArrayCallbackLength(heap,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='length primitive')return continueJavaScriptArrayCallbackLength(heap,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.element));
 if(state.receiver.$!=='object')return {$:'arrayCallbackInvalid',heap,reason:'Array callbacks retain their actual boxed Object receiver'};
 if(state.phase==='length'){
  const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,{$:'text',units:[108,101,110,103,116,104]},state.receiver);if(get.error!=='')return {$:'arrayCallbackInvalid',heap,reason:get.error};
  if(get.action==='call')return requestJavaScriptArrayCallbackCall(heap,{...state,phase:'length getter'},get.callee,get.receiver,get.arguments);
  if(get.action!=='value')return {$:'arrayCallbackInvalid',heap,reason:'Array callback length needs its actual Get completion'};
  return continueJavaScriptArrayCallbackLength(heap,{...state,element:get.value},beginJavaScriptScalarProtocol(heap,get.value,state.exoticKey,'coerce primitive number'));
 }
 if(state.phase!=='ready')return {$:'arrayCallbackInvalid',heap,reason:'Array iteration resumes only after its actual getter or callback completion'};
 if(state.index<0||state.index>=state.length){if(testJavaScriptArrayCallbackReduction(state.operation)&&!state.hasAccumulator)return {$:'arrayCallbackError',heap,errorClass:'TypeError',reason:'Reduce of empty array with no initial value'};return {$:'arrayCallbackCompleted',heap,value:readJavaScriptArrayCallbackEmptyResult(state)};}
 const name=String(state.index),units=[];for(let at=0;at<name.length;at++)units.push(name.charCodeAt(at));const key={$:'text',units};
 if(!testJavaScriptArrayCallbackVisitHoles(state.operation)){const presence=readJavaScriptArrayCallbackPropertyPresence(heap,state.receiver.identity,key);if(presence.error!=='')return {$:'arrayCallbackInvalid',heap,reason:presence.error};if(!presence.present)return {$:'arrayCallbackContinue',heap,state:{...state,index:state.index+(testJavaScriptArrayCallbackReverse(state.operation)?-1:1)}};}
 const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,key,state.receiver);if(get.error!=='')return {$:'arrayCallbackInvalid',heap,reason:get.error};
 if(get.action==='call')return requestJavaScriptArrayCallbackCall(heap,{...state,phase:'element getter'},get.callee,get.receiver,get.arguments);
 if(get.action!=='value')return {$:'arrayCallbackInvalid',heap,reason:'Array callback element needs its actual Get completion'};
 return beginJavaScriptArrayCallbackElement(heap,state,get.value);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCallbackState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayCallbackTransition} */
export function resumeJavaScriptArrayCallback(heap,state,completion){
 const error=readJavaScriptArrayCallbackStateError(heap,state);if(error!=='')return {$:'arrayCallbackInvalid',heap,reason:error};
 if(state.call===undefined)return {$:'arrayCallbackInvalid',heap,reason:'Array callback resumption needs its actual selected child call'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayCallbackInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'arrayCallbackThrow',heap,value:completion.value};
 const {call:priorCall,...base}=state;
 if(state.phase==='length getter')return continueJavaScriptArrayCallbackLength(heap,{...base,element:completion.value},beginJavaScriptScalarProtocol(heap,completion.value,state.exoticKey,'coerce primitive number'));
 if(state.phase==='length conversion')return state.conversion===undefined?{$:'arrayCallbackInvalid',heap,reason:'Array length callback retains its actual conversion state'}:continueJavaScriptArrayCallbackLength(heap,base,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.phase==='element getter')return beginJavaScriptArrayCallbackElement(heap,base,completion.value);
 if(state.phase!=='callback')return {$:'arrayCallbackInvalid',heap,reason:'Array resumption retains its actual length, element or predicate call'};
 if(testJavaScriptArrayCallbackReduction(state.operation))return {$:'arrayCallbackContinue',heap,state:{...base,phase:'ready',hasAccumulator:true,accumulator:completion.value,index:state.index+(testJavaScriptArrayCallbackReverse(state.operation)?-1:1),element:{$:'undefined'}}};
 const predicate=convertJavaScriptSemanticBoolean(completion.value),find=testJavaScriptArrayCallbackVisitHoles(state.operation);
 if(state.operation==='Array.some'&&predicate||state.operation==='Array.every'&&!predicate)return {$:'arrayCallbackCompleted',heap,value:{$:'boolean',value:predicate}};
 if(find&&predicate)return {$:'arrayCallbackCompleted',heap,value:state.operation==='Array.findIndex'||state.operation==='Array.findLastIndex'?{$:'number',value:state.index}:state.element};
 return {$:'arrayCallbackContinue',heap,state:{...base,phase:'ready',index:state.index+(testJavaScriptArrayCallbackReverse(state.operation)?-1:1),element:{$:'undefined'}}};
}
