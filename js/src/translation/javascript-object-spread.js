// CopyDataProperties for actual object-literal targets. Getters execute through
// the common call machine; snapshots never substitute original host bodies.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,readJavaScriptOrdinaryOwnKeys,planJavaScriptOrdinaryGet,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextString,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
/** @typedef {{$:'objectSpreadState',target:JavaScriptSemanticValue,input:JavaScriptSemanticValue,keys:JavaScriptSemanticValue[],position:number,phase:string,value:JavaScriptSemanticValue,callee:JavaScriptSemanticValue}} JavaScriptObjectSpreadState */
/** @typedef {{$:'objectSpreadContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState} |
 * {$:'objectSpreadCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'objectSpreadCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectSpreadThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectSpreadError',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectSpreadInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectSpreadResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState,reason:string}} JavaScriptObjectSpreadTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state @returns {string} */
export function readJavaScriptObjectSpreadStateError(heap,state){
 const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
 for(const value of [state.target,state.input,state.value,state.callee]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.target.$!=='object')return 'object spread retains its actual ordinary literal target';
 const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];if(target===undefined||target.$!=='ordinaryObject'||target.callable)return 'object spread target requires an inspectable ordinary literal object';
 if(!['keys','read','getter','write'].includes(state.phase)||!Number.isSafeInteger(state.position)||state.position<0||Object.is(state.position,-0)||state.position>state.keys.length||state.keys.length>4096)return 'object spread retains its bounded key snapshot and exact cursor';
 if(state.phase==='keys'&&(state.keys.length!==0||state.position!==0)||['getter','write'].includes(state.phase)&&state.position>=state.keys.length)return 'object spread phases retain their actual selected property';
 if(state.phase!=='write'&&state.value.$!=='undefined'||state.phase!=='getter'&&state.callee.$!=='undefined')return 'object spread retains a completed value or getter only in its actual phase';
 if(state.phase==='getter'&&(state.input.$!=='object'||state.callee.$!=='object'||!state.callee.callable))return 'object spread getter suspension retains its actual callable and receiver';
 if(state.phase!=='keys'&&state.input.$!=='object'){
  const length=state.input.$==='text'||state.input.$==='sourceText'?readJavaScriptSemanticTextLength(state.input):0;
  if(length!==state.keys.length)return 'primitive spread snapshots retain all actual String indices and no invented keys';
  for(let at=0;at<state.keys.length;at++){const key=state.keys[at];if(key===undefined||key.$!=='text'||readJavaScriptSemanticTextString(key)!==String(at))return 'String spread snapshots retain their exact ordered UTF16 indices';}
 }
 const names=[];for(const key of state.keys){const error=readJavaScriptOrdinaryKeyError(heap,key);if(error!=='')return error;names.push(key.$==='symbol'?'symbol:'+String(key.identity):'text:'+readJavaScriptSemanticTextString(key));}
 const ordered=names.toSorted();for(let at=1;at<ordered.length;at++)if(ordered[at]===ordered[at-1])return 'object spread own-key snapshots cannot duplicate a semantic key';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} input @returns {JavaScriptObjectSpreadTransition} */
