import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {beginJavaScriptDescriptorConversion,stepJavaScriptDescriptorConversion,resumeJavaScriptDescriptorConversion,readJavaScriptDescriptorConversionStateError} from './javascript-descriptor-conversion.js';

/** @typedef {{$:'propertyDefinitionKey',target:JavaScriptSemanticValue,attributes:JavaScriptSemanticValue,conversion:JavaScriptPrimitiveConversionState} |
 * {$:'propertyDefinitionDescriptor',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,conversion:JavaScriptDescriptorConversionState,exoticKey?:JavaScriptSemanticValue} |
 * {$:'propertyDefinitionArray',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,assignment:JavaScriptArrayAssignmentState}} JavaScriptPropertyDefinitionState
 */
/** @typedef {{$:'definitionContinuation',state:JavaScriptPropertyDefinitionState,heap?:JavaScriptOrdinaryHeap} |
 * {$:'definitionCall',state:JavaScriptPropertyDefinitionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],heap?:JavaScriptOrdinaryHeap} |
 * {$:'definitionCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue,writes:string[]} |
 * {$:'definitionThrow',value:JavaScriptSemanticValue,heap?:JavaScriptOrdinaryHeap} |
 * {$:'definitionIntrinsicError',errorClass:string,reason:string,heap?:JavaScriptOrdinaryHeap,writes?:string[]} |
 * {$:'definitionInvalid',reason:string,heap?:JavaScriptOrdinaryHeap} |
 * {$:'definitionResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyDefinitionState,reason:string}} JavaScriptPropertyDefinitionTransition
 */

/** Transport between the two qualified completion algebras, preserving every
 * constructor and exact value. Their names alone never establish equivalence.
 * @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPrimitiveConversionCompletion} */
export function projectJavaScriptPropertyKeyCompletion(completion){
  if(completion.$==='normalCompletion')return {$:'normalCompletion',value:completion.value};
  return {$:'throwCompletion',value:completion.value};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state @returns {string} */
export function readJavaScriptPropertyDefinitionStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,state.target);if(error!=='')return error;if(state.target.$!=='object')return 'property definition retains its actual Object target';
 if(state.$==='propertyDefinitionKey'){const invalid=readJavaScriptOrdinaryValueError(heap,state.attributes)||readJavaScriptPrimitiveConversionStateError(heap,state.conversion);return invalid|| (state.conversion.hint==='string'?'':'property key conversion retains its actual string hint');}
 const keyError=readJavaScriptOrdinaryKeyError(heap,state.key);if(keyError!=='')return keyError;
 if(state.$==='propertyDefinitionArray'){
  if(state.assignment.definition.identity!==state.target.identity||!testJavaScriptSemanticSameValue(state.assignment.definition.key,state.key))return 'Array definition retains its actual target key and completed descriptor';return readJavaScriptArrayAssignmentStateError(heap,state.assignment);
 }
 if(state.exoticKey!==undefined){const invalid=readJavaScriptOrdinaryValueError(heap,state.exoticKey);if(invalid!==''||state.exoticKey.$!=='symbol')return invalid||'property definition retains its actual primitive conversion Symbol';}
 return readJavaScriptDescriptorConversionStateError(heap,state.conversion);
}

/** @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key
 * @param {JavaScriptArrayAssignmentTransition} transition @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyArrayDefinition(target,key,transition){
 switch(transition.$){
  case 'arrayAssignmentContinue':return {$:'definitionContinuation',heap:transition.heap,state:{$:'propertyDefinitionArray',target,key,assignment:transition.state}};
  case 'arrayAssignmentCall':return {$:'definitionCall',heap:transition.heap,state:{$:'propertyDefinitionArray',target,key,assignment:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'arrayAssignmentCompleted':return transition.success?{$:'definitionCompleted',heap:transition.heap,value:target,writes:transition.writes??[]}:{$:'definitionIntrinsicError',heap:transition.heap,writes:transition.writes??[],errorClass:'TypeError',reason:'Array DefineOwnProperty rejected the requested descriptor'};
  case 'arrayAssignmentThrow':return {$:'definitionThrow',heap:transition.heap,value:transition.value};
  case 'arrayAssignmentError':return {$:'definitionIntrinsicError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arrayAssignmentInvalid':return {$:'definitionInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayAssignmentResourceLimit':return {$:'definitionResourceLimit',heap:transition.heap,state:{$:'propertyDefinitionArray',target,key,assignment:transition.state},reason:transition.reason};
 }
}

/** Complete an ordinary DefinePropertyOrThrow. A valid descriptor's actual
 * specification rejection is distinct from model/resource failure. The shared
 * heap protocol returns incompatibility without an invalid-input error.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function completeJavaScriptPropertyDefinition(heap,target,key,descriptor){return completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,descriptor,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor @param {JavaScriptSemanticValue|undefined} exoticKey @returns {JavaScriptPropertyDefinitionTransition} */
