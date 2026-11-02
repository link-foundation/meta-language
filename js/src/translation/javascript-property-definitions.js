// ObjectDefineProperties first converts every selected descriptor, then defines
// them in order. Internal Lists retain descriptors, not replacement host data.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,readJavaScriptOrdinaryOwnKeys,planJavaScriptOrdinaryGet,insertJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {readJavaScriptPropertyDescriptorError} from './javascript-property-descriptors.js';
import {readJavaScriptSemanticTextLength,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {beginJavaScriptDescriptorConversion,stepJavaScriptDescriptorConversion,resumeJavaScriptDescriptorConversion,readJavaScriptDescriptorConversionStateError} from './javascript-descriptor-conversion.js';
import {completeJavaScriptPropertyDefinitionWithSymbol,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition,readJavaScriptPropertyDefinitionStateError} from './javascript-property-definition.js';
import {readJavaScriptReflectedOwnProperty,constructJavaScriptReflectionText,testJavaScriptReflectionOrdinaryKind,testJavaScriptReflectionPropertyBudget} from './javascript-property-reflection.js';
/** @typedef {{$:'propertyDefinitionsCallRecord',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptPropertyDefinitionsCall */
/** @typedef {{$:'propertyDefinitions',operation:string,target:JavaScriptSemanticValue,input:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,phase:string,keys:JavaScriptSemanticValue[],position:number,descriptors:JavaScriptOrdinaryProperty[],definitionIndex:number,conversion?:JavaScriptDescriptorConversionState,definition?:JavaScriptPropertyDefinitionState,call?:JavaScriptPropertyDefinitionsCall}} JavaScriptPropertyDefinitionsState */
/** @typedef {{$:'propertyDefinitionsContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyDefinitionsState} |
 * {$:'propertyDefinitionsCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyDefinitionsState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'propertyDefinitionsAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyDefinitionsState,prototype:JavaScriptSemanticValue} |
 * {$:'propertyDefinitionsCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'propertyDefinitionsThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'propertyDefinitionsError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'propertyDefinitionsInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'propertyDefinitionsResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyDefinitionsState,reason:string}} JavaScriptPropertyDefinitionsTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptPropertyDefinitionsOperation(operation){return operation==='Object.defineProperties'||operation==='Object.create';}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @returns {string} */
export function readJavaScriptPropertyDefinitionsStateError(heap,state){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return invalid;
 if(!testJavaScriptPropertyDefinitionsOperation(state.operation)||!['allocate','keys','read','getter','convert','define','definition'].includes(state.phase)||state.keys.length>4096||state.descriptors.length>state.position||!Number.isSafeInteger(state.position)||Object.is(state.position,-0)||state.position<0||state.position>state.keys.length||!Number.isSafeInteger(state.definitionIndex)||Object.is(state.definitionIndex,-0)||state.definitionIndex<0||state.definitionIndex>state.descriptors.length)return 'bulk definitions retain their actual operation phase and bounded ordered descriptor positions';
 for(const value of [state.target,state.input,state.prototype,state.exoticKey]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.exoticKey.$!=='symbol'||state.phase==='allocate'&&(state.operation!=='Object.create'||state.target.$!=='undefined'||state.prototype.$!=='object'&&state.prototype.$!=='null')||state.phase!=='allocate'&&state.target.$!=='object')return 'bulk definitions retain actual target allocation and primitive conversion authority';
 const seen=[];for(const key of state.keys){const error=readJavaScriptOrdinaryKeyError(heap,key);if(error!=='')return error;if(seen.some(prior=>testJavaScriptSemanticSameValue(prior,key)))return 'bulk definitions retain actual distinct own-key snapshots';seen.push(key);}
 for(const property of state.descriptors){const error=readJavaScriptOrdinaryKeyError(heap,property.key)||readJavaScriptPropertyDescriptorError(property.descriptor);if(error!=='')return error;for(const value of [property.descriptor.value,property.descriptor.getter,property.descriptor.setter]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}}
 if((state.phase==='convert')!==(state.conversion!==undefined)||(state.phase==='definition')!==(state.definition!==undefined))return 'bulk definitions retain only their actual nested descriptor or definition continuation';
 if(state.conversion!==undefined){const error=readJavaScriptDescriptorConversionStateError(heap,state.conversion);if(error!=='')return error;if(state.conversion.pending!==(state.call!==undefined))return 'bulk descriptor conversion retains its selected Get callback';}
 if(state.definition!==undefined){const error=readJavaScriptPropertyDefinitionStateError(heap,state.definition);if(error!=='')return error;if(!testJavaScriptSemanticSameValue(state.definition.target,state.target))return 'bulk definition retains its actual selected target';const selected=state.descriptors[state.definitionIndex];if(selected===undefined||state.definition.$==='propertyDefinitionKey'||!testJavaScriptSemanticSameValue(state.definition.key,selected.key))return 'bulk definition retains its selected preconverted property key';if(state.definition.$==='propertyDefinitionArray'&&!testJavaScriptSemanticSameValue(state.definition.assignment.exoticKey,state.exoticKey))return 'bulk Array definition retains its actual primitive conversion Symbol';}
 if(state.phase==='getter'&&state.call===undefined||state.call!==undefined&&!['getter','convert','definition'].includes(state.phase))return 'bulk definitions retain only actual selected child calls';
 if(state.call!==undefined){for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}if(state.call.callee.$!=='object'||!state.call.callee.callable)return 'bulk definition callbacks retain actual callability';}
 if((state.phase==='define'||state.phase==='definition')&&state.position!==state.keys.length)return 'all descriptors convert before the first bulk definition';return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptPropertyDefinitionsTransition} */
export function beginJavaScriptPropertyDefinitions(heap,operation,target,input,exoticKey){
 const error=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,target)||readJavaScriptOrdinaryValueError(heap,input);if(error!=='')return {$:'propertyDefinitionsInvalid',heap,reason:error};
 if(operation==='Object.create'?target.$!=='object'&&target.$!=='null':target.$!=='object')return {$:'propertyDefinitionsError',heap,errorClass:'TypeError',reason:operation==='Object.create'?'Object prototype must be an Object or null':'Object.defineProperties requires an Object target'};
 const state={$:'propertyDefinitions',operation,target:operation==='Object.create'?{$:'undefined'}:target,input,prototype:operation==='Object.create'?target:{$:'undefined'},exoticKey,phase:operation==='Object.create'?'allocate':'keys',keys:[],position:0,descriptors:[],definitionIndex:0},invalid=readJavaScriptPropertyDefinitionsStateError(heap,state);return invalid===''?{$:'propertyDefinitionsContinue',heap,state}:{$:'propertyDefinitionsInvalid',heap,reason:invalid};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @param {JavaScriptDescriptorConversionTransition} transition @returns {JavaScriptPropertyDefinitionsTransition} */
export function acceptJavaScriptBulkDescriptorConversion(heap,state,transition){
 switch(transition.$){
  case 'descriptorContinuation':return {$:'propertyDefinitionsContinue',heap,state:{...state,phase:'convert',conversion:transition.state,call:undefined}};
  case 'descriptorCall':{const call={$:'propertyDefinitionsCallRecord',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'propertyDefinitionsCall',heap,state:{...state,phase:'convert',conversion:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'descriptorCompleted':{const key=state.keys[state.position];if(key===undefined)return {$:'propertyDefinitionsInvalid',heap,reason:'converted descriptor retains its selected actual own key'};return {$:'propertyDefinitionsContinue',heap,state:{...state,phase:'read',position:state.position+1,descriptors:state.descriptors.concat([{$:'ordinaryProperty',key,descriptor:transition.descriptor}]),conversion:undefined,call:undefined}};}
  case 'descriptorThrow':return {$:'propertyDefinitionsThrow',heap,value:transition.value};
  case 'descriptorIntrinsicError':return {$:'propertyDefinitionsError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'descriptorInvalid':return {$:'propertyDefinitionsInvalid',heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @param {JavaScriptPropertyDefinitionTransition} transition @returns {JavaScriptPropertyDefinitionsTransition} */
export function acceptJavaScriptBulkPropertyDefinition(heap,state,transition){
 const current=transition.heap??heap;
 switch(transition.$){
  case 'definitionContinuation':return {$:'propertyDefinitionsContinue',heap:current,state:{...state,phase:'definition',definition:transition.state,call:undefined}};
  case 'definitionCall':{const call={$:'propertyDefinitionsCallRecord',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'propertyDefinitionsCall',heap:current,state:{...state,phase:'definition',definition:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'definitionCompleted':return {$:'propertyDefinitionsContinue',heap:current,state:{...state,phase:'define',definitionIndex:state.definitionIndex+1,definition:undefined,call:undefined}};
  case 'definitionThrow':return {$:'propertyDefinitionsThrow',heap:current,value:transition.value};
  case 'definitionIntrinsicError':return {$:'propertyDefinitionsError',heap:current,errorClass:transition.errorClass,reason:transition.reason};
  case 'definitionInvalid':return {$:'propertyDefinitionsInvalid',heap:current,reason:transition.reason};
  case 'definitionResourceLimit':return {$:'propertyDefinitionsResourceLimit',heap:current,state:{...state,phase:'definition',definition:transition.state,call:undefined},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @returns {JavaScriptPropertyDefinitionsTransition} */
export function stepJavaScriptPropertyDefinitions(heap,state){
 const invalid=readJavaScriptPropertyDefinitionsStateError(heap,state);if(invalid!=='')return {$:'propertyDefinitionsInvalid',heap,reason:invalid};
 if(state.call!==undefined)return {$:'propertyDefinitionsCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='allocate')return {$:'propertyDefinitionsAllocate',heap,state,prototype:state.prototype};
 if(state.phase==='convert'&&state.conversion!==undefined)return acceptJavaScriptBulkDescriptorConversion(heap,state,stepJavaScriptDescriptorConversion(heap,state.conversion));
 if(state.phase==='definition'&&state.definition!==undefined)return acceptJavaScriptBulkPropertyDefinition(heap,state,stepJavaScriptPropertyDefinition(heap,state.definition));
 if(state.phase==='keys'){
  if(state.operation==='Object.create'&&state.input.$==='undefined')return {$:'propertyDefinitionsCompleted',heap,value:state.target};
  if(state.input.$==='null'||state.input.$==='undefined')return {$:'propertyDefinitionsError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};
  let keys=[];if(state.input.$==='object'){const owner=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)];if(owner===undefined||!testJavaScriptReflectionOrdinaryKind(owner.$))return {$:'propertyDefinitionsInvalid',heap,reason:'opaque bulk descriptors require actual own-key and descriptor protocols'};const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'propertyDefinitionsInvalid',heap,reason:own.error};keys=own.keys;}
  else if(state.input.$==='text'||state.input.$==='sourceText'){const length=readJavaScriptSemanticTextLength(state.input);if(length>=4096)return {$:'propertyDefinitionsResourceLimit',heap,state,reason:'String bulk descriptor keys exceed their actual snapshot budget'};for(let at=0;at<length;at++)keys.push(constructJavaScriptReflectionText(String(at)));keys.push(constructJavaScriptReflectionText('length'));}
  if(keys.length>4096)return {$:'propertyDefinitionsResourceLimit',heap,state,reason:'bulk descriptor keys exceed their actual snapshot budget'};return {$:'propertyDefinitionsContinue',heap,state:{...state,phase:'read',keys}};
 }
 if(state.phase==='read'){
  if(state.position===state.keys.length)return {$:'propertyDefinitionsContinue',heap,state:{...state,phase:'define'}};
  const key=state.keys[state.position];if(key===undefined)return {$:'propertyDefinitionsInvalid',heap,reason:'bulk descriptor Get retains its selected actual key'};const property=readJavaScriptReflectedOwnProperty(heap,state.input,key);if(property===undefined||!property.descriptor.enumerable)return {$:'propertyDefinitionsContinue',heap,state:{...state,position:state.position+1}};
  if(state.input.$!=='object')return acceptJavaScriptBulkDescriptorConversion(heap,state,beginJavaScriptDescriptorConversion(heap,property.descriptor.value));
  const get=planJavaScriptOrdinaryGet(heap,state.input.identity,key,state.input);if(get.error!=='')return {$:'propertyDefinitionsInvalid',heap,reason:get.error};if(get.action==='value')return acceptJavaScriptBulkDescriptorConversion(heap,state,beginJavaScriptDescriptorConversion(heap,get.value));
  if(get.action!=='call')return {$:'propertyDefinitionsInvalid',heap,reason:'bulk descriptor Get retains its actual selected getter or value'};const call={$:'propertyDefinitionsCallRecord',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'propertyDefinitionsCall',heap,state:{...state,phase:'getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
 }
 if(state.phase==='define'){
  if(state.definitionIndex===state.descriptors.length)return {$:'propertyDefinitionsCompleted',heap,value:state.target};
  const property=state.descriptors[state.definitionIndex];if(property===undefined)return {$:'propertyDefinitionsInvalid',heap,reason:'bulk definition retains its selected preconverted descriptor'};
  if(!testJavaScriptReflectionPropertyBudget(heap,state.target,property.key))return {$:'propertyDefinitionsResourceLimit',heap,state,reason:'bulk definitions exceed the actual heap property budget'};
  return acceptJavaScriptBulkPropertyDefinition(heap,state,completeJavaScriptPropertyDefinitionWithSymbol(heap,state.target,property.key,property.descriptor,state.exoticKey));
 }
 return {$:'propertyDefinitionsInvalid',heap,reason:'bulk definitions retain their actual selected completion phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPropertyDefinitionsTransition} */
export function resumeJavaScriptPropertyDefinitionsCall(heap,state,completion){
 const error=readJavaScriptPropertyDefinitionsStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.call===undefined)return {$:'propertyDefinitionsInvalid',heap,reason:error||'bulk definitions resume only their actual selected child'};
 if(completion.$==='throwCompletion')return {$:'propertyDefinitionsThrow',heap,value:completion.value};
 if(state.phase==='getter')return acceptJavaScriptBulkDescriptorConversion(heap,state,beginJavaScriptDescriptorConversion(heap,completion.value));
 if(state.phase==='convert'&&state.conversion!==undefined)return acceptJavaScriptBulkDescriptorConversion(heap,state,resumeJavaScriptDescriptorConversion(heap,state.conversion,completion));
 if(state.phase==='definition'&&state.definition!==undefined)return acceptJavaScriptBulkPropertyDefinition(heap,state,resumeJavaScriptPropertyDefinition(heap,state.definition,completion));
 return {$:'propertyDefinitionsInvalid',heap,reason:'bulk definitions retain their selected callback phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionsState} state @param {bigint} identity @returns {JavaScriptPropertyDefinitionsTransition} */
export function resumeJavaScriptPropertyDefinitionsAllocation(heap,state,identity){
 const error=readJavaScriptPropertyDefinitionsStateError(heap,state);if(error!==''||state.phase!=='allocate')return {$:'propertyDefinitionsInvalid',heap,reason:error||'Object.create allocation retains its actual pending source operation'};
 if(heap.objects.length>=4096)return {$:'propertyDefinitionsResourceLimit',heap,state,reason:'Object.create exceeds the actual Object identity budget'};
 const allocation=insertJavaScriptOrdinaryObject(heap,identity,state.prototype);return allocation.error!==''?{$:'propertyDefinitionsInvalid',heap:allocation.heap,reason:allocation.error}:{$:'propertyDefinitionsContinue',heap:allocation.heap,state:{...state,phase:'keys',target:{$:'object',identity,callable:false}}};
}
