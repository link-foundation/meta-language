// Authored original/model controls; no execution during bulk drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptArraySearch,stepJavaScriptArraySearch,resumeJavaScriptArraySearch} from '../src/translation/javascript-array-search.js';
import {readJavaScriptArraySearchIndex} from '../src/translation/javascript-array-index-rules.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))}),absent={$:'undefined'},exoticKey={$:'symbol',identity:99n};
function setup(){let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};for(const identity of [1n,2n,3n,4n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===4n?{...row,callable:true}:row.identity===1n?{...row,prototype:object(3n)}:row)};heap=data(heap,1n,'length',number(3));heap=data(heap,1n,'0',number(NaN));return data(heap,3n,'2',number(7));}
function data(heap,identity,name,value){return defineJavaScriptOrdinaryProperty(heap,identity,text(name),createJavaScriptOrdinarySetDescriptor(value,true)).heap;}
function getter(heap,identity,name){return defineJavaScriptOrdinaryProperty(heap,identity,text(name),{configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:absent,writablePresent:false,writable:false,getterPresent:true,getter:object(4n,true),setterPresent:true,setter:absent}).heap;}
function advance(transition){let current=transition;for(let at=0;at<64&&current.$==='arraySearchContinue';at++)current=stepJavaScriptArraySearch(current.heap,current.state);assert.notEqual(current.$,'arraySearchContinue');return current;}
const search=(heap,operation,value=absent,from=absent,present=false)=>advance(beginJavaScriptArraySearch(heap,operation,object(1n),value,from,present,exoticKey));
test('search equality and hole handling remain operation-specific',()=>{
 const heap=setup();for(const [operation,value,expected]of [['Array.includes',number(NaN),{$:'boolean',value:true}],['Array.indexOf',number(NaN),number(-1)],['Array.lastIndexOf',number(NaN),number(-1)],['Array.includes',absent,{$:'boolean',value:true}],['Array.indexOf',absent,number(-1)],['Array.lastIndexOf',number(7),number(2)]]){const actual=search(heap,operation,value);assert.equal(actual.$,'arraySearchCompleted');assert.deepEqual(actual.value,expected);}
});
test('relative index planning retains missing arguments, infinities and exact boundaries',()=>{
 for(const [operation,present,value,index]of [['Array.lastIndexOf',false,NaN,2],['Array.lastIndexOf',true,NaN,0],['Array.lastIndexOf',true,-Infinity,-1],['Array.lastIndexOf',true,Infinity,2],['Array.indexOf',true,Infinity,3],['Array.indexOf',true,-Infinity,0],['Array.at',true,-1.9,2],['Array.at',true,Infinity,-1],['Array.at',true,-4,-1],['Array.at',true,-0,0]])assert.equal(readJavaScriptArraySearchIndex(operation,3,present,value),index);
});
test('index conversion observes prior length capture and retained current heap effects',()=>{
 let heap=data(setup(),2n,'valueOf',object(4n,true));const first=search(heap,'Array.includes',absent,object(2n),true);assert.equal(first.$,'arraySearchCall');assert.equal(first.state.phase,'index conversion');assert.equal(first.state.length,3);assert.deepEqual(first.callee,object(4n,true));assert.deepEqual(first.receiver,object(2n));
 heap=data(first.heap,1n,'length',number(1));heap=data(heap,1n,'effect',number(9));const actual=advance(resumeJavaScriptArraySearch(heap,first.state,{$:'normalCompletion',value:number(1)}));assert.equal(actual.$,'arraySearchCompleted');assert.deepEqual(actual.value,{$:'boolean',value:true});assert.deepEqual(actual.heap,heap);
});
test('empty searches skip index conversion while at preserves it',()=>{
 let heap=data(setup(),1n,'length',number(0));heap=data(heap,2n,'valueOf',object(4n,true));for(const operation of ['Array.includes','Array.indexOf','Array.lastIndexOf']){const actual=search(heap,operation,number(7),object(2n),true);assert.equal(actual.$,'arraySearchCompleted');}
 const at=search(heap,'Array.at',absent,object(2n),true);assert.equal(at.$,'arraySearchCall');const actual=advance(resumeJavaScriptArraySearch(at.heap,at.state,{$:'normalCompletion',value:number(0)}));assert.equal(actual.$,'arraySearchCompleted');assert.deepEqual(actual.value,absent);
});
test('inherited getter results are retained across a source mutation and return',()=>{
 const first=search(getter(setup(),3n,'2'),'Array.indexOf',number(9));assert.equal(first.$,'arraySearchCall');assert.equal(first.state.phase,'element getter');assert.equal(first.state.index,2);assert.deepEqual(first.receiver,object(1n));
 const heap=data(first.heap,1n,'effect',number(7));const actual=advance(resumeJavaScriptArraySearch(heap,first.state,{$:'normalCompletion',value:number(9)}));assert.equal(actual.$,'arraySearchCompleted');assert.deepEqual(actual.value,number(2));assert.deepEqual(actual.heap,heap);
});
test('abrupt getters retain their original thrown payload and do not fabricate a miss',()=>{
 const first=search(getter(setup(),1n,'0'),'Array.includes',number(7));assert.equal(first.$,'arraySearchCall');const heap=data(first.heap,1n,'effect',number(9)),actual=resumeJavaScriptArraySearch(heap,first.state,{$:'throwCompletion',value:text('original')});assert.equal(actual.$,'arraySearchThrow');assert.deepEqual(actual.value,text('original'));assert.deepEqual(actual.heap,heap);assert.deepEqual(stepJavaScriptArraySearch(first.heap,first.state),first);
});
