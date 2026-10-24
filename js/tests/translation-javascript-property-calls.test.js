// Future full controls. Never run during the whole requirements draft.
import assert from'node:assert/strict';
import test from'node:test';
import{runInNewContext}from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{parseJavaScript}from'../src/translation/javascript.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{createJavaScriptStringRangeProgramLinks}from'../src/javascript-string-range-program.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-property-calls.lino',import.meta.url));
const[major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const observe=call=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'abort',value:error.message};}};
test('actual property owners retain original calls, codecs and all four emissions',()=>{
 const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),js=emitJavaScript(program);
 for(const target of['JavaScript','Rust','Lean','Rocq']){
  const original=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),original);
 }
 const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target);
 const functions=runInNewContext('"use strict";\n'+js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const[index,row]of fixture.cases.entries()){
  const expected={kind:row.abort===null?'value':'abort',value:row.abort??row.value};
  assert.deepEqual(observe(()=>runInNewContext('"use strict";\n'+row.source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000})),expected,row.name);
  assert.deepEqual(observe(()=>functions[index]()),expected,row.name);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));

test('complete range rule graphs retain all codecs and target emissions',()=>{
 const packet=createJavaScriptStringRangeProgramLinks();
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
