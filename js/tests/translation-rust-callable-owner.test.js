import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readRustScalarValueType} from '../src/translation/rust-scalar-source-interface-rules.js';
import * as rules from '../src/translation/rust-callable-owner-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-callable-owner.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
const options=fixture.model?{rustSourceHeaderEmissionModel:fixture.model}:{};
test('every recursive callable owner contract join equality and resource premise stays explicit',()=>{
 for(const row of fixture.contractErrors)assert.equal(rules.readRustCallableOwnerError(row.actual,row.expected),row.expectedError);
 for(const row of fixture.equalities)assert.equal(rules.acceptRustCallableOwnerDescriptorsEqual(row.left,row.right),row.expected);
 for(const row of fixture.joins)assert.equal(rules.readRustCallableOwnerJoinError(row.rows),row.expectedError);
 for(const row of fixture.limits)assert.equal(rules.readRustCallableOwnerProjectionLimit(row.name),row.expected);
 for(const row of fixture.oversized){const descriptor=Array(row.size).fill('');assert.equal(rules.readRustCallableOwnerError(descriptor,[]),row.expectedError);assert.equal(rules.readRustCallableOwnerJoinError(descriptor),row.expectedError);assert.equal(rules.acceptRustCallableOwnerDescriptorsEqual(descriptor,descriptor),false);}
});
test('the corrected emitter scope retains the complete initial draft and every original source/type input',()=>{
 assert.equal(readRustScalarValueType('function',0,false),'');
 for(const correction of fixture.assertionCorrections){const path=new URL('../../'+correction.retainedDraft,import.meta.url),bytes=readFileSync(path),prior=readProjectData(path);assert.equal(bytes.length,correction.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),correction.sha256);assert.ok(prior.cases.length<=fixture.cases.length);for(let at=0;at<prior.cases.length;at+=1){const before=prior.cases[at],after=fixture.cases[at];for(const field of ['name','module','source','expected','resultType'])assert.deepEqual(after[field],before[field]);for(const check of before.checks)assert.ok(after.checks.includes(check));}}
});
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),before=packet.network.toLino(),program=decodeTypedProgramLinks(packet.network,packet.root),retained=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),retained);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.equal(row.checks.length,row.canonicalChecks.length);assert.equal(row.checks.length,row.sourceContracts.length);
 for(const contract of row.sourceContracts){const signature=program.declarations.get(contract.name).sourceSignature;assert.deepEqual(signature.parameters.map(position=>position.owner),contract.parameters);assert.deepEqual(signature.result.owner,contract.result);}
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
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));

for(const row of fixture.malformed)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source)));
