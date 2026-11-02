import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment,readJavaScriptArrayAssignmentStateError} from './javascript-array-assignment.js';
import {testJavaScriptReflectionPropertyBudget} from './javascript-property-reflection.js';
// CopyDataProperties for actual object-literal targets. Getters execute through
// the common call machine; snapshots never substitute original host bodies.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,readJavaScriptOrdinaryOwnKeys,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextString,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
/** @typedef {{$:'objectCopyCall',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} JavaScriptObjectCopyCall */
/** @typedef {{$:'objectSpreadState',target:JavaScriptSemanticValue,input:JavaScriptSemanticValue,excluded:JavaScriptSemanticValue[],keys:JavaScriptSemanticValue[],position:number,phase:string,value:JavaScriptSemanticValue,callee:JavaScriptSemanticValue,operation?:string,exoticKey?:JavaScriptSemanticValue,assignment?:JavaScriptArrayAssignmentState,call?:JavaScriptObjectCopyCall}} JavaScriptObjectSpreadState */
/** @typedef {{$:'objectSpreadContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState} |
 * {$:'objectSpreadCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]} |
 * {$:'objectSpreadCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectSpreadThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'objectSpreadError',heap:JavaScriptOrdinaryHeap,reason:string,errorClass?:string} |
 * {$:'objectSpreadInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'objectSpreadResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptObjectSpreadState,reason:string}} JavaScriptObjectSpreadTransition */
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state @returns {string} */
export function readJavaScriptObjectSpreadStateError(heap,state){
 const heapError=readJavaScriptOrdinaryHeapError(heap);if(heapError!=='')return heapError;
 for(const value of [state.target,state.input,state.value,state.callee]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 if(state.target.$!=='object')return 'object spread retains its actual ordinary literal target';
 const assignment=state.operation==='assignment';if(state.operation!==undefined&&!assignment)return 'object copy retains its actual CreateDataProperty or Set operation';
 const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];if(target===undefined||(assignment?!['ordinaryObject','closureObject','arrayObject','mapObject','setObject','collectionIteratorObject'].includes(target.$):target.$!=='ordinaryObject'||target.callable))return 'object copy target retains its actual supported ordinary heap kind';
 if(assignment&&(state.exoticKey?.$!=='symbol'||readJavaScriptOrdinaryValueError(heap,state.exoticKey)!==''))return 'Object.assign retains its actual primitive conversion Symbol';
 if(!['keys','read','getter','write','setter','array definition'].includes(state.phase)||!Number.isSafeInteger(state.position)||state.position<0||Object.is(state.position,-0)||state.position>state.keys.length||state.keys.length>4096)return 'object spread retains its bounded key snapshot and exact cursor';
 if(state.phase==='keys'&&(state.keys.length!==0||state.position!==0)||['getter','write','setter','array definition'].includes(state.phase)&&state.position>=state.keys.length)return 'object spread phases retain their actual selected property';
 if(!['write','setter','array definition'].includes(state.phase)&&state.value.$!=='undefined'||!['getter','setter','array definition'].includes(state.phase)&&state.callee.$!=='undefined')return 'object spread retains a completed value or getter only in its actual phase';
 if(state.phase==='getter'&&(state.input.$!=='object'||state.callee.$!=='object'||!state.callee.callable))return 'object spread getter suspension retains its actual callable and receiver';
 if((state.phase==='array definition')!==(state.assignment!==undefined)||!assignment&&['setter','array definition'].includes(state.phase))return 'Object.assign retains only its actual nested assignment phase';
 if(state.assignment!==undefined){const invalid=readJavaScriptArrayAssignmentStateError(heap,state.assignment);if(invalid!=='')return invalid;if(state.target.$!=='object'||state.assignment.definition.identity!==state.target.identity||!testJavaScriptSemanticSameValue(state.assignment.exoticKey,state.exoticKey))return 'Object.assign Array definition retains actual target and Symbol';}
 if(state.call!==undefined){const call=state.call;for(const value of [call.callee,call.receiver,...call.arguments]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}if(!['getter','setter','array definition'].includes(state.phase)||!testJavaScriptSemanticSameValue(state.callee,call.callee))return 'object copy retains its actual selected call phase';if(state.phase==='setter'&&(!testJavaScriptSemanticSameValue(call.receiver,state.target)||call.arguments.length!==1||!testJavaScriptSemanticSameValue(call.arguments[0],state.value)))return 'Object.assign retains actual setter receiver and held value';}
 if(state.assignment!==undefined&&!testJavaScriptSemanticSameValue(state.assignment.definition.key,state.keys[state.position]))return 'Object.assign Array definition retains its selected actual property key';
 if(state.phase==='setter'&&state.call===undefined)return 'Object.assign setter retains its actual selected call';
 if(state.phase!=='keys'&&state.input.$!=='object'){
  const length=state.input.$==='text'||state.input.$==='sourceText'?readJavaScriptSemanticTextLength(state.input):0;
  if(length!==state.keys.length)return 'primitive spread snapshots retain all actual String indices and no invented keys';
  for(let at=0;at<state.keys.length;at++){const key=state.keys[at];if(key===undefined||key.$!=='text'||readJavaScriptSemanticTextString(key)!==String(at))return 'String spread snapshots retain their exact ordered UTF16 indices';}
 }
 if(state.excluded.length>4096)return 'object spread excluded keys exceed their explicit snapshot budget';
 for(const key of state.excluded){const error=readJavaScriptOrdinaryKeyError(heap,key);if(error!=='')return error;}
 const names=[];for(const key of state.keys){const error=readJavaScriptOrdinaryKeyError(heap,key);if(error!=='')return error;names.push(key.$==='symbol'?'symbol:'+String(key.identity):'text:'+readJavaScriptSemanticTextString(key));}
 const ordered=names.toSorted();for(let at=1;at<ordered.length;at++)if(ordered[at]===ordered[at-1])return 'object spread own-key snapshots cannot duplicate a semantic key';
 return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target
 * @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue[]} [excluded] @returns {JavaScriptObjectSpreadTransition} */
