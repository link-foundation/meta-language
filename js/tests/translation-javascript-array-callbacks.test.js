// Deferred controls. Importing/running represented behavior is not part of drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptArrayCallback,stepJavaScriptArrayCallback,resumeJavaScriptArrayCallback,readJavaScriptArrayCallbackStateError} from '../src/translation/javascript-array-callbacks.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))}),absent={$:'undefined'},key={$:'symbol',identity:99n};
function setup(){let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};for(const identity of [1n,2n,3n,4n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===2n||row.identity===4n?{...row,callable:true}:row.identity===1n?{...row,prototype:object(3n)}:row)};heap=data(heap,1n,'length',number(3));heap=data(heap,1n,'0',number(1));return data(heap,3n,'2',number(3));}
function data(heap,identity,name,value){return defineJavaScriptOrdinaryProperty(heap,identity,text(name),createJavaScriptOrdinarySetDescriptor(value,true)).heap;}
function advance(transition){let current=transition;for(let at=0;at<32&&current.$==='arrayCallbackContinue';at++)current=stepJavaScriptArrayCallback(current.heap,current.state);assert.notEqual(current.$,'arrayCallbackContinue');return current;}
const start=(heap,operation,callback=object(2n,true))=>advance(beginJavaScriptArrayCallback(heap,operation,object(1n),callback,object(3n),key));
test('forward callbacks skip holes and use latest inherited properties with captured length',()=>{
 const first=start(setup(),'Array.forEach');assert.equal(first.$,'arrayCallbackCall');assert.deepEqual(first.callee,object(2n,true));assert.deepEqual(first.receiver,object(3n));assert.deepEqual(first.arguments,[number(1),number(0),object(1n)]);assert.equal(first.state.length,3);
 let heap=data(first.heap,1n,'length',number(1));heap=data(heap,3n,'2',number(9));const second=advance(resumeJavaScriptArrayCallback(heap,first.state,{$:'normalCompletion',value:absent}));assert.equal(second.$,'arrayCallbackCall');assert.deepEqual(second.arguments,[number(9),number(2),object(1n)]);assert.equal(second.state.length,3);
 const done=advance(resumeJavaScriptArrayCallback(second.heap,second.state,{$:'normalCompletion',value:absent}));assert.equal(done.$,'arrayCallbackCompleted');assert.deepEqual(done.value,absent);
});
test('find visits holes and reverse find retains its original descending indices',()=>{
 let heap=setup();heap=data(heap,1n,'0',absent);const first=start(heap,'Array.findIndex');assert.equal(first.$,'arrayCallbackCall');assert.deepEqual(first.arguments,[absent,number(0),object(1n)]);const found=advance(resumeJavaScriptArrayCallback(first.heap,first.state,{$:'normalCompletion',value:{$:'boolean',value:true}}));assert.equal(found.$,'arrayCallbackCompleted');assert.deepEqual(found.value,number(0));
 const last=start(heap,'Array.findLastIndex');assert.equal(last.$,'arrayCallbackCall');assert.deepEqual(last.arguments,[number(3),number(2),object(1n)]);const hole=advance(resumeJavaScriptArrayCallback(last.heap,last.state,{$:'normalCompletion',value:{$:'boolean',value:false}}));assert.equal(hole.$,'arrayCallbackCall');assert.deepEqual(hole.arguments,[absent,number(1),object(1n)]);const reverse=advance(resumeJavaScriptArrayCallback(hole.heap,hole.state,{$:'normalCompletion',value:{$:'boolean',value:true}}));assert.equal(reverse.$,'arrayCallbackCompleted');assert.deepEqual(reverse.value,number(1));
});
test('length getters finish before callback callability is checked',()=>{
 let heap=setup();heap=defineJavaScriptOrdinaryProperty(heap,1n,text('length'),{configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:absent,writablePresent:false,writable:false,getterPresent:true,getter:object(4n,true),setterPresent:true,setter:absent}).heap;
 const first=start(heap,'Array.every',{$:'null'});assert.equal(first.$,'arrayCallbackCall');assert.equal(first.state.phase,'length getter');assert.deepEqual(first.callee,object(4n,true));heap=data(first.heap,3n,'effect',number(7));const actual=resumeJavaScriptArrayCallback(heap,first.state,{$:'normalCompletion',value:number(0)});assert.equal(actual.$,'arrayCallbackError');assert.equal(actual.errorClass,'TypeError');assert.deepEqual(actual.heap,heap);
});
test('callback abrupt completion retains actual current effects and the original thrown value',()=>{
 const first=start(setup(),'Array.some'),heap=data(first.heap,1n,'effect',number(7));const actual=resumeJavaScriptArrayCallback(heap,first.state,{$:'throwCompletion',value:text('original')});assert.equal(actual.$,'arrayCallbackThrow');assert.deepEqual(actual.value,text('original'));assert.deepEqual(actual.heap,heap);
});
test('ToLength truncates fractions and clamps huge lengths without eager allocation',()=>{
 for(const [raw,length]of [[2.9,2],[Infinity,9007199254740991]]){const first=start(data(setup(),1n,'length',number(raw)),'Array.some');assert.equal(first.$,'arrayCallbackCall');assert.equal(first.state.length,length);assert.equal(first.state.index,0);}
 const empty=start(data(setup(),1n,'length',number(-1)),'Array.every');assert.equal(empty.$,'arrayCallbackCompleted');assert.deepEqual(empty.value,{$:'boolean',value:true});
});
test('selected call suspension retains exact callback operands and can retry an unstarted call',()=>{
 const first=start(setup(),'Array.forEach');assert.equal(first.$,'arrayCallbackCall');assert.deepEqual(stepJavaScriptArrayCallback(first.heap,first.state),first);
 const state={...first.state,call:{...first.state.call,arguments:[number(9),number(0),object(1n)]}};assert.notEqual(readJavaScriptArrayCallbackStateError(first.heap,state),'');
});

