// Authored continuation controls; execution is deferred until the full draft.
import test from 'node:test';import assert from 'node:assert/strict';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
import {beginJavaScriptObjectSpread,stepJavaScriptObjectSpread,resumeJavaScriptObjectSpread} from '../src/translation/javascript-object-spread.js';
import {insertJavaScriptOrdinaryObject,defineJavaScriptOrdinaryProperty,deleteJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor,planJavaScriptOrdinaryGet,readJavaScriptOrdinaryOwnKeys} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
function setup(){let heap={$:'ordinaryHeap',objects:[],symbols:[99n]};for(const identity of [1n,2n,3n,4n])heap=insertJavaScriptOrdinaryObject(heap,identity,{$:'null'}).heap;return {...heap,objects:heap.objects.map(row=>row.identity===4n?{...row,callable:true}:row)};}
function data(heap,identity,key,value,enumerable=true){return defineJavaScriptOrdinaryProperty(heap,identity,key,{...createJavaScriptOrdinarySetDescriptor(value,true),enumerable}).heap;}
function getter(heap,identity,key,callee){return defineJavaScriptOrdinaryProperty(heap,identity,text(key),{configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:{$:'undefined'},writablePresent:false,writable:false,getterPresent:true,getter:callee,setterPresent:true,setter:{$:'undefined'}}).heap;}
function advance(transition){let current=transition;for(let step=0;step<64&&current.$==='objectSpreadContinue';step++)current=stepJavaScriptObjectSpread(current.heap,current.state);assert.notEqual(current.$,'objectSpreadContinue');return current;}
const get=(heap,key)=>planJavaScriptOrdinaryGet(heap,2n,key,object(2n)).value;
test('spread selects own enumerable keys in numeric text then Symbol order',()=>{
 let heap=setup();for(const [key,value,enumerable]of [[text('b'),number(1),true],[text('2'),number(2),true],[text('1'),number(3),true],[text('hidden'),number(4),false],[{$:'symbol',identity:99n},number(5),true]])heap=data(heap,1n,key,value,enumerable);
 const actual=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n)));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(actual.value,object(2n));assert.deepEqual(readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys,[text('1'),text('2'),text('b'),{$:'symbol',identity:99n}]);assert.deepEqual(get(actual.heap,text('hidden')),{$:'undefined'});
 for(const key of readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys){const target=actual.heap.objects.find(row=>row.identity===2n),descriptor=target.properties.find(row=>assertKey(row.key,key))?.descriptor;assert.equal(descriptor.valuePresent,true);assert.equal(descriptor.writable,true);assert.equal(descriptor.enumerable,true);assert.equal(descriptor.configurable,true);}
});
function assertKey(left,right){return left.$===right.$&&(left.$==='symbol'?left.identity===right.identity:left.units.join(',')===right.units.join(','));}
test('spread snapshots keys but rereads descriptors after a getter mutates the source',()=>{
 let heap=getter(setup(),1n,'first',object(4n,true));heap=data(heap,1n,text('removed'),number(3));heap=data(heap,1n,text('hidden'),number(4));
 const first=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n)));assert.equal(first.$,'objectSpreadCall');assert.deepEqual(first.receiver,object(1n));assert.deepEqual(first.callee,object(4n,true));
 heap=deleteJavaScriptOrdinaryProperty(first.heap,1n,text('removed')).heap;heap=data(heap,1n,text('hidden'),number(8),false);heap=data(heap,1n,text('new'),number(9));
 const actual=advance(resumeJavaScriptObjectSpread(heap,first.state,{$:'normalCompletion',value:number(7)}));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys,[text('first')]);assert.deepEqual(get(actual.heap,text('first')),number(7));
});
test('spread creates own data fields without invoking an inherited setter',()=>{
 let heap=setup();heap={...heap,objects:heap.objects.map(row=>row.identity===2n?{...row,prototype:object(3n)}:row)};heap=defineJavaScriptOrdinaryProperty(heap,3n,text('value'),{configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:{$:'undefined'},writablePresent:false,writable:false,getterPresent:true,getter:{$:'undefined'},setterPresent:true,setter:object(4n,true)}).heap;heap=data(heap,1n,text('value'),number(7));
 const actual=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n)));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(get(actual.heap,text('value')),number(7));
});
test('abrupt getters retain prior copied properties and current shared effects',()=>{
 let heap=data(setup(),1n,text('first'),number(3));heap=getter(heap,1n,'abrupt',object(4n,true));heap=data(heap,1n,text('later'),number(9));const first=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n)));assert.equal(first.$,'objectSpreadCall');heap=data(first.heap,3n,text('effect'),number(7));
 const actual=resumeJavaScriptObjectSpread(heap,first.state,{$:'throwCompletion',value:text('original')});assert.equal(actual.$,'objectSpreadThrow');assert.deepEqual(actual.heap,heap);assert.deepEqual(actual.value,text('original'));assert.deepEqual(get(actual.heap,text('first')),number(3));assert.deepEqual(get(actual.heap,text('later')),{$:'undefined'});
});
test('spread retains completed getter values across storage resource suspension',()=>{
 let heap=getter(setup(),1n,'value',object(4n,true));for(let index=0;index<4096;index++)heap=data(heap,2n,text('slot'+index),number(index));
 const first=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n)));assert.equal(first.$,'objectSpreadCall');const limited=advance(resumeJavaScriptObjectSpread(first.heap,first.state,{$:'normalCompletion',value:number(7)}));assert.equal(limited.$,'objectSpreadResourceLimit');assert.equal(limited.state.phase,'write');assert.deepEqual(limited.state.value,number(7));
 heap=deleteJavaScriptOrdinaryProperty(limited.heap,2n,text('slot0')).heap;const actual=advance(stepJavaScriptObjectSpread(heap,limited.state));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(get(actual.heap,text('value')),number(7));
});
test('String spread copies UTF16 units while nullish and other primitives have no keys',()=>{
 const heap=setup(),actual=advance(beginJavaScriptObjectSpread(heap,object(2n),{$:'text',units:[0xd83d,0xde00,0xd800]}));assert.equal(actual.$,'objectSpreadCompleted');for(const [key,unit]of [['0',0xd83d],['1',0xde00],['2',0xd800]])assert.deepEqual(get(actual.heap,text(key)),{$:'text',units:[unit]});
 for(const input of [{$:'null'},{$:'undefined'},{$:'boolean',value:false},number(7)]){const empty=advance(beginJavaScriptObjectSpread(heap,object(2n),input));assert.equal(empty.$,'objectSpreadCompleted');assert.deepEqual(empty.heap,heap);}
 const limited=advance(beginJavaScriptObjectSpread(heap,object(2n),{$:'text',units:Array(4097).fill(65)}));assert.equal(limited.$,'objectSpreadResourceLimit');assert.equal(limited.state.phase,'keys');assert.deepEqual(limited.heap,heap);
});

