// Deferred complete executor, independent source and source graph controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {assertVersions,machineFixture,observeCallMachine,observeInvalidCallProgram,observeInvalidCallMachine,observeSourceErrorResumptionRefusal} from '../../experiments/javascript-properties/call-machine-controls.mjs';
assertVersions();
for(const row of machineFixture.programs)test(row.name,()=>observeCallMachine(row));
for(const row of machineFixture.invalid)test(row.name,()=>observeInvalidCallProgram(row));
for(const row of machineFixture.invalidStates)test(row.name,()=>observeInvalidCallMachine(row));
for(const row of machineFixture.errorResumptionRefusals)test(row.name,()=>observeSourceErrorResumptionRefusal(row));
test('the actual machine source graph keeps both codecs and all four targets',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/javascript-call-machine-program');
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
