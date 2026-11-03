import {findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
/** ToLength after the actual ToNumber continuation has completed.
 * @param {number} value @returns {number} */
export function readJavaScriptArrayLikeLength(value){return Number.isNaN(value)||value<=0?0:Math.min(Math.floor(value),9007199254740991);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArraySearchOperation(operation){return ['Array.includes','Array.indexOf','Array.lastIndexOf','Array.at'].includes(operation);}
/** Out-of-range indices use bounded sentinels; Infinity is never an index.
 * @param {string} operation @param {number} length @param {boolean} present
 * @param {number} value @returns {number} */
export function readJavaScriptArraySearchIndex(operation,length,present,value){
 if(operation==='Array.lastIndexOf'&&!present)return length-1;
 const integer=Number.isNaN(value)||value===0?0:Math.trunc(value);
 if(operation==='Array.at'){const index=integer<0?length+integer:integer;return index<0||index>=length?-1:index;}
 if(operation==='Array.lastIndexOf'){if(integer===-Infinity)return -1;return integer>=0?Math.min(integer,length-1):Math.max(length+integer,-1);}
 return integer===Infinity?length:integer>=0?Math.min(integer,length):Math.max(length+integer,0);
}
/** Shared ordinary HasProperty; opaque boundaries require their own protocol.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key
 * @returns {{present:boolean,error:string}} */
export function readJavaScriptIndexedPropertyPresence(heap,identity,key){
 let current=identity;for(let depth=0;depth<heap.objects.length;depth++){
  const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,current)];if(object===undefined)return {present:false,error:'Array callback property lookup retains its actual global Object'};
  if(!['ordinaryObject','closureObject','boundFunctionObject','arrayObject','mapObject','setObject','collectionIteratorObject'].includes(object.$))return {present:false,error:'Array callback HasProperty requires the actual opaque host protocol'};
  if(findJavaScriptOrdinaryProperty(object.properties,key)>=0)return {present:true,error:''};
  if(object.prototype.$!=='object')return {present:false,error:''};current=object.prototype.identity;
 }return {present:false,error:'Array callback property lookup exceeds its actual prototype directory'};
}
