// Deferred output-adapter controls; no represented execution on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {observeWrapperMaterialization} from './support/wrapper-materialization.js';
import {javascriptWrapperMaterializationVersions as versions,javascriptWrapperMaterializationOriginals as originals,javascriptWrapperMaterializationRefusals as refusals} from './fixtures/javascript-wrapper-materialization.mjs';
function requireVersion(){const [major,minor]=process.versions.node.split('.').map(Number);assert.equal(major===24||major===22&&minor>=15,true,'uncovered wrapper materialization Node range: '+versions.node);}
function prepare(source){
 const capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-wrapper-materialization.js',entries:['read']});assert.deepEqual(checkJavaScriptOrdinaryClassModule(capture),capture);const instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:109n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin('read',[]),{steps:65536}).transition;assert.equal(transition.$,'machineCompleted',transition.reason);return {capture,instance,transition};
}
for(const row of originals)test(row.name,()=>{
 requireVersion();const {capture,instance,transition}=prepare(row.source),actual=instance.embedding.materializeData(transition.value),observed=observeWrapperMaterialization(actual);assert.deepEqual(observed,row.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn read;',[])(),expected=observeWrapperMaterialization(original(...row.arguments));assert.deepEqual(expected,row.expected);assert.deepEqual(observed,expected);
});
for(const row of refusals)test(row.name,()=>{
 requireVersion();const {instance,transition}=prepare(row.source),before=instance.embedding.heap();assert.throws(()=>instance.embedding.materializeData(transition.value),error=>error instanceof TypeError&&error.message.includes(row.reason));assert.strictEqual(instance.embedding.heap(),before);
 if(row.name.includes('accessors')){const object=before.objects.find(value=>value.identity===transition.value.identity),reads=object.properties.find(property=>property.key.$==='text'&&property.key.units.join(',')==='114,101,97,100,115');assert.deepEqual(reads.descriptor.value,{$:'number',value:0});}
});
test('String carrier materialization refuses missing or changed immutable index snapshots',()=>{
 requireVersion();for(const change of ['remove','change']){const {instance,transition}=prepare('export function read() { return Object("ab"); }'),heap=instance.embedding.heap(),objects=heap.objects.map(object=>object.identity!==transition.value.identity?object:{...object,properties:change==='remove'?object.properties.filter(property=>!(property.key.$==='text'&&property.key.units.length===1&&property.key.units[0]===48)):object.properties.map(property=>property.key.$==='text'&&property.key.units.length===1&&property.key.units[0]===48?{...property,descriptor:{...property.descriptor,enumerable:false}}:property)});instance.embedding.adoptHeap({...heap,objects});assert.throws(()=>instance.embedding.materializeData(transition.value),/every exact immutable index and length descriptor/);}
});
test('materialized wrapper cycles and aliases use one carrier per graph identity',()=>{
 requireVersion();const {instance,transition}=prepare('export function read() { const value = Object(7); value.self = value; return { first: value, second: value }; }'),first=instance.embedding.materializeData(transition.value),second=instance.embedding.materializeData(transition.value);assert.strictEqual(first.first,first.second);assert.strictEqual(first.first.self,first.first);assert.notStrictEqual(first,second);assert.notStrictEqual(first.first,second.first);assert.strictEqual(second.first,second.second);assert.strictEqual(second.first.self,second.first);assert.deepEqual(observeWrapperMaterialization(first),observeWrapperMaterialization(second));
});
test('wrapper materialization retains the prior plain record and sparse Array data behavior',()=>{
 requireVersion();const {instance,transition}=prepare('export function read() { const values = [3]; values.length = 3; return { first: values, second: values }; }'),value=instance.embedding.materializeData(transition.value);const expected=[3,,,];assert.deepEqual(value,{first:expected,second:expected});assert.strictEqual(value.first,value.second);assert.equal(value.first.length,3);assert.equal(Object.hasOwn(value.first,1),false);assert.equal(Object.hasOwn(value.first,2),false);
});

test('String carrier reification reports its property budget before native carrier creation',()=>{
 requireVersion();const {instance,transition}=prepare('export function read() { return Object("x"); }'),heap=instance.embedding.heap(),stringData={$:'sourceText',profile:'inspectable UTF16 source text 1',text:'x'.repeat(4096)};instance.embedding.adoptHeap({...heap,objects:heap.objects.map(object=>object.identity===transition.value.identity?{...object,stringData}:object)});assert.throws(()=>instance.embedding.materializeData(transition.value),/explicit own-property profile/);
});
