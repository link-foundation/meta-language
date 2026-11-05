// Deferred kernel controls. Execution is forbidden until the complete draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {beginJavaScriptArrayConcatenation,stepJavaScriptArrayConcatenation,readJavaScriptArrayConcatenationStateError,resumeJavaScriptArrayConcatenationCall,resumeJavaScriptArrayConcatenationAllocation,resumeJavaScriptArrayConcatenationConstruction} from '../src/translation/javascript-array-concatenation.js';
import {allocateJavaScriptArray} from '../src/translation/javascript-arrays.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,deleteJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor,locateJavaScriptOrdinaryProperty,planJavaScriptOrdinaryGet} from '../src/translation/javascript-ordinary-heap.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {javascriptArrayConcatenationOriginals,javascriptArrayConcatenationAssertionPolicy} from './fixtures/javascript-array-concatenation-source.mjs';
const absent={$:'undefined'},object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))}),speciesKey={$:'symbol',identity:99n},exoticKey={$:'symbol',identity:100n},spreadableKey={$:'symbol',identity:101n};
const normal=value=>({$:'normalCompletion',value});
function property(heap,identity,key,descriptor){const next=defineJavaScriptOrdinaryProperty(heap,identity,typeof key==='string'?text(key):key,descriptor);assert.equal(next.success,true,next.error);return next.heap;}
function data(heap,identity,key,value){const arrayLength=key==='length'&&heap.objects.some(row=>row.identity===identity&&row.$==='arrayObject');return property(heap,identity,key,createJavaScriptOrdinarySetDescriptor(value,!arrayLength));}
function accessor(heap,identity,key,kind,callee){const descriptor=createJavaScriptPropertyDescriptor('accessor',true,true);descriptor[kind]=callee;return property(heap,identity,key,descriptor);}
function setup(){
 let heap={$:'ordinaryHeap',symbols:[99n,100n,101n],objects:[]};for(const identity of [1n,2n,4n,5n,6n,7n,8n,9n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;
 heap={...heap,objects:heap.objects.map(row=>[2n,4n,5n,9n].includes(row.identity)?{...row,callable:true}:row)};
 const array=allocateJavaScriptArray(heap,3n,object(1n),3);assert.equal(array.$,'arrayNormal');heap=data(array.heap,3n,'0',number(1));heap=data(heap,3n,'2',number(3));heap=data(heap,3n,'constructor',object(2n,true));heap=data(heap,2n,speciesKey,object(2n,true));
 const directory={$:'arraySpeciesDirectory',realm:1n,arrayConstructor:object(2n,true),arrayPrototype:object(1n),speciesKey,constructors:[{$:'arrayConstructorFact',identity:2n,constructable:true,realm:1n,intrinsicArray:true},{$:'arrayConstructorFact',identity:4n,constructable:true,realm:1n,intrinsicArray:false},{$:'arrayConstructorFact',identity:5n,constructable:false,realm:1n,intrinsicArray:false},{$:'arrayConstructorFact',identity:9n,constructable:false,realm:1n,intrinsicArray:false}]};return {heap,directory};
}
function advance(directory,transition){let current=transition;for(let index=0;index<256&&current.$==='arrayConcatenationContinue';index++)current=stepJavaScriptArrayConcatenation(current.heap,directory,current.state);assert.notEqual(current.$,'arrayConcatenationContinue');return current;}
function begin(heap,directory,arguments_=[],receiver=object(3n)){return advance(directory,beginJavaScriptArrayConcatenation(heap,directory,receiver,arguments_,spreadableKey,exoticKey));}
function allocate(directory,request){assert.equal(request.$,'arrayConcatenationAllocate');assert.equal(request.length,0);return advance(directory,resumeJavaScriptArrayConcatenationAllocation(request.heap,directory,request.state,20n));}
function own(heap,identity,key){return locateJavaScriptOrdinaryProperty(heap,identity,text(key));}
function get(heap,identity,key){return planJavaScriptOrdinaryGet(heap,identity,text(key),object(identity)).value;}
function custom(heap){heap=data(heap,3n,'constructor',object(6n));return data(heap,6n,speciesKey,object(4n,true));}

test('concatenation preserves holes and appends primitive arguments as scalar items',()=>{
 const {heap,directory}=setup(),request=begin(heap,directory,[number(4),text('ab'),absent,{$:'null'}]),result=allocate(directory,request);
 assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(get(result.heap,20n,'0'),number(1));assert.equal(own(result.heap,20n,'1').found,false);assert.deepEqual(get(result.heap,20n,'2'),number(3));assert.deepEqual(get(result.heap,20n,'3'),number(4));assert.deepEqual(get(result.heap,20n,'4'),text('ab'));assert.equal(own(result.heap,20n,'5').found,true);assert.deepEqual(get(result.heap,20n,'6'),{$:'null'});assert.deepEqual(get(result.heap,20n,'length'),number(7));
});
test('species construction precedes receiver spreadability and receives zero length',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,3n,spreadableKey,'getter',object(5n,true));const request=begin(heap,directory);assert.equal(request.$,'arrayConcatenationConstruct');assert.deepEqual(request.arguments,[number(0)]);
 const getter=advance(directory,resumeJavaScriptArrayConcatenationConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(getter.$,'arrayConcatenationCall');assert.equal(getter.state.phase,'spreadability getter');assert.deepEqual(getter.receiver,object(3n));assert.deepEqual(getter.arguments,[]);
});
test('generic receiver skips constructor and unselected length reads and retains original identity as a scalar',()=>{
 let {heap,directory}=setup();heap=accessor(heap,8n,'constructor','getter',object(5n,true));heap=accessor(heap,8n,'length','getter',object(9n,true));const result=allocate(directory,begin(heap,directory,[],object(8n)));assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(get(result.heap,20n,'0'),object(8n));assert.deepEqual(get(result.heap,20n,'length'),number(1));
});
test('false spreadability overrides actual Array brand without reading length',()=>{
 let {heap,directory}=setup();heap=data(heap,3n,spreadableKey,{$:'boolean',value:false});const result=allocate(directory,begin(heap,directory));assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(get(result.heap,20n,'0'),object(3n));assert.deepEqual(get(result.heap,20n,'length'),number(1));
});
test('truthy Object spreadability uses ToBoolean without conversion and copies inherited indexed values',()=>{
 let {heap,directory}=setup();heap=data(heap,8n,spreadableKey,object(6n));heap=data(heap,6n,'valueOf',object(5n,true));heap=data(heap,8n,'length',number(2));heap={...heap,objects:heap.objects.map(row=>row.identity===8n?{...row,prototype:object(7n)}:row)};heap=data(heap,7n,'1',number(8));const result=allocate(directory,begin(heap,directory,[object(8n)]));assert.equal(result.$,'arrayConcatenationCompleted');assert.equal(own(result.heap,20n,'3').found,false);assert.deepEqual(get(result.heap,20n,'4'),number(8));assert.deepEqual(get(result.heap,20n,'length'),number(5));
});
test('length conversion retains its saved selected item after actual getter callbacks',()=>{
 let {heap,directory}=setup();heap=data(heap,8n,spreadableKey,{$:'boolean',value:true});heap=data(heap,8n,'length',object(6n));heap=data(heap,6n,'valueOf',object(5n,true));heap=data(heap,8n,'0',number(7));heap=data(heap,8n,'1',number(8));const conversion=allocate(directory,begin(heap,directory,[object(8n)]));assert.equal(conversion.$,'arrayConcatenationCall');assert.equal(conversion.state.phase,'conversion');assert.deepEqual(conversion.receiver,object(6n));
 heap=data(conversion.heap,8n,'length',number(0));const result=advance(directory,resumeJavaScriptArrayConcatenationCall(heap,directory,conversion.state,normal(number(2.9))));assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(get(result.heap,20n,'3'),number(7));assert.deepEqual(get(result.heap,20n,'4'),number(8));assert.deepEqual(get(result.heap,20n,'length'),number(5));
});
test('custom result uses own data definitions and invokes the actual strict final length setter',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,7n,'0','setter',object(5n,true));heap=accessor(heap,7n,'length','setter',object(9n,true));const request=begin(heap,directory,[number(4)]),setter=advance(directory,resumeJavaScriptArrayConcatenationConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(setter.$,'arrayConcatenationCall');assert.deepEqual(setter.callee,object(9n,true));assert.deepEqual(setter.receiver,object(7n));assert.deepEqual(setter.arguments,[number(4)]);assert.deepEqual(get(setter.heap,7n,'0'),number(1));assert.equal(own(setter.heap,7n,'1').found,false);assert.deepEqual(get(setter.heap,7n,'3'),number(4));
 const result=advance(directory,resumeJavaScriptArrayConcatenationCall(setter.heap,directory,setter.state,normal(number(99))));assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(result.value,object(7n));
});
test('strict final length failure preserves every completed indexed definition',()=>{
 let {heap,directory}=setup();heap=custom(heap);const descriptor=createJavaScriptOrdinarySetDescriptor(number(0),true);descriptor.writable=false;heap=property(heap,7n,'length',descriptor);const request=begin(heap,directory),result=advance(directory,resumeJavaScriptArrayConcatenationConstruction(request.heap,directory,request.state,normal(object(7n))));assert.equal(result.$,'arrayConcatenationError');assert.equal(result.errorClass,'TypeError');assert.deepEqual(get(result.heap,7n,'0'),number(1));assert.deepEqual(get(result.heap,7n,'2'),number(3));assert.deepEqual(get(result.heap,7n,'length'),number(0));
});
test('later getter mutations affect following Has/Get while prior saved length and outputs survive',()=>{
 let {heap,directory}=setup();heap=accessor(heap,3n,'0','getter',object(5n,true));const getter=allocate(directory,begin(heap,directory));assert.equal(getter.$,'arrayConcatenationCall');assert.equal(getter.state.phase,'element getter');heap=deleteJavaScriptOrdinaryProperty(getter.heap,3n,text('2')).heap;heap=data(heap,1n,'1',number(8));heap=data(heap,3n,'length',number(1));const result=advance(directory,resumeJavaScriptArrayConcatenationCall(heap,directory,getter.state,normal(number(7))));assert.equal(result.$,'arrayConcatenationCompleted');assert.deepEqual(get(result.heap,20n,'0'),number(7));assert.deepEqual(get(result.heap,20n,'1'),number(8));assert.equal(own(result.heap,20n,'2').found,false);assert.deepEqual(get(result.heap,20n,'length'),number(3));
});
test('spreadability length conversion and indexed getter throws retain actual identities and heap effects',()=>{
 for(const kind of ['spreadability','length','conversion','element']){
  let {heap,directory}=setup();
  if(kind==='spreadability')heap=accessor(heap,3n,spreadableKey,'getter',object(5n,true));
  else if(kind==='length'){heap=data(heap,8n,spreadableKey,{$:'boolean',value:true});heap=accessor(heap,8n,'length','getter',object(5n,true));}
  else if(kind==='conversion'){heap=data(heap,8n,spreadableKey,{$:'boolean',value:true});heap=data(heap,8n,'length',object(6n));heap=data(heap,6n,'valueOf',object(5n,true));}
  else heap=accessor(heap,3n,'2','getter',object(5n,true));
  const getter=allocate(directory,begin(heap,directory,[object(8n)]));assert.equal(getter.$,'arrayConcatenationCall');const changed=data(getter.heap,8n,'effect',number(7)),result=resumeJavaScriptArrayConcatenationCall(changed,directory,getter.state,{$:'throwCompletion',value:object(6n)});assert.equal(result.$,'arrayConcatenationThrow');assert.deepEqual(result.value,object(6n));assert.deepEqual(result.heap,changed);
 }
});
test('completed indexed getter values survive output storage suspension without repeating Get',()=>{
 let {heap,directory}=setup();heap=custom(heap);heap=accessor(heap,3n,'0','getter',object(5n,true));for(let index=0;index<4096;index++)heap=data(heap,7n,'slot'+index,number(index));const request=begin(heap,directory),getter=advance(directory,resumeJavaScriptArrayConcatenationConstruction(request.heap,directory,request.state,normal(object(7n)))),limited=advance(directory,resumeJavaScriptArrayConcatenationCall(getter.heap,directory,getter.state,normal(number(13))));assert.equal(limited.$,'arrayConcatenationResourceLimit');assert.equal(limited.state.phase,'write');assert.deepEqual(limited.state.value,number(13));heap=deleteJavaScriptOrdinaryProperty(limited.heap,7n,text('slot0')).heap;const next=stepJavaScriptArrayConcatenation(heap,directory,limited.state);assert.equal(next.$,'arrayConcatenationContinue');assert.deepEqual(get(next.heap,7n,'0'),number(13));assert.equal(next.state.index,1);
});
test('species authority extension retains the already selected constructor',()=>{
 let {heap,directory}=setup();heap=custom(heap);const partial={...directory,constructors:directory.constructors.filter(fact=>fact.identity!==4n)},request=begin(heap,partial);assert.equal(request.$,'arrayConcatenationNeedAuthority');heap=data(request.heap,6n,speciesKey,object(2n,true));const resumed=advance(directory,stepJavaScriptArrayConcatenation(heap,directory,request.state));assert.equal(resumed.$,'arrayConcatenationConstruct');assert.deepEqual(resumed.constructor,object(4n,true));assert.deepEqual(resumed.arguments,[number(0)]);
});
test('safe output bound rejects full spread length before selected indexed Gets',()=>{
 let {heap,directory}=setup();heap=accessor(heap,3n,'0','getter',object(5n,true));const request=begin(heap,directory),allocated=resumeJavaScriptArrayConcatenationAllocation(request.heap,directory,request.state,20n);assert.equal(allocated.$,'arrayConcatenationContinue');
 // Seed an otherwise valid stored boundary coordinate instead of iterating an
 // unbounded prefix; the whole pending spread is checked before index zero.
 const state={...allocated.state,outputIndex:9007199254740990,itemStart:9007199254740990};assert.equal(readJavaScriptArrayConcatenationStateError(allocated.heap,directory,state),'');const result=advance(directory,stepJavaScriptArrayConcatenation(allocated.heap,directory,state));assert.equal(result.$,'arrayConcatenationError');assert.equal(result.errorClass,'TypeError');assert.equal(own(result.heap,20n,'0').found,false);
});
test('Array final length uses Array length rules independently of safe-coordinate limits',()=>{
 const {heap,directory}=setup(),request=begin(heap,directory),allocated=resumeJavaScriptArrayConcatenationAllocation(request.heap,directory,request.state,20n),state={...allocated.state,itemPosition:1,outputIndex:4294967296,itemStart:4294967296};const result=advance(directory,stepJavaScriptArrayConcatenation(allocated.heap,directory,state));assert.equal(result.$,'arrayConcatenationError');assert.equal(result.errorClass,'RangeError');
});
test('concatenation refuses mismatched nested calls and unrelated allocation or construction responses',()=>{
 const {heap,directory}=setup(),request=begin(heap,directory);assert.equal(readJavaScriptArrayConcatenationStateError(heap,directory,request.state),'');for(const changed of [{...request.state,itemPosition:1},{...request.state,outputIndex:1},{...request.state,allocationPrototype:object(6n)},{...request.state,spreadableKey:absent},{...request.state,conversion:{$:'forged'}}])assert.notEqual(readJavaScriptArrayConcatenationStateError(heap,directory,changed),'');
 assert.equal(resumeJavaScriptArrayConcatenationCall(heap,directory,request.state,normal(number(1))).$,'arrayConcatenationInvalid');assert.equal(resumeJavaScriptArrayConcatenationConstruction(heap,directory,request.state,normal(object(7n))).$,'arrayConcatenationInvalid');assert.equal(resumeJavaScriptArrayConcatenationAllocation(heap,directory,request.state,3n).$,'arrayConcatenationInvalid');const selected=begin(custom(heap),directory);assert.equal(resumeJavaScriptArrayConcatenationConstruction(selected.heap,directory,selected.state,normal(number(1))).$,'arrayConcatenationInvalid');
});
test('independent concatenation originals retain expected true and explicit Node version scope',()=>{
 assert.ok(javascriptArrayConcatenationOriginals.length>=20);assert.equal(new Set(javascriptArrayConcatenationOriginals.map(row=>row.name)).size,javascriptArrayConcatenationOriginals.length);for(const row of javascriptArrayConcatenationOriginals){assert.equal(row.expected,true);assert.deepEqual(row.entries,['read']);assert.equal(row.entry,'read');assert.deepEqual(row.arguments,[]);assert.equal(typeof row.source,'string');}
 assert.deepEqual(javascriptArrayConcatenationAssertionPolicy.expectations.map(row=>[row.minimum,row.maximum]),[['22.15.0','23.0.0'],['24.0.0','25.0.0']]);
});
