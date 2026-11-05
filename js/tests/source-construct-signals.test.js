// Deferred controls: never import this module during all-requirements drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readSourceConstructSignals} from '../src/translation/source-construct-signal-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/source-construct-signals.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
for(const row of fixture.cases)test(row.name,()=>assert.deepEqual(readSourceConstructSignals(String.fromCharCode(...row.sourceUnits),row.needleUnits.map(units=>String.fromCharCode(...units))),row.expected));
test('shared source definition survives both graph transports for every target',()=>{
 const packet=createSourceDefinitionProgramLinks('meta-language/source-program/source-construct-signal-program');
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
