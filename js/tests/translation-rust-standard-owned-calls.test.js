// Deferred actual standard receivers, full values and portable-definition replay.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeFormalConceptDefinition,proveFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import * as rules from '../src/translation/rust-standard-owned-call-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-standard-owned-calls.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source+'fn main(){}'),program=decodeTypedProgramLinks(packet.network,packet.root);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 const value=runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});assert.equal(String(value),row.value);
});
test('all original standard path text and vector decisions preserve their premises',()=>{
 for(const row of fixture.paths)assert.equal(rules.readRustStandardOwnedCallRole(row.path,row.ordinary,row.shadowed),row.expected);
 for(const row of fixture.texts)assert.equal(rules.acceptRustStandardOwnedTextCall(row.role,row.kind,row.model,row.layers,row.proven,row.references,row.known),row.expected);
 for(const row of fixture.scalars)assert.equal(rules.acceptRustStandardOwnedScalarCall(row.role,row.kind,row.layers,row.proven,row.references,row.known),row.expected);
 for(const row of fixture.arrays)assert.equal(rules.acceptRustStandardOwnedArrayCall(row.role,row.model,row.layers,row.known),row.expected);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
test('a complete portable copy definition reparses from actual emitted Rust source',()=>{
 const original=checkProgram(parseRust(fixture.roundTripSource)),emission=emitRust(original,{arrayConsumption:true}),definition=emission.definitions.find(text=>text.startsWith('pub fn copied('));assert.ok(definition);assert.ok(definition.includes('::std::clone::Clone::clone'));
 const restored=checkProgram(parseRust(definition+'\nfn main(){}')),packets=[original,restored].map(program=>encodeTypedProgramLinks(program));
 const definitions=packets.map(packet=>encodeFormalConceptDefinition(packet));assert.ok(proveFormalConceptEquivalence(definitions[0],definitions[0]));assert.ok(proveFormalConceptEquivalence(definitions[0],definitions[1]));
});
