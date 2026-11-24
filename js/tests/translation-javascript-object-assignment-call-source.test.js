// Deferred direct-VM controls: authoring is not execution evidence.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptObjectAssignmentFunction} from '../src/translation/javascript-object-assignment-call-source.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation} from '../src/translation/javascript-call-machine.js';
import {createJavaScriptOrdinarySetDescriptor,defineJavaScriptOrdinaryProperty,locateJavaScriptOrdinaryProperty} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
function fixture(){return {$:'ordinaryHeap',symbols:[99n],objects:[1n,2n,3n,4n,5n,6n,7n,9n,10n,11n,12n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:identity===7n||identity===11n,properties:[]}))};}
function property(heap,identity,key,descriptor){const result=defineJavaScriptOrdinaryProperty(heap,identity,typeof key==='string'?text(key):key,descriptor);assert.equal(result.success,true);return result.heap;}
function data(heap,identity,key,value){return property(heap,identity,key,createJavaScriptOrdinarySetDescriptor(value,true));}
function own(heap,identity,key){return locateJavaScriptOrdinaryProperty(heap,identity,typeof key==='string'?text(key):key);}
function program(functions=[]){return {$:'callProgram',realms:[8n],exoticKey:{$:'symbol',identity:99n},functions:[compileJavaScriptObjectAssignmentFunction(7n,8n,roots),...functions]};}
function execute(heap,args,functions=[],identities=[]){const code=program(functions),entry=beginJavaScriptCallMachine(code,heap,7n,{$:'null'},args);assert.equal(entry.$,'machineContinue',entry.reason);return runJavaScriptCallMachine(code,entry.machine,4096,identities).transition;}
function callback(instructions,registers=6){return {$:'callFunction',identity:11n,realm:8n,receiverMode:'strict',lexicalReceiver:absent,parameters:1,registers,captures:[],instructions};}

