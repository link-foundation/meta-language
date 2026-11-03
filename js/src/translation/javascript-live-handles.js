// Node boundary carriers. All represented property behavior belongs to the
// shared heap; the host targets only satisfy Proxy invariants and callable shape.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySetWithArrayDefinition,locateJavaScriptOrdinaryProperty,readJavaScriptOrdinaryOwnKeys,defineJavaScriptOrdinaryProperty,deleteJavaScriptOrdinaryProperty,preventJavaScriptOrdinaryExtensions,setJavaScriptOrdinaryPrototype} from './javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {testJavaScriptReflectionOrdinaryKind} from './javascript-property-reflection.js';
const native={apply:Reflect.apply,bind:Function.prototype.bind,define:Reflect.defineProperty,delete:Reflect.deleteProperty,keys:Reflect.ownKeys,descriptor:Reflect.getOwnPropertyDescriptor,prototype:Reflect.setPrototypeOf,extensible:Reflect.isExtensible,prevent:Reflect.preventExtensions,create:Object.create,Proxy};
/** Actual protected carrier creation never invokes a represented target body.
 * A constructible bound carrier has no inherent nonconfigurable own prototype;
 * method carriers are genuinely nonconstructible. Both discard mutable metadata.
 * @param {boolean} callable @param {boolean} constructable @param {boolean} array @returns {object} */
export function createJavaScriptLiveHandleCarrier(callable,constructable,array){
 let target=callable?constructable?native.apply(native.bind,function liveConstructCarrier(){},[undefined]):({liveCallCarrier(){}}).liveCallCarrier:array?[]:native.create(null);
 for(const key of native.keys(target)){const descriptor=native.descriptor(target,key);if(descriptor?.configurable&&!native.delete(target,key))throw new TypeError('a live carrier must release every configurable private property');}
 return target;
}
/** The supplied hooks own actual encoding, current heap, constructor authority
 * and portable execution. They never infer source semantics from host carriers.
 * @param {object} hooks @returns {{decode:(value:JavaScriptSemanticValue)=>unknown}} */
