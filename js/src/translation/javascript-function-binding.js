// Function.prototype.bind uses represented internal slots and ordinary metadata
// Get continuations. No original Function.bind or target body executes here.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {testJavaScriptReflectionPropertyBudget} from './javascript-property-reflection.js';
/** @typedef {{$:'functionBindCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptFunctionBindCall */
/** @typedef {{$:'functionBindState',target:JavaScriptSemanticValue,boundThis:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],phase:string,prototype:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,length:number,name:JavaScriptSemanticValue,call?:JavaScriptFunctionBindCall}} JavaScriptFunctionBindState */
/** @typedef {{$:'functionBindContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptFunctionBindState} |
 * {$:'functionBindAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptFunctionBindState,prototype:JavaScriptSemanticValue} |
 * {$:'functionBindCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptFunctionBindState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'functionBindCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'functionBindThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'functionBindError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'functionBindInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'functionBindResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptFunctionBindState,reason:string}} JavaScriptFunctionBindTransition */
const lengthKey={$:'text',units:[108,101,110,103,116,104]},nameKey={$:'text',units:[110,97,109,101]};
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptFunctionBindState} state @returns {string} */
export function readJavaScriptFunctionBindStateError(heap,state){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return invalid;
 if(state.$!=='functionBindState'||!['prototype','allocate','length own','length','length getter','define length','name','name getter','name value','define name'].includes(state.phase)||state.arguments.length>1048576||typeof state.length!=='number'||state.length<0||Object.is(state.length,-0)||Number.isNaN(state.length)||state.length!==Infinity&&!Number.isInteger(state.length))return 'binding retains its actual bounded arguments metadata and phase';
 for(const value of [state.target,state.boundThis,state.prototype,state.receiver,state.name,...state.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.phase==='define name'&&(readJavaScriptSemanticTextLength(state.name)<6||[98,111,117,110,100,32].some((unit,index)=>readJavaScriptSemanticTextUnit(state.name,index)!==unit)))return 'bound Function names retain their actual String prefix';
 if(state.target.$!=='object'||!state.target.callable)return 'binding retains its actual callable target';
 if(state.phase!=='prototype'&&state.prototype.$!=='object'&&state.prototype.$!=='null')return 'binding retains its actual selected target prototype';
 if(!['prototype','allocate'].includes(state.phase)){
  const object=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
  if(object?.$!=='boundFunctionObject'||!testJavaScriptSemanticSameValue(object.boundTarget,state.target)||!testJavaScriptSemanticSameValue(object.boundThis,state.boundThis)||!testJavaScriptSemanticSameValue(object.prototype,state.prototype)||object.boundArguments.length!==state.arguments.length||object.boundArguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,state.arguments[index]??{$:'undefined'})))return 'binding metadata retains its actual fresh receiver and immutable internal slots';
 }
 if(state.call!==undefined){if(!['length getter','name getter'].includes(state.phase)||state.call.$!=='functionBindCall'||state.call.arguments.length!==0||!testJavaScriptSemanticSameValue(state.call.receiver,state.target)||state.call.callee.$!=='object'||!state.call.callee.callable)return 'binding retains its actual selected metadata getter and receiver';}
 else if(['length getter','name getter'].includes(state.phase))return 'binding getter phases retain their actual suspended call';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} boundThis @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptFunctionBindTransition} */
