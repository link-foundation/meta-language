// Deferred compiler/VM controls; imports execute no represented fixture.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptBoxedArrayReadFunction} from '../src/translation/javascript-boxed-array-read-call-source.js';
import {readJavaScriptCallCodeRegisters} from '../src/translation/javascript-call-code.js';
import {beginJavaScriptCallMachine,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),absent={$:'undefined'},text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const operations=['Array.forEach','Array.some','Array.every','Array.find','Array.findIndex','Array.findLast','Array.findLastIndex','Array.reduce','Array.reduceRight','Array.includes','Array.indexOf','Array.lastIndexOf','Array.at'];

test('boxed reads select explicit held receiver without changing protected register zero',()=>{
 for(const operation of operations){const fn=compileJavaScriptBoxedArrayReadFunction(7n,8n,operation,roots);assert.ok(fn,operation);const selected=fn.instructions.find(row=>['arrayCallback','arraySearch'].includes(row.$));assert.equal(selected.input,1);assert.deepEqual(readJavaScriptCallCodeRegisters(selected),[6,1]);assert.equal(fn.instructions.some(row=>row.destination===0),false);assert.equal(fn.instructions[0].operation,'require object coercible');}
 assert.equal(compileJavaScriptBoxedArrayReadFunction(7n,8n,'Array.map',roots),undefined);assert.equal(compileJavaScriptBoxedArrayReadFunction(7n,8n,'Array.at',undefined),undefined);
 assert.deepEqual(readJavaScriptCallCodeRegisters({$:'arraySearch',destination:1,operation:'Array.at'}),[1,0]);
});
function heap(){return {$:'ordinaryHeap',symbols:[99n],objects:[1n,2n,3n,4n,5n,6n,7n,11n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:identity===7n||identity===11n,properties:[]}))};}
function run(operation,input,args=[],callback){const functions=[compileJavaScriptBoxedArrayReadFunction(7n,8n,operation,roots),...(callback?[callback]:[])],program={$:'callProgram',realms:[8n],exoticKey:{$:'symbol',identity:99n},functions};const begun=beginJavaScriptCallMachine(program,heap(),7n,input,args);assert.equal(begun.$,'machineContinue',begun.reason);return runJavaScriptCallMachine(program,begun.machine,4096,[12n]).transition;}
test('String search boxes exact code units before the existing search kernel',()=>{
 const result=run('Array.at',text('\ud800A'),[{$:'number',value:0}]);assert.equal(result.$,'machineCompleted',result.reason);assert.deepEqual(result.value,text('\ud800'));
 assert.deepEqual(result.heap.objects.find(row=>row.identity===12n).stringData,text('\ud800A'));
});
test('String callback receives its boxed source through the existing callback kernel',()=>{
 const callback={$:'callFunction',identity:11n,realm:8n,receiverMode:'strict',lexicalReceiver:absent,parameters:3,registers:5,captures:[],instructions:[{$:'constant',destination:4,value:{$:'boolean',value:true}},{$:'return',source:4}]};
 const result=run('Array.find',text('ab'),[object(11n,true)],callback);assert.equal(result.$,'machineCompleted',result.reason);assert.deepEqual(result.value,text('a'));
});
test('nullish source fails before any boxing identity is consumed',()=>{
 const result=run('Array.at',{$:'null'},[{$:'number',value:0}]);assert.equal(result.$,'machineAllocateError');assert.equal(result.errorClass,'TypeError');assert.equal(result.machine.heap.objects.some(row=>row.identity===12n),false);
});
