// Deferred protocol assertions. No host operation is executed during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine,readJavaScriptCallProgramError} from '../src/translation/javascript-call-machine.js';
import {readJavaScriptHostCapabilityRequestError,resumeJavaScriptSourceHostCapability} from '../src/translation/javascript-host-capabilities.js';
import {runJavaScriptSourceTransition} from 'meta-language/node-source-modules';
import * as rules from '../src/translation/javascript-host-capability-rules.js';
import {javascriptHostCapabilityDecisions as decisions} from './fixtures/javascript-host-capabilities.mjs';
for(const[index,row]of decisions.entries())test('host capability receiver decision '+index,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));

function inputs(){
 const heap={$:'ordinaryHeap',objects:[1n,2n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:identity===1n,properties:[]})),symbols:[99n]};
 const program={$:'callProgram',functions:[{$:'callFunction',identity:1n,realm:7n,receiverMode:'qualified host capability',lexicalReceiver:{$:'undefined'},parameters:1,registers:3,captures:[],instructions:[{$:'invokeHostCapability',destination:2},{$:'return',source:2}]}],realms:[7n],exoticKey:{$:'symbol',identity:99n}};
 const begin=beginJavaScriptCallMachine(program,heap,1n,{$:'undefined'},[{$:'number',value:3}]);
 assert.equal(begin.$,'machineContinue');
 return {program,heap,begin,request:stepJavaScriptCallMachine(program,begin.machine)};
}
test('a qualified capability suspends with exact inputs and resumes one real value',()=>{
 const {program,request}=inputs();let effects=0;
 assert.equal(request.$,'machineInvokeHostCapability');assert.equal(readJavaScriptHostCapabilityRequestError(program,request),'');
 const authority={perform(actual){effects++;assert.equal(actual,request);return {heap:request.machine.heap,value:{$:'number',value:4},abrupt:false};}};
 const resumed=resumeJavaScriptSourceHostCapability({program},request,authority);
 assert.equal(effects,1);assert.equal(resumed.$,'machineContinue');
 assert.equal(resumed.machine.frames.at(-1).pending.$,'callIdle');
 assert.deepEqual(stepJavaScriptCallMachine(program,resumed.machine),{$:'machineCompleted',heap:request.machine.heap,value:{$:'number',value:4}});
});
test('substituted request operands cannot invoke external effects',()=>{
 for(const edit of [row=>row.realm=8n,row=>row.callee.identity=2n,row=>row.receiver={$:'null'},row=>row.arguments[0]={$:'number',value:5},row=>row.arguments.push({$:'undefined'})]){
  const {program,request}=inputs(),forged=structuredClone(request);edit(forged);let effects=0;
  assert.throws(()=>resumeJavaScriptSourceHostCapability({program},forged,{perform(){effects++;throw new Error('the provider must not run');}}),TypeError);
  assert.equal(effects,0);
 }
});
test('host resumption retains an actual abrupt value without manufacturing an Error',()=>{
 const {program,request}=inputs(),value={$:'text',units:[97,98,114,117,112,116]};
 const result=resumeJavaScriptSourceHostCapability({program},request,{perform(){return {heap:request.machine.heap,value,abrupt:true};}});
 assert.deepEqual(result,{$:'machineThrow',heap:request.machine.heap,value});
});
test('host effects cannot remove an unrelated prior global identity',()=>{
 const {program,request}=inputs();let effects=0;
 const result=resumeJavaScriptSourceHostCapability({program},request,{perform(){effects++;return {heap:{...request.machine.heap,objects:request.machine.heap.objects.filter(row=>row.identity!==2n)},value:{$:'undefined'},abrupt:false};}});
 assert.equal(effects,1);assert.equal(result.$,'machineInvalid');
 assert.notEqual(result.reason,'');
});
test('ordinary source functions cannot acquire capability authority by inserting an opcode',()=>{
 const {program,heap}=inputs();program.functions[0].receiverMode='strict';
 assert.equal(readJavaScriptCallProgramError(program,heap),'host capability calls require their explicitly qualified external source owner');
});
test('the packaged runner uses the actual capability provider and retains resource boundaries',()=>{
 const {program,heap,begin}=inputs();let currentHeap=heap,effects=0,allocations=0;
 const embedding={module:{program},heap:()=>currentHeap,adoptHeap(value){currentHeap=value;},allocateIdentity(){allocations++;return 30n;},authority:{perform(request){effects++;return {heap:request.machine.heap,value:{$:'number',value:4},abrupt:false};}}};
 const stopped=runJavaScriptSourceTransition(embedding,begin,{steps:0});
 assert.equal(stopped.transition.$,'machineResourceLimit');assert.equal(stopped.steps,0);assert.equal(effects,0);assert.equal(allocations,0);
 const result=runJavaScriptSourceTransition(embedding,begin,{steps:16});
 assert.equal(result.transition.$,'machineCompleted');assert.deepEqual(result.transition.value,{$:'number',value:4});
 assert.equal(effects,1);assert.equal(allocations,0);assert.equal(result.objectIdentitiesUsed,0);assert.equal(currentHeap,result.transition.heap);
 for(const steps of [-0,-1,0.5,NaN,Infinity,1048577])assert.throws(()=>runJavaScriptSourceTransition(embedding,begin,{steps}),RangeError);
});