test('the shared machine executes an actual spread getter and returns its literal target',()=>{
 let heap=getter(setup(),1n,'value',object(4n,true));heap=insertJavaScriptOrdinaryObject(heap,5n,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===5n?{...row,callable:true}:row)};
 const program={$:'callProgram',realms:[1000n],exoticKey:{$:'symbol',identity:99n},functions:[
  {$:'callFunction',identity:4n,realm:1000n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'constant',destination:1,value:number(7)},{$:'return',source:1}]},
  {$:'callFunction',identity:5n,realm:1000n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:2,registers:4,captures:[],instructions:[{$:'objectSpread',destination:3,target:1,input:2,realm:1000n},{$:'return',source:3}]},
 ]};
 const started=beginJavaScriptCallMachine(program,heap,5n,{$:'undefined'},[object(2n),object(1n)]);assert.equal(started.$,'machineContinue');const actual=runJavaScriptCallMachine(program,started.machine,256,[]).transition;assert.equal(actual.$,'machineCompleted');assert.deepEqual(actual.value,object(2n));assert.deepEqual(get(actual.heap,text('value')),number(7));
});

test('excluded own properties are skipped before descriptor Get or getter execution',()=>{
 let heap=getter(setup(),1n,'drop',object(4n,true));heap=data(heap,1n,text('keep'),number(7));const actual=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n),[text('drop')]));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys,[text('keep')]);assert.deepEqual(get(actual.heap,text('keep')),number(7));
});
test('excluded Symbol identities and duplicate keys preserve exact key semantics',()=>{
 let heap=data(setup(),1n,{$:'symbol',identity:99n},number(7));heap=data(heap,1n,text('Symbol'),number(9));heap=data(heap,1n,text('drop'),number(11));const excluded=[{$:'symbol',identity:99n},text('drop'),text('drop')],actual=advance(beginJavaScriptObjectSpread(heap,object(2n),object(1n),excluded));assert.equal(actual.$,'objectSpreadCompleted');assert.deepEqual(readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys,[text('Symbol')]);assert.deepEqual(get(actual.heap,text('Symbol')),number(9));
 const malformed=beginJavaScriptObjectSpread(heap,object(2n),object(1n),[object(1n)]);assert.equal(malformed.$,'objectSpreadInvalid');
});
test('the VM retains actual excluded-key registers and never calls the excluded getter',()=>{
 let heap=getter(setup(),1n,'drop',object(4n,true));heap=data(heap,1n,text('keep'),number(9));heap=insertJavaScriptOrdinaryObject(heap,5n,{$:'null'}).heap;heap={...heap,objects:heap.objects.map(row=>row.identity===5n?{...row,callable:true}:row)};
 const program={$:'callProgram',realms:[1000n],exoticKey:{$:'symbol',identity:99n},functions:[
  {$:'callFunction',identity:4n,realm:1000n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'constant',destination:1,value:text('excluded getter must not run')},{$:'throw',source:1}]},
  {$:'callFunction',identity:5n,realm:1000n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:3,registers:5,captures:[],instructions:[{$:'objectSpread',destination:4,target:1,input:2,excluded:[3],realm:1000n},{$:'return',source:4}]},
 ]};const started=beginJavaScriptCallMachine(program,heap,5n,{$:'undefined'},[object(2n),object(1n),text('drop')]);assert.equal(started.$,'machineContinue');const actual=runJavaScriptCallMachine(program,started.machine,256,[]).transition;assert.equal(actual.$,'machineCompleted');assert.deepEqual(readJavaScriptOrdinaryOwnKeys(actual.heap,2n).keys,[text('keep')]);assert.deepEqual(get(actual.heap,text('keep')),number(9));
});
