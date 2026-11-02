// Ordinary own-property observations and resumable integrity-level changes.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,readJavaScriptOrdinaryOwnKeys,preventJavaScriptOrdinaryExtensions,defineJavaScriptOrdinaryProperty,setJavaScriptOrdinaryPrototype} from './javascript-ordinary-heap.js';
import {readJavaScriptArrayIndex,testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
import {readJavaScriptSemanticTextLength,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {createJavaScriptPropertyDescriptor,readJavaScriptPropertyDescriptorKind} from './javascript-property-descriptors.js';
/** @typedef {{$:'objectIntegrity',operation:string,input:JavaScriptSemanticValue,phase:string,keys:JavaScriptSemanticValue[],position:number}} JavaScriptObjectIntegrityState */
/** @typedef {{$:'objectIntegrityContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectIntegrityState} |
 * {$:'objectPrototypeMetadata',heap:JavaScriptOrdinaryHeap,input:JavaScriptSemanticValue} |
 * {$:'objectIntegrityCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectIntegrityError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'objectIntegrityInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectIntegrityResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectIntegrityState,reason:string}} JavaScriptObjectIntegrityTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptObjectObservationOperation(operation){return ['Object.prototype.hasOwnProperty','Object.prototype.propertyIsEnumerable','Object.hasOwn','Object.is','Object.isExtensible','Object.isSealed','Object.isFrozen','Object.preventExtensions','Object.seal','Object.freeze','Object.getPrototypeOf','Object.setPrototypeOf'].includes(operation);}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptObjectObservationFunction(identity,realm,operation){
 if(!testJavaScriptObjectObservationOperation(operation))return undefined;
 if(operation==='Object.prototype.hasOwnProperty'||operation==='Object.prototype.propertyIsEnumerable')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'propertyKey',destination:1,input:1,realm},{$:'primitive',destination:2,operation:'require nonnull Object input',arguments:[0]},{$:'objectObservation',destination:3,operation,input:0,other:1},{$:'return',source:3}]};
 /** @type {JavaScriptCallInstruction[]} */const instructions=[{$:'argument',destination:1,index:0}];
 if(operation==='Object.hasOwn')instructions.push({$:'primitive',destination:1,operation:'require nonnull Object input',arguments:[1]});
 instructions.push({$:'argument',destination:2,index:1});if(operation==='Object.hasOwn')instructions.push({$:'propertyKey',destination:2,input:2,realm});
 instructions.push({$:'objectObservation',destination:3,operation,input:1,other:2},{$:'return',source:3});return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions};
}
/** @param {string} kind @returns {boolean} */
export function testJavaScriptOrdinaryIntegrityKind(kind){return ['ordinaryObject','closureObject','arrayObject','mapObject','setObject','collectionIteratorObject'].includes(kind);}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} other @returns {JavaScriptObjectIntegrityTransition} */
export function beginJavaScriptObjectObservation(heap,operation,input,other){
 const error=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,input)||readJavaScriptOrdinaryValueError(heap,other);if(error!==''||!testJavaScriptObjectObservationOperation(operation))return {$:'objectIntegrityInvalid',heap,reason:error||'Object observations require their actual registered operation'};
 if(operation==='Object.is')return {$:'objectIntegrityCompleted',heap,value:{$:'boolean',value:testJavaScriptSemanticSameValue(input,other)}};
 if(operation==='Object.getPrototypeOf'||operation==='Object.setPrototypeOf'){
  if(input.$==='null'||input.$==='undefined')return {$:'objectIntegrityError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};
  if(operation==='Object.setPrototypeOf'&&other.$!=='null'&&other.$!=='object')return {$:'objectIntegrityError',heap,errorClass:'TypeError',reason:'Object prototype must be an Object or null'};
  if(input.$!=='object')return operation==='Object.getPrototypeOf'?{$:'objectPrototypeMetadata',heap,input}:{$:'objectIntegrityCompleted',heap,value:input};
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)];if(object===undefined||!testJavaScriptOrdinaryIntegrityKind(object.$))return {$:'objectIntegrityInvalid',heap,reason:'opaque prototype operations require their actual internal protocol'};
  if(operation==='Object.getPrototypeOf')return {$:'objectIntegrityCompleted',heap,value:object.prototype};
  const changed=setJavaScriptOrdinaryPrototype(heap,input.identity,other);return changed.error!==''?{$:'objectIntegrityInvalid',heap:changed.heap,reason:changed.error}:changed.success?{$:'objectIntegrityCompleted',heap:changed.heap,value:input}:{$:'objectIntegrityError',heap:changed.heap,errorClass:'TypeError',reason:'Cannot set the selected Object prototype'};
 }
 if(operation==='Object.hasOwn'||operation==='Object.prototype.hasOwnProperty'||operation==='Object.prototype.propertyIsEnumerable'){
  if(input.$==='null'||input.$==='undefined')return {$:'objectIntegrityError',heap,errorClass:'TypeError',reason:'Cannot convert undefined or null to object'};
  const keyError=readJavaScriptOrdinaryKeyError(heap,other);if(keyError!=='')return {$:'objectIntegrityInvalid',heap,reason:keyError};let present=false;
  if(input.$==='object'){const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)];if(object===undefined||!testJavaScriptOrdinaryIntegrityKind(object.$))return {$:'objectIntegrityInvalid',heap,reason:'opaque own-property observation requires its actual descriptor protocol'};const property=object.properties[findJavaScriptOrdinaryProperty(object.properties,other)];present=property!==undefined&&(operation!=='Object.prototype.propertyIsEnumerable'||property.descriptor.enumerable);}
  else if(input.$==='text'||input.$==='sourceText'){const index=readJavaScriptArrayIndex(other);present=operation!=='Object.prototype.propertyIsEnumerable'&&testJavaScriptArrayLengthKey(other)||index>=0&&index<readJavaScriptSemanticTextLength(input);}
  return {$:'objectIntegrityCompleted',heap,value:{$:'boolean',value:present}};
 }
 if(input.$!=='object')return {$:'objectIntegrityCompleted',heap,value:operation==='Object.isExtensible'?{$:'boolean',value:false}:operation==='Object.isSealed'||operation==='Object.isFrozen'?{$:'boolean',value:true}:input};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)];if(object===undefined||!testJavaScriptOrdinaryIntegrityKind(object.$))return {$:'objectIntegrityInvalid',heap,reason:'opaque integrity changes require their actual own-key descriptor and extensibility protocols'};
 if(operation==='Object.isExtensible')return {$:'objectIntegrityCompleted',heap,value:{$:'boolean',value:object.extensible}};
 if(operation==='Object.isSealed'||operation==='Object.isFrozen'){
  const fixed=!object.extensible&&object.properties.every(property=>!property.descriptor.configurable&&(operation==='Object.isSealed'||readJavaScriptPropertyDescriptorKind(property.descriptor)!=='data'||!property.descriptor.writable));return {$:'objectIntegrityCompleted',heap,value:{$:'boolean',value:fixed}};
 }
 const state={$:'objectIntegrity',operation,input,phase:'prevent',keys:[],position:0};return {$:'objectIntegrityContinue',heap,state};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectIntegrityState} state @returns {string} */
export function readJavaScriptObjectIntegrityStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,state.input);if(error!=='')return error;
 const object=state.input.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)]:undefined;
 if(!['Object.preventExtensions','Object.seal','Object.freeze'].includes(state.operation)||object===undefined||!testJavaScriptOrdinaryIntegrityKind(object.$)||!['prevent','keys','define'].includes(state.phase)||!Number.isSafeInteger(state.position)||Object.is(state.position,-0)||state.position<0||state.position>state.keys.length||state.keys.length>4096)return 'integrity changes retain their actual ordinary Object operation phase and bounded key position';
 if(state.phase!=='define'&&(state.keys.length!==0||state.position!==0)||state.phase!=='prevent'&&object.extensible)return 'integrity own-key processing follows actual PreventExtensions';
 for(const key of state.keys){const invalid=readJavaScriptOrdinaryKeyError(heap,key);if(invalid!=='')return invalid;}return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectIntegrityState} state @returns {JavaScriptObjectIntegrityTransition} */
export function stepJavaScriptObjectIntegrity(heap,state){
 const error=readJavaScriptObjectIntegrityStateError(heap,state);if(error!=='')return {$:'objectIntegrityInvalid',heap,reason:error};
 if(state.phase==='prevent'){
  if(state.input.$!=='object')return {$:'objectIntegrityInvalid',heap,reason:'integrity changes retain their actual Object identity'};const result=preventJavaScriptOrdinaryExtensions(heap,state.input.identity);if(result.error!=='')return {$:'objectIntegrityInvalid',heap:result.heap,reason:result.error};if(!result.success)return {$:'objectIntegrityError',heap:result.heap,errorClass:'TypeError',reason:'Cannot prevent extensions on the selected Object'};
  return state.operation==='Object.preventExtensions'?{$:'objectIntegrityCompleted',heap:result.heap,value:state.input}:{$:'objectIntegrityContinue',heap:result.heap,state:{...state,phase:'keys'}};
 }
 if(state.input.$!=='object')return {$:'objectIntegrityInvalid',heap,reason:'integrity changes retain their actual Object identity'};
 if(state.phase==='keys'){const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'objectIntegrityInvalid',heap,reason:own.error};if(own.keys.length>4096)return {$:'objectIntegrityResourceLimit',heap,state,reason:'integrity keys exceed their actual snapshot budget'};return {$:'objectIntegrityContinue',heap,state:{...state,phase:'define',keys:own.keys}};}
 if(state.position===state.keys.length)return {$:'objectIntegrityCompleted',heap,value:state.input};
 const key=state.keys[state.position];if(key===undefined)return {$:'objectIntegrityInvalid',heap,reason:'integrity changes retain their selected actual own key'};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)],property=object?.properties[findJavaScriptOrdinaryProperty(object.properties,key)];if(property===undefined)return {$:'objectIntegrityContinue',heap,state:{...state,position:state.position+1}};
 const descriptor=createJavaScriptPropertyDescriptor('data',false,false);descriptor.enumerablePresent=false;descriptor.valuePresent=false;descriptor.writablePresent=state.operation==='Object.freeze'&&readJavaScriptPropertyDescriptorKind(property.descriptor)==='data';
 const defined=defineJavaScriptOrdinaryProperty(heap,state.input.identity,key,descriptor);if(defined.error!=='')return {$:'objectIntegrityInvalid',heap:defined.heap,reason:defined.error};if(!defined.success)return {$:'objectIntegrityError',heap:defined.heap,errorClass:'TypeError',reason:'Cannot set the selected Object integrity level'};return {$:'objectIntegrityContinue',heap:defined.heap,state:{...state,position:state.position+1}};
}
