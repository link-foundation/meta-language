// Global allocation and resumable ArraySetLength. Numeric requests are actual
// ToNumber obligations; a frontend must execute them, including both effects.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,testJavaScriptOrdinarySymbol,replaceJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
import {readJavaScriptPropertyDescriptorKind} from './javascript-property-descriptors.js';
import {testJavaScriptSemanticAccessor} from './javascript-semantic-values.js';
import {constructJavaScriptArrayObject,testJavaScriptArrayLength,testJavaScriptArrayLengthKey,planJavaScriptArrayPropertyDefinition,planJavaScriptArrayLengthDefinition,convertJavaScriptArrayUnsignedLength} from './javascript-array-objects.js';
/** This is an operation identifier, never authority inferred from a method name.
 * @param {string} operation @returns {boolean} */
export function testJavaScriptArrayIntrinsicOperation(operation){return operation==='Array.isArray';}
/** @typedef {{$:'arrayDefinitionState',identity:bigint,key:JavaScriptSemanticValue,descriptor:JavaScriptPropertyDescriptor,phase:string,unsignedLength:number,numberLength:JavaScriptSemanticValue}} JavaScriptArrayDefinitionState */
/** @typedef {{$:'arrayNormal',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayPropertyComplete',heap:JavaScriptOrdinaryHeap,success:boolean,writes:string[]} |
 * {$:'arrayNumberRequest',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayDefinitionState,input:JavaScriptSemanticValue} |
 * {$:'arrayDefinitionContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayDefinitionState} |
 * {$:'arrayInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string,state?:JavaScriptArrayDefinitionState}} JavaScriptArrayCompletion */
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {number} length @returns {JavaScriptArrayCompletion} */
export function allocateJavaScriptArray(heap,identity,prototype,length){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'arrayInvalid',heap,reason:error};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||testJavaScriptOrdinarySymbol(heap.symbols,identity))return {$:'arrayInvalid',heap,reason:'Array allocation requires a fresh external unsigned64 global identity'};
 const actual=length===0?0:length;if(!testJavaScriptArrayLength(actual))return {$:'arrayError',heap,errorClass:'RangeError',reason:'Invalid array length'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'arrayInvalid',heap,reason:'Array allocation retains its actual null or Object prototype'};
 const invalid=readJavaScriptOrdinaryValueError(heap,prototype);if(invalid!=='')return {$:'arrayInvalid',heap,reason:invalid};
 let count=0;for(const object of heap.objects)count+=object.properties.length;
 if(heap.objects.length>=4096||count>=16384)return {$:'arrayResourceLimit',heap,reason:'Array allocation exceeds the shared object or property directory budget'};
 const object=constructJavaScriptArrayObject(identity,prototype,actual);let at=0;while(at<heap.objects.length&&(heap.objects[at]?.identity??identity)<identity)at++;
 const next={...heap,objects:heap.objects.slice(0,at).concat([object],heap.objects.slice(at))},problem=readJavaScriptOrdinaryHeapError(next);
 return problem!==''?{$:'arrayInvalid',heap,reason:problem}:{$:'arrayNormal',heap:next,value:{$:'object',identity,callable:false}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayDefinitionState} state @returns {string} */
export function readJavaScriptArrayDefinitionStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(state.$!=='arrayDefinitionState'||!['first number','second number','apply'].includes(state.phase)||!testJavaScriptArrayLength(state.unsignedLength))return 'Array definition requires its actual bounded conversion phase';
 if(state.numberLength.$!=='number')return 'Array numeric comparison retains its exact semantic Number';
 const numberError=readJavaScriptOrdinaryValueError(heap,state.numberLength);if(numberError!=='')return numberError;
 const keyError=readJavaScriptOrdinaryKeyError(heap,state.key);if(keyError!=='')return keyError;
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.identity)];if(object===undefined||object.$!=='arrayObject')return 'Array definition requires its actual branded global receiver';
 if(readJavaScriptPropertyDescriptorKind(state.descriptor)==='invalid')return 'an Array definition requires a valid partial internal descriptor';
 for(const value of [state.descriptor.value,state.descriptor.getter,state.descriptor.setter]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.descriptor.getterPresent&&!testJavaScriptSemanticAccessor(state.descriptor.getter)||state.descriptor.setterPresent&&!testJavaScriptSemanticAccessor(state.descriptor.setter))return 'Array accessors must be actual callable or undefined semantic values';
 if(state.phase!=='apply'&&(!testJavaScriptArrayLengthKey(state.key)||!state.descriptor.valuePresent))return 'only an actual Array length value starts the numeric conversion protocol';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} descriptor @returns {JavaScriptArrayCompletion} */
