// Deferred ObjectDefineProperties controls. Importing never runs represented code.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptPropertyDefinitions,stepJavaScriptPropertyDefinitions,resumeJavaScriptPropertyDefinitionsCall,resumeJavaScriptPropertyDefinitionsAllocation} from '../src/translation/javascript-property-definitions.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {locateJavaScriptOrdinaryProperty,findJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),text=value=>({$:'text',units:Array.from(value,part=>part.charCodeAt(0))});
function data(value,configurable=true){const descriptor=createJavaScriptPropertyDescriptor('data',configurable,true);descriptor.value=value;descriptor.writable=configurable;return descriptor;}
function property(key,value){return {$:'ordinaryProperty',key,descriptor:data(value)};}
function row(identity,properties=[],callable=false){return {$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable,properties};}
function heap(){return {$:'ordinaryHeap',symbols:[9n],objects:[row(1n),row(2n,[property(text('first'),object(3n)),property(text('second'),object(4n))]),row(3n,[property(text('value'),{$:'number',value:7})]),row(4n,[property(text('value'),{$:'number',value:9})]),row(5n,[],true)]};}
function drain(heap,next){for(let step=0;step<128&&next.$==='propertyDefinitionsContinue';step++){heap=next.heap;next=stepJavaScriptPropertyDefinitions(heap,next.state);}return next;}
test('all descriptors convert before the first target definition and original target returns',()=>{
 const input=heap();let next=beginJavaScriptPropertyDefinitions(input,'Object.defineProperties',object(1n),object(2n),{$:'symbol',identity:9n}),converted=false;
 for(let step=0;step<64&&next.$==='propertyDefinitionsContinue';step++){if(next.state.phase==='define'&&!converted){converted=true;assert.equal(next.state.descriptors.length,2);assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('first')).found,false);}next=stepJavaScriptPropertyDefinitions(next.heap,next.state);}
 assert.equal(converted,true);assert.equal(next.$,'propertyDefinitionsCompleted');assert.deepEqual(next.value,object(1n));assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('first')).descriptor.value.value,7);assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('second')).descriptor.value.value,9);
});
test('invalid later descriptors throw before any target definition',()=>{
 const input=heap();input.objects[1].properties[1].descriptor.value={$:'number',value:3};const next=drain(input,beginJavaScriptPropertyDefinitions(input,'Object.defineProperties',object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'propertyDefinitionsError');assert.equal(next.errorClass,'TypeError');assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('first')).found,false);
});
test('later target rejection retains already completed earlier definitions',()=>{
 const input=heap();input.objects[0].properties=[{$:'ordinaryProperty',key:text('second'),descriptor:data({$:'number',value:3},false)}];const next=drain(input,beginJavaScriptPropertyDefinitions(input,'Object.defineProperties',object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'propertyDefinitionsError');assert.equal(next.errorClass,'TypeError');assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('first')).descriptor.value.value,7);assert.equal(locateJavaScriptOrdinaryProperty(next.heap,1n,text('second')).descriptor.value.value,3);
});
test('descriptor getters retain selected actual callback and abrupt completion identity',()=>{
 const input=heap(),descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.getter=object(5n,true);input.objects[1].properties[0].descriptor=descriptor;let next=drain(input,beginJavaScriptPropertyDefinitions(input,'Object.defineProperties',object(1n),object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'propertyDefinitionsCall');assert.deepEqual(next.receiver,object(2n));assert.deepEqual(next.arguments,[]);const thrown=object(3n);next=resumeJavaScriptPropertyDefinitionsCall(input,next.state,{$:'throwCompletion',value:thrown});assert.equal(next.$,'propertyDefinitionsThrow');assert.equal(next.value,thrown);
});
test('Object.create allocates the actual requested null prototype and applies optional descriptors',()=>{
 const input=heap();let next=drain(input,beginJavaScriptPropertyDefinitions(input,'Object.create',{$:'null'},object(2n),{$:'symbol',identity:9n}));assert.equal(next.$,'propertyDefinitionsAllocate');next=resumeJavaScriptPropertyDefinitionsAllocation(input,next.state,20n);next=drain(next.heap,next);assert.equal(next.$,'propertyDefinitionsCompleted');assert.deepEqual(next.heap.objects[findJavaScriptOrdinaryObject(next.heap.objects,20n)].prototype,{$:'null'});assert.equal(locateJavaScriptOrdinaryProperty(next.heap,20n,text('first')).descriptor.value.value,7);
 next=drain(input,beginJavaScriptPropertyDefinitions(input,'Object.create',object(1n),absent,{$:'symbol',identity:9n}));next=resumeJavaScriptPropertyDefinitionsAllocation(input,next.state,20n);next=drain(next.heap,next);assert.equal(next.$,'propertyDefinitionsCompleted');assert.equal(next.heap.objects[findJavaScriptOrdinaryObject(next.heap.objects,20n)].properties.length,0);
});
