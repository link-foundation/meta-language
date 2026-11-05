// Deferred controls: drafting imports does not execute represented programs.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptInstanceOf,stepJavaScriptInstanceOf,resumeJavaScriptInstanceOf,readJavaScriptInstanceOfStateError} from '../src/translation/javascript-instance-of.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),key={$:'symbol',identity:9n},text=value=>({$:'text',units:Array.from(value,part=>part.charCodeAt(0))});
function data(value){const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=value;descriptor.writable=true;return descriptor;}
function row(identity,prototype,properties=[],callable=false){return {$:'ordinaryObject',identity,prototype,extensible:true,callable,properties};}
function property(key,descriptor){return {$:'ordinaryProperty',key,descriptor};}
function heap(){return {$:'ordinaryHeap',symbols:[9n],objects:[row(1n,{$:'null'}),row(2n,object(1n)),row(3n,{$:'null'},[property(text('prototype'),data(object(1n)))],true),row(4n,{$:'null'},[],true)]};}
function drain(heap,next){for(let step=0;step<32&&next.$==='instanceOfContinue';step++)next=stepJavaScriptInstanceOf(heap,next.state);return next;}
test('ordinary instance identity traverses actual prototypes and primitives avoid prototype getters',()=>{
 const input=heap();let next=drain(input,beginJavaScriptInstanceOf(input,'ordinary has instance',object(2n),object(3n,true),key));assert.equal(next.$,'instanceOfCompleted');assert.deepEqual(next.value,{$:'boolean',value:true});
 next=drain(input,beginJavaScriptInstanceOf(input,'ordinary has instance',{$:'number',value:7},object(4n,true),key));assert.equal(next.$,'instanceOfCompleted');assert.equal(next.value.value,false);
 next=drain(input,beginJavaScriptInstanceOf(input,'ordinary has instance',object(2n),object(4n,true),key));assert.equal(next.$,'instanceOfError');assert.equal(next.errorClass,'TypeError');
});
test('custom hasInstance getter and method retain actual receiver and left value exactly once',()=>{
 const input=heap(),descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.getter=object(4n,true);input.objects[0].properties=[property(key,descriptor)];
 let next=drain(input,beginJavaScriptInstanceOf(input,'instanceof',{$:'number',value:7},object(1n),key));assert.equal(next.$,'instanceOfCall');assert.deepEqual(next.receiver,object(1n));assert.deepEqual(next.arguments,[]);
 next=resumeJavaScriptInstanceOf(input,next.state,{$:'normalCompletion',value:object(3n,true)});assert.equal(next.$,'instanceOfCall');assert.deepEqual(next.receiver,object(1n));assert.deepEqual(next.arguments,[{$:'number',value:7}]);
 next=resumeJavaScriptInstanceOf(input,next.state,{$:'normalCompletion',value:object(2n)});assert.equal(next.$,'instanceOfCompleted');assert.equal(next.value.value,true);
});
test('instanceof rejects nonobjects and noncallable methods and propagates the selected throw',()=>{
 const input=heap();let next=drain(input,beginJavaScriptInstanceOf(input,'instanceof',absent,{$:'number',value:3},key));assert.equal(next.$,'instanceOfError');
 input.objects[0].properties=[property(key,data({$:'number',value:7}))];next=drain(input,beginJavaScriptInstanceOf(input,'instanceof',absent,object(1n),key));assert.equal(next.$,'instanceOfError');
 input.objects[0].properties=[property(key,data(object(4n,true)))];next=drain(input,beginJavaScriptInstanceOf(input,'instanceof',absent,object(1n),key));assert.equal(next.$,'instanceOfCall');const thrown=object(2n);next=resumeJavaScriptInstanceOf(input,next.state,{$:'throwCompletion',value:thrown});assert.equal(next.$,'instanceOfThrow');assert.equal(next.value,thrown);
});
test('known native Error prototypes participate without invoking a host instanceof body',()=>{
 const input=heap();input.objects[1]={...input.objects[1],$:'hostErrorObject',hostGet:object(4n,true),hostSet:object(4n,true)};const next=drain(input,beginJavaScriptInstanceOf(input,'ordinary has instance',object(2n),object(3n,true),key));assert.equal(next.$,'instanceOfCompleted');assert.equal(next.value.value,true);
});
test('continuation validation rejects replaced callback operands and invented prototype paths',()=>{
 const input=heap();input.objects[0].properties=[property(key,data(object(4n,true)))];const next=drain(input,beginJavaScriptInstanceOf(input,'instanceof',{$:'number',value:7},object(1n),key));assert.equal(next.$,'instanceOfCall');assert.notEqual(readJavaScriptInstanceOfStateError(input,{...next.state,call:{...next.state.call,arguments:[absent]}}),'');assert.notEqual(readJavaScriptInstanceOfStateError(input,{...next.state,visited:[1n,1n]}),'');
});