test('reduce without an initializer selects the first present value without calling its reducer',()=>{
 const first=advance(beginJavaScriptArrayCallback(setup(),'Array.reduce',object(1n),object(2n,true),absent,key,false));assert.equal(first.$,'arrayCallbackCall');assert.deepEqual(first.receiver,absent);assert.deepEqual(first.arguments,[number(1),number(3),number(2),object(1n)]);assert.equal(first.state.hasAccumulator,true);assert.equal(first.state.initialPresent,false);
 const done=advance(resumeJavaScriptArrayCallback(first.heap,first.state,{$:'normalCompletion',value:number(4)}));assert.equal(done.$,'arrayCallbackCompleted');assert.deepEqual(done.value,number(4));
});
test('reduceRight visits present indices in descending order with four actual operands',()=>{
 const first=advance(beginJavaScriptArrayCallback(setup(),'Array.reduceRight',object(1n),object(2n,true),number(0),key,true));assert.equal(first.$,'arrayCallbackCall');assert.deepEqual(first.arguments,[number(0),number(3),number(2),object(1n)]);assert.deepEqual(first.receiver,absent);
 const second=advance(resumeJavaScriptArrayCallback(first.heap,first.state,{$:'normalCompletion',value:number(3)}));assert.equal(second.$,'arrayCallbackCall');assert.deepEqual(second.arguments,[number(3),number(1),number(0),object(1n)]);
 const done=advance(resumeJavaScriptArrayCallback(second.heap,second.state,{$:'normalCompletion',value:number(4)}));assert.equal(done.$,'arrayCallbackCompleted');assert.deepEqual(done.value,number(4));
});
test('empty reductions distinguish missing from explicit undefined initial values',()=>{
 const heap=data(setup(),1n,'length',number(0));for(const operation of ['Array.reduce','Array.reduceRight']){
  const missing=advance(beginJavaScriptArrayCallback(heap,operation,object(1n),object(2n,true),absent,key,false));assert.equal(missing.$,'arrayCallbackError');assert.equal(missing.errorClass,'TypeError');assert.deepEqual(missing.heap,heap);
  const present=advance(beginJavaScriptArrayCallback(heap,operation,object(1n),object(2n,true),absent,key,true));assert.equal(present.$,'arrayCallbackCompleted');assert.deepEqual(present.value,absent);
 }
});