test('Object assignment requires the actual complete wrapper-root authority',()=>{
 assert.equal(compileJavaScriptObjectAssignmentFunction(7n,8n,undefined),undefined);
 for(const field of Object.keys(roots).filter(key=>key!=='$'))for(const value of [{$:'null'},object(1n,true)])assert.equal(compileJavaScriptObjectAssignmentFunction(7n,8n,{...roots,[field]:value}),undefined);
});
test('Object assignment preserves existing target identity and consumes the entire actual argument list',()=>{
 let heap=data(fixture(),10n,'first',number(1));heap=data(heap,12n,'first',number(2));heap=data(heap,12n,{$:'symbol',identity:99n},number(3));
 const result=execute(heap,[object(9n),object(10n),{$:'null'},absent,number(8),object(12n)]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,object(9n));assert.deepEqual(own(result.heap,9n,'first').descriptor.value,number(2));assert.deepEqual(own(result.heap,9n,{$:'symbol',identity:99n}).descriptor.value,number(3));
 const empty=execute(fixture(),[object(9n)]);assert.equal(empty.$,'machineCompleted');assert.deepEqual(empty.value,object(9n));
});
test('primitive targets retain real wrapper slots and the selected realm prototype',()=>{
 for(const [input,prototype]of [[number(7),roots.numberPrototype],[{$:'boolean',value:false},roots.booleanPrototype],[{$:'integer',value:7n},roots.integerPrototype],[{$:'symbol',identity:99n},roots.symbolPrototype]]){
  const heap=data(fixture(),10n,'copied',number(9)),result=execute(heap,[input,object(10n)],[],[20n]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,object(20n));const wrapper=result.heap.objects.find(row=>row.identity===20n);assert.deepEqual(wrapper.prototype,prototype);assert.deepEqual(wrapper.primitiveData,input);assert.deepEqual(own(result.heap,20n,'copied').descriptor.value,number(9));
 }
});
test('String target retains exact UTF16 and immutable indices while accepting later properties',()=>{
 const heap=data(fixture(),10n,'extra',number(9)),result=execute(heap,[text('\ud83d\ude00\ud800'),object(10n)],[],[20n]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,object(20n));assert.deepEqual(result.heap.objects.find(row=>row.identity===20n).prototype,roots.stringPrototype);
 for(const [index,unit]of [[0,0xd83d],[1,0xde00],[2,0xd800]]){const descriptor=own(result.heap,20n,String(index)).descriptor;assert.deepEqual(descriptor.value,{$:'text',units:[unit]});assert.equal(descriptor.writable,false);assert.equal(descriptor.configurable,false);assert.equal(descriptor.enumerable,true);}
 assert.deepEqual(own(result.heap,20n,'length').descriptor.value,number(3));assert.deepEqual(own(result.heap,20n,'extra').descriptor.value,number(9));
});
test('String target strict index Set raises TypeError after completed earlier writes',()=>{
 let heap=data(fixture(),10n,'before',number(1));heap=data(heap,12n,'0',text('x'));heap=data(heap,12n,'later',number(3));const result=execute(heap,[text('a'),object(10n),object(12n)],[],[20n]);assert.equal(result.$,'machineAllocateError');assert.equal(result.errorClass,'TypeError');assert.deepEqual(own(result.machine.heap,20n,'before').descriptor.value,number(1));assert.deepEqual(own(result.machine.heap,20n,'0').descriptor.value,text('a'));assert.equal(own(result.machine.heap,20n,'later').found,false);
});
test('nullish and omitted targets fail before any source getter',()=>{
 const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.getter=object(11n,true);const heap=property(fixture(),10n,'field',descriptor),getter=callback([{$:'constant',destination:2,value:text('getter ran')},{$:'throw',source:2}]);
 for(const args of [[],[absent,object(10n)],[{$:'null'},object(10n)]]){const result=execute(heap,args,[getter]);assert.equal(result.$,'machineAllocateError');assert.equal(result.errorClass,'TypeError');assert.deepEqual(result.machine.heap,heap);}
});
test('wrapper allocation suspends before source processing and retains original target input',()=>{
 const heap=data(fixture(),10n,'field',number(9)),code=program(),entry=beginJavaScriptCallMachine(code,heap,7n,absent,[number(7),object(10n)]),request=runJavaScriptCallMachine(code,entry.machine,4096,[]).transition;assert.equal(request.$,'machineAllocateObject');assert.deepEqual(request.machine.heap,heap);
 const resumed=resumeJavaScriptCallObjectAllocation(code,request.machine,20n);assert.equal(resumed.$,'machineContinue');const result=runJavaScriptCallMachine(code,resumed.machine,4096,[]).transition;assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,object(20n));assert.deepEqual(own(result.heap,20n,'field').descriptor.value,number(9));
});
test('inherited setter receives the boxed target and assigned value',()=>{
 const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.setter=object(11n,true);let heap=property(fixture(),3n,'field',descriptor);heap=data(heap,10n,'field',number(9));
 const setter=callback([{$:'constant',destination:2,value:text('seen')},{$:'ordinarySet',destination:3,object:0,key:2,value:1,receiver:0},{$:'return',source:1}]);const result=execute(heap,[number(7),object(10n)],[setter],[20n]);assert.equal(result.$,'machineCompleted');assert.deepEqual(result.value,object(20n));assert.deepEqual(own(result.heap,20n,'seen').descriptor.value,number(9));assert.equal(own(result.heap,20n,'field').found,false);
});
test('source getter abrupt completion retains original throw and earlier writes',()=>{
 const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor.getter=object(11n,true);let heap=data(fixture(),10n,'first',number(1));heap=property(heap,10n,'abrupt',descriptor);heap=data(heap,12n,'later',number(3));const getter=callback([{$:'constant',destination:2,value:object(12n)},{$:'throw',source:2}]);
 const result=execute(heap,[number(7),object(10n),object(12n)],[getter],[20n]);assert.equal(result.$,'machineThrow');assert.deepEqual(result.value,object(12n));assert.deepEqual(own(result.heap,20n,'first').descriptor.value,number(1));assert.equal(own(result.heap,20n,'later').found,false);
});
test('String sources copy UTF16 units into primitive target wrappers without copying length',()=>{
 const result=execute(fixture(),[number(7),text('\ud83d\ude00\ud800')],[],[20n]);assert.equal(result.$,'machineCompleted');for(const [key,unit]of [['0',0xd83d],['1',0xde00],['2',0xd800]])assert.deepEqual(own(result.heap,20n,key).descriptor.value,{$:'text',units:[unit]});assert.equal(own(result.heap,20n,'length').found,false);
});