export function beginJavaScriptObjectSpread(heap,target,input,excluded=[]){
 const state={$:'objectSpreadState',target,input,excluded,keys:[],position:0,phase:'keys',value:{$:'undefined'},callee:{$:'undefined'}};
 const error=readJavaScriptObjectSpreadStateError(heap,state);return error===''?{$:'objectSpreadContinue',heap,state}:{$:'objectSpreadInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state
 * @returns {JavaScriptObjectSpreadTransition} */
export function stepJavaScriptObjectSpread(heap,state){
 const error=readJavaScriptObjectSpreadStateError(heap,state);if(error!=='')return {$:'objectSpreadInvalid',heap,reason:error};
 if(state.phase==='keys'){
  let keys=[];
  if(state.input.$==='object'){
   const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'objectSpreadInvalid',heap,reason:own.error};keys=own.keys;
  }else if(state.input.$==='text'||state.input.$==='sourceText'){
   const length=readJavaScriptSemanticTextLength(state.input);if(length>4096)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread String keys exceed their explicit snapshot budget'};
   for(let at=0;at<length;at++){const key=String(at),units=[];for(let unit=0;unit<key.length;unit++)units.push(key.charCodeAt(unit));keys.push({$:'text',units});}
  }
  if(keys.length>4096)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread own keys exceed their explicit snapshot budget'};
  return {$:'objectSpreadContinue',heap,state:{...state,keys,phase:'read'}};
 }
 if(state.call!==undefined)return {$:'objectSpreadCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments};
 if(state.phase==='array definition'&&state.assignment!==undefined)return acceptJavaScriptObjectAssignmentArray(heap,state,stepJavaScriptArrayAssignment(heap,state.assignment));
 if(state.phase==='getter')return {$:'objectSpreadInvalid',heap,reason:'an object spread getter needs its actual child completion'};
 if(state.position===state.keys.length)return {$:'objectSpreadCompleted',heap,value:state.target};
 const key=state.keys[state.position];if(key===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread requires its actual selected key'};
 if(state.phase==='write'&&state.operation==='assignment')return writeJavaScriptObjectAssignmentProperty(heap,state,key);
 if(state.phase==='write'){
  if(state.target.$!=='object')return {$:'objectSpreadInvalid',heap,reason:'object spread retains its actual ordinary target'};
  const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.target.identity)];if(target===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread target cannot disappear from its global context'};
  if(findJavaScriptOrdinaryProperty(target.properties,key)<0){if(!target.extensible)return {$:'objectSpreadError',heap,reason:'object spread cannot create its selected own data property'};let count=0;for(const object of heap.objects)count+=object.properties.length;if(target.properties.length>=4096||count>=16384)return {$:'objectSpreadResourceLimit',heap,state,reason:'object spread properties exceed their explicit ordered storage budget'};}
  const defined=defineJavaScriptOrdinaryProperty(heap,state.target.identity,key,createJavaScriptOrdinarySetDescriptor(state.value,true));
  if(defined.error!=='')return {$:'objectSpreadInvalid',heap:defined.heap,reason:defined.error};
  if(!defined.success)return {$:'objectSpreadError',heap:defined.heap,reason:'object spread cannot create its selected own data property'};
  return {$:'objectSpreadContinue',heap:defined.heap,state:{...state,position:state.position+1,phase:'read',value:{$:'undefined'}}};
 }
 if(state.excluded.some(excluded=>testJavaScriptSemanticSameValue(excluded,key)))return {$:'objectSpreadContinue',heap,state:{...state,position:state.position+1}};
 if(state.input.$==='object'){
  const input=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)];if(input===undefined)return {$:'objectSpreadInvalid',heap,reason:'object spread source cannot disappear from its global context'};
  if(!['ordinaryObject','closureObject','mapObject','setObject','collectionIteratorObject','arrayObject'].includes(input.$))return {$:'objectSpreadInvalid',heap,reason:'opaque object spread requires its actual own-key and descriptor host protocols'};
  const property=input.properties[findJavaScriptOrdinaryProperty(input.properties,key)];
  // A preceding getter can delete or hide this key. New keys are not in the
  // original snapshot; a recreated snapshotted key uses its latest descriptor.
  if(property===undefined||!property.descriptor.enumerable)return {$:'objectSpreadContinue',heap,state:{...state,position:state.position+1}};
  const get=planJavaScriptOrdinaryGet(heap,state.input.identity,key,state.input);if(get.error!=='')return {$:'objectSpreadInvalid',heap,reason:get.error};
  if(get.action==='call'){const call={$:'objectCopyCall',callee:get.callee,receiver:get.receiver,arguments:get.arguments};return {$:'objectSpreadCall',heap,state:{...state,phase:'getter',callee:get.callee,...(state.operation==='assignment'?{call}:{})},callee:get.callee,receiver:get.receiver,arguments:get.arguments};}
  if(get.action!=='value')return {$:'objectSpreadInvalid',heap,reason:'object spread Get requires its actual value or accessor continuation'};
  return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:get.value}};
 }
 if(state.input.$==='text'||state.input.$==='sourceText'){
  return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:{$:'text',units:[readJavaScriptSemanticTextUnit(state.input,state.position)]}}};
 }
 return {$:'objectSpreadInvalid',heap,reason:'a primitive without own keys cannot retain selected spread properties'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state
 * @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptObjectSpreadTransition} */
