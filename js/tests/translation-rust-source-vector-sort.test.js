// Deferred source/frontend/transport controls. Authored during source-only work.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {checkRustSourceVectorSort} from '../src/translation/rust-source-vector-sort.js';
import {rustSourceVectorSortOriginals as originals,rustSourceVectorSortRefusals as refusals,rustSourceVectorSortVersions as versions} from './fixtures/rust-source-vector-sort.mjs';
const options={sourceVersion:'1.90.0',exclusiveSourceModel:'Rust exclusive source loans 1'};
function stdout(text){const lines=[];vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:10000});return lines.length?lines.join('\n')+'\n':'';}
test('actual nominal field comparators lower through complete source algorithms and preserve stable UTF16 order',()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 for(const row of originals){
  const program=checkProgram(parseRust(row.source,options));assert.equal(stdout(emitJavaScript(program).text),row.stdout,row.name);
  for(const emit of [emitLean,emitRocq]){const result=emit(program);assert.ok(result.encodings.some(value=>value.id==='utf16-formal-strings'),row.name);assert.ok(result.text.includes('MLUtf16Text.compare'),row.name);}
  const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:options.exclusiveSourceModel});
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.equal(stdout(emitTypedProgramLinks(network,packet.root,'JavaScript').code),row.stdout,row.name);
 }
});
test('unproved comparators and invented sort descriptors refuse',()=>{
 for(const row of refusals)assert.throws(()=>checkProgram(parseRust(row.source,options)),error=>['syntax','type','unsupported'].includes(error.kind),row.name);
 assert.equal(checkRustSourceVectorSort({}, {op:'other'},new Map(),[]),null);
 assert.throws(()=>checkRustSourceVectorSort({language:'Rust'},{op:'rustSourceStableUtf16VectorSort',sourceSortAdmission:{},args:[]},new Map(),[]),/actual parsed source comparator/);
});
