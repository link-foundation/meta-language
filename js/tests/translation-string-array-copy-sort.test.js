// Deferred controls; no observations are recorded by declaring these tests.
import assert from 'node:assert/strict';
import test from 'node:test';
import {spawnSync} from 'node:child_process';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/string-array-copy-sort.lino',import.meta.url));
test('original primitive Node source and translated products retain every declared unit sequence',()=>{
 const process=spawnSync(globalThis.process.execPath,['--max-old-space-size=352','../experiments/javascript-program/string-array-copy-sort.mjs'],{cwd:new URL('../',import.meta.url),encoding:'utf8',maxBuffer:16*1024*1024});assert.equal(process.status,0,process.stderr);
 const product=readStructuredDataLino(process.stdout),packet=encodeTypedProgramLinks(checkProgram(parseJavaScript(fixture.source)));
 assert.equal(packet.network.toLino(),product.links);assert.equal(packet.root.asU64(),product.root);
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  for(const target of ['JavaScript','Rust'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),product.emissions[target]);
  for(const target of ['Lean','Rocq'])assert.throws(()=>emitTypedProgramLinks(network,packet.root,target),error=>error.kind==='unsupported'&&error.reason===fixture.formalRefusalReason);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind==='unsupported'&&error.reason===fixture.refusalReason));
test('shared receivers retain their separate identity and storage obligation',()=>assert.throws(()=>checkProgram(parseJavaScript(fixture.source,{referenceSemantics:true})),error=>error.kind==='unsupported'&&error.reason===fixture.refusalReason));
test('a changed result type cannot authenticate a copying-sort wire operation',()=>{const program=checkProgram(parseJavaScript(fixture.source)),changed=structuredClone(program);changed.declarations.get('sortStrings').body.type={kind:'bool'};assert.throws(()=>encodeTypedProgramLinks(changed));});
