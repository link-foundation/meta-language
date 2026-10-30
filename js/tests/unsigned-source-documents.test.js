// Deferred controls. Source authors may parse this file, never import it.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {LinkNetwork} from '../src/network.js';
import {ParserRegistry} from '../src/parser-registry.js';
import {ParseConfiguration} from '../src/primitives.js';
import {UnsignedLinkNetwork,rebaseUnsignedLinkNetwork,createUnsignedSourceDocumentLinks,createUnsignedSourceDocumentLinksWithRegistry,createDenseUnsignedLinkNetwork,encodeUnsignedLinkNetworkBinary,decodeUnsignedLinkNetworkBinary} from '../src/unsigned-link-network.js';
import {createUnsignedTypedProgramLinks,createUnsignedRustSourceOwnerProgramLinks,rebaseUnsignedTypedProgramLinks,emitUnsignedTypedProgramLinks} from '../src/unsigned-typed-program-links.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {runInNewContext} from 'node:vm';
import * as rules from '../src/unsigned-source-document-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-source-documents.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.allocations)test(row.name,()=>{assert.equal(rules.readUnsignedSourceAllocationError(row.first,row.count,row.limit),'');assert.deepEqual(rules.allocateUnsignedSourceIdentities(row.first,row.count,row.limit),row.expected);});
for(const row of fixture.refusals)test(row.name,()=>{assert.equal(rules.readUnsignedSourceAllocationError(row.first,row.count,row.limit),row.error);assert.deepEqual(rules.allocateUnsignedSourceIdentities(row.first,row.count,row.limit),[]);});
function assertTransport(source,packet){
 const view=createDenseUnsignedLinkNetwork(packet.network),sourceIds=packet.sourceIdentities.map(String),newIds=packet.identities.map(String),mapping=new Map(sourceIds.map((id,index)=>[id,newIds[index]]));
 assert.equal(new Set(newIds).size,newIds.length);assert.equal(view.directory.length,newIds.length);
 for(const link of source.links()){
  const restored=packet.network.link(mapping.get(String(link.id())));assert.ok(restored);assert.deepEqual(restored.references().map(String),link.references().map(id=>mapping.get(String(id))));assert.deepEqual(restored.metadata(),link.metadata());
 }
 for(const [term,id]of source._terms)assert.equal(String(packet.network.findTerm(term)),mapping.get(String(id)));
 assert.deepEqual([...packet.network._conceptSyntax],[...source._conceptSyntax]);
 for(const projection of ['ConcreteSyntax','AbstractSyntax','Semantic','Lossless'])assert.deepEqual(packet.network.projectedLinks(projection).map(link=>String(link.id())),source.projectedLinks(projection).map(link=>mapping.get(String(link.id()))));
 assert.deepEqual(packet.network.projectedLinks(),packet.network.projectedLinks('ConcreteSyntax'));assert.throws(()=>packet.network.projectedLinks('unknown'));
}
for(const row of fixture.graphs)test(row.name,()=>{
 const source=UnsignedLinkNetwork.fromLino(row.source),before=encodeUnsignedLinkNetworkBinary(source),packet=rebaseUnsignedLinkNetwork(source,row.first);
 assert.deepEqual(packet.sourceIdentities.map(String),row.sourceIdentities);assert.deepEqual(packet.identities.map(String),row.identities);assert.equal(packet.network.nextIdentity?.toString()??null,row.next);assertTransport(source,packet);assert.deepEqual(encodeUnsignedLinkNetworkBinary(source),before);
 assert.equal(decodeUnsignedLinkNetworkBinary(encodeUnsignedLinkNetworkBinary(packet.network)).toLino(),packet.network.toLino());
});
for(const row of fixture.documents)test(row.name,()=>{
 const original=LinkNetwork.parse(row.source,row.language),before=original.toLino(),packet=createUnsignedSourceDocumentLinks(row.source,row.language,undefined,{firstIdentity:fixture.firstIdentity});assertTransport(original,packet);assert.equal(original.toLino(),before);
 assert.equal(packet.identities[0].toString(),fixture.firstIdentity);assert.ok(packet.identities.every(id=>id.asU64()>BigInt(Number.MAX_SAFE_INTEGER)));
 for(const graph of [packet.network,UnsignedLinkNetwork.fromLino(packet.network.toLino()),decodeUnsignedLinkNetworkBinary(encodeUnsignedLinkNetworkBinary(packet.network))])assert.equal(graph.reconstructText(),row.source);
 assert.equal(UnsignedLinkNetwork.parse(row.source,row.language,undefined,{firstIdentity:fixture.firstIdentity}).toLino(),packet.network.toLino());
});
test('failed rebase preserves the complete source and its allocator',()=>{const graph=UnsignedLinkNetwork.fromLino('(1: 2)\n(2: 1)'),before=encodeUnsignedLinkNetworkBinary(graph);assert.throws(()=>rebaseUnsignedLinkNetwork(graph,'18446744073709551615'));assert.throws(()=>rebaseUnsignedLinkNetwork(graph,1,{recordLimit:1}));assert.deepEqual(encodeUnsignedLinkNetworkBinary(graph),before);});
test('exact source reconstruction retains bytes and complete metadata caches',()=>{const source=LinkNetwork.parseBytes(Uint8Array.from([0,255,128,13,10]),'opaque bytes');source._conceptSyntax.set('meaning\0Rust','selected syntax');const packet=rebaseUnsignedLinkNetwork(source,fixture.firstIdentity);assertTransport(source,packet);assert.deepEqual(packet.network.reconstructBytes(),Uint8Array.from([0,255,128,13,10]));assert.deepEqual(decodeUnsignedLinkNetworkBinary(encodeUnsignedLinkNetworkBinary(packet.network)).reconstructBytes(),packet.network.reconstructBytes());});
test('registry source dispatch remains active before exact allocation',()=>{let calls=0;const selectedConfiguration=ParseConfiguration.default();const registry=new ParserRegistry().register('private source',(text,language,configuration)=>{calls+=1;assert.equal(configuration,selectedConfiguration);return LinkNetwork.parseLosslessText(text,language,configuration);});const packet=createUnsignedSourceDocumentLinksWithRegistry(registry,'custom source','private source',selectedConfiguration,{firstIdentity:fixture.firstIdentity});assert.equal(calls,1);assert.equal(packet.network.reconstructText(),'custom source');assert.equal(UnsignedLinkNetwork.parseWithRegistry(registry,'custom source','private source',selectedConfiguration,{firstIdentity:fixture.firstIdentity}).toLino(),packet.network.toLino());assert.equal(calls,2);});
test('exact allocation source program retains both codecs and four emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/unsigned-source-document-program');for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));});

