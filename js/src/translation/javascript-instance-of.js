// InstanceofOperator and OrdinaryHasInstance use actual shared heap identities.
// Selected getters and methods execute through the ordinary Call machine.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
/** @typedef {{$:'instanceOfCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptInstanceOfCall */
/** @typedef {{$:'instanceOfState',operation:string,left:JavaScriptSemanticValue,right:JavaScriptSemanticValue,key:JavaScriptSemanticValue,phase:string,prototype:JavaScriptSemanticValue,cursor:JavaScriptSemanticValue,visited:bigint[],call?:JavaScriptInstanceOfCall,candidate?:JavaScriptSemanticValue,boundPath?:bigint[]}} JavaScriptInstanceOfState */
/** @typedef {{$:'instanceOfContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptInstanceOfState} |
 * {$:'instanceOfCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptInstanceOfState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'instanceOfCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'instanceOfThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'instanceOfError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'instanceOfInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'instanceOfResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptInstanceOfState,reason:string}} JavaScriptInstanceOfTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @returns {string} */
export function readJavaScriptInstanceOfStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(!['instanceof','ordinary has instance','prototype contains'].includes(state.operation)||!['method','method getter','method value','prototype','prototype getter','prototype check','walk'].includes(state.phase))return 'instanceof retains its actual operation and continuation phase';
 for(const value of [state.left,state.right,state.key,state.prototype,state.cursor]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.candidate!==undefined||state.boundPath!==undefined){
  if(state.operation==='prototype contains'||state.candidate===undefined||state.boundPath===undefined||state.boundPath.length===0||state.boundPath.length>4096||readJavaScriptOrdinaryValueError(heap,state.candidate)!=='')return 'bound instanceof retains its original operands and bounded actual delegated target path';
  let target=state.right;for(const identity of state.boundPath){const bound=target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)]:undefined;if(bound?.$!=='boundFunctionObject'||bound.identity!==identity)return 'bound instanceof delegation follows only actual immutable target links';target=bound.boundTarget;}
  if(!testJavaScriptSemanticSameValue(target,state.candidate))return 'bound instanceof retains its exact selected delegated target';
 }
 if(state.key.$!=='symbol')return 'instanceof retains the actual realm well-known Symbol';
 if(state.visited.length>4096||state.visited.some((id,index)=>typeof id!=='bigint'||findJavaScriptOrdinaryObject(heap.objects,id)<0||state.visited.indexOf(id)!==index))return 'instanceof retains its bounded actual prototype path';
 if(state.phase==='walk'&&(state.prototype.$!=='object'||state.cursor.$!=='object'&&state.cursor.$!=='null'))return 'instanceof traversal retains actual Object prototype identities';
 const selected=state.phase==='method getter'||state.phase==='method value'||state.phase==='prototype getter';
 if(selected!==(state.call!==undefined))return 'instanceof calls retain their selected callback only while suspended';
 if(state.call!==undefined){const call=state.call;for(const value of [call.callee,call.receiver,...call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
  const right=state.candidate??state.right;if(call.callee.$!=='object'||!call.callee.callable||!testJavaScriptSemanticSameValue(call.receiver,right))return 'instanceof retains the actual selected getter or method receiver';
  if(state.phase==='method value'){if(call.arguments.length!==1||!testJavaScriptSemanticSameValue(call.arguments[0],state.left))return 'instanceof method retains its actual left operand';}
  else if(call.arguments.length!==0){
   const owner=right.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,right.identity)]:undefined,expected=state.phase==='method getter'?state.key:{$:'text',units:[112,114,111,116,111,116,121,112,101]};
   if(owner===undefined||!['hostErrorObject','hostURLObject','hostArrayObject','hostByteBufferObject'].includes(owner.$)||!testJavaScriptSemanticSameValue(call.callee,owner.hostGet)||call.arguments.length!==2||!testJavaScriptSemanticSameValue(call.arguments[0],expected)||!testJavaScriptSemanticSameValue(call.arguments[1],right))return 'opaque instanceof Get retains its actual registered host Get callback';
  }}
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right @param {JavaScriptSemanticValue} key @returns {JavaScriptInstanceOfTransition} */
export function beginJavaScriptInstanceOf(heap,operation,left,right,key){
 const state={$:'instanceOfState',operation,left,right,key,phase:operation==='instanceof'?'method':operation==='prototype contains'?'prototype check':'prototype',prototype:{$:'undefined'},cursor:{$:'undefined'},visited:[]},error=readJavaScriptInstanceOfStateError(heap,state);
 return error===''?{$:'instanceOfContinue',heap,state}:{$:'instanceOfInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @param {boolean} value @returns {JavaScriptInstanceOfTransition} */
export function finishJavaScriptInstanceOf(heap,state,value){return {$:'instanceOfCompleted',heap,value:{$:'boolean',value}};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptInstanceOfTransition} */
export function acceptJavaScriptInstanceOfMethod(heap,state,value){
 const right=state.candidate??state.right;
 if(value.$==='null'||value.$==='undefined'){
  if(right.$!=='object'||!right.callable)return {$:'instanceOfError',heap,errorClass:'TypeError',reason:'Right-hand side of instanceof is not callable'};
  return {$:'instanceOfContinue',heap,state:{...state,phase:'prototype',call:undefined}};
 }
 if(value.$!=='object'||!value.callable)return {$:'instanceOfError',heap,errorClass:'TypeError',reason:'Symbol.hasInstance method is not callable'};
 const call={$:'instanceOfCall',callee:value,receiver:right,arguments:[state.left]};return {$:'instanceOfCall',heap,state:{...state,phase:'method value',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptInstanceOfTransition} */
export function acceptJavaScriptInstanceOfPrototype(heap,state,value){
 if(value.$!=='object')return {$:'instanceOfError',heap,errorClass:'TypeError',reason:'Function has non-object prototype in instanceof check'};
 return {$:'instanceOfContinue',heap,state:{...state,phase:'walk',prototype:value,cursor:state.left,call:undefined}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @returns {JavaScriptInstanceOfTransition} */
export function stepJavaScriptInstanceOf(heap,state){
 const error=readJavaScriptInstanceOfStateError(heap,state);if(error!=='')return {$:'instanceOfInvalid',heap,reason:error};
 const right=state.candidate??state.right;
 if(state.call!==undefined)return {$:'instanceOfCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='prototype check'){if(state.left.$!=='object')return finishJavaScriptInstanceOf(heap,state,false);if(state.right.$==='null'||state.right.$==='undefined')return {$:'instanceOfError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};if(state.right.$!=='object')return finishJavaScriptInstanceOf(heap,state,false);return {$:'instanceOfContinue',heap,state:{...state,phase:'walk',prototype:state.right,cursor:state.left}};}
 if(state.phase==='method'){
  if(right.$!=='object')return {$:'instanceOfError',heap,errorClass:'TypeError',reason:'Right-hand side of instanceof is not an object'};
  const get=planJavaScriptOrdinaryGet(heap,right.identity,state.key,right);if(get.error!=='')return {$:'instanceOfInvalid',heap,reason:get.error};
  if(get.action==='value')return acceptJavaScriptInstanceOfMethod(heap,state,get.value);
  if(get.action!=='call')return {$:'instanceOfInvalid',heap,reason:'instanceof requires the actual GetMethod value or selected getter'};
  const call={$:'instanceOfCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'instanceOfCall',heap,state:{...state,phase:'method getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
 }
 if(state.phase==='prototype'){
  if(right.$!=='object'||!right.callable)return finishJavaScriptInstanceOf(heap,state,false);
  const bound=heap.objects[findJavaScriptOrdinaryObject(heap.objects,right.identity)];if(bound?.$==='boundFunctionObject'){
   if((state.boundPath??[]).length>=4096)return {$:'instanceOfResourceLimit',heap,state,reason:'bound instanceof delegation exceeds its explicit target path budget'};
   return {$:'instanceOfContinue',heap,state:{...state,phase:'method',candidate:bound.boundTarget,boundPath:(state.boundPath??[]).concat([right.identity]),prototype:{$:'undefined'},cursor:{$:'undefined'},visited:[]}};
  }
  if(state.left.$!=='object')return finishJavaScriptInstanceOf(heap,state,false);
  const get=planJavaScriptOrdinaryGet(heap,right.identity,{$:'text',units:[112,114,111,116,111,116,121,112,101]},right);if(get.error!=='')return {$:'instanceOfInvalid',heap,reason:get.error};
  if(get.action==='value')return acceptJavaScriptInstanceOfPrototype(heap,state,get.value);
  if(get.action!=='call')return {$:'instanceOfInvalid',heap,reason:'OrdinaryHasInstance retains its actual prototype Get'};
  const call={$:'instanceOfCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'instanceOfCall',heap,state:{...state,phase:'prototype getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
 }
 if(state.phase!=='walk')return {$:'instanceOfInvalid',heap,reason:'instanceof continuation requires its selected call completion'};
 if(state.cursor.$==='null')return finishJavaScriptInstanceOf(heap,state,false);
 if(state.cursor.$!=='object')return {$:'instanceOfInvalid',heap,reason:'instanceof traversal retains its actual Object cursor'};
 if(state.visited.includes(state.cursor.identity))return {$:'instanceOfInvalid',heap,reason:'instanceof cannot fabricate a result for a cyclic exotic prototype path'};
 if(state.visited.length===4096)return {$:'instanceOfResourceLimit',heap,state,reason:'instanceof prototype traversal exceeds its explicit path budget'};
 const entry=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.cursor.identity)];if(entry===undefined)return {$:'instanceOfInvalid',heap,reason:'instanceof cursor must retain its actual heap identity'};
 // Known native Error/URL/Array/byte-buffer rows authenticate a real prototype
 // at their private boundary. No Proxy [[GetPrototypeOf]] trap is simulated.
 const prototype=entry.prototype;if(prototype.$==='null')return finishJavaScriptInstanceOf(heap,state,false);
 if(testJavaScriptSemanticSameValue(prototype,state.prototype))return finishJavaScriptInstanceOf(heap,state,true);
 return {$:'instanceOfContinue',heap,state:{...state,cursor:prototype,visited:state.visited.concat([state.cursor.identity])}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptInstanceOfState} state @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptInstanceOfTransition} */
export function resumeJavaScriptInstanceOf(heap,state,completion){
 const invalid=readJavaScriptInstanceOfStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'instanceOfInvalid',heap,reason:invalid};
 if(state.call===undefined)return {$:'instanceOfInvalid',heap,reason:'instanceof resumes only an actual selected child call'};
 if(completion.$==='throwCompletion')return {$:'instanceOfThrow',heap,value:completion.value};
 if(state.phase==='method getter')return acceptJavaScriptInstanceOfMethod(heap,state,completion.value);
 if(state.phase==='prototype getter')return acceptJavaScriptInstanceOfPrototype(heap,state,completion.value);
 if(state.phase==='method value')return finishJavaScriptInstanceOf(heap,state,convertJavaScriptSemanticBoolean(completion.value));
 return {$:'instanceOfInvalid',heap,reason:'instanceof completion must match the actual selected call phase'};
}
