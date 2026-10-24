// Future controls only. Do not run during the whole requirements draft.
import assert from'node:assert/strict';
import test from'node:test';
import{runInNewContext}from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{parseRust}from'../src/translation/rust.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-inherent.lino',import.meta.url));
const[major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('Rust inherent methods retain actual owners, argument order and four-target codecs',()=>{
 const program=checkProgram(parseRust(fixture.source)),js=emitJavaScript(program),packet=encodeTypedProgramLinks(program);
 for(const target of['JavaScript','Rust','Lean','Rocq']){
  const original=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),original);
 }
 const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.module+'.result').target);
 const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const[index,row]of fixture.cases.entries()){
  if(row.abort!==null)assert.throws(()=>functions[index](),error=>error.message===row.abort,row.name);
  else assert.equal(String(functions[index]()),row.value,row.name);
 }
});
for(const row of fixture.refusals)test('inherent method refusal: '+row.name,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceSignatureModel:row.sourceSignatureModel??''})),error=>error.kind===row.kind&&error.reason===row.reason));
