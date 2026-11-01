// EnumerableOwnProperties and CreateArrayFromList share the ordinary heap.
// Key snapshots/Lists are internal control data, never fabricated Array Objects.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,readJavaScriptOrdinaryOwnKeys,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit,readJavaScriptSemanticTextString,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
/** @typedef {{$:'objectEnumeration',operation:string,input:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,keys:JavaScriptSemanticValue[],values:JavaScriptSemanticValue[],position:number,phase:string,value:JavaScriptSemanticValue,callee:JavaScriptSemanticValue,pair:JavaScriptSemanticValue,pairIndex:number,target:JavaScriptSemanticValue,outputIndex:number}} JavaScriptObjectEnumerationState */
/** @typedef {{$:'objectEnumerationContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectEnumerationState} |
 * {$:'objectEnumerationCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectEnumerationState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'objectEnumerationAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectEnumerationState,prototype:JavaScriptSemanticValue,length:number} |
 * {$:'objectEnumerationCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectEnumerationThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectEnumerationError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'objectEnumerationInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectEnumerationResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectEnumerationState,reason:string}} JavaScriptObjectEnumerationTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptObjectEnumerationOperation(operation){return ['Object.keys','Object.values','Object.entries'].includes(operation);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectEnumerationState} state @returns {string} */
export function readJavaScriptObjectEnumerationStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(!testJavaScriptObjectEnumerationOperation(state.operation)||!['keys','read','getter','pair allocation','pair write','allocate','output write'].includes(state.phase)||state.prototype.$!=='object'||state.prototype.callable||state.input.$==='null'||state.input.$==='undefined')return 'Object enumeration retains its actual operation input Array prototype and phase';
 if(state.keys.length>4096||state.values.length>4096||!Number.isSafeInteger(state.position)||state.position<0||Object.is(state.position,-0)||state.position>state.keys.length||!Number.isSafeInteger(state.outputIndex)||state.outputIndex<0||Object.is(state.outputIndex,-0)||state.outputIndex>state.values.length||!Number.isSafeInteger(state.pairIndex)||state.pairIndex<0||state.pairIndex>2||Object.is(state.pairIndex,-0))return 'Object enumeration retains bounded actual key List and write coordinates';
 for(const value of [state.input,state.prototype,state.value,state.callee,state.pair,state.target,...state.values]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 const seen=[];for(const key of state.keys){const invalid=readJavaScriptOrdinaryKeyError(heap,key);if(invalid!=='')return invalid;if(seen.some(prior=>testJavaScriptSemanticSameValue(prior,key)))return 'Object enumeration key snapshots retain unique actual property keys';seen.push(key);}
 if(state.values.length>state.position)return 'enumeration Lists retain only completed preceding own properties';
 for(const value of state.values){if(state.operation==='Object.keys'&&value.$!=='text'&&value.$!=='sourceText')return 'Object.keys result Lists contain only actual text keys';if(state.operation==='Object.entries'){const pair=value.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)]:undefined;if(pair?.$!=='arrayObject'||pair.length!==2||!testJavaScriptSemanticSameValue(pair.prototype,state.prototype))return 'Object.entries result Lists retain actual length-two entry Arrays';}}
 if((state.phase==='allocate'||state.phase==='output write')&&state.position!==state.keys.length||state.phase==='getter'&&state.operation==='Object.keys')return 'enumeration allocation follows all own keys and key-only enumeration invokes no getter';
 if(state.phase!=='keys'&&state.input.$!=='object'){
  const length=state.input.$==='text'||state.input.$==='sourceText'?readJavaScriptSemanticTextLength(state.input):0;
  if(state.keys.length!==length||state.values.length!==state.position)return 'primitive own enumeration retains exactly its actual String indices or empty key space';
  for(let at=0;at<state.keys.length;at++)if(state.keys[at]?.$!=='text'||readJavaScriptSemanticTextString(state.keys[at]??{$:'undefined'})!==String(at))return 'String own enumeration retains exact ordered UTF16 index keys';
 }
 if(state.phase!=='pair allocation'&&state.phase!=='pair write'&&state.value.$!=='undefined')return 'enumeration holds a completed value only during actual entry pair creation';
 if(state.phase==='keys'&&(state.keys.length!==0||state.values.length!==0||state.position!==0)||['getter','pair allocation','pair write'].includes(state.phase)&&state.position>=state.keys.length)return 'Object enumeration phases retain their actual selected source property';
 if(state.phase==='getter'&&(state.input.$!=='object'||state.callee.$!=='object'||!state.callee.callable)||state.phase!=='getter'&&state.callee.$!=='undefined')return 'Object enumeration getters retain their actual selected callable and receiver';
 if(['pair allocation','pair write'].includes(state.phase)&&state.operation!=='Object.entries'||state.phase!=='pair write'&&(state.pair.$!=='undefined'||state.pairIndex!==0))return 'entry pair storage belongs only to its actual pair creation phase';
 if(state.phase==='pair write'){const pair=state.pair.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.pair.identity)]:undefined;if(pair?.$!=='arrayObject'||pair.length!==2||!testJavaScriptSemanticSameValue(pair.prototype,state.prototype))return 'entry pair writes retain their actual fresh length-two Array';}
 if(state.phase==='output write'){const target=state.target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)]:undefined;if(target?.$!=='arrayObject'||target.length!==state.values.length||!testJavaScriptSemanticSameValue(target.prototype,state.prototype)||state.position!==state.keys.length)return 'Object enumeration output retains its actual completed List and result Array';}
 else if(state.target.$!=='undefined'||state.outputIndex!==0)return 'Object enumeration allocates its final Array only after collecting the complete List';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} prototype @returns {JavaScriptObjectEnumerationTransition} */
