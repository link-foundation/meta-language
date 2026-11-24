import {readSourceMachineTrace} from './support/source-machine-trace.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {captureJavaScriptProjectDataSource} from '../src/translation/javascript-project-data-source.js';
import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
// Deferred species-selection controls. Requests are not allocated result Objects.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptArraySpeciesSelection,stepJavaScriptArraySpeciesSelection,resumeJavaScriptArraySpeciesGetter} from '../src/translation/javascript-array-species.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))}),absent={$:'undefined'},speciesKey={$:'symbol',identity:99n};
function data(heap,identity,key,value){return defineJavaScriptOrdinaryProperty(heap,identity,key,createJavaScriptOrdinarySetDescriptor(value,true)).heap;}
function getter(heap,identity,key){return defineJavaScriptOrdinaryProperty(heap,identity,key,{configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:absent,writablePresent:false,writable:false,getterPresent:true,getter:object(6n,true),setterPresent:true,setter:absent}).heap;}
function setup(){
 let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};for(const identity of [1n,2n,4n,5n,6n,7n,8n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>[2n,4n,6n,7n,8n].includes(row.identity)?{...row,callable:true}:row)};heap=allocateJavaScriptArray(heap,3n,object(1n),2).heap;heap=data(heap,3n,text('constructor'),object(2n,true));heap=getter(heap,2n,speciesKey);
 const directory={$:'arraySpeciesDirectory',realm:1n,arrayConstructor:object(2n,true),arrayPrototype:object(1n),speciesKey,constructors:[
  {$:'arrayConstructorFact',identity:2n,realm:1n,constructable:true,intrinsicArray:true},{$:'arrayConstructorFact',identity:4n,realm:1n,constructable:true,intrinsicArray:false},{$:'arrayConstructorFact',identity:6n,realm:1n,constructable:false,intrinsicArray:false},{$:'arrayConstructorFact',identity:7n,realm:2n,constructable:true,intrinsicArray:true},{$:'arrayConstructorFact',identity:8n,realm:1n,constructable:false,intrinsicArray:false},
 ]};return {heap,directory};
}
function advance(directory,transition){let current=transition;for(let at=0;at<32&&current.$==='arraySpeciesContinue';at++)current=stepJavaScriptArraySpeciesSelection(current.heap,directory,current.state);assert.notEqual(current.$,'arraySpeciesContinue');return current;}
const start=(heap,directory,original=object(3n),length=2)=>advance(directory,beginJavaScriptArraySpeciesSelection(heap,directory,original,length));
test('ordinary non-Array receivers bypass constructor/species getters entirely',()=>{
 let {heap,directory}=setup();heap=getter(heap,5n,text('constructor'));const actual=start(heap,directory,object(5n));assert.equal(actual.$,'arraySpeciesAllocate');assert.deepEqual(actual.prototype,object(1n));assert.equal(actual.length,2);assert.deepEqual(actual.heap,heap);
});
test('standard species getter requests retain actual receiver and constructor selection',()=>{
 const {heap,directory}=setup(),first=start(heap,directory);assert.equal(first.$,'arraySpeciesCall');assert.equal(first.state.phase,'species');assert.deepEqual(first.callee,object(6n,true));assert.deepEqual(first.receiver,object(2n,true));assert.deepEqual(first.arguments,[]);
 const actual=advance(directory,resumeJavaScriptArraySpeciesGetter(heap,directory,first.state,{$:'normalCompletion',value:object(2n,true)}));assert.equal(actual.$,'arraySpeciesConstruct');assert.deepEqual(actual.constructor,object(2n,true));assert.deepEqual(actual.arguments,[number(2)]);assert.equal(actual.fact.intrinsicArray,true);
});
test('a foreign intrinsic Array constructor normalizes before any species getter runs',()=>{
 let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),object(7n,true));heap=getter(heap,7n,speciesKey);const actual=start(heap,directory);assert.equal(actual.$,'arraySpeciesAllocate');assert.deepEqual(actual.prototype,object(1n));assert.equal(actual.state.realm,1n);
});
test('a nonconstructor Object can select a genuine custom constructor through species',()=>{
 let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),object(5n));heap=data(heap,5n,speciesKey,object(4n,true));const actual=start(heap,directory);assert.equal(actual.$,'arraySpeciesConstruct');assert.deepEqual(actual.constructor,object(4n,true));assert.equal(actual.fact.intrinsicArray,false);
});
test('missing constructor authority suspends exact selection without guessing TypeError',()=>{
 let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),object(5n));heap=data(heap,5n,speciesKey,object(4n,true));const partial={...directory,constructors:directory.constructors.filter(row=>row.identity!==4n)},first=start(heap,partial);assert.equal(first.$,'arraySpeciesNeedAuthority');assert.deepEqual(first.constructor,object(4n,true));assert.equal(first.state.phase,'species classification');
 heap=data(first.heap,5n,speciesKey,object(7n,true));const actual=advance(directory,stepJavaScriptArraySpeciesSelection(heap,directory,first.state));assert.equal(actual.$,'arraySpeciesConstruct');assert.deepEqual(actual.constructor,object(4n,true));assert.deepEqual(actual.heap,heap);
});
test('null species defaults but null constructor and nonconstructable selected methods throw',()=>{
 for(const selected of [{$:'null'},number(7),object(8n,true)]){let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),object(5n));heap=data(heap,5n,speciesKey,selected);heap=data(heap,8n,text('prototype'),object(1n));const actual=start(heap,directory);assert.equal(actual.$,selected.$==='null'?'arraySpeciesAllocate':'arraySpeciesError');if(selected.$!=='null')assert.equal(actual.errorClass,'TypeError');}
 let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),{$:'null'});const invalid=start(heap,directory);assert.equal(invalid.$,'arraySpeciesError');assert.equal(invalid.errorClass,'TypeError');
});
test('species getters preserve original abrupt values and current shared effects',()=>{
 const {heap,directory}=setup(),first=start(heap,directory),changed=data(first.heap,3n,text('effect'),number(7));assert.equal(first.$,'arraySpeciesCall');const actual=resumeJavaScriptArraySpeciesGetter(changed,directory,first.state,{$:'throwCompletion',value:text('original')});assert.equal(actual.$,'arraySpeciesThrow');assert.deepEqual(actual.heap,changed);assert.deepEqual(actual.value,text('original'));assert.deepEqual(stepJavaScriptArraySpeciesSelection(first.heap,directory,first.state),first);
});
test('selection does not preempt custom construction with the default Array length limit',()=>{
 let {heap,directory}=setup();heap=data(heap,3n,text('constructor'),absent);const allocate=start(heap,directory,object(3n),4294967296);assert.equal(allocate.$,'arraySpeciesAllocate');const rejected=allocateJavaScriptArray(heap,10n,allocate.prototype,allocate.length);assert.equal(rejected.$,'arrayError');assert.equal(rejected.errorClass,'RangeError');
 heap=data(heap,3n,text('constructor'),object(5n));heap=data(heap,5n,speciesKey,object(4n,true));const custom=start(heap,directory,object(3n),4294967296);assert.equal(custom.$,'arraySpeciesConstruct');assert.deepEqual(custom.arguments,[number(4294967296)]);
});
test('the actual species getter directory code returns its received this value in the VM',()=>{
 const {heap}=setup(),compiled=compileJavaScriptCollectionCallDirectory({$:'collectionCallableDirectory',realms:[1n],exoticKey:speciesKey,constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:6n,realm:1n,operation:'Array.species getter'}]});assert.equal(compiled.error,'');assert.deepEqual(compiled.program.functions[0].instructions,[{$:'copy',destination:1,source:0},{$:'return',source:1}]);
 const started=beginJavaScriptCallMachine(compiled.program,heap,6n,object(2n,true),[]);assert.equal(started.$,'machineContinue');const actual=runJavaScriptCallMachine(compiled.program,started.machine,8,[]).transition;assert.equal(actual.$,'machineCompleted');assert.deepEqual(actual.value,object(2n,true));
});

