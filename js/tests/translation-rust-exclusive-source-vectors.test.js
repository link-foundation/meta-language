// Deferred complete source controls. Actual source helpers use shared checked
// folds and Option values; no original/generation runs on module import.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustExclusiveSourceVectorMethodArity,acceptRustExclusiveSourceVectorReceiver} from '../src/translation/rust-exclusive-source-vector-rules.js';
import {rustExclusiveSourceVectorOriginals as originals,rustExclusiveSourceVectorRefusals as refusals,rustExclusiveSourceVectorVersions as versions} from './fixtures/rust-exclusive-source-vectors.mjs';
function output(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function nodes(root){const result=[],pending=[root],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<1000000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
for(const row of originals)for(const sourceVersion of ['1.90.0','1.99.0'])test('exclusive Vec actual source '+row.name+'/'+sourceVersion,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 const surface=parseRust(row.source,{sourceVersion,exclusiveSourceModel:versions.model}),checked=checkProgram(surface);assert.equal(output(emitJavaScript(checked).text),row.stdout);
 assert.ok(nodes(surface).some(node=>node.k==='fn'&&node.generated));assert.ok(!nodes(checked).some(node=>String(node.op??'').startsWith('rustExclusiveSource')));
 const concrete=LinkNetwork.parse(row.source,'Rust');for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
});
for(const row of refusals)test('exclusive Vec retained source boundary '+row.name,()=>{assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion:'1.90.0',exclusiveSourceModel:versions.model})),error=>['type','unsupported'].includes(error.kind));});
test('Vec source policy requires actual direct owning standard storage',()=>{
 assert.equal(readRustExclusiveSourceVectorMethodArity('pop'),0);assert.equal(readRustExclusiveSourceVectorMethodArity('clear'),0);assert.equal(readRustExclusiveSourceVectorMethodArity('truncate'),1);for(const method of ['reserve','capacity','insert','remove','retain'])assert.equal(readRustExclusiveSourceVectorMethodArity(method),-1);
 assert.equal(acceptRustExclusiveSourceVectorReceiver('array','owned vector',0,true,'library','std::vec::Vec',1),true);
 for(const row of [['array','owned vector',1,true,'library','std::vec::Vec',1],['array','slice',0,true,'library','std::vec::Vec',1],['array','owned vector',0,false,'library','std::vec::Vec',1],['array','owned vector',0,true,'declaration','Vec',1],['array','owned vector',0,true,'library','std::vec::Vec',2]])assert.equal(acceptRustExclusiveSourceVectorReceiver(...row),false);
});
test('Vec helpers retain exact whole-value folds without floating slice bounds',()=>{
 for(const index of [0,4]){const surface=parseRust(originals[index].source,{sourceVersion:'1.90.0',exclusiveSourceModel:versions.model}),helpers=surface.items.filter(node=>node.k==='fn'&&node.generated),all=nodes(helpers);assert.ok(all.some(node=>node.k==='method'&&node.op==='reduce'));assert.ok(!all.some(node=>node.k==='method'&&node.op==='slice'));assert.ok(!all.some(node=>node.k==='cast'&&node.to?.kind==='float'));}
});
test('owned element Vec packet survives typed Links and every emitter',()=>{
 const row=originals[7],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:versions.model}),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),binary);assert.ok(decodeTypedProgramLinks(graph,packet.root).items.length>0);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