export function resumeJavaScriptObjectSpread(heap,state,completion){
 const error=readJavaScriptObjectSpreadStateError(heap,state);if(error!=='')return {$:'objectSpreadInvalid',heap,reason:error};
 if(state.phase!=='getter'&&state.phase!=='setter'&&state.phase!=='array definition')return {$:'objectSpreadInvalid',heap,reason:'object copy resumes only an actual selected child call'};
 const valueError=readJavaScriptOrdinaryValueError(heap,completion.value);if(valueError!=='')return {$:'objectSpreadInvalid',heap,reason:valueError};
 if(completion.$==='throwCompletion')return {$:'objectSpreadThrow',heap,value:completion.value};
 if(state.phase==='setter')return finishJavaScriptObjectAssignmentProperty(heap,state);
 if(state.phase==='array definition'&&state.assignment!==undefined)return acceptJavaScriptObjectAssignmentArray(heap,state,resumeJavaScriptArrayAssignment(heap,state.assignment,completion.$==='normalCompletion'?{$:'normalCompletion',value:completion.value}:{$:'throwCompletion',value:completion.value}));
 return {$:'objectSpreadContinue',heap,state:{...state,phase:'write',value:completion.value,callee:{$:'undefined'},call:undefined}};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} exoticKey @returns {JavaScriptObjectSpreadTransition} */