test('Node constructor facts retain checked declaration kinds independently of public record edits',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('class Box { method() { return 7; } } export function read() { return new Box().method(); }',{identifier:'literal-species-constructor-facts.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:113n}),module=instance.embedding.module;
 const method=module.capture.plan.functions.find(row=>row.$==='errorMethodFunctionPlan'),methodOwner=module.registry.owners.find(row=>row.source===method.identity),read=module.capture.plan.functions.find(row=>row.name==='read'),readOwner=module.registry.owners.find(row=>row.source===read.identity);
 const first=instance.collection.speciesDirectory();assert.equal(first.constructors.find(row=>row.identity===methodOwner.identity).constructable,false);assert.equal(first.constructors.find(row=>row.identity===readOwner.identity).constructable,true);
 first.constructors.find(row=>row.identity===methodOwner.identity).constructable=true;const facts=instance.embedding.sourceConstructorFacts();facts.find(row=>row.identity===methodOwner.identity).constructable=true;
 const second=instance.collection.speciesDirectory();assert.equal(second.constructors.find(row=>row.identity===methodOwner.identity).constructable,false);assert.equal(second.realm,113n);assert.equal(second.constructors.find(row=>row.identity===second.arrayConstructor.identity).intrinsicArray,true);
});

test('an authenticated additional ordinary source unit extends constructor facts dynamically',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('export function read() { return 7; }',{identifier:'literal-species-base-unit.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:127n});
 function helper() { return 7; }
 const source=captureJavaScriptProjectDataSource('export function helper() { return 7; }',{identifier:'literal-species-added-unit.js',entries:['helper']}),before=instance.collection.speciesDirectory(),unit=instance.embedding.attachSourceUnit(source,{helper},[]),owner=unit.registry.owners.find(row=>row.source===source.plan.functions[0].identity),after=instance.collection.speciesDirectory();
 assert.equal(before.constructors.some(row=>row.identity===owner.identity),false);assert.equal(after.constructors.find(row=>row.identity===owner.identity).constructable,true);assert.equal(after.constructors.find(row=>row.identity===owner.identity).realm,127n);
});


test('Node driver publishes authenticated added constructor facts and executes its portable body',()=>{
 const source='export function transform(input, selected) { input.constructor = {[Symbol.species]: selected}; return input.map(value => value * 2); }',capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-species-driver-update.js',entries:['transform']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:131n});
 function helper(length) { return length; }
 const added=captureJavaScriptProjectDataSource('export function helper(length) { return length; }',{identifier:'literal-species-driver-added.js',entries:['helper']}),unit=instance.embedding.attachSourceUnit(added,{helper},[]),owner=unit.registry.owners.find(row=>row.source===added.plan.functions[0].identity),before=instance.embedding.module.program;
 const started=instance.begin('transform',[instance.embedding.encode([2,4]),object(owner.identity,true)]),actual=runJavaScriptSourceTransition(instance.embedding,started,{steps:8192});assert.equal(actual.transition.$,'machineCompleted');assert.equal(actual.transition.value.$,'object');assert.notEqual(instance.embedding.module.program,before);
 const output=actual.transition.heap.objects.find(row=>row.identity===actual.transition.value.identity);assert.equal(output.$,'ordinaryObject');for(const [key,value] of [['0',4],['1',8]])assert.deepEqual(output.properties.find(row=>row.key.$==='text'&&row.key.units.join(',')===text(key).units.join(',')).descriptor.value,number(value));
 const methods=instance.embedding.module.program.functions.flatMap(fn=>fn.instructions).filter(row=>row.$==='arrayTransform');assert.equal(methods.some(row=>row.species.constructors.some(fact=>fact.identity===owner.identity&&fact.constructable)),true);
});
test('unregistered constructors remain suspended and metadata authority cannot be replaced after publication',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('export function transform(input, selected) { input.constructor = {[Symbol.species]: selected}; return input.map(value => value); }',{identifier:'literal-species-driver-unknown.js',entries:['transform']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:137n});
 function unavailable() { throw new Error('original constructor body must never run'); }
 const selected=instance.embedding.encode(unavailable),started=instance.begin('transform',[instance.embedding.encode([2]),selected]),actual=runJavaScriptSourceTransition(instance.embedding,started,{steps:8192});assert.equal(actual.transition.$,'machineNeedConstructorAuthority');assert.deepEqual(actual.transition.constructor,selected);const prior=instance.embedding.module.program,again=instance.embedding.resumeArrayConstructorAuthority(actual.transition);assert.equal(again.updated,false);assert.equal(again.transition,actual.transition);assert.equal(instance.embedding.module.program,prior);assert.throws(()=>instance.embedding.registerArrayConstructorAuthority(()=>instance.collection.speciesDirectory()),/before source publication/);
 assert.throws(()=>instance.embedding.resumeArrayConstructorAuthority({...actual.transition,constructor:instance.collection.speciesDirectory().arrayConstructor}),/actual selected constructor/);
});


test('source trace retains the exact constructor authority directory and changed portable program',()=>{
 const source='export function transform(input, selected) { input.constructor = {[Symbol.species]: selected}; return input.map(value => value * 2); }',capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-species-trace-update.js',entries:['transform']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:139n});
 function helper(length) { return length; }
 const added=captureJavaScriptProjectDataSource('export function helper(length) { return length; }',{identifier:'literal-species-trace-added.js',entries:['helper']}),unit=instance.embedding.attachSourceUnit(added,{helper},[]),owner=unit.registry.owners.find(row=>row.source===added.plan.functions[0].identity),started=instance.begin('transform',[instance.embedding.encode([2,4]),object(owner.identity,true)]),trace=readSourceMachineTrace(instance.embedding,started,{steps:8192});
 assert.equal(trace.transition.$,'machineCompleted');const updates=trace.events.filter(event=>event.operation==='extendConstructorAuthority');assert.equal(updates.length,1);assert.equal(updates[0].constructor.identity,String(owner.identity));assert.equal(updates[0].expected.$,'machineContinue');assert.equal(updates[0].directory.constructors.some(fact=>fact.identity===String(owner.identity)&&fact.constructable),true);assert.equal(trace.budgetExhausted,false);
});
