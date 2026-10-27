// Deferred independent ordinary-object snapshot/call controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {assertVersions,heapFixture,observeHeapCase,observeInvalidHeap} from '../../experiments/javascript-properties/heap-controls.mjs';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
assertVersions();
for(const row of heapFixture.cases)test(row.name,()=>observeHeapCase(row));
for(const row of heapFixture.invalid)test(row.name,()=>observeInvalidHeap(row));
test('the complete heap source graph keeps both codecs and all four emissions',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-ordinary-heap-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
