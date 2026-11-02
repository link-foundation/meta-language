// AddEntriesFromIterable with a fresh ordinary target. Every user Get, Call,
// ToPropertyKey and IteratorClose continuation uses the same inspectable heap.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet,insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {beginJavaScriptSynchronousIterator,beginJavaScriptSynchronousIteratorStep,beginJavaScriptSynchronousIteratorClose,stepJavaScriptIteratorProtocol,resumeJavaScriptIteratorProtocolCall,provideJavaScriptIteratorPrimitivePrototype,readJavaScriptIteratorProtocolStateError,readJavaScriptSynchronousIteratorRecordError} from './javascript-iterator-protocol.js';
import {beginJavaScriptPropertyKeyConversion,stepJavaScriptPropertyKeyConversion,resumeJavaScriptPropertyKeyConversion} from './javascript-property-key-conversion.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {testJavaScriptReflectionPropertyBudget} from './javascript-property-reflection.js';
/** @typedef {{$:'objectFromEntriesCallRecord',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptObjectFromEntriesCall */
/** @typedef {{$:'objectFromEntriesState',input:JavaScriptSemanticValue,prototype:JavaScriptSemanticValue,iteratorKey:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,target:JavaScriptSemanticValue,phase:string,entry:JavaScriptSemanticValue,keyInput:JavaScriptSemanticValue,value:JavaScriptSemanticValue,key:JavaScriptSemanticValue,record?:JavaScriptSynchronousIteratorRecord,iterator?:JavaScriptIteratorProtocolState,conversion?:JavaScriptPrimitiveConversionState,call?:JavaScriptObjectFromEntriesCall,abrupt?:JavaScriptSemanticValue,errorClass?:string,reason?:string}} JavaScriptObjectFromEntriesState */
/** @typedef {{$:'objectFromEntriesContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState} |
 * {$:'objectFromEntriesCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'objectFromEntriesAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState,prototype:JavaScriptSemanticValue} |
 * {$:'objectFromEntriesPrimitiveGet',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState,receiver:JavaScriptSemanticValue,key:JavaScriptSemanticValue} |
 * {$:'objectFromEntriesNeedError',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState,errorClass:string,reason:string} |
 * {$:'objectFromEntriesCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectFromEntriesThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectFromEntriesError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'objectFromEntriesInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectFromEntriesResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectFromEntriesState,reason:string}} JavaScriptObjectFromEntriesTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptObjectFromEntriesOperation(operation){return operation==='Object.fromEntries';}
/** @param {bigint} identity @param {bigint} realm @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptObjectFromEntriesFunction(identity,realm,prototype,iteratorKey){
 if(prototype.$!=='object'||prototype.callable||iteratorKey.$!=='symbol')return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:5,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'constant',destination:2,value:prototype},{$:'constant',destination:3,value:iteratorKey},{$:'objectFromEntries',destination:4,input:1,prototype:2,iteratorKey:3},{$:'return',source:4}]};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @returns {string} */
export function readJavaScriptObjectFromEntriesStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(state.$!=='objectFromEntriesState'||!['allocate','acquire','step','entry key','entry key getter','entry value','entry value getter','convert key','define','close','error'].includes(state.phase))return 'Object.fromEntries retains its actual source continuation phase';
 for(const value of [state.input,state.prototype,state.iteratorKey,state.exoticKey,state.target,state.entry,state.keyInput,state.value,state.key]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.prototype.$!=='object'||state.prototype.callable||state.iteratorKey.$!=='symbol'||state.exoticKey.$!=='symbol')return 'Object.fromEntries retains its actual realm Object prototype and well-known Symbols';
 if(state.phase==='allocate'){if(state.target.$!=='undefined'||state.record!==undefined||state.iterator!==undefined)return 'Object.fromEntries allocates its result before iterator acquisition';}
 else{const target=state.target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)]:undefined;if(target===undefined||target.$!=='ordinaryObject'||target.callable||!testJavaScriptSemanticSameValue(target.prototype,state.prototype))return 'Object.fromEntries retains its actual fresh ordinary result and realm prototype';}
 if(state.record!==undefined){const invalid=readJavaScriptSynchronousIteratorRecordError(heap,state.record);if(invalid!=='')return invalid;}else if(!['allocate','acquire'].includes(state.phase))return 'Object.fromEntries retains its actual acquired iterator record';
 if(state.iterator!==undefined){const invalid=readJavaScriptIteratorProtocolStateError(heap,state.iterator);if(invalid!=='')return invalid;if(!['acquire','step','close'].includes(state.phase)||state.iterator.operation!==(state.phase==='acquire'?'acquire':state.phase==='step'?'step':'close'))return 'Object.fromEntries retains only its selected iterator protocol';if(state.phase==='acquire'&&(!testJavaScriptSemanticSameValue(state.iterator.input,state.input)||!testJavaScriptSemanticSameValue(state.iterator.key,state.iteratorKey)))return 'Object.fromEntries cannot substitute its original iterable or iterator Symbol';if(state.phase!=='acquire'&&(state.record===undefined||!testJavaScriptSemanticSameValue(state.iterator.record.iterator,state.record.iterator)||!testJavaScriptSemanticSameValue(state.iterator.record.nextMethod,state.record.nextMethod)||state.iterator.record.done!==state.record.done))return 'Object.fromEntries retains the acquired iterator and cached next method';}
 if(state.phase==='acquire'&&state.record!==undefined)return 'Object.fromEntries acquisition cannot invent a completed iterator record';
 if(state.phase==='acquire'&&state.iterator===undefined)return 'Object.fromEntries acquisition retains its actual nested protocol';
 if(state.conversion!==undefined){const invalid=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(invalid!=='')return invalid;if(state.phase!=='convert key'||state.conversion.hint!=='string'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.keyInput)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Object.fromEntries retains its actual entry key string-hint conversion';}else if(state.phase==='convert key')return 'Object.fromEntries key conversion retains its actual nested state';
 if(['entry key','entry key getter','entry value','entry value getter','convert key','define'].includes(state.phase)&&state.entry.$!=='object')return 'Object.fromEntries accepts only actual Object entry values';
 if(state.phase==='define'){const invalid=readJavaScriptOrdinaryKeyError(heap,state.key);if(invalid!=='')return invalid;}
 if(state.phase==='close'){if(state.abrupt===undefined||state.record===undefined||state.record.done||state.iterator===undefined||state.iterator.completion.kind!=='throw'||!testJavaScriptSemanticSameValue(state.iterator.completion.value,state.abrupt))return 'Object.fromEntries cleanup retains its actual original throw before close';const invalid=readJavaScriptOrdinaryValueError(heap,state.abrupt);if(invalid!=='')return invalid;}else if(state.abrupt!==undefined)return 'Object.fromEntries holds an abrupt value only during actual iterator cleanup';
 if(state.phase==='error'){if(state.record===undefined||state.record.done||state.iterator!==undefined||state.call!==undefined||state.conversion!==undefined||typeof state.errorClass!=='string'||state.errorClass===''||typeof state.reason!=='string')return 'Object.fromEntries intrinsic failures allocate their actual Error before closing the unfinished iterator';}else if(state.errorClass!==undefined||state.reason!==undefined)return 'Object.fromEntries retains error-allocation details only in its actual pending phase';
 if(state.call!==undefined){const call=state.call;if(call.$!=='objectFromEntriesCallRecord'||call.callee.$!=='object'||!call.callee.callable||call.arguments.length>1||!['acquire','step','entry key getter','entry value getter','convert key','close'].includes(state.phase))return 'Object.fromEntries retains its actual selected bounded getter or method call';for(const value of [call.callee,call.receiver,...call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}if(state.phase.endsWith('getter')&&(!testJavaScriptSemanticSameValue(call.receiver,state.entry)||call.arguments.length!==0))return 'Object.fromEntries entry getters retain their actual Object receiver and empty arguments';if(state.iterator?.call!==undefined){const nested=state.iterator.call;if(!testJavaScriptSemanticSameValue(call.callee,nested.callee)||!testJavaScriptSemanticSameValue(call.receiver,nested.receiver)||call.arguments.length!==0)return 'Object.fromEntries cannot substitute its selected iterator getter or method';}}
 else if(state.phase.endsWith('getter')||state.iterator?.call!==undefined)return 'Object.fromEntries selected getter or method cannot lose its actual call';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} iteratorKey @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptObjectFromEntriesTransition} */
export function beginJavaScriptObjectFromEntries(heap,input,prototype,iteratorKey,exoticKey){
 const state={$:'objectFromEntriesState',input,prototype,iteratorKey,exoticKey,target:{$:'undefined'},phase:'allocate',entry:{$:'undefined'},keyInput:{$:'undefined'},value:{$:'undefined'},key:{$:'undefined'}},error=readJavaScriptObjectFromEntriesStateError(heap,state);if(error!=='')return {$:'objectFromEntriesInvalid',heap,reason:error};
 return input.$==='null'||input.$==='undefined'?{$:'objectFromEntriesError',heap,errorClass:'TypeError',reason:'Object.fromEntries requires an object-coercible iterable'}:{$:'objectFromEntriesContinue',heap,state};
}
/** @param {JavaScriptObjectFromEntriesState} state @returns {JavaScriptObjectFromEntriesState} */
export function clearJavaScriptObjectFromEntriesChild(state){const {call,conversion,iterator,errorClass,reason,abrupt,...base}=state;return base;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptObjectFromEntriesTransition} */
export function throwJavaScriptObjectFromEntries(heap,state,value){
 const base=clearJavaScriptObjectFromEntriesChild(state);if(state.record===undefined||state.record.done)return {$:'objectFromEntriesThrow',heap,value};
 const transition=beginJavaScriptSynchronousIteratorClose(heap,state.record,{$:'iteratorCompletion',kind:'throw',value,target:-1});return acceptJavaScriptObjectFromEntriesIterator(heap,{...base,phase:'close',abrupt:value},transition);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {string} errorClass @param {string} reason @returns {JavaScriptObjectFromEntriesTransition} */
export function failJavaScriptObjectFromEntries(heap,state,errorClass,reason){
 if(state.record===undefined||state.record.done)return {$:'objectFromEntriesError',heap,errorClass,reason};
 const next={...clearJavaScriptObjectFromEntriesChild(state),phase:'error',errorClass,reason};return {$:'objectFromEntriesNeedError',heap,state:next,errorClass,reason};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptIteratorProtocolTransition} transition @returns {JavaScriptObjectFromEntriesTransition} */
export function acceptJavaScriptObjectFromEntriesIterator(heap,state,transition){
 const base={...clearJavaScriptObjectFromEntriesChild(state),...(state.phase==='close'&&state.abrupt!==undefined?{abrupt:state.abrupt}:{})};
 switch(transition.$){
  case 'iteratorProtocolContinue':return {$:'objectFromEntriesContinue',heap:transition.heap,state:{...base,iterator:transition.state}};
  case 'iteratorProtocolCall':{const call={$:'objectFromEntriesCallRecord',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'objectFromEntriesCall',heap:transition.heap,state:{...base,...(state.abrupt===undefined?{}:{abrupt:state.abrupt}),iterator:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'iteratorProtocolPrimitiveGet':return {$:'objectFromEntriesPrimitiveGet',heap:transition.heap,state:{...base,iterator:transition.state},receiver:transition.receiver,key:transition.key};
  case 'iteratorProtocolAcquired':return {$:'objectFromEntriesContinue',heap:transition.heap,state:{...base,record:transition.record,phase:'step'}};
  case 'iteratorProtocolStep':{
   if(transition.done)return {$:'objectFromEntriesCompleted',heap:transition.heap,value:state.target};
   const next={...base,record:transition.record,phase:'entry key',entry:transition.value,keyInput:{$:'undefined'},value:{$:'undefined'},key:{$:'undefined'}};return transition.value.$==='object'?{$:'objectFromEntriesContinue',heap:transition.heap,state:next}:failJavaScriptObjectFromEntries(transition.heap,next,'TypeError','Iterator value is not an entry Object');
  }
  case 'iteratorProtocolClosed':return transition.completion.kind==='throw'?{$:'objectFromEntriesThrow',heap:transition.heap,value:transition.completion.value}:{$:'objectFromEntriesInvalid',heap:transition.heap,reason:'Object.fromEntries cleanup retains its original abrupt completion'};
  case 'iteratorProtocolThrow':return state.phase==='close'&&state.abrupt!==undefined?{$:'objectFromEntriesThrow',heap:transition.heap,value:state.abrupt}:throwJavaScriptObjectFromEntries(transition.heap,{...base,...(state.phase==='step'?{record:transition.state.record}:{})},transition.value);
  case 'iteratorProtocolError':return state.phase==='close'&&state.abrupt!==undefined?{$:'objectFromEntriesThrow',heap:transition.heap,value:state.abrupt}:failJavaScriptObjectFromEntries(transition.heap,{...base,...(state.phase==='step'?{record:transition.state.record}:{})},transition.errorClass,transition.reason);
  case 'iteratorProtocolInvalid':return {$:'objectFromEntriesInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptPropertyKeyConversionTransition} transition @returns {JavaScriptObjectFromEntriesTransition} */
export function acceptJavaScriptObjectFromEntriesKey(heap,state,transition){
 const base=clearJavaScriptObjectFromEntriesChild(state);
 switch(transition.$){
  case 'propertyKeyContinuation':return {$:'objectFromEntriesContinue',heap,state:{...base,phase:'convert key',conversion:transition.state}};
  case 'propertyKeyCall':{const call={$:'objectFromEntriesCallRecord',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'objectFromEntriesCall',heap,state:{...base,phase:'convert key',conversion:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'propertyKeyCompleted':return {$:'objectFromEntriesContinue',heap,state:{...base,phase:'define',key:transition.key}};
  case 'propertyKeyThrow':return throwJavaScriptObjectFromEntries(heap,base,transition.value);
  case 'propertyKeyIntrinsicError':return failJavaScriptObjectFromEntries(heap,base,transition.errorClass,transition.reason);
  case 'propertyKeyInvalid':return {$:'objectFromEntriesInvalid',heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptObjectFromEntriesTransition} */
export function acceptJavaScriptObjectFromEntriesEntryValue(heap,state,value){
 const base=clearJavaScriptObjectFromEntriesChild(state);
 return state.phase==='entry key'||state.phase==='entry key getter'?{$:'objectFromEntriesContinue',heap,state:{...base,phase:'entry value',keyInput:value}}:acceptJavaScriptObjectFromEntriesKey(heap,{...base,value},beginJavaScriptPropertyKeyConversion(heap,state.keyInput,state.exoticKey));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @returns {JavaScriptObjectFromEntriesTransition} */
export function stepJavaScriptObjectFromEntries(heap,state){
 const error=readJavaScriptObjectFromEntriesStateError(heap,state);if(error!=='')return {$:'objectFromEntriesInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'objectFromEntriesCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='allocate')return heap.objects.length>=4096?{$:'objectFromEntriesResourceLimit',heap,state,reason:'Object.fromEntries result exceeds the actual global Object budget'}:{$:'objectFromEntriesAllocate',heap,state,prototype:state.prototype};
 if(state.phase==='error')return {$:'objectFromEntriesNeedError',heap,state,errorClass:state.errorClass??'TypeError',reason:state.reason??''};
 if(state.iterator!==undefined)return acceptJavaScriptObjectFromEntriesIterator(heap,state,stepJavaScriptIteratorProtocol(heap,state.iterator));
 if(state.phase==='step')return state.record===undefined?{$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries retains its actual cached iterator'}:acceptJavaScriptObjectFromEntriesIterator(heap,state,beginJavaScriptSynchronousIteratorStep(heap,state.record));
 if(state.phase==='convert key'&&state.conversion!==undefined)return acceptJavaScriptObjectFromEntriesKey(heap,state,stepJavaScriptPropertyKeyConversion(heap,state.conversion));
 if(state.phase==='entry key'||state.phase==='entry value'){
  if(state.entry.$!=='object')return {$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries entry property access needs its actual Object'};
  const key={$:'text',units:[state.phase==='entry key'?48:49]},get=planJavaScriptOrdinaryGet(heap,state.entry.identity,key,state.entry);if(get.error!=='')return {$:'objectFromEntriesInvalid',heap,reason:get.error};
  if(get.action==='value')return acceptJavaScriptObjectFromEntriesEntryValue(heap,state,get.value);
  if(get.action!=='call')return {$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries entry Get requires its actual property value or selected getter'};
  const call={$:'objectFromEntriesCallRecord',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'objectFromEntriesCall',heap,state:{...state,phase:state.phase+' getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};
 }
 if(state.phase==='define'){
  if(state.target.$!=='object')return {$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries publication retains its actual fresh result'};
  if(!testJavaScriptReflectionPropertyBudget(heap,state.target,state.key))return {$:'objectFromEntriesResourceLimit',heap,state,reason:'Object.fromEntries exceeds the actual shared property budget'};
  const definition=defineJavaScriptOrdinaryProperty(heap,state.target.identity,state.key,createJavaScriptOrdinarySetDescriptor(state.value,true));
  if(definition.error!=='')return {$:'objectFromEntriesInvalid',heap:definition.heap,reason:definition.error};
  if(!definition.success)return failJavaScriptObjectFromEntries(definition.heap,state,'TypeError','Object.fromEntries could not create its own data property');
  return {$:'objectFromEntriesContinue',heap:definition.heap,state:{...state,phase:'step',entry:{$:'undefined'},keyInput:{$:'undefined'},value:{$:'undefined'},key:{$:'undefined'}}};
 }
 return {$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries requires its actual selected continuation'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {bigint} identity @returns {JavaScriptObjectFromEntriesTransition} */
export function resumeJavaScriptObjectFromEntriesAllocation(heap,state,identity){
 const request=stepJavaScriptObjectFromEntries(heap,state);if(request.$!=='objectFromEntriesAllocate')return request.$==='objectFromEntriesResourceLimit'?request:{$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries allocation response requires its actual pending fresh Object request'};
 const allocated=insertJavaScriptOrdinaryObject(heap,identity,state.prototype);if(!allocated.success)return {$:'objectFromEntriesInvalid',heap:allocated.heap,reason:allocated.error};
 const next={...state,target:{$:'object',identity,callable:false},phase:'acquire'};return acceptJavaScriptObjectFromEntriesIterator(allocated.heap,next,beginJavaScriptSynchronousIterator(allocated.heap,state.input,state.iteratorKey));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptObjectFromEntriesTransition} */
export function resumeJavaScriptObjectFromEntriesCall(heap,state,completion){
 const error=readJavaScriptObjectFromEntriesStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.call===undefined)return {$:'objectFromEntriesInvalid',heap,reason:error||'Object.fromEntries resumes only its selected original child'};
 if(state.iterator!==undefined)return acceptJavaScriptObjectFromEntriesIterator(heap,state,resumeJavaScriptIteratorProtocolCall(heap,state.iterator,completion));
 if(state.conversion!==undefined)return acceptJavaScriptObjectFromEntriesKey(heap,state,resumeJavaScriptPropertyKeyConversion(heap,state.conversion,completion));
 if(completion.$==='throwCompletion')return throwJavaScriptObjectFromEntries(heap,state,completion.value);
 return state.phase==='entry key getter'||state.phase==='entry value getter'?acceptJavaScriptObjectFromEntriesEntryValue(heap,state,completion.value):{$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries callback retains its actual entry Get or nested conversion phase'};
}
/** Only authenticated realm metadata supplies the primitive iterable prototype.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptSemanticValue} prototype @returns {JavaScriptObjectFromEntriesTransition} */
export function provideJavaScriptObjectFromEntriesPrimitivePrototype(heap,state,prototype){
 const request=stepJavaScriptObjectFromEntries(heap,state);return request.$!=='objectFromEntriesPrimitiveGet'||state.iterator===undefined?{$:'objectFromEntriesInvalid',heap,reason:'Object.fromEntries primitive prototype retains its actual acquisition request'}:acceptJavaScriptObjectFromEntriesIterator(heap,state,provideJavaScriptIteratorPrimitivePrototype(heap,state.iterator,prototype));
}
/** Error identity is materialized by the actual source realm before cleanup,
 * without unwinding/forgetting the enclosing Object.fromEntries continuation.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectFromEntriesState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptObjectFromEntriesTransition} */
export function resumeJavaScriptObjectFromEntriesError(heap,state,value){
 const error=readJavaScriptObjectFromEntriesStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,value);if(error!==''||state.phase!=='error'||value.$!=='object')return {$:'objectFromEntriesInvalid',heap,reason:error||'Object.fromEntries resumes only its actual realm Error allocation'};
 return throwJavaScriptObjectFromEntries(heap,state,value);
}
