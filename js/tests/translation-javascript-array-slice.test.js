// Deferred shared-kernel controls. No represented execution occurs on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import {beginJavaScriptArraySlice,stepJavaScriptArraySlice,readJavaScriptArraySliceStateError,resumeJavaScriptArraySliceCall,resumeJavaScriptArraySliceAllocation,resumeJavaScriptArraySliceConstruction,readJavaScriptArraySliceRelativeIndex} from '../src/translation/javascript-array-slice.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,deleteJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor,locateJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))}),speciesKey={$:'symbol',identity:99n},exoticKey={$:'symbol',identity:100n};
function property(heap,identity,key,descriptor){const defined=defineJavaScriptOrdinaryProperty(heap,identity,typeof key==='string'?text(key):key,descriptor);assert.equal(defined.success,true,defined.error);return defined.heap;}
function data(heap,identity,key,value){const arrayLength=key==='length'&&heap.objects.some(row=>row.identity===identity&&row.$==='arrayObject');return property(heap,identity,key,createJavaScriptOrdinarySetDescriptor(value,!arrayLength));}
function accessor(heap,identity,key,kind,callee){const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor[kind]=callee;return property(heap,identity,key,descriptor);}
function setup(){
 let heap={$:'ordinaryHeap',symbols:[99n,100n],objects:[]};for(const identity of [1n,2n,4n,5n,6n,7n,8n,9n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;
 heap={...heap,objects:heap.objects.map(row=>[2n,4n,5n,9n].includes(row.identity)?{...row,callable:true}:row)};const array=allocateJavaScriptArray(heap,3n,object(1n),4);assert.equal(array.$,'arrayNormal');heap=array.heap;heap=data(heap,3n,'0',number(3));heap=data(heap,3n,'2',number(7));heap=data(heap,3n,'constructor',object(2n,true));heap=data(heap,2n,speciesKey,object(2n,true));
 const directory={$:'arraySpeciesDirectory',realm:1n,arrayConstructor:object(2n,true),arrayPrototype:object(1n),speciesKey,constructors:[{$:'arrayConstructorFact',identity:2n,constructable:true,realm:1n,intrinsicArray:true},{$:'arrayConstructorFact',identity:4n,constructable:true,realm:1n,intrinsicArray:false},{$:'arrayConstructorFact',identity:5n,constructable:false,realm:1n,intrinsicArray:false},{$:'arrayConstructorFact',identity:9n,constructable:false,realm:1n,intrinsicArray:false}]};return {heap,directory};
}
function advance(directory,transition){let current=transition;for(let at=0;at<128&&current.$==='arraySliceContinue';at++)current=stepJavaScriptArraySlice(current.heap,directory,current.state);assert.notEqual(current.$,'arraySliceContinue');return current;}
function begin(heap,directory,start=absent,end=absent,endPresent=false,receiver=object(3n)){return advance(directory,beginJavaScriptArraySlice(heap,directory,receiver,start,end,endPresent,exoticKey));}
function own(heap,identity,key){return locateJavaScriptOrdinaryProperty(heap,identity,text(key));}
function get(heap,identity,key){return planJavaScriptOrdinaryGet(heap,identity,text(key),object(identity)).value;}
function allocate(directory,transition){assert.equal(transition.$,'arraySliceAllocate');return advance(directory,resumeJavaScriptArraySliceAllocation(transition.heap,directory,transition.state,10n));}
function custom(heap){heap=data(heap,3n,'constructor',object(6n));return data(heap,6n,speciesKey,object(4n,true));}
const normal=value=>({$:'normalCompletion',value});

test('slice keeps holes, copies inherited properties and sets result length',()=>{
 let {heap,directory}=setup();heap=data(heap,1n,'1',number(5));const request=begin(heap,directory,number(1),number(4),true);assert.equal(request.length,3);const result=allocate(directory,request);assert.equal(result.$,'arraySliceCompleted');assert.deepEqual(get(result.heap,10n,'0'),number(5));assert.deepEqual(get(result.heap,10n,'1'),number(7));assert.equal(own(result.heap,10n,'2').found,false);assert.deepEqual(get(result.heap,10n,'length'),number(3));
});
test('slice clamps all relative coordinates including infinities, fractions, NaN and signed zero',()=>{
 for(const [value,expected]of [[-Infinity,0],[Infinity,4],[-7,0],[-2.9,2],[1.9,1],[NaN,0],[-0,0],[7,4]]){assert.equal(readJavaScriptArraySliceRelativeIndex(value,4),expected);assert.equal(Object.is(readJavaScriptArraySliceRelativeIndex(value,4),-0),false);}
 const {heap,directory}=setup();for(const [start,end,present,count]of [[-2,undefined,false,2],[Infinity,undefined,false,0],[-Infinity,Infinity,true,4],[3,1,true,0],[NaN,2.9,true,2]]){const request=begin(heap,directory,number(start),present?number(end):absent,present);assert.equal(request.$,'arraySliceAllocate');assert.equal(request.length,count);}
});
test('slice distinguishes missing and explicit undefined end from numeric zero',()=>{
 const {heap,directory}=setup();for(const present of [false,true]){const request=begin(heap,directory,number(1),absent,present);assert.equal(request.$,'arraySliceAllocate');assert.equal(request.length,3);assert.equal(request.state.endPresent,present);}
 assert.equal(begin(heap,directory,number(1),number(0),true).length,0);
 assert.equal(begin(heap,directory,number(1),number(2),false).$,'arraySliceInvalid');
});
test('slice saves ToLength before converting start then end and retains each selected operand',()=>{
 let {heap,directory}=setup();heap=data(heap,8n,'length',object(7n));heap=data(heap,7n,'valueOf',object(9n,true));heap=data(heap,6n,'valueOf',object(5n,true));heap=data(heap,7n,'0',number(77));heap=data(heap,7n,'toString',object(4n,true));
 const first=begin(heap,directory,object(6n),object(7n),true,object(8n));assert.equal(first.$,'arraySliceCall');assert.deepEqual(first.callee,object(9n,true));assert.deepEqual(first.receiver,object(7n));
 const second=advance(directory,resumeJavaScriptArraySliceCall(first.heap,directory,first.state,normal(number(4))));assert.equal(second.$,'arraySliceCall');assert.deepEqual(second.callee,object(5n,true));assert.equal(second.state.length,4);
 const changed=data(second.heap,8n,'length',number(0)),third=advance(directory,resumeJavaScriptArraySliceCall(changed,directory,second.state,normal(number(1))));assert.equal(third.$,'arraySliceCall');assert.deepEqual(third.callee,object(9n,true));assert.equal(third.state.length,4);
 const request=advance(directory,resumeJavaScriptArraySliceCall(third.heap,directory,third.state,normal(number(3))));assert.equal(request.$,'arraySliceAllocate');assert.equal(request.length,2);assert.equal(request.state.length,4);
});
test('generic slice never reads a non-Array receiver constructor or species',()=>{
 let {heap,directory}=setup();heap=data(heap,8n,'length',number(2));heap=data(heap,8n,'1',number(9));heap=accessor(heap,8n,'constructor','getter',object(5n,true));const request=begin(heap,directory,absent,absent,false,object(8n));assert.equal(request.$,'arraySliceAllocate');const result=allocate(directory,request);assert.equal(result.$,'arraySliceCompleted');assert.equal(own(result.heap,10n,'0').found,false);assert.deepEqual(get(result.heap,10n,'1'),number(9));
});
test('custom species receives selected count, uses own data definitions and final strict length setter',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,7n,'0','setter',object(5n,true));heap=accessor(heap,7n,'length','setter',object(9n,true));const request=begin(heap,directory,number(0),number(3),true);assert.equal(request.$,'arraySliceConstruct');assert.deepEqual(request.arguments,[number(3)]);assert.deepEqual(request.constructor,object(4n,true));
 const setter=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(setter.$,'arraySliceCall');assert.deepEqual(setter.callee,object(9n,true));assert.deepEqual(setter.receiver,object(7n));assert.deepEqual(setter.arguments,[number(3)]);assert.deepEqual(get(setter.heap,7n,'0'),number(3));assert.equal(own(setter.heap,7n,'1').found,false);assert.deepEqual(get(setter.heap,7n,'2'),number(7));
 const result=advance(directory,resumeJavaScriptArraySliceCall(setter.heap,directory,setter.state,normal(number(999))));assert.equal(result.$,'arraySliceCompleted');assert.deepEqual(result.value,object(7n));
});
test('final length Set throws after completed copied writes on a nonwritable custom result',()=>{
 let {heap,directory}=setup();heap=custom(heap);const descriptor=createJavaScriptOrdinarySetDescriptor(number(0),true);descriptor.writable=false;heap=property(heap,7n,'length',descriptor);const request=begin(heap,directory),result=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(result.$,'arraySliceError');assert.equal(result.errorClass,'TypeError');assert.deepEqual(get(result.heap,7n,'0'),number(3));assert.deepEqual(get(result.heap,7n,'2'),number(7));assert.deepEqual(get(result.heap,7n,'length'),number(0));
});
test('zero-count slice still constructs and performs the final strict length Set',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,7n,'length','setter',object(9n,true));const request=begin(heap,directory,number(4));assert.equal(request.$,'arraySliceConstruct');assert.deepEqual(request.arguments,[number(0)]);const setter=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(setter.$,'arraySliceCall');assert.deepEqual(setter.arguments,[number(0)]);
});
test('final slice length Set uses ArraySetLength on a genuine custom Array result',()=>{
 let {heap,directory}=setup();heap=custom(heap);const allocated=allocateJavaScriptArray(heap,11n,object(1n),5);assert.equal(allocated.$,'arrayNormal');heap=data(allocated.heap,11n,'4',number(99));const request=begin(heap,directory,number(0),number(2),true),result=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(11n))));assert.equal(result.$,'arraySliceCompleted');assert.deepEqual(result.value,object(11n));assert.deepEqual(get(result.heap,11n,'length'),number(2));assert.deepEqual(get(result.heap,11n,'0'),number(3));assert.equal(own(result.heap,11n,'4').found,false);
});
test('getter mutations affect following Has/Get but never the saved interval',()=>{
 let {heap,directory}=setup();heap=accessor(heap,3n,'0','getter',object(5n,true));const getter=allocate(directory,begin(heap,directory));assert.equal(getter.$,'arraySliceCall');assert.deepEqual(getter.receiver,object(3n));
 heap=deleteJavaScriptOrdinaryProperty(getter.heap,3n,text('2')).heap;heap=data(heap,1n,'1',number(11));heap=data(heap,3n,'length',number(1));const result=advance(directory,resumeJavaScriptArraySliceCall(heap,directory,getter.state,normal(number(9))));assert.equal(result.$,'arraySliceCompleted');assert.deepEqual(get(result.heap,10n,'0'),number(9));assert.deepEqual(get(result.heap,10n,'1'),number(11));assert.equal(own(result.heap,10n,'2').found,false);assert.deepEqual(get(result.heap,10n,'length'),number(4));
});
test('getter and final setter throws retain actual abrupt identity and completed writes',()=>{
 let {heap,directory}=setup();heap=accessor(heap,3n,'2','getter',object(5n,true));const getter=allocate(directory,begin(heap,directory)),changed=data(getter.heap,8n,'effect',number(9)),result=resumeJavaScriptArraySliceCall(changed,directory,getter.state,{$:'throwCompletion',value:object(6n)});assert.equal(result.$,'arraySliceThrow');assert.deepEqual(result.value,object(6n));assert.deepEqual(result.heap,changed);assert.deepEqual(get(result.heap,10n,'0'),number(3));
 heap=custom(setup().heap);heap=accessor(heap,7n,'length','setter',object(9n,true));const request=begin(heap,directory),setter=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(7n)))),abrupt=resumeJavaScriptArraySliceCall(setter.heap,directory,setter.state,{$:'throwCompletion',value:object(6n)});assert.equal(abrupt.$,'arraySliceThrow');assert.deepEqual(abrupt.value,object(6n));assert.deepEqual(get(abrupt.heap,7n,'2'),number(7));
});
test('completed getter values survive output property resource suspension without replay',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,3n,'0','getter',object(5n,true));for(let at=0;at<4096;at++)heap=data(heap,7n,'slot'+at,number(at));const request=begin(heap,directory),getter=advance(directory,resumeJavaScriptArraySliceConstruction(request.heap,directory,request.state,normal(object(7n)))),limited=advance(directory,resumeJavaScriptArraySliceCall(getter.heap,directory,getter.state,normal(number(13))));assert.equal(limited.$,'arraySliceResourceLimit');assert.equal(limited.state.phase,'write');assert.deepEqual(limited.state.value,number(13));
 heap=deleteJavaScriptOrdinaryProperty(limited.heap,7n,text('slot0')).heap;const next=stepJavaScriptArraySlice(heap,directory,limited.state);assert.equal(next.$,'arraySliceContinue');assert.deepEqual(get(next.heap,7n,'0'),number(13));assert.equal(next.state.index,1);
});
test('species authority can extend without repeating its retained constructor Get',()=>{
 let {heap,directory}=setup();heap=custom(heap);const partial={...directory,constructors:directory.constructors.filter(fact=>fact.identity!==4n)},request=begin(heap,partial);assert.equal(request.$,'arraySliceNeedAuthority');assert.deepEqual(request.constructor,object(4n,true));heap=data(request.heap,6n,speciesKey,object(2n,true));const resumed=advance(directory,stepJavaScriptArraySlice(heap,directory,request.state));assert.equal(resumed.$,'arraySliceConstruct');assert.deepEqual(resumed.constructor,object(4n,true));
});
test('fallback slice rejects oversized Array count before allocating identity',()=>{
 let {heap,directory}=setup();heap=data(heap,8n,'length',number(4294967296));const result=begin(heap,directory,absent,absent,false,object(8n));assert.equal(result.$,'arraySliceError');assert.equal(result.errorClass,'RangeError');assert.deepEqual(result.heap,heap);
});
test('slice refuses forged continuations and unrelated allocation/construction responses',()=>{
 const {heap,directory}=setup(),request=begin(heap,directory);assert.equal(readJavaScriptArraySliceStateError(heap,directory,request.state),'');
 for(const changed of [{...request.state,count:3},{...request.state,outputIndex:1},{...request.state,allocationPrototype:object(6n)},{...request.state,endPresent:false,end:number(9)},{...request.state,conversion:{$:'forged'}}])assert.notEqual(readJavaScriptArraySliceStateError(heap,directory,changed),'');
 assert.equal(resumeJavaScriptArraySliceCall(heap,directory,request.state,normal(number(1))).$,'arraySliceInvalid');assert.equal(resumeJavaScriptArraySliceConstruction(heap,directory,request.state,normal(object(7n))).$,'arraySliceInvalid');assert.equal(resumeJavaScriptArraySliceAllocation(heap,directory,request.state,3n).$,'arraySliceInvalid');
 const customRequest=begin(custom(heap),directory);assert.equal(resumeJavaScriptArraySliceConstruction(customRequest.heap,directory,customRequest.state,normal(number(1))).$,'arraySliceInvalid');
});
