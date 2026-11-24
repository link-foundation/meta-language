// Deferred Set-based copy controls; all represented steps live in test callbacks.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptObjectAssignment,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread} from '../src/translation/javascript-object-spread.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {locateJavaScriptOrdinaryProperty} from '../src/translation/javascript-ordinary-heap.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {readJavaScriptArrayLength} from '../src/translation/javascript-array-objects.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),text=value=>({$:'text',units:Array.from(value,part=>part.charCodeAt(0))}),number=value=>({$:'number',value}),absent={$:'undefined'};
function data(value,writable=true){const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=value;descriptor.writable=writable;return descriptor;}
function property(key,value){return {$:'ordinaryProperty',key,descriptor:data(value)};}
function row(identity,prototype={$:'null'},properties=[],callable=false){return {$:'ordinaryObject',identity,prototype,extensible:true,callable,properties};}
function heap(){return {$:'ordinaryHeap',symbols:[9n],objects:[row(1n),row(2n,{$:'null'},[property(text('field'),number(7)),property({$:'symbol',identity:9n},number(9))]),row(3n,{$:'null'},[],true)]};}
function drain(heap,next){for(let step=0;step<128&&next.$==='objectSpreadContinue';step++){heap=next.heap;next=stepJavaScriptObjectSpread(heap,next.state);}return next;}
test('Object assign copies enumerable Symbol values through actual target Set semantics',()=>{
 const input=heap(),next=drain(input,beginJavaScriptObjectAssignment(input,object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadCompleted');assert.deepEqual(next.value,object(1n));assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('field')).descriptor.value.value,7);assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,{$:'symbol',identity:9n}).descriptor.value.value,9);
});
test('inherited target setters receive the held source value and do not create own data',()=>{
 const input=heap(),descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.setter=object(3n,true);input.objects[0].prototype=object(4n);input.objects.push(row(4n,{$:'null'},[{$:'ordinaryProperty',key:text('field'),descriptor}]));let next=drain(input,beginJavaScriptObjectAssignment(input,object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadCall');assert.equal(next.state.phase,'setter');assert.deepEqual(next.callee,object(3n,true));assert.deepEqual(next.receiver,object(1n));assert.deepEqual(next.arguments,[number(7)]);next=resumeJavaScriptObjectSpread(next.heap,next.state,{$:'normalCompletion',value:number(99)});next=drain(next.heap,next);assert.equal(next.$,'objectSpreadCompleted');assert.equal(next.heap.objects.find(row=>row.identity===1n).properties.some(row=>row.key.$==='text'),false);
});
test('readonly target rejection retains preceding copied properties and skips later keys',()=>{
 const input=heap();input.objects[0].properties=[{$:'ordinaryProperty',key:{$:'symbol',identity:9n},descriptor:data(number(3),false)}];const next=drain(input,beginJavaScriptObjectAssignment(input,object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadError');assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('field')).descriptor.value.value,7);assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,{$:'symbol',identity:9n}).descriptor.value.value,3);
});
test('Array targets retain actual length semantics and RangeError classification',()=>{
 const input=heap(),allocated=allocateJavaScriptArray(input,4n,object(1n),3);assert.equal(allocated.$,'arrayNormal');input.objects=allocated.heap.objects;input.objects[1].properties=[property(text('length'),number(1))];let next=drain(input,beginJavaScriptObjectAssignment(input,object(4n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadCompleted');assert.equal(readJavaScriptArrayLength(next.heap.objects.find(row=>row.identity===4n)),1);
 input.objects[1].properties=[property(text('length'),number(1.5))];next=drain(input,beginJavaScriptObjectAssignment(input,object(4n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadError');assert.equal(next.errorClass,'RangeError');
});
test('nullish sources skip copying and String sources expose exact UTF16 indices',()=>{
 const input=heap();let next=drain(input,beginJavaScriptObjectAssignment(input,object(1n),absent,{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadCompleted');next=drain(input,beginJavaScriptObjectAssignment(input,object(1n),{$:'text',units:[0xd83d,0xde00]},{$:'symbol',identity:9n}));assert.equal(next.$,'objectSpreadCompleted');assert.deepEqual(locateJavaScriptOrdinaryProperty(next.heap,1n,text('1')).descriptor.value,{$:'text',units:[0xde00]});assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('length')).found,false);
});
