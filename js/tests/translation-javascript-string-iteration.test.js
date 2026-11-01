// Deferred exact code-point controls, including ill-formed UTF16 transport.
import test from 'node:test';import assert from 'node:assert/strict';
import {allocateJavaScriptStringIterator,advanceJavaScriptStringIterator,beginJavaScriptStringIteratorResult} from '../src/translation/javascript-string-iteration.js';
import {advanceJavaScriptCollectionIterator,advanceJavaScriptBrandedCollectionIterator} from '../src/translation/javascript-collection-iteration.js';
import {stepJavaScriptCollectionIteratorResult,resumeJavaScriptCollectionIteratorResultAllocation} from '../src/translation/javascript-collection-iterator-results.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {insertJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
const object=identity=>({$:'object',identity,callable:false}),absent={$:'undefined'};
function setup(value){let heap={$:'ordinaryHeap',objects:[],symbols:[]};for(const id of [1n,2n])heap=insertJavaScriptOrdinaryObject(heap,id,{$:'null'}).heap;return allocateJavaScriptStringIterator(heap,3n,object(2n),value);}
for(const profile of ['text','sourceText'])test('String iteration retains exact code points and lone surrogates in '+profile,()=>{
 const units=[65,55357,56832,55296,66,56320],source=String.fromCharCode(...units),value=profile==='text'?{$:'text',units}:{$:'sourceText',profile:'inspectable UTF16 source text 1',text:source},allocated=setup(value);assert.equal(allocated.$,'stringIteratorAllocated');let heap=allocated.heap;const actual=[];
 for(let index=0;index<6;index++){const next=advanceJavaScriptStringIterator(heap,allocated.value);assert.equal(next.$,'stringIteratorComponents');heap=next.heap;if(next.done){assert.deepEqual(next.value,absent);break;}actual.push(next.value.units);}
 assert.deepEqual(actual,[[65],[55357,56832],[55296],[66],[56320]]);const expected=Array.from(source,part=>Array.from({length:part.length},(_,index)=>part.charCodeAt(index)));assert.deepEqual(actual,expected);const row=heap.objects.find(row=>row.identity===3n);assert.equal(row.cursor,0);assert.deepEqual(row.iteratedText,absent);assert.equal(advanceJavaScriptStringIterator(heap,allocated.value).done,true);
});
test('empty String iteration exhausts once and has an actual distinct family brand',()=>{
 const allocated=setup({$:'text',units:[]}),next=advanceJavaScriptStringIterator(allocated.heap,allocated.value);assert.equal(next.done,true);assert.equal(advanceJavaScriptBrandedCollectionIterator(next.heap,allocated.value,'Map').$,'collectionIterationTypeError');assert.equal(advanceJavaScriptCollectionIterator(next.heap,allocated.value).$,'collectionIterationTypeError');assert.equal(advanceJavaScriptStringIterator(next.heap,object(1n)).$,'stringIteratorTypeError');
});
test('String next allocates the shared genuine value/done result after advancing its code-point cursor',()=>{
 const allocated=setup({$:'text',units:[55357,56832,65]}),array=allocateJavaScriptArray(allocated.heap,4n,object(1n),0),pending=beginJavaScriptStringIteratorResult(array.heap,allocated.value,array.value,object(1n));assert.equal(pending.$,'iteratorResultContinue');assert.deepEqual(pending.state.value,{$:'text',units:[55357,56832]});assert.equal(pending.heap.objects.find(row=>row.identity===3n).cursor,2);const request=stepJavaScriptCollectionIteratorResult(pending.heap,pending.state);assert.equal(request.kind,'ordinary');const completed=resumeJavaScriptCollectionIteratorResultAllocation(request.heap,request.state,5n);assert.equal(completed.$,'iteratorResultNormal');assert.deepEqual(completed.value,object(5n));const next=advanceJavaScriptStringIterator(completed.heap,allocated.value);assert.deepEqual(next.value,{$:'text',units:[65]});
});
test('String iterator allocation requires a completed semantic text value and a fresh identity',()=>{
 const allocated=setup({$:'text',units:[65]});assert.equal(allocateJavaScriptStringIterator(allocated.heap,6n,object(2n),{$:'number',value:7}).$,'stringIteratorInvalid');assert.equal(allocateJavaScriptStringIterator(allocated.heap,3n,object(2n),{$:'text',units:[65]}).$,'stringIteratorInvalid');
});
