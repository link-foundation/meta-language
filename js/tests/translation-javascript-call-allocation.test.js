// Deferred controls: authoring this file does not execute the machine.
import test from 'node:test';
import assert from 'node:assert/strict';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
import {lowerJavaScriptCallSourceExpression,compileJavaScriptCallSource} from '../src/translation/javascript-call-source.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {lowerCheckedJavaScriptCallModule} from '../src/translation/javascript-call-lowering.js';

function input(prototype={$:'null'}){
 const heap={$:'ordinaryHeap',symbols:[2n],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:true,properties:[]}]};
 const program={$:'callProgram',realms:[3n],exoticKey:{$:'symbol',identity:2n},functions:[{$:'callFunction',identity:1n,realm:3n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:0,registers:3,captures:[],instructions:[{$:'constant',destination:1,value:prototype},{$:'ordinaryAllocate',destination:2,prototype:1},{$:'return',source:2}]}]};
 const entry=beginJavaScriptCallMachine(program,heap,1n,{$:'undefined'},[]);
 assert.equal(entry.$,'machineContinue');
 const loaded=stepJavaScriptCallMachine(program,entry.machine);
 assert.equal(loaded.$,'machineContinue');
 return {program,heap,machine:loaded.machine};
}

for(const prototype of [{$:'null'},{$:'object',identity:1n,callable:true}])test('ordinary allocation retains its exact prototype and advances once '+prototype.$,()=>{
 const {program,heap,machine}=input(prototype);
 const request=stepJavaScriptCallMachine(program,machine);
 assert.equal(request.$,'machineAllocateObject');
 assert.deepEqual(request.prototype,prototype);
 assert.deepEqual(request.machine,machine);
 assert.deepEqual(stepJavaScriptCallMachine(program,request.machine),request);
 const resumed=resumeJavaScriptCallObjectAllocation(program,request.machine,9n);
 assert.equal(resumed.$,'machineContinue');
 assert.equal(resumed.machine.frames[0].position,2);
 assert.deepEqual(resumed.machine.heap.objects[1],{$:'ordinaryObject',identity:9n,prototype,extensible:true,callable:false,properties:[]});
 assert.equal(heap.objects.length,1);
 assert.equal(resumeJavaScriptCallObjectAllocation(program,resumed.machine,10n).$,'machineInvalid');
 const completed=stepJavaScriptCallMachine(program,resumed.machine);
 assert.equal(completed.$,'machineCompleted');
 assert.deepEqual(completed.value,{$:'object',identity:9n,callable:false});
});

for(const identity of [0n,1n,2n,-1n,18446744073709551616n])test('ordinary allocation rejects an invalid or occupied identity '+identity,()=>{
 const {program,heap,machine}=input();
 const result=resumeJavaScriptCallObjectAllocation(program,machine,identity);
 assert.equal(result.$,'machineInvalid');assert.deepEqual(result.heap,heap);
 assert.equal(machine.frames[0].position,1);
});

test('ordinary allocation refuses a primitive prototype without allocating',()=>{
 const {program,heap,machine}=input({$:'number',value:3});
 const result=stepJavaScriptCallMachine(program,machine);
 assert.equal(result.$,'machineInvalid');assert.deepEqual(result.heap,heap);
});

test('source allocation emits one prototype evaluation before its allocation',()=>{
 const result=lowerJavaScriptCallSourceExpression({$:'sourceAllocate',prototype:{$:'sourceConstant',value:{$:'null'}}},[],{$:'sourceBuilder',instructions:[],next:1,visits:0,parameters:0,handlers:0,error:''},0);
 assert.equal(result.builder.error,'');
 assert.deepEqual(result.builder.instructions,[{$:'constant',destination:1,value:{$:'null'}},{$:'ordinaryAllocate',destination:2,prototype:1}]);
 assert.equal(result.register,2);
});

test('source property definition preserves operand order and the actual realm',()=>{
 const constant=value=>({$:'sourceConstant',value});
 const source={$:'sourcePropertyDefinition',target:constant({$:'object',identity:1n,callable:true}),key:constant({$:'text',units:[102,105,101,108,100]}),attributes:constant({$:'object',identity:9n,callable:false}),realm:3n};
 const result=lowerJavaScriptCallSourceExpression(source,[],{$:'sourceBuilder',instructions:[],next:1,visits:0,parameters:0,handlers:0,error:''},0);
 assert.equal(result.builder.error,'');
 assert.deepEqual(result.builder.instructions,[{$:'constant',destination:1,value:source.target.value},{$:'constant',destination:2,value:source.key.value},{$:'constant',destination:3,value:source.attributes.value},{$:'propertyDefinition',destination:4,target:1,key:2,attributes:3,realm:3n}]);
});

for(const realm of [0n,-1n,18446744073709551616n])test('source property operations reject invalid realm coordinates '+realm,()=>{
 const value={$:'sourceConstant',value:{$:'null'}};
 for(const source of [{$:'sourcePropertyKey',input:value,realm},{$:'sourcePropertyDefinition',target:value,key:value,attributes:value,realm}]){
  const result=lowerJavaScriptCallSourceExpression(source,[],{$:'sourceBuilder',instructions:[],next:1,visits:0,parameters:0,handlers:0,error:''},0);
  assert.notEqual(result.builder.error,'');assert.deepEqual(result.builder.instructions,[]);
 }
});

