// Shared map/filter algorithm; custom Construct remains an explicit protocol.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor,planJavaScriptPropertyDefinition} from './javascript-property-descriptors.js';
import {planJavaScriptArrayPropertyDefinition} from './javascript-array-objects.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {readJavaScriptArrayLikeLength,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptArraySpeciesSelection,stepJavaScriptArraySpeciesSelection,resumeJavaScriptArraySpeciesGetter,readJavaScriptArraySpeciesStateError,readJavaScriptArraySpeciesDirectoryError,findJavaScriptArrayConstructorFact} from './javascript-array-species.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
/** @typedef {{$:'arrayTransformSelectedCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArrayTransformCall */
/** @typedef {{$:'arrayTransformState',operation:string,receiver:JavaScriptSemanticValue,callback:JavaScriptSemanticValue,thisArgument:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,length:number,index:number,outputIndex:number,target:JavaScriptSemanticValue,element:JavaScriptSemanticValue,value:JavaScriptSemanticValue,input:JavaScriptSemanticValue,phase:string,conversion?:JavaScriptPrimitiveConversionState,species?:JavaScriptArraySpeciesState,allocationPrototype?:JavaScriptSemanticValue,constructor?:JavaScriptSemanticValue,constructorFact?:JavaScriptArrayConstructorFact,call?:JavaScriptArrayTransformCall}} JavaScriptArrayTransformState */
/** @typedef {{$:'arrayTransformContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState} |
 * {$:'arrayTransformCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayTransformAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState,prototype:JavaScriptSemanticValue,length:number} |
 * {$:'arrayTransformConstruct',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fact:JavaScriptArrayConstructorFact} |
 * {$:'arrayTransformNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState,constructor:JavaScriptSemanticValue} |
 * {$:'arrayTransformCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayTransformThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayTransformError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayTransformInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayTransformResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayTransformState,reason:string}} JavaScriptArrayTransformTransition */
