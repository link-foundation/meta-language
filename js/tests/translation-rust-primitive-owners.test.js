import assert from'node:assert/strict';
import test from'node:test';
import vm from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{rustPrimitiveOwnerFixture}from'../scripts/generate-rust-primitive-owner-fixtures.mjs';
import{parseRust}from'../src/translation/rust.js';
import{checkProgram}from'../src/translation/check.js';
import{LinkNetwork}from'../src/network.js';
import{createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-primitive-owners.lino',import.meta.url)),options={primitiveOwnerModel:fixture.model,sourceSignatureModel:fixture.signatureModel};
function bits(value){if(typeof value==='boolean')return String(value);if(Number.isNaN(value))return'NaN';const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,false);return view.getBigUint64(0,false).toString(16).padStart(16,'0');}
function observe(code,entry){const context={};vm.runInNewContext(code,context,{timeout:1000});return entry.observations.map(row=>bits(context.apply(...row.javascriptArguments.map(Number))));}
test('forty-two actual complete qualified/imported functions preserve bits and class through genuine Rust replay',()=>{
 assert.deepEqual(rustPrimitiveOwnerFixture(),fixture);assert.equal(fixture.cases.length,42);
 for(const entry of fixture.cases){const expected=entry.observations.map(row=>row.expectedBits);assert.deepEqual(observe(entry.originalJavaScript,entry),expected,entry.id);assert.deepEqual(observe(entry.javascript,entry),expected,entry.id);assert.deepEqual(observe(entry.replayJavaScript,entry),expected,entry.id);assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source,options))),entry.surface,entry.id);}
});
test('source-free links retain actual interface paths, imports, constant sites and canonical signatures',()=>{
 for(const entry of fixture.cases){
  const network=LinkNetwork.fromLino(entry.links),program=decodeTypedProgramLinks(network,entry.root),proof=program.items[0].sourcePrimitiveOwners;
  assert.equal(proof.model,fixture.model,entry.id);assert.deepEqual(proof.namespace.declaredTypeOwners,[],entry.id);
  assert.deepEqual(observe(emitTypedProgramLinks(network,entry.root,'JavaScript').code,entry),entry.observations.map(row=>row.expectedBits),entry.id);
  assert.equal(emitTypedProgramLinks(network,entry.root,'Rust').code,entry.rust,entry.id);
  assert.ok(network.links().every(link=>!link.metadata().definition?.includes(entry.source)),entry.id);
 }
});
test('all defaults stay outside the new selection and every new source boundary has its exact refusal',()=>{
 for(const entry of fixture.cases)assert.throws(()=>checkProgram(parseRust(entry.source)),undefined,entry.id);
 assert.equal(fixture.invalid.length,20);
 for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,options)),error=>{assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;});
 assert.throws(()=>parseRust('fn main(){}',{primitiveOwnerModel:'unknown'}),/Rust primitive owner model/u);
 assert.throws(()=>createTypedProgramLinks('function f(){return 1;}','JavaScript',{rustPrimitiveOwnerModel:fixture.model}),/requires Rust source/u);
});
test('contradictory actual alias/path/site/namespace and interface edits refuse before either target',()=>{
 const entry=fixture.cases.find(entry=>entry.id==='imported-core-INFINITY');
 for(const edit of [
  proof=>proof.namespace.imports[0].target=['crate','Float'],
  proof=>proof.namespace.declaredTypeOwners=['Float'],
  proof=>proof.namespace.externalPrimitiveRoots=['std'],
  proof=>proof.result.resolvedPath=['std::primitive::f64'],
  proof=>proof.result.sourcePath=['Different'],
  proof=>proof.result.alias='Different',
  proof=>proof.constants[0].path=['args',0],
  proof=>proof.constants[0].name='NEG_INFINITY',
  proof=>proof.constants[0].owner.resolvedPath=['crate','Float'],
  proof=>proof.constants[0].span.start+=1,
  proof=>proof.parameters=[null],
  proof=>proof.result.wrappers=['reference'],
  proof=>proof.model='unknown',
 ]){
  const network=LinkNetwork.fromLino(entry.links),program=decodeTypedProgramLinks(network,entry.root);edit(program.items[0].sourcePrimitiveOwners);assert.throws(()=>encodeTypedProgramLinks(program),/primitive owner|signature/u);
 }
 const altered=LinkNetwork.fromLino(entry.links),primitive=altered.links().find(link=>link.metadata().term==='string value'&&link.metadata().definition==='f64');
 assert.ok(primitive);primitive.setMetadata(primitive.metadata().withDefinition('f32'));
 const reloaded=LinkNetwork.fromLino(altered.toLino());
 for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(reloaded,entry.root,target),/primitive|signature/u);
 const original=fixture.cases.find(entry=>entry.id==='qualified-standard-floor');
 for(const flag of['referenceSemantics','utf16Strings','outputThreaded','abortsThreaded']){const program=decodeTypedProgramLinks(LinkNetwork.fromLino(original.links),original.root);program[flag]=true;assert.throws(()=>encodeTypedProgramLinks(program),/primitive owner/u);}
});
test('an owned scalar meaning edit executes after reload while source owner facts stay intact',()=>{
 const entry=fixture.cases.find(entry=>entry.id==='editable-value-body'),network=LinkNetwork.fromLino(entry.links),program=decodeTypedProgramLinks(network,entry.root),before=structuredClone(program.items[0].sourcePrimitiveOwners);
 program.items[0].body.right.value='2.0';const edited=encodeTypedProgramLinks(program),reloaded=LinkNetwork.fromLino(edited.network.toLino());
 assert.deepEqual(decodeTypedProgramLinks(reloaded,edited.root).items[0].sourcePrimitiveOwners,before);
 assert.equal(observe(emitTypedProgramLinks(reloaded,edited.root,'JavaScript').code,entry)[0],'4014000000000000');
});
