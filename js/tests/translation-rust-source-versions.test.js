// Deferred version/context/codec controls; never import while source drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as columns from '../src/translation/source-version-column-rules.js';
import * as versions from '../src/translation/rust-source-version-rules.js';
import * as errors from '../src/translation/rust-primitive-try-from-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-versions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
for(const [index,row]of fixture.premises.entries())test('source version decision '+index,()=>assert.deepEqual((versions[row.operation]??errors[row.operation]??columns[row.operation])(...row.args),row.expected));
for(const version of fixture.sourceVersions)for(const row of fixture.cases)test(version+' '+row.name,()=>{
 const source=row.source+'fn main(){}',concrete=LinkNetwork.parse(source,'Rust');assert.equal(LinkNetwork.fromLino(concrete.toLino()).reconstructText(),source);
 for(const packet of [createRustSourceOwnerProgramLinks(source,{sourceVersion:version}),createTypedProgramLinks(source,'Rust',{rustSourceVersion:version})]){
  const before=decodeTypedProgramLinks(packet.network,packet.root);
  for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
   assert.deepEqual(decodeTypedProgramLinks(network,packet.root).items,before.items);
   assert.ok(network.toLino().includes(version),'the explicit source std version stays in Links provenance');
   for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
   const emission=emitTypedProgramLinks(network,packet.root,'JavaScript'),name=emission.mappings.find(row=>row.kind==='function'&&row.source==='result').target,execute=()=>runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}),expected=row.expectedByVersion[version];
   if(expected.kind==='abort')assert.throws(execute,error=>error.message===expected.message);else assert.equal(String(execute()),expected.value);
  }
 }
});
for(const version of fixture.invalidVersions)test('uncovered source version '+version,()=>assert.throws(()=>parseRust('fn main(){}',{sourceVersion:version}),error=>error.kind==='unsupported'&&error.reason===fixture.versionError));
test('source version selectors require their source language',()=>assert.throws(()=>createTypedProgramLinks('','JavaScript',{rustSourceVersion:'1.90.0'}),TypeError));

test('the source-version option is captured once before parsing and provenance',()=>{
 for(const make of [options=>createRustSourceOwnerProgramLinks(fixture.cases[0].source+'fn main(){}',options),options=>createTypedProgramLinks(fixture.cases[0].source+'fn main(){}','Rust',Object.defineProperty({},'rustSourceVersion',{get:()=>options.sourceVersion}))]){
  let reads=0;const options=Object.defineProperty({},'sourceVersion',{get:()=>{reads+=1;return reads===1?'1.96.0':'1.99.0';}}),packet=make(options);assert.equal(reads,1);
  const emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=emission.mappings.find(row=>row.kind==='function'&&row.source==='result').target;
  assert.throws(()=>runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}),error=>error.message===fixture.cases[0].expectedByVersion['1.96.0'].message);
 }
});
