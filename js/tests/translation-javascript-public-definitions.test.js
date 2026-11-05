// Deferred public descriptor/Array continuation controls; no execution on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptPropertyDefinition,stepJavaScriptPropertyDefinition,resumeJavaScriptPropertyDefinition,completeJavaScriptPropertyDefinition,continueJavaScriptPropertyArrayDefinition} from '../src/translation/javascript-property-definition.js';
import {compileJavaScriptObjectDefinitionFunction} from '../src/translation/javascript-collection-call-source.js';
import {createJavaScriptEmptyDescriptor} from '../src/translation/javascript-descriptor-conversion.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {createJavaScriptIndexedDataProperty} from '../src/translation/javascript-array-transform.js';
import {readJavaScriptArrayLength} from '../src/translation/javascript-array-objects.js';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,part=>part.charCodeAt(0))});
function data(value){const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.value=value;descriptor.writable=true;return descriptor;}
function setup(converted=false){let heap={$:'ordinaryHeap',symbols:[9n],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:false,properties:[]}]};const allocated=allocateJavaScriptArray(heap,2n,object(1n),3);assert.equal(allocated.$,'arrayNormal');heap=allocated.heap;for(const [index,value]of [[0,1],[1,3],[2,7]]){const write=createJavaScriptIndexedDataProperty(heap,object(2n),index,number(value));assert.equal(write.$,'indexedPropertyCreated');heap=write.heap;}
 const conversion={$:'ordinaryObject',identity:3n,prototype:{$:'null'},extensible:true,callable:false,properties:[{$:'ordinaryProperty',key:{$:'symbol',identity:9n},descriptor:data(object(4n,true))}]},method={$:'ordinaryObject',identity:4n,prototype:{$:'null'},extensible:true,callable:true,properties:[]},attributes={$:'ordinaryObject',identity:5n,prototype:{$:'null'},extensible:true,callable:false,properties:[{$:'ordinaryProperty',key:text('value'),descriptor:data(converted?object(3n):number(2))}]};return {...heap,objects:heap.objects.concat([conversion,method,attributes])};}
test('public defineProperty delegates target key and descriptor ordering to one shared opcode',()=>{
 const fn=compileJavaScriptObjectDefinitionFunction(1n,2n),code=fn.instructions;assert.equal(code.some(row=>row.$==='propertyKey'),false);const define=code.find(row=>row.$==='propertyDefinition');assert.equal(define.target,1);assert.equal(define.key,2);assert.equal(define.attributes,3);assert.equal(define.realm,2n);assert.equal(code[code.length-1].source,define.destination);
});
test('public Array length descriptors retain both selected number conversions and original target result',()=>{
 let heap=setup(true),next=beginJavaScriptPropertyDefinition(heap,object(2n),text('length'),object(5n),{$:'symbol',identity:9n}),calls=0;
 for(let step=0;step<64&&(next.$==='definitionContinuation'||next.$==='definitionCall');step++){heap=next.heap??heap;if(next.$==='definitionCall'){calls++;assert.deepEqual(next.callee,object(4n,true));assert.deepEqual(next.receiver,object(3n));next=resumeJavaScriptPropertyDefinition(heap,next.state,{$:'normalCompletion',value:number(2)});}else next=stepJavaScriptPropertyDefinition(heap,next.state);}
 assert.equal(next.$,'definitionCompleted');assert.equal(calls,2);assert.deepEqual(next.value,object(2n));assert.equal(readJavaScriptArrayLength(next.heap.objects.find(row=>row.identity===2n)),2);assert.ok(next.writes.includes('array length'));
});
test('Array failure transport preserves its actual partial heap and write witness while legacy completion remains available',()=>{
 const heap=setup(),writes=['array indices','array length'],projected=continueJavaScriptPropertyArrayDefinition(object(2n),text('length'),{$:'arrayAssignmentCompleted',heap,success:false,writes});assert.equal(projected.$,'definitionIntrinsicError');assert.equal(projected.errorClass,'TypeError');assert.equal(projected.heap,heap);assert.deepEqual(projected.writes,writes);const descriptor=createJavaScriptEmptyDescriptor();descriptor.valuePresent=true;descriptor.value=number(2);const legacy=completeJavaScriptPropertyDefinition(heap,object(2n),text('length'),descriptor);assert.equal(legacy.$,'definitionCompleted');assert.equal(readJavaScriptArrayLength(legacy.heap.objects.find(row=>row.identity===2n)),2);
});