export function beginJavaScriptArrayPropertyDefinition(heap,identity,key,descriptor){
 const state={$:'arrayDefinitionState',identity,key,descriptor,phase:testJavaScriptArrayLengthKey(key)&&descriptor.valuePresent?'first number':'apply',unsignedLength:0,numberLength:{$:'number',value:0}};
 const error=readJavaScriptArrayDefinitionStateError(heap,state);return error!==''?{$:'arrayInvalid',heap,reason:error}:{$:'arrayDefinitionContinue',heap,state};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayDefinitionState} state @returns {JavaScriptArrayCompletion} */
export function stepJavaScriptArrayPropertyDefinition(heap,state){
 const error=readJavaScriptArrayDefinitionStateError(heap,state);if(error!=='')return {$:'arrayInvalid',heap,reason:error};
 if(state.phase!=='apply')return {$:'arrayNumberRequest',heap,state,input:state.descriptor.value};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.identity)];if(object===undefined||object.$!=='arrayObject')return {$:'arrayInvalid',heap,reason:'Array receiver must remain in the shared global directory'};
 if(state.numberLength.$!=='number')return {$:'arrayInvalid',heap,reason:'Array length application retains its actual semantic Number'};
 const plan=testJavaScriptArrayLengthKey(state.key)&&state.descriptor.valuePresent?planJavaScriptArrayLengthDefinition(object,state.descriptor,state.unsignedLength,state.numberLength.value):planJavaScriptArrayPropertyDefinition(object,state.key,state.descriptor);
 if(plan.errorClass==='RangeError'||plan.errorClass==='TypeError')return {$:'arrayError',heap,errorClass:plan.errorClass,reason:plan.error};
 if(plan.errorClass==='resource')return {$:'arrayResourceLimit',heap,reason:plan.error,state};
 if(plan.errorClass!=='')return {$:'arrayInvalid',heap,reason:plan.error};
 const updated=replaceJavaScriptOrdinaryObject(heap,plan.object),invalid=readJavaScriptOrdinaryHeapError(updated);
 if(invalid!=='')return {$:'arrayResourceLimit',heap,reason:invalid,state};
 return {$:'arrayPropertyComplete',heap:updated,success:plan.success,writes:plan.writes};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayDefinitionState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCompletion} */
export function resumeJavaScriptArrayPropertyNumber(heap,state,value){
 const error=readJavaScriptArrayDefinitionStateError(heap,state);if(error!=='')return {$:'arrayInvalid',heap,reason:error};
 const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'arrayInvalid',heap,reason:invalid};
 if(value.$!=='number'||state.phase==='apply')return {$:'arrayInvalid',heap,reason:'Array numeric conversion resumes once with its actual ToNumber result'};
 const next=state.phase==='first number'?{...state,phase:'second number',unsignedLength:convertJavaScriptArrayUnsignedLength(value.value)}:{...state,phase:'apply',numberLength:value};
 return {$:'arrayDefinitionContinue',heap,state:next};
}
/** Array branding is independent of its prototype or indexed properties.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} value @returns {{array:boolean,error:string}} */
export function testJavaScriptArrayValue(heap,value){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {array:false,error};
 const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {array:false,error:invalid};
 const object=value.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)]:undefined;
 return {array:object!==undefined&&(object.$==='arrayObject'||object.$==='hostArrayObject'),error:''};
}
