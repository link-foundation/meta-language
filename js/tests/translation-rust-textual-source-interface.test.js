// Deferred original interfaces, exact Unicode values and complete source owners.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {emitRust} from '../src/translation/emit-rust.js';
import * as rules from '../src/translation/rust-textual-source-interface-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-textual-source-interface.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const options={rustSourceHeaderEmissionModel:fixture.model};
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),before=packet.network.toLino(),expected=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 assert.ok(expected.encodings.some(row=>row.id==='retained-textual-source-interface'));
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),expected);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 for(const model of ['retained scalar Rust function interface 1','retained mathematical Rust function interface 1'])assert.throws(()=>emitRust(program,{sourceHeaderModel:model}),/actual scalar value type/u);
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 const value=runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});assert.equal(String(value),row.expected);assert.equal(packet.network.toLino(),before);
});
test('every actual textual owner selector binary domain and byte-count premise is independent',()=>{
 for(const row of fixture.selections)assert.equal(rules.acceptRustTextualSourceInterfaceModel(row.model),row.expected);
 for(const row of fixture.owners)assert.equal(rules.readRustTextualSourceType(row.kind,row.path,row.children),row.expected);
 for(const row of fixture.binaries)assert.equal(rules.acceptRustTextualSourceBinary(row.domain,row.operation),row.expected);
 for(const row of fixture.measures)assert.equal(rules.acceptRustTextualSourceOperation(row.operation,row.count,row.result),row.expected);
});
test('each complete string argument slot exists before any corresponding string borrow',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('ordered mixed')),packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),emission=emitRust(program,{sourceHeaderModel:fixture.model});
 const result=emission.definitions.find(text=>text.startsWith('pub fn result(')),bindings=[...result.matchAll(/let (ml_source_argument\w*) = /gu)];assert.equal(bindings.length,2);assert.notEqual(bindings[0][1],bindings[1][1]);
 const borrow='::std::string::String::as_str(&('+bindings[0][1]+'))';assert.equal(result.split(borrow).length-1,1);assert.ok(result.indexOf(borrow)>bindings[1].index);
 const count=emission.definitions.find(text=>text.startsWith('pub fn count('));assert.equal(count.split('::std::borrow::ToOwned::to_owned').length-1,2);
});
test('byte-count widths signedness and arity cannot change across the source-free boundary',()=>{
 const packet=createRustSourceOwnerProgramLinks(fixture.cases[0].source);
 for(const row of fixture.edits){const program=decodeTypedProgramLinks(packet.network,packet.root),node=program.declarations.get('count').body;assert.equal(node.k,'math');let target=node;for(const field of row.path.slice(0,-1))target=target[field];target[row.path.at(-1)]=structuredClone(row.value);assert.throws(()=>encodeTypedProgramLinks(program));}
 const program=decodeTypedProgramLinks(packet.network,packet.root);program.utf16Strings=true;assert.throws(()=>encodeTypedProgramLinks(program));
 const staticProgram=decodeTypedProgramLinks(packet.network,packet.root);staticProgram.declarations.get('count').sourceSignature.parameters[0].lifetimes=['static'];const linked=encodeTypedProgramLinks(staticProgram);for(const network of views(linked))assert.throws(()=>emitTypedProgramLinks(network,linked.root,'Rust',options),/nonstatic copied borrows/u);
});
