import {readJavaScriptSemanticValueError,testJavaScriptSemanticAccessor} from './javascript-semantic-values.js';
import {readJavaScriptPropertyDescriptorError} from './javascript-property-descriptors.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,locateJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptDescriptorField} from './javascript-descriptor-field-rules.js';

/** @typedef {{$:'descriptorConversion',receiver:JavaScriptSemanticValue,next:number,
 * descriptor:JavaScriptPropertyDescriptor,pending:boolean}} JavaScriptDescriptorConversionState
 */
/** @typedef {{$:'normalCompletion',value:JavaScriptSemanticValue} |
 * {$:'throwCompletion',value:JavaScriptSemanticValue}} JavaScriptSemanticCompletion
 */
/** @typedef {{$:'descriptorContinuation',state:JavaScriptDescriptorConversionState} |
 * {$:'descriptorCall',state:JavaScriptDescriptorConversionState,callee:JavaScriptSemanticValue,
 * receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'descriptorCompleted',descriptor:JavaScriptPropertyDescriptor} |
 * {$:'descriptorThrow',value:JavaScriptSemanticValue} |
 * {$:'descriptorIntrinsicError',errorClass:string,reason:string} |
 * {$:'descriptorInvalid',reason:string}} JavaScriptDescriptorConversionTransition
 */

/** ToBoolean for represented ordinary values. Host HTMLDDA and other exotic
 * object behavior is a distinct semantic domain, never inferred here.
 * @param {JavaScriptSemanticValue} value @returns {boolean} */
export function convertJavaScriptSemanticBoolean(value) {
  if(readJavaScriptSemanticValueError(value)!=='')throw new TypeError('ToBoolean requires a complete semantic value');
  switch(value.$){
    case 'undefined':return false;
    case 'null':return false;
    case 'boolean':return value.value;
    case 'number':return value.value!==0&&!Number.isNaN(value.value);
    case 'integer':return value.value!==0n;
    case 'text':return value.units.length!==0;
    case 'symbol':return true;
    case 'object':return true;
  }
}

/** @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptEmptyDescriptor(){
  return {configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,
    valuePresent:false,value:{$:'undefined'},writablePresent:false,writable:false,
    getterPresent:false,getter:{$:'undefined'},setterPresent:false,setter:{$:'undefined'}};
}

/** @param {JavaScriptPropertyDescriptor} descriptor @param {number} index @returns {boolean} */
export function testJavaScriptDescriptorFieldPresence(descriptor,index){
  return index===0?descriptor.enumerablePresent:index===1?descriptor.configurablePresent:index===2?descriptor.valuePresent:
    index===3?descriptor.writablePresent:index===4?descriptor.getterPresent:index===5?descriptor.setterPresent:false;
}

/** Prefix states may temporarily contain both data/accessor attributes. The
 * specification rejects that combination only after all required Get steps.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptDescriptorConversionState} state @returns {string} */
export function readJavaScriptDescriptorConversionStateError(heap,state){
  const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
  if(state.receiver.$!=='object')return 'descriptor conversion retains its actual Object receiver';
  const receiverError=readJavaScriptOrdinaryValueError(heap,state.receiver);if(receiverError!=='')return receiverError;
  if(!Number.isSafeInteger(state.next)||state.next<0||state.next>6||state.next===0&&1/state.next<0||state.pending&&state.next===6)return 'descriptor conversion needs its exact ordered field continuation';
  for(let index=state.next;index<6;index+=1)if(testJavaScriptDescriptorFieldPresence(state.descriptor,index))return 'unprocessed descriptor attributes cannot be fabricated in a continuation';
  const descriptor=state.descriptor;
  if(descriptor.valuePresent&&readJavaScriptOrdinaryValueError(heap,descriptor.value)!=='')return 'descriptor values belong to the actual latest global context';
  if(descriptor.getterPresent&&(readJavaScriptOrdinaryValueError(heap,descriptor.getter)!==''||!testJavaScriptSemanticAccessor(descriptor.getter)))return 'a processed descriptor getter retains actual intrinsic callability';
  if(descriptor.setterPresent&&(readJavaScriptOrdinaryValueError(heap,descriptor.setter)!==''||!testJavaScriptSemanticAccessor(descriptor.setter)))return 'a processed descriptor setter retains actual intrinsic callability';
  return '';
}

/** Apply exactly one already resolved Get result. Callable validation happens
 * immediately for get/set; mixed descriptor-kind validation happens at the end.
 * Intrinsic errors are explicit evaluator obligations, preserving their realm
 * and actual Error allocation rather than throwing a substitute host object.
 * @param {JavaScriptDescriptorConversionState} state @param {JavaScriptSemanticValue} value
 * @returns {JavaScriptDescriptorConversionTransition} */
