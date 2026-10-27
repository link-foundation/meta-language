// Deferred independent ordinary-object observations. Never import in authors.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {decodeValue,encodeValue,decodeDescriptor,encodeDescriptor,assertVersions} from './controls.mjs';
import {readJavaScriptOrdinaryHeapError,planJavaScriptOrdinaryGet,planJavaScriptOrdinarySet,defineJavaScriptOrdinaryProperty,deleteJavaScriptOrdinaryProperty,
 preventJavaScriptOrdinaryExtensions,setJavaScriptOrdinaryPrototype,readJavaScriptOrdinaryOwnKeys,insertJavaScriptOrdinaryObject,registerJavaScriptOrdinarySymbol} from '../../js/src/translation/javascript-ordinary-heap.js';
export const heapFixture=readProjectData(new URL('../../parity/fixtures/javascript-ordinary-heap.lino',import.meta.url));
export {assertVersions};
export function decodeHeap(heap){return {...heap,objects:heap.objects.map(object=>({...object,identity:BigInt(object.identity),prototype:decodeValue(object.prototype),properties:object.properties.map(property=>({...property,key:decodeValue(property.key),descriptor:decodeDescriptor(property.descriptor)}))})),symbols:heap.symbols.map(BigInt)};}
export function encodeHeap(heap){return {...heap,objects:heap.objects.map(object=>({...object,identity:String(object.identity),prototype:encodeValue(object.prototype),properties:object.properties.map(property=>({...property,key:encodeValue(property.key),descriptor:encodeDescriptor(property.descriptor)}))})),symbols:heap.symbols.map(String)};}
export function createOrdinaryHostContext(heap,onCall){
 const objects=new Map(),symbols=new Map(),reverse=new Map(),calls=[];
 const materialize=value=>{switch(value.$){case 'undefined':return undefined;case 'null':return null;case 'boolean':case 'number':case 'integer':return value.value;
  case 'text':{let output='';for(let at=0;at<value.units.length;at+=4096)output+=String.fromCharCode(...value.units.slice(at,at+4096));return output;}
  case 'object':assert.ok(objects.has(String(value.identity)));return objects.get(String(value.identity));
  case 'symbol':assert.ok(symbols.has(String(value.identity)));return symbols.get(String(value.identity));
  default:throw new TypeError('uncovered semantic value');}};
 const semantic=value=>{if(value===undefined)return {$:'undefined'};if(value===null)return {$:'null'};
  if(typeof value==='boolean')return {$:'boolean',value};if(typeof value==='number')return {$:'number',value};if(typeof value==='bigint')return {$:'integer',value};
  if(typeof value==='string')return {$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))};
  const identity=reverse.get(value);assert.notEqual(identity,undefined,'every host reference belongs to the actual semantic directory');
  return typeof value==='symbol'?{$:'symbol',identity:BigInt(identity)}:{$:'object',identity:BigInt(identity),callable:typeof value==='function'};};
 const addObject=object=>{const identity=String(object.identity);
  // A nonconstructible method has configurable own name/length and no implicit
  // prototype/arguments/caller properties. Remove those two configurable fields
  // before comparing the complete explicitly represented ordinary own storage.
  const value=object.callable?({invoke(...arguments_){const call={callee:{$:'object',identity:BigInt(identity),callable:true},receiver:semantic(this),arguments:arguments_.map(semantic)};calls.push(call);return onCall?onCall(call):41;}}).invoke:Object.create(null);
  if(object.callable){assert.equal(Reflect.deleteProperty(value,'name'),true);assert.equal(Reflect.deleteProperty(value,'length'),true);}
  objects.set(identity,value);reverse.set(value,identity);return value;};
 for(const object of heap.objects)addObject(object);
 const addSymbol=identity=>{const symbol=Symbol(String(identity));symbols.set(String(identity),symbol);reverse.set(symbol,String(identity));};
 for(const identity of heap.symbols)addSymbol(identity);
 const descriptor=value=>{const result={};for(const [field,key]of [['configurable','configurable'],['enumerable','enumerable'],['value','value'],['writable','writable'],['getter','get'],['setter','set']])if(value[field+'Present'])result[key]=['value','getter','setter'].includes(field)?materialize(value[field]):value[field];return result;};
 for(const object of heap.objects){const host=objects.get(String(object.identity));assert.equal(Reflect.setPrototypeOf(host,materialize(object.prototype)),true);for(const property of object.properties)Object.defineProperty(host,materialize(property.key),descriptor(property.descriptor));}
 for(const object of heap.objects)if(!object.extensible)Object.preventExtensions(objects.get(String(object.identity)));
 return {objects,symbols,calls,materialize,semantic,descriptor,addObject,addSymbol};
}
export function assertOrdinaryHostSnapshot(heap,context){
 assert.equal(heap.objects.length,context.objects.size);assert.equal(heap.symbols.length,context.symbols.size);
 for(const object of heap.objects){const host=context.objects.get(String(object.identity));assert.ok(host);assert.equal(Object.isExtensible(host),object.extensible);assert.equal(typeof host==='function',object.callable);assert.ok(Object.is(Object.getPrototypeOf(host),context.materialize(object.prototype)));
  const order=readJavaScriptOrdinaryOwnKeys(heap,object.identity);assert.equal(order.error,'');assert.deepEqual(Reflect.ownKeys(host),order.keys.map(context.materialize));
  assert.equal(Reflect.ownKeys(host).length,object.properties.length);
  for(const property of object.properties){const actual=Object.getOwnPropertyDescriptor(host,context.materialize(property.key)),expected=context.descriptor(property.descriptor);assert.deepEqual(Reflect.ownKeys(actual).sort(),Reflect.ownKeys(expected).sort());for(const key of Reflect.ownKeys(expected))assert.ok(Object.is(actual[key],expected[key]),'complete heap descriptor attribute '+key);}
 }
}
export function observeHeapCase(row){
 const heap=decodeHeap(row.heap),identity=BigInt(row.identity),key=decodeValue(row.key),value=decodeValue(row.value),receiver=decodeValue(row.receiver),prototype=decodeValue(row.prototype),descriptor=decodeDescriptor(row.descriptor);
 assert.equal(readJavaScriptOrdinaryHeapError(heap),'',row.name);
 const context=createOrdinaryHostContext(heap),host=context.objects.get(String(identity));let actual,hostSuccess;
 switch(row.operation){
  case 'get':{actual=planJavaScriptOrdinaryGet(heap,identity,key,receiver);const result=Reflect.get(host,context.materialize(key),context.materialize(receiver));if(actual.action==='value')assert.ok(Object.is(result,context.materialize(actual.value)));break;}
  case 'set':actual=planJavaScriptOrdinarySet(heap,identity,key,value,receiver);hostSuccess=Reflect.set(host,context.materialize(key),context.materialize(value),context.materialize(receiver));break;
  case 'define':actual=defineJavaScriptOrdinaryProperty(heap,identity,key,descriptor);hostSuccess=Reflect.defineProperty(host,context.materialize(key),context.descriptor(descriptor));break;
  case 'delete':actual=deleteJavaScriptOrdinaryProperty(heap,identity,key);hostSuccess=Reflect.deleteProperty(host,context.materialize(key));break;
  case 'prevent':actual=preventJavaScriptOrdinaryExtensions(heap,identity);hostSuccess=Reflect.preventExtensions(host);break;
  case 'prototype':actual=setJavaScriptOrdinaryPrototype(heap,identity,prototype);hostSuccess=Reflect.setPrototypeOf(host,context.materialize(prototype));break;
  case 'keys':actual=readJavaScriptOrdinaryOwnKeys(heap,identity);assert.deepEqual(actual.keys.map(context.materialize),Reflect.ownKeys(host));break;
  case 'insert':actual=insertJavaScriptOrdinaryObject(heap,identity,prototype);if(actual.success){const fresh=context.addObject({identity,callable:false});assert.equal(Reflect.setPrototypeOf(fresh,context.materialize(prototype)),true);}break;
  case 'symbol':actual=registerJavaScriptOrdinarySymbol(heap,identity);if(actual.success)context.addSymbol(identity);break;
  default:throw new TypeError('uncovered heap operation '+row.operation);
 }
 assert.equal(actual.error,row.expected.error??'',row.name);
 if(row.expected.action!==undefined)assert.equal(actual.action,row.expected.action,row.name);
 if(row.expected.success!==undefined)assert.equal(actual.success,row.expected.success,row.name);
 if(row.expected.value!==undefined)assert.deepEqual(encodeValue(actual.value),row.expected.value,row.name);
 if(row.expected.keys!==undefined)assert.deepEqual(actual.keys.map(encodeValue),row.expected.keys,row.name);
 if(actual.action==='call'){
  assert.equal(actual.success??false,false,'pending calls do not claim a completed write');assert.equal(context.calls.length,1);
  const expected={callee:row.expected.callee,receiver:row.expected.receiver,arguments:row.expected.arguments};
  const call={callee:encodeValue(actual.callee),receiver:encodeValue(actual.receiver),arguments:actual.arguments.map(encodeValue)};assert.deepEqual(call,expected);
  const observed=context.calls[0];assert.deepEqual({callee:encodeValue(observed.callee),receiver:encodeValue(observed.receiver),arguments:observed.arguments.map(encodeValue)},expected);
  if(row.operation==='set')assert.equal(hostSuccess,true,'the independent setter completed normally');
 }else{assert.equal(context.calls.length,0);if(hostSuccess!==undefined)assert.equal(actual.success,hostSuccess,row.name);}
 const resultHeap=actual.heap??heap;assert.equal(readJavaScriptOrdinaryHeapError(resultHeap),'');assertOrdinaryHostSnapshot(resultHeap,context);
 // No-write/rejected operations retain every original encoded Number bit,
 // absent descriptor payload and insertion position, not just SameValue.
 if(actual.success===false||actual.action==='call')assert.deepEqual(encodeHeap(resultHeap),row.heap);
 return {name:row.name,operation:row.operation,heap:row.heap,identity:row.identity,key:row.key,value:row.value,receiver:row.receiver,prototype:row.prototype,descriptor:row.descriptor,
  result:{...actual,...(actual.heap===undefined?{}:{heap:encodeHeap(actual.heap)}),...(actual.value===undefined?{}:{value:encodeValue(actual.value)}),...(actual.callee===undefined?{}:{callee:encodeValue(actual.callee)}),...(actual.receiver===undefined?{}:{receiver:encodeValue(actual.receiver)}),...(actual.arguments===undefined?{}:{arguments:actual.arguments.map(encodeValue)}),...(actual.keys===undefined?{}:{keys:actual.keys.map(encodeValue)})}};
}
export function observeInvalidHeap(row){const actual=readJavaScriptOrdinaryHeapError(decodeHeap(row.heap));assert.equal(actual,row.error,row.name);return {...row,actual};}
