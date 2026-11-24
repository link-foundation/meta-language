// Authored future source checks only. No represented policy/fixture is imported
// or evaluated by the exact reader, including all negative side-effect cases.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readExactFixtureExport,renderExactFixtureData,readNativePolicyLiteralControls} from '../scripts/draft-native-policy-assertion-controls.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {exactFixtureAuthoringOriginals as originals,exactFixtureAuthoringRefusals as refusals} from './fixtures/exact-fixture-authoring.mjs';
const profile='exact pure fixture data 2';
for(const row of originals)test(row.name,()=>{
 const capture=readExactFixtureExport(row.source,{path:'literal.mjs',exportName:'data',profile});
 assert.equal(capture.source,row.source);assert.equal(capture.renderedSource,row.expected);assert.equal(capture.renderedSourceProfile,'structural descriptor display 1');
 if(row.aliases)assert.throws(()=>renderExactFixtureData(capture.descriptor),/shared input aliases/u);else assert.equal(renderExactFixtureData(capture.descriptor),row.expected);
 const restored=readStructuredDataLino(structuredDataToLino(capture));assert.deepEqual(restored,capture);
 if(row.aliases)for(const value of [capture,restored]){assert.equal(value.descriptor.items[0],value.descriptor.items[1]);assert.notEqual(value.descriptor.items[0],value.descriptor.items[2]);}
 for(const reference of capture.constantReferences)assert.equal(row.source.slice(reference.start,reference.end),reference.source);
 for(const derivation of capture.derivations)assert.ok(derivation.start>=0&&derivation.start<derivation.end&&derivation.end<=row.source.length);
});
for(const row of refusals)test(row.name,()=>assert.throws(()=>readExactFixtureExport(row.source,{path:'literal.mjs',exportName:'data',profile})));
test('the default literal reader retains exact BigInt/undefined refusals',()=>{
 for(const value of ['1n','undefined'])assert.throws(()=>readNativePolicyLiteralControls('export const data=[{operation:"f",arguments:['+value+'],expected:null}];',{path:'literal.mjs',exportName:'data'}));
 assert.throws(()=>readExactFixtureExport('export const data=[];',{path:'literal.mjs',exportName:'data',profile:''}));
 assert.throws(()=>readNativePolicyLiteralControls('export const data=[];',{path:'literal.mjs',exportName:'data',profile:'unknown'}));
});
test('native assertion source refuses identity-sensitive repeated aggregate arguments',()=>{
 const source='const item={value:1};export const data=[{operation:"f",arguments:[item,item],expected:true}];';
 const capture=readExactFixtureExport(source,{path:'literal.mjs',exportName:'data',profile});assert.equal(capture.descriptor.items[0].entries.find(entry=>entry.key==='arguments').value.items[0],capture.descriptor.items[0].entries.find(entry=>entry.key==='arguments').value.items[1]);
 assert.throws(()=>readNativePolicyLiteralControls(source,{path:'literal.mjs',exportName:'data',profile}),/shared input aliases/u);
});
test('all Error source fixture exports retain whole exact descriptor data',()=>{
 const path='js/tests/fixtures/javascript-error-classes.mjs',source=readFileSync(new URL('./fixtures/javascript-error-classes.mjs',import.meta.url),'utf8');
 for(const exportName of ['javascriptErrorClassVersions','translationErrorSource','javascriptErrorClassDecisions','javascriptErrorClassAllocationDecisions','javascriptErrorClassOriginals','javascriptErrorClassRefusals','javascriptErrorClassRequiredHostBoundaries']){
  const capture=readExactFixtureExport(source,{path,exportName,profile});assert.equal(capture.source,source);assert.deepEqual(readStructuredDataLino(structuredDataToLino(capture)),capture);
  if(exportName==='javascriptErrorClassOriginals'){assert.equal(capture.descriptor.items.length,5);assert.ok(capture.renderedSource.includes('undefined'));}
 }
 for(const [exportName,count]of [['javascriptErrorClassDecisions',8],['javascriptErrorClassAllocationDecisions',5]]){
  const capture=readNativePolicyLiteralControls(source,{path,exportName,profile});assert.equal(capture.profile,profile);assert.equal(capture.controls.length,count);assert.deepEqual(capture.controls.map(row=>row.index),Array.from({length:count},(_,index)=>index));assert.deepEqual(readStructuredDataLino(structuredDataToLino(capture)),capture);
 }
});
