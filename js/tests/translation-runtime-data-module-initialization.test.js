// All runtime execution/code generation controls are deferred until the draft
// phase ends. Declarative source plans are never passed off as checked Core.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import * as rules from '../src/translation/javascript-module-initialization-rules.js';
import * as machine from '../src/translation/javascript-module-initialization.js';
import {readShippedRuntimeDataAdapterContract,prepareJavaScriptRuntimeDataModule,createRuntimeDataModuleScope} from '../src/translation/runtime-data-module.js';
import {captureJavaScriptRuntimeDataModule,checkJavaScriptRuntimeDataModule} from '../src/translation/runtime-data-module-source.js';
import {captureJavaScriptRuntimeDataFunctions} from '../src/translation/javascript-error-classes.js';
import {createActualErrorSourceDeclarations} from './support/error-source-module-controls.js';
import {readRuntimeDataModelDiagnostic,readRuntimeDataOriginalDiagnostic,readRuntimeDataRefusalDiagnostic} from './support/runtime-data-module-controls.js';
import {createControlDiagnostics,finishControlDiagnostics,requireControlProduct as product} from './support/control-diagnostics.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {runtimeDataModuleDecisions as decisions,runtimeDataModuleModelCases as models,runtimeDataModuleOriginals as originals,runtimeDataModuleRefusals as refusals,runtimeDataModuleSourceAnchors as anchors} from './fixtures/runtime-data-module-initialization.mjs';
const finish=record=>finishControlDiagnostics({name:record.definition.name,...record});
test('actual source-bound runtime module policy decisions',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);});
for(const row of models)test(row.name,()=>finish(readRuntimeDataModelDiagnostic(row)));
for(const row of originals)test(row.name,async()=>finish(await readRuntimeDataOriginalDiagnostic(row)));
for(const row of refusals)test(row.name,()=>finish(readRuntimeDataRefusalDiagnostic(row)));
test('runtime policy source anchors remain whole actual source constructs',()=>{for(const row of anchors)assert.ok(readFileSync(new URL('../../'+row.path,import.meta.url),'utf8').includes(row.source),row.path);});
test('initializer metadata cannot forge providers, binding owners or current source',()=>{
 const readerContract=readShippedRuntimeDataAdapterContract(),source="import { readRuntimeProjectData } from '../project-data-loader.js'; export const POLICY=readRuntimeProjectData(new URL('./policy.lino',import.meta.url)); export function read(){return POLICY.mode;}",capture=captureJavaScriptRuntimeDataModule(source,{identifier:'js/src/translation/source-fixture.js',readerContract});
 for(const edit of [value=>{value.plan.initializers[0].readerOwner='forged';},value=>{value.plan.bindings[0]+=1;},value=>{value.readerContract.loader.source+='\n// changed';},value=>{value.sourceSha256='wrong';}]){const changed=structuredClone(capture);edit(changed);assert.throws(()=>checkJavaScriptRuntimeDataModule(changed));}
 let effects=0;const bad={};Object.defineProperty(bad,'source',{get(){effects++;throw new Error('metadata must not execute');},enumerable:true});assert.throws(()=>checkJavaScriptRuntimeDataModule(bad),/accessor/u);assert.equal(effects,0);
 const functions=captureJavaScriptRuntimeDataFunctions(capture),authority=createActualErrorSourceDeclarations(functions);
 assert.throws(()=>prepareJavaScriptRuntimeDataModule(capture,authority,{moduleURL:new URL('../src/translation/source-fixture.js',import.meta.url).href,scope:createRuntimeDataModuleScope()}),/explicit realm identity/u);
});
test('an invalid uninitialized-binding authority never caches a fabricated source error',()=>{
 const source="import { readRuntimeProjectData } from '../project-data-loader.js'; const FIRST=LATER; export const LATER=readRuntimeProjectData(new URL('./policy.lino',import.meta.url)); export function read(){return FIRST;}",capture=captureJavaScriptRuntimeDataModule(source,{identifier:'js/src/translation/source-fixture.js',readerContract:readShippedRuntimeDataAdapterContract()}),functions=captureJavaScriptRuntimeDataFunctions(capture),authority={...createActualErrorSourceDeclarations(functions),identity:9n};authority.realm.raiseUninitializedBinding=()=>undefined;
 const instance=prepareJavaScriptRuntimeDataModule(capture,authority,{moduleURL:new URL('../src/translation/source-fixture.js',import.meta.url).href,scope:createRuntimeDataModuleScope()});assert.throws(()=>instance.initialize(),/genuine ReferenceError/u);assert.equal(instance.state().phase,'temporal dead zone');assert.deepEqual(instance.state().failure,{$:'undefined'});assert.equal(instance.requests().length,0);
});
test('shared slot coordinates reject noncanonical zero without changing Number wire fields',()=>{
 const plan=structuredClone(models[0].plan);plan.bindings[0]=-0;assert.notEqual(machine.readJavaScriptModuleInitializationPlanError(plan),'');
 const good=models[0].plan,state=machine.createJavaScriptModuleInitialization(good,'file:///source/policy.js',1n);state.position=-0;assert.notEqual(machine.readJavaScriptModuleInitializationStateError(good,state),'');
});
test('only automatically translated initializer kernels retain four-target/codecs obligations',()=>{
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 for(const name of ['javascript-module-initialization-rules-program','javascript-module-initialization-program']){
  const packet=attempt(name+'/checked source kernel',()=>createSourceDefinitionProgramLinks('meta-language/source-program/'+name));
  const variants=[attempt(name+'/original',()=>product(packet,name+'/checked source kernel').network),attempt(name+'/text',()=>LinkNetwork.fromLino(product(packet,name+'/checked source kernel').network.toLino())),attempt(name+'/binary',()=>decodeLinkNetworkBinary(encodeLinkNetworkBinary(product(packet,name+'/checked source kernel').network)))];
  for(let index=0;index<variants.length;index++)for(const target of ['JavaScript','Rust','Lean','Rocq'])attempt(name+'/'+index+'/'+target,()=>{const value=product(packet,name+'/checked source kernel');assert.deepEqual(emitTypedProgramLinks(product(variants[index],name+'/'+index),value.root,target),emitTypedProgramLinks(value.network,value.root,target));});
 }
 finishControlDiagnostics({name:'actual initializer kernels',diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
});
