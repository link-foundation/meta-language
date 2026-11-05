import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation} from '../src/translation/javascript-call-machine.js';
import {readSourceMachineTrace} from './support/source-machine-trace.js';
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


test('String spread records actual primitive prototype witnesses for common native replay',()=>{
 const source='export function read() { const output = [..."A😀"]; return output.length; }',capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-string-iterator-trace.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:149n}),trace=readSourceMachineTrace(instance.embedding,instance.begin('read',[]),{steps:8192});assert.equal(trace.transition.$,'machineCompleted');assert.deepEqual(trace.transition.value,{$:'number',value:2});const witnesses=trace.events.filter(event=>event.operation==='primitiveIteratorPrototype');assert.equal(witnesses.length,1);assert.equal(witnesses[0].receiver.$,'text');assert.equal(witnesses[0].prototype.$,'object');assert.equal(witnesses[0].key.$,'symbol');
});
test('primitive iterator authority rejects substituted operands and replacement roots after publication',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('export function read() { return [..."A"].length; }',{identifier:'literal-string-iterator-authority.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:151n}),started=instance.begin('read',[]),request=runJavaScriptCallMachine(instance.embedding.module.program,started.machine,1024,[]).transition;
 // Reach the metadata boundary using the ordinary explicit identity allocator.
 let current=request;for(let at=0;at<32&&current.$!=='machineNeedPrimitiveIteratorGet';at++){if(current.$!=='machineAllocateObject')assert.fail('expected an actual identity or primitive metadata boundary');instance.embedding.adoptHeap(current.machine.heap);const identity=instance.embedding.allocateIdentity();const allocated=resumeJavaScriptCallObjectAllocation(instance.embedding.module.program,current.machine,identity);assert.equal(allocated.$,'machineContinue');current=runJavaScriptCallMachine(instance.embedding.module.program,allocated.machine,1024,[]).transition;}
 assert.equal(current.$,'machineNeedPrimitiveIteratorGet');instance.embedding.adoptHeap(current.machine.heap);assert.throws(()=>instance.embedding.resumePrimitiveIteratorGet({...current,receiver:{$:'text',units:[66]}}),/actual receiver or key/);assert.throws(()=>instance.embedding.registerPrimitiveIteratorPrototypes([]),/before source publication/);const resumed=instance.embedding.resumePrimitiveIteratorGet(current);assert.equal(resumed.resumed,true);assert.equal(resumed.prototype.$,'object');assert.notEqual(resumed.transition.$,'machineNeedPrimitiveIteratorGet');
});