export function applyJavaScriptDescriptorField(state,value){
  if((state.next===4||state.next===5)&&!testJavaScriptSemanticAccessor(value))
    return {$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a descriptor accessor must be callable or undefined'};
  const before=state.descriptor;
  const descriptor=state.next===0?{...before,enumerablePresent:true,enumerable:convertJavaScriptSemanticBoolean(value)}:
    state.next===1?{...before,configurablePresent:true,configurable:convertJavaScriptSemanticBoolean(value)}:
    state.next===2?{...before,valuePresent:true,value}:
    state.next===3?{...before,writablePresent:true,writable:convertJavaScriptSemanticBoolean(value)}:
    state.next===4?{...before,getterPresent:true,getter:value}:{...before,setterPresent:true,setter:value};
  return {$:'descriptorContinuation',state:{$:'descriptorConversion',receiver:state.receiver,next:state.next+1,descriptor,pending:false}};
}

/** ToPropertyDescriptor as a resumable source operation. Every HasProperty
 * and Get uses the latest supplied heap, so earlier getter effects can change
 * later fields/prototypes. No property is read twice to guess its meaning.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptDescriptorConversionState} state
 * @returns {JavaScriptDescriptorConversionTransition} */
export function stepJavaScriptDescriptorConversion(heap,state){
  const error=readJavaScriptDescriptorConversionStateError(heap,state);if(error!=='')return {$:'descriptorInvalid',reason:error};
  if(state.pending)return {$:'descriptorInvalid',reason:'a pending descriptor Get must be completed before its next step'};
  let current=state;
  while(current.next<6){
    const field=readJavaScriptDescriptorField(current.next);
    const key={$:'text',units:field.units};
    const receiver=current.receiver;
    if(receiver.$!=='object')return {$:'descriptorInvalid',reason:'descriptor conversion retains its actual Object receiver'};
    const property=locateJavaScriptOrdinaryProperty(heap,receiver.identity,key);
    if(!property.found){current={$:'descriptorConversion',receiver,next:current.next+1,descriptor:current.descriptor,pending:false};continue;}
    const plan=planJavaScriptOrdinaryGet(heap,receiver.identity,key,receiver);
    if(plan.error!=='')return {$:'descriptorInvalid',reason:plan.error};
    if(plan.action==='call')return {$:'descriptorCall',state:{$:'descriptorConversion',receiver,next:current.next,descriptor:current.descriptor,pending:true},callee:plan.callee,receiver:plan.receiver,arguments:plan.arguments};
    if(plan.action!=='value')return {$:'descriptorInvalid',reason:'ordinary Get requires its actual value or pending call'};
    const applied=applyJavaScriptDescriptorField(current,plan.value);
    if(applied.$!=='descriptorContinuation')return applied;
    current=applied.state;
  }
  if(readJavaScriptPropertyDescriptorError(current.descriptor)!=='')return {$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a descriptor cannot contain both data and accessor attributes'};
  return {$:'descriptorCompleted',descriptor:current.descriptor};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver
 * @returns {JavaScriptDescriptorConversionTransition} */
export function beginJavaScriptDescriptorConversion(heap,receiver){
  const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return {$:'descriptorInvalid',reason:heapError};
  const error=readJavaScriptOrdinaryValueError(heap,receiver);if(error!=='')return {$:'descriptorInvalid',reason:error};
  if(receiver.$!=='object')return {$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a property descriptor source must be an Object'};
  return stepJavaScriptDescriptorConversion(heap,{$:'descriptorConversion',receiver,next:0,descriptor:createJavaScriptEmptyDescriptor(),pending:false});
}

/** The executor supplies the actual completion and updated global state from
 * the pending call. Abrupt values remain exact; normal results resume at the
 * next ordered field. This protocol does not execute/replace that call itself.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptDescriptorConversionState} state
 * @param {JavaScriptSemanticCompletion} completion
 * @returns {JavaScriptDescriptorConversionTransition} */
export function resumeJavaScriptDescriptorConversion(heap,state,completion){
  const error=readJavaScriptDescriptorConversionStateError(heap,state);if(error!=='')return {$:'descriptorInvalid',reason:error};
  if(!state.pending)return {$:'descriptorInvalid',reason:'descriptor resumption needs its actual pending Get'};
  const valueError=readJavaScriptOrdinaryValueError(heap,completion.value);if(valueError!=='')return {$:'descriptorInvalid',reason:valueError};
  if(completion.$==='throwCompletion')return {$:'descriptorThrow',value:completion.value};
  const applied=applyJavaScriptDescriptorField(state,completion.value);
  if(applied.$!=='descriptorContinuation')return applied;
  return stepJavaScriptDescriptorConversion(heap,applied.state);
}
