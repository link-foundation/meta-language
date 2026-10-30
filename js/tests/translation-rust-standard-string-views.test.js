// Deferred source values, borrow boundaries and actual emitted-source reimport.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-standard-string-view-rules.js';
import {readRustContextualCallArguments} from '../src/translation/rust-copy-closures.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-standard-string-views.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
function value(packet,name='result'){
 const emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),target=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target;
 return String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+target+'()',{}, {timeout:1000}));
}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source+'fn main(){}'),program=decodeTypedProgramLinks(packet.network,packet.root);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.equal(value(packet),row.value);
});
test('every shared decision retains its complete declared premises',()=>{
 for(const row of fixture.paths)assert.equal(rules.acceptRustStandardStringViewPath(row.path,row.ordinary,row.shadowed),row.expected);
 for(const row of fixture.layers)assert.equal(rules.acceptRustStandardStringViewLayers(row.layers),row.expected);
 for(const row of fixture.consumers)assert.equal(rules.acceptRustStandardStringViewConsumer(row.parameter,row.layers,row.knownSlice,row.staticLifetime,row.result,row.resultLayers,row.knownResult),row.expected);
 for(const row of fixture.receivers)assert.equal(rules.acceptRustStandardStringViewReceiver(row.kind,row.model,row.layers,row.proven,row.references,row.known,row.argumentsCount),row.expected);
 for(const row of fixture.arguments)assert.equal(rules.acceptRustStandardStringViewArgument(row.sites,row.root,row.kind,row.layers,row.expectedLayers,row.references),row.expected);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
test('consumer frames restore after both nested reads and errors',()=>{
 const sentinel={enabled:true,layers:3,sites:[]},parser={standardStringBorrowContext:sentinel,origins:{origin:()=>({type:null}),immutableProduct:()=>false},cursor:{is:()=>false},expr(){assert.equal(this.standardStringBorrowContext.enabled,false);throw new Error('actual parser failure');}};
 assert.throws(()=>readRustContextualCallArguments(parser,{k:'str'},[]),/actual parser failure/u);
 assert.equal(parser.standardStringBorrowContext,sentinel);
});
test('complete retained textual definitions reparse actual emitted source',()=>{
 const original=createRustSourceOwnerProgramLinks(fixture.roundTripSource),program=decodeTypedProgramLinks(original.network,original.root),emission=emitRust(program,{sourceHeaderModel:fixture.headerModel,arrayConsumption:true});
 const source=emission.definitions.join('\n\n')+'\nfn main(){}';assert.ok(source.includes('::std::borrow::ToOwned::to_owned'));assert.ok(source.includes('::std::string::String::as_str'));
 const restored=createRustSourceOwnerProgramLinks(source);
 for(const packet of [original,restored]){
  assert.equal(value(packet),fixture.roundTripValue);
  for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const signatures=packet=>[...decodeTypedProgramLinks(packet.network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]);
 assert.deepEqual(signatures(restored),signatures(original));
});
