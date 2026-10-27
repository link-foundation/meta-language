import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {beginJavaScriptDescriptorConversion,stepJavaScriptDescriptorConversion,resumeJavaScriptDescriptorConversion} from './javascript-descriptor-conversion.js';

/** @typedef {{$:'propertyDefinitionKey',target:JavaScriptSemanticValue,attributes:JavaScriptSemanticValue,conversion:JavaScriptPrimitiveConversionState} |
 * {$:'propertyDefinitionDescriptor',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,conversion:JavaScriptDescriptorConversionState}} JavaScriptPropertyDefinitionState
 */
/** @typedef {{$:'definitionContinuation',state:JavaScriptPropertyDefinitionState} |
 * {$:'definitionCall',state:JavaScriptPropertyDefinitionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'definitionCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue,writes:string[]} |
 * {$:'definitionThrow',value:JavaScriptSemanticValue} |
 * {$:'definitionIntrinsicError',errorClass:string,reason:string} |
 * {$:'definitionInvalid',reason:string}} JavaScriptPropertyDefinitionTransition
 */

/** Transport between the two qualified completion algebras, preserving every
 * constructor and exact value. Their names alone never establish equivalence.
 * @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPrimitiveConversionCompletion} */
export function projectJavaScriptPropertyKeyCompletion(completion){
  if(completion.$==='normalCompletion')return {$:'normalCompletion',value:completion.value};
  return {$:'throwCompletion',value:completion.value};
}

/** Complete an ordinary DefinePropertyOrThrow. A valid descriptor's actual
 * specification rejection is distinct from model/resource failure. The shared
 * heap protocol returns incompatibility without an invalid-input error.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function completeJavaScriptPropertyDefinition(heap,target,key,descriptor){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'definitionInvalid',reason:error};
  const valueError=readJavaScriptOrdinaryValueError(heap,target);if(valueError!=='')return {$:'definitionInvalid',reason:valueError};
  if(target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  const transition=defineJavaScriptOrdinaryProperty(heap,target.identity,key,descriptor);
  if(!transition.success){
    if(transition.error!=='')return {$:'definitionInvalid',reason:transition.error};
    return {$:'definitionIntrinsicError',errorClass:'TypeError',reason:'ordinary DefineOwnProperty rejected the requested descriptor'};
  }
  return {$:'definitionCompleted',heap:transition.heap,value:target,writes:transition.writes};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptDescriptorConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyDescriptorDefinition(heap,target,key,transition){
  switch(transition.$){
    case 'descriptorContinuation':return {$:'definitionContinuation',state:{$:'propertyDefinitionDescriptor',target,key,conversion:transition.state}};
    case 'descriptorCall':return {$:'definitionCall',state:{$:'propertyDefinitionDescriptor',target,key,conversion:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'descriptorCompleted':return completeJavaScriptPropertyDefinition(heap,target,key,transition.descriptor);
    case 'descriptorThrow':return {$:'definitionThrow',value:transition.value};
    case 'descriptorIntrinsicError':return {$:'definitionIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
    case 'descriptorInvalid':return {$:'definitionInvalid',reason:transition.reason};
  }
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} attributes @param {JavaScriptPropertyKeyConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyKeyDefinition(heap,target,attributes,transition){
  switch(transition.$){
    case 'propertyKeyContinuation':return {$:'definitionContinuation',state:{$:'propertyDefinitionKey',target,attributes,conversion:transition.state}};
    case 'propertyKeyCall':return {$:'definitionCall',state:{$:'propertyDefinitionKey',target,attributes,conversion:transition.state},callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'propertyKeyCompleted':return continueJavaScriptPropertyDescriptorDefinition(heap,target,transition.key,beginJavaScriptDescriptorConversion(heap,attributes));
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
  return continueJavaScriptPropertyKeyDefinition(heap,target,attributes,beginJavaScriptPropertyKeyConversion(heap,keyInput,exoticKey));
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function stepJavaScriptPropertyDefinition(heap,state){
  const error=readJavaScriptOrdinaryValueError(heap,state.target);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinition(heap,state.target,state.attributes,stepJavaScriptPropertyKeyConversion(heap,state.conversion));
  return continueJavaScriptPropertyDescriptorDefinition(heap,state.target,state.key,stepJavaScriptDescriptorConversion(heap,state.conversion));
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPropertyDefinitionTransition} */
export function resumeJavaScriptPropertyDefinition(heap,state,completion){
  const error=readJavaScriptOrdinaryValueError(heap,state.target);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinition(heap,state.target,state.attributes,resumeJavaScriptPropertyKeyConversion(heap,state.conversion,projectJavaScriptPropertyKeyCompletion(completion)));
  return continueJavaScriptPropertyDescriptorDefinition(heap,state.target,state.key,resumeJavaScriptDescriptorConversion(heap,state.conversion,completion));
}
