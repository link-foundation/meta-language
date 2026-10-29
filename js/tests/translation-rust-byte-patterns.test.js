// Deferred complete-source controls; source authors never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-byte-pattern-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-byte-patterns.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.tokens)test('actual byte pattern token '+row.token,()=>assert.equal(rules.acceptRustSourceBytePatternToken(row.token),row.expected));
for(const [index,row]of fixture.types.entries())test('byte pattern type domain '+index,()=>assert.equal(rules.readRustSourceBytePatternTypeError(row.kind,row.bits,row.signed),row.expected));
for(const row of fixture.aliases)test('actual literal alias '+row.form,()=>assert.equal(rules.acceptRustSourceLiteralPatternAlias(row.form),row.expected));
for(const [index,row]of fixture.inference.entries())test('actual byte scrutinee constraint '+index,()=>assert.equal(rules.acceptRustSourceByteLiteralInference(row.language,row.kind,row.typed,row.patterns),row.expected));
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source+'fn main(){}'),program=decodeTypedProgramLinks(packet.network,packet.root);
 const views=[packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
 for(const network of views){
  const restored=decodeTypedProgramLinks(network,packet.root);
  assert.deepEqual([...restored.declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  const emission=emitTypedProgramLinks(network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
  const observe=()=>String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}));
  if(row.abort)assert.throws(observe,error=>String(error.message).includes(row.abort));else assert.equal(observe(),row.value);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
