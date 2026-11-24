// Deferred conversion/oracle controls; never run during the complete draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions,conversionFixture,observeDescriptorConversion,observeSemanticBoolean,observeInvalidDescriptorState} from '../../experiments/javascript-properties/descriptor-conversion-controls.mjs';
assertVersions();
for(const row of conversionFixture.cases)test(row.name,()=>observeDescriptorConversion(row));
for(const row of conversionFixture.booleans)test('ToBoolean '+row.name,()=>observeSemanticBoolean(row));
for(const row of conversionFixture.invalid)test(row.name,()=>observeInvalidDescriptorState(row));
test('the complete preceding heap oracle remains available without changing its inputs',()=>{
 const record=conversionFixture.oraclePredecessor,prior=readProjectData(new URL('../../'+record.retained,import.meta.url));assert.equal(prior.path,record.path);assert.equal(prior.revision,record.revision);assert.equal(Buffer.byteLength(prior.source),record.bytes);assert.equal(createHash('sha256').update(prior.source).digest('hex'),record.sha256);
});
test('the complete conversion source graph retains both codecs and all four targets',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-descriptor-conversion-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
