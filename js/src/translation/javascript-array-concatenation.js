// Array concat composes species, ordinary Get/Has, scalar conversion, indexed
// definition and strict Set. Its receiver is the caller's actual ToObject result.
import {readJavaScriptOrdinaryValueError,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {readJavaScriptArrayLikeLength,readJavaScriptIndexedPropertyPresence} from './javascript-array-index-rules.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {beginJavaScriptArraySpeciesSelection,stepJavaScriptArraySpeciesSelection,resumeJavaScriptArraySpeciesGetter,readJavaScriptArraySpeciesStateError,readJavaScriptArraySpeciesDirectoryError,classifyJavaScriptArraySpeciesConstructor} from './javascript-array-species.js';
import {allocateJavaScriptArray,testJavaScriptArrayValue} from './javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
import {readJavaScriptObjectSpreadStateError,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread} from './javascript-object-spread.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @typedef {{$:'arrayConcatenationCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptArrayConcatenationCall */
/** @typedef {{$:'arrayConcatenationState',receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],spreadableKey:JavaScriptSemanticValue,exoticKey:JavaScriptSemanticValue,itemPosition:number,item:JavaScriptSemanticValue,itemStart:number,length:number,index:number,outputIndex:number,spread:boolean,target:JavaScriptSemanticValue,value:JavaScriptSemanticValue,input:JavaScriptSemanticValue,phase:string,conversion?:JavaScriptPrimitiveConversionState,species?:JavaScriptArraySpeciesState,allocationPrototype?:JavaScriptSemanticValue,constructor?:JavaScriptSemanticValue,constructorFact?:JavaScriptArrayConstructorFact,assignment?:JavaScriptObjectSpreadState,call?:JavaScriptArrayConcatenationCall}} JavaScriptArrayConcatenationState */
/** @typedef {{$:'arrayConcatenationContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState} |
 * {$:'arrayConcatenationCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'arrayConcatenationAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState,prototype:JavaScriptSemanticValue,length:number} |
 * {$:'arrayConcatenationConstruct',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState,constructor:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],fact:JavaScriptArrayConstructorFact} |
 * {$:'arrayConcatenationNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState,constructor:JavaScriptSemanticValue} |
 * {$:'arrayConcatenationCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayConcatenationThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'arrayConcatenationError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'arrayConcatenationInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'arrayConcatenationResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptArrayConcatenationState,reason:string}} JavaScriptArrayConcatenationTransition */
const absent={$:'undefined'},lengthKey={$:'text',units:[108,101,110,103,116,104]},maximumIndex=9007199254740991;
/** @param {JavaScriptArrayConcatenationCall} actual @param {{callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}|undefined} expected @returns {boolean} */
function testJavaScriptArrayConcatenationCall(actual,expected){return expected!==undefined&&testJavaScriptSemanticSameValue(actual.callee,expected.callee)&&testJavaScriptSemanticSameValue(actual.receiver,expected.receiver)&&actual.arguments.length===expected.arguments.length&&actual.arguments.every((value,index)=>testJavaScriptSemanticSameValue(value,expected.arguments[index]??absent));}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArrayConcatenationState} state @returns {string} */
export function readJavaScriptArrayConcatenationStateError(heap,directory,state){
 const directoryError=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(directoryError!=='')return directoryError;
 if(state.$!=='arrayConcatenationState'||state.receiver.$!=='object'||state.spreadableKey.$!=='symbol'||state.exoticKey.$!=='symbol'||typeof state.spread!=='boolean')return 'concatenation retains its actual Object receiver and spreadability/conversion Symbols';
 for(const value of [state.receiver,...state.arguments,state.spreadableKey,state.exoticKey,state.item,state.target,state.value,state.input]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 for(const coordinate of [state.itemPosition,state.itemStart,state.length,state.index,state.outputIndex])if(!Number.isSafeInteger(coordinate)||coordinate<0||Object.is(coordinate,-0))return 'concatenation retains exact nonnegative safe coordinates';
 if(state.itemPosition>state.arguments.length+1||state.index>state.length||state.itemStart>state.outputIndex)return 'concatenation retains its actual item List and captured spread interval';
 if(!['initialize','species','allocate','construct','select','spreadability','spreadability getter','length','length getter','conversion','primitive','ready','element getter','write','finish'].includes(state.phase))return 'concatenation retains its actual continuation phase';
 const creation=['initialize','species','allocate','construct'].includes(state.phase),active=!creation&&!['select','finish'].includes(state.phase);
 if(creation&&(state.itemPosition!==0||state.outputIndex!==0||state.item.$!=='undefined'||state.target.$!=='undefined'))return 'concatenation completes species creation before selecting any input';
 if(active&&(state.itemPosition>state.arguments.length||!testJavaScriptSemanticSameValue(state.item,state.itemPosition===0?state.receiver:state.arguments[state.itemPosition-1]??absent)))return 'concatenation retains the actual selected receiver or argument';
 if(active&&state.item.$!=='object'&&!(state.phase==='write'&&!state.spread))return 'only a scalar concatenation write accepts a primitive item';
 if(['length','length getter','conversion','primitive','ready','element getter'].includes(state.phase)&&!state.spread)return 'concatenation spread work retains its completed spreadability decision';
 if(['ready','element getter','write'].includes(state.phase)&&state.spread&&(state.length>maximumIndex-state.itemStart||state.outputIndex!==state.itemStart+state.index))return 'concatenation checks the full saved spread length before indexed work';
 if(['element getter','write'].includes(state.phase)&&state.spread&&state.index>=state.length)return 'concatenation spread work retains an in-range input coordinate';
 if(state.phase==='write'&&!state.spread&&(state.outputIndex!==state.itemStart||state.outputIndex>=maximumIndex||!testJavaScriptSemanticSameValue(state.value,state.item)))return 'concatenation scalar writes retain the selected item and checked output coordinate';
 if(state.phase==='conversion'){
  if(state.conversion===undefined||state.conversion.hint!=='number'||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.input)||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,state.exoticKey))return 'concatenation length conversion retains the actual input and number hint';
  const error=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(error!=='')return error;
 }else if(state.conversion!==undefined)return 'only active concatenation conversion retains its scalar child';
 if(state.phase==='primitive'&&state.input.$==='object')return 'concatenation primitive suspension retains its completed scalar';
 if(state.species!==undefined){const error=readJavaScriptArraySpeciesStateError(heap,directory,state.species);if(error!=='')return error;if(!testJavaScriptSemanticSameValue(state.species.original,state.receiver)||state.species.length!==0)return 'concatenation retains ArraySpeciesCreate with its actual receiver and zero length';}
 if(state.phase!=='initialize'&&state.species===undefined)return 'concatenation retains its actual species selection';
 if(!creation&&state.target.$!=='object')return 'concatenation retains its actual constructed result Object';
 if(state.phase==='allocate'&&(state.allocationPrototype===undefined||!testJavaScriptSemanticSameValue(state.allocationPrototype,directory.arrayPrototype)))return 'concatenation fallback retains its actual realm Array prototype';
 if(state.phase==='construct'){
  const fact=state.constructorFact,classification=classifyJavaScriptArraySpeciesConstructor(heap,directory,state.constructor??absent),actual=classification.$==='arrayConstructorKnown'?classification.fact:undefined;
  if(state.constructor?.$!=='object'||fact===undefined||!fact.constructable||actual===undefined||!testJavaScriptSemanticSameValue(state.constructor,state.species?.candidate??absent)||state.constructor.identity!==fact.identity||actual.constructable!==fact.constructable||actual.realm!==fact.realm||actual.intrinsicArray!==fact.intrinsicArray||actual.intrinsicKind!==fact.intrinsicKind)return 'concatenation construction retains the actual selected constructor fact';
 }
 if((state.phase==='finish')!==(state.assignment!==undefined))return 'concatenation final Set retains only its actual assignment child';
 if(state.phase==='finish'&&state.itemPosition!==state.arguments.length+1)return 'concatenation final Set follows every actual input item';
 if(state.assignment!==undefined){const child=state.assignment,error=readJavaScriptObjectSpreadStateError(heap,child);if(error!=='')return error;if(child.operation!=='assignment'||!testJavaScriptSemanticSameValue(child.target,state.target)||!testJavaScriptSemanticSameValue(child.input,state.receiver)||!testJavaScriptSemanticSameValue(child.exoticKey??absent,state.exoticKey)||child.keys.length!==1||!testJavaScriptSemanticSameValue(child.keys[0]??absent,lengthKey)||child.excluded.length!==0||!(child.position===0&&['write','setter','array definition'].includes(child.phase)||child.position===1&&child.phase==='read')||child.position===0&&!testJavaScriptSemanticSameValue(child.value,{$:'number',value:state.outputIndex})||child.assignment!==undefined&&!testJavaScriptSemanticSameValue(child.assignment.definition.descriptor.value,{$:'number',value:state.outputIndex}))return 'concatenation final Set retains its actual result, length key and output count';}
 if(state.call!==undefined){
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
  if(state.call.callee.$!=='object'||!state.call.callee.callable)return 'concatenation child calls retain actual callable identities';
  if(['spreadability getter','length getter','element getter'].includes(state.phase)){if(!testJavaScriptSemanticSameValue(state.call.receiver,state.item)||state.call.arguments.length!==0)return 'concatenation getters retain the actual selected item receiver';}
  else if(state.phase==='species'){if(!testJavaScriptArrayConcatenationCall(state.call,state.species?.call))return 'concatenation species retains its selected nested getter';}
  else if(state.phase==='finish'){if(!testJavaScriptArrayConcatenationCall(state.call,state.assignment?.call))return 'concatenation final Set retains its selected nested setter';}
  else if(state.phase==='conversion'&&state.conversion!==undefined){const child=state.conversion,exotic=child.phase==='exoticCall';if(!['exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(child.phase)||!testJavaScriptSemanticSameValue(state.call.receiver,child.receiver)||state.call.arguments.length!==(exotic?1:0)||exotic&&!testJavaScriptSemanticSameValue(state.call.arguments[0]??absent,{$:'text',units:[110,117,109,98,101,114]}))return 'concatenation scalar calls retain their selected conversion operands';}
  else return 'concatenation retains child calls only in the actual suspended phase';
 }else if(['spreadability getter','length getter','element getter'].includes(state.phase)||state.phase==='conversion'&&state.conversion!==undefined&&['exoticGetter','firstGetter','secondGetter','exoticCall','firstCall','secondCall'].includes(state.conversion.phase)||state.phase==='species'&&state.species?.call!==undefined||state.phase==='finish'&&state.assignment?.call!==undefined)return 'concatenation retains its actual selected child';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConcatenationState} state @param {JavaScriptSemanticValue} callee @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptArrayConcatenationTransition} */
function requestJavaScriptArrayConcatenationCall(heap,state,callee,receiver,arguments_){return {$:'arrayConcatenationCall',heap,state:{...state,call:{$:'arrayConcatenationCall',callee,receiver,arguments:arguments_}},callee,receiver,arguments:arguments_};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConcatenationState} state @returns {JavaScriptArrayConcatenationTransition} */
function requestJavaScriptArrayConcatenationAllocation(heap,state){
 let properties=0;for(const object of heap.objects)properties+=object.properties.length;
 if(heap.objects.length>=4096||properties>=16384)return {$:'arrayConcatenationResourceLimit',heap,state,reason:'concatenation result allocation exceeds shared storage budget'};
 return {$:'arrayConcatenationAllocate',heap,state,prototype:state.allocationPrototype??absent,length:0};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConcatenationState} state @param {JavaScriptArraySpeciesTransition} transition @returns {JavaScriptArrayConcatenationTransition} */
function continueJavaScriptArrayConcatenationSpecies(heap,state,transition){
 const {call:priorCall,...base}=state;
 switch(transition.$){
  case 'arraySpeciesContinue':return {$:'arrayConcatenationContinue',heap:transition.heap,state:{...base,phase:'species',species:transition.state}};
  case 'arraySpeciesCall':return requestJavaScriptArrayConcatenationCall(transition.heap,{...base,phase:'species',species:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'arraySpeciesNeedAuthority':return {$:'arrayConcatenationNeedAuthority',heap:transition.heap,state:{...base,phase:'species',species:transition.state},constructor:transition.constructor};
  case 'arraySpeciesAllocate':return requestJavaScriptArrayConcatenationAllocation(transition.heap,{...base,phase:'allocate',species:transition.state,allocationPrototype:transition.prototype});
  case 'arraySpeciesConstruct':{
   const selected={...base,species:transition.state,constructor:transition.constructor,constructorFact:{...transition.fact}};
   if(transition.fact.intrinsicArray&&transition.fact.realm===transition.state.realm&&testJavaScriptSemanticSameValue(transition.constructor,transition.state.arrayConstructor))return requestJavaScriptArrayConcatenationAllocation(transition.heap,{...selected,phase:'allocate',allocationPrototype:transition.state.arrayPrototype});
   return {$:'arrayConcatenationConstruct',heap:transition.heap,state:{...selected,phase:'construct'},constructor:transition.constructor,arguments:transition.arguments,fact:{...transition.fact}};
  }
  case 'arraySpeciesThrow':return {$:'arrayConcatenationThrow',heap:transition.heap,value:transition.value};
  case 'arraySpeciesError':return {$:'arrayConcatenationError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arraySpeciesInvalid':return {$:'arrayConcatenationInvalid',heap:transition.heap,reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConcatenationState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptArrayConcatenationTransition} */
function continueJavaScriptArrayConcatenationLength(heap,state,transition){
 const {call:priorCall,conversion:priorConversion,...base}=state;
 switch(transition.$){
  case 'scalarContinuation':return {$:'arrayConcatenationContinue',heap,state:{...base,phase:'conversion',conversion:transition.state}};
  case 'scalarCall':return requestJavaScriptArrayConcatenationCall(heap,{...base,phase:'conversion',conversion:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'scalarCompleted':{
   if(transition.value.$!=='number')return {$:'arrayConcatenationInvalid',heap,reason:'concatenation length retains its actual ToNumber result'};
   const length=readJavaScriptArrayLikeLength(transition.value.value);
   if(length>maximumIndex-state.outputIndex)return {$:'arrayConcatenationError',heap,errorClass:'TypeError',reason:'concat exceeds maximum safe output length'};
   return {$:'arrayConcatenationContinue',heap,state:{...base,phase:'ready',length,input:absent}};
  }
  case 'scalarThrow':return {$:'arrayConcatenationThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':return {$:'arrayConcatenationError',heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'scalarInvalid':return {$:'arrayConcatenationInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'arrayConcatenationResourceLimit',heap,state:{...base,phase:'primitive',input:transition.value},reason:transition.reason};
 }
}
/** @param {JavaScriptArrayConcatenationState} state @param {JavaScriptObjectSpreadTransition} transition @returns {JavaScriptArrayConcatenationTransition} */
function continueJavaScriptArrayConcatenationFinalSet(state,transition){
 const {call:priorCall,...base}=state;
 switch(transition.$){
  case 'objectSpreadContinue':return {$:'arrayConcatenationContinue',heap:transition.heap,state:{...base,assignment:transition.state}};
  case 'objectSpreadCall':return requestJavaScriptArrayConcatenationCall(transition.heap,{...base,assignment:transition.state},transition.callee,transition.receiver,transition.arguments);
  case 'objectSpreadCompleted':return {$:'arrayConcatenationCompleted',heap:transition.heap,value:state.target};
  case 'objectSpreadThrow':return {$:'arrayConcatenationThrow',heap:transition.heap,value:transition.value};
  case 'objectSpreadError':return {$:'arrayConcatenationError',heap:transition.heap,errorClass:transition.errorClass??'TypeError',reason:transition.reason};
  case 'objectSpreadInvalid':return {$:'arrayConcatenationInvalid',heap:transition.heap,reason:transition.reason};
  case 'objectSpreadResourceLimit':return {$:'arrayConcatenationResourceLimit',heap:transition.heap,state:{...base,assignment:transition.state},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConcatenationState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptArrayConcatenationTransition} */
function continueJavaScriptArrayConcatenationSpreadability(heap,state,value){
 let spread;
 if(value.$==='undefined'){const brand=testJavaScriptArrayValue(heap,state.item);if(brand.error!=='')return {$:'arrayConcatenationInvalid',heap,reason:brand.error};spread=brand.array;}
 else spread=convertJavaScriptSemanticBoolean(value);
 if(!spread){if(state.outputIndex>=maximumIndex)return {$:'arrayConcatenationError',heap,errorClass:'TypeError',reason:'concat exceeds maximum safe output length'};return {$:'arrayConcatenationContinue',heap,state:{...state,phase:'write',spread:false,value:state.item}};}
 return {$:'arrayConcatenationContinue',heap,state:{...state,phase:'length',spread:true}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSemanticValue} receiver @param {JavaScriptSemanticValue[]} arguments_ @param {JavaScriptSemanticValue} spreadableKey @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptArrayConcatenationTransition} */
export function beginJavaScriptArrayConcatenation(heap,directory,receiver,arguments_,spreadableKey,exoticKey){
 const state={$:'arrayConcatenationState',receiver,arguments:[...arguments_],spreadableKey,exoticKey,itemPosition:0,item:absent,itemStart:0,length:0,index:0,outputIndex:0,spread:false,target:absent,value:absent,input:absent,phase:'initialize'};
 const error=readJavaScriptArrayConcatenationStateError(heap,directory,state);return error===''?continueJavaScriptArrayConcatenationSpecies(heap,state,beginJavaScriptArraySpeciesSelection(heap,directory,receiver,0)):{$:'arrayConcatenationInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArrayConcatenationState} state @returns {JavaScriptArrayConcatenationTransition} */
export function stepJavaScriptArrayConcatenation(heap,directory,state){
 const error=readJavaScriptArrayConcatenationStateError(heap,directory,state);if(error!=='')return {$:'arrayConcatenationInvalid',heap,reason:error};
 if(state.call!==undefined)return {$:'arrayConcatenationCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='initialize')return continueJavaScriptArrayConcatenationSpecies(heap,state,beginJavaScriptArraySpeciesSelection(heap,directory,state.receiver,0));
 if(state.phase==='species')return state.species===undefined?{$:'arrayConcatenationInvalid',heap,reason:'concatenation retains species continuation'}:continueJavaScriptArrayConcatenationSpecies(heap,state,stepJavaScriptArraySpeciesSelection(heap,directory,state.species));
 if(state.phase==='allocate')return requestJavaScriptArrayConcatenationAllocation(heap,state);
 if(state.phase==='construct')return state.constructor===undefined||state.constructorFact===undefined?{$:'arrayConcatenationInvalid',heap,reason:'concatenation retains its selected constructor'}:{$:'arrayConcatenationConstruct',heap,state,constructor:state.constructor,arguments:[{$:'number',value:0}],fact:{...state.constructorFact}};
 if(state.phase==='conversion')return state.conversion===undefined?{$:'arrayConcatenationInvalid',heap,reason:'concatenation retains scalar continuation'}:continueJavaScriptArrayConcatenationLength(heap,state,stepJavaScriptScalarProtocol(heap,state.conversion,'coerce primitive number'));
 if(state.phase==='primitive')return continueJavaScriptArrayConcatenationLength(heap,state,finishJavaScriptScalarPrimitive('coerce primitive number',state.input));
 if(state.phase==='finish')return state.assignment===undefined?{$:'arrayConcatenationInvalid',heap,reason:'concatenation retains final Set'}:continueJavaScriptArrayConcatenationFinalSet(state,stepJavaScriptObjectSpread(heap,state.assignment));
 if(state.phase==='select'){
  if(state.itemPosition===state.arguments.length+1){const assignment={$:'objectSpreadState',target:state.target,input:state.receiver,excluded:[],keys:[lengthKey],position:0,phase:'write',value:{$:'number',value:state.outputIndex},callee:absent,operation:'assignment',exoticKey:state.exoticKey};return {$:'arrayConcatenationContinue',heap,state:{...state,phase:'finish',assignment}};}
  const item=state.itemPosition===0?state.receiver:state.arguments[state.itemPosition-1]??absent,selected={...state,item,itemStart:state.outputIndex,length:0,index:0,spread:false,value:absent,input:absent};
  if(item.$==='object')return {$:'arrayConcatenationContinue',heap,state:{...selected,phase:'spreadability'}};
  if(state.outputIndex>=maximumIndex)return {$:'arrayConcatenationError',heap,errorClass:'TypeError',reason:'concat exceeds maximum safe output length'};
  return {$:'arrayConcatenationContinue',heap,state:{...selected,phase:'write',value:item}};
 }
 if(state.phase==='spreadability'||state.phase==='length'){
  if(state.item.$!=='object')return {$:'arrayConcatenationInvalid',heap,reason:'concatenation property reads retain actual Object items'};
  const spreadability=state.phase==='spreadability',get=planJavaScriptOrdinaryGet(heap,state.item.identity,spreadability?state.spreadableKey:lengthKey,state.item);
  if(get.error!=='')return {$:'arrayConcatenationInvalid',heap,reason:get.error};
  if(get.action==='call')return requestJavaScriptArrayConcatenationCall(heap,{...state,phase:spreadability?'spreadability getter':'length getter'},get.callee,get.receiver,get.arguments);
  if(get.action!=='value')return {$:'arrayConcatenationInvalid',heap,reason:'concatenation retains actual spreadability or length Get completion'};
  return spreadability?continueJavaScriptArrayConcatenationSpreadability(heap,state,get.value):continueJavaScriptArrayConcatenationLength(heap,{...state,input:get.value},beginJavaScriptScalarProtocol(heap,get.value,state.exoticKey,'coerce primitive number'));
 }
 if(state.phase==='write'){
  const written=createJavaScriptIndexedDataProperty(heap,state.target,state.outputIndex,state.value);
  if(written.$==='indexedPropertyResourceLimit')return {$:'arrayConcatenationResourceLimit',heap:written.heap,state,reason:written.reason};
  if(written.$==='indexedPropertyError')return {$:'arrayConcatenationError',heap:written.heap,errorClass:'TypeError',reason:written.reason};
  if(written.$==='indexedPropertyInvalid')return {$:'arrayConcatenationInvalid',heap:written.heap,reason:written.reason};
  return {$:'arrayConcatenationContinue',heap:written.heap,state:{...state,phase:state.spread?'ready':'select',index:state.spread?state.index+1:0,itemPosition:state.itemPosition+(state.spread?0:1),outputIndex:state.outputIndex+1,value:absent}};
 }
 if(state.phase!=='ready'||state.item.$!=='object')return {$:'arrayConcatenationInvalid',heap,reason:'concatenation resumes only its actual selected child'};
 if(state.index===state.length)return {$:'arrayConcatenationContinue',heap,state:{...state,phase:'select',itemPosition:state.itemPosition+1}};
 const name=String(state.index),units=[];for(let index=0;index<name.length;index++)units.push(name.charCodeAt(index));const key={$:'text',units},presence=readJavaScriptIndexedPropertyPresence(heap,state.item.identity,key);
 if(presence.error!=='')return {$:'arrayConcatenationInvalid',heap,reason:presence.error};
 if(!presence.present)return {$:'arrayConcatenationContinue',heap,state:{...state,index:state.index+1,outputIndex:state.outputIndex+1}};
 const get=planJavaScriptOrdinaryGet(heap,state.item.identity,key,state.item);if(get.error!=='')return {$:'arrayConcatenationInvalid',heap,reason:get.error};
 if(get.action==='call')return requestJavaScriptArrayConcatenationCall(heap,{...state,phase:'element getter'},get.callee,get.receiver,get.arguments);
 return get.action==='value'?{$:'arrayConcatenationContinue',heap,state:{...state,phase:'write',value:get.value}}:{$:'arrayConcatenationInvalid',heap,reason:'concatenation indexed Get retains its actual completion'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArrayConcatenationState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayConcatenationTransition} */
export function resumeJavaScriptArrayConcatenationCall(heap,directory,state,completion){
 const error=readJavaScriptArrayConcatenationStateError(heap,directory,state);if(error!=='')return {$:'arrayConcatenationInvalid',heap,reason:error};
 if(state.call===undefined||!['normalCompletion','throwCompletion'].includes(completion.$))return {$:'arrayConcatenationInvalid',heap,reason:'concatenation resumption retains the actual selected call and completion kind'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayConcatenationInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'arrayConcatenationThrow',heap,value:completion.value};
 const {call:priorCall,...base}=state;
 if(state.phase==='species'&&state.species!==undefined)return continueJavaScriptArrayConcatenationSpecies(heap,base,resumeJavaScriptArraySpeciesGetter(heap,directory,state.species,completion));
 if(state.phase==='spreadability getter')return continueJavaScriptArrayConcatenationSpreadability(heap,base,completion.value);
 if(state.phase==='length getter')return continueJavaScriptArrayConcatenationLength(heap,{...base,input:completion.value},beginJavaScriptScalarProtocol(heap,completion.value,state.exoticKey,'coerce primitive number'));
 if(state.phase==='conversion'&&state.conversion!==undefined)return continueJavaScriptArrayConcatenationLength(heap,base,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,'coerce primitive number'));
 if(state.phase==='element getter')return {$:'arrayConcatenationContinue',heap,state:{...base,phase:'write',value:completion.value}};
 if(state.phase==='finish'&&state.assignment!==undefined)return continueJavaScriptArrayConcatenationFinalSet(base,resumeJavaScriptObjectSpread(heap,state.assignment,completion));
 return {$:'arrayConcatenationInvalid',heap,reason:'concatenation resumption retains its actual suspended phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArrayConcatenationState} state @param {bigint} identity @returns {JavaScriptArrayConcatenationTransition} */
export function resumeJavaScriptArrayConcatenationAllocation(heap,directory,state,identity){
 const error=readJavaScriptArrayConcatenationStateError(heap,directory,state);if(error!=='')return {$:'arrayConcatenationInvalid',heap,reason:error};
 if(state.phase!=='allocate'||state.allocationPrototype===undefined)return {$:'arrayConcatenationInvalid',heap,reason:'concatenation allocation retains its actual pending request'};
 const preflight=requestJavaScriptArrayConcatenationAllocation(heap,state);if(preflight.$!=='arrayConcatenationAllocate')return preflight;
 const result=allocateJavaScriptArray(heap,identity,state.allocationPrototype,0);
 if(result.$==='arrayNormal')return {$:'arrayConcatenationContinue',heap:result.heap,state:{...state,phase:'select',target:result.value}};
 if(result.$==='arrayError')return {$:'arrayConcatenationError',heap:result.heap,errorClass:result.errorClass,reason:result.reason};
 if(result.$==='arrayResourceLimit')return {$:'arrayConcatenationResourceLimit',heap:result.heap,state,reason:result.reason};
 return {$:'arrayConcatenationInvalid',heap:result.heap,reason:result.$==='arrayInvalid'?result.reason:'concatenation allocation cannot substitute a definition continuation'};
}
/** Construct has already applied its receiver/return rules; accept only its actual Object result.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptArrayConcatenationState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptArrayConcatenationTransition} */
export function resumeJavaScriptArrayConcatenationConstruction(heap,directory,state,completion){
 const error=readJavaScriptArrayConcatenationStateError(heap,directory,state);if(error!=='')return {$:'arrayConcatenationInvalid',heap,reason:error};
 if(state.phase!=='construct'||!['normalCompletion','throwCompletion'].includes(completion.$))return {$:'arrayConcatenationInvalid',heap,reason:'concatenation construction retains its actual pending request and completion kind'};
 const invalid=readJavaScriptOrdinaryValueError(heap,completion.value);if(invalid!=='')return {$:'arrayConcatenationInvalid',heap,reason:invalid};
 if(completion.$==='throwCompletion')return {$:'arrayConcatenationThrow',heap,value:completion.value};
 return completion.value.$==='object'?{$:'arrayConcatenationContinue',heap,state:{...state,phase:'select',target:completion.value}}:{$:'arrayConcatenationInvalid',heap,reason:'concatenation Construct completion retains its actual Object result'};
}
