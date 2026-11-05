// Deferred independent engine and sparse-state controls. Do not run in draft phase.
import test from 'node:test';import assert from 'node:assert/strict';
import * as rules from '../src/translation/javascript-array-objects.js';
import {allocateJavaScriptArray,beginJavaScriptArrayPropertyDefinition,stepJavaScriptArrayPropertyDefinition,resumeJavaScriptArrayPropertyNumber,testJavaScriptArrayValue} from '../src/translation/javascript-arrays.js';
import {defineJavaScriptOrdinaryProperty,readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryOwnKeys,planJavaScriptOrdinaryGet,insertJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {javascriptArrayObjectDecisions} from './fixtures/javascript-array-objects.mjs';
const key=text=>({$:'text',units:Array.from(text,character=>character.charCodeAt(0))}),number=value=>({$:'number',value}),object=identity=>({$:'object',identity,callable:false});
function setup(length=0){const result=allocateJavaScriptArray({$:'ordinaryHeap',objects:[],symbols:[]},1n,{$:'null'},length);assert.equal(result.$,'arrayNormal');assert.equal(readJavaScriptOrdinaryHeapError(result.heap),'');return result.heap;}
function own(heap,name,value,configurable=true){const descriptor=createJavaScriptPropertyDescriptor('data',configurable,true);descriptor.writable=true;descriptor.value=value;const result=defineJavaScriptOrdinaryProperty(heap,1n,key(name),descriptor);assert.equal(result.success,true,result.error);return result.heap;}
function setLength(heap,value,responses){const descriptor=createJavaScriptPropertyDescriptor('data',false,false);descriptor.value=value;descriptor.writable=false;let result=beginJavaScriptArrayPropertyDefinition(heap,1n,key('length'),descriptor),index=0;assert.equal(result.$,'arrayDefinitionContinue');for(let count=0;count<8;count++){result=stepJavaScriptArrayPropertyDefinition(result.heap,result.state);if(result.$!=='arrayNumberRequest')break;const response=responses[index++];assert.ok(response);result=resumeJavaScriptArrayPropertyNumber(result.heap,result.state,response);assert.equal(result.$,'arrayDefinitionContinue');}assert.equal(index,2);return result;}
test('all sparse Array policy literals retain exact Number inputs',()=>{for(const row of javascriptArrayObjectDecisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
test('holes and index extension preserve genuine Array identity and own key order',()=>{
 let heap=setup(3);heap=own(heap,'1',number(7));heap=own(heap,'4',number(9));
 assert.equal(rules.readJavaScriptArrayLength(heap.objects[0]),5);assert.equal(testJavaScriptArrayValue(heap,object(1n)).array,true);
 assert.deepEqual(readJavaScriptOrdinaryOwnKeys(heap,1n).keys,[key('1'),key('4'),key('length')]);assert.equal(planJavaScriptOrdinaryGet(heap,1n,key('0'),object(1n)).value.$,'undefined');
 const original=Array(3);original[1]=7;original[4]=9;assert.deepEqual(Reflect.ownKeys(original),['1','4','length']);assert.equal(original.length,5);
 const huge=setup(4294967295);assert.equal(huge.objects[0].properties.length,1);assert.equal(rules.readJavaScriptArrayLength(huge.objects[0]),4294967295);
});
test('failed shrink retains exact partial deletion and final nonwritable length',()=>{
 let heap=setup();heap=own(heap,'1',number(1));heap=own(heap,'2',number(2),false);heap=own(heap,'4',number(4));
 const result=setLength(heap,number(1),[number(1),number(1)]);assert.equal(result.$,'arrayPropertyComplete');assert.equal(result.success,false);assert.equal(rules.readJavaScriptArrayLength(result.heap.objects[0]),3);
 assert.deepEqual(readJavaScriptOrdinaryOwnKeys(result.heap,1n).keys,[key('1'),key('2'),key('length')]);const length=result.heap.objects[0].properties.find(property=>rules.testJavaScriptArrayLengthKey(property.key));assert.equal(length.descriptor.writable,false);
 const original=[];original[1]=1;Object.defineProperty(original,'2',{value:2,configurable:false,writable:true,enumerable:true});original[4]=4;
 assert.equal(Reflect.defineProperty(original,'length',{value:1,writable:false}),false);assert.equal(original.length,3);assert.deepEqual(Reflect.ownKeys(original),['1','2','length']);assert.equal(Object.getOwnPropertyDescriptor(original,'length').writable,false);
});
test('both conversions complete before current length is read and effects survive errors',()=>{
 let heap=setup();heap=insertJavaScriptOrdinaryObject(heap,2n,{$:'null'}).heap;const descriptor=createJavaScriptPropertyDescriptor('data',false,false);descriptor.value=object(2n);descriptor.writable=true;
 let result=beginJavaScriptArrayPropertyDefinition(heap,1n,key('length'),descriptor);result=stepJavaScriptArrayPropertyDefinition(result.heap,result.state);assert.equal(result.$,'arrayNumberRequest');assert.deepEqual(result.input,object(2n));
 heap=own(result.heap,'8',number(8));result=resumeJavaScriptArrayPropertyNumber(heap,result.state,number(4));result=stepJavaScriptArrayPropertyDefinition(result.heap,result.state);assert.equal(result.$,'arrayNumberRequest');assert.deepEqual(result.input,object(2n));
 heap=own(result.heap,'7',number(7));result=resumeJavaScriptArrayPropertyNumber(heap,result.state,number(4));result=stepJavaScriptArrayPropertyDefinition(result.heap,result.state);assert.equal(result.$,'arrayPropertyComplete');assert.equal(result.success,true);assert.equal(rules.readJavaScriptArrayLength(result.heap.objects[0]),4);assert.deepEqual(readJavaScriptOrdinaryOwnKeys(result.heap,1n).keys,[key('length')]);
 const original=[];let calls=0;const value={valueOf(){original[calls++===0?8:7]=1;return 4;}};Object.defineProperty(original,'length',{value});assert.equal(calls,2);assert.equal(original.length,4);assert.deepEqual(Reflect.ownKeys(original),['length']);
 const mismatch=setLength(heap,object(2n),[number(4),number(5)]);assert.equal(mismatch.$,'arrayError');assert.equal(mismatch.errorClass,'RangeError');assert.deepEqual(mismatch.heap,heap);
});
