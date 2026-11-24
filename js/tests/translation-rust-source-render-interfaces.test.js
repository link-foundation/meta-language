// Deferred exact-source, original program, codec/target and transport controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {renderRustFunctionCall,renderRustConstructorFields,acceptRustNamedConstructorFields} from '../src/translation/rust-record-rules.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-render-interfaces.lino',import.meta.url));
for(const row of fixture.calls)test(row.name,()=>assert.equal(renderRustFunctionCall(row.head,row.values),row.expected));
for(const row of fixture.constructors)test(row.name,()=>{assert.equal(renderRustConstructorFields(row.language,row.head,row.names,row.values),row.expected);assert.equal(acceptRustNamedConstructorFields(row.language,row.names),row.named);});
for(const row of fixture.programs)test(row.name,()=>{
 const program=checkProgram(parseRust(row.source)),packet=encodeTypedProgramLinks(program),native=emitRust(program);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root),javascript=emitJavaScript(restored),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(String(runInNewContext(javascript.text+'\n'+entry+'();')),row.expected);assert.deepEqual(emitRust(restored),native);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
test('all complete preceding render definitions emitters and artifacts remain exact',()=>{
 assert.equal(fixture.predecessors.length,19);
 for(const row of fixture.predecessors){const file=new URL('../../'+row.retained,import.meta.url),bytes=row.raw?readFileSync(file):Buffer.from(readProjectData(file).source);assert.equal(bytes.length,row.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sha256);}
});
