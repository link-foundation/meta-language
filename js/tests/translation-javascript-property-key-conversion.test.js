// Deferred complete key protocol and source graph controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions,keyFixture,observePropertyKeyConversion,observeInvalidPropertyKeyState} from '../../experiments/javascript-properties/property-key-conversion-controls.mjs';
assertVersions();
for(const row of keyFixture.cases)test(row.name,()=>observePropertyKeyConversion(row));
for(const row of keyFixture.invalid)test(row.name,()=>observeInvalidPropertyKeyState(row));
test('complete property key source retains both codecs and all four targets',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-property-key-conversion-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
