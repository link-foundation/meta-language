// Generic slice composes shared conversion, species, indexed definition and Set
// continuations. Its caller supplies the actual ToObject result.
import {readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptArrayLikeLength,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptArraySpeciesSelection,stepJavaScriptArraySpeciesSelection,resumeJavaScriptArraySpeciesGetter,readJavaScriptArraySpeciesStateError,readJavaScriptArraySpeciesDirectoryError,classifyJavaScriptArraySpeciesConstructor} from './javascript-array-species.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
import {readJavaScriptObjectSpreadStateError,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread} from './javascript-object-spread.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'arraySliceCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArraySliceCall */
/** @typedef {{$:'arraySliceState',receiver:JavaScriptSemanticValue,start:JavaScriptSemanticValue,end:JavaScriptSemanticValue,endPresent:boolean,exoticKey:JavaScriptSemanticValue,length:number,startIndex:number,endIndex:number,count:number,index:number,outputIndex:number,target:JavaScriptSemanticValue,value:JavaScriptSemanticValue,input:JavaScriptSemanticValue,phase:string,conversionKind:string,conversion?:JavaScriptPrimitiveConversionState,species?:JavaScriptArraySpeciesState,allocationPrototype?:JavaScriptSemanticValue,constructor?:JavaScriptSemanticValue,constructorFact?:JavaScriptArrayConstructorFact,assignment?:JavaScriptObjectSpreadState,call?:JavaScriptArraySliceCall}} JavaScriptArraySliceState */
/** @typedef {{$:'arraySliceContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState} |
 * {$:'arraySliceCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arraySliceAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState,prototype:JavaScriptSemanticValue,length:number} |
 * {$:'arraySliceConstruct',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fact:JavaScriptArrayConstructorFact} |
 * {$:'arraySliceNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState,constructor:JavaScriptSemanticValue} |
 * {$:'arraySliceCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySliceThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arraySliceError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arraySliceInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arraySliceResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArraySliceState,reason:string}} JavaScriptArraySliceTransition */