export function beginJavaScriptObjectSpread(heap,target,input){
 const state={$:'objectSpreadState',target,input,keys:[],position:0,phase:'keys',value:{$:'undefined'},callee:{$:'undefined'}};
 const error=readJavaScriptObjectSpreadStateError(heap,state);return error===''?{$:'objectSpreadContinue',heap,state}:{$:'objectSpreadInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state
 * @returns {JavaScriptObjectSpreadTransition} */
export function stepJavaScriptObjectSpread(heap,state){
 const error=readJavaScriptObjectSpreadStateError(heap,state);if(error!=='')return {$:'objectSpreadInvalid',heap,reason:error};
 if(state.phase==='keys'){
  let keys=[];
  if(state.input.$==='object'){
   const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'objectSpreadInvalid',heap,reason:own.error};keys=own.keys;
  }else if(state.input.$==='text'||state.input.$==='sourceText'){
   const length=readJavaScriptSemanticTextLength(state.input);if(length>4096)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread String keys exceed their explicit snapshot budget'};
   for(let at=0;at<length;at++){const key=String(at),units=[];for(let unit=0;unit<key.length;unit++)units.push(key.charCodeAt(unit));keys.push({$:'text',units});}
  }
  if(keys.length>4096)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread own keys exceed their explicit snapshot budget'};
  return {$:'objectSpreadContinue',heap,state:{...state,keys,phase:'read'}};
 }
 if(state.phase==='getter')return {$:'objectSpreadInvalid',heap,reason:'an object spread getter needs its actual child completion'};
 if(state.position===state.keys.length)return {$:'objectSpreadCompleted',heap,value:state.target};
 const key=state.keys[state.position];if(key===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread requires its actual selected key'};
 if(state.phase==='write'){
  if(state.target.$!=='object')return {$:'objectSpreadInvalid',heap,reason:'object spread retains its actual ordinary target'};
  const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];if(target===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread target cannot disappear from its global context'};
  if(findJavaScriptOrdinaryProperty(target.properties,key)<0){if(!target.extensible)return {$:'objectSpreadError',heap,reason:'object spread cannot create its selected own data property'};let count=0;for(const object of heap.objects)count+=object.properties.length;if(target.properties.length>=4096||count>=16384)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread properties exceed their explicit ordered storage budget'};}
  const defined=defineJavaScriptOrdinaryProperty(heap,state.target.identity,key,createJavaScriptOrdinarySetDescriptor(state.value,true));
  if(defined.error!=='')return {$:'objectSpreadInvalid',heap:defined.heap,reason:defined.error};
  if(!defined.success)return {$:'objectSpreadError',heap:defined.heap,reason:'object spread cannot create its selected own data property'};
  return {$:'objectSpreadContinue',heap:defined.heap,state:{...state,position:state.position+1,phase:'read',value:{$:'undefined'}}};
 }
 if(state.input.$==='object'){
  const input=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)];if(input===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread source cannot disappear from its global context'};
  if(!['ordinaryObject','closureObject','mapObject','setObject','collectionIteratorObject','arrayObject'].includes(input.$))return {$:'objectSpreadInvalid',heap,reason:'opaque object spread requires its actual own-key and descriptor host protocols'};
  const property=input.properties[findJavaScriptOrdinaryProperty(input.properties,key)];
  // A preceding getter can delete or hide this key. New keys are not in the
  // original snapshot; a recreated snapshotted key uses its latest descriptor.
  if(property===undefined||!property.descriptor.enumerable)return {$:'objectSpreadContinue',heap,state:{...state,position:state.position+1}};
  const get=planJavaScriptOrdinaryGet(heap,state.input.identity,key,state.input);if(get.error!=='')return {$:'objectSpreadInvalid',heap,reason:get.error};
  if(get.action==='call')return {$:'objectSpreadCall',heap,state:{...state,phase:'getter',callee:get.callee},callee:get.callee,receiver:get.receiver,arguments:get.arguments};
  if(get.action!=='value')return {$:'objectSpreadInvalid',heap,reason:'object spread Get requires its actual value or accessor continuation'};
  return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:get.value}};
 }
 if(state.input.$==='text'||state.input.$==='sourceText'){
  return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:{$:'text',units:[readJavaScriptSemanticTextUnit(state.input,state.position)]}}};
 }
 return {$:'objectSpreadInvalid',heap,reason:'a primitive without own keys cannot retain selected spread properties'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptObjectSpreadTransition} */
export function resumeJavaScriptObjectSpread(heap,state,completion){
 const error=readJavaScriptObjectSpreadStateError(heap,state);if(error!=='')return {$:'objectSpreadInvalid',heap,reason:error};
 if(state.phase!=='getter')return {$:'objectSpreadInvalid',heap,reason:'only a suspended object spread getter accepts a child completion'};
 const valueError=readJavaScriptOrdinaryValueError(heap,completion.value);if(valueError!=='')return {$:'objectSpreadInvalid',heap,reason:valueError};
 if(completion.$==='throwCompletion')return {$:'objectSpreadThrow',heap,value:completion.value};
 return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:completion.value,callee:{$:'undefined'}}};
}
