// Deferred source assertions. Nothing in this file is execution evidence.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import * as rules from '../src/translation/rust-owned-recursive-datatype-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustOwnedRecursiveContainerDecisions as decisions,rustOwnedRecursiveContainerOriginals as originals,rustOwnedRecursiveContainerRefusals as refusals,rustOwnedRecursiveContainerPreviousPendingOriginals as previousPending} from './fixtures/rust-owned-recursive-containers.mjs';
const versions=['1.90.0','1.99.0'];
test('previous pending originals retain their complete independent source and stdout controls',()=>{
 for(const previous of previousPending){const selected=originals.filter(row=>row.name===previous.name);assert.equal(selected.length,1);assert.deepEqual(selected[0],previous);}
});
function observe(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('independent recursive container source policy '+index,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
for(const row of originals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=encodeTypedProgramLinks(checked);for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(graph,packet.root),checked);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of refusals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>row.originalCompilerOutcome==='accept'?error.kind==='unsupported':['unsupported','type','syntax'].includes(error.kind)));
