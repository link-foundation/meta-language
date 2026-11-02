// Deferred actual-library/source controls. No projection or original runs at
// module initialization; no hand-authored replacement kernel is supplied.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustExclusiveCollectionSourceOriginals as originals,rustExclusiveCollectionSourceRefusals as refusals,rustExclusiveCollectionSourceVersions as versions} from './fixtures/rust-exclusive-collection-source.mjs';
import {readRustStandardCollectionSourceTestLibrary as library} from './support/rust-standard-collection-library.js';
function options(sourceVersion='1.90.0'){return {sourceVersion,exclusiveSourceModel:versions.exclusiveModel,standardCollectionSourceModel:versions.collectionModel,standardCollectionSourceLibrary:library(),standardCollectionPointerWidth:64};}
function output(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function nodes(root){const all=[],pending=[root],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<1000000);seen.add(value);all.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return all;}
for(const row of originals)for(const version of ['1.90.0','1.99.0'])test('exclusive actual collection source '+row.name+'/'+version,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 const checked=checkProgram(parseRust(row.source,options(version))),carriers=[...checked.declarations.values()].filter(value=>value.sourceStandardCollectionCarrier),callables=[...checked.declarations.values()].filter(value=>value.sourceExclusiveSignature?.packet.length);assert.ok(carriers.length>0);if(row.requiresExclusivePacket!==false)assert.ok(callables.length>0);assert.equal(output(emitJavaScript(checked).text),row.stdout);
 for(const entry of callables){const signature=entry.sourceExclusiveSignature;assert.equal(signature.model,versions.exclusiveModel);assert.equal(entry.ret.elements.length,signature.packet.length+1);assert.equal(entry.sourceSignature,undefined);}
 for(const node of nodes(checked))assert.ok(!String(node.op??'').startsWith('rustExclusiveSource'),'all actual source packet markers must be consumed');
 const concrete=LinkNetwork.parse(row.source,'Rust');for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
});
for(const row of refusals)test('exclusive collection retained source boundary '+row.name,()=>{assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,options())),error=>['unsupported','type'].includes(error.kind));});
test('actual collection exclusive interfaces survive typed Links text/binary and every emitter',()=>{
 const row=originals[4],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:versions.exclusiveModel,rustStandardCollectionSourceModel:versions.collectionModel,rustStandardCollectionSourceLibrary:library(),rustStandardCollectionPointerWidth:64}),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),binary);const restored=decodeTypedProgramLinks(graph,packet.root);assert.ok([...restored.declarations.values()].some(entry=>entry.sourceStandardCollectionCarrier));for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
test('actual collection source cannot substitute an incomplete library or uncovered target',()=>{
 for(const standardCollectionSourceLibrary of [{}, {...library(),sourceProgram:{language:'Rust',items:[]}}])assert.throws(()=>parseRust(originals[0].source,{...options(),standardCollectionSourceLibrary}));
 assert.throws(()=>parseRust(originals[0].source,{...options(),standardCollectionPointerWidth:32}));
});
test('carrier/source metadata cannot change specialization or expose implementation fields',()=>{
 const surface=parseRust(originals[0].source,options()),carrier=nodes(surface).find(node=>node.k==='data'&&node.sourceStandardCollectionCarrier);assert.ok(carrier);carrier.sourceStandardCollectionCarrier.concreteIdentity='std::collections::HashMap<u16,u8>@64';assert.throws(()=>checkProgram(surface));
 const checked=checkProgram(parseRust(originals[0].source,options())),entry=[...checked.declarations.values()].find(entry=>entry.sourceExclusiveSignature?.packet.length),position=entry.sourceExclusiveSignature.parameters[0].position;assert.equal(position.owner.path.join('::'),'std::collections::HashMap');assert.equal(position.type.kind,'data');
});
