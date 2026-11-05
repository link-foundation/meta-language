// Deferred controls. These tests are authored, never executed while drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {allocateJavaScriptCallClosure,readJavaScriptCallTemplateDirectoryError,readJavaScriptExecutableCallFunction} from '../src/translation/javascript-call-templates.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine,readJavaScriptCallProgramError} from '../src/translation/javascript-call-machine.js';
import {planJavaScriptOrdinaryGet,readJavaScriptOrdinaryOwnKeys,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from '../src/translation/javascript-ordinary-heap.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),number=value=>({$:'number',value}),text=value=>({$:'text',units:Array.from(value,ch=>ch.charCodeAt(0))});
function setup(){
 const heap={$:'ordinaryHeap',symbols:[99n],objects:[1n,2n,3n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:identity!==3n,properties:[]}))};
 const template={$:'callTemplate',owner:1n,ordinal:0,realm:1n,parameters:1,registers:4,captureCount:1,instructions:[{$:'numberBinary',destination:3,left:1,right:2,operator:'add'},{$:'return',source:3}],prototype:object(2n,true),name:text('add'),length:1};
 const fn={$:'callFunction',identity:1n,realm:1n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:1,registers:3,captures:[],instructions:[{$:'closureAllocate',destination:2,owner:1n,ordinal:0,captures:[1],receiver:0},{$:'return',source:2}]};
 return {heap,template,program:{$:'callProgram',functions:[fn],templates:[template],realms:[1n],exoticKey:{$:'symbol',identity:99n}}};
}
function execute(program,heap,identity,receiver,args,identities=[]){const begin=beginJavaScriptCallMachine(program,heap,identity,receiver,args);assert.equal(begin.$,'machineContinue',begin.reason);return runJavaScriptCallMachine(program,begin.machine,256,identities).transition;}
test('each actual evaluation creates a fresh callable while static code stays shared',()=>{
 const {heap,program}=setup();assert.equal(readJavaScriptCallProgramError(program,heap),'');
 const first=execute(program,heap,1n,object(3n),[number(7)],[4n]);assert.equal(first.$,'machineCompleted');assert.deepEqual(first.value,object(4n,true));
 const second=execute(program,first.heap,1n,object(3n),[number(11)],[5n]);assert.equal(second.$,'machineCompleted');assert.notDeepEqual(first.value,second.value);assert.equal(program.functions.length,1);assert.equal(program.templates.length,1);
 assert.deepEqual(execute(program,second.heap,4n,{$:'null'},[number(5)]).value,number(12));assert.deepEqual(execute(program,second.heap,5n,{$:'undefined'},[number(5)]).value,number(16));
 assert.deepEqual(readJavaScriptExecutableCallFunction(program,second.heap,4n).captures,[number(7)]);
 const original=(()=>{const add=value=>input=>input+value;return [add(7),add(11)];})();assert.notEqual(original[0],original[1]);assert.deepEqual(original.map(fn=>fn(5)),[12,16]);
});
test('closure own name/length descriptors and arrow prototype absence remain ordinary data',()=>{
 const {heap,program}=setup(),allocated=allocateJavaScriptCallClosure(program,heap,4n,1n,0,[number(7)],object(3n));assert.equal(allocated.$,'closureAllocationNormal');const closure=allocated.heap.objects.find(row=>row.identity===4n);
 assert.deepEqual(readJavaScriptOrdinaryOwnKeys(allocated.heap,4n).keys,[text('length'),text('name')]);assert.deepEqual(closure.prototype,object(2n,true));
 for(const property of closure.properties){assert.equal(property.descriptor.writable,false);assert.equal(property.descriptor.enumerable,false);assert.equal(property.descriptor.configurable,true);}
 assert.deepEqual(planJavaScriptOrdinaryGet(allocated.heap,4n,text('name'),object(4n,true)).value,text('add'));assert.deepEqual(planJavaScriptOrdinaryGet(allocated.heap,4n,text('length'),object(4n,true)).value,number(1));
 const changed=defineJavaScriptOrdinaryProperty(allocated.heap,4n,text('custom'),createJavaScriptOrdinarySetDescriptor(number(9),true));assert.equal(changed.success,true);assert.deepEqual(readJavaScriptExecutableCallFunction(program,changed.heap,4n).captures,[number(7)]);
});
test('lexical receiver ignores the receiver supplied at an arrow call',()=>{
 const {heap,program,template}=setup();program.templates=[{...template,parameters:0,registers:1,captureCount:0,instructions:[{$:'return',source:0}],length:0}];program.functions=[{...program.functions[0],instructions:[{$:'closureAllocate',destination:2,owner:1n,ordinal:0,captures:[],receiver:0},{$:'return',source:2}]}];
 const allocated=allocateJavaScriptCallClosure(program,heap,4n,1n,0,[],object(3n));assert.equal(allocated.$,'closureAllocationNormal');assert.deepEqual(execute(program,allocated.heap,4n,{$:'null'},[]).value,object(3n));
});
test('invalid owners, capture vectors and duplicate identities cannot manufacture callables',()=>{
 const {heap,program,template}=setup();for(const [identity,owner,ordinal,captures]of [[4n,1n,0,[]],[4n,3n,0,[number(7)]],[4n,1n,1,[number(7)]],[1n,1n,0,[number(7)]],[99n,1n,0,[number(7)]]]){const result=allocateJavaScriptCallClosure(program,heap,identity,owner,ordinal,captures,{$:'undefined'});assert.equal(result.$,'closureAllocationInvalid');assert.deepEqual(result.heap,heap);}
 assert.notEqual(readJavaScriptCallTemplateDirectoryError({...program,templates:[template,template]},heap),'');
 assert.notEqual(readJavaScriptCallTemplateDirectoryError({...program,templates:[{...template,instructions:[{$:'constant',destination:2,value:number(9)},{$:'return',source:2}]}]},heap),'');
});
