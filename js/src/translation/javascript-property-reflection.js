// FromPropertyDescriptor and own-key reflection construct genuine heap values.
// Neither selected accessors nor represented host Object/Reflect bodies run.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,readJavaScriptOrdinaryKeyError,readJavaScriptOrdinaryOwnKeys,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptArrayIndex,testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
import {createJavaScriptPropertyDescriptor,readJavaScriptPropertyDescriptorKind} from './javascript-property-descriptors.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from './javascript-array-transform.js';
/** @typedef {{$:'propertyReflection',operation:string,input:JavaScriptSemanticValue,key:JavaScriptSemanticValue,objectPrototype:JavaScriptSemanticValue,arrayPrototype:JavaScriptSemanticValue,phase:string,keys:JavaScriptSemanticValue[],position:number,fields:JavaScriptOrdinaryProperty[],field:number,target:JavaScriptSemanticValue,descriptor:JavaScriptSemanticValue}} JavaScriptPropertyReflectionState */
/** @typedef {{$:'propertyReflectionContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyReflectionState} |
 * {$:'propertyReflectionAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyReflectionState,prototype:JavaScriptSemanticValue} |
 * {$:'propertyReflectionCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'propertyReflectionError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string} |
 * {$:'propertyReflectionInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'propertyReflectionResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptPropertyReflectionState,reason:string}} JavaScriptPropertyReflectionTransition */
/** @param {string} operation @returns {boolean} */
export function testJavaScriptPropertyReflectionOperation(operation){return ['Object.getOwnPropertyDescriptor','Object.getOwnPropertyDescriptors','Object.getOwnPropertyNames','Object.getOwnPropertySymbols','Reflect.ownKeys','Reflect.getOwnPropertyDescriptor'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptSingleDescriptorOperation(operation){return operation==='Object.getOwnPropertyDescriptor'||operation==='Reflect.getOwnPropertyDescriptor';}
/** @param {string} kind @returns {boolean} */
export function testJavaScriptReflectionOrdinaryKind(kind){return ['ordinaryObject','closureObject','boundFunctionObject','arrayObject','mapObject','setObject','weakMapObject','weakSetObject','collectionIteratorObject'].includes(kind);}
/** @param {string} value @returns {JavaScriptSemanticValue} */
export function constructJavaScriptReflectionText(value){const units=[];for(let at=0;at<value.length;at++)units.push(value.charCodeAt(at));return {$:'text',units};}
/** @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value @returns {JavaScriptOrdinaryProperty} */
export function constructJavaScriptReflectionProperty(key,value){const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=value;descriptor.writable=true;return {$:'ordinaryProperty',key,descriptor};}
/** @param {JavaScriptPropertyDescriptor} descriptor @returns {JavaScriptOrdinaryProperty[]} */
export function constructJavaScriptReflectedDescriptorFields(descriptor){
 const fields=[];if(readJavaScriptPropertyDescriptorKind(descriptor)==='data')fields.push(constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('value'),descriptor.value),constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('writable'),{$:'boolean',value:descriptor.writable}));
 else fields.push(constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('get'),descriptor.getter),constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('set'),descriptor.setter));
 fields.push(constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('enumerable'),{$:'boolean',value:descriptor.enumerable}),constructJavaScriptReflectionProperty(constructJavaScriptReflectionText('configurable'),{$:'boolean',value:descriptor.configurable}));return fields;
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} key @returns {JavaScriptOrdinaryProperty|undefined} */
export function readJavaScriptReflectedOwnProperty(heap,input,key){
 if(input.$==='object'){const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)];return object?.properties[findJavaScriptOrdinaryProperty(object.properties,key)];}
 if(input.$!=='text'&&input.$!=='sourceText')return undefined;
 const length=readJavaScriptSemanticTextLength(input),index=readJavaScriptArrayIndex(key);let value={$:'undefined'},enumerable=false;
 if(testJavaScriptArrayLengthKey(key))value={$:'number',value:length};else if(index>=0&&index<length){value={$:'text',units:[readJavaScriptSemanticTextUnit(input,index)]};enumerable=true;}else return undefined;
 const descriptor=createJavaScriptPropertyDescriptor('data',false,enumerable);descriptor.value=value;descriptor.writable=false;return {$:'ordinaryProperty',key,descriptor};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyReflectionState} state @returns {string} */
