import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptPropertyDescriptorError} from './javascript-property-descriptors.js';
import {readJavaScriptPrimitiveConversionSpelling} from './javascript-primitive-conversion-spellings.js';
import {readJavaScriptDescriptorField} from './javascript-descriptor-field-rules.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {beginJavaScriptDescriptorConversion,stepJavaScriptDescriptorConversion,resumeJavaScriptDescriptorConversion,readJavaScriptDescriptorConversionStateError} from './javascript-descriptor-conversion.js';

/** @typedef {{$:'propertyDefinitionCallRecord',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptPropertyDefinitionCall */
/** @typedef {{$:'propertyDefinitionKey',target:JavaScriptSemanticValue,attributes:JavaScriptSemanticValue,conversion:JavaScriptPrimitiveConversionState,operation?:string,call?:JavaScriptPropertyDefinitionCall} |
 * {$:'propertyDefinitionDescriptor',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,conversion:JavaScriptDescriptorConversionState,exoticKey?:JavaScriptSemanticValue,operation?:string,call?:JavaScriptPropertyDefinitionCall} |
 * {$:'propertyDefinitionArray',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,assignment:JavaScriptArrayAssignmentState,operation?:string,call?:JavaScriptPropertyDefinitionCall} |
 * {$:'propertyDefinitionReady',target:JavaScriptSemanticValue,key:JavaScriptSemanticValue,descriptor:JavaScriptPropertyDescriptor,exoticKey?:JavaScriptSemanticValue,operation?:string,call?:JavaScriptPropertyDefinitionCall}} JavaScriptPropertyDefinitionState
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
 if(!['propertyDefinitionKey','propertyDefinitionDescriptor','propertyDefinitionArray','propertyDefinitionReady'].includes(state.$))return 'property definition retains its actual typed continuation';
 const error=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,state.target);if(error!=='')return error;if(state.target.$!=='object')return 'property definition retains its actual Object target';
 if(state.operation!==undefined&&!['Object.defineProperty','Reflect.defineProperty'].includes(state.operation))return 'property definition retains its actual Object or Reflect result policy';
 if(state.call!==undefined){
  const call=state.call;if(call.$!=='propertyDefinitionCallRecord'||call.callee.$!=='object'||!call.callee.callable||call.arguments.length>2||state.$==='propertyDefinitionReady')return 'property definitions retain only their actual selected bounded child call';for(const value of [call.callee,call.receiver,...call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
  if(state.$==='propertyDefinitionDescriptor'){if(!state.conversion.pending||!testJavaScriptSemanticSameValue(call.receiver,state.conversion.receiver))return 'definition child calls retain their actual pending descriptor receiver';const invalid=readJavaScriptDefinitionGetterArguments(heap,call,{$:'text',units:readJavaScriptDescriptorField(state.conversion.next).units});if(invalid!=='')return invalid;}
  else{const conversion=state.$==='propertyDefinitionKey'?state.conversion:state.$==='propertyDefinitionArray'&&state.assignment.$==='arrayAssignmentNumber'?state.assignment.conversion:undefined;if(conversion===undefined||!testJavaScriptSemanticSameValue(call.receiver,conversion.receiver)||!['exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(conversion.phase))return 'definition child calls retain their actual pending primitive conversion and receiver';if(conversion.phase==='exoticCall'){if(call.arguments.length!==1||!testJavaScriptSemanticSameValue(call.arguments[0]??{$:'undefined'},readJavaScriptPrimitiveConversionSpelling(conversion.hint)))return 'definition exotic conversion calls retain their actual preferred-type hint';}else if(conversion.phase==='firstCall'||conversion.phase==='secondCall'){if(call.arguments.length!==0)return 'definition ordinary conversion methods retain their empty argument List';}else{const first=conversion.phase==='firstGetter',name=conversion.hint==='string'?(first?'toString':'valueOf'):(first?'valueOf':'toString'),key=conversion.phase==='exoticGetter'?conversion.exoticKey:readJavaScriptPrimitiveConversionSpelling(name),invalid=readJavaScriptDefinitionGetterArguments(heap,call,key);if(invalid!=='')return invalid;}}
 }
 if(state.$==='propertyDefinitionKey'){const invalid=readJavaScriptOrdinaryValueError(heap,state.attributes)||readJavaScriptPrimitiveConversionStateError(heap,state.conversion);return invalid|| (state.conversion.hint==='string'?'':'property key conversion retains its actual string hint');}
 const keyError=readJavaScriptOrdinaryKeyError(heap,state.key);if(keyError!=='')return keyError;
 if(state.$==='propertyDefinitionArray'){
  if(state.assignment.definition.identity!==state.target.identity||!testJavaScriptSemanticSameValue(state.assignment.definition.key,state.key))return 'Array definition retains its actual target key and completed descriptor';return readJavaScriptArrayAssignmentStateError(heap,state.assignment);
 }
 if(state.exoticKey!==undefined){const invalid=readJavaScriptOrdinaryValueError(heap,state.exoticKey);if(invalid!==''||state.exoticKey.$!=='symbol')return invalid||'property definition retains its actual primitive conversion Symbol';}
 if(state.$==='propertyDefinitionReady'){const error=readJavaScriptPropertyDescriptorError(state.descriptor);if(error!=='')return error;for(const value of [state.descriptor.value,state.descriptor.getter,state.descriptor.setter]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}return '';}
 return readJavaScriptDescriptorConversionStateError(heap,state.conversion);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionCall} call @param {JavaScriptSemanticValue} key @returns {string} */
