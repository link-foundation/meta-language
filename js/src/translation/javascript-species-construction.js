// New Construct(C, [length]) requests use the ordinary shared constructor kernel.
// States preceding that explicit model retain their exact saved interpretation.
import {readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,insertJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {readJavaScriptArraySpeciesDirectoryError,findJavaScriptArrayConstructorFact,classifyJavaScriptArraySpeciesConstructor} from './javascript-array-species.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {beginJavaScriptConstruction,stepJavaScriptConstruction,resumeJavaScriptConstructionAllocation,resumeJavaScriptConstructionCall,continueJavaScriptConstructionPrototype,readJavaScriptConstructionStateError} from './javascript-construction.js';
/** @typedef {{$:'speciesConstructionCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptSpeciesConstructionCall */
/** @typedef {{$:'speciesConstruction',constructor:JavaScriptSemanticValue,fact:JavaScriptArrayConstructorFact,length:number,phase:string,mode:string,prototype:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,call?:JavaScriptSpeciesConstructionCall,model?:string,operation?:JavaScriptConstructionState}} JavaScriptSpeciesConstructionState */
/** @typedef {{$:'speciesConstructionContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState} |
 * {$:'speciesConstructionCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],constructing:boolean} |
 * {$:'speciesConstructionAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState,prototype:JavaScriptSemanticValue} |
 * {$:'speciesConstructionNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState,constructor:JavaScriptSemanticValue} |
 * {$:'speciesConstructionNeedImplementation',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState,constructor:JavaScriptSemanticValue,newTarget?:JavaScriptSemanticValue,arguments?:JavaScriptSemanticValue[],operation?:string,reason?:string} |
 * {$:'speciesConstructionCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'speciesConstructionThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'speciesConstructionError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string,realm?:bigint} |
 * {$:'speciesConstructionInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'speciesConstructionResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptSpeciesConstructionState,reason:string}} JavaScriptSpeciesConstructionTransition */
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSpeciesConstructionState} state @returns {string} */
export function readJavaScriptSpeciesConstructionStateError(program,heap,directory,state){
 const error=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(error!=='')return error;
 if(state.model!==undefined||state.operation!==undefined)return readJavaScriptSharedSpeciesConstructionStateError(program,heap,directory,state);
 const fact=directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,state.fact.identity)];
 if(fact===undefined||fact.realm!==state.fact.realm||fact.intrinsicArray!==state.fact.intrinsicArray||fact.constructable!==state.fact.constructable||!fact.constructable||state.constructor.$!=='object'||!state.constructor.callable||state.constructor.identity!==fact.identity)return 'species construction retains its actual registered constructor and immutable classification';
 if(!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0)||!['prototype','prototype getter','prototype fallback','allocate','body'].includes(state.phase))return 'species construction retains its actual length and construction phase';
 for(const value of [state.constructor,state.prototype,state.receiver])if(readJavaScriptOrdinaryValueError(heap,value)!=='')return 'species construction values belong to the current shared heap';
 const fn=readJavaScriptExecutableCallFunction(program,heap,state.constructor.identity),mode=fact.intrinsicArray?'Array intrinsic':fn?.receiverMode??'';
 if(state.mode!==mode||fn!==undefined&&!fact.intrinsicArray&&fn.realm!==fact.realm)return 'species construction retains its actual executable constructor mode and realm';
 if(['allocate','body'].includes(state.phase)&&state.prototype.$!=='object')return 'species construction allocation retains its actual selected prototype';
 if(state.phase==='body'){
  if(state.receiver.$!=='object'||state.receiver.callable||state.mode!=='strict'&&state.mode!=='ordinary class')return 'source constructor body retains its actual newly allocated ordinary receiver';
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)];if(object===undefined||object.$!=='ordinaryObject')return 'source construction retains its actual allocated ordinary receiver';
 }
 if(state.call!==undefined){
  if(!['prototype getter','body'].includes(state.phase)||state.call.callee.$!=='object'||!state.call.callee.callable||state.call.arguments.length>2)return 'species construction retains its actual selected getter/body call';
  for(const value of [state.call.callee,state.call.receiver,...state.call.arguments])if(readJavaScriptOrdinaryValueError(heap,value)!=='')return 'species constructor call values retain actual shared identities';
  if(state.phase==='body'&&(!testJavaScriptSemanticSameValue(state.call.callee,state.constructor)||!testJavaScriptSemanticSameValue(state.call.receiver,state.receiver)||state.call.arguments.length!==1||!testJavaScriptSemanticSameValue(state.call.arguments[0]??{$:'undefined'},{$:'number',value:state.length})))return 'constructor body retains its actual callee, allocated receiver and length argument';
 }else if(state.phase==='prototype getter'||state.phase==='body')return 'constructor getter/body phases retain their actual selected call';
 return '';
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSemanticValue} constructor
 * @param {JavaScriptArrayConstructorFact} fact @param {number} length @returns {JavaScriptSpeciesConstructionTransition} */