export function beginJavaScriptObjectAssignment(heap,target,input,exoticKey){
 const state={$:'objectSpreadState',target,input,excluded:[],keys:[],position:0,phase:'keys',value:{$:'undefined'},callee:{$:'undefined'},operation:'assignment',exoticKey},invalid=readJavaScriptObjectSpreadStateError(heap,state);return invalid===''?{$:'objectSpreadContinue',heap,state}:{$:'objectSpreadInvalid',heap,reason:invalid};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state @returns {JavaScriptObjectSpreadTransition} */
export function finishJavaScriptObjectAssignmentProperty(heap,state){return {$:'objectSpreadContinue',heap,state:{...state,phase:'read',position:state.position+1,value:{$:'undefined'},callee:{$:'undefined'},call:undefined,assignment:undefined}};}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state @param {JavaScriptArrayAssignmentTransition} transition @returns {JavaScriptObjectSpreadTransition} */
export function acceptJavaScriptObjectAssignmentArray(heap,state,transition){
 switch(transition.$){
  case 'arrayAssignmentContinue':return {$:'objectSpreadContinue',heap:transition.heap,state:{...state,phase:'array definition',assignment:transition.state,callee:{$:'undefined'},call:undefined}};
  case 'arrayAssignmentCall':{const call={$:'objectCopyCall',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments};return {$:'objectSpreadCall',heap:transition.heap,state:{...state,phase:'array definition',assignment:transition.state,callee:call.callee,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
  case 'arrayAssignmentCompleted':return transition.success?finishJavaScriptObjectAssignmentProperty(transition.heap,state):{$:'objectSpreadError',heap:transition.heap,reason:'Object.assign cannot set its selected Array property'};
  case 'arrayAssignmentError':return {$:'objectSpreadError',heap:transition.heap,errorClass:transition.errorClass,reason:transition.reason};
  case 'arrayAssignmentThrow':return {$:'objectSpreadThrow',heap:transition.heap,value:transition.value};
  case 'arrayAssignmentInvalid':return {$:'objectSpreadInvalid',heap:transition.heap,reason:transition.reason};
  case 'arrayAssignmentResourceLimit':return {$:'objectSpreadResourceLimit',heap:transition.heap,state:{...state,phase:'array definition',assignment:transition.state,callee:{$:'undefined'},call:undefined},reason:transition.reason};
 }
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptObjectSpreadState} state @param {JavaScriptSemanticValue} key @returns {JavaScriptObjectSpreadTransition} */
export function writeJavaScriptObjectAssignmentProperty(heap,state,key){
 if(state.target.$!=='object'||state.exoticKey===undefined)return {$:'objectSpreadInvalid',heap,reason:'Object.assign retains actual target and primitive conversion authority'};
 const plan=planJavaScriptOrdinarySetWithArrayDefinition(heap,state.target.identity,key,state.value,state.target);if(plan.error!=='')return {$:'objectSpreadInvalid',heap:plan.heap,reason:plan.error};
 if(plan.action==='array definition'&&plan.definition!==undefined)return acceptJavaScriptObjectAssignmentArray(plan.heap,state,beginJavaScriptArrayAssignment(plan.heap,state.target.identity,key,plan.definition,state.exoticKey));
 if(plan.action==='call'){const call={$:'objectCopyCall',callee:plan.callee,receiver:plan.receiver,arguments:plan.arguments};return {$:'objectSpreadCall',heap:plan.heap,state:{...state,phase:'setter',callee:call.callee,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments};}
 if(plan.action!=='complete')return {$:'objectSpreadInvalid',heap:plan.heap,reason:'Object.assign Set retains its actual ordinary result or selected setter'};
 if(!plan.success)return {$:'objectSpreadError',heap:plan.heap,reason:'Object.assign cannot set its selected property'};
 if(!testJavaScriptReflectionPropertyBudget(heap,state.target,key))return {$:'objectSpreadResourceLimit',heap,state,reason:'Object.assign exceeds the actual heap property budget'};
 return finishJavaScriptObjectAssignmentProperty(plan.heap,state);
}
