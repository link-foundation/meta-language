// Deferred original-engine oracles; product execution uses only the portable VM.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import * as rules from '../src/translation/javascript-ordinary-class-rules.js';
import {javascriptOrdinaryClassDecisions as decisions,javascriptOrdinaryClassOriginals as originals,javascriptOrdinaryClassRefusals as refusals} from './fixtures/javascript-ordinary-class-source.mjs';
import {readSourceMachineTrace} from './support/source-machine-trace.js';
import * as primitiveMethods from '../src/translation/javascript-primitive-methods.js';
import * as primitives from '../src/translation/javascript-call-primitives.js';
import {javascriptPrimitiveMethodDecisions,javascriptPrimitiveAdditionDecisions} from './fixtures/javascript-primitive-methods.mjs';
test('ordinary class actual call/new and return rules',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
test('actual primitive method and scalar addition decisions',()=>{for(const row of javascriptPrimitiveMethodDecisions)assert.deepEqual(primitiveMethods[row.operation](...row.arguments),row.expected);for(const row of javascriptPrimitiveAdditionDecisions)assert.deepEqual(primitives[row.operation](...row.arguments),row.expected);});
for(const row of originals)test(row.name,()=>{
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-ordinary-class.js',entries:row.entries});assert.deepEqual(checkJavaScriptOrdinaryClassModule(capture),capture);
 const instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:53n}),input=structuredClone(row.arguments),args=input.map(instance.embedding.encode),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,args),{steps:65536}).transition;
 // Only the oracle invokes original bodies. This source is assembled from
 // actual owned declaration spans, removing no unowned import/export tokens.
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn '+row.entry+';',[])();
 if(row.expectedError){assert.equal(transition.$,'machineThrow',transition.reason);const actual=instance.embedding.decode(transition.value);for(const [key,value]of Object.entries(row.expectedError))assert.equal(actual[key],value);const expectedInput=structuredClone(row.arguments);assert.throws(()=>original(...expectedInput),error=>Object.entries(row.expectedError).every(([key,value])=>error[key]===value));assert.deepEqual(instance.embedding.materializeData(args[0]),row.expectedUnchangedArgument);assert.deepEqual(expectedInput[0],row.expectedUnchangedArgument);}
 else{assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),row.expected);assert.deepEqual(original(...structuredClone(row.arguments)),row.expected);}
 if(row.expectedBatchResumption){const replay=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:61n}),trace=readSourceMachineTrace(replay.embedding,replay.begin(row.entry,row.arguments.map(replay.embedding.encode)),{steps:65536});assert.equal(trace.events.some(event=>event.operation==='resumeBatch'),row.expectedBatchResumption);assert.equal(trace.budgetExhausted,false);assert.equal(trace.transition.$,'machineCompleted');assert.deepEqual(replay.embedding.materializeData(trace.transition.value),row.expected);}
});
for(const row of refusals)test(row.name,()=>assert.throws(()=>captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-ordinary-class-refusal.js',entries:row.entries})));
test('a low total source budget never presents partial class loop state as success',()=>{
 const row=originals.find(row=>row.expectedBatchResumption),capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-loop-budget.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:59n}),trace=readSourceMachineTrace(instance.embedding,instance.begin(row.entry,[]),{steps:64});
 assert.equal(trace.budgetExhausted,true);assert.equal(trace.transition.$,'machineResourceLimit');assert.equal(Object.hasOwn(trace.transition,'value'),false);assert.equal(trace.budget,64);assert.ok(trace.used<=trace.budget);
});