export function readJavaScriptDefinitionGetterArguments(heap,call,key){
 const owner=call.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,call.receiver.identity)]:undefined;
 if(owner!==undefined&&['hostErrorObject','hostURLObject','hostArrayObject','hostByteBufferObject'].includes(owner.$))return call.arguments.length===2&&testJavaScriptSemanticSameValue(call.callee,owner.hostGet)&&testJavaScriptSemanticSameValue(call.arguments[0]??{$:'undefined'},key)&&testJavaScriptSemanticSameValue(call.arguments[1]??{$:'undefined'},call.receiver)?'':'definition opaque Get retains its actual registered key and receiver operands';
 return call.arguments.length===0?'':'definition ordinary getters retain their empty argument List';
}

/** @param {JavaScriptPropertyDefinitionState} state @param {string|undefined} operation @param {JavaScriptPropertyDefinitionCall|undefined} [call] @returns {JavaScriptPropertyDefinitionState} */
export function retainJavaScriptPropertyDefinitionContext(state,operation,call=undefined){return {...state,...(operation===undefined?{}:{operation}),...(call===undefined?{}:{call})};}
/** @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptPropertyDefinitionCall} */
export function createJavaScriptPropertyDefinitionCall(callee,receiver,arguments_){return {$:'propertyDefinitionCallRecord',callee,receiver,arguments:arguments_.slice()};}

/** @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key
 * @param {JavaScriptArrayAssignmentTransition} transition @param {string|undefined} [operation] @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyArrayDefinition(target,key,transition,operation=undefined){
 switch(transition.$){
  case 'arrayAssignmentContinue':return {$:'definitionContinuation',heap:transition.heap,state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionArray',target,key,assignment:transition.state},operation)};
  case 'arrayAssignmentCall':return {$:'definitionCall',heap:transition.heap,state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionArray',target,key,assignment:transition.state},operation,createJavaScriptPropertyDefinitionCall(transition.callee,transition.receiver,transition.arguments)),callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
  case 'arrayAssignmentCompleted':return operation==='Reflect.defineProperty'||transition.success?{$:'definitionCompleted',heap:transition.heap,value:operation==='Reflect.defineProperty'?{$:'boolean',value:transition.success}:target,writes:transition.writes??[]}:{$:'definitionIntrinsicError',heap:transition.heap,writes:transition.writes??[],errorClass:'TypeError',reason:'Array DefineOwnProperty rejected the requested descriptor'};
  case 'arrayAssignmentThrow':return {$:'definitionThrow',heap:transition.heap,value:transition.value};
  case 'arrayAssignmentError':return {$:'definitionIntrinsicError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arrayAssignmentInvalid':return {$:'definitionInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayAssignmentResourceLimit':return {$:'definitionResourceLimit',heap:transition.heap,state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionArray',target,key,assignment:transition.state},operation),reason:transition.reason};
 }
}

/** Complete an ordinary DefinePropertyOrThrow. A valid descriptor's actual
 * specification rejection is distinct from model/resource failure. The shared
 * heap protocol returns incompatibility without an invalid-input error.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function completeJavaScriptPropertyDefinition(heap,target,key,descriptor){return completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,descriptor,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor @param {JavaScriptSemanticValue|undefined} exoticKey @param {string|undefined} [operation] @returns {JavaScriptPropertyDefinitionTransition} */