export function readJavaScriptPropertyReflectionStateError(heap,state){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return error;
 if(!testJavaScriptPropertyReflectionOperation(state.operation)||!['keys','target allocation','select','descriptor allocation','descriptor write','publish','array write'].includes(state.phase)||state.keys.length>4096||!Number.isSafeInteger(state.position)||Object.is(state.position,-0)||state.position<0||state.position>state.keys.length||!Number.isSafeInteger(state.field)||Object.is(state.field,-0)||state.field<0||state.field>state.fields.length||state.fields.length>4)return 'property reflection retains its actual operation phase and bounded write positions';
 for(const value of [state.input,state.key,state.objectPrototype,state.arrayPrototype,state.target,state.descriptor,...state.fields.map(field=>field.descriptor.value)]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return invalid;}
 if(state.objectPrototype.$!=='object'||state.arrayPrototype.$!=='object'||state.input.$==='null'||state.input.$==='undefined'||state.operation.startsWith('Reflect.')&&state.input.$!=='object')return 'reflection retains its actual coerced input and selected realm prototypes';
 if(state.input.$==='object'){const owner=heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.input.identity)];if(owner===undefined||!testJavaScriptReflectionOrdinaryKind(owner.$))return 'opaque reflection requires actual own-key and descriptor protocols';}
 const seen=[];for(const key of state.keys){const invalid=readJavaScriptOrdinaryKeyError(heap,key);if(invalid!=='')return invalid;if(seen.some(prior=>testJavaScriptSemanticSameValue(prior,key)))return 'reflection retains distinct actual own keys';seen.push(key);}
 if(testJavaScriptSingleDescriptorOperation(state.operation)){const invalid=readJavaScriptOrdinaryKeyError(heap,state.key);if(invalid!=='')return invalid;}
 if((state.phase==='descriptor allocation'||state.phase==='descriptor write'||state.phase==='publish')&&state.fields.length!==4)return 'FromPropertyDescriptor retains its four selected actual fields';
 if(state.phase==='descriptor write'&&state.descriptor.$!=='object'||state.phase==='publish'&&state.descriptor.$!=='object'||state.phase==='array write'&&state.target.$!=='object')return 'reflection writes retain actual allocated result identities';return '';
}
/** @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} input @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} objectPrototype @param {JavaScriptSemanticValue} arrayPrototype @returns {JavaScriptPropertyReflectionTransition} */
export function beginJavaScriptPropertyReflection(heap,operation,input,key,objectPrototype,arrayPrototype){
 const invalid=readJavaScriptOrdinaryHeapError(heap)||readJavaScriptOrdinaryValueError(heap,input);if(invalid!=='')return {$:'propertyReflectionInvalid',heap,reason:invalid};
 if(input.$==='null'||input.$==='undefined'||operation.startsWith('Reflect.')&&input.$!=='object')return {$:'propertyReflectionError',heap,errorClass:'TypeError',reason:'Property reflection requires its actual nonnull Object input'};
 const state={$:'propertyReflection',operation,input,key,objectPrototype,arrayPrototype,phase:'keys',keys:[],position:0,fields:[],field:0,target:{$:'undefined'},descriptor:{$:'undefined'}},error=readJavaScriptPropertyReflectionStateError(heap,state);return error===''?{$:'propertyReflectionContinue',heap,state}:{$:'propertyReflectionInvalid',heap,reason:error};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} key @returns {boolean} */
