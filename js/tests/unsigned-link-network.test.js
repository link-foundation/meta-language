import assert from 'node:assert/strict';
import {test} from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {UnsignedLinkId,UnsignedLinkNetwork,createDenseUnsignedLinkNetwork,restoreDenseUnsignedLinkNetwork,
 encodeUnsignedLinkNetworkBinary,decodeUnsignedLinkNetworkBinary} from '../src/unsigned-link-network.js';
import {LinkId} from '../src/primitives.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createUnsignedLinkIdentityProgramLinks} from '../src/unsigned-link-identity-program.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-link-network.lino',import.meta.url));
const integer=(value,size)=>{const bytes=new Uint8Array(size),view=new DataView(bytes.buffer);if(size===8)view.setBigUint64(0,BigInt(value),true);else view.setUint32(0,Number(value),true);return Array.from(bytes);};
function independentlyFrame(view){
 const body=Array.from(encodeLinkNetworkBinary(view.network));
 return Uint8Array.from([...new TextEncoder().encode('MLU64N01'),view.nextIdentity===null?1:0,
  ...integer(view.nextIdentity?.asU64()??1n,8),...integer(view.directory.length,4),
  ...view.directory.flatMap(id=>integer(id.asU64(),8)),...integer(body.length,4),...body]);
}
test('full unsigned IDs retain exact bits, immutable state and the old Number refusal domain',()=>{
 for(const text of fixture.valid){const id=UnsignedLinkId.from(text);assert.equal(id.toString(),text);assert.equal(id.asU64(),BigInt(text));assert.equal(UnsignedLinkId.from(BigInt(text)).equals(id),true);assert.equal(JSON.stringify(id),JSON.stringify(text));assert.equal(Object.isFrozen(id),true);const words=id.words();words[0]=0;assert.equal(id.toString(),text);}
 for(const text of fixture.invalid)assert.throws(()=>UnsignedLinkId.from(text));
 for(const input of [Number.MAX_SAFE_INTEGER+1,NaN,Infinity,{},null,undefined])assert.throws(()=>UnsignedLinkId.from(input));
 for(const input of [9007199254740992n,'9007199254740993','18446744073709551615'])assert.throws(()=>LinkId.from(input));
 assert.equal(UnsignedLinkId.from('00000000000000000001').toString(),'1');
 assert.equal(UnsignedLinkId.from('9007199254740992').equals('9007199254740993'),false);
});
for(const row of fixture.cases)test(row.name,()=>{
 const graph=UnsignedLinkNetwork.fromLino(row.source),text=graph.toLino();
 assert.deepEqual(graph.links().map(link=>link.id().toString()),row.identities);
 assert.equal(graph.nextIdentity?.toString()??null,row.next);
 assert.equal(UnsignedLinkNetwork.fromLino(text).toLino(),text);
 const view=createDenseUnsignedLinkNetwork(graph),bytes=encodeUnsignedLinkNetworkBinary(graph);
 assert.deepEqual(bytes,independentlyFrame(view));
 const loaded=decodeUnsignedLinkNetworkBinary(bytes);
 assert.equal(loaded.toLino(),text);assert.equal(loaded.nextIdentity?.toString()??null,row.next);
 assert.deepEqual(encodeUnsignedLinkNetworkBinary(loaded),bytes);
 assert.equal(restoreDenseUnsignedLinkNetwork(view.network,view.directory,view.nextIdentity).toLino(),text);
 if(row.registered)assert.equal(loaded.findTerm(row.registered).toString(),row.identities[0]);
 for(let length=0;length<bytes.length;length+=1)assert.throws(()=>decodeUnsignedLinkNetworkBinary(bytes.slice(0,length)));
 assert.throws(()=>decodeUnsignedLinkNetworkBinary(Uint8Array.from([...bytes,0])));
 if(row.next===null){for(let attempt=0;attempt<2;attempt+=1)assert.throws(()=>loaded.insertLink());assert.deepEqual(encodeUnsignedLinkNetworkBinary(loaded),bytes);}
 else assert.equal(loaded.insertLink().toString(),row.next);
});
test('deleted allocation history, exact caches and edited graphs survive binary state reconstruction',()=>{
 const graph=UnsignedLinkNetwork.fromLino(fixture.cases.at(-1).source),id=graph.findTerm('meaning');
 graph.link(id).setMetadata(graph.link(id).metadata().withTerm('changed'));
 graph._conceptSyntax.set('meaning\0JavaScript','selected syntax');
 graph.deleteLink(graph.insertLinkWithOptionalId('18446744073709551614'));
 const bytes=encodeUnsignedLinkNetworkBinary(graph),restored=decodeUnsignedLinkNetworkBinary(bytes);
 assert.equal(restored.findTerm('meaning').toString(),id.toString());assert.equal(restored.findTerm('changed'),undefined);
 assert.equal(restored._conceptSyntax.get('meaning\0JavaScript'),'selected syntax');
 assert.equal(restored.nextIdentity.toString(),'18446744073709551615');
 assert.equal(restored.insertLink().toString(),'18446744073709551615');assert.equal(restored.nextIdentity,null);
 assert.throws(()=>restored.insertLink());assert.equal(graph.clone().toLino(),graph.toLino());
});
test('invalid edits, exhausted field allocation and forged directories are transactional',()=>{
 const graph=UnsignedLinkNetwork.fromLino('(18446744073709551614: 1)'),before=encodeUnsignedLinkNetworkBinary(graph);
 assert.throws(()=>graph.insertLink(['18446744073709551616']));assert.deepEqual(encodeUnsignedLinkNetworkBinary(graph),before);
 assert.throws(()=>graph.insertFieldRelation(1,'new label',1));assert.deepEqual(encodeUnsignedLinkNetworkBinary(graph),before);
 const view=createDenseUnsignedLinkNetwork(graph);
 assert.throws(()=>restoreDenseUnsignedLinkNetwork(view.network,[...view.directory].reverse(),view.nextIdentity));
 assert.throws(()=>restoreDenseUnsignedLinkNetwork(view.network,view.directory,UnsignedLinkId.from(1)));
 const graph2=UnsignedLinkNetwork.fromLino('(1: 1)'),one=createDenseUnsignedLinkNetwork(graph2);
 assert.throws(()=>restoreDenseUnsignedLinkNetwork(one.network,[...one.directory,UnsignedLinkId.from(3)],one.nextIdentity));
 const raw=independentlyFrame(one);raw[8]=2;assert.throws(()=>decodeUnsignedLinkNetworkBinary(raw));
 const duplicate=createDenseUnsignedLinkNetwork(UnsignedLinkNetwork.fromLino('(1: 2)\n(2: 1)'));
 const bytes=independentlyFrame(duplicate);bytes.set(bytes.slice(21,29),29);assert.throws(()=>decodeUnsignedLinkNetworkBinary(bytes));
 assert.throws(()=>encodeUnsignedLinkNetworkBinary(graph,{recordLimit:0}));
 assert.throws(()=>encodeUnsignedLinkNetworkBinary(graph,{byteLimit:33}));
});
test('complete source-defined identity program survives both codecs and all four target emissions',()=>{
 const packet=createUnsignedLinkIdentityProgramLinks();
 const text=LinkNetwork.fromLino(packet.network.toLino());
 const binary=decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network));
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const graph of [text,binary])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),expected);
 }
});
