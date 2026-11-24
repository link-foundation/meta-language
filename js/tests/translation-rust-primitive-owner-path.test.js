import * as associated from '../src/translation/rust-primitive-associated-call-rules.js';
import * as conversions from '../src/translation/rust-primitive-from-rules.js';
import * as constants from '../src/translation/rust-primitive-integer-constant-rules.js';
import {createHash} from 'node:crypto';
import * as rules from '../src/translation/rust-primitive-owner-path-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino as readProjectDataFromText} from '../src/structured-data-links.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-primitive-owner-path.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
const options={rustSourceHeaderEmissionModel:fixture.model};
test('every original primitive path domain and separate From extension stays explicit',()=>{
 for(const row of fixture.owners){const args=[row.segments,row.ordinary,row.shadowed,row.absolute];assert.equal(rules.readRustPrimitiveOwnerPath(...args),row.expected);assert.equal(associated.readRustPrimitiveAssociatedOwner(...args),row.expected);assert.equal(constants.readRustPrimitiveIntegerConstantOwner(...args),row.expected==='bool'?'':row.expected);assert.equal(conversions.readRustPrimitiveFromOwner(...args),row.fromExpected);}
});
test('complete preceding generation recipes retain every obsolete source part',()=>{
 const archive=readProjectData(new URL('../../'+fixture.retainedRecipes,import.meta.url)),receipt=readProjectData(new URL('../../'+fixture.isolationReceipt,import.meta.url));
 assert.equal(archive.recipes.length,fixture.recipes.length);
 for(const row of fixture.recipes){const retained=archive.recipes.find(item=>item.path===row.path);assert.equal(Buffer.byteLength(retained.source),row.bytes);assert.equal(createHash('sha256').update(retained.source).digest('hex'),row.sha256);}
 for(const row of receipt.removed){const retained=archive.recipes.find(item=>item.path===row.retainedRecipe),record=readProjectDataFromText(retained.source),part=record.files.find(item=>'rust/src/translation/'+item.path===row.path);assert.equal(Buffer.byteLength(part.source),row.bytes);assert.equal(createHash('sha256').update(part.source).digest('hex'),row.sha256);}
});
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),before=packet.network.toLino(),program=decodeTypedProgramLinks(packet.network,packet.root),retained=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),retained);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 assert.equal(String(runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);
 assert.equal(packet.network.toLino(),before);
});
for(const row of fixture.printedSources)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),lines=[],emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
 runInNewContext(emission.code.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 assert.deepEqual(lines,row.lines);
 for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
});