test('source objects define own data properties and retain duplicate-key creation order',()=>{
 const constant=value=>({$:'sourceConstant',value}),text=units=>constant({$:'text',units});
 const source={$:'sourceObject',prototype:constant({$:'null'}),realm:3n,properties:[
  {$:'sourceObjectProperty',key:text([97]),value:constant({$:'number',value:1})},
  {$:'sourceObjectProperty',key:text([98]),value:constant({$:'number',value:2})},
  {$:'sourceObjectProperty',key:text([97]),value:constant({$:'number',value:3})},
 ]};
 const compilation=compileJavaScriptCallSource(source,[]);assert.equal(compilation.error,'');
 const {program,heap}=input();
 program.functions[0]={...program.functions[0],instructions:compilation.instructions,registers:compilation.registers};
 let state=beginJavaScriptCallMachine(program,heap,1n,{$:'undefined'},[]),allocations=0;
 for(let steps=0;state.$==='machineContinue'||state.$==='machineAllocateObject';steps+=1){
  assert.ok(steps<128,'the complete finite object control must finish');
  if(state.$==='machineAllocateObject'){allocations+=1;state=resumeJavaScriptCallObjectAllocation(program,state.machine,9n);}
  else state=stepJavaScriptCallMachine(program,state.machine);
 }
 assert.equal(allocations,1);assert.equal(state.$,'machineCompleted');
 assert.deepEqual(state.value,{$:'object',identity:9n,callable:false});
 const object=state.heap.objects.find(object=>object.identity===9n);
 const original=Object.assign(Object.create(null),{a:1},{b:2},{a:3});
 assert.deepEqual(object.properties.map(property=>String.fromCharCode(...property.key.units)),Reflect.ownKeys(original));
 for(const property of object.properties){
  const name=String.fromCharCode(...property.key.units),expected=Object.getOwnPropertyDescriptor(original,name),actual=property.descriptor;
  assert.equal(actual.value.value,expected.value);assert.equal(actual.value.$,'number');
  for(const flag of ['writable','enumerable','configurable']){assert.equal(actual[flag+'Present'],true);assert.equal(actual[flag],expected[flag]);}
  assert.equal(actual.getterPresent,false);assert.equal(actual.setterPresent,false);
 }
});

test('actual JavaScript frontend records use supplied realm prototypes through the call machine',()=>{
 const source='/** @returns {{value: number}} */ export function make(){return {value:7};}';
 const checked=checkProgram(parseJavaScript(source));
 const registry={functions:[{name:'make',identity:1n,realm:3n}],realms:[3n],exoticKey:{$:'symbol',identity:2n},ordinaryPrototypes:[{realm:3n,prototype:{$:'object',identity:4n,callable:false}}]};
 const lowered=lowerCheckedJavaScriptCallModule(checked,registry);
 const heap={$:'ordinaryHeap',symbols:[2n],objects:[
  {$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:true,properties:[]},
  {$:'ordinaryObject',identity:4n,prototype:{$:'null'},extensible:true,callable:false,properties:[]},
 ]};
 let state=beginJavaScriptCallMachine(lowered.program,heap,1n,{$:'undefined'},[]);
 for(let steps=0;state.$==='machineContinue'||state.$==='machineAllocateObject';steps+=1){
  assert.ok(steps<128);
  state=state.$==='machineAllocateObject'?resumeJavaScriptCallObjectAllocation(lowered.program,state.machine,9n):stepJavaScriptCallMachine(lowered.program,state.machine);
 }
 assert.equal(state.$,'machineCompleted');
 const actual=state.heap.objects.find(object=>object.identity===state.value.identity);
 assert.deepEqual(actual.prototype,registry.ordinaryPrototypes[0].prototype);
 assert.equal(actual.properties[0].descriptor.value.value,7);
 const {ordinaryPrototypes,...withoutPrototype}=registry;
 assert.throws(()=>lowerCheckedJavaScriptCallModule(checked,withoutPrototype),/actual Object prototype/);
 assert.throws(()=>lowerCheckedJavaScriptCallModule(checked,{...registry,ordinaryPrototypes:[...ordinaryPrototypes,...ordinaryPrototypes]}),/one supplied actual Object identity/);
 assert.throws(()=>lowerCheckedJavaScriptCallModule(checked,{...registry,ordinaryPrototypes:[{realm:3n,prototype:{$:'object',identity:4n,callable:true}}]}),/one supplied actual Object identity/);
});

test('bounded execution suspends for allocator supply and consumes only successful identities',()=>{
 const {program,machine}=input();
 const suspended=runJavaScriptCallMachine(program,machine,16,[]);
 assert.equal(suspended.transition.$,'machineAllocateObject');assert.equal(suspended.steps,1);assert.equal(suspended.objectIdentitiesUsed,0);
 const completed=runJavaScriptCallMachine(program,suspended.transition.machine,16,[9n,10n]);
 assert.equal(completed.transition.$,'machineCompleted');assert.equal(completed.objectIdentitiesUsed,1);
 assert.deepEqual(completed.transition.value,{$:'object',identity:9n,callable:false});
 const refused=runJavaScriptCallMachine(program,machine,16,[2n]);
 assert.equal(refused.transition.$,'machineInvalid');assert.equal(refused.objectIdentitiesUsed,0);
 const limited=runJavaScriptCallMachine(program,machine,0,[9n]);
 assert.equal(limited.transition.$,'machineResourceLimit');assert.deepEqual(limited.transition.machine,machine);assert.equal(limited.objectIdentitiesUsed,0);
 for(const limit of [-1,-0,0.5,1048577])assert.equal(runJavaScriptCallMachine(program,machine,limit,[]).transition.$,'machineInvalid');
});
