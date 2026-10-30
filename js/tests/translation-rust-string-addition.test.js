// Deferred standard String addition values, source ownership and complete reload.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-string-addition-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-string-addition.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source+'fn main(){}'),program=decodeTypedProgramLinks(packet.network,packet.root),options={rustSourceHeaderEmissionModel:'retained constructed Rust string function interface 1'};
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),emitTypedProgramLinks(packet.network,packet.root,'Rust',options));
 }
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 assert.equal(String(runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.value);
});
test('all standard String addition decisions retain their full original premises',()=>{
 for(const row of fixture.candidates)assert.equal(rules.acceptRustStringAdditionCandidate(row.operation,row.ordinary,row.left,row.right),row.expected);
 for(const row of fixture.owners)assert.equal(rules.acceptRustStringAdditionOwner(row.side,row.kind,row.path,row.children),row.expected);
 for(const row of fixture.operands)assert.equal(rules.acceptRustStringAdditionOperand(row.side,row.kind,row.model,row.layers,row.proven,row.references,row.known),row.expected);
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