export function createJavaScriptLiveHandles(hooks){
 const handles=new Map();
 function decode(value){
  if(value.$!=='object')return hooks.decodePrimitive(value);
  if(handles.has(value.identity))return handles.get(value.identity);
  const error=readJavaScriptOrdinaryValueError(hooks.heap(),value);if(error!=='')return hooks.refuse(error);
  const initial=hooks.heap().objects[findJavaScriptOrdinaryObject(hooks.heap().objects,value.identity)];if(initial===undefined||!testJavaScriptReflectionOrdinaryKind(initial.$))return hooks.decodeOpaque(value);
  const constructable=value.callable?hooks.constructable(value):false,target=createJavaScriptLiveHandleCarrier(value.callable,constructable,initial.$==='arrayObject');let syncing=false;
  function current(){const heap=hooks.heap(),error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return hooks.refuse(error);const row=heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)];if(row===undefined||row.$!==initial.$||row.callable!==value.callable)return hooks.refuse('a live handle retains its actual heap identity and object kind');if(value.callable&&hooks.constructable(value)!==constructable)return hooks.refuse('a live heap cannot replace published callable constructibility');return row;}
  function descriptor(stored){const result={configurable:stored.configurable,enumerable:stored.enumerable};if(stored.valuePresent){result.value=hooks.decode(stored.value);result.writable=stored.writable;}else{result.get=hooks.decode(stored.getter);result.set=hooks.decode(stored.setter);}return result;}
  function synchronize(){
   if(syncing)return;syncing=true;
   try{
    const row=current();
    for(const key of native.keys(target)){const property=row.properties.find(property=>hooks.decodePrimitive(property.key)===key);if((property===undefined||row.extensible&&property.descriptor.configurable)&&!native.delete(target,key))return hooks.refuse('a live heap cannot remove an irreversible published own descriptor');}
    // Existing Array indices precede installation of a possibly readonly length.
    const selected=row.extensible?row.properties.filter(property=>!property.descriptor.configurable):row.properties,properties=initial.$==='arrayObject'?selected.filter(property=>hooks.decodePrimitive(property.key)!=='length').concat(selected.filter(property=>hooks.decodePrimitive(property.key)==='length')):selected;
    for(const property of properties)if(!native.define(target,hooks.decodePrimitive(property.key),descriptor(property.descriptor)))return hooks.refuse('a live heap cannot replace an irreversible published own descriptor');
    const prototype=row.prototype.$==='null'?null:hooks.decode(row.prototype);if(!native.prototype(target,prototype))return hooks.refuse('a live heap cannot replace an irreversible published prototype');
    if(!row.extensible){if(!native.prevent(target))return hooks.refuse('a live carrier must retain actual nonextensibility');}else if(!native.extensible(target))return hooks.refuse('a live heap cannot restore published extensibility');
   }finally{syncing=false;}
  }
  function commit(result){if(result.error!=='')return hooks.refuse(result.error);hooks.adoptHeap(result.heap);synchronize();return result.success;}
  function convert(proposed){
   const descriptor=createJavaScriptPropertyDescriptor('data',false,false);for(const field of ['configurable','enumerable','value','writable','getter','setter'])descriptor[field+'Present']=false;
   for(const field of ['configurable','enumerable','value','writable'])if(Object.hasOwn(proposed,field)){descriptor[field+'Present']=true;descriptor[field]=field==='value'?hooks.encode(proposed[field]):proposed[field];}
   for(const [nativeField,field]of [['get','getter'],['set','setter']])if(Object.hasOwn(proposed,nativeField)){descriptor[field+'Present']=true;descriptor[field]=hooks.encode(proposed[nativeField]);}
   return descriptor;
  }
  const proxy=new native.Proxy(target,{
   get(_target,key,receiver){current();const selectedKey=hooks.encode(key),selectedReceiver=hooks.encode(receiver),plan=planJavaScriptOrdinaryGet(hooks.heap(),value.identity,selectedKey,selectedReceiver);if(plan.error!=='')return hooks.refuse(plan.error);const result=plan.action==='value'?plan.value:plan.action==='call'?hooks.invoke('get property',value,selectedReceiver,[selectedKey]):hooks.refuse('a live Get retains its actual value or selected call');synchronize();return hooks.decode(result);},
   set(_target,key,assigned,receiver){current();const selectedKey=hooks.encode(key),selectedValue=hooks.encode(assigned),selectedReceiver=hooks.encode(receiver),plan=planJavaScriptOrdinarySetWithArrayDefinition(hooks.heap(),value.identity,selectedKey,selectedValue,selectedReceiver);if(plan.error!=='')return hooks.refuse(plan.error);if(plan.action==='call'||plan.action==='array definition'){const result=hooks.invoke('set property',value,selectedReceiver,[selectedKey,selectedValue]);if(result.$!=='boolean')return hooks.refuse('a live Set retains its actual Boolean completion');synchronize();return result.value;}if(plan.action!=='complete')return hooks.refuse('a live Set retains its actual completion or selected call');return commit(plan);},
   has(_target,key){const selectedKey=hooks.encode(key);synchronize();return locateJavaScriptOrdinaryProperty(hooks.heap(),value.identity,selectedKey).found;},
   ownKeys(){synchronize();const own=readJavaScriptOrdinaryOwnKeys(hooks.heap(),value.identity);if(own.error!=='')return hooks.refuse(own.error);return own.keys.map(hooks.decodePrimitive);},
   getOwnPropertyDescriptor(_target,key){synchronize();const row=current(),at=findJavaScriptOrdinaryProperty(row.properties,hooks.encode(key)),property=row.properties[at];return property===undefined?undefined:descriptor(property.descriptor);},
   defineProperty(_target,key,proposed){current();const selectedKey=hooks.encode(key),descriptor=convert(proposed);return commit(defineJavaScriptOrdinaryProperty(hooks.heap(),value.identity,selectedKey,descriptor));},
   deleteProperty(_target,key){current();const selectedKey=hooks.encode(key);return commit(deleteJavaScriptOrdinaryProperty(hooks.heap(),value.identity,selectedKey));},
   getPrototypeOf(){synchronize();const prototype=current().prototype;return prototype.$==='null'?null:hooks.decode(prototype);},
   setPrototypeOf(_target,prototype){current();const selected=hooks.encode(prototype);return commit(setJavaScriptOrdinaryPrototype(hooks.heap(),value.identity,selected));},
   isExtensible(){synchronize();return current().extensible;},
   preventExtensions(){current();return commit(preventJavaScriptOrdinaryExtensions(hooks.heap(),value.identity));},
   apply(_target,receiver,arguments_){current();return hooks.decode(hooks.invoke('call',value,hooks.encode(receiver),arguments_.map(hooks.encode)));},
   construct(_target,arguments_,newTarget){current();return hooks.decode(hooks.invoke('construct',value,hooks.encode(newTarget),arguments_.map(hooks.encode)));}
  });
  // Publish before decoding descriptors/prototypes so self references and cycles
  // retain one handle, including self-referential readonly property values.
  handles.set(value.identity,proxy);hooks.publish(proxy,value);return proxy;
 }
 return {decode};
}
