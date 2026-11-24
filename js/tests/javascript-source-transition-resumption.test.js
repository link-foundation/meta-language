// Deferred resource-continuation controls; do not execute during source drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {captureJavaScriptOrdinaryClassModule,prepareJavaScriptOrdinaryClassModule,runJavaScriptSourceTransition,resumeJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {javascriptArraySortOriginals} from './fixtures/javascript-array-sort-source.mjs';
import {javascriptArrayCopyOriginals} from './fixtures/javascript-array-copy-source.mjs';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {javascriptArraySortAssertionPolicy} from './fixtures/javascript-array-sort-source.mjs';
for(const row of [javascriptArraySortOriginals[0],javascriptArraySortOriginals[16],javascriptArrayCopyOriginals[0],javascriptArrayCopyOriginals[25]])test('bounded source continuation preserves '+row.name,()=>{
 assert.equal(selectVersionedAssertion(javascriptArraySortAssertionPolicy,process.versions.node).expected,true);
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'resource-source.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:83n});
 let result=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,row.arguments),{steps:1}),slices=1,steps=result.steps;
 assert.equal(result.transition.$,'machineResourceLimit');const zero=resumeJavaScriptSourceTransition(instance.embedding,result.transition,{steps:0});assert.equal(zero.steps,0);assert.deepEqual(zero.transition.machine,result.transition.machine);
 while(result.transition.$==='machineResourceLimit'&&slices<4096){const previous=result.transition.machine;result=resumeJavaScriptSourceTransition(instance.embedding,result.transition,{steps:17});assert.ok(result.steps<=17);steps+=result.steps;slices+=1;assert.ok(previous.frames.length>0);}
 assert.equal(result.transition.$,'machineCompleted',result.transition.reason);assert.ok(slices>1);assert.ok(steps>1);assert.equal(instance.embedding.materializeData(result.transition.value),row.expected);
 assert.throws(()=>resumeJavaScriptSourceTransition(instance.embedding,result.transition),/retained resource machine/);
});
test('source resumption rejects a missing machine and invalid next-slice budgets',()=>{
 for(const transition of [null,{}, {$:'machineResourceLimit'},{$:'machineCompleted',value:{$:'undefined'}}])assert.throws(()=>resumeJavaScriptSourceTransition({},transition),/retained resource machine/);
 for(const steps of [-1,NaN,Infinity,1048577])assert.throws(()=>resumeJavaScriptSourceTransition({},{$:'machineResourceLimit',machine:{}},{steps}),/step budget/);
});
