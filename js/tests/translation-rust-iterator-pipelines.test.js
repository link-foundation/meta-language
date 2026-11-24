import {acceptRustCopyClosureParameterSource} from '../src/translation/rust-copy-closure-parameter-rules.js';
// Deferred controls: source authors may syntax-parse, never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-iterator-pipeline-rules.js';
import * as parameters from '../src/translation/rust-copy-closure-parameter-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-iterator-pipelines.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const [index,row]of fixture.premises.entries())test('iterator decision '+index,()=>assert.deepEqual((rules[row.operation]??parameters[row.operation])(...row.args),row.expected));
for(const [index,row]of fixture.parameterPremises.entries())test('immutable scalar parameter '+index,()=>assert.equal(acceptRustCopyClosureParameterSource(row.known,row.baseCopy,row.layers,row.reference,row.primitiveOwner),row.expected));
for(const row of fixture.cases)test(row.name,()=>{
 const original=row.source+'fn main(){}',concrete=LinkNetwork.parse(original,'Rust');for(const network of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(network.reconstructText(),original);
 const packet=createRustSourceOwnerProgramLinks(original),program=decodeTypedProgramLinks(packet.network,packet.root);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);assert.deepEqual([...restored.declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  const emission=emitTypedProgramLinks(network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target,execute=()=>runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});
  if(row.abort===null)assert.equal(String(execute()),row.value);else assert.throws(execute,error=>error.message===row.abort);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
