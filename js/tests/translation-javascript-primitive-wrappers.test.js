// Deferred source/kernel controls. No represented execution occurs on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {beginJavaScriptPrimitiveWrapper,resumeJavaScriptPrimitiveWrapper,readJavaScriptPrimitiveWrapperStateError} from '../src/translation/javascript-primitive-wrappers.js';
import {readJavaScriptOrdinaryHeapError,findJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {beginJavaScriptObjectObservation} from '../src/translation/javascript-object-integrity.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {javascriptPrimitiveWrapperAllocations as allocations,javascriptPrimitiveWrapperOriginals as originals,javascriptPrimitiveWrapperSourceVersions as versions} from './fixtures/javascript-primitive-wrappers.mjs';
const object=identity=>({$:'object',identity,callable:false}),text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
function ordinary(identity,prototype={$:'null'},properties=[]){return {$:'ordinaryObject',identity,prototype,extensible:true,callable:false,properties};}
function baseHeap(){return {$:'ordinaryHeap',symbols:[41n],objects:[ordinary(1n),ordinary(2n,object(1n)),{...ordinary(3n,object(1n)),primitiveData:{$:'number',value:0}},{...ordinary(4n,object(1n)),primitiveData:{$:'boolean',value:false}},ordinary(5n,object(1n)),ordinary(6n,object(1n)),ordinary(19n,object(1n)),{...ordinary(20n,object(1n)),callable:true}]};}
for(const fixture of allocations)test(fixture.name,()=>{
 const heap=baseHeap(),before=structuredClone(heap),prototype=object(fixture.prototype),request=beginJavaScriptPrimitiveWrapper(heap,fixture.input,prototype);
 assert.equal(request.$,'primitiveWrapperAllocate');assert.strictEqual(request.heap,heap);assert.deepEqual(request.prototype,prototype);assert.equal(readJavaScriptPrimitiveWrapperStateError(heap,request.state),'');
 const result=resumeJavaScriptPrimitiveWrapper(heap,request.state,43n);assert.equal(result.$,'primitiveWrapperCompleted');assert.equal(readJavaScriptOrdinaryHeapError(result.heap),'');assert.deepEqual(heap,before);assert.deepEqual(result.value,object(43n));
 const wrapper=result.heap.objects[findJavaScriptOrdinaryObject(result.heap.objects,43n)];assert.deepEqual(wrapper.prototype,prototype);assert.equal(wrapper.callable,false);assert.equal(wrapper.extensible,true);
 assert.equal(Object.hasOwn(wrapper,'primitiveData'),fixture.slot==='primitiveData');assert.equal(Object.hasOwn(wrapper,'stringData'),fixture.slot==='stringData');
 if(fixture.slot==='primitiveData'){assert.deepEqual(wrapper.primitiveData,fixture.input);assert.equal(wrapper.properties.length,0);const operation=fixture.input.$==='number'?'Number.valueOf':fixture.input.$==='boolean'?'Boolean.valueOf':fixture.input.$==='integer'?'BigInt.valueOf':'Symbol.valueOf',read=beginJavaScriptObjectObservation(result.heap,operation,result.value,{$:'undefined'});assert.equal(read.$,'objectIntegrityCompleted');assert.deepEqual(read.value,fixture.input);}
 else if(fixture.slot==='stringData'){
  assert.deepEqual(wrapper.stringData,fixture.input);assert.equal(wrapper.properties.length,fixture.units.length+1);
  for(const [index,unit] of fixture.units.entries()){const property=wrapper.properties[index];assert.deepEqual(property.key,text(String(index)));assert.deepEqual(property.descriptor.value,{$:'text',units:[unit]});for(const key of ['configurablePresent','enumerablePresent','valuePresent','writablePresent'])assert.equal(property.descriptor[key],true);assert.equal(property.descriptor.enumerable,true);assert.equal(property.descriptor.writable,false);assert.equal(property.descriptor.configurable,false);assert.equal(property.descriptor.getterPresent,false);assert.equal(property.descriptor.setterPresent,false);}
  const length=wrapper.properties.at(-1);assert.deepEqual(length.key,text('length'));assert.deepEqual(length.descriptor.value,{$:'number',value:fixture.units.length});assert.equal(length.descriptor.enumerable,false);assert.equal(length.descriptor.writable,false);assert.equal(length.descriptor.configurable,false);
 }else assert.equal(wrapper.properties.length,0);
});
test('Object Call passthrough includes ordinary callable and genuine Array objects without allocation',()=>{
 const heap=baseHeap();for(const input of [object(19n),{$:'object',identity:20n,callable:true}]){const result=beginJavaScriptPrimitiveWrapper(heap,input,{$:'undefined'});assert.equal(result.$,'primitiveWrapperCompleted');assert.strictEqual(result.heap,heap);assert.strictEqual(result.value,input);}
 const array=allocateJavaScriptArray(heap,43n,object(1n),0);assert.equal(array.$,'arrayNormal');const result=beginJavaScriptPrimitiveWrapper(array.heap,array.value,{$:'undefined'});assert.equal(result.$,'primitiveWrapperCompleted');assert.strictEqual(result.heap,array.heap);assert.deepEqual(result.value,array.value);
});
test('wrapper slots depend on the input rather than resembling a standard prototype',()=>{
 const heap=baseHeap(),plain=beginJavaScriptPrimitiveWrapper(heap,{$:'null'},object(3n)),allocated=resumeJavaScriptPrimitiveWrapper(heap,plain.state,43n),wrapper=allocated.heap.objects[findJavaScriptOrdinaryObject(allocated.heap.objects,43n)];assert.equal(Object.hasOwn(wrapper,'primitiveData'),false);assert.equal(beginJavaScriptObjectObservation(allocated.heap,'Number.valueOf',allocated.value,{$:'undefined'}).$,'objectIntegrityError');
 const request=beginJavaScriptPrimitiveWrapper(heap,{$:'number',value:7},object(2n)),number=resumeJavaScriptPrimitiveWrapper(heap,request.state,44n),row=number.heap.objects[findJavaScriptOrdinaryObject(number.heap.objects,44n)];assert.deepEqual(row.primitiveData,{$:'number',value:7});assert.equal(Object.hasOwn(row,'stringData'),false);
});
test('wrapper allocation rejects collisions invalid identities and forged allocation inputs atomically',()=>{
 const heap=baseHeap(),request=beginJavaScriptPrimitiveWrapper(heap,{$:'boolean',value:false},object(4n));
 for(const identity of [0n,-1n,1n,41n,18446744073709551616n]){const result=resumeJavaScriptPrimitiveWrapper(heap,request.state,identity);assert.equal(result.$,'primitiveWrapperInvalid');assert.strictEqual(result.heap,heap);}
 for(const state of [{...request.state,input:object(19n)},{...request.state,input:{$:'symbol',identity:42n}},{...request.state,prototype:{$:'number',value:7}}]){assert.notEqual(readJavaScriptPrimitiveWrapperStateError(heap,state),'');assert.equal(resumeJavaScriptPrimitiveWrapper(heap,state,43n).$,'primitiveWrapperInvalid');}
});
test('String wrapper work bounds precede identity allocation and preserve their complete input',()=>{
 const heap=baseHeap(),input={$:'sourceText',profile:'inspectable UTF16 source text 1',text:'x'.repeat(4096)},large=beginJavaScriptPrimitiveWrapper(heap,input,object(2n));assert.equal(large.$,'primitiveWrapperResourceLimit');assert.strictEqual(large.heap,heap);assert.strictEqual(large.state.input,input);assert.match(large.reason,/own-property profile/);
 const boundary=beginJavaScriptPrimitiveWrapper(heap,{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'x'.repeat(4095)},object(2n));assert.equal(boundary.$,'primitiveWrapperAllocate');
});
test('wrapper object and shared property budgets remain resource requests before fresh identity consumption',()=>{
 const full={$:'ordinaryHeap',symbols:[],objects:Array.from({length:4096},(_,at)=>ordinary(BigInt(at+1)))};assert.equal(readJavaScriptOrdinaryHeapError(full),'');const result=beginJavaScriptPrimitiveWrapper(full,{$:'number',value:7},object(1n));assert.equal(result.$,'primitiveWrapperResourceLimit');assert.strictEqual(result.heap,full);assert.match(result.reason,/Object identity profile/);
 const properties=[0,1,2,3].map(index=>{const descriptor=createJavaScriptPropertyDescriptor('data',false,false);descriptor.value={$:'number',value:0};return {$:'ordinaryProperty',key:text(String(index)),descriptor};}),crowded={$:'ordinaryHeap',symbols:[],objects:Array.from({length:4095},(_,at)=>ordinary(BigInt(at+1),{$:'null'},properties))};assert.equal(readJavaScriptOrdinaryHeapError(crowded),'');const string=beginJavaScriptPrimitiveWrapper(crowded,text('abcd'),object(1n));assert.equal(string.$,'primitiveWrapperResourceLimit');assert.strictEqual(string.heap,crowded);assert.match(string.reason,/shared own-property profile/);
});
for(const fixture of originals)test(fixture.name,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.equal(major===24||major===22&&minor>=15,true,'uncovered Object wrapper Node range: '+versions.node);
 const capture=captureJavaScriptOrdinaryClassModule(fixture.source,{identifier:'literal-primitive-wrappers.js',entries:fixture.entries});assert.deepEqual(checkJavaScriptOrdinaryClassModule(capture),capture);const instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:101n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(fixture.entry,fixture.arguments.map(instance.embedding.encode)),{steps:65536}).transition;assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),fixture.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn read;',[])();assert.deepEqual(original(...fixture.arguments),fixture.expected);
});
