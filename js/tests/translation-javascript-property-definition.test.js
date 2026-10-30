// Deferred complete definition protocol and source graph controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions,definitionFixture,observePropertyDefinition,observeInvalidPropertyDefinition} from '../../experiments/javascript-properties/property-definition-controls.mjs';
assertVersions();
for(const row of definitionFixture.cases)test(row.name,()=>observePropertyDefinition(row));
for(const row of definitionFixture.invalid)test(row.name,()=>observeInvalidPropertyDefinition(row));
test('complete property definition source retains both codecs and four targets',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-property-definition-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
