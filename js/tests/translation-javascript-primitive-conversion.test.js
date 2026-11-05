// Deferred controls: never import while drafting source.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions,primitiveFixture,observePrimitiveConversion,observeInvalidPrimitiveState} from '../../experiments/javascript-properties/primitive-conversion-controls.mjs';
assertVersions();
for(const row of primitiveFixture.cases)test(row.name,()=>observePrimitiveConversion(row));
for(const row of primitiveFixture.invalid)test(row.name,()=>observeInvalidPrimitiveState(row));
test('the complete preceding ordinary heap controls remain exact',()=>{
 const record=primitiveFixture.oraclePredecessor,prior=readProjectData(new URL('../../'+record.retained,import.meta.url));
 assert.equal(prior.path,record.path);assert.equal(prior.revision,record.revision);assert.equal(Buffer.byteLength(prior.source),record.bytes);assert.equal(createHash('sha256').update(prior.source).digest('hex'),record.sha256);
});
test('complete primitive conversion source retains both codecs and four targets',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-primitive-conversion-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
