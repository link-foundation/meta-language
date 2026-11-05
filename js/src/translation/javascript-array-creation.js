// Array.of / Array.from share actual Construct authority, iterator records,
// scalar protocols and ordinary/Array property definitions. No native fallback.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {beginJavaScriptSynchronousIteratorStep,beginJavaScriptSynchronousIteratorClose,createJavaScriptIteratorProtocolState,requestJavaScriptIteratorProtocolCall,stepJavaScriptIteratorProtocol,resumeJavaScriptIteratorProtocolCall,readJavaScriptIteratorProtocolStateError,readJavaScriptSynchronousIteratorRecordError} from './javascript-iterator-protocol.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptPrimitiveWrapper,resumeJavaScriptPrimitiveWrapper,readJavaScriptPrimitiveWrapperStateError} from './javascript-primitive-wrappers.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
import {readJavaScriptArrayLikeLength} from './javascript-array-index-rules.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
/** @typedef {{$:'arrayCreationCallRecord',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArrayCreationCall */
/** @typedef {{$:'arrayCreationArrayFallback',prototype:JavaScriptSemanticValue,length:number}} JavaScriptArrayCreationFallback */
/** @typedef {{$:'arrayCreationState',operation:string,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],arrayPrototype:JavaScriptSemanticValue,iteratorKey:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,input:JavaScriptSemanticValue,callback:JavaScriptSemanticValue,thisArgument:JavaScriptSemanticValue,mapping:boolean,iterable:boolean,phase:string,index:number,length:number,method:JavaScriptSemanticValue,arrayLike:JavaScriptSemanticValue,target:JavaScriptSemanticValue,value:JavaScriptSemanticValue,lengthInput:JavaScriptSemanticValue,primitivePrototype?:JavaScriptSemanticValue,record?:JavaScriptSynchronousIteratorRecord,iterator?:JavaScriptIteratorProtocolState,conversion?:JavaScriptPrimitiveConversionState,converted?:JavaScriptSemanticValue,wrapper?:JavaScriptPrimitiveWrapperState,assignment?:JavaScriptArrayAssignmentState,call?:JavaScriptArrayCreationCall,abrupt?:JavaScriptSemanticValue,errorClass?:string,reason?:string}} JavaScriptArrayCreationState */
/** @typedef {{$:'arrayCreationContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState} |
 * {$:'arrayCreationConstruct',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fallback:JavaScriptArrayCreationFallback} |
 * {$:'arrayCreationCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayCreationAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,prototype:JavaScriptSemanticValue} |
 * {$:'arrayCreationPrimitiveGet',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,receiver:JavaScriptSemanticValue,key:JavaScriptSemanticValue} |
 * {$:'arrayCreationNeedError',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,errorClass:string,reason:string} |
 * {$:'arrayCreationCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayCreationThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayCreationError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayCreationInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayCreationResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayCreationState,reason:string}} JavaScriptArrayCreationTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayCreationOperation(operation){return operation==='Array.of'||operation==='Array.from';}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @param {JavaScriptSemanticValue} prototype @param {JavaScriptSemanticValue} iteratorKey @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArrayCreationFunction(identity,realm,operation,prototype,iteratorKey){
 if(!testJavaScriptArrayCreationOperation(operation)||prototype.$!=='object'||prototype.callable||iteratorKey.$!=='symbol')return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:4,captures:[],instructions:[{$:'constant',destination:1,value:prototype},{$:'constant',destination:2,value:iteratorKey},{$:'arrayCreation',destination:3,operation,prototype:1,iteratorKey:2},{$:'return',source:3}]};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @returns {string} */
export function readJavaScriptArrayCreationStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(state.$!=='arrayCreationState'||!testJavaScriptArrayCreationOperation(state.operation)||state.arguments.length>1048576||typeof state.mapping!=='boolean'||typeof state.iterable!=='boolean'||!['method','method getter','wrapper','length','length getter','length conversion','construct','iterator acquisition','iterate','select','element getter','map','define','length set','length setter','array length','close','error'].includes(state.phase))return 'Array creation retains its actual operation arguments and continuation phase';
 if(!Number.isSafeInteger(state.index)||state.index<0||state.index>9007199254740991||Object.is(state.index,-0)||!Number.isSafeInteger(state.length)||state.length<0||state.length>9007199254740991||Object.is(state.length,-0))return 'Array creation retains exact ToLength and bounded safe index coordinates';
 for(const value of [state.receiver,state.arrayPrototype,state.iteratorKey,state.exoticKey,state.input,state.callback,state.thisArgument,state.method,state.arrayLike,state.target,state.value,state.lengthInput,...state.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.arrayPrototype.$!=='object'||state.arrayPrototype.callable||state.iteratorKey.$!=='symbol'||state.exoticKey.$!=='symbol')return 'Array creation retains actual realm Array and well-known Symbol authority';
 if(state.operation==='Array.of'&&(state.mapping||state.iterable||state.length!==state.arguments.length||state.input.$!=='undefined'||state.callback.$!=='undefined'||state.thisArgument.$!=='undefined'||state.arrayLike.$!=='undefined'||state.record!==undefined||state.index>state.length))return 'Array.of retains its actual complete immutable argument List';
 if(state.operation==='Array.from'&&(!testJavaScriptSemanticSameValue(state.input,state.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(state.callback,state.arguments[1]??{$:'undefined'})||!testJavaScriptSemanticSameValue(state.thisArgument,state.arguments[2]??{$:'undefined'})||state.mapping!==(state.callback.$!=='undefined')||state.mapping&&(state.callback.$!=='object'||!state.callback.callable)))return 'Array.from retains its actual first three arguments and callable mapper';
 if(state.iterable&&(state.operation!=='Array.from'||state.method.$!=='object'||!state.method.callable))return 'iterable Array.from retains the actual cached iterator method';
 if(state.primitivePrototype!==undefined){const invalid=readJavaScriptOrdinaryValueError(heap,state.primitivePrototype);if(invalid!==''||state.input.$==='object'||state.primitivePrototype.$!=='object')return invalid||'primitive Array.from lookup retains its actual realm prototype';}
 if(state.wrapper!==undefined){const invalid=readJavaScriptPrimitiveWrapperStateError(heap,state.wrapper);if(invalid!==''||state.phase!=='wrapper'||!testJavaScriptSemanticSameValue(state.wrapper.input,state.input)||state.primitivePrototype===undefined||!testJavaScriptSemanticSameValue(state.wrapper.prototype,state.primitivePrototype))return invalid||'Array.from ToObject retains its original primitive and actual prototype';}else if(state.phase==='wrapper')return 'Array.from boxing retains its actual allocation continuation';
 if(state.operation==='Array.of'&&!['construct','select','define','length set','length setter','array length'].includes(state.phase))return 'Array.of uses only its actual Construct element definition and final Set phases';
 if(state.iterable&&['iterate','map','define','close','error'].includes(state.phase)&&(state.record===undefined||state.record.done))return 'live iterable Array.from work retains its actual unfinished record';
 if(state.iterable&&['select','element getter','wrapper','length','length getter','length conversion'].includes(state.phase))return 'iterable Array.from cannot substitute array-like property traversal';
 if(['element getter','map','define'].includes(state.phase)&&state.index>=(state.iterable?9007199254740991:state.length))return 'Array creation element work retains its actual remaining index';
 if(state.arrayLike.$!=='undefined'&&state.arrayLike.$!=='object'||state.target.$!=='undefined'&&state.target.$!=='object')return 'Array creation retains actual array-like and constructed Object identities';
 if(['iterator acquisition','iterate','select','element getter','map','define','length set','length setter','array length','close','error'].includes(state.phase)&&state.target.$!=='object')return 'Array creation element work follows its actual completed Construct or ArrayCreate';
 if(['method','method getter','wrapper','length','length getter','length conversion','construct','iterator acquisition'].includes(state.phase)&&state.record!==undefined)return 'Array.from cannot own an iterator record before actual construction and completed acquisition';
 if(['method','method getter','wrapper','length','length getter','length conversion','construct'].includes(state.phase)&&state.target.$!=='undefined')return 'Array creation cannot publish a result before its actual Construct completion';
 if(['length set','length setter','array length'].includes(state.phase)&&(state.iterable?state.record===undefined||!state.record.done:state.index!==state.length))return 'Array creation final Set follows actual exhausted iteration or completed array-like indices';
 if(state.record!==undefined){const invalid=readJavaScriptSynchronousIteratorRecordError(heap,state.record);if(invalid!==''||!state.iterable)return invalid||'only iterable Array.from owns an iterator record';}
 if(state.iterator!==undefined){const invalid=readJavaScriptIteratorProtocolStateError(heap,state.iterator);if(invalid!==''||!['iterator acquisition','iterate','close'].includes(state.phase)||state.iterator.operation!==(state.phase==='iterator acquisition'?'acquire':state.phase==='iterate'?'step':'close'))return invalid||'Array.from retains only its actual selected iterator protocol';if(state.phase==='iterator acquisition'&&state.iterator.phase==='iterator call'&&state.iterator.call!==undefined&&!testJavaScriptSemanticSameValue(state.iterator.call.callee,state.method))return 'Array.from calls its actual previously cached iterator method';if(state.phase==='iterator acquisition'&&(!testJavaScriptSemanticSameValue(state.iterator.input,state.input)||!testJavaScriptSemanticSameValue(state.iterator.key,state.iteratorKey)))return 'Array.from iterator acquisition retains the original input and cached method';if(state.phase!=='iterator acquisition'&&(state.record===undefined||!testJavaScriptSemanticSameValue(state.iterator.record.iterator,state.record.iterator)||!testJavaScriptSemanticSameValue(state.iterator.record.nextMethod,state.record.nextMethod)||state.iterator.record.done!==state.record.done))return 'Array.from retains its actual cached next method and done flag';}
 if(state.conversion!==undefined){const invalid=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(invalid!==''||state.phase!=='length conversion'||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.lengthInput)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return invalid||'Array.from length conversion retains its original operand number hint and Symbol';}
 if(state.converted!==undefined&&(state.phase!=='length conversion'||state.converted.$==='object'||readJavaScriptOrdinaryValueError(heap,state.converted)!==''))return 'Array.from resource suspension retains its completed primitive length';
 if(state.phase==='length conversion'&&state.conversion===undefined&&state.converted===undefined)return 'Array.from length conversion cannot lose its actual continuation';
 if(state.assignment!==undefined){const invalid=readJavaScriptArrayAssignmentStateError(heap,state.assignment);if(invalid!==''||state.phase!=='array length'||state.target.$!=='object'||state.assignment.definition.identity!==state.target.identity||!testJavaScriptSemanticSameValue(state.assignment.definition.key,{$:'text',units:[108,101,110,103,116,104]})||!testJavaScriptSemanticSameValue(state.assignment.definition.descriptor.value,{$:'number',value:state.iterable?state.index:state.length}))return invalid||'Array creation final length Set retains its actual Array definition';}else if(state.phase==='array length')return 'Array creation Array length Set retains its actual nested definition';
 if(state.phase==='close'){if(state.abrupt===undefined||state.record===undefined||state.record.done||state.iterator===undefined||state.iterator.operation!=='close'||state.iterator.completion.kind!=='throw'||!testJavaScriptSemanticSameValue(state.iterator.completion.value,state.abrupt))return 'Array.from cleanup retains its original throw and unfinished iterator';}else if(state.abrupt!==undefined)return 'Array creation holds an abrupt value only while closing its actual iterator';
 if(state.phase==='error'){if(state.record===undefined||state.record.done||state.call!==undefined||state.iterator!==undefined||typeof state.errorClass!=='string'||state.errorClass===''||typeof state.reason!=='string')return 'Array.from allocates the actual intrinsic Error before closing the unfinished iterator';}else if(state.errorClass!==undefined||state.reason!==undefined)return 'Array creation retains intrinsic Error details only in its actual error phase';
 if(state.call!==undefined){const call=state.call;if(call.$!=='arrayCreationCallRecord'||call.callee.$!=='object'||!call.callee.callable||call.arguments.length>2||!['method getter','length getter','length conversion','iterator acquisition','iterate','element getter','map','length setter','array length','close'].includes(state.phase))return 'Array creation retains an actual selected bounded callback';for(const value of [call.callee,call.receiver,...call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
  if(state.phase==='length conversion'&&(state.conversion===undefined||!state.conversion.phase.endsWith('Getter')&&!state.conversion.phase.endsWith('Call'))||state.phase==='array length'&&state.assignment?.$!=='arrayAssignmentNumber')return 'Array creation nested calls retain their actual conversion callback state';
  if(state.phase==='map'&&(!testJavaScriptSemanticSameValue(call.callee,state.callback)||!testJavaScriptSemanticSameValue(call.receiver,state.thisArgument)||call.arguments.length!==2||!testJavaScriptSemanticSameValue(call.arguments[0]??{$:'undefined'},state.value)||!testJavaScriptSemanticSameValue(call.arguments[1]??{$:'undefined'},{$:'number',value:state.index})))return 'Array.from mapping retains actual value index thisArgument and callback';
  if(state.phase==='method getter'&&(!testJavaScriptSemanticSameValue(call.receiver,state.input)||call.arguments.length!==0)||['length getter','element getter'].includes(state.phase)&&(!testJavaScriptSemanticSameValue(call.receiver,state.arrayLike)||call.arguments.length!==0)||state.phase==='length setter'&&(!testJavaScriptSemanticSameValue(call.receiver,state.target)||call.arguments.length!==1||!testJavaScriptSemanticSameValue(call.arguments[0]??{$:'undefined'},{$:'number',value:state.iterable?state.index:state.length})))return 'Array creation property callbacks retain their actual receiver and arguments';
  if(state.iterator?.call!==undefined){const nested=state.iterator.call;if(!testJavaScriptSemanticSameValue(call.callee,nested.callee)||!testJavaScriptSemanticSameValue(call.receiver,nested.receiver)||call.arguments.length!==nested.arguments.length)return 'Array.from retains its selected iterator callback';}
 }else if(['method getter','length getter','element getter','map','length setter'].includes(state.phase)||state.iterator?.call!==undefined)return 'Array creation cannot lose its selected callback';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @param {JavaScriptSemanticValue} arrayPrototype @param {JavaScriptSemanticValue} iteratorKey @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArrayCreationTransition} */
export function beginJavaScriptArrayCreation(heap,operation,receiver,arguments_,arrayPrototype,iteratorKey,exoticKey){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return {$:'arrayCreationInvalid',heap,reason:invalid};if(arguments_.length>1048576)return {$:'arrayCreationInvalid',heap,reason:'Array creation argument Lists retain their selected 1048576-value model profile'};for(const value of [receiver,arrayPrototype,iteratorKey,exoticKey,...arguments_]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return {$:'arrayCreationInvalid',heap,reason:error};}
 if(!testJavaScriptArrayCreationOperation(operation))return {$:'arrayCreationInvalid',heap,reason:'Array creation requires its actual protected operation'};
 const input=operation==='Array.from'?arguments_[0]??{$:'undefined'}:{$:'undefined'},callback=operation==='Array.from'?arguments_[1]??{$:'undefined'}:{$:'undefined'},thisArgument=operation==='Array.from'?arguments_[2]??{$:'undefined'}:{$:'undefined'},mapping=callback.$!=='undefined';
 if(mapping&&(callback.$!=='object'||!callback.callable))return {$:'arrayCreationError',heap,errorClass:'TypeError',reason:'Array.from mapper must be callable'};
 if(operation==='Array.from'&&(input.$==='undefined'||input.$==='null'))return {$:'arrayCreationError',heap,errorClass:'TypeError',reason:'Array.from requires an object-coercible input'};
 const state={$:'arrayCreationState',operation,receiver,arguments:arguments_,arrayPrototype,iteratorKey,exoticKey,input,callback,thisArgument,mapping,iterable:false,phase:operation==='Array.of'?'construct':'method',index:0,length:operation==='Array.of'?arguments_.length:0,method:{$:'undefined'},arrayLike:{$:'undefined'},target:{$:'undefined'},value:{$:'undefined'},lengthInput:{$:'undefined'}},error=readJavaScriptArrayCreationStateError(heap,state);return error!==''?{$:'arrayCreationInvalid',heap,reason:error}:{$:'arrayCreationContinue',heap,state};
}
/** @param {JavaScriptArrayCreationState} state @returns {JavaScriptArrayCreationState} */
export function clearJavaScriptArrayCreationChild(state){const {call,iterator,conversion,converted,wrapper,assignment,abrupt,errorClass,reason,...base}=state;return base;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCreationTransition} */
export function throwJavaScriptArrayCreation(heap,state,value){
 const base=clearJavaScriptArrayCreationChild(state);if(state.record===undefined||state.record.done)return {$:'arrayCreationThrow',heap,value};return acceptJavaScriptArrayCreationIterator({...base,phase:'close',abrupt:value},beginJavaScriptSynchronousIteratorClose(heap,state.record,{$:'iteratorCompletion',kind:'throw',value,target:-1}));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {string} errorClass @param {string} reason @returns {JavaScriptArrayCreationTransition} */
export function failJavaScriptArrayCreation(heap,state,errorClass,reason){
 return state.record===undefined||state.record.done?{$:'arrayCreationError',heap,errorClass,reason}:{$:'arrayCreationNeedError',heap,state:{...clearJavaScriptArrayCreationChild(state),phase:'error',errorClass,reason},errorClass,reason};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArrayCreationTransition} */
export function requestJavaScriptArrayCreationCall(heap,state,callee,receiver,arguments_){const call={$:'arrayCreationCallRecord',callee,receiver,arguments:arguments_};return {$:'arrayCreationCall',heap,state:{...state,call},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptArrayCreationState} state @param {JavaScriptIteratorProtocolTransition} transition @returns {JavaScriptArrayCreationTransition} */
export function acceptJavaScriptArrayCreationIterator(state,transition){
 const base={...clearJavaScriptArrayCreationChild(state),...(state.phase==='close'&&state.abrupt!==undefined?{abrupt:state.abrupt}:{})};
 switch(transition.$){
  case 'iteratorProtocolContinue':return {$:'arrayCreationContinue',heap:transition.heap,state:{...base,iterator:transition.state}};
  case 'iteratorProtocolCall':return requestJavaScriptArrayCreationCall(transition.heap,{...base,iterator:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'iteratorProtocolPrimitiveGet':return {$:'arrayCreationInvalid',heap:transition.heap,reason:'cached Array.from iterator acquisition cannot repeat its original primitive method Get'};
  case 'iteratorProtocolAcquired':return {$:'arrayCreationContinue',heap:transition.heap,state:{...base,record:transition.record,phase:'iterate'}};
  case 'iteratorProtocolStep':return transition.done?{$:'arrayCreationContinue',heap:transition.heap,state:{...base,record:transition.record,phase:'length set'}}:selectJavaScriptArrayCreationValue(transition.heap,{...base,record:transition.record},transition.value);
  case 'iteratorProtocolClosed':return transition.completion.kind==='throw'?{$:'arrayCreationThrow',heap:transition.heap,value:transition.completion.value}:{$:'arrayCreationInvalid',heap:transition.heap,reason:'Array.from cleanup retains its actual original throw'};
  case 'iteratorProtocolThrow':return state.phase==='close'&&state.abrupt!==undefined?{$:'arrayCreationThrow',heap:transition.heap,value:state.abrupt}:throwJavaScriptArrayCreation(transition.heap,{...base,...(state.phase==='iterate'?{record:transition.state.record}:{})},transition.value);
  case 'iteratorProtocolError':return state.phase==='close'&&state.abrupt!==undefined?{$:'arrayCreationThrow',heap:transition.heap,value:state.abrupt}:failJavaScriptArrayCreation(transition.heap,{...base,...(state.phase==='iterate'?{record:transition.state.record}:{})},transition.errorClass,transition.reason);
  case 'iteratorProtocolInvalid':return {$:'arrayCreationInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCreationTransition} */
export function selectJavaScriptArrayCreationValue(heap,state,value){const base=clearJavaScriptArrayCreationChild(state),next={...base,value,phase:'define'};return state.mapping?requestJavaScriptArrayCreationCall(heap,{...next,phase:'map'},state.callback,state.thisArgument,[value,{$:'number',value:state.index}]):{$:'arrayCreationContinue',heap,state:next};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCreationTransition} */
export function acceptJavaScriptArrayCreationGet(heap,state,value){
 const base=clearJavaScriptArrayCreationChild(state),phase=state.phase.endsWith(' getter')?state.phase.slice(0,-7):state.phase;
 if(phase==='method'){
  if(value.$==='undefined'||value.$==='null')return state.input.$==='object'?{$:'arrayCreationContinue',heap,state:{...base,arrayLike:state.input,phase:'length'}}:state.primitivePrototype===undefined?{$:'arrayCreationInvalid',heap,reason:'Array.from primitive ToObject retains its selected realm prototype'}:acceptJavaScriptArrayCreationWrapper({...base,phase:'wrapper'},beginJavaScriptPrimitiveWrapper(heap,state.input,state.primitivePrototype));
  return value.$==='object'&&value.callable?{$:'arrayCreationContinue',heap,state:{...base,iterable:true,method:value,phase:'construct'}}:failJavaScriptArrayCreation(heap,base,'TypeError','Array.from iterator method must be callable');
 }
 if(phase==='length')return acceptJavaScriptArrayCreationLength(heap,{...base,phase:'length conversion',lengthInput:value},beginJavaScriptScalarProtocol(heap,value,state.exoticKey,'coerce primitive number'));
 return phase==='element'?selectJavaScriptArrayCreationValue(heap,base,value):{$:'arrayCreationInvalid',heap,reason:'Array creation Get completion retains its actual property phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} object @param {JavaScriptSemanticValue} key @param {string} phase @param {JavaScriptSemanticValue} receiver @returns {JavaScriptArrayCreationTransition} */
export function readJavaScriptArrayCreationProperty(heap,state,object,key,phase,receiver){
 if(object.$!=='object')return {$:'arrayCreationInvalid',heap,reason:'Array creation property Get retains its actual lookup Object'};const get=planJavaScriptOrdinaryGet(heap,object.identity,key,receiver);if(get.error!=='')return {$:'arrayCreationInvalid',heap,reason:get.error};
 return get.action==='value'?acceptJavaScriptArrayCreationGet(heap,{...state,phase},get.value):get.action==='call'?requestJavaScriptArrayCreationCall(heap,{...state,phase:phase+' getter'},get.callee,get.receiver,get.arguments):{$:'arrayCreationInvalid',heap,reason:'Array creation opaque property Get requires its actual host protocol'};
}
/** @param {JavaScriptArrayCreationState} state @param {JavaScriptPrimitiveWrapperTransition} transition @returns {JavaScriptArrayCreationTransition} */
export function acceptJavaScriptArrayCreationWrapper(state,transition){
 const base=clearJavaScriptArrayCreationChild(state);switch(transition.$){
  case 'primitiveWrapperAllocate':return {$:'arrayCreationAllocate',heap:transition.heap,state:{...base,phase:'wrapper',wrapper:transition.state},prototype:transition.prototype};
  case 'primitiveWrapperCompleted':return {$:'arrayCreationContinue',heap:transition.heap,state:{...base,arrayLike:transition.value,phase:'length'}};
  case 'primitiveWrapperInvalid':return {$:'arrayCreationInvalid',heap:transition.heap,reason:transition.reason};
  case 'primitiveWrapperResourceLimit':return {$:'arrayCreationResourceLimit',heap:transition.heap,state:{...base,phase:'wrapper',wrapper:transition.state},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArrayCreationTransition} */
export function acceptJavaScriptArrayCreationLength(heap,state,transition){
 const base=clearJavaScriptArrayCreationChild(state);switch(transition.$){
  case 'scalarContinuation':return {$:'arrayCreationContinue',heap,state:{...base,phase:'length conversion',conversion:transition.state}};
  case 'scalarCall':return requestJavaScriptArrayCreationCall(heap,{...base,phase:'length conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':return transition.value.$==='number'?{$:'arrayCreationContinue',heap,state:{...base,length:readJavaScriptArrayLikeLength(transition.value.value),lengthInput:{$:'undefined'},phase:'construct'}}:{$:'arrayCreationInvalid',heap,reason:'Array.from ToLength retains its actual completed Number'};
  case 'scalarThrow':return throwJavaScriptArrayCreation(heap,base,transition.value);
  case 'scalarIntrinsicError':return failJavaScriptArrayCreation(heap,base,transition.errorClass,transition.reason);
  case 'scalarInvalid':return {$:'arrayCreationInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayCreationResourceLimit',heap,state:{...base,phase:'length conversion',converted:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptArrayCreationState} state @param {JavaScriptArrayAssignmentTransition} transition @returns {JavaScriptArrayCreationTransition} */
export function acceptJavaScriptArrayCreationLengthSet(state,transition){
 const base=clearJavaScriptArrayCreationChild(state);switch(transition.$){
  case 'arrayAssignmentContinue':return {$:'arrayCreationContinue',heap:transition.heap,state:{...base,phase:'array length',assignment:transition.state}};
  case 'arrayAssignmentCall':return requestJavaScriptArrayCreationCall(transition.heap,{...base,phase:'array length',assignment:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'arrayAssignmentCompleted':return transition.success?{$:'arrayCreationCompleted',heap:transition.heap,value:state.target}:failJavaScriptArrayCreation(transition.heap,base,'TypeError','Array creation could not set result length');
  case 'arrayAssignmentThrow':return throwJavaScriptArrayCreation(transition.heap,base,transition.value);
  case 'arrayAssignmentError':return failJavaScriptArrayCreation(transition.heap,base,transition.errorClass,transition.reason);
  case 'arrayAssignmentInvalid':return {$:'arrayCreationInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayAssignmentResourceLimit':return {$:'arrayCreationResourceLimit',heap:transition.heap,state:{...base,phase:'array length',assignment:transition.state},reason:transition.reason};
 }
}
/** @param {number} index @returns {JavaScriptSemanticValue} */
function constructJavaScriptArrayCreationIndex(index){const units=[],reversed=[];let remaining=index;do{const digit=remaining%10;reversed.push(48+digit);remaining=Math.floor(remaining/10);}while(remaining>0);for(let at=reversed.length-1;at>=0;at-=1)units.push(reversed[at]??48);return {$:'text',units};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @returns {JavaScriptArrayCreationTransition} */
export function stepJavaScriptArrayCreation(heap,state){
 const error=readJavaScriptArrayCreationStateError(heap,state);if(error!=='')return {$:'arrayCreationInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arrayCreationCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='error')return {$:'arrayCreationNeedError',heap,state,errorClass:state.errorClass??'TypeError',reason:state.reason??''};
 if(state.iterator!==undefined)return acceptJavaScriptArrayCreationIterator(state,stepJavaScriptIteratorProtocol(heap,state.iterator));
 if(state.phase==='method')return state.input.$==='object'?readJavaScriptArrayCreationProperty(heap,state,state.input,state.iteratorKey,'method',state.input):state.primitivePrototype===undefined?{$:'arrayCreationPrimitiveGet',heap,state,receiver:state.input,key:state.iteratorKey}:readJavaScriptArrayCreationProperty(heap,state,state.primitivePrototype,state.iteratorKey,'method',state.input);
 if(state.phase==='wrapper')return state.wrapper===undefined?{$:'arrayCreationInvalid',heap,reason:'Array.from ToObject retains its selected wrapper state'}:acceptJavaScriptArrayCreationWrapper(state,beginJavaScriptPrimitiveWrapper(heap,state.wrapper.input,state.wrapper.prototype));
 if(state.phase==='length')return readJavaScriptArrayCreationProperty(heap,state,state.arrayLike,{$:'text',units:[108,101,110,103,116,104]},'length',state.arrayLike);
 if(state.phase==='length conversion')return state.conversion!==undefined?acceptJavaScriptArrayCreationLength(heap,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number')):state.converted!==undefined?acceptJavaScriptArrayCreationLength(heap,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.converted)):{$:'arrayCreationInvalid',heap,reason:'Array.from retains its completed length conversion'};
 if(state.phase==='construct')return {$:'arrayCreationConstruct',heap,state,constructor:state.receiver,arguments:state.iterable?[]:[{$:'number',value:state.length}],fallback:{$:'arrayCreationArrayFallback',prototype:state.arrayPrototype,length:state.iterable?0:state.length}};
 if(state.phase==='iterate')return state.index>=9007199254740991?failJavaScriptArrayCreation(heap,state,'TypeError','Array.from result exceeds the maximum safe index'):state.record===undefined?{$:'arrayCreationInvalid',heap,reason:'Array.from iteration retains its acquired iterator record'}:acceptJavaScriptArrayCreationIterator(state,beginJavaScriptSynchronousIteratorStep(heap,state.record));
 if(state.phase==='select'){
  if(state.index>=state.length)return {$:'arrayCreationContinue',heap,state:{...state,phase:'length set'}};
  return state.operation==='Array.of'?selectJavaScriptArrayCreationValue(heap,state,state.arguments[state.index]??{$:'undefined'}):readJavaScriptArrayCreationProperty(heap,state,state.arrayLike,constructJavaScriptArrayCreationIndex(state.index),'element',state.arrayLike);
 }
 if(state.phase==='define'){
  const written=createJavaScriptIndexedDataProperty(heap,state.target,state.index,state.value);if(written.$==='indexedPropertyResourceLimit')return {$:'arrayCreationResourceLimit',heap:written.heap,state,reason:written.reason};if(written.$==='indexedPropertyError')return failJavaScriptArrayCreation(written.heap,state,'TypeError',written.reason);if(written.$==='indexedPropertyInvalid')return {$:'arrayCreationInvalid',heap:written.heap,reason:written.reason};
  return {$:'arrayCreationContinue',heap:written.heap,state:{...state,index:state.index+1,value:{$:'undefined'},phase:state.iterable?'iterate':'select'}};
 }
 if(state.phase==='length set'){
  if(state.target.$!=='object')return {$:'arrayCreationInvalid',heap,reason:'Array creation final Set needs its actual result Object'};
  const key={$:'text',units:[108,101,110,103,116,104]},value={$:'number',value:state.iterable?state.index:state.length},plan=planJavaScriptOrdinarySetWithArrayDefinition(heap,state.target.identity,key,value,state.target);
  if(plan.error!=='')return plan.error==='ordinary properties exceed their explicit ordered storage budget'?{$:'arrayCreationResourceLimit',heap:plan.heap,state,reason:plan.error}:{$:'arrayCreationInvalid',heap:plan.heap,reason:plan.error};
  if(plan.action==='array definition'&&plan.definition!==undefined)return acceptJavaScriptArrayCreationLengthSet(state,beginJavaScriptArrayAssignment(plan.heap,state.target.identity,key,plan.definition,state.exoticKey));
  if(plan.action==='call')return requestJavaScriptArrayCreationCall(plan.heap,{...state,phase:'length setter'},plan.callee,plan.receiver,plan.arguments);
  return plan.action!=='complete'?{$:'arrayCreationInvalid',heap:plan.heap,reason:'Array creation final length Set requires its actual ordinary/Array or selected setter protocol'}:plan.success?{$:'arrayCreationCompleted',heap:plan.heap,value:state.target}:failJavaScriptArrayCreation(plan.heap,state,'TypeError','Array creation could not set result length');
 }
 if(state.phase==='array length'&&state.assignment!==undefined)return acceptJavaScriptArrayCreationLengthSet(state,stepJavaScriptArrayAssignment(heap,state.assignment));
 return {$:'arrayCreationInvalid',heap,reason:'Array creation resumes only its actual selected continuation'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayCreationTransition} */
export function resumeJavaScriptArrayCreationCall(heap,state,completion){
 const error=readJavaScriptArrayCreationStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.call===undefined)return {$:'arrayCreationInvalid',heap,reason:error||'Array creation retains its actual selected callback'};
 if(state.iterator!==undefined)return acceptJavaScriptArrayCreationIterator(state,resumeJavaScriptIteratorProtocolCall(heap,state.iterator,completion));
 if(state.conversion!==undefined)return acceptJavaScriptArrayCreationLength(heap,state,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.assignment!==undefined)return acceptJavaScriptArrayCreationLengthSet(state,resumeJavaScriptArrayAssignment(heap,state.assignment,completion));
 if(completion.$==='throwCompletion')return throwJavaScriptArrayCreation(heap,state,completion.value);
 if(state.phase==='map')return {$:'arrayCreationContinue',heap,state:{...clearJavaScriptArrayCreationChild(state),phase:'define',value:completion.value}};
 if(state.phase==='length setter')return {$:'arrayCreationCompleted',heap,value:state.target};
 return ['method getter','length getter','element getter'].includes(state.phase)?acceptJavaScriptArrayCreationGet(heap,state,completion.value):{$:'arrayCreationInvalid',heap,reason:'Array creation callback completion retains its selected property or mapper phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayCreationTransition} */
export function resumeJavaScriptArrayCreationConstruction(heap,state,completion){
 const error=readJavaScriptArrayCreationStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.phase!=='construct')return {$:'arrayCreationInvalid',heap,reason:error||'Array creation retains its actual Construct request'};
 if(completion.$==='throwCompletion')return {$:'arrayCreationThrow',heap,value:completion.value};if(completion.value.$!=='object')return {$:'arrayCreationInvalid',heap,reason:'actual Construct/ArrayCreate returns its Object result'};
 const next={...state,target:completion.value,phase:state.iterable?'iterator acquisition':'select'};if(!state.iterable)return {$:'arrayCreationContinue',heap,state:next};
 const protocol=createJavaScriptIteratorProtocolState('acquire','iterator call',state.input,state.iteratorKey,{$:'synchronousIteratorRecord',iterator:{$:'undefined'},nextMethod:{$:'undefined'},done:false},{$:'iteratorCompletion',kind:'normal',value:{$:'undefined'},target:-1});return acceptJavaScriptArrayCreationIterator(next,requestJavaScriptIteratorProtocolCall(heap,protocol,state.method,state.input,'iterator call'));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {bigint} identity @returns {JavaScriptArrayCreationTransition} */
export function resumeJavaScriptArrayCreationAllocation(heap,state,identity){
 const error=readJavaScriptArrayCreationStateError(heap,state);if(error!==''||state.phase!=='wrapper'||state.wrapper===undefined)return {$:'arrayCreationInvalid',heap,reason:error||'Array.from allocation retains its actual temporary ToObject request'};
 const request=stepJavaScriptArrayCreation(heap,state);if(request.$!=='arrayCreationAllocate')return request.$==='arrayCreationResourceLimit'?request:{$:'arrayCreationInvalid',heap,reason:'Array.from Object identity response retains its actual temporary allocation request'};
 return acceptJavaScriptArrayCreationWrapper(state,resumeJavaScriptPrimitiveWrapper(heap,state.wrapper,identity));
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} prototype @returns {JavaScriptArrayCreationTransition} */
export function provideJavaScriptArrayCreationPrimitivePrototype(heap,state,prototype){
 const error=readJavaScriptArrayCreationStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,prototype);if(error!==''||state.phase!=='method'||state.input.$==='object'||state.primitivePrototype!==undefined||prototype.$!=='object')return {$:'arrayCreationInvalid',heap,reason:error||'Array.from primitive method Get retains its actual realm prototype request'};
 return stepJavaScriptArrayCreation(heap,{...state,primitivePrototype:prototype});
}
/** Actual realm Error is allocated before IteratorClose, preserving callbacks
 * and original error identity when cleanup also fails.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayCreationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayCreationTransition} */
export function resumeJavaScriptArrayCreationError(heap,state,value){
 const error=readJavaScriptArrayCreationStateError(heap,state)||readJavaScriptOrdinaryValueError(heap,value);if(error!==''||state.phase!=='error'||value.$!=='object'||value.callable)return {$:'arrayCreationInvalid',heap,reason:error||'Array.from cleanup retains its actual materialized realm Error'};return throwJavaScriptArrayCreation(heap,state,value);
}