export function beginJavaScriptObjectEnumeration(heap,operation,input,prototype){
 const invalid=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,input)||readJavaScriptOrdinaryValueError(heap,prototype);if(invalid!=='')return {$:'objectEnumerationInvalid',heap,reason:invalid};
 if(input.$==='null'||input.$==='undefined')return {$:'objectEnumerationError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};
 const absent={$:'undefined'},state={$:'objectEnumeration',operation,input,prototype,keys:[],values:[],position:0,phase:'keys',value:absent,callee:absent,pair:absent,pairIndex:0,target:absent,outputIndex:0};const error=readJavaScriptObjectEnumerationStateError(heap,state);return error===''?{$:'objectEnumerationContinue',heap,state}:{$:'objectEnumerationInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectEnumerationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptObjectEnumerationTransition} */
export function acceptJavaScriptObjectEnumerationValue(heap,state,value){
 if(state.operation==='Object.entries')return {$:'objectEnumerationContinue',heap,state:{...state,phase:'pair allocation',value,callee:{$:'undefined'}}};
 return {$:'objectEnumerationContinue',heap,state:{...state,phase:'read',values:state.values.concat([value]),position:state.position+1,value:{$:'undefined'},callee:{$:'undefined'}}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectEnumerationState} state @returns {JavaScriptObjectEnumerationTransition} */
export function stepJavaScriptObjectEnumeration(heap,state){
 const error=readJavaScriptObjectEnumerationStateError(heap,state);if(error!=='')return {$:'objectEnumerationInvalid',heap,reason:error};
 if(state.phase==='keys'){
  let keys=[];if(state.input.$==='object'){const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'objectEnumerationInvalid',heap,reason:own.error};keys=own.keys;}
  else if(state.input.$==='text'||state.input.$==='sourceText'){const length=readJavaScriptSemanticTextLength(state.input);if(length>4096)return {$:'objectEnumerationResourceLimit',heap,state,reason:'String enumeration exceeds its actual own-key snapshot budget'};for(let at=0;at<length;at++){const text=String(at),units=[];for(let part=0;part<text.length;part++)units.push(text.charCodeAt(part));keys.push({$:'text',units});}}
  if(keys.length>4096)return {$:'objectEnumerationResourceLimit',heap,state,reason:'Object enumeration exceeds its actual own-key snapshot budget'};return {$:'objectEnumerationContinue',heap,state:{...state,keys,phase:'read'}};
 }
 if(state.phase==='allocate'||state.phase==='pair allocation')return {$:'objectEnumerationAllocate',heap,state,prototype:state.prototype,length:state.phase==='allocate'?state.values.length:2};
 if(state.phase==='getter')return {$:'objectEnumerationInvalid',heap,reason:'Object enumeration getter requires its actual selected child completion'};
 if(state.phase==='pair write'||state.phase==='output write'){
  if(state.phase==='pair write'&&state.pairIndex===2)return {$:'objectEnumerationContinue',heap,state:{...state,phase:'read',values:state.values.concat([state.pair]),position:state.position+1,pair:{$:'undefined'},pairIndex:0,value:{$:'undefined'}}};
  if(state.phase==='output write'&&state.outputIndex===state.values.length)return {$:'objectEnumerationCompleted',heap,value:state.target};
  const index=state.phase==='pair write'?state.pairIndex:state.outputIndex,target=state.phase==='pair write'?state.pair:state.target,value=state.phase==='pair write'?(index===0?state.keys[state.position]??{$:'undefined'}:state.value):state.values[index]??{$:'undefined'},written=createJavaScriptIndexedDataProperty(heap,target,index,value);
  if(written.$==='indexedPropertyResourceLimit')return {$:'objectEnumerationResourceLimit',heap:written.heap,state,reason:written.reason};if(written.$!=='indexedPropertyCreated')return {$:'objectEnumerationInvalid',heap:written.heap,reason:written.reason};return {$:'objectEnumerationContinue',heap:written.heap,state:state.phase==='pair write'?{...state,pairIndex:state.pairIndex+1}:{...state,outputIndex:state.outputIndex+1}};
 }
 if(state.position===state.keys.length)return {$:'objectEnumerationContinue',heap,state:{...state,phase:'allocate'}};
 const key=state.keys[state.position];if(key===undefined)return {$:'objectEnumerationInvalid',heap,reason:'Object enumeration retains its selected own key'};
 if(key.$==='symbol')return {$:'objectEnumerationContinue',heap,state:{...state,position:state.position+1}};
 if(state.input.$==='object'){
  const source=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)],property=source?.properties[findJavaScriptOrdinaryProperty(source.properties,key)];if(property===undefined||!property.descriptor.enumerable)return {$:'objectEnumerationContinue',heap,state:{...state,position:state.position+1}};
  if(state.operation==='Object.keys')return acceptJavaScriptObjectEnumerationValue(heap,state,key);
  const get=planJavaScriptOrdinaryGet(heap,state.input.identity,key,state.input);if(get.error!=='')return {$:'objectEnumerationInvalid',heap,reason:get.error};if(get.action==='call')return {$:'objectEnumerationCall',heap,state:{...state,phase:'getter',callee:get.callee},callee:get.callee,receiver:get.receiver,arguments:get.arguments};if(get.action!=='value')return {$:'objectEnumerationInvalid',heap,reason:'Object enumeration Get retains its actual value or selected getter'};return acceptJavaScriptObjectEnumerationValue(heap,state,get.value);
 }
 if(state.input.$!=='text'&&state.input.$!=='sourceText')return {$:'objectEnumerationInvalid',heap,reason:'primitive enumeration cannot invent own properties'};
 return acceptJavaScriptObjectEnumerationValue(heap,state,state.operation==='Object.keys'?key:{$:'text',units:[readJavaScriptSemanticTextUnit(state.input,state.position)]});
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectEnumerationState} state @param {bigint} identity @returns {JavaScriptObjectEnumerationTransition} */
export function resumeJavaScriptObjectEnumerationAllocation(heap,state,identity){
 const error=readJavaScriptObjectEnumerationStateError(heap,state);if(error!==''||state.phase!=='allocate'&&state.phase!=='pair allocation')return {$:'objectEnumerationInvalid',heap,reason:error||'Object enumeration allocation retains its actual pending phase'};
 const allocated=allocateJavaScriptArray(heap,identity,state.prototype,state.phase==='allocate'?state.values.length:2);if(allocated.$==='arrayResourceLimit')return {$:'objectEnumerationResourceLimit',heap:allocated.heap,state,reason:allocated.reason};if(allocated.$!=='arrayNormal')return {$:'objectEnumerationInvalid',heap:allocated.heap,reason:allocated.$==='arrayInvalid'||allocated.$==='arrayError'?allocated.reason:'Object enumeration allocation requires its actual fresh Array'};
 return {$:'objectEnumerationContinue',heap:allocated.heap,state:state.phase==='allocate'?{...state,phase:'output write',target:allocated.value}:{...state,phase:'pair write',pair:allocated.value}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectEnumerationState} state @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptObjectEnumerationTransition} */
export function resumeJavaScriptObjectEnumerationGetter(heap,state,completion){
 const error=readJavaScriptObjectEnumerationStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.phase!=='getter')return {$:'objectEnumerationInvalid',heap,reason:error||'Object enumeration completion retains its actual selected getter'};return completion.$==='throwCompletion'?{$:'objectEnumerationThrow',heap,value:completion.value}:acceptJavaScriptObjectEnumerationValue(heap,state,completion.value);
}