export function beginJavaScriptFunctionBind(heap,target,boundThis,arguments_){
 if(target.$!=='object'||!target.callable)return {$:'functionBindError',heap,errorClass:'TypeError',reason:'Bind must be called on a function'};
 const state={$:'functionBindState',target,boundThis,arguments:arguments_.slice(),phase:'prototype',prototype:{$:'undefined'},receiver:{$:'undefined'},length:0,name:{$:'text',units:[]}},error=readJavaScriptFunctionBindStateError(heap,state);return error===''?{$:'functionBindContinue',heap,state}:{$:'functionBindInvalid',heap,reason:error};
}
/** @param {JavaScriptSemanticValue} value @param {number} count @returns {number} */
export function readJavaScriptBoundFunctionLength(value,count){if(value.$!=='number'||Number.isNaN(value.value)||value.value===-Infinity)return 0;if(value.value===Infinity)return Infinity;return Math.max(0,Math.trunc(value.value)-count);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptFunctionBindState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptFunctionBindTransition} */
export function retainJavaScriptFunctionBindMetadata(heap,state,value){
 const {call:previous,...base}=state;if(state.phase==='length getter'||state.phase==='length')return {$:'functionBindContinue',heap,state:{...base,phase:'define length',length:readJavaScriptBoundFunctionLength(value,state.arguments.length)}};
 const size=readJavaScriptSemanticTextLength(value);if(size>65530)return {$:'functionBindResourceLimit',heap,state:{...base,phase:'name value',name:value},reason:'bound Function name exceeds the selected UTF16 metadata work profile'};
 const units=[98,111,117,110,100,32];for(let index=0;index<size;index++)units.push(readJavaScriptSemanticTextUnit(value,index));return {$:'functionBindContinue',heap,state:{...base,phase:'define name',name:{$:'text',units}}};
}
/** @param {JavaScriptSemanticValue} value @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptBoundFunctionMetadataDescriptor(value){const descriptor=createJavaScriptPropertyDescriptor('data',true,false);descriptor.value=value;descriptor.writable=false;return descriptor;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptFunctionBindState} state @returns {JavaScriptFunctionBindTransition} */
export function stepJavaScriptFunctionBind(heap,state){
 const error=readJavaScriptFunctionBindStateError(heap,state);if(error!=='')return {$:'functionBindInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'functionBindCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='name value')return retainJavaScriptFunctionBindMetadata(heap,state,state.name);
 if(state.phase==='prototype'){const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];if(target===undefined)return {$:'functionBindInvalid',heap,reason:'binding prototype selection retains its actual callable Object'};return {$:'functionBindContinue',heap,state:{...state,phase:'allocate',prototype:target.prototype}};}
 if(state.phase==='allocate')return heap.objects.length>=4096?{$:'functionBindResourceLimit',heap,state,reason:'bound Function allocation exceeds the shared heap identity budget'}:{$:'functionBindAllocate',heap,state,prototype:state.prototype};
 if(state.phase==='length own'){const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];return target===undefined?{$:'functionBindInvalid',heap,reason:'binding retains its actual target own-property inventory'}:{$:'functionBindContinue',heap,state:{...state,phase:findJavaScriptOrdinaryProperty(target.properties,lengthKey)<0?'define length':'length'}};}
 if(state.phase==='length'||state.phase==='name'){
  const get=planJavaScriptOrdinaryGet(heap,state.target.identity,state.phase==='length'?lengthKey:nameKey,state.target);if(get.error!=='')return {$:'functionBindInvalid',heap,reason:get.error};if(get.action==='value')return retainJavaScriptFunctionBindMetadata(heap,state,get.value);if(get.action!=='call')return {$:'functionBindInvalid',heap,reason:'binding metadata Get retains its actual value or getter'};
  const call={$:'functionBindCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'functionBindCall',heap,state:{...state,phase:state.phase==='length'?'length getter':'name getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
 }
 if(state.receiver.$!=='object')return {$:'functionBindInvalid',heap,reason:'binding metadata definitions retain their actual allocated Function'};
 if(state.phase==='define name'&&readJavaScriptSemanticTextLength(state.name)>65536)return {$:'functionBindResourceLimit',heap,state,reason:'bound Function name exceeds the selected UTF16 metadata work profile'};
 if(!testJavaScriptReflectionPropertyBudget(heap,state.receiver,state.phase==='define length'?lengthKey:nameKey))return {$:'functionBindResourceLimit',heap,state,reason:'bound Function metadata exceeds the shared property budget'};
 const length=state.phase==='define length',descriptor=createJavaScriptBoundFunctionMetadataDescriptor(length?{$:'number',value:state.length}:state.name),defined=defineJavaScriptOrdinaryProperty(heap,state.receiver.identity,length?lengthKey:nameKey,descriptor);if(defined.error!==''||!defined.success)return {$:'functionBindInvalid',heap,reason:defined.error||'fresh bound Function metadata definition succeeds'};
 return length?{$:'functionBindContinue',heap:defined.heap,state:{...state,phase:'name'}}:{$:'functionBindCompleted',heap:defined.heap,value:state.receiver};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptFunctionBindState} state @param {bigint} identity @returns {JavaScriptFunctionBindTransition} */
export function resumeJavaScriptFunctionBindAllocation(heap,state,identity){
 const error=readJavaScriptFunctionBindStateError(heap,state);if(error!==''||state.phase!=='allocate')return {$:'functionBindInvalid',heap,reason:error||'bound Function identity resumes only its actual allocation'};
 const request=stepJavaScriptFunctionBind(heap,state);if(request.$!=='functionBindAllocate')return request;
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||heap.symbols.includes(identity))return {$:'functionBindInvalid',heap,reason:'bound Function allocation requires a fresh external global identity'};
 const object={$:'boundFunctionObject',identity,prototype:state.prototype,extensible:true,callable:true,properties:[],boundTarget:state.target,boundThis:state.boundThis,boundArguments:state.arguments.slice()};let at=0;while(at<heap.objects.length&&(heap.objects[at]?.identity??identity)<identity)at++;const next={...heap,objects:heap.objects.slice(0,at).concat([object],heap.objects.slice(at))},invalid=readJavaScriptOrdinaryHeapError(next);
 return invalid!==''?{$:'functionBindInvalid',heap,reason:invalid}:{$:'functionBindContinue',heap:next,state:{...state,phase:'length own',receiver:{$:'object',identity,callable:true}}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptFunctionBindState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptFunctionBindTransition} */
export function resumeJavaScriptFunctionBindCall(heap,state,completion){const error=readJavaScriptFunctionBindStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.call===undefined)return {$:'functionBindInvalid',heap,reason:error||'binding resumes only its actual selected metadata getter'};return completion.$==='throwCompletion'?{$:'functionBindThrow',heap,value:completion.value}:retainJavaScriptFunctionBindMetadata(heap,state,completion.value);}
