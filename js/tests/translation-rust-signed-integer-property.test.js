import * as rules from '../src/translation/rust-signed-integer-property-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-signed-integer-property.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
const options={rustSourceHeaderEmissionModel:fixture.model};
test('every signed property method plan result and receiver premise remains explicit',()=>{
 for(const row of fixture.methods)assert.equal(rules.acceptRustSignedIntegerPropertyMethod(row.method,row.ordinary),row.expected);
 for(const row of fixture.plans){assert.deepEqual(rules.planRustSignedIntegerProperty(row.method),row.expected);assert.equal(rules.readRustSignedIntegerPropertyResult(row.method),row.result);}
 for(const row of fixture.receivers)assert.equal(rules.acceptRustSignedIntegerPropertyReceiver(row.kind,row.bits,row.signed,row.owner,row.path,row.children,row.proven,row.layers),row.expected);
});
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),before=packet.network.toLino(),program=decodeTypedProgramLinks(packet.network,packet.root),retained=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),retained);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 assert.equal(String(runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);
 assert.equal(packet.network.toLino(),before);
});
for(const row of fixture.printedSources)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),lines=[],emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
 runInNewContext(emission.code.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 assert.deepEqual(lines,row.lines);
 for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
test('every shared sign plan operand refers to one actual receiver evaluation',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('all sign-plan')),packet=createRustSourceOwnerProgramLinks(row.source),body=decodeTypedProgramLinks(packet.network,packet.root).declarations.get('select').body;
 assert.equal(body.k,'let');assert.equal(body.value.k,'call');assert.equal(body.value.fn,'input');
 const pending=[body],seen=new WeakSet();let calls=0;
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);if(node.k==='call'&&node.fn==='input')calls+=1;for(const [field,child] of Object.entries(node))if(!['span','type','domain'].includes(field)){if(Array.isArray(child))pending.push(...child);else if(child&&typeof child==='object')pending.push(child);}}
 assert.equal(calls,1);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
