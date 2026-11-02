// Deferred controls only. No represented program runs during source drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {beginJavaScriptCollectionForEach,stepJavaScriptCollectionForEach,resumeJavaScriptCollectionForEach,readJavaScriptCollectionForEachStateError,compileJavaScriptCollectionForEachFunction} from '../src/translation/javascript-collection-for-each.js';
import {allocateJavaScriptCollection,executeJavaScriptCollectionOperation} from '../src/translation/javascript-collections.js';
import {insertJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {javascriptCollectionForEachOriginals as originals,javascriptCollectionForEachAssertionPolicy as policy} from './fixtures/javascript-collection-for-each-source.mjs';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,character=>character.charCodeAt(0))}),absent={$:'undefined'};
function setup(kind){let heap={$:'ordinaryHeap',objects:[],symbols:[]};heap=insertJavaScriptOrdinaryObject(heap,1n,{$:'null'}).heap;heap=allocateJavaScriptCollection(heap,2n,{$:'null'},kind).heap;heap=insertJavaScriptOrdinaryObject(heap,3n,{$:'null'}).heap;return {...heap,objects:heap.objects.map(row=>row.identity===3n?{...row,callable:true}:row)};}
function mutate(heap,operation,arguments_){const next=executeJavaScriptCollectionOperation(heap,object(2n),operation,arguments_);assert.equal(next.$,'collectionNormal');return next.heap;}
function advance(transition){let current=transition;for(let step=0;step<64&&current.$==='collectionForEachContinue';step++)current=stepJavaScriptCollectionForEach(current.heap,current.state);assert.notEqual(current.$,'collectionForEachContinue');return current;}
function start(heap,operation){return advance(beginJavaScriptCollectionForEach(heap,operation,object(2n),object(3n,true),object(1n)));}
function resume(transition,heap=transition.heap){return advance(resumeJavaScriptCollectionForEach(heap,transition.state,{$:'normalCompletion',value:{$:'boolean',value:false}}));}
test('Map forEach uses live ordinals and current future entry values',()=>{
 let heap=setup('Map');for(const [key,value]of [['a',1],['b',2],['c',3]])heap=mutate(heap,'Map.set',[text(key),number(value)]);
 const first=start(heap,'Map.forEach');assert.equal(first.$,'collectionForEachCall');assert.deepEqual(first.arguments,[number(1),text('a'),object(2n)]);assert.deepEqual(first.receiver,object(1n));
 heap=mutate(first.heap,'Map.set',[text('b'),number(20)]);heap=mutate(heap,'Map.delete',[text('c')]);heap=mutate(heap,'Map.set',[text('d'),number(4)]);const second=resume(first,heap);assert.deepEqual(second.arguments,[number(20),text('b'),object(2n)]);
 const third=resume(second);assert.deepEqual(third.arguments,[number(4),text('d'),object(2n)]);assert.equal(resume(third).$,'collectionForEachCompleted');
});
test('Map forEach delete and reinsertion appends without replaying an overwritten present key',()=>{
 let heap=mutate(setup('Map'),'Map.set',[text('a'),number(1)]);heap=mutate(heap,'Map.set',[text('b'),number(2)]);const first=start(heap,'Map.forEach');heap=mutate(first.heap,'Map.delete',[text('a')]);heap=mutate(heap,'Map.set',[text('a'),number(9)]);const second=resume(first,heap);assert.deepEqual(second.arguments,[number(2),text('b'),object(2n)]);heap=mutate(second.heap,'Map.set',[text('b'),number(20)]);const third=resume(second,heap);assert.deepEqual(third.arguments,[number(9),text('a'),object(2n)]);assert.equal(resume(third).$,'collectionForEachCompleted');
});
test('Set forEach clear keeps held operands and discovers appended values',()=>{
 let heap=mutate(setup('Set'),'Set.add',[number(3)]);heap=mutate(heap,'Set.add',[number(7)]);const first=start(heap,'Set.forEach');assert.deepEqual(first.arguments,[number(3),number(3),object(2n)]);heap=mutate(first.heap,'Set.clear',[]);assert.equal(readJavaScriptCollectionForEachStateError(heap,first.state),'');heap=mutate(heap,'Set.add',[number(9)]);const second=resume(first,heap);assert.deepEqual(second.arguments,[number(9),number(9),object(2n)]);assert.equal(resume(second).$,'collectionForEachCompleted');
});
test('forEach receiver slots are checked before callback callability including empty data',()=>{
 const heap=setup('Set'),wrong=beginJavaScriptCollectionForEach(heap,'Map.forEach',object(2n),{$:'null'},absent);assert.equal(wrong.$,'collectionForEachError');assert.equal(wrong.errorClass,'TypeError');assert.match(wrong.reason,/internal slot/u);
 const callback=beginJavaScriptCollectionForEach(heap,'Set.forEach',object(2n),{$:'null'},absent);assert.equal(callback.$,'collectionForEachError');assert.match(callback.reason,/callback/u);assert.equal(start(heap,'Set.forEach').$,'collectionForEachCompleted');
});
test('forEach suspended calls retain actual held operands after callback effects and can retry',()=>{
 const first=start(mutate(setup('Map'),'Map.set',[text('a'),number(1)]),'Map.forEach');assert.deepEqual(stepJavaScriptCollectionForEach(first.heap,first.state),first);const heap=mutate(first.heap,'Map.clear',[]);assert.equal(readJavaScriptCollectionForEachStateError(heap,first.state),'');
 const substituted={...first.state,call:{...first.state.call,arguments:[number(99),text('a'),object(2n)]}};assert.notEqual(readJavaScriptCollectionForEachStateError(heap,substituted),'');
 const thrown=resumeJavaScriptCollectionForEach(heap,first.state,{$:'throwCompletion',value:object(1n)});assert.equal(thrown.$,'collectionForEachThrow');assert.deepEqual(thrown.value,object(1n));assert.deepEqual(thrown.heap,heap);
});
test('forEach ordinal gaps remain bounded resumable transitions rather than eager snapshots',()=>{
 let heap=mutate(setup('Set'),'Set.add',[number(1)]);heap=mutate(heap,'Set.add',[number(2)]);heap=mutate(heap,'Set.delete',[number(1)]);let begun=beginJavaScriptCollectionForEach(heap,'Set.forEach',object(2n),object(3n,true),absent);assert.equal(begun.$,'collectionForEachContinue');const skipped=stepJavaScriptCollectionForEach(heap,begun.state);assert.equal(skipped.$,'collectionForEachContinue');assert.equal(skipped.state.cursor,1);const selected=stepJavaScriptCollectionForEach(heap,skipped.state);assert.equal(selected.$,'collectionForEachCall');assert.deepEqual(selected.arguments,[number(2),number(2),object(2n)]);
});
test('forEach protected bodies use the actual operation identity without iterator allocation',()=>{
 for(const operation of ['Map.forEach','Set.forEach']){const fn=compileJavaScriptCollectionForEachFunction(1n,2n,operation);assert.deepEqual(fn.instructions,[{$:'collectionForEach',destination:1,operation},{$:'return',source:1}]);assert.equal(fn.receiverMode,'primitive intrinsic');}assert.equal(compileJavaScriptCollectionForEachFunction(1n,2n,'Array.forEach'),undefined);
});
for(const row of originals)test(row.name,()=>{
 const expected=selectVersionedAssertion(policy,process.versions.node).expected;assert.equal(expected,row.expected);
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-collection-forEach.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:73n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,[]),{steps:65536}).transition;
 assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),row.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn '+row.entry+';',[])();assert.equal(original(),row.expected);
});
