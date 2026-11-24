// Deferred tests: imports do not execute the represented iterator operations.
import test from 'node:test';import assert from 'node:assert/strict';
import {allocateJavaScriptArray,testJavaScriptArrayValue} from '../src/translation/javascript-arrays.js';
import {allocateJavaScriptCollection,executeJavaScriptCollectionOperation} from '../src/translation/javascript-collections.js';
import {allocateJavaScriptCollectionIterator,advanceJavaScriptBrandedCollectionIterator} from '../src/translation/javascript-collection-iteration.js';
import {beginJavaScriptCollectionIteratorResult,stepJavaScriptCollectionIteratorResult,resumeJavaScriptCollectionIteratorResultAllocation} from '../src/translation/javascript-collection-iterator-results.js';
import {insertJavaScriptOrdinaryObject,readJavaScriptOrdinaryOwnKeys,planJavaScriptOrdinaryGet} from '../src/translation/javascript-ordinary-heap.js';
const object=identity=>({$:'object',identity,callable:false}),text=value=>({$:'text',units:Array.from(value,character=>character.charCodeAt(0))});
function setup(){
 let heap={$:'ordinaryHeap',objects:[],symbols:[]};heap=insertJavaScriptOrdinaryObject(heap,1n,{$:'null'}).heap;
 heap=allocateJavaScriptArray(heap,2n,object(1n),0).heap;
 heap=allocateJavaScriptCollection(heap,3n,object(1n),'Map').heap;
 heap=insertJavaScriptOrdinaryObject(heap,5n,object(1n)).heap;
 heap=executeJavaScriptCollectionOperation(heap,object(3n),'Map.set',[text('key'),object(5n)]).heap;
 heap=allocateJavaScriptCollectionIterator(heap,4n,object(1n),object(3n),'Map','entries').heap;
 return heap;
}
function read(heap,identity,key){const result=planJavaScriptOrdinaryGet(heap,identity,text(key),object(identity));assert.equal(result.action,'value');return result.value;}
test('entries allocate a genuine Array and fresh ordered iterator result without readvancing',()=>{
 const initial=setup(),begin=beginJavaScriptCollectionIteratorResult(initial,object(4n),'Map',object(2n),object(1n));assert.equal(begin.$,'iteratorResultContinue');
 assert.equal(begin.heap.objects.find(row=>row.identity===4n).cursor,1);
 const request=stepJavaScriptCollectionIteratorResult(begin.heap,begin.state);assert.equal(request.$,'iteratorResultAllocate');assert.equal(request.kind,'array');assert.deepEqual(stepJavaScriptCollectionIteratorResult(begin.heap,begin.state),request);
 const entry=resumeJavaScriptCollectionIteratorResultAllocation(begin.heap,begin.state,6n);assert.equal(entry.$,'iteratorResultContinue');assert.equal(testJavaScriptArrayValue(entry.heap,entry.state.value).array,true);
 assert.deepEqual(read(entry.heap,6n,'0'),text('key'));assert.deepEqual(read(entry.heap,6n,'1'),object(5n));assert.deepEqual(read(entry.heap,6n,'length'),{$:'number',value:2});
 const next=stepJavaScriptCollectionIteratorResult(entry.heap,entry.state);assert.equal(next.kind,'ordinary');
 const result=resumeJavaScriptCollectionIteratorResultAllocation(entry.heap,entry.state,7n);assert.equal(result.$,'iteratorResultNormal');assert.deepEqual(result.value,object(7n));
 assert.deepEqual(readJavaScriptOrdinaryOwnKeys(result.heap,7n).keys,[text('value'),text('done')]);assert.deepEqual(read(result.heap,7n,'value'),object(6n));assert.deepEqual(read(result.heap,7n,'done'),{$:'boolean',value:false});
 for(const property of result.heap.objects.find(row=>row.identity===7n).properties)assert.equal(property.descriptor.writable&&property.descriptor.enumerable&&property.descriptor.configurable,true);
 assert.equal(result.heap.objects.find(row=>row.identity===4n).cursor,1);
 const original=new Map([['key',{value:1}]]).entries().next();assert.equal(Array.isArray(original.value),true);assert.deepEqual(Reflect.ownKeys(original),['value','done']);assert.equal(original.done,false);
});
test('exhaustion yields a new object every time and preserves its method brand',()=>{
 let begin=beginJavaScriptCollectionIteratorResult(setup(),object(4n),'Map',object(2n),object(1n));let entry=resumeJavaScriptCollectionIteratorResultAllocation(begin.heap,begin.state,6n);let result=resumeJavaScriptCollectionIteratorResultAllocation(entry.heap,entry.state,7n);
 begin=beginJavaScriptCollectionIteratorResult(result.heap,object(4n),'Map',object(2n),object(1n));assert.equal(begin.state.done,true);assert.equal(stepJavaScriptCollectionIteratorResult(begin.heap,begin.state).kind,'ordinary');
 result=resumeJavaScriptCollectionIteratorResultAllocation(begin.heap,begin.state,8n);assert.deepEqual(read(result.heap,8n,'done'),{$:'boolean',value:true});assert.deepEqual(read(result.heap,8n,'value'),{$:'undefined'});
 const other=beginJavaScriptCollectionIteratorResult(result.heap,object(4n),'Map',object(2n),object(1n));const final=resumeJavaScriptCollectionIteratorResultAllocation(other.heap,other.state,9n);assert.notEqual(result.value.identity,final.value.identity);
 assert.equal(advanceJavaScriptBrandedCollectionIterator(final.heap,object(4n),'Set').$,'collectionIterationTypeError');assert.equal(final.heap.objects.find(row=>row.identity===4n).collectionKind,'Map');
 const mapIterator=new Map().keys(),setNext=Object.getPrototypeOf(new Set().keys()).next;mapIterator.next();assert.throws(()=>Reflect.apply(setNext,mapIterator,[]),TypeError);
});
test('invalid prototype operands cannot consume a source item or mint an identity',()=>{
 const heap=setup(),invalid=beginJavaScriptCollectionIteratorResult(heap,object(4n),'Map',object(1n),object(1n));assert.equal(invalid.$,'iteratorResultInvalid');assert.deepEqual(invalid.heap,heap);assert.equal(heap.objects.find(row=>row.identity===4n).cursor,0);
 const begin=beginJavaScriptCollectionIteratorResult(heap,object(4n),'Map',object(2n),object(1n));for(const identity of [6,'6',0n,3n,18446744073709551616n]){const result=resumeJavaScriptCollectionIteratorResultAllocation(begin.heap,begin.state,identity);assert.equal(result.$,'iteratorResultInvalid');assert.deepEqual(result.heap,begin.heap);}
});
