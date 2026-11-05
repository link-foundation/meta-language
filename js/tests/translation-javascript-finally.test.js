// Deferred actual common-machine finalizer controls. No behavioral import work.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine,unwindJavaScriptCallCompletion} from '../src/translation/javascript-call-machine.js';
import {insertJavaScriptOrdinaryObject} from '../src/translation/javascript-ordinary-heap.js';
const number=value=>({$:'number',value}),constant=value=>({$:'sourceConstant',value}),absent={$:'undefined'};
function setup(source){const code=compileJavaScriptMutableCallSourceWithCaptures(source,[],[]);assert.equal(code.error,'');const inserted=insertJavaScriptOrdinaryObject({$:'ordinaryHeap',objects:[],symbols:[99n]},1n,{$:'null'}),heap={...inserted.heap,objects:inserted.heap.objects.map(row=>({...row,callable:true}))},program={$:'callProgram',realms:[1n],exoticKey:{$:'symbol',identity:99n},functions:[{$:'callFunction',identity:1n,realm:1n,receiverMode:'strict',lexicalReceiver:absent,parameters:0,registers:code.registers,captures:[],instructions:code.instructions}]};return {code,heap,program,started:beginJavaScriptCallMachine(program,heap,1n,absent,[])};}
test('common source finalizer return replaces a deferred return through actual instructions',()=>{
 const {code,program,started}=setup({$:'sourceFinally',body:{$:'sourceReturn',value:constant(number(7))},finalizer:{$:'sourceReturn',value:constant(number(9))}});assert.equal(code.instructions.some(row=>row.$==='pushFinally'),true);assert.equal(code.instructions.some(row=>row.$==='finishFinally'),true);const actual=runJavaScriptCallMachine(program,started.machine,128,[]).transition;assert.equal(actual.$,'machineCompleted');assert.deepEqual(actual.value,number(9));
});
test('a finalizer-local catch retains the original deferred completion',()=>{
 const source={$:'sourceFinally',body:{$:'sourceReturn',value:constant(number(7))},finalizer:{$:'sourceTry',identity:1,body:{$:'sourceThrow',value:constant(number(3))},handler:constant(absent)}},{program,started}=setup(source),actual=runJavaScriptCallMachine(program,started.machine,128,[]).transition;assert.equal(actual.$,'machineCompleted');assert.deepEqual(actual.value,number(7));
});
test('malformed completion coordinates refuse before source unwinding',()=>{
 const {started}=setup(constant(number(7)));for(const completion of [{$:'callCompletion',kind:'jump',value:absent,target:0,depth:-1},{$:'callCompletion',kind:'return',value:absent,target:0,depth:0},{$:'callCompletion',kind:'jump',value:absent,target:NaN,depth:0}])assert.equal(unwindJavaScriptCallCompletion(started.machine,completion).$,'machineInvalid');
});
