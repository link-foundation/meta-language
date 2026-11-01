// Deferred source numeric controls; do not execute during bulk drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol} from '../src/translation/javascript-scalar-conversion-protocol.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
const number=value=>({$:'number',value}),integer=value=>({$:'integer',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))}),object=(identity,callable=false)=>({$:'object',identity,callable});
test('ToNumeric retains BigInt while primitive ToNumber rejects it',()=>{
 assert.deepEqual(executeJavaScriptCallPrimitive('coerce primitive numeric',[integer(9007199254740993n)]),{$:'primitiveValue',value:integer(9007199254740993n)});
 const rejected=executeJavaScriptCallPrimitive('coerce primitive number',[integer(7n)]);assert.equal(rejected.$,'primitiveError');assert.equal(rejected.errorClass,'TypeError');
 for(const [input,value]of [[text('7'),number(7)],[{$:'null'},number(0)],[{$:'boolean',value:true},number(1)],[{$:'undefined'},number(NaN)]])assert.deepEqual(executeJavaScriptCallPrimitive('coerce primitive numeric',[input]),{$:'primitiveValue',value});
});
test('numeric updates preserve Number special values and exact BigInt steps',()=>{
 for(const [operation,input,expected]of [['increment numeric value',number(-0),number(1)],['decrement numeric value',number(Infinity),number(Infinity)],['increment numeric value',number(NaN),number(NaN)],['negate numeric value',number(0),number(-0)],['increment numeric value',integer(9007199254740993n),integer(9007199254740994n)],['decrement numeric value',integer(7n),integer(6n)],['negate numeric value',integer(7n),integer(-7n)]])assert.deepEqual(executeJavaScriptCallPrimitive(operation,[input]),{$:'primitiveValue',value:expected});
});
test('ToNumeric uses its actual number-hint object callback and retains BigInt completion',()=>{
 let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};for(const identity of [1n,2n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===2n?{...row,callable:true}:row)};heap=defineJavaScriptOrdinaryProperty(heap,1n,text('valueOf'),createJavaScriptOrdinarySetDescriptor(object(2n,true),true)).heap;
 let current=beginJavaScriptScalarProtocol(heap,object(1n),{$:'symbol',identity:99n},'coerce primitive numeric');for(let at=0;at<16&&current.$==='scalarContinuation';at++)current=stepJavaScriptScalarProtocol(heap,current.state,'coerce primitive numeric');assert.equal(current.$,'scalarCall');assert.deepEqual(current.callee,object(2n,true));assert.deepEqual(current.receiver,object(1n));assert.deepEqual(current.arguments,[]);
 const completed=resumeJavaScriptScalarProtocol(heap,current.state,{$:'normalCompletion',value:integer(7n)},'coerce primitive numeric');assert.deepEqual(completed,{$:'scalarCompleted',value:integer(7n)});
});