/** @typedef {{$:'indexedPropertyCreated',heap:JavaScriptOrdinaryHeap} | {$:'indexedPropertyError',heap:JavaScriptOrdinaryHeap,reason:string} | {$:'indexedPropertyInvalid',heap:JavaScriptOrdinaryHeap,reason:string} | {$:'indexedPropertyResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptIndexedPropertyCreation */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayTransformOperation(operation){return operation==='Array.map'||operation==='Array.filter';}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @returns {string} */
export function readJavaScriptArrayTransformStateError(heap,directory,state){
 const error=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(error!=='')return error;
 if(!testJavaScriptArrayTransformOperation(state.operation)||state.receiver.$!=='object'||state.exoticKey.$!=='symbol')return 'Array transformation retains its actual operation, Object receiver and conversion Symbol';
 for(const value of [state.receiver,state.callback,state.thisArgument,state.exoticKey,state.target,state.element,state.value,state.input]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0)||!Number.isSafeInteger(state.index)||state.index<0||Object.is(state.index,-0)||state.index>state.length||!Number.isSafeInteger(state.outputIndex)||state.outputIndex<0||Object.is(state.outputIndex,-0)||state.outputIndex>state.index)return 'Array transformation retains its captured length and exact input/output coordinates';
 if(!['length','length getter','length conversion','length primitive','species','allocate','construct','ready','element getter','callback','write'].includes(state.phase))return 'Array transformation retains its actual algorithm phase';
 if(state.phase==='length conversion'){if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.input)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'Array transform length conversion retains its actual operand, hint and Symbol';const invalid=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(invalid!=='')return invalid;}else if(state.conversion!==undefined)return 'only Array transform length conversion retains a scalar continuation';
 if(!['length','length getter','length conversion','length primitive'].includes(state.phase)&&(state.callback.$!=='object'||!state.callback.callable))return 'initialized Array transformation retains its actual callable callback';
 if(state.species!==undefined){const invalid=readJavaScriptArraySpeciesStateError(heap,directory,state.species);if(invalid!=='')return invalid;if(!testJavaScriptSemanticSameValue(state.species.original,state.receiver)||state.species.length!==(state.operation==='Array.map'?state.length:0))return 'Array transformation retains its original source and actual species allocation length';}
 if(['species','allocate','construct'].includes(state.phase)&&state.species===undefined)return 'Array result creation retains its actual species continuation';
 if(['ready','element getter','callback','write'].includes(state.phase)&&state.target.$!=='object')return 'Array transformation retains its genuine constructed result Object';
 if(['element getter','callback','write'].includes(state.phase)&&state.index>=state.length)return 'Array transformation work retains its actual in-range source index';
 if(state.phase==='allocate'&&(state.allocationPrototype===undefined||state.allocationPrototype.$!=='object'||!testJavaScriptSemanticSameValue(state.allocationPrototype,directory.arrayPrototype)||readJavaScriptOrdinaryValueError(heap,state.allocationPrototype)!==''))return 'Array result allocation retains its actual prototype';
 if(state.phase==='construct'&&(state.constructor===undefined||state.constructorFact===undefined||state.constructor.$!=='object'||state.constructor.identity!==state.constructorFact.identity||!state.constructorFact.constructable||!testJavaScriptSemanticSameValue(state.constructor,state.species?.candidate??{$:'undefined'})))return 'Array custom construction retains its actual selected constructor and fact';
 if(state.phase==='construct'&&state.constructorFact!==undefined){const fact=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,state.constructorFact.identity)];if(fact===undefined||fact.constructable!==state.constructorFact.constructable||fact.realm!==state.constructorFact.realm||fact.intrinsicArray!==state.constructorFact.intrinsicArray)return 'Array custom construction retains its actual registered constructor classification';}
 if(state.phase==='write'&&state.operation==='Array.filter'&&!testJavaScriptSemanticSameValue(state.value,state.element))return 'filter writes its retained original element rather than its predicate result';
 if(state.call!==undefined){
  if(!['length getter','length conversion','species','element getter','callback'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length>3)return 'Array transformation retains its actual selected getter or callback';
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
  if(state.phase==='callback'){const args=state.call.arguments;if(!testJavaScriptSemanticSameValue(state.call.callee,state.callback)||!testJavaScriptSemanticSameValue(state.call.receiver,state.thisArgument)||args.length!==3||!testJavaScriptSemanticSameValue(args[0]??{$:'undefined'},state.element)||!testJavaScriptSemanticSameValue(args[1]??{$:'undefined'},{$:'number',value:state.index})||!testJavaScriptSemanticSameValue(args[2]??{$:'undefined'},state.receiver))return 'Array transform callbacks retain actual element, index, source and thisArg';}
  if(state.phase==='species'){const call=state.species?.call;if(call===undefined||!testJavaScriptSemanticSameValue(call.callee,state.call.callee)||!testJavaScriptSemanticSameValue(call.receiver,state.call.receiver)||call.arguments.length!==state.call.arguments.length)return 'Array transform species calls retain their actual nested selection';for(let at=0;at<call.arguments.length;at++)if(!testJavaScriptSemanticSameValue(call.arguments[at]??{$:'undefined'},state.call.arguments[at]??{$:'undefined'}))return 'Array species call arguments cannot change between continuations';}
 }else if(['length getter','element getter','callback'].includes(state.phase))return 'Array transform getter/callback phases retain their selected call';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayTransformState} state
 * @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver
 * @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArrayTransformTransition} */
