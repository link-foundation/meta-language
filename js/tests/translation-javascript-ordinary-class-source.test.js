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
 if(row.expectedError){assert.equal(transition.$,'machineThrow',transition.reason);const actual=instance.embedding.decode(transition.value);for(const [key,value]of Object.entries(row.expectedError))assert.equal(actual[key],value);const expectedInput=structuredClone(row.arguments);let originalError;assert.throws(()=>original(...expectedInput),error=>{originalError=error;return Object.entries(row.expectedError).every(([key,value])=>error[key]===value);});assert.equal(actual.name,originalError.name);assert.equal(actual.message,originalError.message);if(Object.hasOwn(row,'expectedUnchangedArgument')){assert.deepEqual(instance.embedding.materializeData(args[0]),row.expectedUnchangedArgument);assert.deepEqual(expectedInput[0],row.expectedUnchangedArgument);}if(Object.hasOwn(row,'expectedArgumentState')){assert.deepEqual(args.map(value=>instance.embedding.materializeData(value)),row.expectedArgumentState);assert.deepEqual(expectedInput,row.expectedArgumentState);}}
 else{assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),row.expected);assert.deepEqual(original(...structuredClone(row.arguments)),row.expected);}
 if(row.expectedBatchResumption){const replay=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:61n}),trace=readSourceMachineTrace(replay.embedding,replay.begin(row.entry,row.arguments.map(replay.embedding.encode)),{steps:65536});assert.equal(trace.events.some(event=>event.operation==='resumeBatch'),row.expectedBatchResumption);assert.equal(trace.budgetExhausted,false);assert.equal(trace.transition.$,'machineCompleted');assert.deepEqual(replay.embedding.materializeData(trace.transition.value),row.expected);}
});
for(const row of refusals)test(row.name,()=>assert.throws(()=>captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-ordinary-class-refusal.js',entries:row.entries})));
test('a low total source budget never presents partial class loop state as success',()=>{
 const row=originals.find(row=>row.expectedBatchResumption),capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-loop-budget.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:59n}),trace=readSourceMachineTrace(instance.embedding,instance.begin(row.entry,[]),{steps:64});
 assert.equal(trace.budgetExhausted,true);assert.equal(trace.transition.$,'machineResourceLimit');assert.equal(Object.hasOwn(trace.transition,'value'),false);assert.equal(trace.budget,64);assert.ok(trace.used<=trace.budget);
});
test('ordinary source array input and materialization retain holes, aliases and actual descriptors',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('export function read(value) { return value; }',{identifier:'literal-array-identity.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:71n});
 const child={value:3},input=[];input[2]=child;input[3]=child;input.length=6;Object.defineProperty(input,'length',{writable:false});Object.preventExtensions(input);
 const encoded=instance.embedding.encode(input),row=instance.embedding.heap().objects.find(row=>row.identity===encoded.identity);assert.equal(row.$,'arrayObject');
 const actual=instance.embedding.materializeData(encoded);assert.equal(Array.isArray(actual),true);assert.equal(actual.length,6);assert.equal(Object.hasOwn(actual,0),false);assert.equal(Object.hasOwn(actual,5),false);assert.equal(actual[2],actual[3]);assert.equal(Object.getOwnPropertyDescriptor(actual,'length').writable,false);assert.equal(Object.isExtensible(actual),false);assert.deepEqual(actual,input);
 const prototype=instance.embedding.encode(Array.prototype);assert.equal(instance.embedding.heap().objects.find(row=>row.identity===prototype.identity).$,'arrayObject');
});
test('ordinary source input capture refuses Proxy reflection before any trap',()=>{
 const capture=captureJavaScriptOrdinaryClassModule('export function read(value) { return value; }',{identifier:'literal-proxy-input.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:73n});let traps=0;
 const value=new Proxy([],{ownKeys(){traps++;return [];},getPrototypeOf(){traps++;return Array.prototype;},isExtensible(){traps++;return true;}});assert.throws(()=>instance.embedding.encode(value),/Proxy/);assert.equal(traps,0);
});
test('computed assignment checks nullish receiver after key expression and RHS but before conversion',()=>{
 const source='class Key { constructor(state) { this.state = state; } toString() { this.state.converted = 1; return "value"; } } function key(state) { state.key = 1; return new Key(state); } function rhs(state) { state.rhs = 1; return 7; } export function read(state) { return null[key(state)] = rhs(state); }',capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-nullish-computed.js',entries:['read']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:79n}),state={key:0,converted:0,rhs:0},argument=instance.embedding.encode(state);
 // Prior draft expected rhs:0 on this SAME input. ECMA 2026 13.3.3 and
 // 6.2.5.6 require RHS evaluation before PutValue/ToObject; retain that former
 // assertion here as correction history, not a passing behavior oracle.
 const previousDraftExpectation={key:1,converted:0,rhs:0};
 const transition=runJavaScriptSourceTransition(instance.embedding,instance.begin('read',[argument]),{steps:65536}).transition;assert.equal(transition.$,'machineThrow');assert.equal(instance.embedding.decode(transition.value).name,'TypeError');assert.deepEqual(instance.embedding.materializeData(argument),{key:1,converted:0,rhs:1});assert.notDeepEqual(instance.embedding.materializeData(argument),previousDraftExpectation);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+';return read;',[])(),expected={key:0,converted:0,rhs:0};assert.throws(()=>original(expected),TypeError);assert.deepEqual(expected,{key:1,converted:0,rhs:1});
});


test('object binding coercibility rejects only nullish semantic values without inspecting properties',()=>{
 for(const value of [{$:'undefined'},{$:'null'}]){const actual=primitives.executeJavaScriptCallPrimitive('require object coercible',[value]);assert.equal(actual.$,'primitiveError');assert.equal(actual.errorClass,'TypeError');}
 for(const value of [{$:'boolean',value:false},{$:'number',value:0},{$:'integer',value:0n},{$:'text',units:[]},{$:'symbol',identity:1n},{$:'object',identity:1n,callable:false}])assert.deepEqual(primitives.executeJavaScriptCallPrimitive('require object coercible',[value]),{$:'primitiveValue',value:{$:'undefined'}});
 assert.notEqual(primitives.readJavaScriptCallPrimitiveArityError('require object coercible',0),'');assert.notEqual(primitives.readJavaScriptCallPrimitiveArityError('require object coercible',2),'');
});