export function testJavaScriptReflectionPropertyBudget(heap,target,key){
 if(target.$!=='object')return false;const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)];if(object===undefined)return false;if(findJavaScriptOrdinaryProperty(object.properties,key)>=0)return true;let total=0;for(const row of heap.objects)total+=row.properties.length;return object.properties.length<4096&&total<16384;
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyReflectionState} state @returns {JavaScriptPropertyReflectionTransition} */
export function stepJavaScriptPropertyReflection(heap,state){
 const error=readJavaScriptPropertyReflectionStateError(heap,state);if(error!=='')return {$:'propertyReflectionInvalid',heap,reason:error};
 const single=testJavaScriptSingleDescriptorOperation(state.operation),plural=state.operation==='Object.getOwnPropertyDescriptors';
 if(state.phase==='keys'){
  let keys=[];if(single)keys=[state.key];else if(state.input.$==='object'){const own=readJavaScriptOrdinaryOwnKeys(heap,state.input.identity);if(own.error!=='')return {$:'propertyReflectionInvalid',heap,reason:own.error};keys=own.keys;}
  else if(state.input.$==='text'||state.input.$==='sourceText'){const length=readJavaScriptSemanticTextLength(state.input);if(length>=4096)return {$:'propertyReflectionResourceLimit',heap,state,reason:'String reflection exceeds its actual own-key snapshot budget'};for(let at=0;at<length;at++)keys.push(constructJavaScriptReflectionText(String(at)));keys.push(constructJavaScriptReflectionText('length'));}
  if(state.operation==='Object.getOwnPropertyNames')keys=keys.filter(key=>key.$!=='symbol');if(state.operation==='Object.getOwnPropertySymbols')keys=keys.filter(key=>key.$==='symbol');
  if(keys.length>4096)return {$:'propertyReflectionResourceLimit',heap,state,reason:'reflection exceeds its actual own-key snapshot budget'};
  return {$:'propertyReflectionContinue',heap,state:{...state,keys,phase:single?'select':'target allocation'}};
 }
 if(state.phase==='target allocation'||state.phase==='descriptor allocation')return {$:'propertyReflectionAllocate',heap,state,prototype:state.phase==='descriptor allocation'||plural?state.objectPrototype:state.arrayPrototype};
 if(state.phase==='array write'){
  if(state.position===state.keys.length)return {$:'propertyReflectionCompleted',heap,value:state.target};const key=state.keys[state.position];if(key===undefined)return {$:'propertyReflectionInvalid',heap,reason:'reflection retains its actual selected key'};
  const write=createJavaScriptIndexedDataProperty(heap,state.target,state.position,key);if(write.$!=='indexedPropertyCreated')return write.$==='indexedPropertyResourceLimit'?{$:'propertyReflectionResourceLimit',heap:write.heap,state,reason:write.reason}:{$:'propertyReflectionInvalid',heap:write.heap,reason:write.reason};return {$:'propertyReflectionContinue',heap:write.heap,state:{...state,position:state.position+1}};
 }
 if(state.phase==='select'){
  if(state.position===state.keys.length)return {$:'propertyReflectionCompleted',heap,value:single?{$:'undefined'}:state.target};
  const key=state.keys[state.position];if(key===undefined)return {$:'propertyReflectionInvalid',heap,reason:'reflection retains its selected own key'};const property=readJavaScriptReflectedOwnProperty(heap,state.input,key);
  if(property===undefined)return {$:'propertyReflectionContinue',heap,state:{...state,position:state.position+1}};
  return {$:'propertyReflectionContinue',heap,state:{...state,phase:'descriptor allocation',fields:constructJavaScriptReflectedDescriptorFields(property.descriptor),field:0}};
 }
 if(state.phase==='descriptor write'){
  if(state.field===state.fields.length)return single?{$:'propertyReflectionCompleted',heap,value:state.descriptor}:{$:'propertyReflectionContinue',heap,state:{...state,phase:'publish'}};
  const field=state.fields[state.field];if(field===undefined||state.descriptor.$!=='object')return {$:'propertyReflectionInvalid',heap,reason:'reflection retains its actual descriptor field and result'};
  if(!testJavaScriptReflectionPropertyBudget(heap,state.descriptor,field.key))return {$:'propertyReflectionResourceLimit',heap,state,reason:'reflected descriptor fields exceed the actual heap property budget'};
  const write=defineJavaScriptOrdinaryProperty(heap,state.descriptor.identity,field.key,field.descriptor);if(write.error!==''||!write.success)return {$:'propertyReflectionInvalid',heap:write.heap,reason:write.error||'fresh reflected descriptor field must accept its data definition'};return {$:'propertyReflectionContinue',heap:write.heap,state:{...state,field:state.field+1}};
 }
 if(state.phase==='publish'){
  const key=state.keys[state.position];if(key===undefined||state.target.$!=='object')return {$:'propertyReflectionInvalid',heap,reason:'reflection retains its actual descriptor publication key and result'};
  if(!testJavaScriptReflectionPropertyBudget(heap,state.target,key))return {$:'propertyReflectionResourceLimit',heap,state,reason:'reflected descriptor publication exceeds the actual heap property budget'};
  const field=constructJavaScriptReflectionProperty(key,state.descriptor),write=defineJavaScriptOrdinaryProperty(heap,state.target.identity,key,field.descriptor);if(write.error!==''||!write.success)return {$:'propertyReflectionInvalid',heap:write.heap,reason:write.error||'fresh descriptor result must accept its own data property'};
  return {$:'propertyReflectionContinue',heap:write.heap,state:{...state,phase:'select',position:state.position+1,fields:[],field:0,descriptor:{$:'undefined'}}};
 }
 return {$:'propertyReflectionInvalid',heap,reason:'reflection continuation retains its actual phase'};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyReflectionState} state @param {bigint} identity @returns {JavaScriptPropertyReflectionTransition} */
export function resumeJavaScriptPropertyReflectionAllocation(heap,state,identity){
 const error=readJavaScriptPropertyReflectionStateError(heap,state);if(error!==''||state.phase!=='target allocation'&&state.phase!=='descriptor allocation')return {$:'propertyReflectionInvalid',heap,reason:error||'reflection allocation retains its actual pending phase'};
 const ordinary=state.phase==='descriptor allocation'||state.operation==='Object.getOwnPropertyDescriptors';let nextHeap=heap,value={$:'undefined'};
 if(ordinary){if(heap.objects.length>=4096)return {$:'propertyReflectionResourceLimit',heap,state,reason:'reflected descriptor allocation exceeds the actual Object identity budget'};const allocated=insertJavaScriptOrdinaryObject(heap,identity,state.objectPrototype);if(allocated.error!=='')return {$:'propertyReflectionInvalid',heap:allocated.heap,reason:allocated.error};nextHeap=allocated.heap;value={$:'object',identity,callable:false};}
 else {const allocated=allocateJavaScriptArray(heap,identity,state.arrayPrototype,state.keys.length);if(allocated.$==='arrayResourceLimit')return {$:'propertyReflectionResourceLimit',heap:allocated.heap,state,reason:allocated.reason};if(allocated.$!=='arrayNormal')return {$:'propertyReflectionInvalid',heap:allocated.heap,reason:'reflection allocation requires its actual fresh Array'};nextHeap=allocated.heap;value=allocated.value;}
 return {$:'propertyReflectionContinue',heap:nextHeap,state:state.phase==='descriptor allocation'?{...state,phase:'descriptor write',descriptor:value}:{...state,phase:ordinary?'select':'array write',target:value}};
}