export function completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,descriptor,exoticKey){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'definitionInvalid',reason:error};
  const valueError=readJavaScriptOrdinaryValueError(heap,target);if(valueError!=='')return {$:'definitionInvalid',reason:valueError};
  if(target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)];
  if(object?.$==='arrayObject'&&testJavaScriptArrayLengthKey(key)&&exoticKey!==undefined)return continueJavaScriptPropertyArrayDefinition(target,key,beginJavaScriptArrayAssignment(heap,target.identity,key,descriptor,exoticKey));
  const transition=defineJavaScriptOrdinaryProperty(heap,target.identity,key,descriptor);
  if(!transition.success){
    if(transition.error!=='')return {$:'definitionInvalid',reason:transition.error};
    return {$:'definitionIntrinsicError',heap:transition.heap,errorClass:'TypeError',reason:'ordinary DefineOwnProperty rejected the requested descriptor'};
  }
  return {$:'definitionCompleted',heap:transition.heap,value:target,writes:transition.writes};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptDescriptorConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyDescriptorDefinition(heap,target,key,transition){return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,key,transition,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key @param {JavaScriptDescriptorConversionTransition} transition @param {JavaScriptSemanticValue|undefined} exoticKey @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,key,transition,exoticKey){
  switch(transition.$){
    case 'descriptorContinuation':return {$:'definitionContinuation',state:{$:'propertyDefinitionDescriptor',target,key,conversion:transition.state,...(exoticKey===undefined?{}:{exoticKey})}};
    case 'descriptorCall':return {$:'definitionCall',state:{$:'propertyDefinitionDescriptor',target,key,conversion:transition.state,...(exoticKey===undefined?{}:{exoticKey})},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'descriptorCompleted':return completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,transition.descriptor,exoticKey);
    case 'descriptorThrow':return {$:'definitionThrow',value:transition.value};
    case 'descriptorIntrinsicError':return {$:'definitionIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
    case 'descriptorInvalid':return {$:'definitionInvalid',reason:transition.reason};
  }
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} attributes @param {JavaScriptPropertyKeyConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyKeyDefinition(heap,target,attributes,transition){return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,transition,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} attributes @param {JavaScriptPropertyKeyConversionTransition} transition @param {JavaScriptSemanticValue|undefined} exoticKey @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,transition,exoticKey){
  switch(transition.$){
    case 'propertyKeyContinuation':return {$:'definitionContinuation',state:{$:'propertyDefinitionKey',target,attributes,conversion:transition.state}};
    case 'propertyKeyCall':return {$:'definitionCall',state:{$:'propertyDefinitionKey',target,attributes,conversion:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'propertyKeyCompleted':return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,transition.key,beginJavaScriptDescriptorConversion(heap,attributes),exoticKey);
    case 'propertyKeyThrow':return {$:'definitionThrow',value:transition.value};
    case 'propertyKeyIntrinsicError':return {$:'definitionIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
    case 'propertyKeyInvalid':return {$:'definitionInvalid',reason:transition.reason};
  }
}

/** Object.defineProperty: check the target, convert the property key, read the
 * descriptor, then define or throw. Descriptor getters never precede key
 * conversion, and the original target is the successful result.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} keyInput @param {JavaScriptSemanticValue} attributes
 * @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptPropertyDefinitionTransition} */
export function beginJavaScriptPropertyDefinition(heap,target,keyInput,attributes,exoticKey){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'definitionInvalid',reason:error};
  const valueError=readJavaScriptOrdinaryValueError(heap,target);if(valueError!=='')return {$:'definitionInvalid',reason:valueError};
  if(target.$!=='object')return {$:'definitionIntrinsicError',errorClass:'TypeError',reason:'Object.defineProperty requires an Object target'};
  return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,beginJavaScriptPropertyKeyConversion(heap,keyInput,exoticKey),exoticKey);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function stepJavaScriptPropertyDefinition(heap,state){
  const error=readJavaScriptPropertyDefinitionStateError(heap,state);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,state.target,state.attributes,stepJavaScriptPropertyKeyConversion(heap,state.conversion),state.conversion.exoticKey);
  if(state.$==='propertyDefinitionArray')return continueJavaScriptPropertyArrayDefinition(state.target,state.key,stepJavaScriptArrayAssignment(heap,state.assignment));
  return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,state.target,state.key,stepJavaScriptDescriptorConversion(heap,state.conversion),state.exoticKey);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPropertyDefinitionTransition} */
export function resumeJavaScriptPropertyDefinition(heap,state,completion){
  const error=readJavaScriptPropertyDefinitionStateError(heap,state);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,state.target,state.attributes,resumeJavaScriptPropertyKeyConversion(heap,state.conversion,projectJavaScriptPropertyKeyCompletion(completion)),state.conversion.exoticKey);
  if(state.$==='propertyDefinitionArray')return continueJavaScriptPropertyArrayDefinition(state.target,state.key,resumeJavaScriptArrayAssignment(heap,state.assignment,completion));
  return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,state.target,state.key,resumeJavaScriptDescriptorConversion(heap,state.conversion,completion),state.exoticKey);
}
