// Retention projection over actual shared heap slots. This is deliberately
// separate from liveness authority and performs no scheduled collection.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject} from './javascript-ordinary-heap.js';
/** Ordinary properties (including Symbol keys) and prototypes are strong.
 * Weak keys are never strong; WeakMap values are conditional ephemeron edges.
 * @param {JavaScriptOrdinaryObject} object @returns {{strong:JavaScriptSemanticValue[],weak:JavaScriptSemanticValue[],conditional:{key:JavaScriptSemanticValue,value:JavaScriptSemanticValue}[]}} */
export function projectJavaScriptHeapRetentionEdges(object){
 const strong=[object.prototype],weak=[],conditional=[];
 for(const property of object.properties){strong.push(property.key);const descriptor=property.descriptor;if(descriptor.valuePresent)strong.push(descriptor.value);if(descriptor.getterPresent)strong.push(descriptor.getter);if(descriptor.setterPresent)strong.push(descriptor.setter);}
 if(object.$==='closureObject')strong.push(object.lexicalReceiver,...object.captures,...(object.lexicalNewTarget===undefined?[]:[object.lexicalNewTarget]),{$:'object',identity:object.owner,callable:true});
 if(object.$==='ordinaryObject'&&object.primitiveData!==undefined)strong.push(object.primitiveData);
 if(object.$==='mapObject'||object.$==='setObject')for(const entry of object.entries)if(entry.present)strong.push(entry.key,entry.value);
 if(object.$==='weakMapObject'||object.$==='weakSetObject')for(const entry of object.weakEntries){weak.push(entry.key);if(object.$==='weakMapObject')conditional.push({key:entry.key,value:entry.value});}
 if(object.$==='collectionIteratorObject'){if(object.collection!==0n)strong.push({$:'object',identity:object.collection,callable:false});if(object.iteratedText!==undefined)strong.push(object.iteratedText);}
 if(['hostErrorObject','hostURLObject','hostArrayObject','hostByteBufferObject'].includes(object.$))strong.push(object.hostGet,object.hostSet);
 return {strong,weak,conditional};
}
/** Compute the ephemeron fixed point relative to SUPPLIED roots only. The
 * embedding still owns complete observable-world root/liveness authority.
 * No object directory is pruned and no WeakRef/cleanup event is fabricated.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue[]} roots @returns {{identities:bigint[],error:string}} */
export function traceJavaScriptWeakCollectionRetention(heap,roots){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {identities:[],error};if(roots.length>1048576)return {identities:[],error:'retention roots exceed the explicit shared value budget'};
 for(const value of roots){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {identities:[],error:invalid};}
 const identities=[],pending=roots.slice(),conditional=[],objects=heap.objects.map(()=>false),symbols=heap.symbols.map(()=>false);let conditionalIndex=0,work=0;
 const symbolIndex=identity=>{let low=0,high=heap.symbols.length;while(low<high){const middle=Math.floor((low+high)/2);if((heap.symbols[middle]??identity)<identity)low=middle+1;else high=middle;}return heap.symbols[low]===identity?low:-1;};
 const live=value=>value.$==='object'?objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)]===true:value.$==='symbol'?symbols[symbolIndex(value.identity)]===true:false;
 for(;;){
  while(pending.length){if(++work>4194304)return {identities:[],error:'retention inspection exceeds its explicit finite work profile'};const value=pending.pop();if(value===undefined||value.$!=='object'&&value.$!=='symbol'||live(value))continue;identities.push(value.identity);if(value.$==='symbol'){symbols[symbolIndex(value.identity)]=true;continue;}objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)]=true;const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,value.identity)];if(object===undefined)return {identities:[],error:'retention visits actual current global Object identities'};const edges=projectJavaScriptHeapRetentionEdges(object);pending.push(...edges.strong);conditional.push(...edges.conditional);}
  let activated=false;for(let index=0;index<conditional.length;index++){if(++work>4194304)return {identities:[],error:'ephemeron activation exceeds its explicit finite work profile'};const edge=conditional[index];if(edge!==undefined&&live(edge.key)&&!live(edge.value)){pending.push(edge.value);conditional[index]={key:{$:'undefined'},value:{$:'undefined'}};activated=true;}}
  if(!activated)break;if(++conditionalIndex>heap.objects.length+heap.symbols.length+1)return {identities:[],error:'ephemeron activation exceeds the finite actual global identity budget'};
 }
 return {identities:identities.toSorted((first,second)=>first<second?-1:first>second?1:0),error:''};
}