export function completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,descriptor,exoticKey,operation=undefined){
  if(operation!==undefined&&!['Object.defineProperty','Reflect.defineProperty'].includes(operation))return {$:'definitionInvalid',heap,reason:'a completed property definition retains its actual Object or Reflect result policy'};
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'definitionInvalid',reason:error};
  const valueError=readJavaScriptOrdinaryValueError(heap,target);if(valueError!=='')return {$:'definitionInvalid',reason:valueError};
  if(target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)];
  if(object?.$==='arrayObject'&&testJavaScriptArrayLengthKey(key)&&exoticKey!==undefined)return continueJavaScriptPropertyArrayDefinition(target,key,beginJavaScriptArrayAssignment(heap,target.identity,key,descriptor,exoticKey),operation);
  const transition=defineJavaScriptOrdinaryProperty(heap,target.identity,key,descriptor);
  if(!transition.success){
    if(transition.resourceLimit===true)return {$:'definitionResourceLimit',heap:transition.heap,state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionReady',target,key,descriptor,...(exoticKey===undefined?{}:{exoticKey})},operation),reason:transition.error};
    if(transition.error!=='')return {$:'definitionInvalid',reason:transition.error};
    if(operation==='Reflect.defineProperty')return {$:'definitionCompleted',heap:transition.heap,value:{$:'boolean',value:false},writes:transition.writes};
    return {$:'definitionIntrinsicError',heap:transition.heap,errorClass:'TypeError',reason:'ordinary DefineOwnProperty rejected the requested descriptor'};
  }
  return {$:'definitionCompleted',heap:transition.heap,value:operation==='Reflect.defineProperty'?{$:'boolean',value:true}:target,writes:transition.writes};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} key @param {JavaScriptDescriptorConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyDescriptorDefinition(heap,target,key,transition){return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,key,transition,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key @param {JavaScriptDescriptorConversionTransition} transition @param {JavaScriptSemanticValue|undefined} exoticKey @param {string|undefined} [operation] @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,key,transition,exoticKey,operation=undefined){
  switch(transition.$){
    case 'descriptorContinuation':return {$:'definitionContinuation',state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionDescriptor',target,key,conversion:transition.state,...(exoticKey===undefined?{}:{exoticKey})},operation)};
    case 'descriptorCall':return {$:'definitionCall',state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionDescriptor',target,key,conversion:transition.state,...(exoticKey===undefined?{}:{exoticKey})},operation,createJavaScriptPropertyDefinitionCall(transition.callee,transition.receiver,transition.arguments)),callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'descriptorCompleted':return completeJavaScriptPropertyDefinitionWithSymbol(heap,target,key,transition.descriptor,exoticKey,operation);
    case 'descriptorThrow':return {$:'definitionThrow',value:transition.value};
    case 'descriptorIntrinsicError':return {$:'definitionIntrinsicError',errorClass:transition.errorClass,reason:transition.reason};
    case 'descriptorInvalid':return {$:'definitionInvalid',reason:transition.reason};
  }
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} attributes @param {JavaScriptPropertyKeyConversionTransition} transition
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyKeyDefinition(heap,target,attributes,transition){return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,transition,undefined);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} attributes @param {JavaScriptPropertyKeyConversionTransition} transition @param {JavaScriptSemanticValue|undefined} exoticKey @param {string|undefined} [operation] @returns {JavaScriptPropertyDefinitionTransition} */
export function continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,transition,exoticKey,operation=undefined){
  switch(transition.$){
    case 'propertyKeyContinuation':return {$:'definitionContinuation',state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionKey',target,attributes,conversion:transition.state},operation)};
    case 'propertyKeyCall':return {$:'definitionCall',state:retainJavaScriptPropertyDefinitionContext({$:'propertyDefinitionKey',target,attributes,conversion:transition.state},operation,createJavaScriptPropertyDefinitionCall(transition.callee,transition.receiver,transition.arguments)),callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};
    case 'propertyKeyCompleted':return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,target,transition.key,beginJavaScriptDescriptorConversion(heap,attributes),exoticKey,operation);
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
 * @param {JavaScriptSemanticValue} exoticKey @param {string|undefined} [operation] @returns {JavaScriptPropertyDefinitionTransition} */
export function beginJavaScriptPropertyDefinition(heap,target,keyInput,attributes,exoticKey,operation=undefined){
  const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'definitionInvalid',reason:error};
  const valueError=readJavaScriptOrdinaryValueError(heap,target);if(valueError!=='')return {$:'definitionInvalid',reason:valueError};
  if(operation!==undefined&&!['Object.defineProperty','Reflect.defineProperty'].includes(operation))return {$:'definitionInvalid',reason:'a property definition requires its actual selected Object or Reflect operation'};
  if(target.$!=='object')return {$:'definitionIntrinsicError',errorClass:'TypeError',reason:(operation??'Object.defineProperty')+' requires an Object target'};
  return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,target,attributes,beginJavaScriptPropertyKeyConversion(heap,keyInput,exoticKey),exoticKey,operation);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @returns {JavaScriptPropertyDefinitionTransition} */
export function stepJavaScriptPropertyDefinition(heap,state){
  const error=readJavaScriptPropertyDefinitionStateError(heap,state);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.call!==undefined)return {$:'definitionCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,state.target,state.attributes,stepJavaScriptPropertyKeyConversion(heap,state.conversion),state.conversion.exoticKey,state.operation);
  if(state.$==='propertyDefinitionArray')return continueJavaScriptPropertyArrayDefinition(state.target,state.key,stepJavaScriptArrayAssignment(heap,state.assignment),state.operation);
  if(state.$==='propertyDefinitionReady')return completeJavaScriptPropertyDefinitionWithSymbol(heap,state.target,state.key,state.descriptor,state.exoticKey,state.operation);
  return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,state.target,state.key,stepJavaScriptDescriptorConversion(heap,state.conversion),state.exoticKey,state.operation);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDefinitionState} state
 * @param {JavaScriptSemanticCompletion} completion @returns {JavaScriptPropertyDefinitionTransition} */
export function resumeJavaScriptPropertyDefinition(heap,state,completion){
  const error=readJavaScriptPropertyDefinitionStateError(heap,state);if(error!=='')return {$:'definitionInvalid',reason:error};
  if(state.target.$!=='object')return {$:'definitionInvalid',reason:'a property definition continuation retains its actual Object target'};
  if(state.$==='propertyDefinitionReady')return {$:'definitionInvalid',heap,reason:'a completed descriptor storage suspension cannot accept a child completion'};
  if(state.$==='propertyDefinitionKey')return continueJavaScriptPropertyKeyDefinitionWithSymbol(heap,state.target,state.attributes,resumeJavaScriptPropertyKeyConversion(heap,state.conversion,projectJavaScriptPropertyKeyCompletion(completion)),state.conversion.exoticKey,state.operation);
  if(state.$==='propertyDefinitionArray')return continueJavaScriptPropertyArrayDefinition(state.target,state.key,resumeJavaScriptArrayAssignment(heap,state.assignment,completion),state.operation);
  return continueJavaScriptPropertyDescriptorDefinitionWithSymbol(heap,state.target,state.key,resumeJavaScriptDescriptorConversion(heap,state.conversion,completion),state.exoticKey,state.operation);
}
