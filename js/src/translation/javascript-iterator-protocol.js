// Synchronous GetIterator, IteratorStepValue and IteratorClose continuations.
// Actual getters/calls run in the caller's shared machine, never host fallbacks.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
/** @typedef {{$:'synchronousIteratorRecord',iterator:JavaScriptSemanticValue,nextMethod:JavaScriptSemanticValue,done:boolean}} JavaScriptSynchronousIteratorRecord */
/** @typedef {{$:'iteratorCompletion',kind:string,value:JavaScriptSemanticValue,target:number}} JavaScriptIteratorCompletion */
/** @typedef {{$:'iteratorSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptIteratorSelectedCall */
/** @typedef {{$:'iteratorProtocol',operation:string,phase:string,input:JavaScriptSemanticValue,key:JavaScriptSemanticValue,record:JavaScriptSynchronousIteratorRecord,result:JavaScriptSemanticValue,completion:JavaScriptIteratorCompletion,call?:JavaScriptIteratorSelectedCall}} JavaScriptIteratorProtocolState */
/** @typedef {{$:'iteratorProtocolContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptIteratorProtocolState} |
 * {$:'iteratorProtocolCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptIteratorProtocolState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'iteratorProtocolPrimitiveGet',heap:JavaScriptOrdinaryHeap,state:JavaScriptIteratorProtocolState,receiver:JavaScriptSemanticValue,key:JavaScriptSemanticValue} |
 * {$:'iteratorProtocolAcquired',heap:JavaScriptOrdinaryHeap,record:JavaScriptSynchronousIteratorRecord} |
 * {$:'iteratorProtocolStep',heap:JavaScriptOrdinaryHeap,record:JavaScriptSynchronousIteratorRecord,done:boolean,value:JavaScriptSemanticValue} |
 * {$:'iteratorProtocolClosed',heap:JavaScriptOrdinaryHeap,record:JavaScriptSynchronousIteratorRecord,completion:JavaScriptIteratorCompletion} |
 * {$:'iteratorProtocolThrow',heap:JavaScriptOrdinaryHeap,state:JavaScriptIteratorProtocolState,value:JavaScriptSemanticValue} |
 * {$:'iteratorProtocolError',heap:JavaScriptOrdinaryHeap,state:JavaScriptIteratorProtocolState,errorClass:string,reason:string} |
 * {$:'iteratorProtocolInvalid',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptIteratorProtocolTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSynchronousIteratorRecord} record @returns {string} */
