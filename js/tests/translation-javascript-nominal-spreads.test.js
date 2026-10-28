// Deferred complete source, copy-order, guard and codec controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {readJavaScriptNominalSpreadError,readJavaScriptNominalSpreadGuardArm} from '../src/translation/javascript-nominal-spread-rules.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-nominal-spreads.lino',import.meta.url));
for(const row of fixture.policies)test(row.name,()=>assert.equal(readJavaScriptNominalSpreadError(row.language,row.constructors,row.declared,row.kinds,row.names,row.owners,row.owner,row.prototype),row.expected));
for(const row of fixture.guards)test('actual absence guard '+row.kind+' '+row.operation+' '+row.absence,()=>assert.equal(readJavaScriptNominalSpreadGuardArm(row.kind,row.operation,row.absence),row.expected));
for(const row of fixture.programs)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program),native=emitRust(program);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root),javascript=emitJavaScript(restored),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);assert.deepEqual(emitRust(restored),native);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source))));
test('every complete preceding source remains exact',()=>{assert.equal(fixture.predecessors.length,11);for(const row of fixture.predecessors){const record=readProjectData(new URL('../../'+row.retained,import.meta.url));assert.equal(record.path,row.path);assert.equal(record.revision,row.revision);assert.equal(Buffer.byteLength(record.source),row.bytes);assert.equal(createHash('sha256').update(record.source).digest('hex'),row.sha256);}});
