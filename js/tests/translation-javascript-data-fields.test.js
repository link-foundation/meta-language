// Deferred complete source controls, not run during the implementation draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {createHash} from 'node:crypto';
import {readJavaScriptDataFieldPlan,readJavaScriptDataFieldInferenceForm,acceptJavaScriptDataFieldOwnerFallback} from '../src/translation/javascript-data-field-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-data-fields.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
for(const row of fixture.cases)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.value);
 const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program),native=emitRust(program);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root),output=emitJavaScript(restored);
  const entry=output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(output.text+'\n'+entry+'();'),row.value);
  assert.deepEqual(emitRust(restored),native);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
test('nominal owner fallback waits for actual collection and record constraints',()=>{
 for(const row of fixture.ownerFallbackCases)assert.equal(acceptJavaScriptDataFieldOwnerFallback(row.kind,row.owners,row.settled),row.expected);
 assert.equal(fixture.continuationPredecessors.length,6);
 for(const record of fixture.continuationPredecessors){const prior=readProjectData(new URL('../../'+record.retained,import.meta.url));assert.equal(prior.path,record.path);assert.equal(prior.revision,record.revision);assert.equal(Buffer.byteLength(prior.source),record.bytes);assert.equal(createHash('sha256').update(prior.source).digest('hex'),record.sha256);}
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind==='unsupported'));
test('all preceding checker inference and projection source bytes remain exact',()=>{
 assert.equal(fixture.predecessors.length,5);
 for(const record of fixture.predecessors){const prior=readProjectData(new URL('../../'+record.retained,import.meta.url));assert.equal(prior.path,record.path);assert.equal(prior.revision,record.revision);assert.equal(Buffer.byteLength(prior.source),record.bytes);assert.equal(createHash('sha256').update(prior.source).digest('hex'),record.sha256);}
});
for(const row of fixture.plans)test(row.name,()=>{
 assert.deepEqual(readJavaScriptDataFieldPlan(row.language,row.field,row.inherited,row.positions,row.types),row.expected);
 assert.equal(readJavaScriptDataFieldInferenceForm(row.field,row.positions),row.inference);
});