const absent={$:'undefined'},lengthKey={$:'text',units:[108,101,110,103,116,104]};
/** @param {number} value @param {number} length @returns {number} */
export function readJavaScriptArraySliceRelativeIndex(value,length){
 const integer=Number.isNaN(value)||value===0?0:Math.trunc(value);
 return integer<0?Math.max(length+integer,0):Math.min(integer,length);
}
/** @param {JavaScriptArraySliceCall} actual @param {{callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}|undefined} expected @returns {boolean} */
function sameCall(actual,expected){return expected!==undefined&&testJavaScriptSemanticSameValue(actual.callee,expected.callee)&&testJavaScriptSemanticSameValue(actual.receiver,expected.receiver)&&actual.arguments.length===expected.arguments.length&&actual.arguments.every((value,index)=>testJavaScriptSemanticSameValue(value,expected.arguments[index]??absent));}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @returns {string} */
export function readJavaScriptArraySliceStateError(heap,directory,state){
 const directoryError=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(directoryError!=='')return directoryError;
 if(state.$!=='arraySliceState'||state.receiver.$!=='object'||state.exoticKey.$!=='symbol'||typeof state.endPresent!=='boolean'||!state.endPresent&&state.end.$!=='undefined')return 'slice retains its actual Object receiver, argument presence and conversion Symbol';
 for(const value of [state.receiver,state.start,state.end,state.exoticKey,state.target,state.value,state.input]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 for(const coordinate of [state.length,state.startIndex,state.endIndex,state.count,state.index,state.outputIndex])if(!Number.isSafeInteger(coordinate)||coordinate<0||Object.is(coordinate,-0))return 'slice retains exact nonnegative safe coordinates';
 if(state.startIndex>state.length||state.endIndex>state.length||state.count>state.length||state.index>state.length||state.outputIndex>state.count)return 'slice coordinates remain within its saved length';
 if(!['length','length getter','conversion','primitive','species','allocate','construct','ready','element getter','write','finish'].includes(state.phase)||!['length','start','end'].includes(state.conversionKind))return 'slice retains its actual phase and ordered conversion operand';
 if(state.phase==='conversion'){
  if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.input)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'slice conversion retains its actual operand, hint and Symbol';
  const error=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(error!=='')return error;
 }else if(state.conversion!==undefined)return 'only an active slice conversion retains its scalar child';
 if(state.phase==='primitive'&&state.input.$==='object')return 'slice primitive suspension retains its completed scalar';
 if(['conversion','primitive'].includes(state.phase)&&state.conversionKind!=='length'&&!testJavaScriptSemanticSameValue(state.input,state.conversionKind==='start'?state.start:state.end)&&state.phase!=='primitive')return 'slice bounds retain the actual original argument';
 const initialized=['species','allocate','construct','ready','element getter','write','finish'].includes(state.phase);
 if(initialized&&(state.count!==Math.max(state.endIndex-state.startIndex,0)||state.index!==state.startIndex+state.outputIndex))return 'slice retains its selected interval and corresponding input/output coordinates';
 if(['species','allocate','construct'].includes(state.phase)&&(state.outputIndex!==0||state.target.$!=='undefined')||state.phase==='finish'&&state.outputIndex!==state.count)return 'slice construction precedes element work and final Set follows the complete selected interval';
 if(state.phase!=='write'&&state.value.$!=='undefined')return 'slice retains an element value only during its actual output definition';
 if(['element getter','write'].includes(state.phase)&&state.outputIndex>=state.count)return 'slice element work retains an in-range output coordinate';
 if(state.species!==undefined){const error=readJavaScriptArraySpeciesStateError(heap,directory,state.species);if(error!=='')return error;if(!testJavaScriptSemanticSameValue(state.species.original,state.receiver)||state.species.length!==state.count)return 'slice species retains its actual source and requested count';}
 if(initialized&&state.species===undefined)return 'slice result creation retains its actual species selection';
 if(['ready','element getter','write','finish'].includes(state.phase)&&state.target.$!=='object')return 'slice retains its actual constructed result Object';
 if(state.phase==='allocate'&&(state.allocationPrototype===undefined||!testJavaScriptSemanticSameValue(state.allocationPrototype,directory.arrayPrototype)))return 'slice fallback allocation retains its actual realm Array prototype';
 if(state.phase==='construct'){
  const fact=state.constructorFact,classification=classifyJavaScriptArraySpeciesConstructor(heap,directory,state.constructor??absent),actual=classification.$==='arrayConstructorKnown'?classification.fact:undefined;
  if(state.constructor?.$!=='object'||fact===undefined||!fact.constructable||actual===undefined||!testJavaScriptSemanticSameValue(state.constructor,state.species?.candidate??absent)||state.constructor.identity!==fact.identity||actual.constructable!==fact.constructable||actual.realm!==fact.realm||actual.intrinsicArray!==fact.intrinsicArray||actual.intrinsicKind!==fact.intrinsicKind)return 'slice construction retains the actual selected constructor fact';
 }
 if((state.phase==='finish')!==(state.assignment!==undefined))return 'slice final Set retains only its actual assignment child';
 if(state.assignment!==undefined){const child=state.assignment,error=readJavaScriptObjectSpreadStateError(heap,child);if(error!=='')return error;if(child.operation!=='assignment'||!testJavaScriptSemanticSameValue(child.target,state.target)||!testJavaScriptSemanticSameValue(child.input,state.receiver)||!testJavaScriptSemanticSameValue(child.exoticKey??absent,state.exoticKey)||child.keys.length!==1||!testJavaScriptSemanticSameValue(child.keys[0]??absent,lengthKey)||child.excluded.length!==0||!(child.position===0&&['write','setter','array definition'].includes(child.phase)||child.position===1&&child.phase==='read')||child.position===0&&!testJavaScriptSemanticSameValue(child.value,{$:'number',value:state.count})||child.assignment!==undefined&&!testJavaScriptSemanticSameValue(child.assignment.definition.descriptor.value,{$:'number',value:state.count}))return 'slice final Set retains its actual target, length key and count';}
 if(state.call!==undefined){
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
  if(state.call.callee.$!=='object'||!state.call.callee.callable)return 'slice child calls retain an actual callable';
  if(['length getter','element getter'].includes(state.phase)){if(!testJavaScriptSemanticSameValue(state.call.receiver,state.receiver)||state.call.arguments.length!==0)return 'slice getters retain the original boxed receiver and no arguments';}
  else if(state.phase==='species'){if(!sameCall(state.call,state.species?.call))return 'slice species retains its selected nested getter';}
  else if(state.phase==='finish'){if(!sameCall(state.call,state.assignment?.call))return 'slice final Set retains its selected nested setter';}
  else if(state.phase==='conversion'&&state.conversion!==undefined){const child=state.conversion,exotic=child.phase==='exoticCall';if(!['exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(child.phase)||!testJavaScriptSemanticSameValue(state.call.receiver,child.receiver)||state.call.arguments.length!==(exotic?1:0)||exotic&&!testJavaScriptSemanticSameValue(state.call.arguments[0]??absent,{$:'text',units:[110,117,109,98,101,114]}))return 'slice scalar calls retain the selected conversion operands';}
  else return 'slice retains child calls only in their actual suspended phase';
 }else if(['length getter','element getter'].includes(state.phase)||state.phase==='conversion'&&state.conversion!==undefined&&['exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(state.conversion.phase)||state.phase==='species'&&state.species?.call!==undefined||state.phase==='finish'&&state.assignment?.call!==undefined)return 'slice retains its actual selected child';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySliceState} state @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArraySliceTransition} */
function requestCall(heap,state,callee,receiver,arguments_){return {$:'arraySliceCall',heap,state:{...state,call:{$:'arraySliceCall',callee,receiver,arguments:arguments_}},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySliceState} state @returns {JavaScriptArraySliceTransition} */
function allocationRequest(heap,state){
 if(state.count>4294967295)return {$:'arraySliceError',heap,errorClass:'RangeError',reason:'Invalid array length'};
 let properties=0;for(const object of heap.objects)properties+=object.properties.length;
 if(heap.objects.length>=4096||properties>=16384)return {$:'arraySliceResourceLimit',heap,state,reason:'slice result allocation exceeds its shared storage budget'};
 return {$:'arraySliceAllocate',heap,state,prototype:state.allocationPrototype??absent,length:state.count};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySliceState} state @param {JavaScriptArraySpeciesTransition} transition @returns {JavaScriptArraySliceTransition} */
function acceptSpecies(heap,state,transition){
 const {call:priorCall,...base}=state;
 switch(transition.$){
  case 'arraySpeciesContinue':return {$:'arraySliceContinue',heap:transition.heap,state:{...base,phase:'species',species:transition.state}};
  case 'arraySpeciesCall':return requestCall(transition.heap,{...base,phase:'species',species:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'arraySpeciesNeedAuthority':return {$:'arraySliceNeedAuthority',heap:transition.heap,state:{...base,phase:'species',species:transition.state},constructor:transition.constructor};
  case 'arraySpeciesAllocate':return allocationRequest(transition.heap,{...base,phase:'allocate',species:transition.state,allocationPrototype:transition.prototype});
  case 'arraySpeciesConstruct':{
   const selected={...base,species:transition.state,constructor:transition.constructor,constructorFact:{...transition.fact}};
   if(transition.fact.intrinsicArray&&transition.fact.realm===transition.state.realm&&testJavaScriptSemanticSameValue(transition.constructor,transition.state.arrayConstructor))return allocationRequest(transition.heap,{...selected,phase:'allocate',allocationPrototype:transition.state.arrayPrototype});
   return {$:'arraySliceConstruct',heap:transition.heap,state:{...selected,phase:'construct'},constructor:transition.constructor,arguments:transition.arguments,fact:{...transition.fact}};
  }
  case 'arraySpeciesThrow':return {$:'arraySliceThrow',heap:transition.heap,value:transition.value};
  case 'arraySpeciesError':return {$:'arraySliceError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arraySpeciesInvalid':return {$:'arraySliceInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArraySliceTransition} */
function acceptScalar(heap,directory,state,transition){
 const {call:priorCall,conversion:priorConversion,...base}=state;
 switch(transition.$){
  case 'scalarContinuation':return {$:'arraySliceContinue',heap,state:{...base,phase:'conversion',conversion:transition.state}};
  case 'scalarCall':return requestCall(heap,{...base,phase:'conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arraySliceInvalid',heap,reason:'slice scalar conversion retains its actual ToNumber result'};
   if(state.conversionKind==='length'){const next={...base,length:readJavaScriptArrayLikeLength(transition.value.value),conversionKind:'start',input:state.start};return acceptScalar(heap,directory,next,beginJavaScriptScalarProtocol(heap,state.start,state.exoticKey,'coerce primitive number'));}
   if(state.conversionKind==='start'){
    const startIndex=readJavaScriptArraySliceRelativeIndex(transition.value.value,state.length),next={...base,startIndex,index:startIndex,conversionKind:'end',input:state.end};
    if(state.endPresent&&state.end.$!=='undefined')return acceptScalar(heap,directory,next,beginJavaScriptScalarProtocol(heap,state.end,state.exoticKey,'coerce primitive number'));
    const count=Math.max(state.length-startIndex,0),selected={...next,endIndex:state.length,count,input:absent,phase:'species'};return acceptSpecies(heap,selected,beginJavaScriptArraySpeciesSelection(heap,directory,state.receiver,count));
   }
   const endIndex=readJavaScriptArraySliceRelativeIndex(transition.value.value,state.length),count=Math.max(endIndex-state.startIndex,0),selected={...base,endIndex,count,input:absent,phase:'species'};return acceptSpecies(heap,selected,beginJavaScriptArraySpeciesSelection(heap,directory,state.receiver,count));
  }
  case 'scalarThrow':return {$:'arraySliceThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arraySliceError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arraySliceInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arraySliceResourceLimit',heap,state:{...base,phase:'primitive',input:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptArraySliceState} state @param {JavaScriptObjectSpreadTransition} transition @returns {JavaScriptArraySliceTransition} */
function acceptFinalSet(state,transition){
 const {call:priorCall,...base}=state;
 switch(transition.$){
  case 'objectSpreadContinue':return {$:'arraySliceContinue',heap:transition.heap,state:{...base,assignment:transition.state}};
  case 'objectSpreadCall':return requestCall(transition.heap,{...base,assignment:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'objectSpreadCompleted':return {$:'arraySliceCompleted',heap:transition.heap,value:state.target};
  case 'objectSpreadThrow':return {$:'arraySliceThrow',heap:transition.heap,value:transition.value};
  case 'objectSpreadError':return {$:'arraySliceError',heap:transition.heap,errorClass:transition.errorClass??'TypeError',reason:transition.reason};
  case 'objectSpreadInvalid':return {$:'arraySliceInvalid',heap:transition.heap,reason:transition.reason};
  case 'objectSpreadResourceLimit':return {$:'arraySliceResourceLimit',heap:transition.heap,state:{...base,assignment:transition.state},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue} start @param {JavaScriptSemanticValue} end @param {boolean} endPresent @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArraySliceTransition} */
export function beginJavaScriptArraySlice(heap,directory,receiver,start,end,endPresent,exoticKey){
 const state={$:'arraySliceState',receiver,start,end,endPresent,exoticKey,length:0,startIndex:0,endIndex:0,count:0,index:0,outputIndex:0,target:absent,value:absent,input:absent,phase:'length',conversionKind:'length'};
 const error=readJavaScriptArraySliceStateError(heap,directory,state);return error===''?{$:'arraySliceContinue',heap,state}:{$:'arraySliceInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @returns {JavaScriptArraySliceTransition} */
export function stepJavaScriptArraySlice(heap,directory,state){
 const error=readJavaScriptArraySliceStateError(heap,directory,state);if(error!=='')return {$:'arraySliceInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arraySliceCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='conversion')return state.conversion===undefined?{$:'arraySliceInvalid',heap,reason:'slice retains its scalar child'}:acceptScalar(heap,directory,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='primitive')return acceptScalar(heap,directory,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.input));
 if(state.phase==='species')return state.species===undefined?{$:'arraySliceInvalid',heap,reason:'slice retains its species child'}:acceptSpecies(heap,state,stepJavaScriptArraySpeciesSelection(heap,directory,state.species));
 if(state.phase==='allocate')return allocationRequest(heap,state);
 if(state.phase==='construct')return state.constructor===undefined||state.constructorFact===undefined?{$:'arraySliceInvalid',heap,reason:'slice retains its actual constructor request'}:{$:'arraySliceConstruct',heap,state,constructor:state.constructor,arguments:[{$:'number',value:state.count}],fact:{...state.constructorFact}};
 if(state.phase==='finish')return state.assignment===undefined?{$:'arraySliceInvalid',heap,reason:'slice retains final Set'}:acceptFinalSet(state,stepJavaScriptObjectSpread(heap,state.assignment));
 if(state.receiver.$!=='object')return {$:'arraySliceInvalid',heap,reason:'slice requires its actual ToObject receiver'};
 if(state.phase==='length'){
  const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,lengthKey,state.receiver);if(get.error!=='')return {$:'arraySliceInvalid',heap,reason:get.error};
  if(get.action==='call')return requestCall(heap,{...state,phase:'length getter'},get.callee,get.receiver,get.arguments);
  if(get.action!=='value')return {$:'arraySliceInvalid',heap,reason:'slice length retains its actual Get result'};
  return acceptScalar(heap,directory,{...state,input:get.value},beginJavaScriptScalarProtocol(heap,get.value,state.exoticKey,'coerce primitive number'));
 }
 if(state.phase==='write'){
  const written=createJavaScriptIndexedDataProperty(heap,state.target,state.outputIndex,state.value);
  if(written.$==='indexedPropertyResourceLimit')return {$:'arraySliceResourceLimit',heap:written.heap,state,reason:written.reason};
  if(written.$==='indexedPropertyError')return {$:'arraySliceError',heap:written.heap,errorClass:'TypeError',reason:written.reason};
  if(written.$==='indexedPropertyInvalid')return {$:'arraySliceInvalid',heap:written.heap,reason:written.reason};
  return {$:'arraySliceContinue',heap:written.heap,state:{...state,phase:'ready',index:state.index+1,outputIndex:state.outputIndex+1,value:absent}};
 }
 if(state.phase!=='ready')return {$:'arraySliceInvalid',heap,reason:'slice retains its actual child completion'};
 if(state.outputIndex===state.count){
  const assignment={$:'objectSpreadState',target:state.target,input:state.receiver,excluded:[],keys:[lengthKey],position:0,phase:'write',value:{$:'number',value:state.count},callee:absent,operation:'assignment',exoticKey:state.exoticKey};
  return {$:'arraySliceContinue',heap,state:{...state,phase:'finish',assignment}};
 }
 const name=String(state.index),units=[];for(let at=0;at<name.length;at++)units.push(name.charCodeAt(at));const key={$:'text',units},presence=readJavaScriptIndexedPropertyPresence(heap,state.receiver.identity,key);
 if(presence.error!=='')return {$:'arraySliceInvalid',heap,reason:presence.error};
 if(!presence.present)return {$:'arraySliceContinue',heap,state:{...state,index:state.index+1,outputIndex:state.outputIndex+1}};
 const get=planJavaScriptOrdinaryGet(heap,state.receiver.identity,key,state.receiver);if(get.error!=='')return {$:'arraySliceInvalid',heap,reason:get.error};
 if(get.action==='call')return requestCall(heap,{...state,phase:'element getter'},get.callee,get.receiver,get.arguments);
 return get.action==='value'?{$:'arraySliceContinue',heap,state:{...state,phase:'write',value:get.value}}:{$:'arraySliceInvalid',heap,reason:'slice element retains its actual Get result'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArraySliceTransition} */
export function resumeJavaScriptArraySliceCall(heap,directory,state,completion){
 const error=readJavaScriptArraySliceStateError(heap,directory,state);if(error!=='')return {$:'arraySliceInvalid',heap,reason:error};
 if(state.call===undefined||!['normalCompletion','throwCompletion'].includes(completion.$))return {$:'arraySliceInvalid',heap,reason:'slice resumption retains its actual selected call and completion kind'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arraySliceInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'arraySliceThrow',heap,value:completion.value};
 const {call:priorCall,...base}=state;
 if(state.phase==='length getter')return acceptScalar(heap,directory,{...base,input:completion.value},beginJavaScriptScalarProtocol(heap,completion.value,state.exoticKey,'coerce primitive number'));
 if(state.phase==='conversion'&&state.conversion!==undefined)return acceptScalar(heap,directory,base,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.phase==='species'&&state.species!==undefined)return acceptSpecies(heap,base,resumeJavaScriptArraySpeciesGetter(heap,directory,state.species,completion));
 if(state.phase==='finish'&&state.assignment!==undefined)return acceptFinalSet(base,resumeJavaScriptObjectSpread(heap,state.assignment,completion));
 if(state.phase==='element getter')return {$:'arraySliceContinue',heap,state:{...base,phase:'write',value:completion.value}};
 return {$:'arraySliceInvalid',heap,reason:'slice resumption retains the actual suspended phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @param {bigint} identity @returns {JavaScriptArraySliceTransition} */
export function resumeJavaScriptArraySliceAllocation(heap,directory,state,identity){
 const error=readJavaScriptArraySliceStateError(heap,directory,state);if(error!=='')return {$:'arraySliceInvalid',heap,reason:error};
 if(state.phase!=='allocate'||state.allocationPrototype===undefined)return {$:'arraySliceInvalid',heap,reason:'slice allocation resumption retains its actual pending request'};
 const preflight=allocationRequest(heap,state);if(preflight.$!=='arraySliceAllocate')return preflight;
 const result=allocateJavaScriptArray(heap,identity,state.allocationPrototype,state.count);
 if(result.$==='arrayNormal')return {$:'arraySliceContinue',heap:result.heap,state:{...state,phase:'ready',target:result.value}};
 if(result.$==='arrayError')return {$:'arraySliceError',heap:result.heap,errorClass:result.errorClass,reason:result.reason};
 if(result.$==='arrayResourceLimit')return {$:'arraySliceResourceLimit',heap:result.heap,state,reason:result.reason};
 return {$:'arraySliceInvalid',heap:result.heap,reason:result.$==='arrayInvalid'?result.reason:'slice allocation cannot substitute a definition continuation'};
}
/** Construct has already applied actual receiver/return rules in the shared
 * species construction kernel; this resumer accepts only its Object result.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArraySliceState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArraySliceTransition} */
export function resumeJavaScriptArraySliceConstruction(heap,directory,state,completion){
 const error=readJavaScriptArraySliceStateError(heap,directory,state);if(error!=='')return {$:'arraySliceInvalid',heap,reason:error};
 if(state.phase!=='construct'||!['normalCompletion','throwCompletion'].includes(completion.$))return {$:'arraySliceInvalid',heap,reason:'slice construction retains its actual request and completion kind'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arraySliceInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'arraySliceThrow',heap,value:completion.value};
 return completion.value.$==='object'?{$:'arraySliceContinue',heap,state:{...state,phase:'ready',target:completion.value}}:{$:'arraySliceInvalid',heap,reason:'slice Construct completion retains an actual Object result'};
}
