// Immutable validation receipts are controller metadata, never represented
// WeakMap contents or a user-provided admission flag. Public data is copied
// before validation; only exact privately published frozen identities cache it.
import {readJavaScriptOrdinaryHeapError} from './javascript-ordinary-heap.js';
const validatedHeaps=new WeakSet(),immutableData=new WeakSet();
const descriptors=Object.getOwnPropertyDescriptors,prototype=Object.getPrototypeOf,define=Object.defineProperty,freeze=Object.freeze,plainPrototype=Object.prototype,arrayPrototype=Array.prototype;
/** Physical immutable data copying supplies NO semantic validation authority.
 * Previously owned immutable subgraphs may share their exact safe identity.
 * @param {unknown} value @returns {unknown} */
export function copyJavaScriptImmutableSnapshotData(value){
 const copies=new WeakMap();let work=0;
 function copy(input,depth){
  if(++work>4194304||depth>256)throw new RangeError('immutable snapshot data exceeds its explicit graph/depth profile');
  if(input===null||typeof input!=='object'){if(['undefined','boolean','number','bigint','string'].includes(typeof input)||input===null)return input;throw new TypeError('immutable snapshot data excludes native callable or Symbol values');}
  if(immutableData.has(input))return input;
  if(copies.has(input))return copies.get(input);
  const parent=prototype(input),array=Array.isArray(input);if(array?parent!==arrayPrototype:parent!==plainPrototype&&parent!==null)throw new TypeError('immutable snapshots require actual plain records and arrays');
  const output=array?[]:Object.create(null);copies.set(input,output);
  const fields=descriptors(input);for(const key of Reflect.ownKeys(fields)){
   if(typeof key!=='string')throw new TypeError('immutable snapshot metadata keys are actual strings');
   const field=fields[key];if(!field||!Object.hasOwn(field,'value'))throw new TypeError('immutable snapshots do not execute data accessors');
   if(array&&key==='length')continue;
   define(output,key,{value:copy(field.value,depth+1),enumerable:field.enumerable,writable:true,configurable:true});
  }
  if(array){const length=fields.length;if(!length||!Object.hasOwn(length,'value'))throw new TypeError('immutable arrays retain their actual own length descriptor');define(output,'length',{value:length.value,writable:true});}
  freeze(output);immutableData.add(output);return output;
 }
 return copy(value,0);
}
/** This predicate recognizes only internally validated immutable publications.
 * Object.freeze, matching fields, cloning or a claimed model cannot forge it.
 * @param {JavaScriptOrdinaryHeap} heap @returns {boolean} */
export function testJavaScriptValidatedHeapSnapshot(heap){return typeof heap==='object'&&heap!==null&&validatedHeaps.has(heap);}
/** Full validation occurs before the exact immutable heap identity is admitted.
 * Public mutable snapshots always take this boundary again.
 * @param {JavaScriptOrdinaryHeap} heap @returns {{heap:JavaScriptOrdinaryHeap|null,error:string}} */
export function sealJavaScriptOrdinaryHeapSnapshot(heap){
 if(testJavaScriptValidatedHeapSnapshot(heap))return {heap,error:''};
 try{
  const value=copyJavaScriptImmutableSnapshotData(heap);if(!value||typeof value!=='object'||value.$!=='ordinaryHeap'||!Array.isArray(value.objects)||!Array.isArray(value.symbols))return {heap:null,error:'an immutable heap receipt requires its actual complete ordinary heap snapshot'};
  const error=readJavaScriptOrdinaryHeapError(value);if(error!=='')return {heap:null,error};validatedHeaps.add(value);return {heap:value,error:''};
 }catch(error){return {heap:null,error:error instanceof Error?error.message:'immutable heap snapshot creation failed'};}
}