export function requestJavaScriptArrayTransformCall(heap,state,callee,receiver,arguments_){return {$:'arrayTransformCall',heap,state:{...state,call:{$:'arrayTransformSelectedCall',callee,receiver,arguments:arguments_}},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayTransformState} state
 * @param {JavaScriptArraySpeciesTransition} transition @returns {JavaScriptArrayTransformTransition} */
export function continueJavaScriptArrayTransformSpecies(heap,state,transition){
 const {call:priorCall,...base}=state;
 switch(transition.$){
  case 'arraySpeciesContinue':return {$:'arrayTransformContinue',heap:transition.heap,state:{...base,phase:'species',species:transition.state}};
  case 'arraySpeciesCall':return requestJavaScriptArrayTransformCall(transition.heap,{...base,phase:'species',species:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'arraySpeciesNeedAuthority':return {$:'arrayTransformNeedAuthority',heap:transition.heap,state:{...base,phase:'species',species:transition.state},constructor:transition.constructor};
  case 'arraySpeciesAllocate':return requestJavaScriptArrayTransformAllocation(transition.heap,{...base,phase:'allocate',species:transition.state,allocationPrototype:transition.prototype},transition.prototype,transition.length);
  case 'arraySpeciesConstruct':{
   const selected={...base,species:transition.state,constructor:transition.constructor,constructorFact:{...transition.fact}};
   if(transition.fact.intrinsicArray&&transition.fact.realm===transition.state.realm&&testJavaScriptSemanticSameValue(transition.constructor,transition.state.arrayConstructor))return requestJavaScriptArrayTransformAllocation(transition.heap,{...selected,phase:'allocate',allocationPrototype:transition.state.arrayPrototype},transition.state.arrayPrototype,transition.state.length);
   return {$:'arrayTransformConstruct',heap:transition.heap,state:{...selected,phase:'construct'},constructor:transition.constructor,arguments:transition.arguments,fact:{...transition.fact}};
  }
  case 'arraySpeciesThrow':return {$:'arrayTransformThrow',heap:transition.heap,value:transition.value};
  case 'arraySpeciesError':return {$:'arrayTransformError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arraySpeciesInvalid':return {$:'arrayTransformInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArrayTransformTransition} */
export function continueJavaScriptArrayTransformLength(heap,directory,state,transition){
 const {call:priorCall,conversion:priorConversion,...base}=state;
 switch(transition.$){
  case 'scalarContinuation':return {$:'arrayTransformContinue',heap,state:{...base,phase:'length conversion',conversion:transition.state}};
  case 'scalarCall':return requestJavaScriptArrayTransformCall(heap,{...base,phase:'length conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arrayTransformInvalid',heap,reason:'Array transform length retains its actual ToNumber result'};
   if(state.callback.$!=='object'||!state.callback.callable)return {$:'arrayTransformError',heap,errorClass:'TypeError',reason:'Array callback must be callable'};
   const length=readJavaScriptArrayLikeLength(transition.value.value),next={...base,length,input:{$:'undefined'},phase:'species'};
   return continueJavaScriptArrayTransformSpecies(heap,next,beginJavaScriptArraySpeciesSelection(heap,directory,state.receiver,state.operation==='Array.map'?length:0));
  }
  case 'scalarThrow':return {$:'arrayTransformThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arrayTransformError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arrayTransformInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayTransformResourceLimit',heap,state:{...base,phase:'length primitive',input:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {string} operation @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} callback
 * @param {JavaScriptSemanticValue} thisArgument @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArrayTransformTransition} */
export function beginJavaScriptArrayTransform(heap,directory,operation,receiver,callback,thisArgument,exoticKey){
 if(receiver.$==='null'||receiver.$==='undefined')return {$:'arrayTransformError',heap,errorClass:'TypeError',reason:'Array transformation called on null or undefined'};
 const state={$:'arrayTransformState',operation,receiver,callback,thisArgument,exoticKey,length:0,index:0,outputIndex:0,target:{$:'undefined'},element:{$:'undefined'},value:{$:'undefined'},input:{$:'undefined'},phase:'length'};
 const error=readJavaScriptArrayTransformStateError(heap,directory,state);return error===''?{$:'arrayTransformContinue',heap,state}:{$:'arrayTransformInvalid',heap,reason:error};
}
/** Indexed CreateDataPropertyOrThrow. The key is never the special length key.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {number} index @param {JavaScriptSemanticValue} value @returns {JavaScriptIndexedPropertyCreation} */
export function createJavaScriptIndexedDataProperty(heap,target,index,value){
 const invalid=readJavaScriptOrdinaryHeapError(heap);if(invalid!=='')return {$:'indexedPropertyInvalid',heap,reason:invalid};
 if(target.$!=='object'||!Number.isSafeInteger(index)||index<0||Object.is(index,-0)||readJavaScriptOrdinaryValueError(heap,target)!==''||readJavaScriptOrdinaryValueError(heap,value)!=='')return {$:'indexedPropertyInvalid',heap,reason:'indexed property creation retains its actual Object, index and value'};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)];if(object===undefined||!['ordinaryObject','closureObject','arrayObject','mapObject','setObject','collectionIteratorObject'].includes(object.$))return {$:'indexedPropertyInvalid',heap,reason:'indexed property creation requires its actual opaque host/Proxy definition protocol'};
 const name=String(index),units=[];for(let at=0;at<name.length;at++)units.push(name.charCodeAt(at));const key={$:'text',units},descriptor=createJavaScriptOrdinarySetDescriptor(value,true),at=findJavaScriptOrdinaryProperty(object.properties,key);
 const keyError=readJavaScriptOrdinaryKeyError(heap,key);if(keyError!=='')return {$:'indexedPropertyInvalid',heap,reason:keyError};
 if(object.$==='arrayObject'){
  const plan=planJavaScriptArrayPropertyDefinition(object,key,descriptor);if(plan.errorClass==='resource')return {$:'indexedPropertyResourceLimit',heap,reason:plan.error};if(plan.errorClass!=='')return {$:'indexedPropertyInvalid',heap,reason:plan.error};if(!plan.success)return {$:'indexedPropertyError',heap,reason:'indexed CreateDataProperty was rejected'};
 }else{const current=object.properties[at]?.descriptor??createJavaScriptPropertyDescriptor('data',false,false),plan=planJavaScriptPropertyDefinition(at>=0,object.extensible,current,descriptor);if(plan.disposition==='reject')return {$:'indexedPropertyError',heap,reason:'indexed CreateDataProperty was rejected'};}
 if(at<0){let total=0;for(const row of heap.objects)total+=row.properties.length;if(object.properties.length>=4096||total>=16384)return {$:'indexedPropertyResourceLimit',heap,reason:'indexed properties exceed their actual shared storage budget'};}
 const defined=defineJavaScriptOrdinaryProperty(heap,target.identity,key,descriptor);return defined.success?{$:'indexedPropertyCreated',heap:defined.heap}:{$:'indexedPropertyInvalid',heap:defined.heap,reason:defined.error||'an indexed definition changed after its complete plan'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayTransformState} state @returns {JavaScriptArrayTransformTransition} */
export function beginJavaScriptArrayTransformElement(heap,state,value){return requestJavaScriptArrayTransformCall(heap,{...state,phase:'callback',element:value},state.callback,state.thisArgument,[value,{$:'number',value:state.index},state.receiver]);}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @returns {JavaScriptArrayTransformTransition} */
export function stepJavaScriptArrayTransform(heap,directory,state){
 const error=readJavaScriptArrayTransformStateError(heap,directory,state);if(error!=='')return {$:'arrayTransformInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arrayTransformCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='length conversion')return state.conversion===undefined?{$:'arrayTransformInvalid',heap,reason:'Array transform length requires its actual continuation'}:continueJavaScriptArrayTransformLength(heap,directory,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='length primitive')return continueJavaScriptArrayTransformLength(heap,directory,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.input));
 if(state.phase==='species')return state.species===undefined?{$:'arrayTransformInvalid',heap,reason:'Array transformation requires its actual species continuation'}:continueJavaScriptArrayTransformSpecies(heap,state,stepJavaScriptArraySpeciesSelection(heap,directory,state.species));
 if(state.phase==='allocate')return requestJavaScriptArrayTransformAllocation(heap,state,state.allocationPrototype??{$:'undefined'},state.operation==='Array.map'?state.length:0);
 if(state.phase==='construct')return state.constructor===undefined||state.constructorFact===undefined?{$:'arrayTransformInvalid',heap,reason:'Array transformation retains its selected custom constructor'}:{$:'arrayTransformConstruct',heap,state,constructor:state.constructor,arguments:[{$:'number',value:state.operation==='Array.map'?state.length:0}],fact:{...state.constructorFact}};
 if(state.receiver.$!=='object')return {$:'arrayTransformInvalid',heap,reason:'Array transformation retains its actual boxed source'};
 if(state.phase==='length'){
  const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,{$:'text',units:[108,101,110,103,116,104]},state.receiver);if(get.error!=='')return {$:'arrayTransformInvalid',heap,reason:get.error};
  if(get.action==='call')return requestJavaScriptArrayTransformCall(heap,{...state,phase:'length getter'},get.callee,get.receiver,get.arguments);if(get.action!=='value')return {$:'arrayTransformInvalid',heap,reason:'Array transformation length retains its actual Get result'};
  return continueJavaScriptArrayTransformLength(heap,directory,{...state,input:get.value},beginJavaScriptScalarProtocol(heap,get.value,state.exoticKey,'coerce primitive number'));
 }
 if(state.phase==='write'){
  const written=createJavaScriptIndexedDataProperty(heap,state.target,state.operation==='Array.map'?state.index:state.outputIndex,state.value);
  if(written.$==='indexedPropertyResourceLimit')return {$:'arrayTransformResourceLimit',heap:written.heap,state,reason:written.reason};if(written.$==='indexedPropertyError')return {$:'arrayTransformError',heap:written.heap,errorClass:'TypeError',reason:written.reason};if(written.$==='indexedPropertyInvalid')return {$:'arrayTransformInvalid',heap:written.heap,reason:written.reason};
  return {$:'arrayTransformContinue',heap:written.heap,state:{...state,phase:'ready',index:state.index+1,outputIndex:state.outputIndex+1,element:{$:'undefined'},value:{$:'undefined'}}};
 }
 if(state.phase!=='ready')return {$:'arrayTransformInvalid',heap,reason:'Array transformation requires its actual child completion'};
 if(state.index>=state.length)return {$:'arrayTransformCompleted',heap,value:state.target};
 const name=String(state.index),units=[];for(let at=0;at<name.length;at++)units.push(name.charCodeAt(at));const key={$:'text',units},presence=readJavaScriptIndexedPropertyPresence(heap,state.receiver.identity,key);if(presence.error!=='')return {$:'arrayTransformInvalid',heap,reason:presence.error};if(!presence.present)return {$:'arrayTransformContinue',heap,state:{...state,index:state.index+1}};
 const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,key,state.receiver);if(get.error!=='')return {$:'arrayTransformInvalid',heap,reason:get.error};if(get.action==='call')return requestJavaScriptArrayTransformCall(heap,{...state,phase:'element getter'},get.callee,get.receiver,get.arguments);if(get.action!=='value')return {$:'arrayTransformInvalid',heap,reason:'Array transformation element retains its actual Get result'};
 return beginJavaScriptArrayTransformElement(heap,state,get.value);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayTransformTransition} */
export function resumeJavaScriptArrayTransformCall(heap,directory,state,completion){
 const error=readJavaScriptArrayTransformStateError(heap,directory,state);if(error!=='')return {$:'arrayTransformInvalid',heap,reason:error};if(state.call===undefined)return {$:'arrayTransformInvalid',heap,reason:'Array transformation resumption retains its actual selected child'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayTransformInvalid',heap,reason:invalid};if(completion.$==='throwCompletion')return {$:'arrayTransformThrow',heap,value:completion.value};
 const {call:priorCall,...base}=state;
 if(state.phase==='length getter')return continueJavaScriptArrayTransformLength(heap,directory,{...base,input:completion.value},beginJavaScriptScalarProtocol(heap,completion.value,state.exoticKey,'coerce primitive number'));
 if(state.phase==='length conversion')return state.conversion===undefined?{$:'arrayTransformInvalid',heap,reason:'Array length child retains its actual scalar continuation'}:continueJavaScriptArrayTransformLength(heap,directory,base,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.phase==='species')return state.species===undefined?{$:'arrayTransformInvalid',heap,reason:'Array species child retains its actual selection'}:continueJavaScriptArrayTransformSpecies(heap,base,resumeJavaScriptArraySpeciesGetter(heap,directory,state.species,completion));
 if(state.phase==='element getter')return beginJavaScriptArrayTransformElement(heap,base,completion.value);
 if(state.phase!=='callback')return {$:'arrayTransformInvalid',heap,reason:'Array callback completion retains its actual transformation phase'};
 if(state.operation==='Array.filter'&&!convertJavaScriptSemanticBoolean(completion.value))return {$:'arrayTransformContinue',heap,state:{...base,phase:'ready',index:state.index+1,element:{$:'undefined'}}};
 return {$:'arrayTransformContinue',heap,state:{...base,phase:'write',value:state.operation==='Array.map'?completion.value:state.element}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @param {bigint} identity @returns {JavaScriptArrayTransformTransition} */
export function resumeJavaScriptArrayTransformAllocation(heap,directory,state,identity){
 const error=readJavaScriptArrayTransformStateError(heap,directory,state);if(error!=='')return {$:'arrayTransformInvalid',heap,reason:error};if(state.phase!=='allocate'||state.allocationPrototype===undefined)return {$:'arrayTransformInvalid',heap,reason:'Array allocation resumption retains its actual target creation request'};
 const created=allocateJavaScriptArray(heap,identity,state.allocationPrototype,state.operation==='Array.map'?state.length:0);
 if(created.$==='arrayNormal')return {$:'arrayTransformContinue',heap:created.heap,state:{...state,phase:'ready',target:created.value}};
 if(created.$==='arrayResourceLimit')return {$:'arrayTransformResourceLimit',heap:created.heap,state,reason:created.reason};if(created.$==='arrayError')return {$:'arrayTransformError',heap:created.heap,errorClass:created.errorClass,reason:created.reason};
 return {$:'arrayTransformInvalid',heap:created.heap,reason:created.$==='arrayInvalid'?created.reason:'Array creation cannot substitute an unrelated definition continuation'};
}
/** Construct has already performed its return/receiver rules; primitive normal
 * results are malformed protocol completions, not fabricated species Objects.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptArrayTransformState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayTransformTransition} */
export function resumeJavaScriptArrayTransformConstruction(heap,directory,state,completion){
 const error=readJavaScriptArrayTransformStateError(heap,directory,state);if(error!=='')return {$:'arrayTransformInvalid',heap,reason:error};if(state.phase!=='construct')return {$:'arrayTransformInvalid',heap,reason:'custom construction completion retains its actual selected constructor'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayTransformInvalid',heap,reason:invalid};if(completion.$==='throwCompletion')return {$:'arrayTransformThrow',heap,value:completion.value};if(completion.value.$!=='object')return {$:'arrayTransformInvalid',heap,reason:'Construct must return its actual result Object'};
 return {$:'arrayTransformContinue',heap,state:{...state,phase:'ready',target:completion.value}};
}

/** ArrayCreate rejects its length and storage limits before requesting identity.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayTransformState} state
 * @param {JavaScriptSemanticValue} prototype @param {number} length @returns {JavaScriptArrayTransformTransition} */
export function requestJavaScriptArrayTransformAllocation(heap,state,prototype,length){
 if(length>4294967295)return {$:'arrayTransformError',heap,errorClass:'RangeError',reason:'Invalid array length'};
 let properties=0;for(const object of heap.objects)properties+=object.properties.length;
 if(heap.objects.length>=4096||properties>=16384)return {$:'arrayTransformResourceLimit',heap,state,reason:'Array transform result allocation exceeds its shared storage budget'};
 return {$:'arrayTransformAllocate',heap,state,prototype,length};
}