for(const row of fixture.programs)test(row.name,()=>{
 const original=row.factory==='Rust source owners'?createRustSourceOwnerProgramLinks(row.source):createTypedProgramLinks(row.source,row.language),before=original.network.toLino();
 const packet=row.factory==='Rust source owners'?createUnsignedRustSourceOwnerProgramLinks(row.source,{}, {firstIdentity:fixture.firstIdentity}):createUnsignedTypedProgramLinks(row.source,row.language,{}, {firstIdentity:fixture.firstIdentity});
 assertTransport(original.network,packet);assert.ok(packet.root.asU64()>BigInt(Number.MAX_SAFE_INTEGER));assert.equal(packet.sourceIdentities[packet.identities.findIndex(id=>id.equals(packet.root))].toString(),String(original.root));
 for(const graph of [packet.network,UnsignedLinkNetwork.fromLino(packet.network.toLino()),decodeUnsignedLinkNetworkBinary(encodeUnsignedLinkNetworkBinary(packet.network))]){
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitUnsignedTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(original.network,original.root,target));
  const emission=emitUnsignedTypedProgramLinks(graph,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
  assert.equal(String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);
 }
 const damaged=packet.network.clone();damaged.deleteLink(packet.root);assert.throws(()=>emitUnsignedTypedProgramLinks(damaged,packet.root,'JavaScript'));assert.throws(()=>rebaseUnsignedTypedProgramLinks({network:damaged,root:packet.root},1));assert.equal(original.network.toLino(),before);
});
test('exact Rust typed import passes the selected version and emission models',()=>{
 const source='pub fn result()->u8{7}',original=createRustSourceOwnerProgramLinks(source,{sourceVersion:'1.90.0'}),packet=createUnsignedRustSourceOwnerProgramLinks(source,{sourceVersion:'1.90.0'},{firstIdentity:fixture.firstIdentity});
 assertTransport(original.network,packet);
 for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitUnsignedTypedProgramLinks(packet.network,packet.root,target),emitTypedProgramLinks(original.network,original.root,target));
 assert.throws(()=>emitUnsignedTypedProgramLinks(packet.network,packet.root,'JavaScript',{rustSourceHeaderEmissionModel:'not a model'}));
});
