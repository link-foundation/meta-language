// Deferred controls for inspectable slots. Do not run during bulk source draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {allocateJavaScriptCollection,executeJavaScriptCollectionOperation} from '../src/translation/javascript-collections.js';
import {allocateJavaScriptCollectionIterator,advanceJavaScriptCollectionIterator} from '../src/translation/javascript-collection-iteration.js';
import {readJavaScriptOrdinaryHeapError,defineJavaScriptOrdinaryProperty,preventJavaScriptOrdinaryExtensions} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import * as rules from '../src/translation/javascript-collection-rules.js';
import {javascriptInspectableCollectionDecisions} from './fixtures/javascript-inspectable-collections.mjs';
test('every exact inspectable collection decision',()=>{for(const row of javascriptInspectableCollectionDecisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);});
const number=value=>({$:'number',value});
const object=identity=>({$:'object',identity,callable:false});
function setup(kind){
 const initial={$:'ordinaryHeap',objects:[],symbols:[]};
 const result=allocateJavaScriptCollection(initial,1n,{$:'null'},kind);
 assert.equal(result.$,'collectionNormal');
 return result.heap;
}
function operation(heap,kind,key,value){
 const result=executeJavaScriptCollectionOperation(heap,object(1n),kind,key===undefined?[]:value===undefined?[key]:[key,value]);
 assert.equal(result.$,'collectionNormal');assert.equal(readJavaScriptOrdinaryHeapError(result.heap),'');
 return result;
}
for(const kind of ['Map','Set'])test(kind+' exact key equality and insertion order survive live mutation',()=>{
 let heap=setup(kind);const add=kind==='Map'?'Map.set':'Set.add';
 heap=operation(heap,add,number(-0),number(10)).heap;
 heap=operation(heap,add,number(NaN),number(20)).heap;
 heap=operation(heap,add,number(NaN),number(30)).heap;
 assert.equal(operation(heap,kind+'.size').value.value,2);
 assert.equal(operation(heap,kind+'.has',number(0)).value.value,true);
 if(kind==='Map')assert.equal(operation(heap,'Map.get',number(NaN)).value.value,30);
 const allocated=allocateJavaScriptCollectionIterator(heap,2n,{$:'null'},object(1n),kind,'entries');assert.equal(allocated.$,'collectionIteratorAllocated');heap=allocated.heap;
 let step=advanceJavaScriptCollectionIterator(heap,object(2n));assert.equal(step.$,'collectionIteration');assert.equal(step.done,false);assert.equal(step.entry,true);assert.equal(Object.is(step.key.value,0),true);heap=step.heap;
 heap=operation(heap,kind+'.delete',number(NaN)).heap;
 heap=operation(heap,add,number(7),number(70)).heap;
 step=advanceJavaScriptCollectionIterator(heap,object(2n));assert.equal(step.done,false);assert.equal(step.key.value,7);heap=step.heap;
 heap=operation(heap,kind+'.clear').heap;
 heap=operation(heap,add,number(8),number(80)).heap;
 step=advanceJavaScriptCollectionIterator(heap,object(2n));assert.equal(step.done,false);assert.equal(step.key.value,8);heap=step.heap;
 step=advanceJavaScriptCollectionIterator(heap,object(2n));assert.equal(step.done,true);heap=step.heap;
 heap=operation(heap,add,number(9),number(90)).heap;
 step=advanceJavaScriptCollectionIterator(heap,object(2n));assert.equal(step.done,true);
 assert.equal(step.heap.objects.find(row=>row.identity===2n).collection,0n);
 assert.equal(heap.objects[0].entries.filter(row=>!row.present).every(row=>row.key.$==='undefined'&&row.value.$==='undefined'),true);
});
test('ordinary mutations preserve genuine collection slots and key identity',()=>{
 let heap=setup('Map');heap=operation(heap,'Map.set',number(1),number(2)).heap;
 const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=number(3);descriptor.writable=true;
 const defined=defineJavaScriptOrdinaryProperty(heap,1n,{$:'text',units:[120]},descriptor);assert.equal(defined.success,true);
 const frozen=preventJavaScriptOrdinaryExtensions(defined.heap,1n);assert.equal(frozen.success,true);assert.equal(frozen.heap.objects[0].$,'mapObject');
 assert.equal(operation(frozen.heap,'Map.get',number(1)).value.value,2);
 assert.equal(operation(frozen.heap,'Map.set',number(4),number(5)).value.identity,1n);
 const duplicate=structuredClone(heap);duplicate.objects[0].entries.push({...duplicate.objects[0].entries[0]});assert.match(readJavaScriptOrdinaryHeapError(duplicate),/unique/);
 const bad=structuredClone(heap);bad.objects[0].entries[0].key=number(-0);assert.match(readJavaScriptOrdinaryHeapError(bad),/negative/);
 const receiver=executeJavaScriptCollectionOperation(heap,number(1),'Map.get',[]);assert.equal(receiver.$,'collectionTypeError');assert.deepEqual(receiver.heap,heap);
});

test('collection allocation retains exact global identity authority',()=>{
 const empty={$:'ordinaryHeap',objects:[],symbols:[]};
 for(const identity of [1,Number.MAX_SAFE_INTEGER,'1',0n,-1n,18446744073709551616n]){
  const result=allocateJavaScriptCollection(empty,identity,{$:'null'},'Map');assert.equal(result.$,'collectionInvalid');assert.deepEqual(result.heap,empty);
 }
 const heap=setup('Map');
 for(const identity of [2,'2',0n,1n,18446744073709551616n]){
  const result=allocateJavaScriptCollectionIterator(heap,identity,{$:'null'},object(1n),'Map','entries');assert.equal(result.$,'collectionIterationInvalid');assert.deepEqual(result.heap,heap);
 }
 const malformed=structuredClone(heap);malformed.objects[0].identity=1;assert.match(readJavaScriptOrdinaryHeapError(malformed),/unsigned64/);
});
