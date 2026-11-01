// Deferred controls for the genuine two-conversion protocol. Never run while drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {beginJavaScriptArrayAssignment,stepJavaScriptArrayAssignment,resumeJavaScriptArrayAssignment} from '../src/translation/javascript-array-assignment.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor,planJavaScriptOrdinarySetWithArrayDefinition,planJavaScriptOrdinaryGet} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))}),exoticKey={$:'symbol',identity:99n};
function setup(){
 let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};heap=insertJavaScriptOrdinaryObject(heap,1n,{$:'null'}).heap;heap=allocateJavaScriptArray(heap,2n,object(1n),3).heap;
 heap=insertJavaScriptOrdinaryObject(heap,3n,object(1n)).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===3n?{...row,callable:true}:row)};heap=insertJavaScriptOrdinaryObject(heap,4n,object(1n)).heap;
 heap=defineJavaScriptOrdinaryProperty(heap,4n,text('valueOf'),createJavaScriptOrdinarySetDescriptor(object(3n,true),true)).heap;
 return heap;
}
function advance(transition){let current=transition;for(let step=0;step<32&&current.$==='arrayAssignmentContinue';step++)current=stepJavaScriptArrayAssignment(current.heap,current.state);assert.notEqual(current.$,'arrayAssignmentContinue');return current;}
function length(heap){return planJavaScriptOrdinaryGet(heap,2n,text('length'),object(2n)).value;}
test('ordinary Set defers Array length while preserving normal receiver checks',()=>{
 const heap=setup(),plan=planJavaScriptOrdinarySetWithArrayDefinition(heap,2n,text('length'),object(4n),object(2n));assert.equal(plan.action,'array definition');assert.equal(plan.definition.valuePresent,true);assert.equal(plan.definition.writablePresent,false);assert.deepEqual(plan.heap,heap);
 const readonly=defineJavaScriptOrdinaryProperty(heap,2n,text('length'),{...createJavaScriptOrdinarySetDescriptor(number(3),false),valuePresent:false,writablePresent:true,writable:false}).heap;
 const rejected=planJavaScriptOrdinarySetWithArrayDefinition(readonly,2n,text('length'),object(4n),object(2n));assert.equal(rejected.action,'complete');assert.equal(rejected.success,false);assert.deepEqual(rejected.heap,readonly);
});
test('two callbacks use the latest shared heap and only then read old length',()=>{
 const heap=setup(),descriptor=createJavaScriptOrdinarySetDescriptor(object(4n),false),first=advance(beginJavaScriptArrayAssignment(heap,2n,text('length'),descriptor,exoticKey));assert.equal(first.$,'arrayAssignmentCall');assert.deepEqual(first.callee,object(3n,true));assert.deepEqual(first.receiver,object(4n));assert.deepEqual(length(first.heap),number(3));
 const mutated=defineJavaScriptOrdinaryProperty(first.heap,2n,text('4'),createJavaScriptOrdinarySetDescriptor(number(9),true)).heap;
 const second=advance(resumeJavaScriptArrayAssignment(mutated,first.state,{$:'normalCompletion',value:number(1)}));assert.equal(second.$,'arrayAssignmentCall');assert.equal(second.state.definition.phase,'second number');assert.deepEqual(length(second.heap),number(5));
 const result=advance(resumeJavaScriptArrayAssignment(second.heap,second.state,{$:'normalCompletion',value:number(1)}));assert.equal(result.$,'arrayAssignmentCompleted');assert.equal(result.success,true);assert.deepEqual(length(result.heap),number(1));assert.deepEqual(planJavaScriptOrdinaryGet(result.heap,2n,text('4'),object(2n)).value,{$:'undefined'});
});
test('different numeric values produce RangeError after both retained callbacks',()=>{
 const first=advance(beginJavaScriptArrayAssignment(setup(),2n,text('length'),createJavaScriptOrdinarySetDescriptor(object(4n),false),exoticKey));const second=advance(resumeJavaScriptArrayAssignment(first.heap,first.state,{$:'normalCompletion',value:number(1)}));assert.equal(second.$,'arrayAssignmentCall');
 const changed=defineJavaScriptOrdinaryProperty(second.heap,4n,text('effect'),createJavaScriptOrdinarySetDescriptor(number(7),true)).heap;
 const result=advance(resumeJavaScriptArrayAssignment(changed,second.state,{$:'normalCompletion',value:number(2)}));assert.equal(result.$,'arrayAssignmentError');assert.equal(result.errorClass,'RangeError');assert.deepEqual(result.heap,changed);assert.deepEqual(length(result.heap),number(3));
});
test('abrupt first conversion skips the second and preserves its real thrown value',()=>{
 const first=advance(beginJavaScriptArrayAssignment(setup(),2n,text('length'),createJavaScriptOrdinarySetDescriptor(object(4n),false),exoticKey));const result=resumeJavaScriptArrayAssignment(first.heap,first.state,{$:'throwCompletion',value:text('original abrupt')});assert.equal(result.$,'arrayAssignmentThrow');assert.deepEqual(result.value,text('original abrupt'));assert.deepEqual(length(result.heap),number(3));
});
test('failed truncation retains partial deletion for the strict write guard',()=>{
 let heap=setup();for(const index of [1,2])heap=defineJavaScriptOrdinaryProperty(heap,2n,text(String(index)),{...createJavaScriptOrdinarySetDescriptor(number(index),true),configurable:index===2}).heap;
 const plan=planJavaScriptOrdinarySetWithArrayDefinition(heap,2n,text('length'),number(0),object(2n));assert.equal(plan.action,'array definition');
 const result=advance(beginJavaScriptArrayAssignment(plan.heap,2n,text('length'),plan.definition,exoticKey));assert.equal(result.$,'arrayAssignmentCompleted');assert.equal(result.success,false);assert.deepEqual(length(result.heap),number(2));assert.deepEqual(planJavaScriptOrdinaryGet(result.heap,2n,text('1'),object(2n)).value,number(1));assert.deepEqual(planJavaScriptOrdinaryGet(result.heap,2n,text('2'),object(2n)).value,{$:'undefined'});
});
