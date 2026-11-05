// Deferred authoring controls. Nothing here is run during source drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readSourceLibraryClosure} from '../scripts/read-source-library-closure.mjs';
import {readNativePolicyLiteralControls,composeNativePolicyAssertionProgram,draftNativePolicyAssertionControls,draftNativePolicyAssertionBatch} from '../scripts/draft-native-policy-assertion-controls.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
const sourcePath='js/tests/support/native-policy-authoring-rules.js',peerPath='js/tests/support/native-policy-authoring-peer-rules.js',path='js/tests/fixtures/native-policy-authoring-controls.mjs';
const source=readFileSync(new URL('./fixtures/native-policy-authoring-controls.mjs',import.meta.url),'utf8');
test('literal capture preserves special Number spellings constant provenance and every appended original index',()=>{
 const fixture=readNativePolicyLiteralControls(source,{path,exportName:'nativeLiteralAssertions'});assert.equal(fixture.controls.length,7);assert.deepEqual(fixture.controls.map(row=>row.index),[0,1,2,3,4,5,6]);
 for(const [index,expected]of [[0,'NaN'],[1,'Infinity'],[2,'-Infinity'],[3,'-0']]){assert.equal(fixture.controls[index].args[0].source,expected);assert.equal(fixture.controls[index].expected.source,expected);}
 assert.equal(fixture.controls[4].expected.originalSource,'failure');assert.equal(fixture.controls[4].expected.source,"'literal failure'");assert.equal(fixture.controls[4].expected.constantReferences[0].name,'failure');
 const closure=readSourceLibraryClosure(sourcePath),program=composeNativePolicyAssertionProgram({sourcePath,closure,fixtures:[fixture]});assert.equal(program.controls.length,7);assert.ok(program.source.startsWith(closure.source));assert.ok(program.source.includes('assert.deepEqual(readNativeLiteralNumber(-0),-0);'));assert.ok(program.source.includes('assert.deepEqual(readNativeLiteralNumber(NaN),NaN);'));
});
test('literal capture refuses dynamic unknown cyclic or omitted append shapes',()=>{
 const row="{operation:'readNativeLiteralChoice',arguments:[true],expected:'yes'}",prefix='export const cells=['+row+'];';
 for(const text of [prefix+'cells.push(make());',prefix+'if(true){cells.push('+row+');}',prefix+'if(true)cells.push('+row+');',prefix+'cells.splice(0,1);',prefix+'mutate(cells);',"const value=read();export const cells=[{operation:'readNativeLiteralChoice',arguments:[true],expected:value}];",'const first=second;const second=first;export const cells=[first];',prefix+'cells.push(cells);'])assert.throws(()=>readNativePolicyLiteralControls(text,{path:'literal.mjs',exportName:'cells'}));
 assert.throws(()=>readNativePolicyLiteralControls('export const cells=[{operation:"f",arguments:[undefined],expected:null}];',{path:'literal.mjs',exportName:'cells'}),/unsupported/u);
 assert.throws(()=>readNativePolicyLiteralControls('export const cells=[{operation:"f",arguments:[1n],expected:1n}];',{path:'literal.mjs',exportName:'cells'}),/Number literals/u);
 const fixture=readNativePolicyLiteralControls('export const cells=[{operation:"notAnExport",arguments:[],expected:true}];',{path:'literal.mjs',exportName:'cells'});assert.throws(()=>composeNativePolicyAssertionProgram({sourcePath,closure:readSourceLibraryClosure(sourcePath),fixtures:[fixture]}),/unknown policy operation/u);
 const good=readNativePolicyLiteralControls(source,{path,exportName:'nativeLiteralAssertions'}),closure=readSourceLibraryClosure(sourcePath);good.controls[0].expected.source='readNativeLiteralNumber(7)';assert.throws(()=>composeNativePolicyAssertionProgram({sourcePath,closure,fixtures:[good]}),/complete original literal source/u);
 const accessor={};Object.defineProperty(accessor,'source',{get(){throw new Error('an accessor must not execute');},enumerable:true});assert.throws(()=>composeNativePolicyAssertionProgram({sourcePath,closure,fixtures:[accessor]}),/accessors/u);
 assert.throws(()=>composeNativePolicyAssertionProgram({sourcePath,closure,fixtures:[],assignment:new Map()}),/batch reconciliation/u);
 for(const name of ['NaN','Infinity']){
  const shadow=structuredClone(closure);shadow.inputs[0].declarations.push(name);
  if(shadow.inputs[0].compiledDeclarations)shadow.inputs[0].compiledDeclarations.push(name);
  assert.throws(()=>composeNativePolicyAssertionProgram({sourcePath,closure:shadow,fixtures:[readNativePolicyLiteralControls(source,{path,exportName:'nativeLiteralAssertions'})]}),/literal Number globals/u);
 }
});
test('literal const array aliases and earlier appends introduced by later rows retain actual representation order',()=>{
 const text="const values=[];export const cells=[];values.push('literal failure');cells.push({operation:'readNativeLiteralChoice',arguments:[false],expected:values});";
 const fixture=readNativePolicyLiteralControls(text,{path:'literal.mjs',exportName:'cells'});assert.equal(fixture.controls[0].expected.source,"['literal failure']");
 const punctuation=readNativePolicyLiteralControls('export const cells=[{operation:"f",arguments:["(","{",";"],expected:[")"]}];',{path:'literal.mjs',exportName:'cells'});assert.equal(punctuation.controls.length,1);
});
test('complete assertion program translates once into one same-file native test and exact Links author receipt',()=>{
 const product=draftNativePolicyAssertionControls({sourcePath,fixtureSelections:[{path,exportName:'nativeLiteralAssertions'}],moduleName:'native_policy_authoring_controls'});
 assert.equal(product.receipt.controls.length,7);assert.match(product.rust,/#\[test\]\nfn native_policy_literal_assertions\(\) \{ main\(\); \}/u);assert.match(product.registration,/mod native_policy_authoring_controls;/u);
 const restored=readStructuredDataLino(product.receiptLino);assert.deepEqual(restored,product.receipt);assert.equal(restored.controls[3].expected.source,'-0');assert.equal(restored.controls[0].expected.source,'NaN');assert.equal(restored.assertionProgram.source,product.source);
});
function group(moduleName,root,operations){return {moduleName,sourcePath:root,fixtureSelections:[{path,exportName:'mixedLiteralAssertions',operations}]};}
test('explicit batch reconciles mixed original coordinates once and retains counterpart ownership on exclusions',()=>{
 const product=draftNativePolicyAssertionBatch({groups:[group('native_policy_first',sourcePath,['readNativeLiteralChoice']),group('native_policy_second',peerPath,['readNativeLiteralBoolean'])]});
 assert.equal(product.modules.length,2);assert.equal(product.receipt.assignments.length,2);for(const module of product.modules){assert.equal(module.receipt.controls.length,1);assert.equal(module.receipt.excludedControls.length,1);assert.notEqual(module.receipt.excludedControls[0].assignedTo.sourcePath,module.receipt.sourceRoot);assert.equal(module.receipt.fixtures[0].controls.length,2);}
 assert.deepEqual(readStructuredDataLino(product.receiptLino),product.receipt);
});
test('batch refuses missing duplicate and unknown assignments before translation or publication',()=>{
 assert.throws(()=>draftNativePolicyAssertionBatch({groups:[group('native_policy_first',sourcePath,['readNativeLiteralChoice'])]}),/unassigned original/u);
 assert.throws(()=>draftNativePolicyAssertionBatch({groups:[group('native_policy_first',sourcePath,['readNativeLiteralChoice']),group('native_policy_duplicate',sourcePath,['readNativeLiteralChoice']),group('native_policy_second',peerPath,['readNativeLiteralBoolean'])]}),/duplicate native policy assertion coordinate/u);
 assert.throws(()=>draftNativePolicyAssertionBatch({groups:[group('native_policy_first',sourcePath,['notAnOperation'])]}),/no original fixture coordinates/u);
});
