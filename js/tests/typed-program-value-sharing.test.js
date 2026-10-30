// Deferred scope identity, source parity and codec controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
import {sharingFixture,observeTypedSharing} from '../../experiments/javascript-properties/typed-sharing-controls.mjs';
assertVersions();
for(const row of sharingFixture.programs)test(row.name,()=>{
 observeTypedSharing(row);const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program,{valueSharing:sharingFixture.model});
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(network,packet.root),decodeTypedProgramLinks(packet.network,packet.root));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.throws(()=>encodeTypedProgramLinks(program,{valueSharing:'unregistered profile'}));
});
test('all preceding complete graph and transport sources remain exact',()=>{assert.equal(sharingFixture.predecessors.length,6);for(const row of sharingFixture.predecessors){const record=readProjectData(new URL('../../'+row.retained,import.meta.url));assert.equal(record.path,row.path);assert.equal(record.revision,row.revision);assert.equal(Buffer.byteLength(record.source),row.bytes);assert.equal(createHash('sha256').update(record.source).digest('hex'),row.sha256);}});
