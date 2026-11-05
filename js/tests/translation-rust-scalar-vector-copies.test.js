// Deferred complete values, actual copy rules and independent source domains.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-scalar-vector-copy-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-scalar-vector-copies.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('every actual source and full-copy decision retains its complete premise',()=>{
 for(const row of fixture.queries)assert.equal(rules.acceptRustScalarVectorCopyQuery(row.ordinary,row.operation,row.model,row.layers),row.expected);
 for(const row of fixture.receivers)assert.equal(rules.acceptRustScalarVectorCopyReceiver(row.kind,row.element,row.proven,row.references,row.copy,row.count),row.expected);
 for(const row of fixture.owners)assert.equal(rules.acceptRustScalarVectorCopyOwner(row.kind,row.path,row.paths,row.children),row.expected);
 for(const row of fixture.occurrences)assert.equal(rules.acceptRustScalarVectorCopyOccurrence(row.kind,row.model,row.wrappers,row.recorded),row.expected);
 for(const row of fixture.values)assert.equal(rules.acceptRustScalarVectorCopyValue(row.language,row.kind,row.operation,row.receiver,row.element,row.reference,row.count,row.startKind,row.start,row.endKind,row.end),row.expected);
});
test('actual scalar copies preserve signed zero, NaN, infinities, input and independent result identities',()=>{
 const packet=createRustSourceOwnerProgramLinks(fixture.controls.source),javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
 for(const name of fixture.controls.names){
  const target=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target,operation=runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+target+';',{}, {timeout:1000});
  const input=Object.freeze(fixture.controls.values.map(Number)),first=operation(input),second=operation(input);
  assert.notStrictEqual(first,input);assert.notStrictEqual(first,second);assert.equal(first.length,input.length);assert.equal(second.length,input.length);
  for(let index=0;index<input.length;index+=1){const expected=Number(fixture.controls.values[index]);assert.ok(Object.is(input[index],expected));assert.ok(Object.is(first[index],expected));assert.ok(Object.is(second[index],expected));}
 }
 const program=decodeTypedProgramLinks(packet.network,packet.root),before=[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]);
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),before);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const rust=emitTypedProgramLinks(packet.network,packet.root,'Rust').code;assert.ok(rust.includes('::std::clone::Clone::clone'));assert.ok(!rust.includes('ml_array::slice('));
});
test('the exact reference-clone overload keeps all original refusal metadata',()=>{
 const previous=readProjectData(new URL('../../parity/fixtures/rust-array-source.lino',import.meta.url)).invalid.find(row=>row.id==='array-clone');assert.deepEqual(fixture.refusals[0].legacy,previous);
});
