import * as rules from '../src/translation/rust-wrapping-integer-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-wrapping-integer.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
const options={rustSourceHeaderEmissionModel:fixture.model};
test('every wrapping arity and receiver premise remains explicit',()=>{
 for(const row of fixture.arities)assert.equal(rules.readRustWrappingIntegerArity(row.method,row.ordinary),row.expected);
 for(const row of fixture.receivers)assert.equal(rules.acceptRustWrappingIntegerReceiver(row.method,row.kind,row.bits,row.signed,row.owner,row.path,row.children,row.proven,row.layers),row.expected);
 for(const method of ['wrapping_mul','wrapping_div','unknown'])for(const signed of [false,true])assert.deepEqual(rules.planRustWrappingInteger(method,signed),[]);
 assert.deepEqual(rules.planRustWrappingInteger('wrapping_abs',false),[]);
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
test('the actual wrapping receiver and argument calls occur once in source order',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('actual wrapping operands')),packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),calls=[];
 let body=program.declarations.get('select').body;
 while(body.k==='let'){if(body.value.k==='call')calls.push(body.value.fn);body=body.body;}
 assert.deepEqual(calls,['first','second']);
 const pending=[program.declarations.get('select').body],seen=new WeakSet(),actual=[];
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);if(node.k==='call')actual.push(node.fn);for(const [field,child] of Object.entries(node))if(!['span','type','domain'].includes(field)){if(Array.isArray(child))pending.push(...child);else if(child&&typeof child==='object')pending.push(child);}}
 assert.deepEqual(actual.sort(),['first','second']);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