export function beginJavaScriptSpeciesConstruction(program,heap,directory,constructor,fact,length){
 const seed={$:'speciesConstruction',constructor,fact:{...fact},length,phase:'target',mode:'',prototype:{$:'undefined'},receiver:{$:'undefined'}},transition=adaptJavaScriptSpeciesConstruction(seed,beginJavaScriptConstruction(program,heap,directory,constructor,constructor,[{$:'number',value:length}]));
 if(transition.$!=='speciesConstructionContinue')return transition;const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,transition.state);return error===''?transition:{$:'speciesConstructionInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory
 * @param {JavaScriptSpeciesConstructionState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptSpeciesConstructionTransition} */
export function continueJavaScriptSpeciesPrototype(heap,directory,state,value){
 if(state.operation!==undefined)return adaptJavaScriptSpeciesConstruction(state,continueJavaScriptConstructionPrototype(heap,directory,state.operation,value));
 const {call:priorCall,...base}=state;
 if(value.$==='object')return {$:'speciesConstructionContinue',heap,state:{...base,prototype:value,phase:'allocate'}};
 const root=(directory.constructorRealms??[]).find(row=>row.realm===state.fact.realm);
 if(root===undefined)return {$:'speciesConstructionNeedAuthority',heap,state:{...base,prototype:value,phase:'prototype fallback'},constructor:state.constructor};
 return {$:'speciesConstructionContinue',heap,state:{...base,prototype:state.mode==='Array intrinsic'?root.arrayPrototype:root.objectPrototype,phase:'allocate'}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSpeciesConstructionState} state @returns {JavaScriptSpeciesConstructionTransition} */
export function stepJavaScriptSpeciesConstruction(program,heap,directory,state){
 const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,state);if(error!=='')return {$:'speciesConstructionInvalid',heap,reason:error};
 if(state.operation!==undefined)return adaptJavaScriptSpeciesConstruction(state,stepJavaScriptConstruction(program,heap,directory,state.operation));
 if(state.call!==undefined)return {$:'speciesConstructionCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments,constructing:state.phase==='body'};
 if(!['strict','ordinary class','Array intrinsic'].includes(state.mode))return {$:'speciesConstructionNeedImplementation',heap,state,constructor:state.constructor};
 if(state.phase==='prototype fallback')return continueJavaScriptSpeciesPrototype(heap,directory,state,state.prototype);
 if(state.phase==='allocate'){
  if(state.mode==='Array intrinsic'&&state.length>4294967295)return {$:'speciesConstructionError',heap,errorClass:'RangeError',reason:'Invalid array length'};
  let properties=0;for(const object of heap.objects)properties+=object.properties.length;if(heap.objects.length>=4096||state.mode==='Array intrinsic'&&properties>=16384)return {$:'speciesConstructionResourceLimit',heap,state,reason:'species construction allocation exceeds its actual shared storage budget'};
  return {$:'speciesConstructionAllocate',heap,state,prototype:state.prototype};
 }
 if(state.phase!=='prototype')return {$:'speciesConstructionInvalid',heap,reason:'species construction needs its actual getter or constructor completion'};
 const get=planJavaScriptOrdinaryGet(heap,state.constructor.identity,{$:'text',units:[112,114,111,116,111,116,121,112,101]},state.constructor);if(get.error!=='')return {$:'speciesConstructionInvalid',heap,reason:get.error};
 if(get.action==='call'){const call={$:'speciesConstructionCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'speciesConstructionCall',heap,state:{...state,phase:'prototype getter',call},callee:get.callee,receiver:get.receiver,arguments:get.arguments,constructing:false};}
 if(get.action!=='value')return {$:'speciesConstructionInvalid',heap,reason:'constructor prototype acquisition retains its actual Get result'};
 return continueJavaScriptSpeciesPrototype(heap,directory,state,get.value);
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSpeciesConstructionState} state
 * @param {bigint} identity @returns {JavaScriptSpeciesConstructionTransition} */
export function resumeJavaScriptSpeciesConstructionAllocation(program,heap,directory,state,identity){
 if(state.operation!==undefined){const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,state);return error!==''?{$:'speciesConstructionInvalid',heap,reason:error}:adaptJavaScriptSpeciesConstruction(state,resumeJavaScriptConstructionAllocation(program,heap,directory,state.operation,identity));}
 const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,state);if(error!=='')return {$:'speciesConstructionInvalid',heap,reason:error};if(state.phase!=='allocate')return {$:'speciesConstructionInvalid',heap,reason:'species constructor allocation retains its actual pending request'};
 if(state.mode==='Array intrinsic'){
  const created=allocateJavaScriptArray(heap,identity,state.prototype,state.length);if(created.$==='arrayNormal')return {$:'speciesConstructionCompleted',heap:created.heap,value:created.value};if(created.$==='arrayResourceLimit')return {$:'speciesConstructionResourceLimit',heap:created.heap,state,reason:created.reason};if(created.$==='arrayError')return {$:'speciesConstructionError',heap:created.heap,errorClass:created.errorClass,reason:created.reason};return {$:'speciesConstructionInvalid',heap:created.heap,reason:created.$==='arrayInvalid'?created.reason:'Array construction cannot use a property definition completion'};
 }
 const created=insertJavaScriptOrdinaryObject(heap,identity,state.prototype);if(!created.success)return {$:'speciesConstructionInvalid',heap:created.heap,reason:created.error};
 const receiver={$:'object',identity,callable:false},call={$:'speciesConstructionCall',callee:state.constructor,receiver,arguments:[{$:'number',value:state.length}]};return {$:'speciesConstructionContinue',heap:created.heap,state:{...state,phase:'body',receiver,call}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSpeciesConstructionState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptSpeciesConstructionTransition} */
export function resumeJavaScriptSpeciesConstructionCall(program,heap,directory,state,completion){
 if(state.operation!==undefined){const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,state);return error!==''?{$:'speciesConstructionInvalid',heap,reason:error}:adaptJavaScriptSpeciesConstruction(state,resumeJavaScriptConstructionCall(program,heap,directory,state.operation,completion));}
 const error=readJavaScriptSpeciesConstructionStateError(program,heap,directory,state);if(error!=='')return {$:'speciesConstructionInvalid',heap,reason:error};if(state.call===undefined)return {$:'speciesConstructionInvalid',heap,reason:'species constructor resumption retains its actual selected child'};
 if(readJavaScriptOrdinaryValueError(heap,completion.value)!=='')return {$:'speciesConstructionInvalid',heap,reason:'species constructor completion retains its actual shared value'};if(completion.$==='throwCompletion')return {$:'speciesConstructionThrow',heap,value:completion.value};
 if(state.phase==='prototype getter')return continueJavaScriptSpeciesPrototype(heap,directory,state,completion.value);
 if(state.phase!=='body')return {$:'speciesConstructionInvalid',heap,reason:'species constructor completion retains its actual body or prototype phase'};
 return {$:'speciesConstructionCompleted',heap,value:completion.value.$==='object'?completion.value:state.receiver};
}

/** @param {JavaScriptSpeciesConstructionState} previous @param {JavaScriptConstructionState} operation @returns {JavaScriptSpeciesConstructionState} */
export function retainJavaScriptSharedSpeciesConstruction(previous,operation){
 const {call:prior,...base}=previous;
 return {...base,model:'shared Construct species continuation 1',operation,phase:operation.phase,mode:operation.mode,prototype:operation.prototype,receiver:operation.receiver,...(operation.call===undefined?{}:{call:{$:'speciesConstructionCall',callee:operation.call.callee,receiver:operation.call.receiver,arguments:operation.call.arguments.slice()}})};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSpeciesConstructionState} state @returns {string} */
export function readJavaScriptSharedSpeciesConstructionStateError(program,heap,directory,state){
 const operation=state.operation,classification=classifyJavaScriptArraySpeciesConstructor(heap,directory,state.constructor),fact=classification.$==='arrayConstructorKnown'?classification.fact:undefined;
 if(state.model!=='shared Construct species continuation 1'||operation===undefined||fact===undefined||!fact.constructable||fact.realm!==state.fact.realm||fact.constructable!==state.fact.constructable||fact.intrinsicArray!==state.fact.intrinsicArray||fact.intrinsicKind!==state.fact.intrinsicKind||state.constructor.$!=='object'||!state.constructor.callable||state.constructor.identity!==fact.identity||!Number.isSafeInteger(state.length)||state.length<0||Object.is(state.length,-0))return 'shared species construction retains its actual immutable constructor classification and nonnegative length';
 if(!testJavaScriptSemanticSameValue(operation.target,state.constructor)||!testJavaScriptSemanticSameValue(operation.newTarget,state.constructor)||operation.arguments.length!==1||!testJavaScriptSemanticSameValue(operation.arguments[0]??{$:'undefined'},{$:'number',value:state.length})||state.phase!==operation.phase||state.mode!==operation.mode||!testJavaScriptSemanticSameValue(state.prototype,operation.prototype)||!testJavaScriptSemanticSameValue(state.receiver,operation.receiver))return 'species construction retains the exact selected target newTarget length and shared operation';
 const call=state.call,actual=operation.call;if((call===undefined)!==(actual===undefined))return 'species construction retains its actual shared child call';
 if(call!==undefined&&actual!==undefined&&(call.$!=='speciesConstructionCall'||!testJavaScriptSemanticSameValue(call.callee,actual.callee)||!testJavaScriptSemanticSameValue(call.receiver,actual.receiver)||call.arguments.length!==actual.arguments.length||call.arguments.some((value,index)=>!testJavaScriptSemanticSameValue(value,actual.arguments[index]??{$:'undefined'}))))return 'species child selection retains its actual shared callee receiver and ordered operands';
 return readJavaScriptConstructionStateError(program,heap,directory,operation);
}
/** @param {JavaScriptSpeciesConstructionState} state @param {JavaScriptConstructionTransition} transition @returns {JavaScriptSpeciesConstructionTransition} */
export function adaptJavaScriptSpeciesConstruction(state,transition){
 switch(transition.$){
  case 'constructionContinue':return {$:'speciesConstructionContinue',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state)};
  case 'constructionCall':return {$:'speciesConstructionCall',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state),callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments,constructing:transition.constructing};
  case 'constructionAllocate':return {$:'speciesConstructionAllocate',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state),prototype:transition.prototype};
  case 'constructionNeedAuthority':return {$:'speciesConstructionNeedAuthority',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state),constructor:transition.constructor};
  case 'constructionNeedImplementation':return {$:'speciesConstructionNeedImplementation',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state),constructor:transition.constructor,newTarget:transition.newTarget,arguments:transition.arguments.slice(),operation:transition.operation,reason:transition.reason};
  case 'constructionResourceLimit':return {$:'speciesConstructionResourceLimit',heap:transition.heap,state:retainJavaScriptSharedSpeciesConstruction(state,transition.state),reason:transition.reason};
  case 'constructionCompleted':return {$:'speciesConstructionCompleted',heap:transition.heap,value:transition.value};
  case 'constructionThrow':return {$:'speciesConstructionThrow',heap:transition.heap,value:transition.value};
  case 'constructionError':return {$:'speciesConstructionError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason,...(transition.realm===undefined?{}:{realm:transition.realm})};
  case 'constructionInvalid':return {$:'speciesConstructionInvalid',heap:transition.heap,reason:transition.reason};
 }
}