export function readJavaScriptSynchronousIteratorRecordError(heap,record){
 const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
 if(record.$!=='synchronousIteratorRecord'||record.iterator.$!=='object'||typeof record.done!=='boolean')return 'an iterator record retains its actual Object and done slot';
 for(const value of [record.iterator,record.nextMethod]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @returns {string} */
export function readJavaScriptIteratorProtocolStateError(heap,state){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return invalid;
 const phases=state.operation==='acquire'?['method','method getter','iterator call','next','next getter']:state.operation==='step'?['next call','done','done getter','value','value getter']:state.operation==='close'?['return','return getter','return call']:[];
 if(state.$!=='iteratorProtocol'||!phases.includes(state.phase)||typeof state.record.done!=='boolean')return 'iterator work retains its actual operation and continuation phase';
 for(const value of [state.input,state.key,state.record.iterator,state.record.nextMethod,state.result,state.completion.value]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.operation==='acquire'&&state.key.$!=='symbol')return 'iterator acquisition retains its actual well-known iterator Symbol';
 if(state.operation!=='acquire'||['next','next getter'].includes(state.phase)){const error=readJavaScriptSynchronousIteratorRecordError(heap,state.record);if(error!=='')return error;}
 if(!['normal','throw','return','break','continue'].includes(state.completion.kind)||!Number.isSafeInteger(state.completion.target)||Object.is(state.completion.target,-0)||state.completion.target< -1||state.completion.target>=65536||!['break','continue'].includes(state.completion.kind)&&state.completion.target!==-1)return 'iterator cleanup retains its complete actual abrupt completion and control target';
 if(state.call!==undefined){
  if(!state.phase.endsWith('getter')&&!state.phase.endsWith('call')||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length!==0)return 'iterator callbacks retain their actual selected zero-argument getter or method';
  if(state.phase==='next call'&&(state.call.callee.$!=='object'||state.record.nextMethod.$!=='object'||state.call.callee.identity!==state.record.nextMethod.identity||state.call.receiver.$!=='object'||state.record.iterator.$!=='object'||state.call.receiver.identity!==state.record.iterator.identity))return 'iterator advancement retains the cached next method and actual iterator receiver';
  const receiver=state.phase==='method getter'||state.phase==='iterator call'?state.input:state.phase==='done getter'||state.phase==='value getter'?state.result:state.record.iterator;
  if(!testJavaScriptSemanticSameValue(state.call.receiver,receiver))return 'iterator callbacks retain the original property or method receiver';
  for(const value of [state.call.callee,state.call.receiver]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 }else if(state.phase.endsWith('getter')||state.phase==='iterator call'||state.phase==='return call')return 'a pending iterator getter or method retains its selected call';
 return '';
}
/** @param {string} operation @param {string} phase @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} key @param {JavaScriptSynchronousIteratorRecord} record @param {JavaScriptIteratorCompletion} completion @returns {JavaScriptIteratorProtocolState} */
export function createJavaScriptIteratorProtocolState(operation,phase,input,key,record,completion){return {$:'iteratorProtocol',operation,phase,input,key,record:{...record},result:{$:'undefined'},completion:{...completion}};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptIteratorProtocolTransition} */
export function beginJavaScriptSynchronousIterator(heap,input,iteratorKey){
 const state=createJavaScriptIteratorProtocolState('acquire','method',input,iteratorKey,{$:'synchronousIteratorRecord',iterator:{$:'undefined'},nextMethod:{$:'undefined'},done:false},{$:'iteratorCompletion',kind:'normal',value:{$:'undefined'},target:-1});
 const error=readJavaScriptIteratorProtocolStateError(heap,state);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};
 return input.$==='null'||input.$==='undefined'?failJavaScriptIteratorProtocol(heap,state,'value is not iterable'):{$:'iteratorProtocolContinue',heap,state};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSynchronousIteratorRecord} record @returns {JavaScriptIteratorProtocolTransition} */
export function beginJavaScriptSynchronousIteratorStep(heap,record){
 const error=readJavaScriptSynchronousIteratorRecordError(heap,record);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};
 const state=createJavaScriptIteratorProtocolState('step','next call',record.iterator,{$:'undefined'},record,{$:'iteratorCompletion',kind:'normal',value:{$:'undefined'},target:-1});
 return record.done?{$:'iteratorProtocolStep',heap,record:{...record},done:true,value:{$:'undefined'}}:{$:'iteratorProtocolContinue',heap,state};
}
/** The caller decides whether an unfinished record needs closing. This exact
 * close operation acquires return even if the supplied record was marked done.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSynchronousIteratorRecord} record @param {JavaScriptIteratorCompletion} completion @returns {JavaScriptIteratorProtocolTransition} */
export function beginJavaScriptSynchronousIteratorClose(heap,record,completion){
 const state=createJavaScriptIteratorProtocolState('close','return',record.iterator,{$:'undefined'},record,completion),error=readJavaScriptIteratorProtocolStateError(heap,state);return error===''?{$:'iteratorProtocolContinue',heap,state}:{$:'iteratorProtocolInvalid',heap,reason:error};
}
/** TypeError allocation is requested only after original-throw precedence.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {string} reason @returns {JavaScriptIteratorProtocolTransition} */
export function failJavaScriptIteratorProtocol(heap,state,reason){
 if(state.operation==='close'&&state.completion.kind==='throw')return {$:'iteratorProtocolClosed',heap,record:state.record,completion:state.completion};
 return {$:'iteratorProtocolError',heap,state:state.operation==='step'?{...state,record:{...state.record,done:true}}:state,errorClass:'TypeError',reason};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {string} phase @returns {JavaScriptIteratorProtocolTransition} */
export function requestJavaScriptIteratorProtocolCall(heap,state,callee,receiver,phase){
 if(callee.$!=='object'||!callee.callable)return failJavaScriptIteratorProtocol(heap,state,'iterator operation is not callable');
 const call={$:'iteratorSelectedCall',callee,receiver,arguments:[]};return {$:'iteratorProtocolCall',heap,state:{...state,phase,call},callee,receiver,arguments:[]};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @returns {JavaScriptIteratorProtocolTransition} */
export function stepJavaScriptIteratorProtocol(heap,state){
 const error=readJavaScriptIteratorProtocolStateError(heap,state);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'iteratorProtocolCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='next call')return requestJavaScriptIteratorProtocolCall(heap,state,state.record.nextMethod,state.record.iterator,'next call');
 const receiver=state.phase==='method'?state.input:state.phase==='next'||state.phase==='return'?state.record.iterator:state.result,key=state.phase==='method'?state.key:{$:'text',units:Array.from(state.phase,ch=>ch.charCodeAt(0))};
 if(receiver.$!=='object')return {$:'iteratorProtocolPrimitiveGet',heap,state,receiver,key};
 const get=planJavaScriptOrdinaryGet(heap,receiver.identity,key,receiver);if(get.error!=='')return {$:'iteratorProtocolInvalid',heap,reason:get.error};
 if(get.action==='call')return requestJavaScriptIteratorProtocolCall(heap,state,get.callee,get.receiver,state.phase+' getter');
 return get.action==='value'?continueJavaScriptIteratorProtocolValue(heap,state,get.value):{$:'iteratorProtocolInvalid',heap,reason:'iterator Get retains its actual value or getter'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptIteratorProtocolTransition} */
export function continueJavaScriptIteratorProtocolValue(heap,state,value){
 const {call:previous,...base}=state,phase=state.phase.endsWith(' getter')?state.phase.slice(0,-7):state.phase;
 if(phase==='method')return requestJavaScriptIteratorProtocolCall(heap,base,value,state.input,'iterator call');
 if(phase==='iterator call')return value.$!=='object'?failJavaScriptIteratorProtocol(heap,base,'iterator method returned a non-Object'):{$:'iteratorProtocolContinue',heap,state:{...base,phase:'next',record:{...base.record,iterator:value}}};
 if(phase==='next')return {$:'iteratorProtocolAcquired',heap,record:{...base.record,nextMethod:value}};
 if(phase==='next call')return value.$!=='object'?failJavaScriptIteratorProtocol(heap,base,'iterator next returned a non-Object'):{$:'iteratorProtocolContinue',heap,state:{...base,phase:'done',result:value}};
 if(phase==='done')return convertJavaScriptSemanticBoolean(value)?{$:'iteratorProtocolStep',heap,record:{...base.record,done:true},done:true,value:{$:'undefined'}}:{$:'iteratorProtocolContinue',heap,state:{...base,phase:'value'}};
 if(phase==='value')return {$:'iteratorProtocolStep',heap,record:base.record,done:false,value};
 if(phase==='return')return value.$==='undefined'||value.$==='null'?{$:'iteratorProtocolClosed',heap,record:base.record,completion:base.completion}:requestJavaScriptIteratorProtocolCall(heap,base,value,base.record.iterator,'return call');
 if(phase==='return call')return base.completion.kind==='throw'||value.$==='object'?{$:'iteratorProtocolClosed',heap,record:base.record,completion:base.completion}:failJavaScriptIteratorProtocol(heap,base,'iterator return returned a non-Object');
 return {$:'iteratorProtocolInvalid',heap,reason:'iterator completion retains its actual pending operation'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptIteratorProtocolTransition} */
export function resumeJavaScriptIteratorProtocolCall(heap,state,completion){
 const error=readJavaScriptIteratorProtocolStateError(heap,state);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};if(state.call===undefined)return {$:'iteratorProtocolInvalid',heap,reason:'iterator call completion requires its actual selected child'};
 return resumeJavaScriptIteratorProtocolValue(heap,state,completion);
}
/** Actual primitive Get authority resumes here; it supplies original receiver
 * semantics, not a fabricated boxed Object or a replacement iterator method.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptIteratorProtocolTransition} */
export function resumeJavaScriptIteratorPrimitiveGet(heap,state,completion){
 const error=readJavaScriptIteratorProtocolStateError(heap,state);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};if(state.phase!=='method'||state.input.$==='object'||state.input.$==='null'||state.input.$==='undefined'||state.call!==undefined)return {$:'iteratorProtocolInvalid',heap,reason:'primitive iterator Get requires its actual primitive acquisition request'};
 return resumeJavaScriptIteratorProtocolValue(heap,state,completion);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptIteratorProtocolState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptIteratorProtocolTransition} */
export function resumeJavaScriptIteratorProtocolValue(heap,state,completion){
 const error=readJavaScriptOrdinaryValueError(heap,completion.value);if(error!=='')return {$:'iteratorProtocolInvalid',heap,reason:error};
 if(completion.$==='normalCompletion')return continueJavaScriptIteratorProtocolValue(heap,state,completion.value);
 if(state.operation==='close'&&state.completion.kind==='throw')return {$:'iteratorProtocolClosed',heap,record:state.record,completion:state.completion};
 return {$:'iteratorProtocolThrow',heap,state:state.operation==='step'?{...state,record:{...state.record,done:true}}:state,value:completion.value};
}
